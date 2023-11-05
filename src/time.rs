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
//   File: src/time.rs
//   Desc: Uptime & sleep implementations

// RustDoc
//! # ChadOS Time Module
//!
//! This module provides time-related functionality for ChadOS, an operating system implemented in Rust.
//! It includes uptime tracking, a timer handler, sleep function, and related utility functions.
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
//! # File: src/time.rs
//!
//! This file contains the implementation of uptime tracking, a timer handler, a sleep function, and related utilities.

// @NOTE:Rust implementation from: http://www.osdever.net/tutorials/view/brans-kernel-development-tutorial

// Imports
#[cfg(test)]
use crate::test;
use crate::{
    cfg::time::{PIT_ADDR_PORT, PIT_CMD_PORT, PIT_HZ},
    interrupt::handler::set_irq_handler,
};
use alloc::{format, string::String};
use core::sync::atomic::{AtomicUsize, Ordering};
use x86_64::instructions::{interrupts, port::PortWriteOnly};

// Globals
static UPTIME_TICKS: AtomicUsize = AtomicUsize::new(0);
static UPTIME_SECS: AtomicUsize = AtomicUsize::new(0);

/// Handles timer interrupts and increments the uptime counters.
fn timer_handler() {
    UPTIME_TICKS.fetch_add(1, Ordering::Relaxed);
    if UPTIME_TICKS.load(Ordering::Relaxed) % PIT_HZ as usize == 0 {
        UPTIME_SECS.fetch_add(1, Ordering::Relaxed);
    }
}

/// Initializes the timer and sets the timer interrupt handler.
pub fn init() {
    let div = 1193180 / PIT_HZ;
    let mut cmd_port: PortWriteOnly<u8> = PortWriteOnly::new(PIT_CMD_PORT);
    let mut data_port: PortWriteOnly<u8> = PortWriteOnly::new(PIT_ADDR_PORT);
    unsafe {
        cmd_port.write(0x36);
        data_port.write((div & 0xFF) as u8);
        data_port.write((div >> 8) as u8);
    }
    set_irq_handler(0, timer_handler)
}

/// Represents uptime information.
pub struct Uptime;

impl Uptime {
    /// Returns the uptime in seconds.
    pub fn secs() -> usize {
        UPTIME_SECS.load(Ordering::Relaxed)
    }

    /// Returns the uptime in timer ticks.
    pub fn ticks() -> usize {
        UPTIME_TICKS.load(Ordering::Relaxed)
    }

    /// Formats the uptime into a readable representation.
    pub fn fmt() -> UptimeRepr {
        let secs = UPTIME_SECS.load(Ordering::Relaxed);
        let years = secs / (365 * 24 * 60 * 60);
        let rem_secs = secs % (365 * 24 * 60 * 60);

        let days = rem_secs / (24 * 60 * 60);
        let rem_secs = rem_secs % (24 * 60 * 60);

        let hours = rem_secs / (60 * 60);
        let rem_secs = rem_secs % (60 * 60);

        let mins = rem_secs / 60;
        let secs = rem_secs % 60;
        UptimeRepr {
            secs,
            mins,
            hours,
            days,
            years,
        }
    }

    /// Formats the uptime as a string.
    pub fn string_fmt() -> String {
        let uptime = Self::fmt();
        let mut fmt = String::new();
        // INFO: Represents the biggest value meaning you don't have missing values `1h 1s` instead `1h 0m 1s`
        // Values:
        //  - 0 -> Unset
        //  - 5 -> years
        //  - 4 -> days
        //  - 3 -> hours
        //  - 3 -> mins
        //  - 2 -> secs
        let mut biggest: u8 = 0;
        if uptime.years > 0 {
            biggest = 5;
            fmt.push_str(&format!("{}y ", uptime.years))
        }
        if uptime.days > 0 || biggest > 4 {
            biggest = 4;
            fmt.push_str(&format!("{}d ", uptime.days))
        }
        if uptime.hours > 0 || biggest > 3 {
            biggest = 3;
            fmt.push_str(&format!("{}h ", uptime.hours))
        }
        if uptime.mins > 0 || biggest > 2 {
            fmt.push_str(&format!("{}m ", uptime.mins))
        }
        fmt.push_str(&format!("{}s", uptime.secs));
        fmt
    }
}

/// Represents the components of uptime.
#[derive(Debug)]
pub struct UptimeRepr {
    pub secs: usize,
    pub mins: usize,
    pub hours: usize,
    pub days: usize,
    pub years: usize,
}

/// Sleeps for the specified number of seconds.
///
/// # Parameters
///
/// - `secs`: The number of seconds to sleep.
pub fn sleep(secs: usize) {
    let start = UPTIME_TICKS.load(Ordering::Relaxed);
    while ((UPTIME_TICKS.load(Ordering::Relaxed) - start) / PIT_HZ as usize) < secs {
        hlt();
    }
}

/// Makes the CPU wait until the next timer interrupt (HLT instruction).
fn hlt() {
    let disabled = !interrupts::are_enabled();
    interrupts::enable_and_hlt();
    if disabled {
        interrupts::disable();
    }
}

// Tests
#[test_case]
fn tets_timer_handler() {
    UPTIME_TICKS.store(0, Ordering::Relaxed);
    UPTIME_SECS.store(0, Ordering::Relaxed);
    timer_handler();
    test!(
        "TIME timer_handler() ticks",
        assert_eq!(UPTIME_TICKS.load(Ordering::Relaxed), 1)
    );
    UPTIME_TICKS.store(0, Ordering::Relaxed);
    UPTIME_SECS.store(0, Ordering::Relaxed);
    timer_handler();
    test!(
        "TIME timer_handler() secs",
        assert_eq!(UPTIME_SECS.load(Ordering::Relaxed), 0)
    );
    for _ in 1..PIT_HZ {
        timer_handler();
    }
}

#[test_case]
fn test_sleep_function() {
    UPTIME_TICKS.store(0, Ordering::Relaxed);
    let secs = 2;
    let start_ticks = UPTIME_TICKS.load(Ordering::Relaxed);
    sleep(2);
    let elapsed = UPTIME_TICKS.load(Ordering::Relaxed) - start_ticks;
    test!("TIME sleep()", assert_eq!(elapsed / PIT_HZ as usize, secs))
}
