// Macros (crate level)
#![no_std]  // Compile as native binary without libc runtime
#![no_main] // Override main function

// Imports
use core::panic::PanicInfo;

// HACK: Hacky panic impl
// TODO: Add panic message and free stack variables 
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop { }
}

// Custom main function
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
