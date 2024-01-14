use crate::{CHType, Window};
use std::ffi::c_int;

#[link(name = "curses")]
extern "C" {
    /// Add a character (with attributes) to a `curses` window, then advance the cursor
    pub fn addch(ch: CHType) -> c_int;

    /// Add a character (with attributes) to a `curses` window, then advance the cursor
    pub fn waddch(window: *mut Window, ch: CHType) -> c_int;

    /// Add a character (with attributes) to a `curses` window, then advance the cursor
    pub fn mvaddch(y: c_int, x: c_int, ch: CHType) -> c_int;

    /// Add a character (with attributes) to a `curses` window, then advance the cursor
    pub fn mvwaddch(window: *mut Window, y: c_int, x: c_int, ch: CHType) -> c_int;

}
