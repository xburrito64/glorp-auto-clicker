use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
use tauri::{AppHandle, Manager};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ClickKind {
    Single,
    Double,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RepeatMode {
    Forever,
    Count,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PositionMode {
    Cursor,
    Fixed,
}

/// Everything the user can configure. Shared 1:1 with the frontend (camelCase JSON).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Settings {
    pub interval_ms: f64,
    /// Only used by the UI to remember which unit the user prefers ("ms" | "s" | "cps").
    pub interval_unit: String,
    /// Random variation of the interval, in percent (0 = perfectly steady).
    pub jitter_pct: u32,
    pub button: MouseButton,
    pub click_kind: ClickKind,
    pub repeat_mode: RepeatMode,
    pub repeat_count: u64,
    pub position_mode: PositionMode,
    pub fixed_x: i32,
    pub fixed_y: i32,
    pub hotkey: String,
    pub theme: String,
    pub autostart: bool,
    pub start_minimized: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            interval_ms: 100.0,
            interval_unit: "ms".into(),
            jitter_pct: 0,
            button: MouseButton::Left,
            click_kind: ClickKind::Single,
            repeat_mode: RepeatMode::Forever,
            repeat_count: 100,
            position_mode: PositionMode::Cursor,
            fixed_x: 0,
            fixed_y: 0,
            hotkey: "F6".into(),
            theme: "glorp".into(),
            autostart: false,
            start_minimized: false,
        }
    }
}

impl Settings {
    /// Clamp values into ranges the engine can handle.
    pub fn sanitized(mut self) -> Self {
        if !self.interval_ms.is_finite() {
            self.interval_ms = 100.0;
        }
        self.interval_ms = self.interval_ms.clamp(1.0, 24.0 * 3600.0 * 1000.0);
        self.jitter_pct = self.jitter_pct.min(90);
        self.repeat_count = self.repeat_count.max(1);
        self
    }
}

fn settings_path(app: &AppHandle) -> Option<PathBuf> {
    app.path().app_config_dir().ok().map(|d| d.join("settings.json"))
}

pub fn load(app: &AppHandle) -> Settings {
    settings_path(app)
        .and_then(|p| fs::read_to_string(p).ok())
        .and_then(|s| serde_json::from_str::<Settings>(&s).ok())
        .unwrap_or_default()
        .sanitized()
}

pub fn save(app: &AppHandle, settings: &Settings) -> Result<(), String> {
    let path = settings_path(app).ok_or("no config directory")?;
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    }
    let json = serde_json::to_string_pretty(settings).map_err(|e| e.to_string())?;
    fs::write(path, json).map_err(|e| e.to_string())
}
