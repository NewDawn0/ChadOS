use crate::{
    cfg::interrupt::KEYBOARD_PORT,
    eprintln,
    interrupt::pic::{InterruptIndex, PICS},
    print, util,
};
use lazy_static::lazy_static;
use pc_keyboard::DecodedKey;
use pc_keyboard::{layouts, HandleControl, Keyboard, ScancodeSet1};
use spin::Mutex;
use x86_64::{
    instructions::port::Port,
    registers::control::Cr2,
    structures::idt::{InterruptStackFrame, PageFaultErrorCode},
};
pub extern "x86-interrupt" fn breakpoint(stack_frame: InterruptStackFrame) {
    eprintln!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}
pub extern "x86-interrupt" fn double_fault(stack_frame: InterruptStackFrame, error_code: u64) -> ! {
    panic!(
        "EXCEPTION: DOUBLE FAULT\n-> Error Code {:?}\n-> {:#?}",
        error_code, stack_frame
    );
}
pub extern "x86-interrupt" fn timer_interrupt(_stack_frame: InterruptStackFrame) {
    print!(".");
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}
pub extern "x86-interrupt" fn page_fault(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    eprintln!(
        "EXCEPTION: PAGE FAULT\n-> Accessed Addr: {:?}\n-> Error Code {:?}\n-> {:#?}",
        Cr2::read(),
        error_code,
        stack_frame
    );
    unsafe {
        util::hlt_loop();
    }
}
pub extern "x86-interrupt" fn keyboard_interrupt(_stack_frame: InterruptStackFrame) {
    lazy_static! {
        static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = Mutex::new(
            Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore)
        );
    }
    let mut keyboard = KEYBOARD.lock();
    let mut port = Port::new(KEYBOARD_PORT);
    let scancode: u8 = unsafe { port.read() };
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => print!("{}", character),
                DecodedKey::RawKey(key) => print!("{:?}", key),
            }
        }
    }
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}
