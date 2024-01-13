use std::ffi::{c_uint, c_void};

/// A curses window
pub type Window = c_void;

/// Attributes and a character ORed
pub type CHType = c_uint;
