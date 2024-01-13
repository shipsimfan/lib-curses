use crate::{curses_bits, CHType};

/// The offset for attribute bits
pub const CURSES_ATTR_SHIFT: CHType = 8;

/// Normal text
pub const A_NORMAL: CHType = 0;

/// Mask for attributes in a [`CHType`]
pub const A_ATTRIBUTES: CHType = curses_bits!(!0, 0);

/// Mask for characters in a [`CHType`]
pub const A_CHARTEXT: CHType = curses_bits!(1, 0) - 1;

/// Mask for colors in a [`CHType`]
pub const A_COLOR: CHType = curses_bits!((1 << 8) - 1, 0);

/// Standout text
pub const A_STANDOUT: CHType = curses_bits!(1, 8);

/// Underlined text
pub const A_UNDERLINE: CHType = curses_bits!(1, 9);

/// Reversed text
pub const A_REVERSE: CHType = curses_bits!(1, 10);

/// Blinking text
pub const A_BLINK: CHType = curses_bits!(1, 11);

/// Dim text
pub const A_DIM: CHType = curses_bits!(1, 12);

/// Bold text
pub const A_BOLD: CHType = curses_bits!(1, 13);

/// Use an alternative character set
pub const A_ALTCHARSET: CHType = curses_bits!(1, 14);

/// Invisible text
pub const A_INVIS: CHType = curses_bits!(1, 15);

/// Protected text
pub const A_PROTECT: CHType = curses_bits!(1, 16);

/// Horizontal text
pub const A_HORIZONTAL: CHType = curses_bits!(1, 17);

/// Left text
pub const A_LEFT: CHType = curses_bits!(1, 18);

/// Low text
pub const A_LOW: CHType = curses_bits!(1, 19);

/// Right text
pub const A_RIGHT: CHType = curses_bits!(1, 20);

/// Top text
pub const A_TOP: CHType = curses_bits!(1, 21);

/// Vertical text
pub const A_VERTICAL: CHType = curses_bits!(1, 22);
