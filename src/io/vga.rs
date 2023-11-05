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
//   File: src/io/vga.rs
//   Desc: VGA printing implementation

// RustDoc
//! # VGA Printing Module
//!
//! This module provides an implementation for printing to the VGA text buffer. It includes macros for
//! formatted printing and a VGA text buffer writer.
//!
//! For more information about ChadOS, visit [the ChadOS GitHub repository](https://github.com/NewDawn0/ChadOS).
//!
//! ## Author
//!
//! - [NewDawn0](https://github.com/NewDawn0)
//!
//! ## License
//!
//! This code is licensed under the MIT License. See the MIT License section below for details.
//!
//! # File: src/io/vga.rs
//!
//! This file defines the VGA printing implementation for ChadOS.

// Imports
use crate::cfg::vga::*;
#[cfg(test)]
use crate::test;
use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

// Macros

/// Prints formatted text to the VGA buffer.
///
/// This macro is similar to the standard `print!` macro but outputs to the VGA buffer.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::io::vga::_print(format_args!($($arg)*), false));
}

/// Prints formatted text to the VGA buffer and keeps the last character on the line.
///
/// This macro is similar to `print!` but clears line first.
#[macro_export]
macro_rules! rprint {
    ($($arg:tt)*) => ($crate::io::vga::_print(format_args!($($arg)*), true));
}

/// Prints a newline to the VGA buffer.
///
/// This macro is similar to `println!` but outputs a newline character to the VGA buffer.
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

/// Prints a formatted message to the VGA buffer with the kernel label.
///
/// This macro is similar to `println!` but prepends a "\[KERNEL\]" label to the output.
#[macro_export]
macro_rules! kprintln {
    ($($arg:tt)*) => {
        $crate::io::vga::COL.lock().set_fg($crate::cfg::vga::KERNELP_COLOUR);
        $crate::print!("[KERNEL]");
        $crate::io::vga::COL.lock().set_fg($crate::cfg::vga::KERNELM_COLOUR);
        $crate::println!(" {}", format_args!($($arg)*));
        $crate::io::vga::COL.lock().set_default();
    };
}

/// Prints a warning message to the VGA buffer.
///
/// This macro is similar to `println!` but prepends a "\[WARN\]" label to the output.
#[macro_export]
macro_rules! wprintln {
    ($($arg:tt)*) => {
        $crate::io::vga::COL.lock().set_fg($crate::cfg::vga::WARN_COLOUR);
        $crate::println!("[WARN] {}", format_args!($($arg)*));
        $crate::io::vga::COL.lock().set_default();
    };
}

/// Prints an error message to the VGA buffer.
///
/// This macro is similar to `println!` but prepends an "\[ERROR\]" label to the output.
#[macro_export]
macro_rules! eprintln {
    ($($arg:tt)*) => {
        $crate::io::vga::COL.lock().set_fg($crate::cfg::vga::ERRP_COLOUR);
        $crate::print!("[ERROR]");
        $crate::io::vga::COL.lock().set_fg($crate::cfg::vga::ERRM_COLOUR);
        $crate::println!(" {}", format_args!($($arg)*));
        $crate::io::vga::COL.lock().set_default();
    };
}

// Globals
/// The global static Col instance colour of the text
pub static COL: Mutex<Col> = Mutex::new(Col::new(FG_COL, BG_COL));
lazy_static! {
    /// The global static writer instance for the VGA buffer.
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        buffer: unsafe { &mut *(BUF_ADDR as *mut Buffer) },
    });
}

/// The standard colour palette in VGA text mode.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Colour {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGrey = 7,
    DarkGrey = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

/// Represents a VGA text color configuration.
pub struct Col(Colour, Colour);

impl Col {
    /// Creates a new `Col` instance with the specified foreground and background colors.
    ///
    /// # Arguments
    ///
    /// * `fg`: The foreground color.
    /// * `bg`: The background color.
    pub const fn new(fg: Colour, bg: Colour) -> Self {
        Self(fg, bg)
    }

    /// Retrieves the current color configuration as a `ColourCode`.
    ///
    /// # Returns
    ///
    /// The current color configuration as a `ColourCode`.
    pub const fn get(&self) -> ColourCode {
        ColourCode::new(self.0, self.1)
    }

    /// Sets both foreground and background colors to the specified values.
    ///
    /// # Arguments
    ///
    /// * `fg`: The new foreground color.
    /// * `bg`: The new background color.
    pub fn set(&mut self, fg: Colour, bg: Colour) {
        self.0 = fg;
        self.1 = bg;
    }

    /// Sets the foreground color to the specified value.
    ///
    /// # Arguments
    ///
    /// * `fg`: The new foreground color.
    pub fn set_fg(&mut self, fg: Colour) {
        self.0 = fg
    }

    /// Sets the background color to the specified value.
    ///
    /// # Arguments
    ///
    /// * `bg`: The new background color.
    pub fn set_bg(&mut self, bg: Colour) {
        self.1 = bg
    }

    /// Restores the default color configuration (foreground and background colors).
    ///
    /// The default colors are defined by `FG_COL` and `BG_COL` constants.
    pub fn set_default(&mut self) {
        self.0 = FG_COL;
        self.1 = BG_COL
    }
}

/// VGA text mode colour code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColourCode(u8);
impl ColourCode {
    pub const fn new(fg: Colour, bg: Colour) -> Self {
        Self((bg as u8) << 4 | (fg as u8))
    }
}

/// Represents a single character on the VGA screen.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_char: u8,
    colour_code: ColourCode,
}
impl ScreenChar {
    pub const fn new(ascii_char: u8, colour_code: ColourCode) -> Self {
        Self {
            ascii_char,
            colour_code,
        }
    }
    pub fn blank() -> Self {
        Self {
            ascii_char: ASCII_BLANK,
            colour_code: COL.lock().get(),
        }
    }
}

/// Represents the VGA text buffer.
#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

/// VGA text buffer writer.
pub struct Writer {
    column_position: usize,
    buffer: &'static mut Buffer,
}

impl Writer {
    /// Writes a single byte to the VGA text buffer.
    ///
    /// This function writes a single byte to the VGA text buffer at the current position. It handles
    /// newline characters and wraps to the next line when the current line is full.
    ///
    /// # Arguments
    ///
    /// * `byte`: The byte to be written to the buffer.
    pub fn wb(&mut self, byte: u8) {
        match byte {
            b'\0' => {} // Ignore
            b'\n' => self.nl(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.nl();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                self.buffer.chars[row][col].write(ScreenChar::new(byte, COL.lock().get()));
                self.column_position += 1;
            }
        }
    }

    /// Writes a string to the VGA text buffer.
    ///
    /// This function writes a string to the VGA text buffer. It processes each byte in the string,
    /// ensuring that only printable ASCII characters or newline characters are written to the buffer.
    ///
    /// # Arguments
    ///
    /// * `s`: The string to be written to the buffer.
    fn ws(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.wb(byte),
                // not part of printable ASCII range
                _ => self.wb(0xfe),
            }
        }
    }

    /// Handles a newline character in the VGA text buffer.
    ///
    /// This function processes a newline character, moving the contents of the buffer up by one line
    /// and clearing the current line. It also resets the column position to the beginning of the line.
    fn nl(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.cr(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    /// Clears a row in the VGA text buffer.
    ///
    /// This function clears a specific row in the VGA text buffer, filling it with blank characters.
    ///
    /// # Arguments
    ///
    /// * `row`: The row index to be cleared.
    pub fn cr(&mut self, row: usize) {
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(ScreenChar::blank());
        }
    }

    /// Clears a character in the VGA text buffer.
    ///
    /// This function clears the character at the current position in the VGA text buffer.
    pub fn cc(&mut self) {
        let col = match self.column_position {
            0..=2 => self.column_position,
            3..=usize::MAX => self.column_position - 1,
            _ => 1, // Impossible, but to make the compiler happy...
        };
        self.column_position = col;
        self.buffer.chars[BUFFER_HEIGHT - 1][col].write(ScreenChar::blank())
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.ws(s);
        Ok(())
    }
}

#[doc(hidden)]
pub fn clear_char() {
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        WRITER.lock().cc();
    })
}
#[doc(hidden)]
pub fn clear_all() {
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        let mut writer = WRITER.lock();
        for row in 1..BUFFER_HEIGHT {
            writer.cr(row);
        }
        writer.cr(BUFFER_HEIGHT - 1);
        writer.column_position = 0;
    })
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments, clear: bool) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        let mut writer = WRITER.lock();
        if clear {
            writer.cr(BUFFER_HEIGHT - 1);
            writer.column_position = 0;
        }
        writer.write_fmt(args).unwrap();
    });
}

/// A prelude for working with the VGA module.
pub mod prelude {
    pub use crate::io::vga::{Colour, COL};
    pub use crate::{eprintln, kprintln, print, println, wprintln};
}

// Tests
#[test_case]
fn test_vga() {
    use crate::io::vga::prelude::*;
    COL.lock().set(Colour::Blue, Colour::Black);
    test!(
        "VGA Col.set() + Col.get()",
        assert_eq!(
            COL.lock().get(),
            ColourCode::new(Colour::Blue, Colour::Black)
        )
    );
    COL.lock().set(Colour::Cyan, Colour::Green);
    COL.lock().set_fg(Colour::Red);
    test!(
        "VGA Col.set_fg() ",
        assert_eq!(
            COL.lock().get(),
            ColourCode::new(Colour::Red, Colour::Green)
        )
    );
    COL.lock().set(Colour::Magenta, Colour::Brown);
    COL.lock().set_bg(Colour::LightGrey);
    test!(
        "VGA Col.set_bg() ",
        assert_eq!(
            COL.lock().get(),
            ColourCode::new(Colour::Magenta, Colour::LightGrey)
        )
    );
    COL.lock().set(Colour::DarkGrey, Colour::LightBlue);
    COL.lock().set_default();
    test!(
        "VGA Col.set_default() ",
        assert_eq!(COL.lock().get(), ColourCode::new(FG_COL, BG_COL))
    );
}
#[test_case]
fn test_print() {
    test!(
        "Testing print!()",
        assert_eq!(
            {
                crate::print!("Hello world");
            },
            ()
        )
    );
    test!(
        "Testing println!()",
        assert_eq!(
            {
                crate::println!("Hello world ln");
            },
            ()
        )
    );
    test!(
        "Testing kprintln!()",
        assert_eq!(
            {
                crate::kprintln!("Kernel msg");
            },
            ()
        )
    );
    test!(
        "Testing wprintln!()",
        assert_eq!(
            {
                crate::wprintln!("Warn msg");
            },
            ()
        )
    );
    test!(
        "Testing eprintln!()",
        assert_eq!(
            {
                crate::eprintln!("Err msg");
            },
            ()
        )
    );
}
