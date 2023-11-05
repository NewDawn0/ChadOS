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
//   Desc: Bump allocator module file

// RustDoc
//! # Bump Allocator Module
//!
//! This module provides various implementations of the bump allocator in ChadOS, including the block-based, linked list-based, and global allocator implementations.
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
//! This file contains the bump allocator module, which provides various implementations of the bump allocator.

// Modules
pub mod block;
pub mod bump;
pub mod linked_list;
