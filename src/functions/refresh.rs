use crate::Window;
use std::ffi::c_int;

#[link(name = "curses")]
extern "C" {
    /// Refresh `curses` windows and lines
    pub fn refresh() -> c_int;

    /// Refresh `curses` windows and lines
    pub fn wrefresh(win: *mut Window) -> c_int;
}
