use crate::Window;
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::{getch, wgetch, ERR};

#[link(name = "curses")]
extern "C" {
    /// The [`cbreak`] routine disables line buffering and erase/kill character-processing
    /// (interrupt and flow control characters are unaffected), making characters typed by the user
    /// immediately available to the program.
    pub fn cbreak() -> c_int;

    /// The [`nocbreak`] routine returns the terminal to normal (cooked) mode.
    pub fn nocbreak() -> c_int;

    /// The [`echo`] and [`noecho`] routines control whether characters typed by the user are
    /// echoed by [`getch`] as they are typed.
    pub fn echo() -> c_int;

    /// The [`echo`] and [`noecho`] routines control whether characters typed by the user are
    /// echoed by [`getch`] as they are typed.
    pub fn noecho() -> c_int;

    /// The [`halfdelay`] routine is used for half-delay mode, which is similar to [`cbreak`] mode
    /// in that characters typed by the user are immediately available to the program. However,
    /// after blocking for `tenths` tenths of seconds, [`ERR`] is returned if nothing has been
    /// typed. The value of `tenths` must be a number between 1 and 255. Use [`nocbreak`] to leave
    /// half-delay mode.
    pub fn halfdelay(tenths: c_int) -> c_int;

    /// If the [`intrflush`] option is enabled, (`bf` is `true`), when an interrupt key is pressed
    /// on the keyboard (interrupt, break, quit) all output in the tty driver queue will be
    /// flushed, giving the effect of faster response to the interrupt, but causing curses to have
    /// the wrong idea of what is on the screen. Disabling (`bf` is `false`), the option prevents
    /// the flush. The default for the option is inherited from the tty driver settings. The window
    /// argument is ignored.
    pub fn intrflush(win: *mut Window, bf: bool) -> c_int;

    /// The [`keypad`] option enables the keypad of the user's terminal. If enabled (`bf` is
    /// `true`), the user can press a function key (such as an arrow key) and [`wgetch`] returns a
    /// single value representing the function key, as in [`KEY_LEFT`]. If disabled (`bf` is
    /// `false`), curses does not treat function keys specially and the program has to interpret
    /// the escape sequences itself. If the keypad in the terminal can be turned on (made to
    /// transmit) and off (made to work locally), turning on this option causes the terminal keypad
    /// to be turned on when [`wgetch`] is called. The default value for keypad is false.
    pub fn keypad(win: *mut Window, bf: bool) -> c_int;

    /// Initially, whether the terminal returns 7 or 8 significant bits on input depends on the
    /// control mode of the tty driver. To force 8 bits to be returned, invoke `meta(win, TRUE)`;
    /// this is equivalent, under POSIX, to setting the CS8 flag on the terminal. To force 7 bits
    /// to be returned, invoke `meta(win, FALSE)`; this is equivalent, under POSIX, to setting the
    /// CS7 flag on the terminal. The window argument, `win`, is always ignored. If the terminfo
    /// capabilities `smm` (meta_on) and `rmm` (meta_off) are defined for the terminal, `smm` is
    /// sent to the terminal when `meta(win, TRUE)` is called and `rmm` is sent when
    /// `meta(win, FALSE)` is called.
    pub fn meta(win: *mut Window, bf: bool) -> c_int;

    /// The [`nodelay`] option causes [`getch`] to be a non-blocking call. If no input is ready,
    /// [`getch`] returns [`ERR`]. If disabled (`bf` is `false`), [`getch`] waits until a key is
    /// pressed.
    pub fn nodelay(win: *mut Window, bf: bool) -> c_int;

    /// The [`raw`] and [`noraw`] routines place the terminal into or out of raw mode. Raw mode is
    /// similar to cbreak mode, in that characters typed are immediately passed through to the user
    /// program. The differences are that in raw mode, the interrupt, quit, suspend, and flow
    /// control characters are all passed through uninterpreted, instead of generating a signal.
    /// The behavior of the BREAK key depends on other bits in the tty driver that are not set by
    /// curses.
    pub fn raw(win: *mut Window, bf: bool) -> c_int;

    /// The [`raw`] and [`noraw`] routines place the terminal into or out of raw mode. Raw mode is
    /// similar to cbreak mode, in that characters typed are immediately passed through to the user
    /// program. The differences are that in raw mode, the interrupt, quit, suspend, and flow
    /// control characters are all passed through uninterpreted, instead of generating a signal.
    /// The behavior of the BREAK key depends on other bits in the tty driver that are not set by
    /// curses.
    pub fn noraw(win: *mut Window, bf: bool) -> c_int;

    /// When the [`noqiflush`] routine is used, normal flush of input and output queues associated
    /// with the INTR, QUIT and SUSP characters will not be done
    pub fn noqiflush();

    /// When [`qiflush`] is called, the queues will be flushed when these control characters are
    /// read
    pub fn qiflush();

    /// While interpreting an input escape sequence, [`wgetch`] sets a timer while waiting for the
    /// next character. If `notimeout(win, TRUE)` is called, then [`wgetch`] does not set a timer.
    /// The purpose of the timeout is to differentiate between sequences received from a function
    /// key and those typed by a user.
    pub fn notimeout(win: *mut Window, bf: bool) -> c_int;

    /// The [`timeout`] and [`wtimeout`] routines set blocking or non-blocking read for a given
    /// window. If `delay` is negative, blocking read is used (i.e., waits indefinitely for input).
    /// If `delay` is zero, then non-blocking read is used (i.e., read returns [`ERR`] if no input
    /// is waiting). If `delay` is positive, then read blocks for `delay` milliseconds, and returns
    /// [`ERR`] if there is still no input. Hence, these routines provide the same functionality as
    /// [`nodelay`], plus the additional capability of being able to block for only `delay`
    /// milliseconds (where `delay` is positive).
    pub fn timeout(delay: c_int);

    /// The [`timeout`] and [`wtimeout`] routines set blocking or non-blocking read for a given
    /// window. If `delay` is negative, blocking read is used (i.e., waits indefinitely for input).
    /// If `delay` is zero, then non-blocking read is used (i.e., read returns [`ERR`] if no input
    /// is waiting). If `delay` is positive, then read blocks for `delay` milliseconds, and returns
    /// [`ERR`] if there is still no input. Hence, these routines provide the same functionality as
    /// [`nodelay`], plus the additional capability of being able to block for only `delay`
    /// milliseconds (where `delay` is positive).
    pub fn wtimeout(win: *mut Window, delay: c_int);
}
