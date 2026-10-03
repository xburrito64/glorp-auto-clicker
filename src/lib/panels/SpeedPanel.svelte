<script lang="ts">
  import Card from "$lib/components/Card.svelte";
  import NumberField from "$lib/components/NumberField.svelte";
  import Segmented from "$lib/components/Segmented.svelte";
  import type { IntervalUnit, Settings } from "$lib/types";

  let { settings = $bindable() }: { settings: Settings } = $props();

  const MAX_MS = 3_600_000;

  const unitCfg = {
    ms: { min: 1, max: MAX_MS, step: 10, decimals: 0, suffix: "ms", to: (ms: number) => ms, from: (v: number) => v },
    s: { min: 0.001, max: MAX_MS / 1000, step: 0.1, decimals: 3, suffix: "sec", to: (ms: number) => ms / 1000, from: (v: number) => v * 1000 },
    cps: { min: 0.01, max: 1000, step: 1, decimals: 2, suffix: "/ sec", to: (ms: number) => 1000 / ms, from: (v: number) => 1000 / v },
  } satisfies Record<IntervalUnit, unknown>;

  const cfg = $derived(unitCfg[settings.intervalUnit]);
  const cps = $derived(1000 / settings.intervalMs);

  const presets = [
    { label: "Chill", cps: 1 },
    { label: "Quick", cps: 10 },
    { label: "Turbo", cps: 30 },
    { label: "Glorpspeed", cps: 100 },
  ];

  const fmtCps = (n: number) => (n >= 10 ? Math.round(n).toString() : Number(n.toFixed(2)).toString());
  const summary = $derived(
    cps >= 1
      ? `≈ ${fmtCps(cps)} click${cps === 1 ? "" : "s"} per second`
      : `1 click every ${Number((settings.intervalMs / 1000).toFixed(2))} seconds`,
  );
</script>

<Card title="Click every" icon="clock">
  <div class="interval">
    <NumberField
      label="Interval"
      value={cfg.to(settings.intervalMs)}
      min={cfg.min}
      max={cfg.max}
      step={cfg.step}
      decimals={cfg.decimals}
      suffix={cfg.suffix}
      onchange={(v) => (settings.intervalMs = Math.min(MAX_MS, Math.max(1, cfg.from(v))))}
    />
    <div class="unit">
      <Segmented
        small
        label="Unit"
        bind:value={settings.intervalUnit}
        options={[
          { value: "ms", label: "ms" },
          { value: "s", label: "sec" },
          { value: "cps", label: "cps" },
        ]}
      />
    </div>
  </div>
  <p class="summary">{summary}</p>
  <div class="presets">
    {#each presets as p (p.label)}
      <button
        type="button"
        class="chip"
        class:on={Math.abs(cps - p.cps) < 0.001}
        onclick={() => (settings.intervalMs = 1000 / p.cps)}
      >
        {p.label}<small>{p.cps}/s</small>
      </button>
    {/each}
  </div>
</Card>

<Card title="Wobble" icon="wave" hint="Randomly varies each gap so clicks feel more human.">
  <div class="slider-row">
    <input
      type="range"
      min="0"
      max="50"
      step="5"
      bind:value={settings.jitterPct}
      style="--p:{settings.jitterPct * 2}%"
      aria-label="Random timing wobble"
    />
    <span class="badge">{settings.jitterPct === 0 ? "off" : `±${settings.jitterPct}%`}</span>
  </div>
</Card>

<Card title="Repeat" icon="repeat">
  <Segmented
    label="Repeat mode"
    bind:value={settings.repeatMode}
    options={[
      { value: "forever", label: "Until stopped" },
      { value: "count", label: "Set amount" },
    ]}
  />
  {#if settings.repeatMode === "count"}
    <div class="count">
      <NumberField
        label="Number of clicks"
        value={settings.repeatCount}
        min={1}
        max={10_000_000}
        step={10}
        suffix="clicks"
        onchange={(v) => (settings.repeatCount = Math.round(v))}
      />
    </div>
  {/if}
</Card>

<style>
  .interval {
    display: grid;
    grid-template-columns: 1fr 138px;
    gap: 8px;
    align-items: center;
  }
  .summary {
    margin: 8px 4px 10px;
    font-size: 13px;
    color: var(--text-soft);
    font-weight: 600;
  }
  .presets {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 6px;
  }
  .chip {
    font: inherit;
    font-family: var(--font-display);
    font-weight: 600;
    font-size: 12.5px;
    color: var(--text-soft);
    background: var(--surface-2);
    border: 2px solid var(--border);
    border-radius: 14px;
    padding: 6px 2px 5px;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1px;
    cursor: pointer;
    transition: transform 0.15s, border-color 0.2s, background 0.2s, color 0.2s;
  }
  .chip small {
    font-size: 11px;
    color: var(--muted);
    font-weight: 500;
  }
  .chip:hover {
    border-color: var(--primary);
  }
  .chip:active {
    transform: scale(0.93);
  }
  .chip.on {
    background: color-mix(in srgb, var(--primary) 18%, var(--surface));
    border-color: var(--primary);
    color: var(--text);
  }
  .slider-row {
    display: flex;
    align-items: center;
    gap: 12px;
  }
  .badge {
    min-width: 52px;
    text-align: center;
    font-family: var(--font-display);
    font-weight: 600;
    font-size: 13px;
    padding: 4px 8px;
    border-radius: 999px;
    background: var(--surface-3);
    color: var(--text);
  }
  .count {
    margin-top: 10px;
  }
</style>
