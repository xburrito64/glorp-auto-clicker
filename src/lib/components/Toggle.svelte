<script lang="ts">
  let { checked = $bindable(), label, hint }: { checked: boolean; label: string; hint?: string } = $props();
</script>

<label class="row">
  <span class="text">
    <span class="label">{label}</span>
    {#if hint}<span class="hint">{hint}</span>{/if}
  </span>
  <input type="checkbox" bind:checked />
  <span class="track" aria-hidden="true"><span class="knob"></span></span>
</label>

<style>
  .row {
    position: relative;
    display: flex;
    align-items: center;
    gap: 12px;
    cursor: pointer;
    padding: 4px 0;
  }
  .text {
    flex: 1;
    display: flex;
    flex-direction: column;
  }
  .label {
    font-weight: 700;
    color: var(--text);
  }
  .hint {
    font-size: 12.5px;
    color: var(--text-soft);
  }
  input {
    position: absolute;
    opacity: 0;
    pointer-events: none;
  }
  .track {
    flex: none;
    width: 46px;
    height: 28px;
    border-radius: 999px;
    background: var(--surface-3);
    position: relative;
    transition: background 0.25s;
    box-shadow: inset 0 1px 3px rgb(var(--shadow) / 0.15);
  }
  .knob {
    position: absolute;
    top: 3px;
    left: 3px;
    width: 22px;
    height: 22px;
    border-radius: 50%;
    background: var(--surface);
    box-shadow: 0 2px 5px rgb(var(--shadow) / 0.25);
    transition: transform 0.35s cubic-bezier(0.34, 1.5, 0.5, 1), width 0.2s;
  }
  input:checked + .track {
    background: var(--primary);
  }
  input:checked + .track .knob {
    transform: translateX(18px);
  }
  .row:active .knob {
    width: 26px;
  }
  .row:active input:checked + .track .knob {
    transform: translateX(14px);
  }
  input:focus-visible + .track {
    outline: 3px solid color-mix(in srgb, var(--primary) 50%, transparent);
    outline-offset: 2px;
  }
</style>
