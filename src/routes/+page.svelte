<script lang="ts">
  import { onMount } from "svelte";
  import { fly } from "svelte/transition";
  import { Tween } from "svelte/motion";
  import { cubicOut } from "svelte/easing";
  import { api, on } from "$lib/api";
  import { applyTheme, getTheme } from "$lib/themes";
  import { hotkeyParts, prettyHotkey } from "$lib/hotkeys";
  import { DEFAULT_SETTINGS, EMPTY_STATS, type LifetimeStats, type Mood, type Settings } from "$lib/types";
  import { F, MOOD_FACES, type Expression, type LineContext, type Reaction } from "$lib/expression";
  import { lineText } from "$lib/themes/glorp/lines";
  import { Brain, lineDuration } from "$lib/brain";
  import { fmtClock, fmtCps } from "$lib/format";
  import TitleBar from "$lib/components/TitleBar.svelte";
  import Hero from "$lib/components/Hero.svelte";
  import Icon from "$lib/components/Icon.svelte";
  import SpeedPanel from "$lib/panels/SpeedPanel.svelte";
  import MousePanel from "$lib/panels/MousePanel.svelte";
  import GlorpPanel from "$lib/panels/GlorpPanel.svelte";
  import StatsPanel from "$lib/panels/StatsPanel.svelte";

  const COUNTDOWN_FROM = 3;
  const SLEEP_AFTER_MS = 60_000;
  /** Random idle chatter happens every IDLE_MIN..IDLE_MAX ms while nothing else is going on. */
  const IDLE_MIN = 14_000;
  const IDLE_MAX = 30_000;
  const CLICK_LINE_EVERY = 3_200;
  const MILESTONES = [100, 1_000, 5_000, 10_000, 50_000, 100_000, 500_000, 1_000_000, 10_000_000];

  let settings = $state<Settings>({ ...DEFAULT_SETTINGS });
  let loaded = $state(false);
  let running = $state(false);
  let countdown = $state(0);
  let clicks = $state(0);
  let pulse = $state(0);
  let picking = $state(false);
  let hovering = $state(true);
  let tab = $state<"speed" | "mouse" | "stats" | "glorp">("speed");
  let lifetime = $state<LifetimeStats>({ ...EMPTY_STATS });
  let avgCps = $state(0);
  let runStartedAt = $state(0);
  let clock = $state(Date.now());
  let clockTimer: ReturnType<typeof setInterval> | undefined;

  const cps = new Tween(0, { duration: 300, easing: cubicOut });

  const theme = $derived(getTheme(settings.theme));
  $effect(() => applyTheme(theme));
  const brain = new Brain(getTheme(DEFAULT_SETTINGS.theme).lines);
  $effect(() => {
    brain.lines = theme.lines;
  });

  const ctx = (): LineContext => ({
    hotkey: prettyHotkey(settings.hotkey),
    cps: cps.target,
    clicks,
    total: lifetime.totalClicks,
    pokes: lifetime.pokes,
    milestone: 0,
  });

  // ---------------------------------------------------------------- reactions

  // A short-lived line + face (poke reactions, errors, celebrations) shown over the defaults.
  let flash = $state<{ text: string; face: Expression; mood?: Mood } | null>(null);
  let flashTimer: ReturnType<typeof setTimeout> | undefined;
  let faceId = 0;
  let nextIdleAt = Date.now() + IDLE_MIN;

  function react(r: Reaction, extra: Partial<LineContext> = {}) {
    const text = lineText(r, { ...ctx(), ...extra });
    clearTimeout(flashTimer);
    flash = { text, face: { ...r.face, id: ++faceId } };
    flashTimer = setTimeout(() => (flash = null), lineDuration(r, text));
    nextIdleAt = Date.now() + IDLE_MIN + Math.random() * (IDLE_MAX - IDLE_MIN);
  }

  /** Quick helper for one-off lines that only need a mood's face. */
  function say(text: string, ms = 2600, mood: Mood = "idle") {
    react({ text, face: MOOD_FACES[mood], ms });
  }

  // Rotating lines while clicking.
  let runLine = $state<Reaction | null>(null);
  let lastRunLineAt = 0;
  function nextRunLine() {
    const l = theme.lines;
    const speed = cps.target || avgCps;
    runLine =
      speed >= 50 ? brain.pick("turbo", l.turbo) : speed > 0 && speed < 1 ? brain.pick("slow", l.slow) : brain.pick("clicking", l.clicking);
  }

  let lastActivity = $state(Date.now());
  let now = $state(Date.now());
  const sleepy = $derived(!running && countdown === 0 && now - lastActivity > SLEEP_AFTER_MS);

  const mood = $derived.by<Mood>(() => {
    if (countdown > 0) return "countdown";
    if (running) return "clicking";
    if (flash?.mood) return flash.mood;
    if (sleepy && !flash) return "sleepy";
    return "idle";
  });

  const expression = $derived.by<Expression | null>(() => {
    if (countdown > 0) return null;
    if (flash) return flash.face;
    if (running && hovering && settings.positionMode === "cursor") return F("wide", "o", { fx: ["question"] });
    if (running && runLine) return runLine.face;
    return null;
  });

  const hotkeyLabel = $derived(prettyHotkey(settings.hotkey));

  const speech = $derived.by(() => {
    if (countdown > 0) {
      if (countdown === 3) return "3… get ready!";
      if (countdown === 2) return settings.positionMode === "cursor" ? "2… point me at the spot!" : "2…";
      return "1… here we go!";
    }
    if (flash) return flash.text;
    if (running && hovering && settings.positionMode === "cursor") return "move off my window to glorp!";
    if (running && runLine) return lineText(runLine, ctx());
    if (running) return "glorp glorp!";
    if (mood === "sleepy") return "zzz… glorp…";
    return `press ${hotkeyLabel} and i'll glorp!`;
  });

  const remaining = $derived.by(() => {
    if (settings.repeatMode === "count") {
      return { label: "left", value: Math.max(0, settings.repeatCount - clicks).toLocaleString() };
    }
    if (settings.repeatMode === "time") {
      const left = running ? settings.durationSecs - (clock - runStartedAt) / 1000 : settings.durationSecs;
      return { label: "time left", value: fmtClock(left) };
    }
    return { label: "repeat", value: "∞" };
  });

  function poke() {
    react(brain.poke(Date.now(), sleepy));
    wake();
    api.recordPoke().then((n) => (lifetime.pokes = n)).catch(() => {});
  }

  function pet() {
    if (running || brain.isGrumpy(Date.now())) return;
    react(brain.pick("pet", theme.lines.pet));
  }

  function squeeze(phase: "long" | "release") {
    if (phase === "long") react({ ...brain.pick("holdLong", theme.lines.holdLong), ms: 60_000 });
    else react(brain.pick("holdRelease", theme.lines.holdRelease));
  }

  function wake() {
    lastActivity = Date.now();
    now = lastActivity;
  }

  // ---------------------------------------------------------------- persistence

  let saveTimer: ReturnType<typeof setTimeout> | undefined;
  let lastSaved: Settings = { ...DEFAULT_SETTINGS };
  $effect(() => {
    const snapshot = $state.snapshot(settings) as Settings;
    if (!loaded) return;
    clearTimeout(saveTimer);
    saveTimer = setTimeout(async () => {
      try {
        lastSaved = await api.updateSettings(snapshot);
      } catch (e) {
        // Only system-level settings (like start-with-Windows) can fail; roll them back.
        say(String(e), 3500, "sad");
        settings.autostart = lastSaved.autostart;
      }
    }, 250);
  });

  // React to a new theme (but not to the theme loading at startup).
  let shownTheme = "";
  $effect(() => {
    const id = settings.theme;
    if (!loaded) return;
    if (shownTheme && shownTheme !== id) react(brain.pick("themeChange", theme.lines.themeChange));
    shownTheme = id;
  });

  async function changeHotkey(accel: string): Promise<string | null> {
    try {
      const saved = await api.updateSettings({ ...$state.snapshot(settings), hotkey: accel } as Settings);
      settings.hotkey = saved.hotkey;
      react({ text: `new hotkey: ${prettyHotkey(accel)}!`, face: F("star", "grin", { anim: "hop", fx: ["sparkles"] }) });
      return null;
    } catch (e) {
      react({ text: "that key is taken :(", face: F("teary", "frown", { brows: "worried", anim: "shake" }) });
      return String(e);
    }
  }

  // ---------------------------------------------------------------- start / stop

  let countdownTimer: ReturnType<typeof setInterval> | undefined;

  function cancelCountdown() {
    clearInterval(countdownTimer);
    countdown = 0;
  }

  async function onMainButton() {
    if (running) {
      await api.stop();
      return;
    }
    if (countdown > 0) {
      cancelCountdown();
      react({ text: "okay, never mind!", face: F("happy", "cat", { anim: "nod" }) });
      return;
    }
    // Give the user a moment to move the mouse where the clicks should go.
    flash = null;
    countdown = COUNTDOWN_FROM;
    countdownTimer = setInterval(async () => {
      countdown -= 1;
      if (countdown <= 0) {
        cancelCountdown();
        await api.start();
      }
    }, 1000);
  }

  function pickSpot() {
    if (picking) return;
    picking = true;
    react({ text: "hiding! click the spot you want", face: F("wink", "grin"), ms: 900 });
    setTimeout(() => api.pickLocation(), 700);
  }

  onMount(() => {
    const unlisten: Promise<() => void>[] = [];

    unlisten.push(
      on("clicker-state", ({ running: r, reason }) => {
        running = r;
        clearInterval(clockTimer);
        if (r) {
          cancelCountdown();
          flash = null;
          clicks = 0;
          avgCps = 0;
          cps.set(0, { duration: 0 });
          runStartedAt = Date.now();
          clock = runStartedAt;
          runLine = null;
          clockTimer = setInterval(() => {
            clock = Date.now();
            if (!runLine || clock - lastRunLineAt > CLICK_LINE_EVERY) {
              lastRunLineAt = clock;
              nextRunLine();
            }
          }, 250);
        } else {
          cps.target = 0;
          wake();
          const l = theme.lines;
          if (reason === "done") {
            react(settings.repeatMode === "time" ? brain.pick("timeUp", l.timeUp) : brain.pick("done", l.done));
          } else if (clicks > 0) {
            react(brain.pick("stopped", l.stopped));
          }
        }
      }),
      on("clicker-stats", (s) => {
        if (s.clicks !== clicks) pulse += 1;
        clicks = s.clicks;
        cps.target = s.cps;
        if (s.avgCps > 0) avgCps = s.avgCps;
      }),
      on("stats-updated", (s) => {
        const before = lifetime.totalClicks;
        lifetime = s;
        const hit = MILESTONES.filter((m) => before < m && s.totalClicks >= m).pop();
        // Let the "done" line show first, then party.
        if (hit) setTimeout(() => react(brain.pick("milestone", theme.lines.milestone), { milestone: hit }), 3000);
      }),
      on("pick-result", (p) => {
        picking = false;
        if (p) {
          settings.fixedX = p.x;
          settings.fixedY = p.y;
          react({ text: `got it! (${p.x}, ${p.y})`, face: F("star", "grin", { anim: "hop", fx: ["sparkles"] }) });
        } else {
          react({ text: "never mind then!", face: F("happy", "cat", { anim: "nod" }) });
        }
      }),
    );

    (async () => {
      try {
        settings = await api.getSettings();
        lastSaved = { ...settings };
        running = await api.isRunning();
        lifetime = await api.getStats();
      } catch {
        /* keep defaults */
      }
      applyTheme(getTheme(settings.theme));
      loaded = true;
      // Let the first frame paint before showing the window.
      requestAnimationFrame(() => requestAnimationFrame(() => api.ready()));
      setTimeout(() => react(brain.pick("greeting", theme.lines.greeting(new Date().getHours()))), 500);
    })();

    // Once a second: sleepiness, forgiving after a grumpy spell, and random idle chatter.
    const tick = setInterval(() => {
      now = Date.now();
      const sorry = brain.maybeForgive(now);
      if (sorry) {
        react(sorry);
        return;
      }
      const busy = running || countdown > 0 || picking || flash || sleepy || document.hidden;
      if (!busy && now > nextIdleAt) react(brain.pick("idle", theme.lines.idle));
    }, 1000);

    // Say hi again when the window comes back from the tray.
    let hiddenAt = 0;
    const visibility = () => {
      if (document.hidden) {
        hiddenAt = Date.now();
      } else if (hiddenAt && Date.now() - hiddenAt > 20_000 && !running) {
        wake();
        react(brain.pick("welcomeBack", theme.lines.welcomeBack));
      }
    };
    document.addEventListener("visibilitychange", visibility);

    const noMenu = (e: MouseEvent) => {
      if (!(e.target instanceof HTMLInputElement)) e.preventDefault();
    };
    window.addEventListener("pointermove", wake);
    window.addEventListener("keydown", wake);
    document.addEventListener("contextmenu", noMenu);
    const enter = () => (hovering = true);
    const leave = () => (hovering = false);
    document.documentElement.addEventListener("mouseenter", enter);
    document.documentElement.addEventListener("mouseleave", leave);

    return () => {
      unlisten.forEach((p) => p.then((u) => u()));
      clearInterval(tick);
      clearInterval(clockTimer);
      cancelCountdown();
      document.removeEventListener("visibilitychange", visibility);
      window.removeEventListener("pointermove", wake);
      window.removeEventListener("keydown", wake);
      document.removeEventListener("contextmenu", noMenu);
      document.documentElement.removeEventListener("mouseenter", enter);
      document.documentElement.removeEventListener("mouseleave", leave);
    };
  });

  const tabs = [
    { id: "speed", label: "Speed", icon: "clock" },
    { id: "mouse", label: "Mouse", icon: "cursor" },
    { id: "stats", label: "Stats", icon: "chart" },
    { id: "glorp", label: "Glorp", icon: "sparkle" },
  ] as const;
  const tabIndex = $derived(tabs.findIndex((t) => t.id === tab));
</script>

<div class="app" class:ready={loaded}>
  <TitleBar {running} />

  <Hero {theme} {mood} cps={cps.current} {speech} {expression} {pulse} onpoke={poke} onpet={pet} onsqueeze={squeeze} />

  <div class="controls">
    <button
      type="button"
      class="go"
      class:running
      class:counting={countdown > 0}
      onclick={onMainButton}
    >
      <span class="go-icon">
        {#if countdown > 0}
          <span class="count">{countdown}</span>
        {:else}
          <Icon name={running ? "stop" : "play"} size={20} filled />
        {/if}
      </span>
      <span class="go-label">
        {#if running}Stop glorping{:else if countdown > 0}Starting… (tap to cancel){:else}Start glorping{/if}
      </span>
      <span class="go-keys">
        {#each hotkeyParts(settings.hotkey) as k, i (i)}<kbd>{k}</kbd>{/each}
      </span>
    </button>

    <div class="stats">
      <div class="stat">
        <span class="label">clicks</span>
        <span class="value">{clicks.toLocaleString()}</span>
      </div>
      <div class="stat">
        <span class="label">avg speed</span>
        <span class="value">{fmtCps(avgCps)}<small>/s</small></span>
      </div>
      <div class="stat">
        <span class="label">{remaining.label}</span>
        <span class="value">{remaining.value}</span>
      </div>
    </div>
  </div>

  <nav class="tabs" style="--i:{tabIndex}">
    <span class="tab-pill" aria-hidden="true"></span>
    {#each tabs as t (t.id)}
      <button type="button" class:on={tab === t.id} onclick={() => (tab = t.id)}>
        <Icon name={t.icon} size={16} />{t.label}
      </button>
    {/each}
  </nav>

  <main class="panel">
    {#key tab}
      <div class="panel-inner" in:fly={{ y: 10, duration: 260, easing: cubicOut }}>
        {#if tab === "speed"}
          <SpeedPanel bind:settings />
        {:else if tab === "mouse"}
          <MousePanel bind:settings {picking} onpick={pickSpot} />
        {:else if tab === "stats"}
          <StatsPanel stats={lifetime} onreset={async () => (lifetime = await api.resetStats())} />
        {:else}
          <GlorpPanel bind:settings onhotkey={changeHotkey} onquit={() => api.quit()} />
        {/if}
      </div>
    {/key}
  </main>
</div>

<style>
  .app {
    height: 100vh;
    display: flex;
    flex-direction: column;
    opacity: 0;
    transition: opacity 0.3s;
  }
  .app.ready {
    opacity: 1;
  }

  .controls {
    padding: 12px 12px 0;
    display: flex;
    flex-direction: column;
    gap: 10px;
  }
  .go {
    position: relative;
    display: flex;
    align-items: center;
    gap: 12px;
    width: 100%;
    height: 58px;
    padding: 0 10px 0 8px;
    border: 0;
    border-radius: 999px;
    cursor: pointer;
    font: inherit;
    font-family: var(--font-display);
    font-weight: 600;
    font-size: 18px;
    color: var(--primary-text);
    background: linear-gradient(180deg, var(--primary), var(--primary-2));
    box-shadow:
      0 5px 0 color-mix(in srgb, var(--primary-2) 70%, #000),
      0 12px 22px -8px color-mix(in srgb, var(--primary-2) 70%, transparent);
    transition: transform 0.12s, box-shadow 0.12s, background 0.3s;
  }
  .go:hover {
    filter: brightness(1.04);
  }
  .go:active {
    transform: translateY(4px) scale(0.99);
    box-shadow:
      0 1px 0 color-mix(in srgb, var(--primary-2) 70%, #000),
      0 6px 12px -8px color-mix(in srgb, var(--primary-2) 70%, transparent);
  }
  .go.running {
    background: linear-gradient(180deg, var(--stop), var(--stop-2));
    box-shadow:
      0 5px 0 color-mix(in srgb, var(--stop-2) 70%, #000),
      0 12px 22px -8px color-mix(in srgb, var(--stop-2) 70%, transparent);
    color: #fff;
  }
  .go.running::before {
    content: "";
    position: absolute;
    inset: -4px;
    border-radius: inherit;
    border: 3px solid var(--stop);
    animation: halo 1.3s ease-out infinite;
    pointer-events: none;
  }
  .go.counting {
    background: linear-gradient(180deg, var(--accent-2), color-mix(in srgb, var(--accent-2) 70%, #000));
    box-shadow: 0 5px 0 color-mix(in srgb, var(--accent-2) 55%, #000);
    color: #fff;
  }
  .go-icon {
    width: 42px;
    height: 42px;
    flex: none;
    border-radius: 50%;
    display: grid;
    place-items: center;
    background: rgb(255 255 255 / 0.25);
  }
  .count {
    font-size: 22px;
    animation: tick 1s ease-out infinite;
  }
  .go-label {
    flex: 1;
    text-align: left;
    white-space: nowrap;
    text-shadow: 0 1px 0 rgb(0 0 0 / 0.12);
  }
  .counting .go-label {
    font-size: 15px;
  }
  .go-keys {
    display: flex;
    gap: 3px;
  }
  .go-keys kbd {
    background: rgb(255 255 255 / 0.25);
    border-color: rgb(255 255 255 / 0.35);
    color: inherit;
    box-shadow: none;
  }

  .stats {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 8px;
  }
  .stat {
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: 16px;
    padding: 6px 10px 7px;
    display: flex;
    flex-direction: column;
    align-items: center;
  }
  .stat .label {
    font-size: 11px;
    font-weight: 700;
    letter-spacing: 0.6px;
    text-transform: uppercase;
    color: var(--muted);
  }
  .stat .value {
    font-family: var(--font-display);
    font-weight: 600;
    font-size: 19px;
    color: var(--text);
    font-variant-numeric: tabular-nums;
    line-height: 1.2;
  }
  .stat small {
    font-size: 12px;
    color: var(--muted);
    margin-left: 1px;
  }

  .tabs {
    position: relative;
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    margin: 12px 12px 0;
    padding: 4px;
    background: var(--surface-3);
    border-radius: 18px;
    isolation: isolate;
  }
  .tab-pill {
    position: absolute;
    top: 4px;
    bottom: 4px;
    left: 4px;
    width: calc((100% - 8px) / 4);
    transform: translateX(calc(100% * var(--i)));
    background: var(--surface);
    border-radius: 14px;
    box-shadow: 0 2px 8px rgb(var(--shadow) / 0.14);
    transition: transform 0.4s cubic-bezier(0.34, 1.35, 0.5, 1);
    z-index: -1;
  }
  .tabs button {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 5px;
    font: inherit;
    font-family: var(--font-display);
    font-weight: 600;
    font-size: 13.5px;
    color: var(--text-soft);
    background: none;
    border: 0;
    padding: 8px 0;
    cursor: pointer;
    transition: color 0.2s, transform 0.12s;
  }
  .tabs button:active {
    transform: scale(0.94);
  }
  .tabs button.on {
    color: var(--text);
  }

  .panel {
    flex: 1;
    min-height: 0;
    overflow-y: auto;
    padding: 12px 12px 14px;
    scrollbar-gutter: stable;
  }
  .panel-inner {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  @keyframes halo {
    0% { transform: scale(1); opacity: 0.8; }
    100% { transform: scale(1.06, 1.25); opacity: 0; }
  }
  @keyframes tick {
    0% { transform: scale(1.5); }
    30% { transform: scale(1); }
  }
</style>
