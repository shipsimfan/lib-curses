use crate::Window;
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::wrefresh;

#[link(name = "curses")]
extern "C" {
    /// Calling [`newwin`] creates and returns a pointer to a new window with the given number of
    /// lines and columns. The upper left-hand corner of the window is at line `begin_y`, column
    /// `begin_x`. If either `nlines` or `ncols` is zero, they default to `LINES` - `begin_y` and
    /// `COLS` - `begin_x`. A new full-screen window is created by calling `newwin(0,0,0,0)`.
    pub fn newwin(nlines: c_int, ncols: c_int, begin_y: c_int, begin_x: c_int) -> *mut Window;

    /// Calling [`delwin`] deletes the named window, freeing all memory associated with it (it does
    /// not actually erase the window's screen image). Subwindows must be deleted before the main
    /// window can be deleted.
    pub fn delwin(win: *mut Window) -> c_int;

    /// Calling [`mvwin`] moves the window so that the upper left-hand corner is at position
    /// `(x, y)`. If the move would cause the window to be off the screen, it is an error and the
    /// window is not moved. Moving subwindows is allowed, but should be avoided.
    pub fn mvwin(win: *mut Window) -> c_int;

    /// Calling [`subwin`] creates and returns a pointer to a new window with the given number of
    /// lines, `nlines`, and columns, `ncols`. The window is at position `(begin_y, begin_x)` on
    /// the screen. (This position is relative to the screen, and not to the window `orig`.) The
    /// window is made in the middle of the window `orig`, so that changes made to one window will
    /// affect both windows. The subwindow shares memory with the window `orig`. When using this
    /// routine, it is necessary to call [`touchwin`] or [`touchline`] on `orig` before calling
    /// [`wrefresh`] on the subwindow.
    pub fn subwin(
        orig: *mut Window,
        nlines: c_int,
        ncols: c_int,
        begin_y: c_int,
        begin_x: c_int,
    ) -> *mut Window;

    /// Calling [`derwin`] is the same as calling [`subwin`], except that `begin_y` and `begin_x`
    /// are relative to the origin of the window `orig` rather than the screen. There is no
    /// difference between the subwindows and the derived windows.
    pub fn derwin(
        orig: *mut Window,
        nlines: c_int,
        ncols: c_int,
        begin_y: c_int,
        begin_x: c_int,
    ) -> *mut Window;

    /// Calling [`mvderwin`] moves a derived window (or subwindow) inside its parent window. The
    /// screen-relative parameters of the window are not changed. This routine is used to display
    /// different parts of the parent window at the same physical position on the screen.
    pub fn mvderwin(win: *mut Window, par_y: c_int, par_x: c_int) -> c_int;

    /// Calling [`dupwin`] creates an exact duplicate of the window `win`.
    pub fn dupwin(win: *mut Window) -> *mut Window;

    /// Calling [`wsyncup`] touches all locations in ancestors of win that are changed in `win`
    pub fn wsyncup(win: *mut Window) -> c_int;

    /// If [`syncok`] is called with second argument `true` then [`wsyncup`] is called
    /// automatically whenever there is a change in the window
    pub fn syncok(win: *mut Window, bf: bool) -> c_int;

    /// The routine [`wcursyncup`] updates the current cursor position of all the
    /// ancestors of the window to reflect the current cursor position of the window.
    pub fn wcursyncup(win: *mut Window);

    /// The [`wsyncdown`] routine touches each location in win that has been touched in any of its
    /// ancestor windows. This routine is called by [`wrefresh`], so it should almost never be
    /// necessary to call it manually.
    pub fn wsyncdown(win: *mut Window);
}
