use crate::{
    cfg::interrupt::*,
    eprintln,
    interrupt::{
        idt::{irq_index, IRQ_HANDLERS},
        pic::PICS,
    },
    util,
};
use x86_64::{
    instructions::{interrupts, port::Port},
    registers::control::Cr2,
    structures::idt::{InterruptStackFrame as StackFrame, PageFaultErrorCode},
};
pub extern "x86-interrupt" fn breakpoint(stack_frame: StackFrame) {
    eprintln!("EXCEPTION: BREAKPOINT\n-> Stack Frame: {:#?}", stack_frame);
}
pub extern "x86-interrupt" fn double_fault(stack_frame: StackFrame, error_code: u64) -> ! {
    panic!(
        "EXCEPTION: DOUBLE FAULT\n-> Error Code {:?}\nStack Frame-> {:#?}",
        error_code, stack_frame
    );
}
pub extern "x86-interrupt" fn stack_segment_fault(stack_frame: StackFrame, error_code: u64) {
    panic!(
        "EXCEPTION: GENERAL PROTECTION FAULT\n-> Error Code {:?}\nStack Frame-> {:#?}",
        error_code, stack_frame
    );
}
pub extern "x86-interrupt" fn segment_not_present(stack_frame: StackFrame, error_code: u64) {
    panic!(
        "EXCEPTION: SEGMENT NOT FAULT\n-> Error Code {:?}\nStack Frame-> {:#?}",
        error_code, stack_frame
    );
}
pub extern "x86-interrupt" fn general_protection_fault(stack_frame: StackFrame, error_code: u64) {
    panic!(
        "EXCEPTION: STACK SEGMENT FAULT\n-> Error Code {:?}\nStack Frame-> {:#?}",
        error_code, stack_frame
    );
}
pub extern "x86-interrupt" fn page_fault(stack_frame: StackFrame, error_code: PageFaultErrorCode) {
    eprintln!(
        "EXCEPTION: PAGE FAULT\n-> Accessed Addr: {:?}\n-> Error Code {:?}\nStack Frame-> {:#?}",
        Cr2::read(),
        error_code,
        stack_frame
    );
    unsafe {
        util::hlt_loop();
    }
}

macro_rules! init_irq_handler {
    ($irq:literal, $name:ident) => {
        pub extern "x86-interrupt" fn $name(_stack_frame: StackFrame) {
            let handlers = IRQ_HANDLERS.read();
            handlers[$irq]();
            unsafe {
                PICS.notify_end_of_interrupt(irq_index($irq));
            }
        }
    };
}
init_irq_handler!(0, irq0_handler); // Timer
init_irq_handler!(1, irq1_handler); // Keyboard
init_irq_handler!(2, irq2_handler);
init_irq_handler!(3, irq3_handler);
init_irq_handler!(4, irq4_handler);
init_irq_handler!(5, irq5_handler);
init_irq_handler!(6, irq6_handler);
init_irq_handler!(7, irq7_handler);
init_irq_handler!(8, irq8_handler);
init_irq_handler!(9, irq9_handler);
init_irq_handler!(10, irq10_handler);
init_irq_handler!(11, irq11_handler);
init_irq_handler!(12, irq12_handler);
init_irq_handler!(13, irq13_handler);
init_irq_handler!(14, irq14_handler);
init_irq_handler!(15, irq15_handler);

pub fn set_irq_handler(irq: u8, handler: fn()) {
    interrupts::without_interrupts(|| {
        let mut handlers = IRQ_HANDLERS.write();
        handlers[irq as usize] = handler;
        clear_irq_mask(irq);
    });
}

pub fn clear_irq_mask(irq: u8) {
    let mut port: Port<u8> = Port::new(if irq < 8 { PIC_1_ADDR } else { PIC_2_ADDR });
    unsafe {
        let value = port.read() & !(1 << if irq < 8 { irq } else { irq - 8 });
        port.write(value);
    }
}
