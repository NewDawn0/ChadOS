/// Config file
#[cfg(test)]
use crate::test;

pub mod vga {
    use crate::io::vga::Colour;
    pub const BUFFER_HEIGHT: usize = 25;
    pub const BUFFER_WIDTH: usize = 80;
    pub const ASCII_BLANK: u8 = b' ';
    pub const BUF_ADDR: u32 = 0xb8000;
    pub const FG_COL: Colour = Colour::White;
    pub const BG_COL: Colour = Colour::Black;
    pub const WARN_COLOUR: Colour = Colour::Yellow;
    pub const ERRP_COLOUR: Colour = Colour::Red;
    pub const ERRM_COLOUR: Colour = Colour::LightRed;
    pub const KERNELP_COLOUR: Colour = Colour::Green;
    pub const KERNELM_COLOUR: Colour = Colour::LightGreen;
}
pub mod serial {
    pub const SERIAL1_PORT: u16 = 0x3f8;
}
pub mod interrupt {
    pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;
    pub const STACK_SIZE: usize = 1024 * 20; // 5*4KiB
    pub const PIC_1_OFFSET: u8 = 32;
    pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;
    pub const KEYBOARD_PORT: u16 = 0x60;
}
pub mod mem {
    pub const HEAP_START: usize = 0x444444440000;
    pub const HEAP_SIZE: usize = 100 * 1024; // 100 KiB
    #[cfg(feature = "alloc-bump")]
    pub const BLOCK_SIZES: &[usize] = &[8, 16, 32, 64, 128, 256, 512, 1024, 2048];
}
// Tests
#[test_case]
fn test_cfg() {
    use crate::io::vga::Colour;
    test!(
        "CFG VGA BUFFER_HEIGHT",
        assert_eq!(vga::BUFFER_HEIGHT, 25 as usize)
    );
    test!(
        "CFG VGA BUFFER_WIDTH",
        assert_eq!(vga::BUFFER_WIDTH, 80 as usize)
    );
    test!(
        "CFG VGA BUFFER_HEIGHT",
        assert_eq!(vga::ASCII_BLANK, b' ' as u8)
    );
    test!(
        "CFG VGA BUFFER_HEIGHT",
        assert_eq!(vga::BUF_ADDR, 0xb8000 as u32)
    );
    test!("CFG VGA FG_COL", assert_eq!(vga::FG_COL, Colour::White));
    test!("CFG VGA BG_COL", assert_eq!(vga::BG_COL, Colour::Black));
    test!(
        "CFG SERIAL SERIAL1_PORT",
        assert_eq!(serial::SERIAL1_PORT, 0x3f8 as u16)
    );
    test!(
        "CFG INTERRUPT DOUBLE_FAULT_IST_INDEX",
        assert_eq!(interrupt::DOUBLE_FAULT_IST_INDEX, 0 as u16)
    );
    test!(
        "CFG INTERRUPT STACK_SIZE",
        assert_eq!(interrupt::STACK_SIZE, 20480 as usize)
    );
    test!(
        "CFG INTERRUPT PIC_1_OFFSET",
        assert_eq!(interrupt::PIC_1_OFFSET, 32 as u8)
    );
    test!(
        "CFG INTERRUPT PIC_2_OFFSET",
        assert_eq!(interrupt::PIC_2_OFFSET, 40 as u8)
    );
    test!(
        "CFG INTERRUPT KEYBOARD_PORT",
        assert_eq!(interrupt::KEYBOARD_PORT, 0x60 as u16)
    );
}
