use crate::{cfg::interrupt::*, interrupt::handler, wprintln};
use lazy_static::lazy_static;
use spin::RwLock;
use x86_64::structures::idt::InterruptDescriptorTable;

fn default_irq_handler() {
    wprintln!("Unregistered handler called");
}
lazy_static! {
    pub static ref IRQ_HANDLERS: RwLock<[fn(); 16]> = RwLock::new([default_irq_handler; 16]);
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
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

pub fn init() {
    IDT.load()
}

#[inline]
pub fn irq_index(index: u8) -> u8 {
    PIC_1_OFFSET + index
}

#[test_case]
fn test_idt_breakpoint() {
    use crate::test;
    use core::arch::asm;
    test!("IDT breakpoint handler", unsafe { asm!("int3") })
}
