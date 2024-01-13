use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::{ERR, OK};

#[link(name = "curses")]
extern "C" {
    /// Refresh curses windows and lines
    ///
    /// The [`refresh`] routine must be called to get actual output to the terminal, as other
    /// routines merely manipulate data structures. The routine [`refresh`] copies `stdscr`
    /// to the physical terminal screen, taking into account what is already there to do
    /// optimizations. Unless `leaveok` has been enabled, the physical cursor of the terminal is
    /// left at the location of the cursor for that window.
    ///
    /// # Return Value
    /// [`refresh`] returns the integer [`ERR`] upon failure and [`OK`] upon successful completion
    pub fn refresh() -> c_int;
}
