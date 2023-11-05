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
//   File: src/interrupt/mod.rs
//   Desc: Interrupt module file

// RustDoc
//! # Interrupt Module
//!
//! This module provides functionality for managing interrupts in ChadOS, including setting up the
//! Global Descriptor Table (GDT), the Interrupt Descriptor Table (IDT), and the Programmable
//! Interrupt Controller (PIC). It also contains interrupt handlers.
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
//! # File: src/interrupt/mod.rs
//!
//! This file is the entry point to the interrupt module in ChadOS.

// Modules
pub mod gdt;
pub mod handler;
pub mod idt;
pub mod pic;
