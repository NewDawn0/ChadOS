use core::alloc::GlobalAlloc;
use good_memory_allocator::SpinLockedAllocator;

#[global_allocator]
pub static ALLOC: LockInterfaceSpinLockedAllocator = LockInterfaceSpinLockedAllocator::new();

pub struct LockInterfaceSpinLockedAllocator {
    inner: SpinLockedAllocator,
}
impl LockInterfaceSpinLockedAllocator {
    pub const fn new() -> Self {
        Self {
            inner: SpinLockedAllocator::empty(),
        }
    }
    // Function does nothing except provide the .lock Function for itself
    pub const fn lock(&self) -> &SpinLockedAllocator {
        &self.inner
    }
}

unsafe impl GlobalAlloc for LockInterfaceSpinLockedAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        self.inner.alloc(layout)
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        self.inner.dealloc(ptr, layout)
    }
}
