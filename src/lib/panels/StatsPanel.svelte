<script lang="ts">
  import Card from "$lib/components/Card.svelte";
  import { dayKey, fmtCompact, fmtCps, fmtDuration } from "$lib/format";
  import type { LifetimeStats } from "$lib/types";

  let { stats, onreset }: { stats: LifetimeStats; onreset: () => void } = $props();

  /** Typical sustained human clicking speed, for the fun comparison. */
  const HUMAN_CPS = 6.5;
  /** Common rated lifetime of a mouse switch. */
  const SWITCH_LIFETIME = 20_000_000;

  const since = $derived(
    stats.firstSessionAt
      ? new Date(stats.firstSessionAt * 1000).toLocaleDateString(undefined, { day: "numeric", month: "short", year: "numeric" })
      : null,
  );
  const accuracy = $derived(stats.accuracySecs > 0 ? (stats.accuracyWeighted / stats.accuracySecs) * 100 : null);
  const cpu = $derived(stats.totalSecs > 0 ? (stats.cpuSecs / (stats.totalSecs * Math.max(1, stats.cpuCores))) * 100 : null);

  const tiles = $derived([
    { label: "Time glorping", value: fmtDuration(stats.totalSecs) },
    { label: "Sessions", value: stats.sessions.toLocaleString() },
    { label: "Longest session", value: fmtDuration(stats.longestSessionSecs) },
    { label: "Biggest session", value: `${fmtCompact(stats.biggestSessionClicks)}`, unit: "clicks" },
    { label: "Top speed", value: stats.bestCps > 0 ? fmtCps(stats.bestCps) : "–", unit: stats.bestCps > 0 ? "/s" : "" },
    {
      label: "Timing accuracy",
      value: accuracy === null ? "–" : accuracy >= 99.995 ? "100" : accuracy.toFixed(2),
      unit: accuracy === null ? "" : "%",
    },
    { label: "Avg CPU use", value: cpu === null ? "–" : cpu < 0.01 && cpu > 0 ? "<0.01" : (cpu ?? 0).toFixed(2), unit: cpu === null ? "" : "%" },
    { label: "Glorp pokes", value: stats.pokes.toLocaleString() },
  ]);

  // ---- last 7 days
  let hovered = $state<number | null>(null);
  const days = $derived.by(() => {
    const out = [];
    for (let i = 6; i >= 0; i--) {
      const d = new Date();
      d.setDate(d.getDate() - i);
      out.push({
        key: dayKey(d),
        label: i === 0 ? "today" : d.toLocaleDateString(undefined, { weekday: "short" }),
        long: d.toLocaleDateString(undefined, { weekday: "long", day: "numeric", month: "short" }),
        clicks: stats.daily[dayKey(d)] ?? 0,
      });
    }
    return out;
  });
  const maxDay = $derived(Math.max(1, ...days.map((d) => d.clicks)));
  const bestIndex = $derived(days.findIndex((d) => d.clicks === maxDay && d.clicks > 0));
  const weekTotal = $derived(days.reduce((a, d) => a + d.clicks, 0));

  // ---- buttons
  const presses = $derived(stats.leftClicks + stats.rightClicks + stats.middleClicks);
  const buttons = $derived([
    { label: "Left", value: stats.leftClicks },
    { label: "Middle", value: stats.middleClicks },
    { label: "Right", value: stats.rightClicks },
  ]);

  // ---- fun facts
  const tower = $derived.by(() => {
    const meters = stats.totalClicks / 1000; // one 1 mm slime drop per click
    if (meters < 1) return `${Math.round(meters * 100)} cm`;
    if (meters < 330) return `${meters.toFixed(1)} m, ${Math.round((meters / 330) * 100)}% of the Eiffel Tower`;
    if (meters < 8849) return `${(meters / 330).toFixed(1)} Eiffel Towers`;
    return `${(meters / 8849).toFixed(2)} Mount Everests`;
  });
  const switchPct = $derived((stats.leftClicks / SWITCH_LIFETIME) * 100);

  let confirmReset = $state(false);
  let confirmTimer: ReturnType<typeof setTimeout> | undefined;
  function reset() {
    if (!confirmReset) {
      confirmReset = true;
      clearTimeout(confirmTimer);
      confirmTimer = setTimeout(() => (confirmReset = false), 3000);
      return;
    }
    confirmReset = false;
    onreset();
  }
</script>

<section class="hero-stat">
  <span class="hero-label">Total clicks</span>
  <span class="hero-value">{stats.totalClicks.toLocaleString()}</span>
  <span class="hero-sub">
    {#if since}glorping since {since}{:else}no clicks yet. press start and make history!{/if}
  </span>
</section>

<div class="tiles">
  {#each tiles as t (t.label)}
    <div class="tile">
      <span class="tile-label">{t.label}</span>
      <span class="tile-value">{t.value}{#if t.unit}<small>{t.unit}</small>{/if}</span>
    </div>
  {/each}
</div>

<Card title="Clicks this week" icon="calendar">
  <div class="week-total">{weekTotal.toLocaleString()} <span>in the last 7 days</span></div>
  <div class="chart" role="list" aria-label="Clicks per day, last 7 days">
    {#each days as d, i (d.key)}
      <div
        class="col"
        role="listitem"
        aria-label="{d.long}: {d.clicks.toLocaleString()} clicks"
        onmouseenter={() => (hovered = i)}
        onmouseleave={() => (hovered = null)}
      >
        {#if hovered === i}
          <span class="tip">{d.long}<b>{d.clicks.toLocaleString()} clicks</b></span>
        {:else if i === bestIndex}
          <span class="best">{fmtCompact(d.clicks)}</span>
        {/if}
        <span class="bar-track">
          <span
            class="bar"
            class:today={i === 6}
            class:dim={hovered !== null && hovered !== i}
            style="height:{d.clicks > 0 ? Math.max(3, (d.clicks / maxDay) * 100) : 0}%"
          ></span>
        </span>
        <span class="day" class:today={i === 6}>{d.label}</span>
      </div>
    {/each}
  </div>
</Card>

<Card title="Mouse buttons" icon="mouse" hint={stats.doubleClicks > 0 ? `${stats.doubleClicks.toLocaleString()} of those came as double clicks.` : undefined}>
  <div class="buttons">
    {#each buttons as b (b.label)}
      {@const pct = presses > 0 ? (b.value / presses) * 100 : 0}
      <div class="brow">
        <span class="bname">{b.label}</span>
        <span class="btrack"><span class="bfill" style="width:{pct}%"></span></span>
        <span class="bval">{fmtCompact(b.value)}</span>
      </div>
    {/each}
  </div>
</Card>

<Card title="Fun facts" icon="trophy">
  <ul class="facts">
    <li>
      A typical human (≈{HUMAN_CPS} clicks/s, no breaks) would need
      <b>{fmtDuration(stats.totalClicks / HUMAN_CPS)}</b> to click that much.
    </li>
    <li>Stacked as 1 mm slime drops, your clicks would be <b>{tower}</b> tall.</li>
    <li>
      Glorp has spared your left mouse button <b>{switchPct < 0.01 && switchPct > 0 ? "<0.01" : switchPct.toFixed(2)}%</b>
      of a 20-million-click lifetime.
    </li>
    <li>
      Glorp has been poked <b>{stats.pokes.toLocaleString()}</b> time{stats.pokes === 1 ? "" : "s"}. He's keeping count.
    </li>
  </ul>
</Card>

<div class="reset-row">
  <button type="button" class="reset" class:confirm={confirmReset} onclick={reset}>
    {confirmReset ? "Sure? Tap again to wipe everything" : "Reset stats"}
  </button>
</div>

<style>
  .hero-stat {
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 16px 12px 14px;
    border-radius: 22px;
    background:
      radial-gradient(120% 120% at 50% 0%, color-mix(in srgb, var(--primary) 30%, transparent), transparent 70%),
      var(--surface);
    border: 1px solid var(--border);
    box-shadow: 0 6px 18px -8px rgb(var(--shadow) / 0.18);
  }
  .hero-label {
    font-size: 11px;
    font-weight: 800;
    letter-spacing: 0.8px;
    text-transform: uppercase;
    color: var(--text-soft);
  }
  .hero-value {
    font-family: var(--font-display);
    font-weight: 700;
    font-size: 40px;
    line-height: 1.1;
    color: var(--text);
    font-variant-numeric: tabular-nums;
  }
  .hero-sub {
    font-size: 12.5px;
    color: var(--text-soft);
  }

  .tiles {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 8px;
  }
  .tile {
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: 18px;
    padding: 9px 12px 10px;
    display: flex;
    flex-direction: column;
    gap: 1px;
  }
  .tile-label {
    font-size: 11px;
    font-weight: 800;
    letter-spacing: 0.5px;
    text-transform: uppercase;
    color: var(--muted);
  }
  .tile-value {
    font-family: var(--font-display);
    font-weight: 600;
    font-size: 20px;
    color: var(--text);
    font-variant-numeric: tabular-nums;
  }
  .tile-value small {
    font-size: 13px;
    color: var(--text-soft);
    margin-left: 2px;
  }

  .week-total {
    font-family: var(--font-display);
    font-weight: 600;
    font-size: 18px;
    color: var(--text);
    margin: -4px 2px 6px;
  }
  .week-total span {
    font-family: var(--font-body);
    font-weight: 600;
    font-size: 12.5px;
    color: var(--text-soft);
  }
  .chart {
    display: grid;
    grid-template-columns: repeat(7, 1fr);
    gap: 6px;
    height: 132px;
  }
  .col {
    position: relative;
    display: flex;
    flex-direction: column;
    align-items: center;
    padding-top: 22px;
    cursor: default;
  }
  .bar-track {
    flex: 1;
    width: 100%;
    display: flex;
    align-items: flex-end;
    justify-content: center;
    border-bottom: 1px solid var(--border);
  }
  .bar {
    width: 70%;
    max-width: 30px;
    border-radius: 4px 4px 0 0;
    background: color-mix(in srgb, var(--primary) 75%, var(--surface));
    transition: height 0.6s cubic-bezier(0.34, 1.3, 0.5, 1), opacity 0.2s;
  }
  .bar.today {
    background: var(--primary-2);
  }
  .bar.dim {
    opacity: 0.45;
  }
  .day {
    margin-top: 5px;
    font-size: 11.5px;
    font-weight: 700;
    color: var(--muted);
  }
  .day.today {
    color: var(--text);
  }
  .best {
    position: absolute;
    top: 2px;
    font-family: var(--font-display);
    font-weight: 600;
    font-size: 12px;
    color: var(--text-soft);
  }
  .tip {
    position: absolute;
    top: -6px;
    left: 50%;
    transform: translateX(-50%);
    z-index: 2;
    white-space: nowrap;
    background: var(--text);
    color: var(--surface);
    font-size: 11.5px;
    font-weight: 600;
    padding: 4px 9px;
    border-radius: 10px;
    display: flex;
    flex-direction: column;
    align-items: center;
    pointer-events: none;
    box-shadow: 0 4px 12px -4px rgb(var(--shadow) / 0.4);
  }
  .col:first-child .tip {
    left: 0;
    transform: none;
  }
  .col:last-child .tip {
    left: auto;
    right: 0;
    transform: none;
  }
  .tip b {
    font-family: var(--font-display);
    font-size: 13px;
  }

  .buttons {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }
  .brow {
    display: grid;
    grid-template-columns: 52px 1fr 52px;
    align-items: center;
    gap: 8px;
  }
  .bname {
    font-weight: 700;
    color: var(--text-soft);
    font-size: 13px;
  }
  .btrack {
    height: 12px;
    border-radius: 999px;
    background: var(--surface-3);
    overflow: hidden;
  }
  .bfill {
    display: block;
    height: 100%;
    border-radius: 999px;
    background: var(--primary);
    transition: width 0.6s cubic-bezier(0.34, 1.3, 0.5, 1);
  }
  .bval {
    text-align: right;
    font-family: var(--font-display);
    font-weight: 600;
    font-size: 13.5px;
    color: var(--text);
    font-variant-numeric: tabular-nums;
  }

  .facts {
    margin: 0;
    padding: 0;
    list-style: none;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }
  .facts li {
    position: relative;
    padding-left: 18px;
    font-size: 13px;
    line-height: 1.45;
    color: var(--text-soft);
  }
  .facts li::before {
    content: "";
    position: absolute;
    left: 3px;
    top: 6px;
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: var(--primary);
  }
  .facts b {
    color: var(--text);
  }

  .reset-row {
    display: flex;
    justify-content: center;
    padding: 2px 0 6px;
  }
  .reset {
    font: inherit;
    font-family: var(--font-display);
    font-weight: 600;
    font-size: 13px;
    color: var(--text-soft);
    background: transparent;
    border: 2px solid var(--border);
    border-radius: 999px;
    padding: 6px 16px;
    cursor: pointer;
    transition: all 0.2s;
  }
  .reset:hover {
    border-color: var(--stop);
    color: var(--stop-2);
  }
  .reset.confirm {
    background: var(--stop);
    border-color: var(--stop);
    color: #fff;
  }
</style>
