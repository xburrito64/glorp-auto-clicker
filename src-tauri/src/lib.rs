mod clicker;
mod input;
mod settings;
mod stats;

use serde::Serialize;
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};
use tauri::{
    image::Image,
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Emitter, Manager, WindowEvent, Wry,
};
use tauri_plugin_autostart::ManagerExt as _;
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};

use clicker::Clicker;
use settings::Settings;
use stats::{LifetimeStats, Session};

const TRAY_ID: &str = "glorp-tray";
const AUTOSTART_FLAG: &str = "--minimized";
const TRAY_IDLE: &[u8] = include_bytes!("../icons/tray-idle.png");
const TRAY_ACTIVE: &[u8] = include_bytes!("../icons/tray-active.png");

struct AppState {
    settings: Mutex<Settings>,
    stats: Mutex<LifetimeStats>,
    clicker: Clicker,
    /// Set while the user is picking a screen position (hotkey is ignored meanwhile).
    picking: Arc<AtomicBool>,
    /// Set while the UI is recording a new hotkey.
    hotkey_suspended: AtomicBool,
    launched_minimized: bool,
    shown_once: AtomicBool,
    tray_toggle: Mutex<Option<MenuItem<Wry>>>,
}

#[derive(Clone, Serialize)]
struct StatePayload {
    running: bool,
    reason: &'static str,
}

#[derive(Clone, Serialize)]
struct PickPayload {
    x: i32,
    y: i32,
}

/// Tells the UI and the tray that the clicker started/stopped.
pub(crate) fn notify_state(app: &AppHandle, running: bool, reason: &'static str) {
    let _ = app.emit("clicker-state", StatePayload { running, reason });
    refresh_tray(app, running);
}

/// Adds a finished run to the lifetime stats, saves them, and tells the UI.
pub(crate) fn record_session(app: &AppHandle, session: Session) {
    let state = app.state::<AppState>();
    let snapshot = {
        let mut stats = state.stats.lock().unwrap();
        stats.record(&session);
        stats.clone()
    };
    let _ = stats::save(app, &snapshot);
    let _ = app.emit("stats-updated", snapshot);
}

fn refresh_tray(app: &AppHandle, running: bool) {
    let state = app.state::<AppState>();
    let hotkey = state.settings.lock().unwrap().hotkey.clone();
    if let Some(tray) = app.tray_by_id(TRAY_ID) {
        let bytes = if running { TRAY_ACTIVE } else { TRAY_IDLE };
        if let Ok(icon) = Image::from_bytes(bytes) {
            let _ = tray.set_icon(Some(icon));
        }
        let tip = if running {
            "Glorp Clicker · glorping!"
        } else {
            "Glorp Clicker · resting"
        };
        let _ = tray.set_tooltip(Some(tip));
    }
    let item = state.tray_toggle.lock().unwrap().clone();
    if let Some(item) = item {
        let label = if running { "Stop clicking" } else { "Start clicking" };
        let _ = item.set_text(format!("{label}\t{}", pretty_hotkey(&hotkey)));
    }
}

fn pretty_hotkey(hotkey: &str) -> String {
    hotkey
        .split('+')
        .map(|part| {
            let p = part.trim();
            if let Some(rest) = p.strip_prefix("Key") {
                rest.to_string()
            } else if let Some(rest) = p.strip_prefix("Digit") {
                rest.to_string()
            } else {
                p.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("+")
}

fn main_window_handle(app: &AppHandle) -> isize {
    app.get_webview_window("main")
        .and_then(|w| w.hwnd().ok())
        .map(|h| h.0 as isize)
        .unwrap_or(0)
}

fn show_main(app: &AppHandle) {
    if let Some(w) = app.get_webview_window("main") {
        let _ = w.unminimize();
        let _ = w.show();
        let _ = w.set_focus();
    }
}

fn start(app: &AppHandle) -> bool {
    let state = app.state::<AppState>();
    let settings = state.settings.lock().unwrap().clone();
    let started = state
        .clicker
        .start(app.clone(), settings, main_window_handle(app));
    if started {
        notify_state(app, true, "started");
    }
    started
}

fn stop(app: &AppHandle) -> bool {
    let stopped = app.state::<AppState>().clicker.stop();
    if stopped {
        notify_state(app, false, "stopped");
    }
    stopped
}

fn toggle(app: &AppHandle) -> bool {
    if app.state::<AppState>().clicker.is_running() {
        stop(app);
        false
    } else {
        start(app)
    }
}

fn register_hotkey(app: &AppHandle, hotkey: &str) -> Result<(), String> {
    let shortcut: Shortcut = hotkey
        .parse()
        .map_err(|_| format!("\"{hotkey}\" isn't a key combo Glorp understands"))?;
    let gs = app.global_shortcut();
    gs.unregister_all().map_err(|e| e.to_string())?;
    gs.register(shortcut)
        .map_err(|_| format!("{} is already used by another app", pretty_hotkey(hotkey)))
}

// ---------------------------------------------------------------- commands

#[tauri::command]
fn get_settings(state: tauri::State<AppState>) -> Settings {
    state.settings.lock().unwrap().clone()
}

#[tauri::command]
fn update_settings(app: AppHandle, settings: Settings) -> Result<Settings, String> {
    let state = app.state::<AppState>();
    let new = settings.sanitized();
    let old = state.settings.lock().unwrap().clone();

    if new.hotkey != old.hotkey {
        if let Err(e) = register_hotkey(&app, &new.hotkey) {
            // Put the old hotkey back so the user is never left without one.
            let _ = register_hotkey(&app, &old.hotkey);
            return Err(e);
        }
    }
    if new.autostart != old.autostart {
        let al = app.autolaunch();
        let res = if new.autostart { al.enable() } else { al.disable() };
        res.map_err(|e| format!("Couldn't change start-with-Windows: {e}"))?;
    }

    *state.settings.lock().unwrap() = new.clone();
    settings::save(&app, &new)?;
    if new.hotkey != old.hotkey {
        refresh_tray(&app, state.clicker.is_running());
    }
    Ok(new)
}

#[tauri::command]
fn start_clicking(app: AppHandle) -> bool {
    start(&app)
}

#[tauri::command]
fn stop_clicking(app: AppHandle) -> bool {
    stop(&app)
}

#[tauri::command]
fn is_running(state: tauri::State<AppState>) -> bool {
    state.clicker.is_running()
}

/// While the UI records a new hotkey, the current one must not fire.
#[tauri::command]
fn suspend_hotkey(app: AppHandle, suspend: bool) -> Result<(), String> {
    let state = app.state::<AppState>();
    state.hotkey_suspended.store(suspend, Ordering::Relaxed);
    if suspend {
        app.global_shortcut()
            .unregister_all()
            .map_err(|e| e.to_string())
    } else {
        let hotkey = state.settings.lock().unwrap().hotkey.clone();
        register_hotkey(&app, &hotkey)
    }
}

/// Hides the window, waits for a left click anywhere, then reports the point via
/// the `pick-result` event (payload is `null` when cancelled with Esc).
#[tauri::command]
fn pick_location(app: AppHandle) {
    let state = app.state::<AppState>();
    if state.picking.swap(true, Ordering::Relaxed) {
        return;
    }
    stop(&app);
    let picking = Arc::clone(&state.picking);
    if let Some(w) = app.get_webview_window("main") {
        let _ = w.hide();
    }
    thread::spawn(move || {
        let cancel = AtomicBool::new(false);
        let point = input::pick_point(&cancel, Duration::from_secs(60));
        picking.store(false, Ordering::Relaxed);
        show_main(&app);
        let _ = app.emit("pick-result", point.map(|(x, y)| PickPayload { x, y }));
    });
}

#[tauri::command]
fn get_stats(state: tauri::State<AppState>) -> LifetimeStats {
    state.stats.lock().unwrap().clone()
}

#[tauri::command]
fn reset_stats(app: AppHandle) -> Result<LifetimeStats, String> {
    let fresh = LifetimeStats::default();
    *app.state::<AppState>().stats.lock().unwrap() = fresh.clone();
    stats::save(&app, &fresh)?;
    Ok(fresh)
}

/// Someone poked Glorp. This is important data.
#[tauri::command]
fn record_poke(app: AppHandle) -> u64 {
    let state = app.state::<AppState>();
    let snapshot = {
        let mut stats = state.stats.lock().unwrap();
        stats.pokes += 1;
        stats.clone()
    };
    let _ = stats::save(&app, &snapshot);
    snapshot.pokes
}

/// Called by the UI once it has rendered, so the window never flashes blank.
#[tauri::command]
fn frontend_ready(app: AppHandle) {
    let state = app.state::<AppState>();
    if state.shown_once.swap(true, Ordering::Relaxed) {
        return;
    }
    let minimized = state.launched_minimized || state.settings.lock().unwrap().start_minimized;
    if !minimized {
        show_main(&app);
    }
}

#[tauri::command]
fn hide_window(app: AppHandle) {
    if let Some(w) = app.get_webview_window("main") {
        let _ = w.hide();
    }
}

#[tauri::command]
fn quit_app(app: AppHandle) {
    stop(&app);
    app.exit(0);
}

// ---------------------------------------------------------------- setup

fn build_tray(app: &AppHandle) -> tauri::Result<()> {
    let toggle_item = MenuItem::with_id(app, "toggle", "Start clicking", true, None::<&str>)?;
    let open_item = MenuItem::with_id(app, "open", "Open Glorp", true, None::<&str>)?;
    let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let menu = Menu::with_items(
        app,
        &[
            &toggle_item,
            &open_item,
            &PredefinedMenuItem::separator(app)?,
            &quit_item,
        ],
    )?;
    *app.state::<AppState>().tray_toggle.lock().unwrap() = Some(toggle_item);

    TrayIconBuilder::with_id(TRAY_ID)
        .icon(Image::from_bytes(TRAY_IDLE)?)
        .tooltip("Glorp Clicker · resting")
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "toggle" => {
                toggle(app);
            }
            "open" => show_main(app),
            "quit" => quit_app(app.clone()),
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                show_main(tray.app_handle());
            }
        })
        .build(app)?;
    refresh_tray(app, false);
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let launched_minimized = std::env::args().any(|a| a == AUTOSTART_FLAG);

    tauri::Builder::default()
        // Must be first: a second launch just brings the existing window forward.
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            show_main(app);
        }))
        .plugin(
            tauri_plugin_autostart::Builder::new()
                .arg(AUTOSTART_FLAG)
                .app_name("Glorp Clicker")
                .build(),
        )
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(|app, _shortcut, event| {
                    if event.state() != ShortcutState::Pressed {
                        return;
                    }
                    let state = app.state::<AppState>();
                    if state.picking.load(Ordering::Relaxed)
                        || state.hotkey_suspended.load(Ordering::Relaxed)
                    {
                        return;
                    }
                    toggle(app);
                })
                .build(),
        )
        .setup(move |app| {
            let handle = app.handle().clone();
            let settings = settings::load(&handle);
            let hotkey = settings.hotkey.clone();
            app.manage(AppState {
                settings: Mutex::new(settings),
                stats: Mutex::new(stats::load(&handle)),
                clicker: Clicker::default(),
                picking: Arc::new(AtomicBool::new(false)),
                hotkey_suspended: AtomicBool::new(false),
                launched_minimized,
                shown_once: AtomicBool::new(false),
                tray_toggle: Mutex::new(None),
            });

            if register_hotkey(&handle, &hotkey).is_err() {
                // Fall back to the default if the saved one is taken.
                let _ = register_hotkey(&handle, "F6");
            }
            build_tray(&handle)?;

            // Closing the window only hides it; Glorp keeps living in the tray.
            if let Some(window) = app.get_webview_window("main") {
                let w = window.clone();
                window.on_window_event(move |event| {
                    if let WindowEvent::CloseRequested { api, .. } = event {
                        api.prevent_close();
                        let _ = w.hide();
                    }
                });
            }

            // Safety net in case the UI never reports ready.
            let h = handle.clone();
            thread::spawn(move || {
                thread::sleep(Duration::from_secs(4));
                frontend_ready(h);
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_settings,
            update_settings,
            start_clicking,
            stop_clicking,
            is_running,
            suspend_hotkey,
            pick_location,
            frontend_ready,
            get_stats,
            reset_stats,
            record_poke,
            hide_window,
            quit_app,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Glorp Clicker");
}
