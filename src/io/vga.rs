use crate::cfg::vga::*;
use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

#[cfg(test)]
use crate::test;

pub static COL: Mutex<Col> = Mutex::new(Col::new(FG_COL, BG_COL));
lazy_static! {
    /// A global `Writer` instance that can be used for printing to the VGA text buffer.
    ///
    /// Used by the `print!` and `println!` macros.
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

pub struct Col(Colour, Colour);
impl Col {
    pub const fn new(fg: Colour, bg: Colour) -> Self {
        Self(fg, bg)
    }
    pub const fn get(&self) -> ColourCode {
        ColourCode::new(self.0, self.1)
    }
    pub fn set(&mut self, fg: Colour, bg: Colour) {
        self.0 = fg;
        self.1 = bg;
    }
    pub fn set_fg(&mut self, fg: Colour) {
        self.0 = fg
    }
    pub fn set_bg(&mut self, bg: Colour) {
        self.1 = bg
    }
    pub fn set_default(&mut self) {
        self.0 = FG_COL;
        self.1 = BG_COL
    }
}

/// A combination of a foreground and a background colour.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColourCode(u8);
impl ColourCode {
    /// Create a new `colourCode` with the given foreground and background colours.
    pub const fn new(fg: Colour, bg: Colour) -> Self {
        Self((bg as u8) << 4 | (fg as u8))
    }
}

/// A screen character in the VGA text buffer, consisting of an ASCII character and a `colourCode`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    colour_code: ColourCode,
}

/// A structure representing the VGA text buffer.
#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

/// A writer type that allows writing ASCII bytes and strings to an underlying `Buffer`.
///
/// Wraps lines at `BUFFER_WIDTH`. Supports newline characters and implements the
/// `core::fmt::Write` trait.
pub struct Writer {
    column_position: usize,
    buffer: &'static mut Buffer,
}

impl Writer {
    /// Writes an ASCII byte to the buffer.
    ///
    /// Wraps lines at `BUFFER_WIDTH`. Supports the `\n` newline character.
    pub fn wb(&mut self, byte: u8) {
        match byte {
            b'\n' => self.nl(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.nl();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    colour_code: COL.lock().get(),
                });
                self.column_position += 1;
            }
        }
    }

    /// Writes the given ASCII string to the buffer.
    ///
    /// Wraps lines at `BUFFER_WIDTH`. Supports the `\n` newline character. Does **not**
    /// support strings with non-ASCII characters, since they can't be printed in the VGA text
    /// mode.
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

    /// Shifts all lines one line up and clears the last row.
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

    /// Clears a row by overwriting it with blank characters.
    fn cr(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: ASCII_BLANK,
            colour_code: COL.lock().get(),
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.ws(s);
        Ok(())
    }
}

/// Like the `print!` macro in the standard library, but prints to the VGA text buffer.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::io::vga::_print(format_args!($($arg)*)));
}

/// Like the `println!` macro in the standard library, but prints to the VGA text buffer.
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
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
#[macro_export]
macro_rules! wprintln {
    ($($arg:tt)*) => {
        $crate::io::vga::COL.lock().set_fg($crate::cfg::vga::WARN_COLOUR);
        $crate::println!("[WARN] {}", format_args!($($arg)*));
        $crate::io::vga::COL.lock().set_default();
    };
}
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

/// Prints the given formatted string to the VGA text buffer
/// through the global `WRITER` instance.
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}

pub mod prelude {
    pub use crate::io::vga::{Colour, COL};
    pub use crate::{eprintln, kprintln, print, println, wprintln};
}

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
