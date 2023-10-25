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

use crate::cfg::interrupt::*;

lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
            VirtAddr::from_ptr(unsafe { &STACK }) + STACK_SIZE
        };
        tss
    };
    static ref GDT: (GlobalDescriptorTable, Selector) = {
        let mut gdt = GlobalDescriptorTable::new();
        let code = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss = gdt.add_entry(Descriptor::tss_segment(&TSS));
        (gdt, Selector { code, tss })
    };
}

struct Selector {
    code: SegmentSelector,
    tss: SegmentSelector,
}

pub fn init() {
    GDT.0.load();
    unsafe {
        CS::set_reg(GDT.1.code);
        load_tss(GDT.1.tss);
    }
}
