use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::{ERR, OK};

#[link(name = "curses")]
extern "C" {
    /// Get characters from curses terminal keyboard
    ///
    /// The [`getch`] routine reads a character from the window. In no-delay mode, if no input is
    /// waiting, the value [`ERR`] is returned. In delay mode, the program waits until the system
    /// passes text through to the program. Depending on the setting of `cbreak`, this is after one
    /// character (`cbreak` mode), or after the first newline (`nocbreak` mode). In half-delay
    /// mode, the program waits until a character is typed or the specified timeout has been
    /// reached.
    ///
    /// Unless `noecho` has been set, then the character will also be echoed into the designated
    /// window according to the following rules: If the character is the current erase character,
    /// left arrow, or backspace, the cursor is moved one space to the left and that screen
    /// position is erased as if [`delch`] had been called. If the character value is any other
    /// `KEY_` constant, the user is alerted with a beep call. Otherwise the character is simply
    /// output to the screen.
    ///
    /// If the window is not a pad, and it has been moved or modified since the last call to
    /// [`wrefresh`], [`wrefresh`] will be called before another character is read.
    ///
    /// If `keypad` is `true`, and a function key is pressed, the token for that function key is
    /// returned instead of the raw characters. Possible function keys are defined in [`crate`] as
    /// macros with values outside the range of 8-bit characters whose names begin with `KEY_`.
    /// Thus, a variable intended to hold the return value of a function key must be of short size
    /// or larger.
    ///
    /// When a character that could be the beginning of a function key is received (which, on
    /// modern terminals, means an escape character), curses sets a timer. If the remainder of the
    /// sequence does not come in within the designated time, the character is passed through;
    /// otherwise, the function key value is returned. For this reason, many terminals experience
    /// a delay between the time a user presses the escape key and the escape is returned to the
    /// program.
    pub fn getch() -> c_int;
}
