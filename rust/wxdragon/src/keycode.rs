//! Standard wxWidgets key codes (`WXK_*`).
//!
//! [`KeyEvent::get_key_code`](crate::event::KeyEvent::get_key_code) and
//! [`get_key_state`](crate::utils::get_key_state) both work with plain `i32`
//! values rather than a typed enum, since values below [`WXK_START`] are just
//! ASCII/Unicode code points. These constants mirror the `wxKeyCode` enum from
//! wxWidgets' `wx/defs.h`, so callers can match on named keys instead of
//! magic numbers.

/// No key; returned by some APIs to mean "not applicable".
pub const WXK_NONE: i32 = 0;

pub const WXK_CONTROL_A: i32 = 1;
pub const WXK_CONTROL_B: i32 = 2;
pub const WXK_CONTROL_C: i32 = 3;
pub const WXK_CONTROL_D: i32 = 4;
pub const WXK_CONTROL_E: i32 = 5;
pub const WXK_CONTROL_F: i32 = 6;
pub const WXK_CONTROL_G: i32 = 7;
pub const WXK_CONTROL_H: i32 = 8;
pub const WXK_CONTROL_I: i32 = 9;
pub const WXK_CONTROL_J: i32 = 10;
pub const WXK_CONTROL_K: i32 = 11;
pub const WXK_CONTROL_L: i32 = 12;
pub const WXK_CONTROL_M: i32 = 13;
pub const WXK_CONTROL_N: i32 = 14;
pub const WXK_CONTROL_O: i32 = 15;
pub const WXK_CONTROL_P: i32 = 16;
pub const WXK_CONTROL_Q: i32 = 17;
pub const WXK_CONTROL_R: i32 = 18;
pub const WXK_CONTROL_S: i32 = 19;
pub const WXK_CONTROL_T: i32 = 20;
pub const WXK_CONTROL_U: i32 = 21;
pub const WXK_CONTROL_V: i32 = 22;
pub const WXK_CONTROL_W: i32 = 23;
pub const WXK_CONTROL_X: i32 = 24;
pub const WXK_CONTROL_Y: i32 = 25;
pub const WXK_CONTROL_Z: i32 = 26;

/// Backspace.
pub const WXK_BACK: i32 = 8;
/// Tab.
pub const WXK_TAB: i32 = 9;
/// Enter/Return on the main keyboard.
pub const WXK_RETURN: i32 = 13;
/// Escape.
pub const WXK_ESCAPE: i32 = 27;

// Values from 33 to 126 are reserved for the standard ASCII characters, and
// 128 to 255 for the (platform-dependent) extended ASCII characters.

/// Space.
pub const WXK_SPACE: i32 = 32;
/// Delete (Forward Delete on macOS laptop keyboards).
pub const WXK_DELETE: i32 = 127;

/// First of the wxWidgets-specific key codes below, which have no ASCII or
/// Unicode equivalent.
pub const WXK_START: i32 = 300;
pub const WXK_LBUTTON: i32 = 301;
pub const WXK_RBUTTON: i32 = 302;
pub const WXK_CANCEL: i32 = 303;
pub const WXK_MBUTTON: i32 = 304;
pub const WXK_CLEAR: i32 = 305;
pub const WXK_SHIFT: i32 = 306;
pub const WXK_ALT: i32 = 307;
pub const WXK_CONTROL: i32 = 308;
pub const WXK_MENU: i32 = 309;
pub const WXK_PAUSE: i32 = 310;
pub const WXK_CAPITAL: i32 = 311;
pub const WXK_END: i32 = 312;
pub const WXK_HOME: i32 = 313;
pub const WXK_LEFT: i32 = 314;
pub const WXK_UP: i32 = 315;
pub const WXK_RIGHT: i32 = 316;
pub const WXK_DOWN: i32 = 317;
pub const WXK_SELECT: i32 = 318;
pub const WXK_PRINT: i32 = 319;
pub const WXK_EXECUTE: i32 = 320;
pub const WXK_SNAPSHOT: i32 = 321;
pub const WXK_INSERT: i32 = 322;
pub const WXK_HELP: i32 = 323;
pub const WXK_NUMPAD0: i32 = 324;
pub const WXK_NUMPAD1: i32 = 325;
pub const WXK_NUMPAD2: i32 = 326;
pub const WXK_NUMPAD3: i32 = 327;
pub const WXK_NUMPAD4: i32 = 328;
pub const WXK_NUMPAD5: i32 = 329;
pub const WXK_NUMPAD6: i32 = 330;
pub const WXK_NUMPAD7: i32 = 331;
pub const WXK_NUMPAD8: i32 = 332;
pub const WXK_NUMPAD9: i32 = 333;
pub const WXK_MULTIPLY: i32 = 334;
pub const WXK_ADD: i32 = 335;
pub const WXK_SEPARATOR: i32 = 336;
pub const WXK_SUBTRACT: i32 = 337;
pub const WXK_DECIMAL: i32 = 338;
pub const WXK_DIVIDE: i32 = 339;
pub const WXK_F1: i32 = 340;
pub const WXK_F2: i32 = 341;
pub const WXK_F3: i32 = 342;
pub const WXK_F4: i32 = 343;
pub const WXK_F5: i32 = 344;
pub const WXK_F6: i32 = 345;
pub const WXK_F7: i32 = 346;
pub const WXK_F8: i32 = 347;
pub const WXK_F9: i32 = 348;
pub const WXK_F10: i32 = 349;
pub const WXK_F11: i32 = 350;
pub const WXK_F12: i32 = 351;
pub const WXK_F13: i32 = 352;
pub const WXK_F14: i32 = 353;
pub const WXK_F15: i32 = 354;
pub const WXK_F16: i32 = 355;
pub const WXK_F17: i32 = 356;
pub const WXK_F18: i32 = 357;
pub const WXK_F19: i32 = 358;
pub const WXK_F20: i32 = 359;
pub const WXK_F21: i32 = 360;
pub const WXK_F22: i32 = 361;
pub const WXK_F23: i32 = 362;
pub const WXK_F24: i32 = 363;
pub const WXK_NUMLOCK: i32 = 364;
pub const WXK_SCROLL: i32 = 365;
pub const WXK_PAGEUP: i32 = 366;
pub const WXK_PAGEDOWN: i32 = 367;
pub const WXK_NUMPAD_SPACE: i32 = 368;
pub const WXK_NUMPAD_TAB: i32 = 369;
pub const WXK_NUMPAD_ENTER: i32 = 370;
pub const WXK_NUMPAD_F1: i32 = 371;
pub const WXK_NUMPAD_F2: i32 = 372;
pub const WXK_NUMPAD_F3: i32 = 373;
pub const WXK_NUMPAD_F4: i32 = 374;
pub const WXK_NUMPAD_HOME: i32 = 375;
pub const WXK_NUMPAD_LEFT: i32 = 376;
pub const WXK_NUMPAD_UP: i32 = 377;
pub const WXK_NUMPAD_RIGHT: i32 = 378;
pub const WXK_NUMPAD_DOWN: i32 = 379;
pub const WXK_NUMPAD_PAGEUP: i32 = 380;
pub const WXK_NUMPAD_PAGEDOWN: i32 = 381;
pub const WXK_NUMPAD_END: i32 = 382;
pub const WXK_NUMPAD_BEGIN: i32 = 383;
pub const WXK_NUMPAD_INSERT: i32 = 384;
pub const WXK_NUMPAD_DELETE: i32 = 385;
pub const WXK_NUMPAD_EQUAL: i32 = 386;
pub const WXK_NUMPAD_MULTIPLY: i32 = 387;
pub const WXK_NUMPAD_ADD: i32 = 388;
pub const WXK_NUMPAD_SEPARATOR: i32 = 389;
pub const WXK_NUMPAD_SUBTRACT: i32 = 390;
pub const WXK_NUMPAD_DECIMAL: i32 = 391;
pub const WXK_NUMPAD_DIVIDE: i32 = 392;
pub const WXK_WINDOWS_LEFT: i32 = 393;
pub const WXK_WINDOWS_RIGHT: i32 = 394;
pub const WXK_WINDOWS_MENU: i32 = 395;

/// The physical Control key on macOS. wx maps [`WXK_CONTROL`] to Command
/// there to match platform conventions, so this is the only way to detect
/// the actual Control key; on every other platform it's identical to
/// `WXK_CONTROL`.
#[cfg(target_os = "macos")]
pub const WXK_RAW_CONTROL: i32 = 396;
/// The physical Control key. Identical to [`WXK_CONTROL`] outside of macOS,
/// where wx maps `WXK_CONTROL` to Command instead.
#[cfg(not(target_os = "macos"))]
pub const WXK_RAW_CONTROL: i32 = WXK_CONTROL;

/// Alias for [`WXK_CONTROL`]: on macOS this is the code wx reports for
/// Command, matching wxWidgets' own `WXK_COMMAND` alias.
pub const WXK_COMMAND: i32 = WXK_CONTROL;

// Hardware-specific buttons found on some keyboards/remotes.
pub const WXK_SPECIAL1: i32 = 397;
pub const WXK_SPECIAL2: i32 = 398;
pub const WXK_SPECIAL3: i32 = 399;
pub const WXK_SPECIAL4: i32 = 400;
pub const WXK_SPECIAL5: i32 = 401;
pub const WXK_SPECIAL6: i32 = 402;
pub const WXK_SPECIAL7: i32 = 403;
pub const WXK_SPECIAL8: i32 = 404;
pub const WXK_SPECIAL9: i32 = 405;
pub const WXK_SPECIAL10: i32 = 406;
pub const WXK_SPECIAL11: i32 = 407;
pub const WXK_SPECIAL12: i32 = 408;
pub const WXK_SPECIAL13: i32 = 409;
pub const WXK_SPECIAL14: i32 = 410;
pub const WXK_SPECIAL15: i32 = 411;
pub const WXK_SPECIAL16: i32 = 412;
pub const WXK_SPECIAL17: i32 = 413;
pub const WXK_SPECIAL18: i32 = 414;
pub const WXK_SPECIAL19: i32 = 415;
pub const WXK_SPECIAL20: i32 = 416;

pub const WXK_BROWSER_BACK: i32 = 417;
pub const WXK_BROWSER_FORWARD: i32 = 418;
pub const WXK_BROWSER_REFRESH: i32 = 419;
pub const WXK_BROWSER_STOP: i32 = 420;
pub const WXK_BROWSER_SEARCH: i32 = 421;
pub const WXK_BROWSER_FAVORITES: i32 = 422;
pub const WXK_BROWSER_HOME: i32 = 423;
pub const WXK_VOLUME_MUTE: i32 = 424;
pub const WXK_VOLUME_DOWN: i32 = 425;
pub const WXK_VOLUME_UP: i32 = 426;
pub const WXK_MEDIA_NEXT_TRACK: i32 = 427;
pub const WXK_MEDIA_PREV_TRACK: i32 = 428;
pub const WXK_MEDIA_STOP: i32 = 429;
pub const WXK_MEDIA_PLAY_PAUSE: i32 = 430;
pub const WXK_LAUNCH_MAIL: i32 = 431;

// Events for the WXK_LAUNCH_* keys below are currently only generated by
// wxGTK, with the exception of WXK_LAUNCH_APP1/APP2 (aliases of A/B), which
// wxMSW also generates.
pub const WXK_LAUNCH_0: i32 = 432;
pub const WXK_LAUNCH_1: i32 = 433;
pub const WXK_LAUNCH_2: i32 = 434;
pub const WXK_LAUNCH_3: i32 = 435;
pub const WXK_LAUNCH_4: i32 = 436;
pub const WXK_LAUNCH_5: i32 = 437;
pub const WXK_LAUNCH_6: i32 = 438;
pub const WXK_LAUNCH_7: i32 = 439;
pub const WXK_LAUNCH_8: i32 = 440;
pub const WXK_LAUNCH_9: i32 = 441;
pub const WXK_LAUNCH_A: i32 = 442;
pub const WXK_LAUNCH_B: i32 = 443;
pub const WXK_LAUNCH_C: i32 = 444;
pub const WXK_LAUNCH_D: i32 = 445;
pub const WXK_LAUNCH_E: i32 = 446;
pub const WXK_LAUNCH_F: i32 = 447;

/// Portable alias for the key event generated by wxMSW and wxGTK for the
/// first extra application-launcher button on some keyboards.
pub const WXK_LAUNCH_APP1: i32 = WXK_LAUNCH_A;
/// Portable alias for the key event generated by wxMSW and wxGTK for the
/// second extra application-launcher button on some keyboards.
pub const WXK_LAUNCH_APP2: i32 = WXK_LAUNCH_B;

/// Portable way to refer to the "5" key on the numpad when Num Lock is off.
#[cfg(windows)]
pub const WXK_NUMPAD_CENTER: i32 = WXK_CLEAR;
/// Portable way to refer to the "5" key on the numpad when Num Lock is off.
#[cfg(not(windows))]
pub const WXK_NUMPAD_CENTER: i32 = WXK_NUMPAD_BEGIN;
