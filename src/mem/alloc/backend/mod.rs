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
//   File: src/mem/alloc/backend/mod.rs
//   Desc: Allocator backend module file

// RustDoc
//! # Allocator Backend Module
//!
//! This module contains the backend implementations of memory allocators used in ChadOS. Depending on
//! the feature flags enabled during the build, either the Bump Allocator or the Good Memory Allocator
//! is included as the global allocator.
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
//! # File: src/mem/alloc/backend/mod.rs
//!
//! This file contains the module that includes the various allocator backend implementations.
//!

#[cfg(feature = "alloc-bump")]
/// ## Modules
/// - [`bump`]: The Bump Allocator backend module.

#[cfg(feature = "alloc-galloc")]
/// ## Modules
/// - [`galloc`]: The Good Memory Allocator backend module.

// Modules
#[cfg(feature = "alloc-bump")]
pub mod bump;
#[cfg(feature = "alloc-galloc")]
pub mod galloc;
