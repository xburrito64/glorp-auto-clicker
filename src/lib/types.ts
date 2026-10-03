export type MouseButton = "left" | "right" | "middle";
export type ClickKind = "single" | "double";
export type RepeatMode = "forever" | "count";
export type PositionMode = "cursor" | "fixed";
export type IntervalUnit = "ms" | "s" | "cps";

/** Mirrors `Settings` in src-tauri/src/settings.rs. */
export interface Settings {
  intervalMs: number;
  intervalUnit: IntervalUnit;
  jitterPct: number;
  button: MouseButton;
  clickKind: ClickKind;
  repeatMode: RepeatMode;
  repeatCount: number;
  positionMode: PositionMode;
  fixedX: number;
  fixedY: number;
  hotkey: string;
  theme: string;
  autostart: boolean;
  startMinimized: boolean;
}

export const DEFAULT_SETTINGS: Settings = {
  intervalMs: 100,
  intervalUnit: "ms",
  jitterPct: 0,
  button: "left",
  clickKind: "single",
  repeatMode: "forever",
  repeatCount: 100,
  positionMode: "cursor",
  fixedX: 0,
  fixedY: 0,
  hotkey: "F6",
  theme: "glorp",
  autostart: false,
  startMinimized: false,
};

export interface ClickerStats {
  clicks: number;
  cps: number;
}

export interface ClickerState {
  running: boolean;
  reason: "started" | "stopped" | "done";
}

export type Mood = "idle" | "countdown" | "clicking" | "celebrate" | "sleepy" | "poked" | "sad";
