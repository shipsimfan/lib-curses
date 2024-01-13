use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::{ERR, OK};

#[link(name = "curses")]
extern "C" {
    /// Terminate a window
    ///
    /// A program should always call [`endwin`] before exiting or escaping from curses mode
    /// temporarily. This routine restores tty modes, moves the cursor to the lower left-hand
    /// corner of the screen and resets the terminal into the proper non-visual mode. Calling
    /// [`refresh`] or [`doupdate`] after a temporary escape causes the program to resume visual
    /// mode.
    ///
    /// # Return Value
    /// [`endwin`] returns the integer [`ERR`] upon failure and [`OK`] upon successful completion
    pub fn endwin() -> c_int;
}
