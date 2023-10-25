// Config macros
#![no_std]
#![no_main]
#![allow(non_snake_case)]
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::testing::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(const_mut_refs)]

// Modules
mod cfg;
mod interrupt;
mod io;
mod keys;
mod mem;
mod sched;
#[cfg(test)]
mod testing;
mod util;

// Imports
extern crate alloc;
use bootloader::{entry_point, BootInfo};
#[cfg(not(test))]
use core::panic::PanicInfo;
use keys::scancode;
use sched::{Exec, Task};

// Bootloader entrypoint
entry_point!(kmain);
fn kmain(boot_info: &'static BootInfo) -> ! {
    // util::logo();
    util::init(boot_info);

    #[cfg(test)]
    test_main();

    let mut exec = Exec::new();
    exec.spawn(Task::new(scancode::print_keys()));
    exec.run();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    unsafe {
        util::hlt_loop();
    }
}
