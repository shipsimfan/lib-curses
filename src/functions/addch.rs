use crate::{CHType, Window};
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::{refresh, wrefresh};

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

    /// The [`echochar`] routine is equivalent to a call to [`addch`] followed by a call to
    /// [`refresh`]
    pub fn echochar(ch: CHType) -> c_int;

    /// The [`wechochar`] routine is equivalent to a call to [`waddch`] followed by a call to
    /// [`wrefresh`]
    pub fn wechochar(window: *mut Window, ch: CHType) -> c_int;

}
