//! Thin, safe wrappers around the Win32 calls we need for clicking.

use std::{
    mem::size_of,
    sync::atomic::{AtomicBool, Ordering},
    thread,
    time::{Duration, Instant},
};
use windows_sys::Win32::{
    Foundation::POINT,
    Media::{timeBeginPeriod, timeEndPeriod},
    UI::{
        Input::KeyboardAndMouse::{
            GetAsyncKeyState, SendInput, INPUT, INPUT_0, INPUT_MOUSE, MOUSEEVENTF_LEFTDOWN,
            MOUSEEVENTF_LEFTUP, MOUSEEVENTF_MIDDLEDOWN, MOUSEEVENTF_MIDDLEUP,
            MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP, MOUSEINPUT, MOUSE_EVENT_FLAGS, VK_ESCAPE,
            VK_LBUTTON,
        },
        WindowsAndMessaging::{GetAncestor, GetCursorPos, SetCursorPos, WindowFromPoint, GA_ROOT},
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

/// Top-level window handle under the cursor (as an integer so it can be compared/stored).
pub fn root_window_at_cursor() -> isize {
    let (x, y) = cursor_pos();
    unsafe {
        let hwnd = WindowFromPoint(POINT { x, y });
        if hwnd.is_null() {
            return 0;
        }
        GetAncestor(hwnd, GA_ROOT) as isize
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

/// Raises the Windows timer resolution to 1 ms while alive, so sleeps are precise.
pub struct HighResTimer;

impl HighResTimer {
    pub fn new() -> Self {
        unsafe { timeBeginPeriod(1) };
        HighResTimer
    }
}

impl Drop for HighResTimer {
    fn drop(&mut self) {
        unsafe { timeEndPeriod(1) };
    }
}
