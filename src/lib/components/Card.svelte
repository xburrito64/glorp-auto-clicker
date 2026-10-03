<script lang="ts">
  import type { Snippet } from "svelte";
  import Icon, { type IconName } from "./Icon.svelte";

  let { title, icon, hint, children }: { title: string; icon: IconName; hint?: string; children: Snippet } = $props();
</script>

<section class="card">
  <h3><span class="icon"><Icon name={icon} size={15} /></span>{title}</h3>
  {@render children()}
  {#if hint}<p class="hint">{hint}</p>{/if}
</section>

<style>
  .card {
    background: var(--surface);
    border-radius: 22px;
    padding: 14px 14px 14px;
    box-shadow: 0 1px 0 var(--border), 0 6px 18px -8px rgb(var(--shadow) / 0.18);
    border: 1px solid var(--border);
  }
  h3 {
    margin: 0 0 10px 2px;
    display: flex;
    align-items: center;
    gap: 8px;
    font-family: var(--font-display);
    font-weight: 600;
    font-size: 15px;
    color: var(--text);
  }
  .icon {
    display: grid;
    place-items: center;
    width: 26px;
    height: 26px;
    border-radius: 9px;
    background: color-mix(in srgb, var(--primary) 18%, var(--surface));
    color: var(--primary-2);
  }
  .hint {
    margin: 8px 4px 0;
    font-size: 12.5px;
    line-height: 1.4;
    color: var(--text-soft);
  }
</style>
