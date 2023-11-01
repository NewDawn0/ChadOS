use crate::mem::alloc::{
    backend::bump::block::FixedSizeBlockAlloc,
    init::{align_up, Locked},
};
use alloc::alloc::{GlobalAlloc, Layout};
use core::ptr;

#[global_allocator]
pub static ALLOC: Locked<FixedSizeBlockAlloc> = Locked::new(FixedSizeBlockAlloc::new());

pub struct BumpAlloc {
    start: usize,
    end: usize,
    next: usize,
    allocs: usize,
}
impl BumpAlloc {
    pub const fn new() -> Self {
        Self {
            start: 0,
            end: 0,
            next: 0,
            allocs: 0,
        }
    }
    pub unsafe fn init(&mut self, start: usize, size: usize) {
        self.start = start;
        self.end = start.saturating_add(size);
        self.next = start;
    }
}
unsafe impl GlobalAlloc for Locked<BumpAlloc> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut bump = self.lock();
        let start = align_up(bump.next, layout.align());
        let end = match start.checked_add(layout.size()) {
            Some(end) => end,
            None => return ptr::null_mut(),
        };
        match end > bump.end {
            true => ptr::null_mut(),
            false => {
                bump.next = end;
                bump.allocs += 1;
                start as *mut u8
            }
        }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let mut bump = self.lock();
        bump.allocs -= 1;
        if bump.allocs == 0 {
            bump.next = bump.start;
        }
    }
}
