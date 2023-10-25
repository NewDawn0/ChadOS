use x86_64::{
    structures::paging::{
        mapper::MapToError, FrameAllocator as FrameAlloc, Mapper, Page, PageTableFlags as Flags,
        Size4KiB,
    },
    VirtAddr,
};

use crate::cfg::mem::{HEAP_SIZE, HEAP_START};
#[cfg(feature = "alloc-bump")]
use spin::{Mutex, MutexGuard};

#[cfg(feature = "alloc-bump")]
use crate::mem::alloc::backend::bump::bump::ALLOC;
#[cfg(feature = "alloc-dlmalloc")]
use crate::mem::alloc::backend::dlmalloc::ALLOC;
#[cfg(feature = "alloc-galloc")]
use crate::mem::alloc::backend::galloc::ALLOC;
#[cfg(feature = "alloc-slab")]
use crate::mem::alloc::backend::slab::ALLOC;

#[cfg(feature = "alloc-bump")]
pub struct Locked<T> {
    inner: Mutex<T>,
}
#[cfg(feature = "alloc-bump")]
impl<T> Locked<T> {
    pub const fn new(inner: T) -> Self {
        Self {
            inner: Mutex::new(inner),
        }
    }
    pub fn lock(&self) -> MutexGuard<T> {
        self.inner.lock()
    }
}

#[cfg(feature = "alloc-bump")]
pub fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}

pub fn init(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_alloc: &mut impl FrameAlloc<Size4KiB>,
) -> Result<(), MapToError<Size4KiB>> {
    let page_range = {
        let start = VirtAddr::new(HEAP_START as u64);
        let end = start + HEAP_SIZE - 1u64;
        let start_page = Page::containing_address(start);
        let end_page = Page::containing_address(end);
        Page::range_inclusive(start_page, end_page)
    };
    for page in page_range {
        let frame = frame_alloc
            .allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?;
        let flags = Flags::PRESENT | Flags::WRITABLE;
        unsafe { mapper.map_to(page, frame, flags, frame_alloc)?.flush() };
    }
    unsafe {
        ALLOC.lock().init(HEAP_START, HEAP_SIZE);
    }
    Ok(())
}
