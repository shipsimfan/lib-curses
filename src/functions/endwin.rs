use std::ffi::c_int;

#[link(name = "curses")]
extern "C" {
    /// Terminate a window
    ///
    /// A program should always call [`endwin`] before exiting or escaping from curses mode
    /// temporarily. This routine restores tty modes, moves the cursor to the lower left-hand
    /// corner of the screen and resets the terminal into the proper non-visual mode. Calling
    /// [`refresh`] or [`doupdate`] after a temporary escape causes the program to resume visual
    /// mode.
    pub fn endwin() -> c_int;
}