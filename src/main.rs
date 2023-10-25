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
mod mem;
#[cfg(test)]
mod testing;
mod util;

// Imports
extern crate alloc;
use bootloader::{entry_point, BootInfo};
#[cfg(not(test))]
use core::panic::PanicInfo;

// Bootloader entrypoint
entry_point!(kmain);
fn kmain(boot_info: &'static BootInfo) -> ! {
    // util::logo();
    util::init(boot_info);

    #[cfg(test)]
    test_main();

    use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};
    let heap_value = Box::new(41);
    println!("heap_value at {:p}", heap_value);

    let mut vec = Vec::new();
    for i in 0..1000 {
        vec.push(i);
    }
    println!("Vec at {:p}", vec.as_slice());

    let rc = Rc::new(vec![1, 2, 3]);
    let cr = rc.clone();
    println!("Current ref count is {}", Rc::strong_count(&cr));
    core::mem::drop(rc);
    println!("Current ref count is {}", Rc::strong_count(&cr));

    println!("Hello world{}", '!');
    unsafe {
        util::hlt_loop();
    }
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
