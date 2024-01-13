use crate::Window;
use std::ffi::c_int;

#[link(name = "curses")]
extern "C" {
    /// Get characters from `curses` terminal keyboard
    pub fn getch() -> c_int;

    /// Get characters from `curses` terminal keyboard
    pub fn wgetch(win: *mut Window) -> c_int;

    /// Get characters from `curses` terminal keyboard
    pub fn mvgetch(y: c_int, x: c_int) -> c_int;

    /// Get characters from `curses` terminal keyboard
    pub fn mvwgetch(y: c_int, x: c_int, win: *mut Window) -> c_int;
}
