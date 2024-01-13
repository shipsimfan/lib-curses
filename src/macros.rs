/// Creates a properly shifted attribute
#[macro_export]
macro_rules! curses_bits {
    ($mask: expr, $shift: expr) => {
        $mask << ($shift + CURSES_ATTR_SHIFT)
    };
}

/// Creates color attributes
#[macro_export]
macro_rules! color_pair {
    ($n: expr) => {
        $crate::curses_bits!($n, 0) & $crate::A_COLOR
    };
}
