use crate::Window;

// rustdoc imports
#[allow(unused_imports)]
use std::ptr::null;

#[link(name = "curses")]
extern "C" {
    /// `curses` screen initialization
    ///
    /// [`initscr`] is normally the first curses routine to call when initializing a program. A few
    /// special routines sometimes need to be called before it; these are [`slk_init`], [`filter`],
    /// [`ripoffline`], [`use_env`]. For multiple-terminal applications, [`newterm`] may be called
    /// before [`initscr`].
    ///
    /// The [`initscr`] code determines the terminal type and initializes all curses data
    /// structures. [`initscr`] also causes the first call to refresh to clear the screen. If
    /// errors occur, [`initscr`] writes an appropriate error message to standard error and exits;
    /// otherwise, a pointer is returned to `stdscr`.
    ///
    /// # Return Value
    /// Routines that return pointers always return [`null`] on error
    pub fn initscr() -> *mut Window;
}
