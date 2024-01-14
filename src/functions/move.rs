use crate::Window;
use std::ffi::c_int;

#[link(name = "curses")]
extern "C" {
    /// These routines move the cursor associated with the window to line `y` and column `x`. This
    /// routine does not move the physical cursor of the terminal until refresh is called. The
    /// position specified is relative to the upper left-hand corner of the window, which is (0,0).
    pub fn r#move(y: c_int, x: c_int) -> c_int;

    /// These routines move the cursor associated with the window to line `y` and column `x`. This
    /// routine does not move the physical cursor of the terminal until refresh is called. The
    /// position specified is relative to the upper left-hand corner of the window, which is (0,0).
    pub fn wmove(win: *mut Window, y: c_int, x: c_int) -> c_int;
}
