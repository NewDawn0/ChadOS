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
//   File: src/interrupt/pic.rs
//   Desc: Setus up the Programmable Interrupt Controller (PIC)

// RustDoc
//! # Programmable Interrupt Controller (PIC) Module
//!
//! This module sets up the Programmable Interrupt Controller (PIC) for managing hardware interrupts.
//! It provides functions to initialize the PIC and enable interrupts.
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
//! # File: src/interrupt/pic.rs
//!
//! This file contains the implementation of the Programmable Interrupt Controller (PIC) in ChadOS.

// Imports
use crate::cfg::interrupt::{PIC_1_OFFSET, PIC_2_OFFSET};
use core::arch::asm;
use once_cell::sync::Lazy;
use pic8259::ChainedPics;

// Globals
/// Represents the Programmable Interrupt Controller (PIC) configuration.
pub static mut PICS: Lazy<ChainedPics> =
    Lazy::new(|| unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

/// Initializes the Programmable Interrupt Controller (PIC) and enables interrupts.
pub fn init() {
    unsafe {
        PICS.initialize();
        asm!("sti");
    };
}
