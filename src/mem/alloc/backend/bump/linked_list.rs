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
//   File: src/mem/alloc/backend/bump/linked_list.rs
//   Desc: Bump allocator linked_list implementation

// RustDoc
//! # Bump Allocator Linked List Implementation
//!
//! This module provides the linked list implementation of the bump allocator in ChadOS.
//! It includes the [`LinkedListAlloc`] structure and its implementation, serving as a global allocator for memory allocation.
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
//! # File: src/mem/alloc/backend/bump/linked_list.rs
//!
//! This file contains the linked list implementation of the bump allocator.

// Imports
use crate::mem::alloc::init::{align_up, Locked};
use alloc::alloc::{GlobalAlloc, Layout};
use core::{mem, ptr};

/// Represents a node in the linked list, used to manage memory blocks.
struct ListNode {
    size: usize,
    next: Option<&'static mut ListNode>,
}

impl ListNode {
    /// Creates a new `ListNode` with the given size.
    const fn new(size: usize) -> Self {
        Self { size, next: None }
    }

    /// Returns the start address of the node.
    fn start_addr(&self) -> usize {
        self as *const Self as usize
    }

    /// Returns the end address of the node.
    fn end_addr(&self) -> usize {
        self.start_addr() + self.size
    }
}

/// Represents a linked list-based allocator.
pub struct LinkedListAlloc {
    head: ListNode,
}

impl LinkedListAlloc {
    /// Creates a new `LinkedListAlloc` instance.
    pub const fn new() -> Self {
        Self {
            head: ListNode::new(0),
        }
    }
    /// Initializes the linked list allocator with a memory block.
    pub unsafe fn init(&mut self, start: usize, size: usize) {
        self.add_free_reg(start, size);
    }

    unsafe fn add_free_reg(&mut self, addr: usize, size: usize) {
        assert_eq!(align_up(addr, mem::align_of::<ListNode>()), addr);
        assert!(size >= mem::size_of::<ListNode>());
        let mut node = ListNode::new(size);
        node.next = self.head.next.take();
        let node_ptr = addr as *mut ListNode;
        node_ptr.write(node);
        self.head.next = Some(&mut *node_ptr)
    }
    fn find_reg(&mut self, size: usize, align: usize) -> Option<(&'static mut ListNode, usize)> {
        let mut curr = &mut self.head;
        while let Some(ref mut reg) = curr.next {
            match Self::alloc_from_reg(&reg, size, align) {
                Ok(start) => {
                    let next = reg.next.take();
                    let ret = Some((curr.next.take().unwrap(), start));
                    curr.next = next;
                    return ret;
                }
                Err(_) => curr = curr.next.as_mut().unwrap(),
            }
        }
        None
    }
    fn alloc_from_reg(reg: &ListNode, size: usize, align: usize) -> Result<usize, ()> {
        let start = align_up(reg.start_addr(), align);
        let end = start.checked_add(size).ok_or(())?;
        if end > reg.end_addr() {
            return Err(());
        }
        let excess = reg.end_addr() - end;
        if excess > 0 && excess < mem::size_of::<ListNode>() {
            return Err(());
        }
        Ok(start)
    }
    fn size_align(layout: Layout) -> (usize, usize) {
        let layout = layout
            .align_to(mem::align_of::<ListNode>())
            .expect("Adjustment of LinkedListAlloc alignment failed")
            .pad_to_align();
        let size = layout.size().max(mem::size_of::<ListNode>());
        (size, layout.align())
    }
}

/// Implements the `GlobalAlloc` trait for `LinkedListAlloc`.
unsafe impl GlobalAlloc for Locked<LinkedListAlloc> {
    /// Allocates memory with the specified layout.
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let (size, align) = LinkedListAlloc::size_align(layout);
        let mut alloc = self.lock();
        match alloc.find_reg(size, align) {
            Some((region, alloc_start)) => {
                let alloc_end = alloc_start.checked_add(size).expect("Buffer overflow");
                let excess_size = region.end_addr() - alloc_end;
                if excess_size > 0 {
                    alloc.add_free_reg(alloc_end, excess_size);
                }
                alloc_start as *mut u8
            }
            None => ptr::null_mut(),
        }
    }
    /// Deallocates the memory block associated with the given pointer and layout.
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let (size, _) = LinkedListAlloc::size_align(layout);
        self.lock().add_free_reg(ptr as usize, size)
    }
}
