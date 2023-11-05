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
//   File: src/interrupt/idt.rs
//   Desc: Sets up the Interrupt descriptor table (IDT)

// RustDoc
//! # Interrupt Descriptor Table (IDT) Setup
//!
//! This module sets up the Interrupt Descriptor Table (IDT) for ChadOS, which handles interrupt
//! handling and management for the operating system.
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
//! # File: src/interrupt/idt.rs
//!
//! This file defines the setup for the IDT in ChadOS.

// Imports
use crate::{cfg::interrupt::*, interrupt::handler, wprintln};
use lazy_static::lazy_static;
use spin::RwLock;
use x86_64::structures::idt::InterruptDescriptorTable;

// Globals
lazy_static! {
    /// Array of IRQ handlers. This array stores functions that handle specific IRQ interrupts.
    pub static ref IRQ_HANDLERS: RwLock<[fn(); 16]> = RwLock::new([default_irq_handler; 16]);

    /// The Interrupt Descriptor Table (IDT) for ChadOS.
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        // Set up specific interrupt handlers.
        idt.breakpoint.set_handler_fn(handler::breakpoint);
        idt.stack_segment_fault
            .set_handler_fn(handler::stack_segment_fault);
        idt.segment_not_present
            .set_handler_fn(handler::segment_not_present);
        unsafe {
            idt.double_fault
                .set_handler_fn(handler::double_fault)
                .set_stack_index(DOUBLE_FAULT_IST_INDEX);
            idt.page_fault
                .set_handler_fn(handler::page_fault)
                .set_stack_index(PAGE_FAULT_IST_INDEX);
            idt.general_protection_fault
                .set_handler_fn(handler::general_protection_fault)
                .set_stack_index(GENERAL_PROTECTION_FAULT_IST_INDEX);
        }
        // Set up IRQ interrupt handlers.
        idt[irq_index(0) as usize].set_handler_fn(handler::irq0_handler);
        idt[irq_index(1) as usize].set_handler_fn(handler::irq1_handler);
        idt[irq_index(2) as usize].set_handler_fn(handler::irq2_handler);
        idt[irq_index(3) as usize].set_handler_fn(handler::irq3_handler);
        idt[irq_index(4) as usize].set_handler_fn(handler::irq4_handler);
        idt[irq_index(5) as usize].set_handler_fn(handler::irq5_handler);
        idt[irq_index(6) as usize].set_handler_fn(handler::irq6_handler);
        idt[irq_index(7) as usize].set_handler_fn(handler::irq7_handler);
        idt[irq_index(8) as usize].set_handler_fn(handler::irq8_handler);
        idt[irq_index(9) as usize].set_handler_fn(handler::irq9_handler);
        idt[irq_index(10) as usize].set_handler_fn(handler::irq10_handler);
        idt[irq_index(11) as usize].set_handler_fn(handler::irq11_handler);
        idt[irq_index(12) as usize].set_handler_fn(handler::irq12_handler);
        idt[irq_index(13) as usize].set_handler_fn(handler::irq13_handler);
        idt[irq_index(14) as usize].set_handler_fn(handler::irq14_handler);
        idt[irq_index(15) as usize].set_handler_fn(handler::irq15_handler);
        idt
    };
}

/// The default IRQ handler, called when an unregistered IRQ is triggered.
fn default_irq_handler() {
    wprintln!("Unregistered handler called");
}

/// Initializes and loads the IDT, setting up interrupt handlers and their stack indices.
pub fn init() {
    IDT.load()
}

/// Converts an IRQ index to the corresponding interrupt index.
#[inline]
pub fn irq_index(index: u8) -> u8 {
    PIC_1_OFFSET + index
}

// Tests
#[test_case]
fn test_idt_breakpoint() {
    use crate::test;
    use core::arch::asm;
    test!("IDT breakpoint handler", unsafe { asm!("int3") })
}
