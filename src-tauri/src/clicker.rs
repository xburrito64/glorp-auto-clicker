//! The click engine: one background thread per run, with precise timing.

use serde::Serialize;
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread::{self, Thread},
    time::{Duration, Instant},
};
use tauri::{AppHandle, Emitter};

use crate::{
    input,
    settings::{PositionMode, RepeatMode, Settings},
};

/// How often live stats are pushed to the UI.
const STATS_EVERY: Duration = Duration::from_millis(150);
/// Below this much remaining wait we busy-wait instead of sleeping, for accuracy.
const SPIN_WINDOW: Duration = Duration::from_micros(1500);

#[derive(Clone, Serialize)]
pub struct Stats {
    pub clicks: u64,
    pub cps: f64,
}

struct Run {
    active: Arc<AtomicBool>,
    thread: Thread,
}

#[derive(Default)]
pub struct Clicker {
    current: Arc<Mutex<Option<Run>>>,
}

impl Clicker {
    pub fn is_running(&self) -> bool {
        self.current.lock().unwrap().is_some()
    }

    /// Starts a run with a snapshot of `settings`. Returns false if already running.
    /// `own_window` is our main window handle; clicks that would land on it are skipped.
    pub fn start(&self, app: AppHandle, settings: Settings, own_window: isize) -> bool {
        let mut current = self.current.lock().unwrap();
        if current.is_some() {
            return false;
        }
        let active = Arc::new(AtomicBool::new(true));
        let slot = Arc::clone(&self.current);
        let run_active = Arc::clone(&active);
        let handle = thread::Builder::new()
            .name("glorp-clicker".into())
            .spawn(move || {
                let finished = run_loop(&app, &settings, own_window, &run_active);
                // Only report "done" if nobody stopped us in the meantime.
                if finished {
                    let mut current = slot.lock().unwrap();
                    let still_ours = current
                        .as_ref()
                        .is_some_and(|r| Arc::ptr_eq(&r.active, &run_active));
                    if still_ours {
                        *current = None;
                        drop(current);
                        crate::notify_state(&app, false, "done");
                    }
                }
            })
            .expect("failed to spawn clicker thread");
        *current = Some(Run {
            active,
            thread: handle.thread().clone(),
        });
        true
    }

    /// Stops the current run. Returns false if nothing was running.
    pub fn stop(&self) -> bool {
        match self.current.lock().unwrap().take() {
            Some(run) => {
                run.active.store(false, Ordering::Relaxed);
                run.thread.unpark();
                true
            }
            None => false,
        }
    }
}

/// Returns true if the run ended by itself (repeat count reached).
fn run_loop(app: &AppHandle, s: &Settings, own_window: isize, active: &AtomicBool) -> bool {
    let _timer = input::HighResTimer::new();
    let base = s.interval_ms / 1000.0;
    let jitter = s.jitter_pct as f64 / 100.0;
    let limit = match s.repeat_mode {
        RepeatMode::Forever => u64::MAX,
        RepeatMode::Count => s.repeat_count,
    };

    let mut clicks: u64 = 0;
    let mut last_emit = Instant::now();
    let mut clicks_at_emit: u64 = 0;
    let mut next = Instant::now();

    let emit = |clicks: u64, cps: f64| {
        let _ = app.emit("clicker-stats", Stats { clicks, cps });
    };

    while active.load(Ordering::Relaxed) {
        if s.position_mode == PositionMode::Fixed {
            input::set_cursor_pos(s.fixed_x, s.fixed_y);
        }
        // Safety: never click on Glorp's own window (e.g. the Start/Stop button).
        if own_window == 0 || input::root_window_at_cursor() != own_window {
            input::click(s.button, s.click_kind);
            clicks += 1;
        }

        let since = last_emit.elapsed();
        if since >= STATS_EVERY {
            emit(clicks, (clicks - clicks_at_emit) as f64 / since.as_secs_f64());
            last_emit = Instant::now();
            clicks_at_emit = clicks;
        }

        if clicks >= limit {
            emit(clicks, 0.0);
            return true;
        }

        let factor = if jitter > 0.0 {
            1.0 + jitter * (fastrand::f64() * 2.0 - 1.0)
        } else {
            1.0
        };
        next += Duration::from_secs_f64((base * factor).max(0.0005));
        // If we fell behind (e.g. the PC stalled), don't fire a burst to catch up.
        let now = Instant::now();
        if next < now {
            next = now;
        }
        wait_until(next, active);
    }
    emit(clicks, 0.0);
    false
}

fn wait_until(deadline: Instant, active: &AtomicBool) {
    loop {
        if !active.load(Ordering::Relaxed) {
            return;
        }
        let now = Instant::now();
        if now >= deadline {
            return;
        }
        let left = deadline - now;
        if left > SPIN_WINDOW {
            // `stop()` unparks us, so stopping is instant even with long intervals.
            thread::park_timeout(left - SPIN_WINDOW);
        } else {
            std::hint::spin_loop();
            thread::yield_now();
        }
    }
}
