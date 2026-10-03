<script lang="ts">
  import { api } from "$lib/api";
  import { hotkeyFromEvent, hotkeyParts } from "$lib/hotkeys";

  let {
    value,
    onchange,
  }: {
    value: string;
    /** Should apply the hotkey and return an error message if it couldn't be used. */
    onchange: (accel: string) => Promise<string | null>;
  } = $props();

  let recording = $state(false);
  let error = $state<string | null>(null);
  let btn: HTMLButtonElement;

  async function begin() {
    if (recording) return;
    error = null;
    recording = true;
    await api.suspendHotkey(true).catch(() => {});
  }

  async function end() {
    if (!recording) return;
    recording = false;
    await api.suspendHotkey(false).catch(() => {});
  }

  async function onkeydown(e: KeyboardEvent) {
    if (!recording) return;
    e.preventDefault();
    e.stopPropagation();
    if (e.code === "Escape") {
      await end();
      btn.blur();
      return;
    }
    const res = hotkeyFromEvent(e);
    if (!res) return;
    if (!res.ok) {
      error = res.reason;
      return;
    }
    error = await onchange(res.accel);
    await end();
    btn.blur();
  }
</script>

<div class="wrap">
  <button
    bind:this={btn}
    type="button"
    class="hotkey"
    class:recording
    onclick={begin}
    onblur={end}
    {onkeydown}
    aria-label="Change start/stop hotkey"
  >
    {#if recording}
      <span class="listening"><span class="dot"></span>press your combo…</span>
    {:else}
      <span class="keys">
        {#each hotkeyParts(value) as part, i (i)}
          {#if i > 0}<span class="plus">+</span>{/if}
          <kbd>{part}</kbd>
        {/each}
      </span>
      <span class="change">change</span>
    {/if}
  </button>
  {#if error}
    <p class="error">{error}</p>
  {:else if recording}
    <p class="hint">Esc to cancel</p>
  {/if}
</div>

<style>
  .hotkey {
    width: 100%;
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 8px;
    font: inherit;
    background: var(--surface-2);
    border: 2px dashed var(--border);
    border-radius: 16px;
    padding: 10px 12px;
    cursor: pointer;
    color: var(--text);
    transition: border-color 0.2s, background 0.2s, transform 0.15s;
    min-height: 52px;
  }
  .hotkey:hover {
    border-color: var(--primary);
  }
  .hotkey:active {
    transform: scale(0.98);
  }
  .hotkey.recording {
    border-style: solid;
    border-color: var(--accent);
    background: color-mix(in srgb, var(--accent) 12%, var(--surface));
    justify-content: center;
  }
  .keys {
    display: flex;
    align-items: center;
    gap: 5px;
  }
  .plus {
    color: var(--muted);
    font-weight: 700;
  }
  .change {
    font-family: var(--font-display);
    font-size: 13px;
    color: var(--muted);
  }
  .listening {
    display: flex;
    align-items: center;
    gap: 8px;
    font-family: var(--font-display);
    font-weight: 600;
    color: var(--text-soft);
  }
  .dot {
    width: 10px;
    height: 10px;
    border-radius: 50%;
    background: var(--accent);
    animation: pulse 1s ease-in-out infinite;
  }
  .error,
  .hint {
    margin: 6px 4px 0;
    font-size: 12.5px;
    color: var(--text-soft);
  }
  .error {
    color: var(--stop-2);
    font-weight: 700;
  }
  @keyframes pulse {
    0%, 100% { transform: scale(0.7); opacity: 0.5; }
    50% { transform: scale(1.1); opacity: 1; }
  }
</style>
