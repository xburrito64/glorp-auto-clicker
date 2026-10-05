export type MouseButton = "left" | "right" | "middle";
export type ClickKind = "single" | "double";
export type RepeatMode = "forever" | "count" | "time";
export type PositionMode = "cursor" | "fixed";
export type IntervalUnit = "ms" | "s" | "cps";
export type DurationUnit = "s" | "min" | "h";

/** Mirrors `Settings` in src-tauri/src/settings.rs. */
export interface Settings {
  intervalMs: number;
  intervalUnit: IntervalUnit;
  jitterPct: number;
  button: MouseButton;
  clickKind: ClickKind;
  repeatMode: RepeatMode;
  repeatCount: number;
  durationSecs: number;
  durationUnit: DurationUnit;
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
  durationSecs: 60,
  durationUnit: "min",
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
  /** Clicks/sec over the last moment. */
  cps: number;
  /** Average clicks/sec since the run started. */
  avgCps: number;
  elapsedSecs: number;
}

/** Mirrors `LifetimeStats` in src-tauri/src/stats.rs. */
export interface LifetimeStats {
  totalClicks: number;
  totalSecs: number;
  sessions: number;
  longestSessionSecs: number;
  biggestSessionClicks: number;
  bestCps: number;
  leftClicks: number;
  rightClicks: number;
  middleClicks: number;
  doubleClicks: number;
  cpuSecs: number;
  cpuCores: number;
  accuracyWeighted: number;
  accuracySecs: number;
  pokes: number;
  firstSessionAt: number | null;
  daily: Record<string, number>;
}

export const EMPTY_STATS: LifetimeStats = {
  totalClicks: 0,
  totalSecs: 0,
  sessions: 0,
  longestSessionSecs: 0,
  biggestSessionClicks: 0,
  bestCps: 0,
  leftClicks: 0,
  rightClicks: 0,
  middleClicks: 0,
  doubleClicks: 0,
  cpuSecs: 0,
  cpuCores: 1,
  accuracyWeighted: 0,
  accuracySecs: 0,
  pokes: 0,
  firstSessionAt: null,
  daily: {},
};

export interface ClickerState {
  running: boolean;
  reason: "started" | "stopped" | "done";
}

export type Mood = "idle" | "countdown" | "clicking" | "celebrate" | "sleepy" | "poked" | "sad";
