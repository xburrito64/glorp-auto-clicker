<script lang="ts">
  import { api } from "$lib/api";
  import Icon from "./Icon.svelte";

  let { running }: { running: boolean } = $props();
</script>

<header class="titlebar" data-tauri-drag-region>
  <div class="brand">
    <img src="/favicon.png" alt="" width="22" height="22" />
    <span class="name">glorp clicker</span>
    {#if running}<span class="live">clicking</span>{/if}
  </div>
  <div class="buttons">
    <button type="button" onclick={() => api.minimize()} title="Minimize" aria-label="Minimize">
      <Icon name="minus" size={16} />
    </button>
    <button type="button" class="close" onclick={() => api.hide()} title="Hide to tray" aria-label="Hide to tray">
      <Icon name="close" size={16} />
    </button>
  </div>
</header>

<style>
  .titlebar {
    height: 42px;
    flex: none;
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 8px 0 14px;
    user-select: none;
  }
  .brand {
    display: flex;
    align-items: center;
    gap: 8px;
    pointer-events: none;
  }
  .name {
    font-family: var(--font-display);
    font-weight: 600;
    font-size: 15px;
    color: var(--text);
    letter-spacing: 0.2px;
  }
  .live {
    font-family: var(--font-display);
    font-size: 11px;
    font-weight: 600;
    color: var(--primary-text);
    background: var(--stop);
    padding: 2px 8px;
    border-radius: 999px;
    animation: live 1.2s ease-in-out infinite;
  }
  .buttons {
    display: flex;
    gap: 4px;
  }
  button {
    width: 32px;
    height: 32px;
    display: grid;
    place-items: center;
    border: 0;
    border-radius: 11px;
    background: transparent;
    color: var(--text-soft);
    cursor: pointer;
    transition: background 0.15s, color 0.15s, transform 0.12s;
  }
  button:hover {
    background: var(--surface-3);
    color: var(--text);
  }
  button.close:hover {
    background: var(--stop);
    color: #fff;
  }
  button:active {
    transform: scale(0.88);
  }
  @keyframes live {
    0%, 100% { opacity: 1; }
    50% { opacity: 0.6; }
  }
</style>
