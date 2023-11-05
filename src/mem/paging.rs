// Imports
use bootloader::bootinfo::{BootInfo, MemoryMap, MemoryRegionType};
use x86_64::{
    registers::control::Cr3,
    structures::paging::{FrameAllocator, OffsetPageTable, PageTable, PhysFrame, Size4KiB},
    PhysAddr, VirtAddr,
};

// Init func
unsafe fn uinit(phys_mem_offset: VirtAddr) -> OffsetPageTable<'static> {
    OffsetPageTable::new(active_lvl4_pt(phys_mem_offset), phys_mem_offset)
}

// Wrapped init fn
pub fn init(boot_info: &'static BootInfo) -> (OffsetPageTable, BootInfoFrameAlloc) {
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mapper = unsafe { uinit(phys_mem_offset) };
    let frame_alloc = unsafe { BootInfoFrameAlloc::init(&boot_info.memory_map) };
    (mapper, frame_alloc)
}

// Get level 4 PageTable
unsafe fn active_lvl4_pt(phys_mem_offset: VirtAddr) -> &'static mut PageTable {
    let (lvl4_pt, _) = Cr3::read();
    let phys = lvl4_pt.start_address();
    let virt = phys_mem_offset + phys.as_u64();
    let pt_ptr: *mut PageTable = virt.as_mut_ptr();
    &mut *pt_ptr
}

// Empty frame allocator
pub struct EmptyFrameAlloc;
unsafe impl FrameAllocator<Size4KiB> for EmptyFrameAlloc {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        None
    }
}

// Boot info frame alloc
pub struct BootInfoFrameAlloc {
    mem_map: &'static MemoryMap,
    next: usize,
}
impl BootInfoFrameAlloc {
    // Init the boot info frame
    unsafe fn init(mem_map: &'static MemoryMap) -> Self {
        Self { mem_map, next: 0 }
    }
    // get usable frames
    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        let regions = self.mem_map.iter();
        let usable_regions = regions.filter(|r| r.region_type == MemoryRegionType::Usable);
        let addr_ranges = usable_regions.map(|r| r.range.start_addr()..r.range.end_addr());
        let frame_addrs = addr_ranges.flat_map(|r| r.step_by(4096));
        frame_addrs.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}
// Allocate 4KiB frames in the boot info frame allocator
unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAlloc {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}
