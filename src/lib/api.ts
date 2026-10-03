// Bridge to the Rust backend. When the UI runs in a normal browser (e.g. `npm run dev`
// without Tauri) a small mock stands in, so the design can be previewed anywhere.
import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { DEFAULT_SETTINGS, type ClickerState, type ClickerStats, type Settings } from "./types";

export const inTauri = typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;

type Events = {
  "clicker-state": ClickerState;
  "clicker-stats": ClickerStats;
  "pick-result": { x: number; y: number } | null;
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

  const emit = (event: string, payload: unknown) => listeners.get(event)?.forEach((cb) => cb(payload));

  const stop = (reason: ClickerState["reason"]) => {
    if (!timer) return false;
    clearInterval(timer);
    timer = undefined;
    emit("clicker-stats", { clicks, cps: 0 });
    emit("clicker-state", { running: false, reason });
    return true;
  };

  const start = () => {
    if (timer) return false;
    clicks = 0;
    emit("clicker-state", { running: true, reason: "started" });
    const cps = 1000 / settings.intervalMs;
    let last = performance.now();
    timer = setInterval(() => {
      const now = performance.now();
      clicks += Math.max(1, Math.round(((now - last) / 1000) * cps));
      last = now;
      if (settings.repeatMode === "count" && clicks >= settings.repeatCount) {
        clicks = settings.repeatCount;
        stop("done");
        return;
      }
      emit("clicker-stats", { clicks, cps: cps * (0.95 + Math.random() * 0.1) });
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
        case "pick_location":
          setTimeout(() => emit("pick-result", { x: 640 + Math.round(Math.random() * 400), y: 360 + Math.round(Math.random() * 300) }), 1200);
          return;
        default:
          return;
      }
    },
  };
})();
