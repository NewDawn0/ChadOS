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
//   File: src/mem/alloc/backend/bump/bump.rs
//   Desc: Bump allocator global allocator implementation

// RustDoc
//! # Bump Allocator Global Allocator Implementation
//!
//! This module provides the global allocator implementation for the bump allocator in ChadOS. It includes
//! the [`BumpAlloc`] structure and its implementation, which serves as a global allocator for memory allocation.
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
//! # File: src/mem/alloc/backend/bump/bump.rs
//!
//! This file contains the global allocator implementation for the bump allocator.

// Imports
use crate::mem::alloc::{
    backend::bump::block::FixedSizeBlockAlloc,
    init::{align_up, Locked},
};
use alloc::alloc::{GlobalAlloc, Layout};
use core::ptr;

// Globals
/// The global allocator instance that uses `BumpAlloc`.
/// It is tagged with `#[global_allocator]`, making it the global allocator used by the application.
#[global_allocator]
pub static ALLOC: Locked<FixedSizeBlockAlloc> = Locked::new(FixedSizeBlockAlloc::new());

/// Represents the bump allocator used as a global allocator.
pub struct BumpAlloc {
    start: usize,
    end: usize,
    next: usize,
    allocs: usize,
}
impl BumpAlloc {
    /// Creates a new instance of `BumpAlloc`.
    pub const fn new() -> Self {
        Self {
            start: 0,
            end: 0,
            next: 0,
            allocs: 0,
        }
    }

    /// Initializes the bump allocator with a memory block.
    pub unsafe fn init(&mut self, start: usize, size: usize) {
        self.start = start;
        self.end = start.saturating_add(size);
        self.next = start;
    }
}

/// Implements the `GlobalAlloc` trait for `BumpAlloc`.
unsafe impl GlobalAlloc for Locked<BumpAlloc> {
    /// Allocates memory with the specified layout.
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut bump = self.lock();
        let start = align_up(bump.next, layout.align());
        let end = match start.checked_add(layout.size()) {
            Some(end) => end,
            None => return ptr::null_mut(),
        };
        match end > bump.end {
            true => ptr::null_mut(),
            false => {
                bump.next = end;
                bump.allocs += 1;
                start as *mut u8
            }
        }
    }

    /// Deallocates the memory block associated with the given pointer and layout.
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let mut bump = self.lock();
        bump.allocs -= 1;
        if bump.allocs == 0 {
            bump.next = bump.start;
        }
    }
}
