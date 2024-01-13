use crate::Window;
use std::ffi::c_int;

#[link(name = "curses")]
extern "C" {
    /// The [`touchwin`] routine throws away all optimization information about which parts of the
    /// window have been touched, by pretending that the entire window has been drawn on. This is
    /// sometimes necessary when using overlapping windows, since a change to one window affects
    /// the other window, but the records of which lines have been changed in the other window do
    /// not reflect the change.
    pub fn touchwin(win: *mut Window) -> c_int;
}
