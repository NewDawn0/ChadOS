/// Config file
#[cfg(test)]
use crate::test;
pub mod util {
    // pub const ACPI_PORT: u16 = 0;
}
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
    pub const PAGE_FAULT_IST_INDEX: u16 = 1;
    pub const GENERAL_PROTECTION_FAULT_IST_INDEX: u16 = 2;
    pub const STACK_SIZE: usize = 1024 * 20; // 5*4KiB
    pub const PIC_1_OFFSET: u8 = 32;
    pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;
    pub const PIC_1_ADDR: u16 = 0x21;
    pub const PIC_2_ADDR: u16 = 0xA1;
    pub const KEYBOARD_PORT: u16 = 0x60;
    pub const PIT_CMD_PORT: u16 = 0x43;
    pub const PIT_ADDR_PORT: u16 = 0x40;
    pub const PIT_HZ: u32 = 100;
}
pub mod mem {
    pub const HEAP_START: usize = 0x444444440000;
    pub const HEAP_SIZE: usize = 100 * 1024; // 100 KiB
    #[cfg(feature = "alloc-bump")]
    pub const BLOCK_SIZES: &[usize] = &[8, 16, 32, 64, 128, 256, 512, 1024, 2048];
}
pub mod time {
    pub const EPOCH: u16 = 2000;
    pub const CMOS_ADDR_PORT: u16 = 0x70;
    pub const CMOS_DATA_PORT: u16 = 0x71;
    pub const PIT_DIVISOR: f64 = 1193.0 * 1.428751429;
    pub const PIT_FREQUENCY: f64 = 3579545.0 / 3.0; // See: https://wiki.osdev.org/PIT
    pub const PIT_INTERVAL: f64 = PIT_DIVISOR / PIT_FREQUENCY; // HACK:constant 1.428751429 fixes the time
    pub const PIT_FREQ_CMD_PORT: u16 = 0x40;
    pub const PIT_FREQ_DATA_PORT: u16 = 0x43;
}
pub mod keys {
    use crate::keys::WrappedLayout;
    // OPTIONS
    //   Name      : Description
    //   - De      : De 105 Key
    //   - Us      : Us 104 Key (default)
    //   - Uk      : Uk 105 Key
    //   - Jis     : Jis 109 Key
    //   - Dvp     : Dvp 104 Key
    //   - Azerty  : Azerty
    //   - Colemak : Colemak
    //   - Dvorak  : Dvorak 104 Key
    pub const LAYOUT: WrappedLayout = WrappedLayout::Us;
}
pub mod fs {
    pub const BLOCK_SIZE: usize = 512;
}
// Tests
#[test_case]
fn test_cfg() {
    test!(
        "CFG VGA BUFFER_HEIGHT",
        assert_eq!(vga::BUFFER_HEIGHT, 25 as usize)
    );
    test!(
        "CFG VGA BUFFER_WIDTH",
        assert_eq!(vga::BUFFER_WIDTH, 80 as usize)
    );
    test!(
        "CFG VGA ASCII_BLANK",
        assert_eq!(vga::ASCII_BLANK, b' ' as u8)
    );
    test!(
        "CFG VGA BUF_ADDR",
        assert_eq!(vga::BUF_ADDR, 0xb8000 as u32)
    );
    test!(
        "CFG SERIAL SERIAL1_PORT",
        assert_eq!(serial::SERIAL1_PORT, 0x3f8 as u16)
    );
    test!(
        "CFG INTERRUPT DOUBLE_FAULT_IST_INDEX",
        assert_eq!(interrupt::DOUBLE_FAULT_IST_INDEX, 0 as u16)
    );
    test!(
        "CFG INTERRUPT PAGE_FAULT_IST_INDEX",
        assert_eq!(interrupt::PAGE_FAULT_IST_INDEX, 1 as u16)
    );
    test!(
        "CFG INTERRUPT GENERAL_PROTECTION_FAULT_IST_INDEX",
        assert_eq!(interrupt::GENERAL_PROTECTION_FAULT_IST_INDEX, 2 as u16)
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
        "CFG INTERRUPT PIC_1_ADDR",
        assert_eq!(interrupt::PIC_1_ADDR, 0x21 as u16)
    );
    test!(
        "CFG INTERRUPT PIC_2_ADDR",
        assert_eq!(interrupt::PIC_2_ADDR, 0xA1 as u16)
    );
    test!(
        "CFG INTERRUPT KEYBOARD_PORT",
        assert_eq!(interrupt::KEYBOARD_PORT, 0x60 as u16)
    );
    test!(
        "CFG MEM HEAP_START",
        assert_eq!(mem::HEAP_START, 0x444444440000 as usize)
    );
    test!(
        "CFG MEM HEAP_SIZE",
        assert_eq!(mem::HEAP_SIZE, 102400 as usize)
    );
    test!("CFG TIME EPOCH", assert_eq!(time::EPOCH, 2000));
    test!(
        "CFG TIME CMOS_ADDR_PORT",
        assert_eq!(time::CMOS_ADDR_PORT, 0x70 as u16)
    );
    test!(
        "CFG TIME CMOS_DATA_PORT",
        assert_eq!(time::CMOS_DATA_PORT, 0x71 as u16)
    );
    test!(
        "CFG TIME PIT_FREQ_CMD_PORT",
        assert_eq!(time::PIT_FREQ_CMD_PORT, 0x40 as u16)
    );
    test!(
        "CFG TIME PIT_FREQ_DATA_PORT",
        assert_eq!(time::PIT_FREQ_DATA_PORT, 0x43 as u16)
    );
}
