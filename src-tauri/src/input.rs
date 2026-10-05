//! Thin, safe wrappers around the Win32 calls we need for clicking.

use std::{
    cell::Cell,
    ffi::c_void,
    mem::size_of,
    ptr::null,
    sync::atomic::{AtomicBool, Ordering},
    thread,
    time::{Duration, Instant},
};
use windows_sys::Win32::{
    Foundation::{CloseHandle, FILETIME, HANDLE, HWND, POINT, RECT},
    Media::{timeBeginPeriod, timeEndPeriod},
    System::Threading::{
        CreateWaitableTimerExW, GetCurrentProcess, GetCurrentThread, GetProcessTimes,
        ProcessPowerThrottling, SetProcessInformation, SetThreadPriority, SetWaitableTimer,
        WaitForSingleObject, CREATE_WAITABLE_TIMER_HIGH_RESOLUTION,
        PROCESS_POWER_THROTTLING_CURRENT_VERSION, PROCESS_POWER_THROTTLING_EXECUTION_SPEED,
        PROCESS_POWER_THROTTLING_IGNORE_TIMER_RESOLUTION, PROCESS_POWER_THROTTLING_STATE,
        THREAD_PRIORITY_HIGHEST, TIMER_ALL_ACCESS,
    },
    UI::{
        Input::KeyboardAndMouse::{
            GetAsyncKeyState, SendInput, INPUT, INPUT_0, INPUT_MOUSE, MOUSEEVENTF_LEFTDOWN,
            MOUSEEVENTF_LEFTUP, MOUSEEVENTF_MIDDLEDOWN, MOUSEEVENTF_MIDDLEUP,
            MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP, MOUSEINPUT, MOUSE_EVENT_FLAGS, VK_ESCAPE,
            VK_LBUTTON,
        },
        WindowsAndMessaging::{
            GetCursorPos, GetForegroundWindow, GetWindowRect, IsWindowVisible, SetCursorPos,
        },
    },
};

use crate::settings::{ClickKind, MouseButton};

fn mouse_input(flags: MOUSE_EVENT_FLAGS) -> INPUT {
    INPUT {
        r#type: INPUT_MOUSE,
        Anonymous: INPUT_0 {
            mi: MOUSEINPUT {
                dx: 0,
                dy: 0,
                mouseData: 0,
                dwFlags: flags,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    }
}

/// Sends a full click (down + up, twice for a double click) at the current cursor position.
pub fn click(button: MouseButton, kind: ClickKind) {
    let (down, up) = match button {
        MouseButton::Left => (MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP),
        MouseButton::Right => (MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP),
        MouseButton::Middle => (MOUSEEVENTF_MIDDLEDOWN, MOUSEEVENTF_MIDDLEUP),
    };
    let single = [mouse_input(down), mouse_input(up)];
    let double = [single[0], single[1], single[0], single[1]];
    let inputs: &[INPUT] = match kind {
        ClickKind::Single => &single,
        ClickKind::Double => &double,
    };
    unsafe {
        SendInput(inputs.len() as u32, inputs.as_ptr(), size_of::<INPUT>() as i32);
    }
}

pub fn cursor_pos() -> (i32, i32) {
    let mut p = POINT { x: 0, y: 0 };
    unsafe { GetCursorPos(&mut p) };
    (p.x, p.y)
}

pub fn set_cursor_pos(x: i32, y: i32) {
    unsafe { SetCursorPos(x, y) };
}

/// True if the cursor is over our own window while it's visible and in front.
///
/// Deliberately avoids `WindowFromPoint`: that sends a message to whatever window is under the
/// cursor and waits for its reply, so a busy game or app would stall every click by
/// milliseconds. These calls only read state and never wait on other programs.
pub fn cursor_over_own_window(own: isize) -> bool {
    if own == 0 {
        return false;
    }
    let hwnd = own as HWND;
    unsafe {
        if IsWindowVisible(hwnd) == 0 || GetForegroundWindow() != hwnd {
            return false;
        }
        let mut r = RECT {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        };
        if GetWindowRect(hwnd, &mut r) == 0 {
            return false;
        }
        let (x, y) = cursor_pos();
        x >= r.left && x < r.right && y >= r.top && y < r.bottom
    }
}

fn key_down(vk: u16) -> bool {
    unsafe { GetAsyncKeyState(vk as i32) as u16 & 0x8000 != 0 }
}

/// Waits for the user to left-click somewhere on screen and returns that point.
/// Returns `None` if Escape is pressed, `cancel` is set, or it times out.
pub fn pick_point(cancel: &AtomicBool, timeout: Duration) -> Option<(i32, i32)> {
    let started = Instant::now();
    let tick = Duration::from_millis(8);
    let alive = || !cancel.load(Ordering::Relaxed) && started.elapsed() < timeout;

    // If the button is still held from clicking "Pick", wait for it to be released first.
    while key_down(VK_LBUTTON) && alive() {
        thread::sleep(tick);
    }
    while alive() {
        if key_down(VK_ESCAPE) {
            return None;
        }
        if key_down(VK_LBUTTON) {
            let point = cursor_pos();
            while key_down(VK_LBUTTON) && alive() {
                thread::sleep(tick);
            }
            return Some(point);
        }
        thread::sleep(tick);
    }
    None
}

/// Total CPU time (user + kernel) this process has used so far.
pub fn process_cpu_time() -> Duration {
    let zero = FILETIME {
        dwLowDateTime: 0,
        dwHighDateTime: 0,
    };
    let (mut created, mut exited, mut kernel, mut user) = (zero, zero, zero, zero);
    let ok = unsafe {
        GetProcessTimes(
            GetCurrentProcess(),
            &mut created,
            &mut exited,
            &mut kernel,
            &mut user,
        )
    };
    if ok == 0 {
        return Duration::ZERO;
    }
    // FILETIME counts 100-nanosecond ticks.
    let ticks = |ft: FILETIME| ((ft.dwHighDateTime as u64) << 32) | ft.dwLowDateTime as u64;
    Duration::from_nanos((ticks(kernel) + ticks(user)) * 100)
}

fn set_power_throttling(control_mask: u32) -> bool {
    let state = PROCESS_POWER_THROTTLING_STATE {
        Version: PROCESS_POWER_THROTTLING_CURRENT_VERSION,
        ControlMask: control_mask,
        // 0 = "don't throttle" for every aspect named in the control mask.
        StateMask: 0,
    };
    unsafe {
        SetProcessInformation(
            GetCurrentProcess(),
            ProcessPowerThrottling,
            &state as *const _ as *const c_void,
            size_of::<PROCESS_POWER_THROTTLING_STATE>() as u32,
        ) != 0
    }
}

/// Keeps Windows timing precise while alive.
///
/// Windows 11 saves power by ignoring timer-precision requests from "background" apps (like
/// one sitting in the tray) and by running them on slow cores. That turns a 10 ms wait into
/// ~15.6 ms, capping clickers at ~64 clicks/sec. While a run is active we opt out of both,
/// and hand control back to Windows afterwards so Glorp stays power-friendly when idle.
pub struct HighResTimer {
    pub unthrottled: bool,
}

impl HighResTimer {
    pub fn new() -> Self {
        let unthrottled = set_power_throttling(
            PROCESS_POWER_THROTTLING_EXECUTION_SPEED
                | PROCESS_POWER_THROTTLING_IGNORE_TIMER_RESOLUTION,
        );
        unsafe { timeBeginPeriod(1) };
        HighResTimer { unthrottled }
    }
}

impl Drop for HighResTimer {
    fn drop(&mut self) {
        unsafe { timeEndPeriod(1) };
        set_power_throttling(0);
    }
}

/// Sleeps using a high-resolution waitable timer (sub-millisecond precision on Windows 10+,
/// independent of the global timer resolution). Falls back to `thread::sleep`.
pub struct PreciseSleeper {
    timer: HANDLE,
    /// How many naps were taken, and how much longer than asked they lasted in total / at worst.
    pub naps: Cell<u64>,
    pub oversleep_total: Cell<Duration>,
    pub oversleep_max: Cell<Duration>,
}

impl PreciseSleeper {
    pub fn new() -> Self {
        let timer = unsafe {
            CreateWaitableTimerExW(
                null(),
                null(),
                CREATE_WAITABLE_TIMER_HIGH_RESOLUTION,
                TIMER_ALL_ACCESS,
            )
        };
        PreciseSleeper {
            timer,
            naps: Cell::new(0),
            oversleep_total: Cell::new(Duration::ZERO),
            oversleep_max: Cell::new(Duration::ZERO),
        }
    }

    pub fn sleep(&self, d: Duration) {
        let started = Instant::now();
        self.sleep_inner(d);
        let over = started.elapsed().saturating_sub(d);
        self.naps.set(self.naps.get() + 1);
        self.oversleep_total.set(self.oversleep_total.get() + over);
        self.oversleep_max.set(self.oversleep_max.get().max(over));
    }

    fn sleep_inner(&self, d: Duration) {
        if self.timer.is_null() {
            thread::sleep(d);
            return;
        }
        // Negative = relative time, in 100 ns units.
        let due = -((d.as_nanos() / 100).max(1) as i64);
        unsafe {
            if SetWaitableTimer(self.timer, &due, 0, None, null(), 0) == 0 {
                thread::sleep(d);
                return;
            }
            WaitForSingleObject(self.timer, u32::MAX);
        }
    }

    pub fn is_high_res(&self) -> bool {
        !self.timer.is_null()
    }
}

impl Drop for PreciseSleeper {
    fn drop(&mut self) {
        if !self.timer.is_null() {
            unsafe { CloseHandle(self.timer) };
        }
    }
}

/// Gives the calling thread a higher scheduling priority, so clicks aren't kept waiting.
pub fn boost_current_thread() {
    unsafe { SetThreadPriority(GetCurrentThread(), THREAD_PRIORITY_HIGHEST) };
}

#[cfg(test)]
mod tests {
    use super::*;
    use windows_sys::Win32::UI::Input::KeyboardAndMouse::MOUSEEVENTF_MOVE;

    fn time_it(name: &str, n: u32, mut f: impl FnMut()) {
        let mut worst = Duration::ZERO;
        let start = Instant::now();
        for _ in 0..n {
            let t = Instant::now();
            f();
            worst = worst.max(t.elapsed());
        }
        let avg = start.elapsed() / n;
        println!("{name:<28} avg {avg:>10.2?}   worst {worst:>10.2?}");
    }

    /// Measures how long each Win32 call in the click path takes on this PC.
    /// Harmless: only zero-distance mouse moves are injected, nothing is clicked.
    /// Run with `cargo test latency -- --ignored --nocapture`.
    #[test]
    #[ignore]
    fn latency() {
        let (x, y) = cursor_pos();
        time_it("GetCursorPos", 300, || {
            cursor_pos();
        });
        time_it("SetCursorPos (same spot)", 300, || set_cursor_pos(x, y));
        time_it("own-window check", 300, || {
            cursor_over_own_window(1);
        });
        let moves = [mouse_input(MOUSEEVENTF_MOVE), mouse_input(MOUSEEVENTF_MOVE)];
        time_it("SendInput (2 zero moves)", 300, || unsafe {
            SendInput(2, moves.as_ptr(), size_of::<INPUT>() as i32);
        });
    }
}