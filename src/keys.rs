//     ____ _               _  ___  ____
//    / ___| |__   __ _  __| |/ _ \/ ___|
//   | |   | '_ \ / _` |/ _` | | | \___ \
//   | |___| | | | (_| | (_| | |_| |___) |
//    \____|_| |_|\__,_|\__,_|\___/|____/
//    https://github.com/NewDawn0/ChadOS
//
//   @Author: NewDawn0
//   @Contributors: -
//   @License: MIT
//
//   File: src/keys.rs
//   Desc: Keyboard implemenation

// RustDoc
//! # ChadOS Keyboard Handling
//!
//! This module provides keyboard handling functionality for ChadOS, including the definition of keyboard layouts,
//! handling key events, and setting up the keyboard interrupt handler.
//!
//! For more information about ChadOS, visit [the ChadOS GitHub repository](https://github.com/NewDawn0/ChadOS).
//!
//! ## Author
//!
//! - [NewDawn0](https://github.com/NewDawn0)
//!
//! ## License
//!
//! This code is licensed under the MIT License.
//!
//! # File: src/keys.rs
//!
//! This file contains the implementation of keyboard handling for ChadOS.

// Imports
#[cfg(test)]
use crate::test;
use crate::{
    cfg::{interrupt::KEYBOARD_PORT, keys::LAYOUT},
    interrupt::handler::set_irq_handler,
    io::vga::clear_char,
    print,
};
use lazy_static::lazy_static;
use pc_keyboard::{
    layouts, DecodedKey, Error, HandleControl::MapLettersToUnicode, KeyCode as KC, KeyEvent,
    Keyboard, ScancodeSet1,
};
use spin::{Mutex, RwLock};
use x86_64::instructions::port::Port;

// Macros
macro_rules! layout {
    ($layout:ident) => {
        Keyboard::new(ScancodeSet1::new(), layouts::$layout, MapLettersToUnicode)
    };
}

// Globals
static KEYBOARD: Mutex<Layout> = Mutex::new(Layout::new(LAYOUT));
lazy_static! {
    /// A read-write lock containing the key handler function.
    pub static ref KEY_HANDLER: RwLock<fn(c: char, mods: Modifiers)> =
        RwLock::new(default_key_handler);
}

/// Enumeration of wrapped keyboard layouts.
#[derive(Copy, Clone)]
pub enum WrappedLayout {
    Us,
    Uk,
    Azerty,
    De,
    Jis,
    Colemak,
    Dvorak,
    Dvp,
}

/// Enum representing keyboard layouts.
pub enum Layout {
    Us(Keyboard<layouts::Us104Key, ScancodeSet1>),
    Uk(Keyboard<layouts::Uk105Key, ScancodeSet1>),
    Azerty(Keyboard<layouts::Azerty, ScancodeSet1>),
    De(Keyboard<layouts::De105Key, ScancodeSet1>),
    Jis(Keyboard<layouts::Jis109Key, ScancodeSet1>),
    Colemak(Keyboard<layouts::Colemak, ScancodeSet1>),
    Dvorak(Keyboard<layouts::Dvorak104Key, ScancodeSet1>),
    Dvp(Keyboard<layouts::DVP104Key, ScancodeSet1>),
}

impl Layout {
    /// Creates a new instance of the `Layout` enum based on the specified `WrappedLayout`.
    pub const fn new(layout: WrappedLayout) -> Self {
        match layout {
            WrappedLayout::Us => Layout::Us(layout!(Us104Key)),
            WrappedLayout::Uk => Layout::Uk(layout!(Uk105Key)),
            WrappedLayout::Azerty => Layout::Azerty(layout!(Azerty)),
            WrappedLayout::De => Layout::De(layout!(De105Key)),
            WrappedLayout::Jis => Layout::Jis(layout!(Jis109Key)),
            WrappedLayout::Colemak => Layout::Colemak(layout!(Colemak)),
            WrappedLayout::Dvorak => Layout::Dvorak(layout!(Dvorak104Key)),
            WrappedLayout::Dvp => Layout::Dvp(layout!(DVP104Key)),
        }
    }

    /// Adds a byte to the layout and returns a `KeyEvent` if one is available.
    fn add_byte(&mut self, code: u8) -> Result<Option<KeyEvent>, Error> {
        match self {
            Layout::Us(inner) => inner.add_byte(code),
            Layout::Uk(inner) => inner.add_byte(code),
            Layout::Azerty(inner) => inner.add_byte(code),
            Layout::De(inner) => inner.add_byte(code),
            Layout::Jis(inner) => inner.add_byte(code),
            Layout::Colemak(inner) => inner.add_byte(code),
            Layout::Dvorak(inner) => inner.add_byte(code),
            Layout::Dvp(inner) => inner.add_byte(code),
        }
    }

    /// Processes a `KeyEvent` and returns a `DecodedKey`.
    fn process_keyevent(&mut self, event: KeyEvent) -> Option<DecodedKey> {
        match self {
            Layout::Us(inner) => inner.process_keyevent(event),
            Layout::Uk(inner) => inner.process_keyevent(event),
            Layout::Azerty(inner) => inner.process_keyevent(event),
            Layout::De(inner) => inner.process_keyevent(event),
            Layout::Jis(inner) => inner.process_keyevent(event),
            Layout::Colemak(inner) => inner.process_keyevent(event),
            Layout::Dvorak(inner) => inner.process_keyevent(event),
            Layout::Dvp(inner) => inner.process_keyevent(event),
        }
    }
}

/// Initializes the keyboard handling for ChadOS.
pub fn init() {
    set_irq_handler(1, key_handler)
}

/// Reads a scancode from the keyboard port.
fn read_scancode() -> u8 {
    unsafe { Port::new(KEYBOARD_PORT).read() }
}

/// Modifiers for keyboard input.
#[repr(packed)]
pub struct Modifiers {
    // Standard
    pub shift: bool, // byte 1
    pub alt: bool,   // byte 2
    pub ctrl: bool,  // byte 3
    pub meta: bool,  // byte 4
    // Non-Standard: to fill the 8-bits
    pub clear: bool, // byte 5
    pub tab: bool,   // byte 6
    pub enter: bool, // byte 7
    pub caps: bool,  // byte 8
}
impl Modifiers {
    /// Creates a new instance of `Modifiers` with default values.
    pub const fn new() -> Self {
        Self {
            // Standard
            shift: false, // byte 1
            alt: false,   // byte 2
            ctrl: false,  // byte 3
            meta: false,  // byte 4
            // Non-Standard: to fill the 8-bits
            clear: false, // byte 5
            tab: false,   // byte 6
            enter: false, // byte 7
            caps: false,  // byte 8
        }
    }
}

/// Default key handler function that prints characters to the console.
fn default_key_handler(c: char, mods: Modifiers) {
    if mods.clear {
        clear_char();
    } else if mods.caps || mods.shift {
        print!("{}", c.to_uppercase());
    } else {
        print!("{}", c)
    }
}

/// Keyboard interrupt handler function.
fn key_handler() {
    let mut keyboard = KEYBOARD.lock();
    let code = read_scancode();
    if let Ok(Some(event)) = keyboard.add_byte(code) {
        let mut flags = Modifiers::new();
        match event.code {
            KC::LAlt | KC::RAlt2 => {
                flags.alt = true;
            }
            KC::LShift | KC::RShift => {
                flags.shift = true;
            }
            KC::LControl | KC::RControl | KC::RControl2 => {
                flags.ctrl = true;
            }
            KC::LWin | KC::RWin => {
                flags.meta = true;
            }
            KC::Return => {
                flags.enter = true;
            }
            _ => {}
        }
        if let Some(key) = keyboard.process_keyevent(event) {
            match key {
                DecodedKey::Unicode('\u{8}') => {
                    flags.clear = true;
                    KEY_HANDLER.read()('\0', flags);
                }
                DecodedKey::Unicode('\t') => {
                    flags.tab = true;
                    KEY_HANDLER.read()('\0', flags);
                }
                DecodedKey::Unicode(c) => KEY_HANDLER.read()(c, flags),
                _ => {}
            };
        }
    }
}

// Tests
#[test_case]
fn test_layout_creation() {
    let layouts = [
        WrappedLayout::Us,
        WrappedLayout::Uk,
        WrappedLayout::Azerty,
        WrappedLayout::De,
        WrappedLayout::Jis,
        WrappedLayout::Colemak,
        WrappedLayout::Dvorak,
        WrappedLayout::Dvp,
    ];
    for layout in layouts.iter() {
        let _ = Layout::new(*layout);
    }
    test!("KEYZ create layouts", assert_eq!(1, 1));
}
#[test_case]
fn test_key_handler() {
    use core::mem::ManuallyDrop;
    use pc_keyboard::KeyState as KS;
    let mut layout = ManuallyDrop::new(Layout::Us(layout!(Us104Key)));
    let event = KeyEvent {
        state: KS::Down,
        code: KC::A,
    };
    let res = layout.process_keyevent(event);
    test!(
        "KEYZ process_keyevent()",
        assert_eq!(res, Some(DecodedKey::Unicode('a')))
    )
}
