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
//   File: src/mem/mod.rs
//   Desc: Memory module file

// RustDoc
//! # ChadOS Memory Module
//!
//! This module provides memory management functionality for ChadOS, including memory allocation
//! and paging submodules. It serves as a central point for memory-related operations in the OS.
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
//! # File: src/mem/mod.rs
//!
//! This file serves as the entry point for the memory module. It contains submodules for memory allocation
//! and paging operations.

// Modules
pub mod alloc;
pub mod paging;
