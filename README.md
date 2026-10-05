# Glorp Clicker

A cute auto clicker for Windows, starring **Glorp** the green slime alien cat.

Built with [Tauri 2](https://tauri.app) (Rust backend) and [Svelte 5](https://svelte.dev) (UI).

## Download

**[⬇ Get the latest installer](https://github.com/xburrito64/glorp-auto-clicker/releases/latest)**: run the
`setup.exe`, and Glorp appears in your Start menu. If Windows shows "Windows protected your PC", click
**More info → Run anyway** (the app just isn't code-signed).

## Features

- Click interval in ms, seconds or clicks-per-second, with speed presets
- "Wobble": random timing variation, so clicks look more human
- Left / middle / right button, single or double click
- Click forever, a set number of times, or for a set amount of time
- **Drift-free timing**: clicks follow a fixed timetable and course-correct after every click,
  so 100 CPS really is 100 CPS, even when Windows lags or Glorp is hidden in the tray
- **Stats page**: total clicks, time spent clicking, sessions, top speed, timing accuracy,
  CPU use, clicks per day, and some fun facts
- Follow the mouse, or click a fixed spot (with an on-screen picker)
- Global start/stop hotkey (default **F6**), can be changed in the app
- Closing the window hides Glorp in the system tray (the hidden icons next to the clock)
- Start with Windows, and optionally start hidden in the tray
- Themes: Slime Meadow, Space Goo, Bubblegum
- Safety: Glorp never clicks on his own window

## Project layout

```
src/                      Svelte UI
  routes/+page.svelte     main screen (state, start/stop, countdown)
  lib/themes/             theme registry + mascots (add new themes here)
  lib/themes/glorp/       Glorp mascot component (animated SVG)
  lib/panels/             Speed / Mouse / Glorp tabs
  lib/components/         buttons, toggles, inputs...
  lib/api.ts              bridge to Rust (with a browser mock for UI previews)
src-tauri/src/
  clicker.rs              the click engine (background thread, precise timing)
  input.rs                Win32 SendInput / cursor helpers
  settings.rs             settings + saving to %APPDATA%\com.glorp.clicker
  stats.rs                lifetime stats (stats.json in the same folder)
  lib.rs                  tray, hotkey, window, commands
scripts/make-icons.mjs    renders Glorp into the app + tray icons
```

## Commands

```bash
npm install          # once
npm run tauri dev    # run the app with hot reload
npm run dev          # UI only in the browser (mock backend) at http://localhost:1420
npm run tauri build  # build the installer
npm run icons        # re-render icons after changing Glorp's look
```

After `npm run tauri build`, the installer is in `src-tauri/target/release/bundle/nsis/`.

Tip: if the project lives in a synced folder (OneDrive, Dropbox), move the multi-GB Rust build
output elsewhere with a local, git-ignored `src-tauri/.cargo/config.toml`:

```toml
[build]
target-dir = "C:/some/folder/outside/onedrive"
```

## Timing tests & diagnostics

```bash
cd src-tauri
cargo test --lib -- --nocapture                    # engine accuracy tests (with simulated lag)
cargo test --lib naive -- --ignored --nocapture    # how a naive clicker drifts, for comparison
cargo test --lib latency -- --ignored --nocapture  # how long each Windows call takes on this PC
```

Start the app with the environment variable `GLORP_TIMING_LOG=1` to get a `timing-log.txt` in
`%APPDATA%\com.glorp.clicker` after each run (actual vs target speed, oversleep, time per step).

## Adding a theme

Open `src/lib/themes/index.ts`, copy a theme object, give it a new `id`/`name` and change its
colors. The `--glorp-*` colors recolor the mascot. For a whole new character, create a component
like `lib/themes/glorp/Glorp.svelte` (it receives `mood`, `cps` and `look`) and set it as the
theme's `mascot`.
