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
//   File: src/mem/alloc/init.rs
//   Desc: Initalizes the allocator

// RustDoc
//! # Allocator Initialization
//!
//! This module contains functions and structures related to initializing the memory allocator
//! for ChadOS. The allocator can be configured to use either the Bump Allocator or the Good Memory
//! Allocator based on feature flags.
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
//! # File: src/mem/alloc/init.rs
//!
//! This file contains functions and structures for initializing the memory allocator based on
//! feature flags.

// Imports
use crate::cfg::mem::{HEAP_SIZE, HEAP_START};
#[cfg(test)]
use crate::test;
#[cfg(feature = "alloc-bump")]
use spin::{Mutex, MutexGuard};
use x86_64::{
    structures::paging::{
        mapper::MapToError, FrameAllocator as FrameAlloc, Mapper, Page, PageTableFlags as Flags,
        Size4KiB,
    },
    VirtAddr,
};

// Set allocator
#[cfg(feature = "alloc-bump")]
use crate::mem::alloc::backend::bump::bump::ALLOC;
#[cfg(feature = "alloc-galloc")]
use crate::mem::alloc::backend::galloc::ALLOC;

#[cfg(feature = "alloc-bump")]
pub struct Locked<T> {
    inner: Mutex<T>,
}
#[cfg(feature = "alloc-bump")]
impl<T> Locked<T> {
    pub const fn new(inner: T) -> Self {
        Self {
            inner: Mutex::new(inner),
        }
    }
    pub fn lock(&self) -> MutexGuard<T> {
        self.inner.lock()
    }
}

#[cfg(feature = "alloc-bump")]
pub fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}

/// Initializes the memory allocator.
///
/// This function sets up the memory allocator's heap, maps the heap pages, and initializes
/// the selected allocator (Bump Allocator or Good Memory Allocator) based on the feature flags.
///
/// # Arguments
///
/// - `mapper`: A mutable reference to the memory mapper for mapping heap pages.
/// - `frame_alloc`: A mutable reference to the frame allocator for allocating physical frames.
///
/// # Returns
///
/// Returns `Ok(())` if the initialization is successful, or an error of type `MapToError` if there's an issue
/// mapping the heap pages.
pub fn init(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_alloc: &mut impl FrameAlloc<Size4KiB>,
) -> Result<(), MapToError<Size4KiB>> {
    let page_range = {
        let start = VirtAddr::new(HEAP_START as u64);
        let end = start + HEAP_SIZE - 1u64;
        let start_page = Page::containing_address(start);
        let end_page = Page::containing_address(end);
        Page::range_inclusive(start_page, end_page)
    };
    for page in page_range {
        let frame = frame_alloc
            .allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?;
        let flags = Flags::PRESENT | Flags::WRITABLE;
        unsafe { mapper.map_to(page, frame, flags, frame_alloc)?.flush() };
    }
    unsafe {
        ALLOC.lock().init(HEAP_START, HEAP_SIZE);
    }
    Ok(())
}

// Tests
#[test_case]
fn test_alloc() {
    use alloc::{boxed::Box, rc::Rc, vec::Vec};
    let r#box = Box::new(911);
    test!("MEM ALLOC INIT box heap malloc", assert_eq!(*r#box, 911));
    let mut vec = Vec::new();
    for i in 0..1000 {
        vec.push(i);
    }
    test!(
        "MEM ALLOC INIT large vec heap malloc",
        assert_eq!(vec.iter().sum::<u64>(), (1000 - 1) * 1000 / 2)
    );
    let rc1 = Rc::new(5);
    let rc2 = Rc::clone(&rc1);
    test!(
        "MEM ALLOC INIT large rc ref pt1 count heap malloc",
        assert_eq!(Rc::strong_count(&rc1), 2)
    );
    core::mem::drop(rc2);
    test!(
        "MEM ALLOC INIT large rc ref pt2 count heap malloc",
        assert_eq!(Rc::strong_count(&rc1), 1)
    );
}
