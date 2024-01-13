use crate::Window;
use std::ffi::c_int;

#[link(name = "curses")]
extern "C" {
    /// Gets the beginning x of `win`
    pub fn getbegx(win: *mut Window) -> c_int;

    /// Gets the beginning y of `win`
    pub fn getbegy(win: *mut Window) -> c_int;

    /// Gets the current cursor x of `win`
    pub fn getcurx(win: *mut Window) -> c_int;

    /// Gets the current cursor  y of `win`
    pub fn getcury(win: *mut Window) -> c_int;

    /// Gets the maximum x of `win`
    pub fn getmaxx(win: *mut Window) -> c_int;

    /// Gets the maximum y of `win`
    pub fn getmaxy(win: *mut Window) -> c_int;

    /// Gets the x position relative to the parent window, or -1 if there isn't one
    pub fn getparx(win: *mut Window) -> c_int;

    /// Gets the y position relative to the parent window, or -1 if there isn't one
    pub fn getpary(win: *mut Window) -> c_int;

}
