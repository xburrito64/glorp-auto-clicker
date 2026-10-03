<script lang="ts">
  import Card from "$lib/components/Card.svelte";
  import HotkeyInput from "$lib/components/HotkeyInput.svelte";
  import Icon from "$lib/components/Icon.svelte";
  import Toggle from "$lib/components/Toggle.svelte";
  import { themes } from "$lib/themes";
  import type { Settings } from "$lib/types";

  let {
    settings = $bindable(),
    onhotkey,
    onquit,
  }: {
    settings: Settings;
    onhotkey: (accel: string) => Promise<string | null>;
    onquit: () => void;
  } = $props();

  const varsStyle = (vars: Record<string, string>) =>
    Object.entries(vars)
      .map(([k, v]) => `--${k}:${v}`)
      .join(";");
</script>

<Card title="Start / stop hotkey" icon="keyboard" hint="Works anywhere, even while Glorp naps in the tray.">
  <HotkeyInput value={settings.hotkey} onchange={onhotkey} />
</Card>

<Card title="Theme" icon="palette">
  <div class="themes">
    {#each themes as t (t.id)}
      {@const Mascot = t.mascot}
      <button
        type="button"
        class="theme"
        class:on={settings.theme === t.id}
        style={varsStyle(t.vars)}
        onclick={() => (settings.theme = t.id)}
        title={t.blurb}
      >
        <span class="preview">
          <span class="mini"><Mascot mood="idle" cps={0} look={{ x: 0, y: 0 }} /></span>
        </span>
        <span class="tname">{t.name}</span>
      </button>
    {/each}
  </div>
</Card>

<Card title="Startup" icon="power">
  <Toggle bind:checked={settings.autostart} label="Start with Windows" hint="Glorp wakes up when your PC does" />
  <Toggle bind:checked={settings.startMinimized} label="Start hidden in tray" hint="Skip the window, just the tray icon" />
</Card>

<div class="footer">
  <p><Icon name="tray" size={15} /> Closing the window tucks Glorp into your tray, down by the clock.</p>
  <button type="button" class="quit" onclick={onquit}>Quit Glorp for real</button>
</div>

<style>
  .themes {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 8px;
  }
  .theme {
    font: inherit;
    border: 2px solid var(--border);
    border-radius: 18px;
    padding: 6px 6px 8px;
    background: var(--surface);
    cursor: pointer;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 6px;
    transition: transform 0.15s, border-color 0.2s, box-shadow 0.2s;
  }
  .theme:hover {
    transform: translateY(-2px);
  }
  .theme:active {
    transform: scale(0.95);
  }
  .theme.on {
    border-color: var(--primary);
    box-shadow: 0 0 0 3px color-mix(in srgb, var(--primary) 35%, transparent);
  }
  .preview {
    width: 100%;
    height: 64px;
    border-radius: 13px;
    background: linear-gradient(160deg, var(--hero-1), var(--hero-2));
    display: flex;
    align-items: flex-end;
    justify-content: center;
    overflow: hidden;
  }
  .mini {
    width: 58px;
    height: 58px;
  }
  .tname {
    font-family: var(--font-display);
    font-weight: 600;
    font-size: 12.5px;
    color: var(--text);
  }
  .footer {
    text-align: center;
    padding: 4px 8px 8px;
  }
  .footer p {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 6px;
    margin: 0 0 10px;
    font-size: 12.5px;
    color: var(--text-soft);
  }
  .quit {
    font: inherit;
    font-family: var(--font-display);
    font-weight: 600;
    font-size: 13px;
    color: var(--stop-2);
    background: transparent;
    border: 2px solid color-mix(in srgb, var(--stop) 45%, transparent);
    border-radius: 999px;
    padding: 6px 16px;
    cursor: pointer;
    transition: background 0.2s, transform 0.15s;
  }
  .quit:hover {
    background: color-mix(in srgb, var(--stop) 12%, transparent);
  }
  .quit:active {
    transform: scale(0.95);
  }
</style>
