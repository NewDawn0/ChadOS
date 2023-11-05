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
//   File: src/mem/alloc/backend/bump/block.rs
//   Desc: Bump allocator block implementation

// RustDoc
//! # Bump Allocator Block Implementation
//!
//! This module contains the implementation of the bump allocator block, a part of the memory allocation system.
//! It includes structures and methods for allocating and deallocating fixed-size memory blocks.
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
//! # File: src/mem/alloc/backend/bump/block.rs
//!
//! This file contains the implementation of the bump allocator block in ChadOS.

// Imports
use crate::cfg::mem::BLOCK_SIZES;
use crate::mem::alloc::init::Locked;
use alloc::alloc::{GlobalAlloc, Layout};
use core::{
    mem,
    ptr::{self, NonNull},
};
use linked_list_allocator::Heap;

/// Determines the index of the block size based on the provided layout.
fn list_index(layout: &Layout) -> Option<usize> {
    let req_block_size = layout.size().max(layout.align());
    BLOCK_SIZES.iter().position(|&s| s >= req_block_size)
}

/// Represents a node in the linked list used by the bump allocator.
struct ListNode {
    next: Option<&'static mut ListNode>,
}

/// A fixed-size block allocator.
pub struct FixedSizeBlockAlloc {
    list_heads: [Option<&'static mut ListNode>; BLOCK_SIZES.len()],
    fallback_alloc: Heap,
}

impl FixedSizeBlockAlloc {
    /// Creates a new instance of the `FixedSizeBlockAlloc`.
    pub const fn new() -> Self {
        const EMPTY: Option<&'static mut ListNode> = None;
        Self {
            list_heads: [EMPTY; BLOCK_SIZES.len()],
            fallback_alloc: linked_list_allocator::Heap::empty(),
        }
    }

    /// Initializes the allocator with a memory block.
    pub unsafe fn init(&mut self, start: usize, size: usize) {
        self.fallback_alloc.init(start as *mut u8, size)
    }

    /// Falls back to a global allocator if the block allocation fails.
    fn fallback_alloc(&mut self, layout: Layout) -> *mut u8 {
        match self.fallback_alloc.allocate_first_fit(layout) {
            Ok(ptr) => ptr.as_ptr(),
            Err(_) => ptr::null_mut(),
        }
    }
}

/// Implements the `GlobalAlloc` trait for `FixedSizeBlockAlloc`.
unsafe impl GlobalAlloc for Locked<FixedSizeBlockAlloc> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut alloc = self.lock();
        match list_index(&layout) {
            Some(index) => match alloc.list_heads[index].take() {
                Some(node) => {
                    alloc.list_heads[index] = node.next.take();
                    node as *mut ListNode as *mut u8
                }
                None => {
                    let layout =
                        Layout::from_size_align(BLOCK_SIZES[index], BLOCK_SIZES[index]).unwrap();
                    alloc.fallback_alloc(layout)
                }
            },
            None => alloc.fallback_alloc(layout),
        }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let mut alloc = self.lock();
        match list_index(&layout) {
            Some(index) => {
                let new_node = ListNode {
                    next: alloc.list_heads[index].take(),
                };
                // Verify that block has size and alignment required for storing node
                assert!(mem::size_of::<ListNode>() <= BLOCK_SIZES[index]);
                assert!(mem::align_of::<ListNode>() <= BLOCK_SIZES[index]);
                let new_node_ptr = ptr as *mut ListNode;
                new_node_ptr.write(new_node);
                alloc.list_heads[index] = Some(&mut *new_node_ptr);
            }
            None => {
                let ptr = NonNull::new(ptr).unwrap();
                alloc.fallback_alloc.deallocate(ptr, layout);
            }
        }
    }
}
