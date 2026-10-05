//! The click engine: one background thread per run, with drift-free timing.
//!
//! Timing works like a train timetable: click #n is due at exactly `start + n × interval`.
//! Each wait is measured against that timetable, never against the previous click, so if
//! Windows makes one click a little late, the next gap is automatically a little shorter
//! (and vice versa). Errors can't stack up, however long Glorp runs.

use serde::Serialize;
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread,
    time::{Duration, Instant},
};
use tauri::{AppHandle, Emitter};

use crate::{
    input,
    settings::{ClickKind, PositionMode, RepeatMode, Settings},
    stats::Session,
};

/// How often live stats are pushed to the UI.
const STATS_EVERY: Duration = Duration::from_millis(150);
/// Below this much remaining wait we busy-wait instead of sleeping, for accuracy.
const SPIN_WINDOW: Duration = Duration::from_micros(700);
/// Longest single nap, so a stop request is noticed within a few milliseconds.
const MAX_NAP: Duration = Duration::from_millis(8);
/// While catching up after a late click, gaps may shrink to this fraction of normal, no further.
/// That keeps the correction smooth instead of firing a burst of back-to-back clicks.
const MIN_GAP_FRACTION: f64 = 0.5;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveStats {
    pub clicks: u64,
    /// Clicks/sec over the last moment (lively, for animations).
    pub cps: f64,
    /// Average clicks/sec since the run started (steady, shows the real accuracy).
    pub avg_cps: f64,
    pub elapsed_secs: f64,
}

// ---------------------------------------------------------------- pacing

pub struct PaceConfig {
    /// Seconds between clicks.
    pub interval: f64,
    /// Random variation of each gap, as a fraction (0.1 = ±10%).
    pub jitter: f64,
    pub max_ticks: Option<u64>,
    pub max_secs: Option<f64>,
}

pub struct PaceReport {
    pub ticks: u64,
    /// From start until the run ended.
    pub elapsed: Duration,
    /// From the first tick to the last tick.
    pub span: Duration,
    /// True if the run reached its click/time goal (false if it was stopped).
    pub completed: bool,
    /// Diagnostics: time spent inside `tick`, and how the sleeps behaved.
    pub tick_time_total: Duration,
    pub tick_time_max: Duration,
    pub naps: u64,
    pub oversleep_total: Duration,
    pub oversleep_max: Duration,
    pub high_res_timer: bool,
}

/// Calls `tick` on a drift-free schedule until stopped or a goal is reached.
pub fn pace(cfg: &PaceConfig, active: &AtomicBool, mut tick: impl FnMut(u64)) -> PaceReport {
    let start = Instant::now();
    let end = cfg.max_secs.map(|s| start + Duration::from_secs_f64(s));
    let max_ticks = cfg.max_ticks.unwrap_or(u64::MAX);
    // If we ever fall further behind than this (e.g. the PC froze for a moment), the missed
    // time is forgiven rather than "made up" with a flood of clicks.
    let catch_up_limit = (cfg.interval * 4.0).clamp(0.03, 0.5);

    // Seconds after `start` at which the next click is due. Kept as one running total
    // (not a chain of rounded waits) so no rounding error can accumulate.
    let mut due_at = 0.0f64;
    let mut ticks = 0u64;
    let mut first_tick: Option<Instant> = None;
    let mut last_tick = start;
    let sleeper = input::PreciseSleeper::new();
    let mut tick_time_total = Duration::ZERO;
    let mut tick_time_max = Duration::ZERO;

    let completed = loop {
        if !active.load(Ordering::Relaxed) {
            break false;
        }
        if end.is_some_and(|e| Instant::now() >= e) {
            break true;
        }

        // A click "happens" when it starts. Gaps are measured start-to-start, so a click that
        // takes Windows a few ms to deliver doesn't push all later clicks back.
        last_tick = Instant::now();
        tick(ticks);
        ticks += 1;
        let took = last_tick.elapsed();
        tick_time_total += took;
        tick_time_max = tick_time_max.max(took);
        first_tick.get_or_insert(last_tick);
        if ticks >= max_ticks {
            break true;
        }

        let gap = if cfg.jitter > 0.0 {
            cfg.interval * (1.0 + cfg.jitter * (fastrand::f64() * 2.0 - 1.0))
        } else {
            cfg.interval
        };
        due_at += gap;

        let now_secs = start.elapsed().as_secs_f64();
        if now_secs - due_at > catch_up_limit {
            due_at = now_secs;
        }

        // Course correction: aim for the timetable, but never closer to the last click
        // than half a normal gap.
        let on_schedule = start + Duration::from_secs_f64(due_at);
        let earliest = last_tick + Duration::from_secs_f64(gap * MIN_GAP_FRACTION);
        let deadline = on_schedule.max(earliest);

        if let Some(e) = end {
            if deadline >= e {
                wait_until(e, active, &sleeper);
                break active.load(Ordering::Relaxed);
            }
        }
        wait_until(deadline, active, &sleeper);
    };

    PaceReport {
        ticks,
        elapsed: start.elapsed(),
        span: first_tick.map(|f| last_tick - f).unwrap_or_default(),
        completed,
        tick_time_total,
        tick_time_max,
        naps: sleeper.naps.get(),
        oversleep_total: sleeper.oversleep_total.get(),
        oversleep_max: sleeper.oversleep_max.get(),
        high_res_timer: sleeper.is_high_res(),
    }
}

fn wait_until(deadline: Instant, active: &AtomicBool, sleeper: &input::PreciseSleeper) {

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
            sleeper.sleep((left - SPIN_WINDOW).min(MAX_NAP));
        } else {
            std::hint::spin_loop();
            thread::yield_now();
        }
    }
}

// ---------------------------------------------------------------- runs

struct Run {
    active: Arc<AtomicBool>,
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
        thread::Builder::new()
            .name("glorp-clicker".into())
            .spawn(move || {
                let completed = run(&app, &settings, own_window, &run_active);
                // Only report "done" if nobody stopped us in the meantime.
                if completed {
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
        *current = Some(Run { active });
        true
    }

    /// Stops the current run. Returns false if nothing was running.
    pub fn stop(&self) -> bool {
        match self.current.lock().unwrap().take() {
            Some(run) => {
                run.active.store(false, Ordering::Relaxed);
                true
            }
            None => false,
        }
    }
}

/// Runs one clicking session. Returns true if it ended by itself (goal reached).
fn run(app: &AppHandle, s: &Settings, own_window: isize, active: &AtomicBool) -> bool {
    let timer = input::HighResTimer::new();
    input::boost_current_thread();
    let cpu_before = input::process_cpu_time();
    let started = Instant::now();

    let cfg = PaceConfig {
        interval: s.interval_ms / 1000.0,
        jitter: s.jitter_pct as f64 / 100.0,
        max_ticks: (s.repeat_mode == RepeatMode::Count).then_some(s.repeat_count),
        max_secs: (s.repeat_mode == RepeatMode::Time).then_some(s.duration_secs),
    };

    let mut clicks: u64 = 0;
    let mut last_emit = Instant::now();
    let mut clicks_at_emit: u64 = 0;
    let emit = |clicks: u64, cps: f64, ticks: u64| {
        let elapsed = started.elapsed().as_secs_f64();
        let avg_cps = if ticks > 1 && elapsed > 0.0 {
            (ticks - 1) as f64 / elapsed
        } else {
            0.0
        };
        let _ = app.emit(
            "clicker-stats",
            LiveStats {
                clicks,
                cps,
                avg_cps,
                elapsed_secs: elapsed,
            },
        );
    };

    // Diagnostics: time spent in each step of a tick (move, check, click, ui update).
    let mut steps = [StepTime::default(); 4];
    let report = pace(&cfg, active, |tick| {
        let t0 = Instant::now();
        if s.position_mode == PositionMode::Fixed {
            input::set_cursor_pos(s.fixed_x, s.fixed_y);
        }
        let t1 = Instant::now();
        // Safety: never click on Glorp's own window (e.g. the Start/Stop button).
        let blocked = input::cursor_over_own_window(own_window);
        let t2 = Instant::now();
        if !blocked {
            input::click(s.button, s.click_kind);
            clicks += 1;
        }
        let t3 = Instant::now();
        let since = last_emit.elapsed();
        if since >= STATS_EVERY {
            emit(clicks, (clicks - clicks_at_emit) as f64 / since.as_secs_f64(), tick + 1);
            last_emit = Instant::now();
            clicks_at_emit = clicks;
        }
        let t4 = Instant::now();
        for (step, d) in steps.iter_mut().zip([t1 - t0, t2 - t1, t3 - t2, t4 - t3]) {
            step.add(d);
        }
    });
    emit(clicks, 0.0, report.ticks);
    write_timing_log(app, &cfg, &report, timer.unthrottled, &steps);

    crate::record_session(
        app,
        Session {
            clicks,
            ticks: report.ticks,
            secs: report.elapsed.as_secs_f64(),
            span_secs: report.span.as_secs_f64(),
            target_cps: 1.0 / cfg.interval,
            button: s.button,
            double: s.click_kind == ClickKind::Double,
            cpu_secs: (input::process_cpu_time() - cpu_before).as_secs_f64(),
        },
    );
    report.completed
}

#[derive(Default, Clone, Copy)]
struct StepTime {
    n: u32,
    total: Duration,
    max: Duration,
}

impl StepTime {
    fn add(&mut self, d: Duration) {
        self.n += 1;
        self.total += d;
        self.max = self.max.max(d);
    }
}

impl std::fmt::Display for StepTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let avg = if self.n == 0 { Duration::ZERO } else { self.total / self.n };
        write!(f, "avg {avg:?}, max {:?}", self.max)
    }
}

/// Writes timing diagnostics to `timing-log.txt` in the config folder, but only when the
/// `GLORP_TIMING_LOG` environment variable is set. Used to investigate accuracy problems.
fn write_timing_log(
    app: &AppHandle,
    cfg: &PaceConfig,
    r: &PaceReport,
    unthrottled: bool,
    steps: &[StepTime; 4],
) {
    use tauri::Manager;
    if std::env::var_os("GLORP_TIMING_LOG").is_none() {
        return;
    }
    let Ok(dir) = app.path().app_config_dir() else { return };
    let per = |total: Duration, n: u64| if n == 0 { Duration::ZERO } else { total / n as u32 };
    let actual = if r.span.as_secs_f64() > 0.0 { (r.ticks.saturating_sub(1)) as f64 / r.span.as_secs_f64() } else { 0.0 };
    let text = format!(
        "target {:.2} cps, actual {:.2} cps, ticks {}\n\
         power throttling opt-out ok: {unthrottled}, high-res timer: {}\n\
         tick time avg {:?}, max {:?}\n\
         naps {}, oversleep avg {:?}, max {:?}\n\
         move {}\ncheck {}\nclick {}\nui {}\n",
        1.0 / cfg.interval,
        actual,
        r.ticks,
        r.high_res_timer,
        per(r.tick_time_total, r.ticks),
        r.tick_time_max,
        r.naps,
        per(r.oversleep_total, r.naps),
        r.oversleep_max,
        steps[0],
        steps[1],
        steps[2],
        steps[3],
    );
    let _ = std::fs::write(dir.join("timing-log.txt"), text);
}
// ---------------------------------------------------------------- tests

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg(interval: f64) -> PaceConfig {
        PaceConfig {
            interval,
            jitter: 0.0,
            max_ticks: None,
            max_secs: None,
        }
    }

    /// The headline test: 100 clicks/sec for 3 seconds while the "click" itself randomly
    /// takes up to 4 ms (simulated system lag). A naive clicker drifts badly here;
    /// course correction must land on 300 ticks.
    #[test]
    fn stays_on_target_despite_lag() {
        let _t = input::HighResTimer::new();
        let active = AtomicBool::new(true);
        let mut c = cfg(0.010);
        c.max_secs = Some(3.0);
        let report = pace(&c, &active, |_| {
            thread::sleep(Duration::from_micros(fastrand::u64(0..4000)));
        });
        println!("lagged 100 cps for 3s -> {} ticks", report.ticks);
        assert!(report.completed);
        assert!((299..=301).contains(&report.ticks), "got {} ticks", report.ticks);
    }

    /// Some PCs take several ms to deliver each click (other programs watching the mouse).
    /// As long as a click takes less than one gap, the speed must not suffer.
    #[test]
    fn slow_clicks_dont_slow_the_schedule() {
        let _t = input::HighResTimer::new();
        let active = AtomicBool::new(true);
        let mut c = cfg(0.010);
        c.max_secs = Some(2.0);
        let report = pace(&c, &active, |_| thread::sleep(Duration::from_millis(7)));
        println!("100 cps with 7ms clicks for 2s -> {} ticks", report.ticks);
        assert!((199..=201).contains(&report.ticks), "got {} ticks", report.ticks);
    }

    /// Big occasional hiccups (longer than one interval) are caught up smoothly.
    #[test]
    fn catches_up_after_hiccups() {
        let _t = input::HighResTimer::new();
        let active = AtomicBool::new(true);
        let mut c = cfg(0.005);
        c.max_secs = Some(2.0);
        let report = pace(&c, &active, |i| {
            if i % 50 == 25 {
                thread::sleep(Duration::from_millis(15));
            }
        });
        println!("200 cps for 2s with 15ms hiccups -> {} ticks", report.ticks);
        assert!((398..=401).contains(&report.ticks), "got {} ticks", report.ticks);
    }

    /// Jitter varies each gap but the average speed still matches the target.
    #[test]
    fn jitter_keeps_average_speed() {
        let _t = input::HighResTimer::new();
        let active = AtomicBool::new(true);
        let mut c = cfg(0.010);
        c.jitter = 0.3;
        c.max_secs = Some(3.0);
        let report = pace(&c, &active, |_| {});
        println!("100 cps ±30% for 3s -> {} ticks", report.ticks);
        assert!((290..=310).contains(&report.ticks), "got {} ticks", report.ticks);
    }

    #[test]
    fn stops_after_click_goal() {
        let active = AtomicBool::new(true);
        let mut c = cfg(0.001);
        c.max_ticks = Some(25);
        let report = pace(&c, &active, |_| {});
        assert!(report.completed);
        assert_eq!(report.ticks, 25);
    }

    #[test]
    fn time_goal_ends_on_time() {
        let active = AtomicBool::new(true);
        let mut c = cfg(0.3);
        c.max_secs = Some(1.0);
        let report = pace(&c, &active, |_| {});
        assert!(report.completed);
        assert_eq!(report.ticks, 4); // at 0.0, 0.3, 0.6 and 0.9 s
        let secs = report.elapsed.as_secs_f64();
        assert!((0.99..1.05).contains(&secs), "ran for {secs}s");
    }

    /// For comparison only (`cargo test naive -- --ignored --nocapture`): the classic
    /// "click, then sleep(interval)" loop under the same lag as the headline test.
    #[test]
    #[ignore]
    fn naive_clicker_drifts() {
        let start = Instant::now();
        let mut ticks = 0;
        while start.elapsed() < Duration::from_secs(3) {
            thread::sleep(Duration::from_micros(fastrand::u64(0..4000)));
            ticks += 1;
            thread::sleep(Duration::from_millis(10));
        }
        println!("naive 100 cps for 3s -> {ticks} ticks (target 300)");
    }
}
