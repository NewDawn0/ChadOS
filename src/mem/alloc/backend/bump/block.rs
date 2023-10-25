use crate::cfg::mem::BLOCK_SIZES;
use crate::mem::alloc::init::Locked;
use alloc::alloc::{GlobalAlloc, Layout};
use core::{
    mem,
    ptr::{self, NonNull},
};
use linked_list_allocator::Heap;

fn list_index(layout: &Layout) -> Option<usize> {
    let req_block_size = layout.size().max(layout.align());
    BLOCK_SIZES.iter().position(|&s| s >= req_block_size)
}
struct ListNode {
    next: Option<&'static mut ListNode>,
}
pub struct FixedSizeBlockAlloc {
    list_heads: [Option<&'static mut ListNode>; BLOCK_SIZES.len()],
    fallback_alloc: Heap,
}
impl FixedSizeBlockAlloc {
    pub const fn new() -> Self {
        const EMPTY: Option<&'static mut ListNode> = None;
        Self {
            list_heads: [EMPTY; BLOCK_SIZES.len()],
            fallback_alloc: linked_list_allocator::Heap::empty(),
        }
    }
    pub unsafe fn init(&mut self, start: usize, size: usize) {
        self.fallback_alloc.init(start as *mut u8, size)
    }
    fn fallback_alloc(&mut self, layout: Layout) -> *mut u8 {
        match self.fallback_alloc.allocate_first_fit(layout) {
            Ok(ptr) => ptr.as_ptr(),
            Err(_) => ptr::null_mut(),
        }
    }
}
unsafe impl GlobalAlloc for Locked<FixedSizeBlockAlloc> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut alloc = self.lock();
        match list_index(&layout) {
            Some(index) => match alloc.list_heads[index].take() {
                Some(node) => {
                    alloc.list_heads[index] = node.next.take();
                    node as *mut ListNode as *mut u8
                }
                None => {
                    let layout =
                        Layout::from_size_align(BLOCK_SIZES[index], BLOCK_SIZES[index]).unwrap();
                    alloc.fallback_alloc(layout)
                }
            },
            None => alloc.fallback_alloc(layout),
        }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let mut alloc = self.lock();
        match list_index(&layout) {
            Some(index) => {
                let new_node = ListNode {
                    next: alloc.list_heads[index].take(),
                };
                // verify that block has size and alignment required for storing node
                assert!(mem::size_of::<ListNode>() <= BLOCK_SIZES[index]);
                assert!(mem::align_of::<ListNode>() <= BLOCK_SIZES[index]);
                let new_node_ptr = ptr as *mut ListNode;
                new_node_ptr.write(new_node);
                alloc.list_heads[index] = Some(&mut *new_node_ptr);
            }
            None => {
                let ptr = NonNull::new(ptr).unwrap();
                alloc.fallback_alloc.deallocate(ptr, layout);
            }
        }
    }
}
