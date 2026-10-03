<script lang="ts">
  let {
    value,
    onchange,
    min = 0,
    max = Number.MAX_SAFE_INTEGER,
    step = 1,
    suffix,
    label,
    decimals = 0,
  }: {
    value: number;
    onchange: (v: number) => void;
    min?: number;
    max?: number;
    step?: number;
    suffix?: string;
    label?: string;
    decimals?: number;
  } = $props();

  const fmt = (v: number) => String(Number(v.toFixed(decimals)));
  let text = $state("");
  let focused = $state(false);

  // Keep the text in sync with the outside value, except while the user is typing.
  $effect(() => {
    const v = fmt(value);
    if (!focused) text = v;
  });

  function commit() {
    const n = Number(text.replace(",", "."));
    if (Number.isFinite(n)) {
      const clamped = Math.min(max, Math.max(min, n));
      onchange(clamped);
      text = fmt(clamped);
    } else {
      text = fmt(value);
    }
  }

  function nudge(dir: 1 | -1) {
    const next = Math.min(max, Math.max(min, Number((value + dir * step).toFixed(decimals))));
    onchange(next);
  }
</script>

<div class="field" class:focused>
  <button type="button" class="step" onclick={() => nudge(-1)} aria-label="decrease">−</button>
  <input
    type="text"
    inputmode="decimal"
    aria-label={label}
    bind:value={text}
    onfocus={() => (focused = true)}
    onblur={() => {
      focused = false;
      commit();
    }}
    oninput={() => {
      const n = Number(text.replace(",", "."));
      if (text !== "" && Number.isFinite(n) && n >= min && n <= max) onchange(n);
    }}
    onkeydown={(e) => {
      if (e.key === "Enter") (e.currentTarget as HTMLInputElement).blur();
      if (e.key === "ArrowUp") (e.preventDefault(), nudge(1));
      if (e.key === "ArrowDown") (e.preventDefault(), nudge(-1));
    }}
  />
  {#if suffix}<span class="suffix">{suffix}</span>{/if}
  <button type="button" class="step" onclick={() => nudge(1)} aria-label="increase">+</button>
</div>

<style>
  .field {
    display: flex;
    align-items: center;
    background: var(--surface-2);
    border: 2px solid var(--border);
    border-radius: 16px;
    padding: 3px;
    transition: border-color 0.2s, box-shadow 0.2s;
    min-width: 0;
  }
  .field.focused {
    border-color: var(--primary);
    box-shadow: 0 0 0 4px color-mix(in srgb, var(--primary) 20%, transparent);
  }
  input {
    flex: 1;
    min-width: 0;
    width: 100%;
    font: inherit;
    font-family: var(--font-display);
    font-weight: 600;
    font-size: 17px;
    text-align: center;
    color: var(--text);
    background: none;
    border: 0;
    outline: none;
    padding: 4px 0;
  }
  .suffix {
    font-family: var(--font-display);
    font-size: 13px;
    color: var(--muted);
    padding-right: 6px;
  }
  .step {
    flex: none;
    width: 30px;
    height: 30px;
    border-radius: 12px;
    border: 0;
    background: var(--surface);
    color: var(--text-soft);
    font-size: 18px;
    font-weight: 700;
    line-height: 1;
    cursor: pointer;
    box-shadow: 0 1px 3px rgb(var(--shadow) / 0.12);
    transition: transform 0.12s, background 0.2s, color 0.2s;
  }
  .step:hover {
    color: var(--primary-2);
  }
  .step:active {
    transform: scale(0.88);
  }
</style>
