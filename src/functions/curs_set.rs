use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::ERR;

#[link(name = "curses")]
extern "C" {
    /// The [`curs_set`] routine sets the cursor state is set to invisible, normal, or very visible
    /// for `visibility` equal to 0, 1, or 2 respectively. If the terminal supports the visibility
    /// requested, the previous cursor state is returned; otherwise, [`ERR`] is returned.
    pub fn curs_set(visibility: c_int) -> c_int;
}
