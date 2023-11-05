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
//   File: src/mem/alloc/backend/galloc
//   Desc: Good memory allocator global allocator implementation

// RustDoc
//! # Good Memory Allocator Module
//!
//! This module provides the implementation of the Good Memory Allocator as the global allocator in ChadOS.
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
//! # File: src/mem/alloc/backend/galloc.rs
//!
//! This file contains the global allocator implementation for the Good Memory Allocator.

// Imports
use core::alloc::GlobalAlloc;
use good_memory_allocator::SpinLockedAllocator;

// Globals
/// The global allocator for ChadOS, which uses the Good Memory Allocator.
#[global_allocator]
pub static ALLOC: LockInterfaceSpinLockedAllocator = LockInterfaceSpinLockedAllocator::new();

/// A wrapper structure providing the lock function for the Good Memory Allocator.
pub struct LockInterfaceSpinLockedAllocator {
    inner: SpinLockedAllocator,
}

impl LockInterfaceSpinLockedAllocator {
    /// Creates a new `LockInterfaceSpinLockedAllocator` instance.
    pub const fn new() -> Self {
        Self {
            inner: SpinLockedAllocator::empty(),
        }
    }

    /// Returns a reference to the inner `SpinLockedAllocator`.
    ///
    /// This function does nothing except provide the `.lock` function for itself.
    pub const fn lock(&self) -> &SpinLockedAllocator {
        &self.inner
    }
}

unsafe impl GlobalAlloc for LockInterfaceSpinLockedAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        self.inner.alloc(layout)
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        self.inner.dealloc(ptr, layout)
    }
}
