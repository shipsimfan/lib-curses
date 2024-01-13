use crate::{CHType, Window};
use std::ffi::c_int;

#[link(name = "curses")]
extern "C" {
    /// The [`border`] routine draws a box around the edges of a window. Each argument is a
    /// character with attributes:
    ///  * `ls` - left side,
    ///  * `rs` - right side,
    ///  * `ts` - top side,
    ///  * `bs` - bottom side,
    ///  * `tl` - top left-hand corner,
    ///  * `tr` - top right-hand corner,
    ///  * `bl` - bottom left-hand corner, and
    ///  * `br` - bottom right-hand corner.
    ///
    /// If any of these arguments is zero, then the corresponding default values are used instead.
    pub fn border(
        ls: CHType,
        rs: CHType,
        ts: CHType,
        bs: CHType,
        tl: CHType,
        tr: CHType,
        bl: CHType,
        br: CHType,
    ) -> c_int;

    /// The [`wborder`] routine draws a box around the edges of a window. Other than the window,
    /// each argument is a character with attributes:
    ///  * `ls` - left side,
    ///  * `rs` - right side,
    ///  * `ts` - top side,
    ///  * `bs` - bottom side,
    ///  * `tl` - top left-hand corner,
    ///  * `tr` - top right-hand corner,
    ///  * `bl` - bottom left-hand corner, and
    ///  * `br` - bottom right-hand corner.
    ///
    /// If any of these arguments is zero, then the corresponding default values are used instead.
    pub fn wborder(
        win: *mut Window,
        ls: CHType,
        rs: CHType,
        ts: CHType,
        bs: CHType,
        tl: CHType,
        tr: CHType,
        bl: CHType,
        br: CHType,
    ) -> c_int;

    /// The [`r#box`] routine draws a box around the edges of a window. `r#box(win, verch, horch)`
    /// is a shorthand for the following call:
    /// `wborder(win, verch, verch, horch, horch, 0, 0, 0, 0)`.
    pub fn r#box(win: *mut Window, verch: CHType, verch: CHType) -> c_int;

    /// The [`hline`] function draws a horizontal (left to right) line using `ch` starting at the
    /// current cursor position in the window. The current cursor position is not changed. The line
    /// is at most `n` characters long, or as many as fit into the window.
    pub fn hline(ch: CHType, c: c_int) -> c_int;

    /// The [`whline`] function draws a horizontal (left to right) line using `ch` starting at the
    /// current cursor position in the window. The current cursor position is not changed. The line
    /// is at most `n` characters long, or as many as fit into the window.
    pub fn whline(win: *mut Window, ch: CHType, c: c_int) -> c_int;

    /// The [`vline`] function draws a vertical (top to bottom) line using `ch` starting at the
    /// current cursor position in the window. The current cursor position is not changed. The line
    /// is at most `n` characters long, or as many as fit into the window.
    pub fn vline(ch: CHType, c: c_int) -> c_int;

    /// The [`wvline`] function draws a vertical (top to bottom) line using `ch` starting at the
    /// current cursor position in the window. The current cursor position is not changed. The line
    /// is at most `n` characters long, or as many as fit into the window.
    pub fn wvline(win: *mut Window, ch: CHType, c: c_int) -> c_int;

    /// The [`mvhline`] function draws a horizontal (left to right) line using `ch` starting at the
    /// current cursor position in the window. The current cursor position is not changed. The line
    /// is at most `n` characters long, or as many as fit into the window.
    pub fn mvhline(y: c_int, x: c_int, ch: CHType, c: c_int) -> c_int;

    /// The [`mvwhline`] function draws a horizontal (left to right) line using `ch` starting at the
    /// current cursor position in the window. The current cursor position is not changed. The line
    /// is at most `n` characters long, or as many as fit into the window.
    pub fn mvwhline(win: *mut Window, y: c_int, x: c_int, ch: CHType, c: c_int) -> c_int;

    /// The [`mvvline`] function draws a vertical (top to bottom) line using `ch` starting at the
    /// current cursor position in the window. The current cursor position is not changed. The line
    /// is at most `n` characters long, or as many as fit into the window.
    pub fn mvvline(y: c_int, x: c_int, ch: CHType, c: c_int) -> c_int;

    /// The [`mvwvline`] function draws a vertical (top to bottom) line using `ch` starting at the
    /// current cursor position in the window. The current cursor position is not changed. The line
    /// is at most `n` characters long, or as many as fit into the window.
    pub fn mvwvline(win: *mut Window, y: c_int, x: c_int, ch: CHType, c: c_int) -> c_int;
}
