use crate::mem::alloc::init::{align_up, Locked};
use alloc::alloc::{GlobalAlloc, Layout};
use core::{mem, ptr};

struct ListNode {
size: usize,
next: Option<&'static mut ListNode>,
}
impl ListNode {
const fn new(size: usize) -> Self {
    Self { size, next: None }
}
fn start_addr(&self) -> usize {
    self as *const Self as usize
}
fn end_addr(&self) -> usize {
    self.start_addr() + self.size
}
}
pub struct LinkedListAlloc {
head: ListNode,
}
impl LinkedListAlloc {
pub const fn new() -> Self {
    Self {
        head: ListNode::new(0),
    }
}
pub unsafe fn init(&mut self, start: usize, size: usize) {
    self.add_free_reg(start, size);
}
unsafe fn add_free_reg(&mut self, addr: usize, size: usize) {
    assert_eq!(align_up(addr, mem::align_of::<ListNode>()), addr);
    assert!(size >= mem::size_of::<ListNode>());
    let mut node = ListNode::new(size);
    node.next = self.head.next.take();
    let node_ptr = addr as *mut ListNode;
    node_ptr.write(node);
    self.head.next = Some(&mut *node_ptr)
}
fn find_reg(&mut self, size: usize, align: usize) -> Option<(&'static mut ListNode, usize)> {
    let mut curr = &mut self.head;
    while let Some(ref mut reg) = curr.next {
        match Self::alloc_from_reg(&reg, size, align) {
            Ok(start) => {
                let next = reg.next.take();
                let ret = Some((curr.next.take().unwrap(), start));
                curr.next = next;
                return ret;
            }
            Err(_) => curr = curr.next.as_mut().unwrap(),
        }
    }
    None
}
fn alloc_from_reg(reg: &ListNode, size: usize, align: usize) -> Result<usize, ()> {
    let start = align_up(reg.start_addr(), align);
    let end = start.checked_add(size).ok_or(())?;
    if end > reg.end_addr() {
        return Err(());
    }
    let excess = reg.end_addr() - end;
    if excess > 0 && excess < mem::size_of::<ListNode>() {
        return Err(());
    }
    Ok(start)
}
fn size_align(layout: Layout) -> (usize, usize) {
    let layout = layout
        .align_to(mem::align_of::<ListNode>())
        .expect("Adjustment of LinkedListAlloc alignment failed")
        .pad_to_align();
    let size = layout.size().max(mem::size_of::<ListNode>());
    (size, layout.align())
}
}
unsafe impl GlobalAlloc for Locked<LinkedListAlloc> {
unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
    let (size, align) = LinkedListAlloc::size_align(layout);
    let mut alloc = self.lock();
    match alloc.find_reg(size, align) {
        Some((region, alloc_start)) => {
            let alloc_end = alloc_start.checked_add(size).expect("Buffer overflow");
            let excess_size = region.end_addr() - alloc_end;
            if excess_size > 0 {
                alloc.add_free_reg(alloc_end, excess_size);
            }
            alloc_start as *mut u8
        }
        None => ptr::null_mut(),
    }
}
unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
    let (size, _) = LinkedListAlloc::size_align(layout);
    self.lock().add_free_reg(ptr as usize, size)
}
}
