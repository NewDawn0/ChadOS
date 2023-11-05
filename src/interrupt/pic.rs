use crate::cfg::interrupt::{PIC_1_OFFSET, PIC_2_OFFSET};
use core::arch::asm;
use once_cell::sync::Lazy;
use pic8259::ChainedPics;

pub static mut PICS: Lazy<ChainedPics> =
    Lazy::new(|| unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

pub fn init() {
    unsafe {
        PICS.initialize();
        asm!("sti");
    };
}
