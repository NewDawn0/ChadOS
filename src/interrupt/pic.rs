use crate::cfg::interrupt::{PIC_1_OFFSET, PIC_2_OFFSET};
use core::arch::asm;
use pic8259::ChainedPics;
use spin::Mutex;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
}
impl InterruptIndex {
    pub fn as_u8(self) -> u8 {
        self as u8
    }
    pub fn as_usize(self) -> usize {
        self.as_u8() as usize
    }
}

pub static PICS: Mutex<ChainedPics> =
    Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

pub fn init() {
    unsafe {
        PICS.lock().initialize();
        asm!("sti");
    };
}
