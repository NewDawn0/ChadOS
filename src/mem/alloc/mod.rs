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
//   File: src/mem/alloc/mod.rs
//   Desc: Heap allocation module file

// RustDoc
//! # ChadOS Memory Allocation Module
//!
//! This module provides the memory allocation functionality for ChadOS. It includes submodules for
//! configuring different memory allocation backends, such as the Bump Allocator or the Good Memory Allocator.
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
//! # File: src/mem/alloc/mod.rs
//!
//! This file serves as the entry point for the memory allocation module. It contains submodules
//! for configuring and initializing memory allocation backends.

// Modules
pub mod backend;
pub mod init;
