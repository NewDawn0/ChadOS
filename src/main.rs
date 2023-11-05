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
//
//   MIT License
//   Copyright (c) 2023 NewDawn0
//
//   Permission is hereby granted, free of charge, to any person obtaining a copy
//   of this software and associated documentation files (the "Software"), to deal
//   in the Software without restriction, including without limitation the rights
//   to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//   copies of the Software, and to permit persons to whom the Software is
//   furnished to do so, subject to the following conditions:
//
//   The above copyright notice and this permission notice shall be included in all
//   copies or substantial portions of the Software.
//
//   THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//   IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//   FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//   AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//   LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//   OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
//   SOFTWARE.
//
//
//   File: src/main.rs
//   Desc: Main program flie

// RustDoc
//! # ChadOS
//!
//! ChadOS is an operating system implemented in Rust. It includes various modules for system configuration, console,
//! interrupts, I/O, keyboard handling, memory management, scheduling, time tracking, user binaries, and utility functions.
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
//! # File: src/main.rs
//!
//! This file is the entry point for the ChadOS operating system.

// Config macros
#![no_std]
#![no_main]
#![allow(non_snake_case)]
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::testing::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(const_mut_refs)]

// Modules
pub mod api;
pub mod cfg;
mod console;
mod interrupt;
mod io;
mod keys;
mod mem;
mod sched;
#[cfg(test)]
mod testing;
mod time;
mod usr_bin;
mod util;

// Imports
extern crate alloc;
use bootloader::{entry_point, BootInfo};
use cfg::console::CMD_OK_COL;
#[cfg(not(test))]
use core::panic::PanicInfo;
use io::vga::COL;
use sched::Exec;

// Bootloader entrypoint
entry_point!(kmain);

/// The main entry point for ChadOS.
///
/// This function is called on system startup.
fn kmain(boot_info: &'static BootInfo) -> ! {
    util::init(boot_info);
    COL.lock().set_fg(CMD_OK_COL);
    println!("CheapShell intialized");
    print!("> "); // print console init
    COL.lock().set_default();

    // Tests
    #[cfg(test)]
    test_main();
    // Start the async executor
    let mut exec = Exec::new();
    // exec.spawn(Task::new(scancode::print_keys()));
    exec.run();
}

/// This function is called on panic.
///
/// It handles panic by printing the panic information and entering an endless loop.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    unsafe {
        util::hlt_loop();
    }
}
