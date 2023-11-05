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
//   File: src/cfg.rs
//   Desc: Global configuration

// RustDoc
//! # ChadOS Configuration
//!
//! This module defines the global configuration for ChadOS. It contains constants and configurations
//! that are used throughout the operating system.
//!
//! For more information about ChadOS, visit [the ChadOS GitHub repository](https://github.com/NewDawn0/ChadOS).
//!
//! ## Author
//!
//! - [NewDawn0](https://github.com/NewDawn0)
//!
//! ## License
//!
//! This code is licensed under the MIT License.
//!
//! # File: src/cfg.rs
//!
//! Global configuration settings for ChadOS.

// Imports
#[cfg(test)]
use crate::test;

pub mod vga {
    //! VGA Configuration
    //!
    //! This module contains constants related to VGA configuration in ChadOS.

    use crate::io::vga::Colour;

    /// The height of the VGA buffer.
    pub const BUFFER_HEIGHT: usize = 25;

    /// The width of the VGA buffer.
    pub const BUFFER_WIDTH: usize = 80;

    /// The ASCII value for the blank character.
    pub const ASCII_BLANK: u8 = b' ';

    /// The address of the VGA buffer (only change if you know what you are doing).
    pub const BUF_ADDR: u32 = 0xb8000;

    /// The default foreground print color.
    pub const FG_COL: Colour = Colour::White;

    /// The default background print color.
    pub const BG_COL: Colour = Colour::Black;

    /// The warning message color.
    pub const WARN_COLOUR: Colour = Colour::Yellow;

    /// The color of [ERROR] in the error message.
    pub const ERRP_COLOUR: Colour = Colour::Red;

    /// The color of the message in the error message.
    pub const ERRM_COLOUR: Colour = Colour::LightRed;

    /// The color of [KERNEL] in the kernel message.
    pub const KERNELP_COLOUR: Colour = Colour::LightBlue;

    /// The color of message in the kernel message.
    pub const KERNELM_COLOUR: Colour = Colour::Cyan;
}

pub mod serial {
    //! Serial Configuration
    //!
    //! This module contains constants related to serial communication configuration in ChadOS.

    /// The serial port address (only change if you know what you are doing).
    pub const SERIAL1_PORT: u16 = 0x3f8;
}

pub mod interrupt {
    //! Interrupt Configuration
    //!
    //! This module contains constants related to interrupt handling and configuration in ChadOS.

    /// The double fault IST index (only change if you know what you are doing).
    pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

    /// The page fault IST index (only change if you know what you are doing).
    pub const PAGE_FAULT_IST_INDEX: u16 = 1;

    /// The general protection fault IST index (only change if you know what you are doing).
    pub const GENERAL_PROTECTION_FAULT_IST_INDEX: u16 = 2;

    /// Size of the GDT interrupt stack.
    pub const STACK_SIZE: usize = 1024 * 20;

    /// PIC 1 offset (only change if you know what you are doing).
    pub const PIC_1_OFFSET: u8 = 32;

    /// PIC 2 offset (only change if you know what you are doing).
    pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

    /// PIC 1 address (only change if you know what you are doing).
    pub const PIC_1_ADDR: u16 = 0x21;

    /// PIC 2 address (only change if you know what you are doing).
    pub const PIC_2_ADDR: u16 = 0xA1;

    /// Keyboard port (only change if you know what you are doing).
    pub const KEYBOARD_PORT: u16 = 0x60;
}

pub mod mem {
    //! Memory Configuration
    //!
    //! This module contains constants related to memory management and configuration in ChadOS.

    /// The starting address of the heap.
    pub const HEAP_START: usize = 0x444444440000;

    /// The size of the heap in KiB.
    pub const HEAP_SIZE: usize = 100 * 1024;

    /// Block sizes for the heap allocator.
    #[cfg(feature = "alloc-bump")]
    pub const BLOCK_SIZES: &[usize] = &[8, 16, 32, 64, 128, 256, 512, 1024, 2048];
}

pub mod time {
    //! Time Configuration
    //!
    //! This module contains constants related to time and timer configuration in ChadOS.

    /// PIT command port (only change if you know what you are doing).
    pub const PIT_CMD_PORT: u16 = 0x43;

    /// PIT address port (only change if you know what you are doing).
    pub const PIT_ADDR_PORT: u16 = 0x40;

    /// PIT Hz frequency (how fast the ticks in the time tick) (only change if you know what you are doing).
    pub const PIT_HZ: u32 = 100;
}

pub mod keys {
    //! Key Layout Options
    //!
    //! This module provides options for configuring the key layout used in ChadOS

    /// The supported key layout options
    use crate::keys::WrappedLayout;
    /// The supported layout options
    ///   Name      : Description
    ///   - De      : De 105 Key
    ///   - Us      : Us 104 Key (default)
    ///   - Uk      : Uk 105 Key
    ///   - Jis     : Jis 109 Key
    ///   - Dvp     : Dvp 104 Key
    ///   - Azerty  : Azerty
    ///   - Colemak : Colemak
    ///   - Dvorak  : Dvorak 104 Key
    /// The selected key layout
    pub const LAYOUT: WrappedLayout = WrappedLayout::Us;
}

pub mod console {
    //! Console Configuration
    //!
    //! This module contains constants related to console and command line configuration in ChadOS.

    use crate::io::vga::Colour;

    /// Character that separates different commands like the Unix pipe '|'.
    pub const CMD_SEPERATOR: char = '!';

    /// Colour of the command line symbol `>` if the command was a success.
    pub const CMD_OK_COL: Colour = Colour::LightGreen;

    /// Colour of the command line symbol `>` if the command was a failure.
    pub const CMD_ERR_COL: Colour = Colour::LightRed;

    /// Colour of the command line output.
    pub const CMD_OUT_COL: Colour = Colour::Pink;
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
    test!(
        "CFG TIME PIT_CMD_PORT",
        assert_eq!(time::PIT_CMD_PORT, 0x43 as u16)
    );
    test!(
        "CFG TIME PIT_ADDR_PORT",
        assert_eq!(time::PIT_ADDR_PORT, 0x40 as u16)
    );
    test!("CFG TIME PIT_HZ", assert_eq!(time::PIT_HZ, 100 as u32));
}
