use crate::{CHType, Window};
use std::ffi::c_int;

#[link(name = "curses")]
extern "C" {
    /// The [`bkgdset`] and [`wbkgdset`] routines manipulate the background of the named window.
    /// The window background is a [`CHType`] consisting of any combination of attributes (i.e.,
    /// rendition) and a character. The attribute part of the background is combined (OR'ed) with
    /// all non-blank characters that are written into the window with [`waddch`]. Both the
    /// character and attribute parts of the background are combined with the blank characters. The
    /// background becomes a property of the character and moves with the character through any
    /// scrolling and insert/delete line/character operations.
    pub fn bkgdset(ch: CHType);

    /// The [`bkgdset`] and [`wbkgdset`] routines manipulate the background of the named window.
    /// The window background is a [`CHType`] consisting of any combination of attributes (i.e.,
    /// rendition) and a character. The attribute part of the background is combined (OR'ed) with
    /// all non-blank characters that are written into the window with [`waddch`]. Both the
    /// character and attribute parts of the background are combined with the blank characters. The
    /// background becomes a property of the character and moves with the character through any
    /// scrolling and insert/delete line/character operations.
    pub fn wbkgdset(win: *mut Window, ch: CHType);

    /// The [`bkgd`] and [`wbkgd`] functions set the background property of the current or
    /// specified window and then apply this setting to every character position in that window:
    ///  - The rendition of every character on the screen is changed to the new background
    ///    rendition.
    ///  - Wherever the former background character appears, it is changed to the new background
    ///    character.
    pub fn bkgd(ch: CHType) -> c_int;

    /// The [`bkgd`] and [`wbkgd`] functions set the background property of the current or
    /// specified window and then apply this setting to every character position in that window:
    ///  - The rendition of every character on the screen is changed to the new background
    ///    rendition.
    ///  - Wherever the former background character appears, it is changed to the new background
    ///    character.
    pub fn wbkgd(win: *mut Window, ch: CHType) -> c_int;

    /// The [`getbkgd`] function returns the given window's current background character/attribute
    /// pair.
    pub fn getbkgd(win: *mut Window) -> CHType;
}
