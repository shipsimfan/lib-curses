use std::ffi::{c_int, c_short};

// rustdoc imports
#[allow(unused_imports)]
use crate::{color_pair, initscr};

#[link(name = "curses")]
extern "C" {
    /// The [`start_color`] routine requires no arguments. It must be called if the programmer
    /// wants to use colors, and before any other color manipulation routine is called. It is good
    /// practice to call this routine right after [`initscr`]. [`start_color`] initializes eight
    /// basic colors (black, red, green, yellow, blue, magenta, cyan, and white), and two global
    /// variables, [`COLORS`] and [`COLOR_PAIRS`] (respectively defining the maximum number of
    /// colors and color-pairs the terminal can support). It also restores the colors on the
    /// terminal to the values they had when the terminal was just turned on.
    pub fn start_color() -> c_int;

    /// The [`init_pair`] routine changes the definition of a color-pair. It takes three arguments:
    /// the number of the color-pair to be changed, the foreground color number, and the background
    /// color number. For portable applications:
    ///  - The value of the first argument must be between `1` and `COLOR_PAIRS-1`, except that if
    ///    default colors are used (see [`use_default_colors`]) the upper limit is adjusted to
    ///    allow for extra pairs which use a default color in foreground and/or background.
    ///  - The value of the second and third arguments must be between `0` and [`COLORS`]. Color
    ///    pair 0 is assumed to be white on black, but is actually whatever the terminal implements
    ///    before color is initialized. It cannot be modified by the application.
    ///
    /// If the color-pair was previously initialized, the screen is refreshed and all occurrences
    /// of that color-pair are changed to the new definition.
    ///
    /// As an extension, ncurses allows you to set color pair 0 via the [`assume_default_colors`]
    /// routine, or to specify the use of default colors (color number `-1`) if you first invoke
    /// the [`use_default_colors`] routine.
    pub fn init_pair(pair: c_short, f: c_short, b: c_short) -> c_int;

    /// The [`init_color`] routine changes the definition of a color. It takes four arguments: the
    /// number of the color to be changed followed by three RGB values (for the amounts of red,
    /// green, and blue components). The value of the first argument must be between `0` and
    /// [`COLORS`]. Each of the last three arguments must be a value between `0` and `1000`. When
    /// [`init_color`] is used, all occurrences of that color on the screen immediately change to
    /// the new definition.
    pub fn init_color(color: c_short, r: c_short, b: c_short, g: c_short) -> c_int;

    /// The [`has_colors`] routine requires no arguments. It returns `true` if the terminal can
    /// manipulate colors; otherwise, it returns `false`. This routine facilitates writing
    /// terminal-independent programs. For example, a programmer can use it to decide whether to
    /// use color or some other video attribute.
    pub fn has_colors() -> bool;

    /// The [`can_change_color`] routine requires no arguments. It returns `true` if the terminal
    /// supports colors and can change their definitions; other, it returns `false`. This routine
    /// facilitates writing terminal-independent programs.
    pub fn can_change_color() -> bool;

    /// The [`color_content`] routine gives programmers a way to find the intensity of the red,
    /// green, and blue (RGB) components in a color. It requires four arguments: the color number,
    /// and three addresses of shorts for storing the information about the amounts of red, green,
    /// and blue components in the given color. The value of the first argument must be between `0`
    /// and [`COLORS`]. The values that are stored at the addresses pointed to by the last three
    /// arguments are between 0 (no component) and 1000 (maximum amount of component).
    pub fn color_content(
        color: c_short,
        r: *mut c_short,
        g: *mut c_short,
        b: *mut c_short,
    ) -> c_int;

    /// The [`pair_content`] routine allows programmers to find out what colors a given color-pair
    /// consists of. It requires three arguments: the color-pair number, and two addresses of
    /// shorts for storing the foreground and the background color numbers. The value of the first
    /// argument must be between `1` and `COLOR_PAIRS-1`. The values that are stored at the
    /// addresses pointed to by the second and third arguments are between `0` and [`COLORS`].
    pub fn pair_content(short: c_short, f: *mut c_short, b: *mut c_short) -> c_int;
}
