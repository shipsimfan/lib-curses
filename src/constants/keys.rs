/// A wchar_t contains a key code
pub const KEY_CODE_YES: i32 = 0o400;

/// Minimum curses key
pub const KEY_MIN: i32 = 0o401;

/// Break key (unreliable)
pub const KEY_BREAK: i32 = 0o401;

/// Soft (partial) reset (unreliable)
pub const KEY_SRESET: i32 = 0o530;

/// Reset or hard reset (unreliable)
pub const KEY_RESET: i32 = 0o531;

/// down-arrow key
pub const KEY_DOWN: i32 = 0o402;

/// up-arrow key
pub const KEY_UP: i32 = 0o403;

/// left-arrow key
pub const KEY_LEFT: i32 = 0o404;

/// right-arrow key
pub const KEY_RIGHT: i32 = 0o405;

/// home key
pub const KEY_HOME: i32 = 0o406;

/// backspace key
pub const KEY_BACKSPACE: i32 = 0o407;

/// Function keys.  Space for 64
pub const KEY_F0: i32 = 0o410;

/// delete-line key
pub const KEY_DL: i32 = 0o510;

/// insert-line key
pub const KEY_IL: i32 = 0o511;

/// delete-character key
pub const KEY_DC: i32 = 0o512;

/// insert-character key
pub const KEY_IC: i32 = 0o513;

/// sent by rmir or smir in insert mode
pub const KEY_EIC: i32 = 0o514;

/// clear-screen or erase key
pub const KEY_CLEAR: i32 = 0o515;

/// clear-to-end-of-screen key
pub const KEY_EOS: i32 = 0o516;

/// clear-to-end-of-line key
pub const KEY_EOL: i32 = 0o517;

/// scroll-forward key
pub const KEY_SF: i32 = 0o520;

/// scroll-backward key
pub const KEY_SR: i32 = 0o521;

/// next-page key
pub const KEY_NPAGE: i32 = 0o522;

/// previous-page key
pub const KEY_PPAGE: i32 = 0o523;

/// set-tab key
pub const KEY_STAB: i32 = 0o524;

/// clear-tab key
pub const KEY_CTAB: i32 = 0o525;

/// clear-all-tabs key
pub const KEY_CATAB: i32 = 0o526;

/// enter/send key
pub const KEY_ENTER: i32 = 0o527;

/// print key
pub const KEY_PRINT: i32 = 0o532;

/// lower-left key (home down)
pub const KEY_LL: i32 = 0o533;

/// upper left of keypad
pub const KEY_A1: i32 = 0o534;

/// upper right of keypad
pub const KEY_A3: i32 = 0o535;

/// center of keypad
pub const KEY_B2: i32 = 0o536;

/// lower left of keypad
pub const KEY_C1: i32 = 0o537;

/// lower right of keypad
pub const KEY_C3: i32 = 0o540;

/// back-tab key
pub const KEY_BTAB: i32 = 0o541;

/// begin key
pub const KEY_BEG: i32 = 0o542;

/// cancel key
pub const KEY_CANCEL: i32 = 0o543;

/// close key
pub const KEY_CLOSE: i32 = 0o544;

/// command key
pub const KEY_COMMAND: i32 = 0o545;

/// copy key
pub const KEY_COPY: i32 = 0o546;

/// create key
pub const KEY_CREATE: i32 = 0o547;

/// end key
pub const KEY_END: i32 = 0o550;

/// exit key
pub const KEY_EXIT: i32 = 0o551;

/// find key
pub const KEY_FIND: i32 = 0o552;

/// help key
pub const KEY_HELP: i32 = 0o553;

/// mark key
pub const KEY_MARK: i32 = 0o554;

/// message key
pub const KEY_MESSAGE: i32 = 0o555;

/// move key
pub const KEY_MOVE: i32 = 0o556;

/// next key
pub const KEY_NEXT: i32 = 0o557;

/// open key
pub const KEY_OPEN: i32 = 0o560;

/// options key
pub const KEY_OPTIONS: i32 = 0o561;

/// previous key
pub const KEY_PREVIOUS: i32 = 0o562;

/// redo key
pub const KEY_REDO: i32 = 0o563;

/// reference key
pub const KEY_REFERENCE: i32 = 0o564;

/// refresh key
pub const KEY_REFRESH: i32 = 0o565;

/// replace key
pub const KEY_REPLACE: i32 = 0o566;

/// restart key
pub const KEY_RESTART: i32 = 0o567;

/// resume key
pub const KEY_RESUME: i32 = 0o570;

/// save key
pub const KEY_SAVE: i32 = 0o571;

/// shifted begin key
pub const KEY_SBEG: i32 = 0o572;

/// shifted cancel key
pub const KEY_SCANCEL: i32 = 0o573;

/// shifted command key
pub const KEY_SCOMMAND: i32 = 0o574;

/// shifted copy key
pub const KEY_SCOPY: i32 = 0o575;

/// shifted create key
pub const KEY_SCREATE: i32 = 0o576;

/// shifted delete-character key
pub const KEY_SDC: i32 = 0o577;

/// shifted delete-line key
pub const KEY_SDL: i32 = 0o600;

/// select key
pub const KEY_SELECT: i32 = 0o601;

/// shifted end key
pub const KEY_SEND: i32 = 0o602;

/// shifted clear-to-end-of-line key
pub const KEY_SEOL: i32 = 0o603;

/// shifted exit key
pub const KEY_SEXIT: i32 = 0o604;

/// shifted find key
pub const KEY_SFIND: i32 = 0o605;

/// shifted help key
pub const KEY_SHELP: i32 = 0o606;

/// shifted home key
pub const KEY_SHOME: i32 = 0o607;

/// shifted insert-character key
pub const KEY_SIC: i32 = 0o610;

/// shifted left-arrow key
pub const KEY_SLEFT: i32 = 0o611;

/// shifted message key
pub const KEY_SMESSAGE: i32 = 0o612;

/// shifted move key
pub const KEY_SMOVE: i32 = 0o613;

/// shifted next key
pub const KEY_SNEXT: i32 = 0o614;

/// shifted options key
pub const KEY_SOPTIONS: i32 = 0o615;

/// shifted previous key
pub const KEY_SPREVIOUS: i32 = 0o616;

/// shifted print key
pub const KEY_SPRINT: i32 = 0o617;

/// shifted redo key
pub const KEY_SREDO: i32 = 0o620;

/// shifted replace key
pub const KEY_SREPLACE: i32 = 0o621;

/// shifted right-arrow key
pub const KEY_SRIGHT: i32 = 0o622;

/// shifted resume key
pub const KEY_SRSUME: i32 = 0o623;

/// shifted save key
pub const KEY_SSAVE: i32 = 0o624;

/// shifted suspend key
pub const KEY_SSUSPEND: i32 = 0o625;

/// shifted undo key
pub const KEY_SUNDO: i32 = 0o626;

/// suspend key
pub const KEY_SUSPEND: i32 = 0o627;

/// undo key
pub const KEY_UNDO: i32 = 0o630;

/// Mouse event has occurred
pub const KEY_MOUSE: i32 = 0o631;
