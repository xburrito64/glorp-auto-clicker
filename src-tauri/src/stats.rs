//! Lifetime statistics, saved next to the settings as stats.json.

use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fs, path::PathBuf};
use tauri::{AppHandle, Manager};

use crate::settings::MouseButton;

/// How many days of per-day click history to keep.
const DAILY_HISTORY_DAYS: usize = 60;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct LifetimeStats {
    pub total_clicks: u64,
    /// Seconds spent clicking, across all sessions.
    pub total_secs: f64,
    pub sessions: u64,
    pub longest_session_secs: f64,
    pub biggest_session_clicks: u64,
    /// Highest average clicks/sec achieved in a session (that lasted at least a second).
    pub best_cps: f64,
    pub left_clicks: u64,
    pub right_clicks: u64,
    pub middle_clicks: u64,
    pub double_clicks: u64,
    /// CPU time the engine used while clicking, and the core count it's measured against.
    pub cpu_secs: f64,
    pub cpu_cores: u32,
    /// Timing accuracy, weighted by session length (sum of accuracy × seconds, and the seconds).
    pub accuracy_weighted: f64,
    pub accuracy_secs: f64,
    pub pokes: u64,
    /// Unix timestamp (seconds) of the first recorded session.
    pub first_session_at: Option<i64>,
    /// "YYYY-MM-DD" (local time) → clicks that day.
    pub daily: BTreeMap<String, u64>,
}

/// Everything the engine knows about one finished run.
pub struct Session {
    pub clicks: u64,
    pub ticks: u64,
    pub secs: f64,
    /// Time between the first and the last scheduled click.
    pub span_secs: f64,
    pub target_cps: f64,
    pub button: MouseButton,
    pub double: bool,
    pub cpu_secs: f64,
}

impl Session {
    /// How close the real click rate was to the target (1.0 = perfect), if measurable.
    pub fn accuracy(&self) -> Option<f64> {
        if self.ticks < 10 || self.span_secs <= 0.0 {
            return None;
        }
        let actual = (self.ticks - 1) as f64 / self.span_secs;
        Some((1.0 - (actual - self.target_cps).abs() / self.target_cps).clamp(0.0, 1.0))
    }
}

impl LifetimeStats {
    pub fn record(&mut self, s: &Session) {
        if s.clicks == 0 {
            return;
        }
        let now = chrono::Local::now();
        self.first_session_at.get_or_insert(now.timestamp());
        self.sessions += 1;
        self.total_clicks += s.clicks;
        self.total_secs += s.secs;
        self.longest_session_secs = self.longest_session_secs.max(s.secs);
        self.biggest_session_clicks = self.biggest_session_clicks.max(s.clicks);
        if s.span_secs >= 1.0 {
            self.best_cps = self.best_cps.max((s.ticks - 1) as f64 / s.span_secs);
        }
        let presses = if s.double { s.clicks * 2 } else { s.clicks };
        match s.button {
            MouseButton::Left => self.left_clicks += presses,
            MouseButton::Right => self.right_clicks += presses,
            MouseButton::Middle => self.middle_clicks += presses,
        }
        if s.double {
            self.double_clicks += s.clicks;
        }
        self.cpu_secs += s.cpu_secs;
        self.cpu_cores = std::thread::available_parallelism()
            .map(|n| n.get() as u32)
            .unwrap_or(1);
        if let Some(acc) = s.accuracy() {
            self.accuracy_weighted += acc * s.secs;
            self.accuracy_secs += s.secs;
        }

        *self
            .daily
            .entry(now.format("%Y-%m-%d").to_string())
            .or_default() += s.clicks;
        while self.daily.len() > DAILY_HISTORY_DAYS {
            let oldest = self.daily.keys().next().cloned().unwrap();
            self.daily.remove(&oldest);
        }
    }
}

fn stats_path(app: &AppHandle) -> Option<PathBuf> {
    app.path().app_config_dir().ok().map(|d| d.join("stats.json"))
}

pub fn load(app: &AppHandle) -> LifetimeStats {
    stats_path(app)
        .and_then(|p| fs::read_to_string(p).ok())
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

pub fn save(app: &AppHandle, stats: &LifetimeStats) -> Result<(), String> {
    let path = stats_path(app).ok_or("no config directory")?;
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    }
    let json = serde_json::to_string_pretty(stats).map_err(|e| e.to_string())?;
    fs::write(path, json).map_err(|e| e.to_string())
}
