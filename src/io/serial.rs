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
//   File: src/io/serial.rs
//   Desc: Serial port printing implementation

// RustDoc
//! # ChadOS Serial Port Module
//!
//! This module provides an implementation for printing to a serial port. It is commonly used for debugging
//! purposes and logging messages.
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
//! # File: src/io/serial.rs
//!
//! This file defines the serial port printing implementation for ChadOS.

// Imports
use crate::cfg::serial::SERIAL1_PORT;
use core::fmt::Arguments;
use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::SerialPort;

// Macros

/// Prints a formatted string to the serial port with a newline at the end.
///
/// This macro is similar to `println!`, but it outputs to the serial port.
///
/// # Examples
///
/// ```
/// # use crate::serial_println;
/// serial_println!("This is a serial port message");
/// ```
#[macro_export]
macro_rules! serial_println {
    () => (serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(
        concat!($fmt, "\n"), $($arg)*));
}

/// Prints a formatted string to the serial port without a newline at the end.
///
/// This macro is similar to `print!`, but it outputs to the serial port.
///
/// # Examples
///
/// ```
/// # use crate::serial_print;
/// serial_print!("This is a serial port message");
/// ```
#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => ($crate::io::serial::_print(format_args!($($arg)*)));
}

// Globals
lazy_static! {
    /// The global static serial port instance for writing to the serial port.
    pub static ref SERIAL1: Mutex<SerialPort> = {
        let mut port = unsafe { SerialPort::new(SERIAL1_PORT) };
        port.init();
        Mutex::new(port)
    };
}

#[doc(hidden)]
pub fn _print(args: Arguments) {
    use core::fmt::Write;
    SERIAL1
        .lock()
        .write_fmt(args)
        .expect("Printing to serial failed");
}

/// A prelude for working with the serial module.
///
/// This module provides easy access to the serial printing macros.
#[allow(unused)]
pub(crate) mod prelude {
    pub use crate::{serial_print, serial_println};
}
