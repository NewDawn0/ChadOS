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
//   File: src/interrupt/gdt.rs
//   Desc: Sets up the Global descriptor table (GDT)

// RustDoc
//! # Global Descriptor Table (GDT) Setup
//!
//! This module sets up the Global Descriptor Table (GDT) for ChadOS, which is used for managing memory
//! segments and defining the privilege levels for the CPU.
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
//! # File: src/interrupt/gdt.rs
//!
//! This file defines the setup for the GDT in ChadOS.

// Imports
use crate::cfg::interrupt::*;
use lazy_static::lazy_static;
use x86_64::{
    instructions::{segmentation::CS, tables::load_tss},
    registers::segmentation::Segment,
    structures::{
        gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector},
        tss::TaskStateSegment,
    },
    VirtAddr,
};

// Globals
lazy_static! {
    /// The Task State Segment (TSS) for ChadOS.
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        // Set up the double fault stack.
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            // Define the double fault stack with a size of STACK_SIZE.
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
            VirtAddr::from_ptr(unsafe { &STACK }) + STACK_SIZE
        };
        tss
    };
    /// The Global Descriptor Table (GDT) and corresponding selectors for ChadOS.
    static ref GDT: (GlobalDescriptorTable, Selector) = {
        let mut gdt = GlobalDescriptorTable::new();
        // Add the kernel code segment to the GDT.
        let code = gdt.add_entry(Descriptor::kernel_code_segment());
        // Add the Task State Segment (TSS) to the GDT.
        let tss = gdt.add_entry(Descriptor::tss_segment(&TSS));
        (gdt, Selector { code, tss })
    };
}
/// Represents the GDT selectors for ChadOS.
struct Selector {
    code: SegmentSelector,
    tss: SegmentSelector,
}

/// Initializes and loads the GDT, including the kernel code segment and the Task State Segment (TSS).
pub fn init() {
    // Load the GDT and set the code segment and TSS.
    GDT.0.load();
    unsafe {
        CS::set_reg(GDT.1.code);
        load_tss(GDT.1.tss);
    }
}
