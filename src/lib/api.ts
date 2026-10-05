// Bridge to the Rust backend. When the UI runs in a normal browser (e.g. `npm run dev`
// without Tauri) a small mock stands in, so the design can be previewed anywhere.
import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { getCurrentWindow } from "@tauri-apps/api/window";
import {
  DEFAULT_SETTINGS,
  EMPTY_STATS,
  type ClickerState,
  type ClickerStats,
  type LifetimeStats,
  type Settings,
} from "./types";

export const inTauri = typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;

type Events = {
  "clicker-state": ClickerState;
  "clicker-stats": ClickerStats;
  "pick-result": { x: number; y: number } | null;
  "stats-updated": LifetimeStats;
};

export function on<K extends keyof Events>(event: K, cb: (payload: Events[K]) => void): Promise<UnlistenFn> {
  if (inTauri) return listen<Events[K]>(event, (e) => cb(e.payload));
  return mock.on(event, cb);
}

async function call<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  if (inTauri) return invoke<T>(cmd, args);
  return mock.call(cmd, args) as Promise<T>;
}

export const api = {
  getSettings: () => call<Settings>("get_settings"),
  updateSettings: (settings: Settings) => call<Settings>("update_settings", { settings }),
  start: () => call<boolean>("start_clicking"),
  stop: () => call<boolean>("stop_clicking"),
  isRunning: () => call<boolean>("is_running"),
  suspendHotkey: (suspend: boolean) => call<void>("suspend_hotkey", { suspend }),
  pickLocation: () => call<void>("pick_location"),
  getStats: () => call<LifetimeStats>("get_stats"),
  resetStats: () => call<LifetimeStats>("reset_stats"),
  recordPoke: () => call<number>("record_poke"),
  ready: () => call<void>("frontend_ready"),
  hide: () => call<void>("hide_window"),
  quit: () => call<void>("quit_app"),
  minimize: () => (inTauri ? getCurrentWindow().minimize() : Promise.resolve()),
};

// ---------------------------------------------------------------- browser mock

const mock = (() => {
  const listeners = new Map<string, Set<(p: any) => void>>();
  let settings: Settings = { ...DEFAULT_SETTINGS };
  let timer: ReturnType<typeof setInterval> | undefined;
  let clicks = 0;
  let startedAt = 0;

  // Some made-up history so the stats page has something to show in the preview.
  const day = (offset: number) => {
    const d = new Date();
    d.setDate(d.getDate() - offset);
    return `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, "0")}-${String(d.getDate()).padStart(2, "0")}`;
  };
  const stats: LifetimeStats = {
    ...EMPTY_STATS,
    totalClicks: 48_213,
    totalSecs: 2_950,
    sessions: 37,
    longestSessionSecs: 754,
    biggestSessionClicks: 12_000,
    bestCps: 99.97,
    leftClicks: 41_002,
    rightClicks: 6_811,
    middleClicks: 400,
    doubleClicks: 1_250,
    cpuSecs: 18.4,
    cpuCores: 16,
    accuracyWeighted: 2_949.1,
    accuracySecs: 2_950,
    pokes: 23,
    firstSessionAt: Date.now() / 1000 - 9 * 86_400,
    daily: { [day(6)]: 3_100, [day(5)]: 0, [day(4)]: 12_500, [day(3)]: 6_020, [day(2)]: 800, [day(1)]: 15_400, [day(0)]: 4_210 },
  };

  const emit = (event: string, payload: unknown) => listeners.get(event)?.forEach((cb) => cb(payload));
  const elapsed = () => (performance.now() - startedAt) / 1000;

  const stop = (reason: ClickerState["reason"]) => {
    if (!timer) return false;
    clearInterval(timer);
    timer = undefined;
    emit("clicker-stats", { clicks, cps: 0, avgCps: clicks / Math.max(elapsed(), 0.001), elapsedSecs: elapsed() });
    stats.totalClicks += clicks;
    stats.sessions += 1;
    stats.totalSecs += elapsed();
    stats.daily[day(0)] = (stats.daily[day(0)] ?? 0) + clicks;
    emit("stats-updated", { ...stats, daily: { ...stats.daily } });
    emit("clicker-state", { running: false, reason });
    return true;
  };

  const start = () => {
    if (timer) return false;
    clicks = 0;
    startedAt = performance.now();
    emit("clicker-state", { running: true, reason: "started" });
    const cps = 1000 / settings.intervalMs;
    timer = setInterval(() => {
      clicks = Math.floor(elapsed() * cps) + 1;
      if (settings.repeatMode === "count" && clicks >= settings.repeatCount) {
        clicks = settings.repeatCount;
        stop("done");
        return;
      }
      if (settings.repeatMode === "time" && elapsed() >= settings.durationSecs) {
        stop("done");
        return;
      }
      emit("clicker-stats", { clicks, cps: cps * (0.95 + Math.random() * 0.1), avgCps: cps, elapsedSecs: elapsed() });
    }, Math.max(150, settings.intervalMs));
    return true;
  };

  return {
    on(event: string, cb: (p: any) => void): Promise<UnlistenFn> {
      if (!listeners.has(event)) listeners.set(event, new Set());
      listeners.get(event)!.add(cb);
      return Promise.resolve(() => listeners.get(event)?.delete(cb));
    },
    async call(cmd: string, args?: Record<string, unknown>): Promise<unknown> {
      switch (cmd) {
        case "get_settings":
          return { ...settings };
        case "update_settings": {
          const next = args!.settings as Settings;
          if (next.hotkey === "F1") throw "F1 is already used by another app";
          settings = { ...next };
          return { ...settings };
        }
        case "start_clicking":
          return start();
        case "stop_clicking":
          return stop("stopped");
        case "is_running":
          return !!timer;
        case "get_stats":
          return { ...stats, daily: { ...stats.daily } };
        case "reset_stats":
          Object.assign(stats, { ...EMPTY_STATS, daily: {} });
          return { ...stats };
        case "record_poke":
          return ++stats.pokes;
        case "pick_location":
          setTimeout(() => emit("pick-result", { x: 640 + Math.round(Math.random() * 400), y: 360 + Math.round(Math.random() * 300) }), 1200);
          return;
        default:
          return;
      }
    },
  };
})();
