use crate::Window;
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::wrefresh;

#[link(name = "curses")]
extern "C" {
    /// `curses` screen initialization
    pub fn initscr() -> *mut Window;

    /// `curses` termination
    pub fn endwin() -> c_int;

    /// Returns `true` if [`endwin`] has been called without any subsequent calls to [`wrefresh`],
    /// and `false` otherwise
    pub fn isendwin() -> bool;
}
