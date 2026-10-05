<script lang="ts" module>
  const paths = {
    clock: "M12 7v5l3 2 M12 21a9 9 0 1 0 0-18 9 9 0 0 0 0 18z",
    mouse: "M12 3a6 6 0 0 1 6 6v6a6 6 0 0 1-12 0V9a6 6 0 0 1 6-6z M12 7v3",
    sparkle: "M12 3l1.8 5.2L19 10l-5.2 1.8L12 17l-1.8-5.2L5 10l5.2-1.8z M19 16l.7 2.3L22 19l-2.3.7L19 22l-.7-2.3L16 19l2.3-.7z",
    repeat: "M17 2l3 3-3 3 M4 11V9a4 4 0 0 1 4-4h12 M7 22l-3-3 3-3 M20 13v2a4 4 0 0 1-4 4H4",
    target: "M12 21a9 9 0 1 0 0-18 9 9 0 0 0 0 18z M12 16a4 4 0 1 0 0-8 4 4 0 0 0 0 8z M12 12h.01",
    wave: "M2 12c2-4 4-4 6 0s4 4 6 0 4-4 6 0",
    keyboard: "M3 7a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z M7 9h.01 M11 9h.01 M15 9h.01 M7 13h.01 M17 13h.01 M9 16h6",
    palette: "M12 21a9 9 0 1 1 9-9c0 2.5-2 3-3.5 3H16a2 2 0 0 0-1.5 3.3A1.6 1.6 0 0 1 12 21z M7.5 11.5h.01 M10 7.5h.01 M14.5 7.5h.01",
    power: "M12 3v8 M6.3 6.3a8 8 0 1 0 11.4 0",
    minus: "M6 12h12",
    close: "M7 7l10 10 M17 7L7 17",
    crosshair: "M12 3v4 M12 17v4 M3 12h4 M17 12h4 M12 15a3 3 0 1 0 0-6 3 3 0 0 0 0 6z",
    play: "M8 5.5v13l10.5-6.5z",
    stop: "M7 7h10v10H7z",
    tray: "M4 14h4l2 3h4l2-3h4 M4 14V6a2 2 0 0 1 2-2h12a2 2 0 0 1 2 2v8 M4 14v4a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-4",
    cursor: "M5 3l14 7-6 2-2 6z",
    chart: "M5 20v-6 M12 20V5 M19 20v-10",
    trophy: "M8 4h8v5a4 4 0 0 1-8 0z M8 6H5a3 3 0 0 0 3 4 M16 6h3a3 3 0 0 1-3 4 M12 13v4 M9 20h6",
    calendar: "M4 7a2 2 0 0 1 2-2h12a2 2 0 0 1 2 2v12a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2z M4 10h16 M9 3v4 M15 3v4",
  } as const;
  export type IconName = keyof typeof paths;
</script>

<script lang="ts">
  let { name, size = 18, filled = false }: { name: IconName; size?: number; filled?: boolean } = $props();
</script>

<svg
  width={size}
  height={size}
  viewBox="0 0 24 24"
  fill={filled ? "currentColor" : "none"}
  stroke="currentColor"
  stroke-width="2.2"
  stroke-linecap="round"
  stroke-linejoin="round"
  aria-hidden="true"
>
  <path d={paths[name]} />
</svg>
