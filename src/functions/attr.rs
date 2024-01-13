use crate::{CHType, Window};
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::waddstr;

#[link(name = "curses")]
extern "C" {
    /// [`attroff`] manipulates the current attributes of the named window. The current attributes
    /// of a window apply to all characters that are written into the window with [`waddch`],
    /// [`waddstr`] and [`wprintw`]. Attributes are a property of the character, and move with the
    /// character through any scrolling and insert/delete line/character operations. To the extent
    /// possible, they are displayed as appropriate modifications to the graphic rendition of
    /// characters put on the screen.
    pub fn attroff(attrs: CHType) -> c_int;

    /// [`wattroff`] manipulates the current attributes of the named window. The current attributes
    /// of a window apply to all characters that are written into the window with [`waddch`],
    /// [`waddstr`] and [`wprintw`]. Attributes are a property of the character, and move with the
    /// character through any scrolling and insert/delete line/character operations. To the extent
    /// possible, they are displayed as appropriate modifications to the graphic rendition of
    /// characters put on the screen.
    pub fn wattroff(win: *mut Window, attrs: CHType) -> c_int;

    /// [`attron`] manipulates the current attributes of the named window. The current attributes
    /// of a window apply to all characters that are written into the window with [`waddch`],
    /// [`waddstr`] and [`wprintw`]. Attributes are a property of the character, and move with the
    /// character through any scrolling and insert/delete line/character operations. To the extent
    /// possible, they are displayed as appropriate modifications to the graphic rendition of
    /// characters put on the screen.
    pub fn attron(attrs: CHType) -> c_int;

    /// [`wattron`] manipulates the current attributes of the named window. The current attributes
    /// of a window apply to all characters that are written into the window with [`waddch`],
    /// [`waddstr`] and [`wprintw`]. Attributes are a property of the character, and move with the
    /// character through any scrolling and insert/delete line/character operations. To the extent
    /// possible, they are displayed as appropriate modifications to the graphic rendition of
    /// characters put on the screen.
    pub fn wattron(win: *mut Window, attrs: CHType) -> c_int;
}
