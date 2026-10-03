<script lang="ts" generics="T extends string">
  let {
    options,
    value = $bindable(),
    small = false,
    label,
  }: { options: { value: T; label: string }[]; value: T; small?: boolean; label?: string } = $props();

  const index = $derived(Math.max(0, options.findIndex((o) => o.value === value)));
</script>

<div class="seg" class:small role="radiogroup" aria-label={label} style="--n:{options.length}; --i:{index}">
  <span class="pill" aria-hidden="true"></span>
  {#each options as opt (opt.value)}
    <button
      type="button"
      role="radio"
      aria-checked={opt.value === value}
      class:on={opt.value === value}
      onclick={() => (value = opt.value)}
    >
      {opt.label}
    </button>
  {/each}
</div>

<style>
  .seg {
    position: relative;
    display: grid;
    grid-template-columns: repeat(var(--n), 1fr);
    background: var(--surface-3);
    border-radius: 999px;
    padding: 4px;
    isolation: isolate;
  }
  .pill {
    position: absolute;
    top: 4px;
    bottom: 4px;
    left: 4px;
    width: calc((100% - 8px) / var(--n));
    transform: translateX(calc(100% * var(--i)));
    background: var(--surface);
    border-radius: 999px;
    box-shadow: 0 2px 6px rgb(var(--shadow) / 0.14), 0 0 0 1px var(--border);
    transition: transform 0.35s cubic-bezier(0.34, 1.4, 0.5, 1);
    z-index: -1;
  }
  button {
    font: inherit;
    font-family: var(--font-display);
    font-weight: 600;
    font-size: 14px;
    color: var(--text-soft);
    background: none;
    border: 0;
    padding: 8px 6px;
    border-radius: 999px;
    cursor: pointer;
    transition: color 0.2s, transform 0.15s;
    white-space: nowrap;
  }
  button:active {
    transform: scale(0.94);
  }
  button.on {
    color: var(--text);
  }
  .small button {
    font-size: 13px;
    padding: 6px 4px;
  }
</style>
