<script lang="ts">
  import { untrack } from "svelte";
  import type { Expression } from "$lib/expression";
  import type { Theme } from "$lib/themes";
  import type { Mood } from "$lib/types";

  let {
    theme,
    mood,
    cps,
    speech,
    expression,
    pulse,
    onpoke,
    onpet,
    onsqueeze,
  }: {
    theme: Theme;
    mood: Mood;
    cps: number;
    speech: string;
    expression: Expression | null;
    /** Increments whenever new clicks happened, to spawn little click rings. */
    pulse: number;
    onpoke: () => void;
    /** Mouse rubbed back and forth over the mascot. */
    onpet: () => void;
    /** "long" fires while still held; "release" when a long squeeze is let go. */
    onsqueeze: (phase: "long" | "release") => void;
  } = $props();

  // ---- squeezing: hold the mouse down on the mascot
  const LONG_PRESS_MS = 650;
  let held = $state(false);
  let longPress = false;
  let skipNextClick = false;
  let holdTimer: ReturnType<typeof setTimeout> | undefined;

  function pressDown(e: PointerEvent) {
    if (e.button !== 0) return;
    held = true;
    longPress = false;
    clearTimeout(holdTimer);
    holdTimer = setTimeout(() => {
      longPress = true;
      onsqueeze("long");
    }, LONG_PRESS_MS);
  }

  function pressUp() {
    if (!held) return;
    held = false;
    clearTimeout(holdTimer);
    if (longPress) {
      skipNextClick = true;
      onsqueeze("release");
    }
  }

  function clicked() {
    if (skipNextClick) {
      skipNextClick = false;
      return;
    }
    onpoke();
  }

  // ---- petting: rub the mouse left and right over the mascot
  const PET_FLIPS = 4;
  let petX = 0;
  let petDir = 0;
  let petFlips: number[] = [];
  let petCooldownUntil = 0;

  function rub(e: PointerEvent) {
    if (held || e.buttons) return;
    const dx = e.clientX - petX;
    if (Math.abs(dx) < 10) return;
    const dir = Math.sign(dx);
    const now = performance.now();
    if (petDir !== 0 && dir !== petDir) {
      petFlips = petFlips.filter((t) => now - t < 1400);
      petFlips.push(now);
      if (petFlips.length >= PET_FLIPS && now > petCooldownUntil) {
        petFlips = [];
        petCooldownUntil = now + 3500;
        onpet();
      }
    }
    petDir = dir;
    petX = e.clientX;
  }

  let stage: HTMLDivElement;
  let look = $state({ x: 0, y: 0 });
  let rings = $state<{ id: number; x: number }[]>([]);
  let ringId = 0;

  $effect(() => {
    if (pulse === 0) return;
    untrack(() => {
      const id = ++ringId;
      rings.push({ id, x: 30 + Math.random() * 40 });
      setTimeout(() => (rings = rings.filter((r) => r.id !== id)), 800);
    });
  });

  function track(e: MouseEvent) {
    if (!stage) return;
    const r = stage.getBoundingClientRect();
    look = {
      x: (e.clientX - (r.left + r.width / 2)) / (r.width * 0.9),
      y: (e.clientY - (r.top + r.height * 0.55)) / (r.height * 1.2),
    };
  }

  const bubbles = [
    { l: 8, s: 14, d: 9, delay: 0 },
    { l: 22, s: 8, d: 7, delay: -3 },
    { l: 78, s: 18, d: 11, delay: -5 },
    { l: 90, s: 10, d: 8, delay: -1 },
    { l: 62, s: 7, d: 6.5, delay: -4 },
    { l: 36, s: 11, d: 10, delay: -7 },
  ];
  const stars = [
    { l: 12, t: 20, d: 2.4 },
    { l: 84, t: 14, d: 3.1 },
    { l: 70, t: 40, d: 2.1 },
    { l: 26, t: 56, d: 2.8 },
    { l: 93, t: 62, d: 3.5 },
    { l: 5, t: 72, d: 2.2 },
    { l: 50, t: 10, d: 2.9 },
  ];

  const Mascot = $derived(theme.mascot);
</script>

<svelte:window onmousemove={track} />
<svelte:document onmouseleave={() => (look = { x: 0, y: 0 })} />

<div class="hero">
  {#if theme.dark}
    {#each stars as s, i (i)}
      <span class="star" style="left:{s.l}%; top:{s.t}%; animation-duration:{s.d}s"></span>
    {/each}
  {/if}
  {#each bubbles as b, i (i)}
    <span class="bubble" style="left:{b.l}%; width:{b.s}px; height:{b.s}px; animation-duration:{b.d}s; animation-delay:{b.delay}s"></span>
  {/each}

  <div class="speech" class:shout={mood === "clicking" || mood === "celebrate"}>
    {#key speech}
      <span class="speech-text">{speech}</span>
    {/key}
  </div>

  <div class="stage" bind:this={stage}>
    {#each rings as r (r.id)}
      <span class="ring" style="left:{r.x}%"></span>
    {/each}
    <button
      type="button"
      class="poke"
      onclick={clicked}
      onpointerdown={pressDown}
      onpointerup={pressUp}
      onpointerleave={pressUp}
      onpointercancel={pressUp}
      onpointermove={rub}
      aria-label="Poke Glorp (hold to squish, rub to pet)"
    >
      <Mascot {mood} {cps} {look} {expression} {held} />
    </button>
  </div>
</div>

<style>
  .hero {
    position: relative;
    height: 196px;
    margin: 0 12px;
    border-radius: 28px;
    background:
      radial-gradient(120% 90% at 50% 110%, color-mix(in srgb, var(--primary) 22%, transparent), transparent 60%),
      linear-gradient(160deg, var(--hero-1), var(--hero-2));
    overflow: hidden;
    display: flex;
    align-items: flex-end;
    justify-content: center;
    box-shadow: inset 0 0 0 1px var(--border);
  }
  .stage {
    position: relative;
    width: 168px;
    height: 168px;
    margin-bottom: -4px;
  }
  .poke {
    all: unset;
    display: block;
    width: 100%;
    height: 100%;
    cursor: pointer;
    -webkit-tap-highlight-color: transparent;
  }
  .poke:focus-visible {
    outline: 3px dashed var(--primary);
    outline-offset: -6px;
    border-radius: 30px;
  }

  .speech {
    position: absolute;
    top: 14px;
    left: 50%;
    transform: translateX(-50%);
    width: max-content;
    max-width: 300px;
    text-align: center;
    line-height: 1.25;
    background: var(--bubble);
    color: var(--text);
    font-family: var(--font-display);
    font-weight: 600;
    font-size: 14px;
    padding: 7px 14px;
    border-radius: 18px;
    box-shadow: 0 4px 14px -4px rgb(var(--shadow) / 0.25);
    z-index: 2;
    transition: transform 0.2s;
  }
  .speech::after {
    content: "";
    position: absolute;
    bottom: -7px;
    left: 50%;
    margin-left: 18px;
    border: 8px solid transparent;
    border-top-color: var(--bubble);
    border-bottom: 0;
  }
  .speech-text {
    display: inline-block;
    animation: pop 0.35s cubic-bezier(0.34, 1.6, 0.5, 1);
  }
  .shout {
    transform: translateX(-50%) rotate(-2deg);
  }

  .bubble {
    position: absolute;
    bottom: -20px;
    border-radius: 50%;
    background: radial-gradient(circle at 35% 30%, #fff, color-mix(in srgb, var(--particle) 70%, transparent));
    opacity: 0.55;
    animation: float linear infinite;
  }
  .star {
    position: absolute;
    width: 3px;
    height: 3px;
    border-radius: 50%;
    background: #fff;
    box-shadow: 0 0 6px 1px rgb(255 255 255 / 0.6);
    animation: twinkle ease-in-out infinite;
  }
  .ring {
    position: absolute;
    bottom: 8px;
    width: 26px;
    height: 10px;
    margin-left: -13px;
    border-radius: 50%;
    border: 3px solid var(--particle);
    animation: ring 0.8s ease-out forwards;
    pointer-events: none;
  }

  @keyframes pop {
    0% { transform: scale(0.6); opacity: 0; }
    100% { transform: scale(1); opacity: 1; }
  }
  @keyframes float {
    0% { transform: translateY(0) translateX(0); }
    50% { transform: translateY(-110px) translateX(8px); }
    100% { transform: translateY(-230px) translateX(-4px); }
  }
  @keyframes twinkle {
    0%, 100% { opacity: 0.2; transform: scale(0.6); }
    50% { opacity: 1; transform: scale(1.2); }
  }
  @keyframes ring {
    0% { transform: scale(0.4); opacity: 1; }
    100% { transform: scale(3); opacity: 0; }
  }
</style>
