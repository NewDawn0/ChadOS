use crate::cfg::vga::*;
#[cfg(test)]
use crate::test;
use core::fmt::{self, Arguments};
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

pub(crate) static mut WRITER_COL: Mutex<ColourCode> = Mutex::new(ColourCode::new(FG_COL, BG_COL));
lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_pos: 0,
        buf: unsafe { &mut *(BUF_ADDR as *mut Buf) }
    });
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColourCode {
    fg: Colour,
    bg: Colour,
}
#[allow(unused)]
impl ColourCode {
    pub const fn new(fg: Colour, bg: Colour) -> Self {
        Self { fg, bg }
    }
    pub fn as_u8(&self) -> u8 {
        (self.bg as u8) << 4 | (self.fg as u8)
    }
    pub fn get() -> Self {
        unsafe { *WRITER_COL.lock() }
    }
    pub fn set(colour_code: Self) {
        unsafe { *WRITER_COL.lock() = colour_code }
    }
    pub fn set_bg(bg: Colour) {
        let fg = Self::get().fg;
        Self::set(Self::new(fg, bg))
    }
    pub fn set_fg(fg: Colour) {
        let bg = Self::get().bg;
        Self::set(Self::new(fg, bg))
    }
    pub fn default() {
        Self::set(ColourCode::new(FG_COL, BG_COL));
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_char: u8,
    colour_code: u8,
}

impl ScreenChar {
    const fn new(ascii_char: u8, colour_code: u8) -> Self {
        Self {
            ascii_char,
            colour_code,
        }
    }
}

#[repr(transparent)]
struct Buf {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_pos: usize,
    buf: &'static mut Buf,
}

impl Writer {
    fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_pos >= BUFFER_WIDTH {
                    self.new_line();
                }
                let row = BUFFER_HEIGHT - 1;
                self.buf.chars[row][self.column_pos]
                    .write(ScreenChar::new(byte, unsafe { WRITER_COL.lock().as_u8() }));
                self.column_pos += 1;
            }
        }
    }
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(NON_PRINTABLE),
            }
        }
    }
    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let char = self.buf.chars[row][col].read();
                self.buf.chars[row - 1][col].write(char);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_pos = 0;
    }
    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar::new(ASCII_BLANK, unsafe { WRITER_COL.lock().as_u8() });
        for col in 0..BUFFER_WIDTH {
            self.buf.chars[row][col].write(blank)
        }
    }
}
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::io::vga::_print(format_args!($($arg)*)));
}
#[macro_export]
macro_rules! eprintln {
    ($($arg:tt)*) => {
        $crate::io::vga::ColourCode::set_fg($crate::cfg::global::ERR_PRINT_COLOUR);
        $crate::print!("[ERROR]");
        $crate::io::vga::ColourCode::set_fg($crate::cfg::global::PANIC_COLOUR);
        $crate::print!("  {}\n", format_args!($($arg)*));
        $crate::io::vga::ColourCode::default();
    };
}
#[macro_export]
macro_rules! kprintln {
    ($($arg:tt)*) => {
        if ($crate::cfg::global::DEBUG) {
            $crate::io::vga::ColourCode::set_fg($crate::cfg::global::KERNEL_PRINT_COLOUR);
            $crate::print!("[KERNEL]");
            $crate::io::vga::ColourCode::set_fg($crate::cfg::global::KPRINTLN_COLOUR);
            $crate::print!(" {}\n", format_args!($($arg)*));
            $crate::io::vga::ColourCode::default();
        }
    };
}
#[doc(hidden)]
pub fn _print(args: Arguments) {
    use core::fmt::Write;
    WRITER
        .lock()
        .write_fmt(args)
        .expect("Printing to VGA failed")
}

#[test_case]
fn test_set_colour() {
    ColourCode::set(ColourCode::new(Colour::LightGreen, Colour::Black));
    test!(
        "VGA set colour",
        assert_eq!(
            ColourCode::get(),
            ColourCode::new(Colour::LightGreen, Colour::Black)
        )
    )
}
#[test_case]
fn test_set_bg_colour() {
    ColourCode::set(ColourCode::new(Colour::LightGreen, Colour::Black));
    ColourCode::set_bg(Colour::Blue);
    test!(
        "VGA set bg colour",
        assert_eq!(
            ColourCode::get(),
            ColourCode::new(Colour::LightGreen, Colour::Blue)
        )
    )
}
#[test_case]
fn test_set_fg_colour() {
    ColourCode::set(ColourCode::new(Colour::LightGreen, Colour::Black));
    ColourCode::set_fg(Colour::LightRed);
    test!(
        "VGA set fg colour",
        assert_eq!(
            ColourCode::get(),
            ColourCode::new(Colour::LightRed, Colour::Black)
        )
    )
}
#[test_case]
fn test_get_colour() {
    ColourCode::set(ColourCode::new(Colour::LightGreen, Colour::Black));
    test!(
        "VGA get colour",
        assert_eq!(
            unsafe { *WRITER_COL.lock() },
            ColourCode::new(Colour::LightGreen, Colour::Black)
        )
    )
}
#[test_case]
fn test_default_colour() {
    ColourCode::default();
    test!(
        "VGA default colour",
        assert_eq!(ColourCode::get(), ColourCode::new(FG_COL, BG_COL))
    )
}

pub mod prelude {
    pub use crate::eprintln;
    pub use crate::io::vga::{Colour, ColourCode};
    pub use crate::kprintln;
    pub use crate::print;
    pub use crate::println;
}
