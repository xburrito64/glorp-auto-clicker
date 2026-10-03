<script lang="ts">
  import Card from "$lib/components/Card.svelte";
  import Icon from "$lib/components/Icon.svelte";
  import NumberField from "$lib/components/NumberField.svelte";
  import Segmented from "$lib/components/Segmented.svelte";
  import type { Settings } from "$lib/types";

  let {
    settings = $bindable(),
    picking,
    onpick,
  }: { settings: Settings; picking: boolean; onpick: () => void } = $props();
</script>

<Card title="Mouse button" icon="mouse">
  <Segmented
    label="Mouse button"
    bind:value={settings.button}
    options={[
      { value: "left", label: "Left" },
      { value: "middle", label: "Middle" },
      { value: "right", label: "Right" },
    ]}
  />
</Card>

<Card title="Click type" icon="sparkle">
  <Segmented
    label="Click type"
    bind:value={settings.clickKind}
    options={[
      { value: "single", label: "Single" },
      { value: "double", label: "Double" },
    ]}
  />
</Card>

<Card
  title="Where to click"
  icon="target"
  hint={settings.positionMode === "cursor"
    ? "Clicks wherever your mouse is. Glorp never clicks on his own window."
    : "Moves the mouse to this spot for every click."}
>
  <Segmented
    label="Click position"
    bind:value={settings.positionMode}
    options={[
      { value: "cursor", label: "Follow mouse" },
      { value: "fixed", label: "Fixed spot" },
    ]}
  />
  {#if settings.positionMode === "fixed"}
    <div class="coords">
      <div class="coord">
        <span>X</span>
        <NumberField label="X position" value={settings.fixedX} min={-32000} max={32000} onchange={(v) => (settings.fixedX = Math.round(v))} />
      </div>
      <div class="coord">
        <span>Y</span>
        <NumberField label="Y position" value={settings.fixedY} min={-32000} max={32000} onchange={(v) => (settings.fixedY = Math.round(v))} />
      </div>
    </div>
    <button type="button" class="pick" onclick={onpick} disabled={picking}>
      <Icon name="crosshair" size={18} />
      {picking ? "Glorp is hiding… click the spot!" : "Pick a spot on screen"}
    </button>
  {/if}
</Card>

<style>
  .coords {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 8px;
    margin-top: 10px;
  }
  .coord {
    display: flex;
    align-items: center;
    gap: 6px;
  }
  .coord span {
    font-family: var(--font-display);
    font-weight: 700;
    color: var(--muted);
    width: 12px;
  }
  .coord :global(.field) {
    flex: 1;
  }
  .pick {
    margin-top: 10px;
    width: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    font: inherit;
    font-family: var(--font-display);
    font-weight: 600;
    font-size: 14px;
    color: var(--text);
    background: color-mix(in srgb, var(--accent-2) 22%, var(--surface));
    border: 2px solid color-mix(in srgb, var(--accent-2) 55%, transparent);
    border-radius: 16px;
    padding: 10px;
    cursor: pointer;
    transition: transform 0.15s, background 0.2s;
  }
  .pick:hover {
    background: color-mix(in srgb, var(--accent-2) 32%, var(--surface));
  }
  .pick:active {
    transform: scale(0.97);
  }
  .pick:disabled {
    cursor: default;
    opacity: 0.8;
  }
</style>
