use std::ffi::{c_char, c_int};

// rustdoc imports
#[allow(unused_imports)]
use crate::{ERR, OK};

#[link(name = "curses")]
extern "C" {
    /// Add a string of characters to a curses window and advance cursor
    ///
    /// [`addnstr`] writes the characters of the (null-terminated) character string `str` on the
    /// given window. [`addnstr`] writes at most `n` characters. If `n` is -1, then the entire
    /// string will be added, up to the maximum number of characters that will fit on the line,
    /// or until a terminating null is reached.
    ///
    /// # Return Value
    /// Returns the integer [`ERR`] upon failure and [`OK`] on success
    pub fn addnstr(str: *const c_char, n: c_int) -> c_int;
}
