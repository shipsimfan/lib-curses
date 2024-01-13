use crate::Window;
use std::ffi::{c_char, c_int};

#[link(name = "curses")]
extern "C" {
    /// Add a string of characters to a curses window and advance cursor
    pub fn addstr(str: *const c_char) -> c_int;

    /// Add a string of characters to a curses window and advance cursor
    pub fn addnstr(str: *const c_char, n: c_int) -> c_int;

    /// Add a string of characters to a curses window and advance cursor
    pub fn waddstr(win: *mut Window, str: *const c_char) -> c_int;

    /// Add a string of characters to a curses window and advance cursor
    pub fn waddnstr(win: *mut Window, str: *mut Window, n: c_int) -> c_int;

    /// Add a string of characters to a curses window and advance cursor
    pub fn mvaddstr(y: c_int, x: c_int, str: *const c_char) -> c_int;

    /// Add a string of characters to a curses window and advance cursor
    pub fn mvaddnstr(y: c_int, x: c_int, str: *const c_char, n: c_int) -> c_int;

    /// Add a string of characters to a curses window and advance cursor
    pub fn mvwaddstr(y: c_int, x: c_int, win: *mut Window, str: *const c_char) -> c_int;

    /// Add a string of characters to a curses window and advance cursor
    pub fn mvwaddnstr(y: c_int, x: c_int, win: *mut Window, str: *const c_char, n: c_int) -> c_int;
}
