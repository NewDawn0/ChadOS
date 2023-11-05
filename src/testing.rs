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
//   File: src/testing.rs
//   Desc: Testing related functions & macros

//! # ChadOS Testing Module
//!
//! This module provides testing-related functions, macros, and utilities for ChadOS, an
//! operating system implemented in Rust. It includes the test runner, testing macros, and
//! functions to communicate with the QEMU emulator for test exits.
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
//! # File: src/testing.rs
//!
//! This file contains testing-related functions and macros.

// Imports
use crate::{io::serial::prelude::*, util::hlt_loop};
use core::panic::PanicInfo;

// Macros

/// A testing macro for running tests and reporting results.
///
/// # Parameters
///
/// - `$name`: A string describing the test.
/// - `$expr`: The expression to test.
#[macro_export]
macro_rules! test {
    ($name:expr, $expr:expr) => {{
        use $crate::io::serial::prelude::*;
        serial_print!("Testing `{}` ... ", $name);
        $expr;
        serial_println!("[OK]");
    }};
}

/// Runs a collection of tests and reports the results.
///
/// # Parameters
///
/// - `tests`: An array of test functions to run.
pub(crate) fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} test(s)", tests.len());
    for test in tests {
        test()
    }
    exit_qemu(QemuExitCode::Success);
}

/// Represents exit codes for the QEMU emulator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub(crate) enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

/// Exits the QEMU emulator with the specified exit code.
///
/// # Parameters
///
/// - `exit_code`: The exit code for the emulator.
pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;
    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32)
    }
}

// Tests
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[FAILED]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    unsafe {
        hlt_loop();
    }
}
