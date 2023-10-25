use crate::interrupt::handler;
use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptDescriptorTable;

use crate::{cfg::interrupt::DOUBLE_FAULT_IST_INDEX, interrupt::pic::InterruptIndex};

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(handler::breakpoint);
        idt.page_fault.set_handler_fn(handler::page_fault);
        unsafe {
            idt.double_fault
                .set_handler_fn(handler::double_fault)
                .set_stack_index(DOUBLE_FAULT_IST_INDEX);
        }
        idt[InterruptIndex::Timer.as_usize()].set_handler_fn(handler::timer_interrupt);
        idt[InterruptIndex::Keyboard.as_usize()].set_handler_fn(handler::keyboard_interrupt);
        idt
    };
}

pub fn init() {
    IDT.load()
}

#[test_case]
fn test_idt_breakpoint() {
    use crate::test;
    use core::arch::asm;
    test!("IDT breakpoint handler", unsafe { asm!("int3") })
}
