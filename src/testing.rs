use crate::{io::serial::prelude::*, util::hlt_loop};
use core::panic::PanicInfo;

#[macro_export]
macro_rules! test {
    ($name:expr, $expr:expr) => {{
        use $crate::io::serial::prelude::*;
        serial_print!("Testing `{}` ... ", $name);
        $expr;
        serial_println!("[OK]");
    }};
}

pub(crate) fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} test(s)", tests.len());
    for test in tests {
        test()
    }
    exit_qemu(QemuExitCode::Success);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub(crate) enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;
    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32)
    }
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[FAILED]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    unsafe {
        hlt_loop();
    }
}
