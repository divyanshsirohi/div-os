// kernel/memory/allocator.rs
//! Kernel heap allocator
//! 
//! Implements a simple linked-list heap allocator for the kernel.
//! This is not the most efficient allocator, but it's simple and works well
//! for kernel use cases with moderate allocation patterns.

use core::alloc::{GlobalAlloc, Layout};
use core::mem;
use core::ptr;
use spin::Mutex;
use crate::println;

// Heap start and end addresses (from linker script)
extern "C" {
    static _heap_start: u8;
    static _heap_end: u8;
}

/// Node in the free list
struct ListNode {
    size: usize,
    next: Option<&'static mut ListNode>,
}

impl ListNode {
    const fn new(size: usize) -> Self {
        ListNode { size, next: None }
    }

    fn start_addr(&self) -> usize {
        self as *const Self as usize
    }

    fn end_addr(&self) -> usize {
        self.start_addr() + self.size
    }
}

/// Linked-list heap allocator
pub struct LinkedListAllocator {
    head: Option<&'static mut ListNode>,
}

impl LinkedListAllocator {
    /// Create a new empty allocator
    pub const fn new() -> Self {
        LinkedListAllocator { head: None }
    }

    /// Initialize the allocator with the given heap bounds
    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        self.add_free_region(heap_start, heap_size);
    }

    /// Add a free memory region to the allocator
    unsafe fn add_free_region(&mut self, addr: usize, size: usize) {
        // Ensure the region can hold a ListNode
        assert!(size >= mem::size_of::<ListNode>());
        assert!(addr % mem::align_of::<ListNode>() == 0);

        // Create new node and append to list
        let mut node = ListNode::new(size);
        node.next = self.head.take();
        let node_ptr = addr as *mut ListNode;
        node_ptr.write(node);
        self.head = Some(&mut *node_ptr);
    }

    /// Find a free region that fits the given layout
    fn find_region(&mut self, size: usize, align: usize) -> Option<(&'static mut ListNode, usize)> {
        // Convert the entire chain to raw pointers to avoid borrow issues
        let mut prev = &mut self.head as *mut Option<&'static mut ListNode>;
        
        unsafe {
            while let Some(ref mut region) = *prev {
                match Self::alloc_from_region(region, size, align) {
                    Ok(alloc_start) => {
                        // Remove this region from the list
                        let next = region.next.take();
                        let region_ptr: *mut ListNode = *region;
                        *prev = next;
                        return Some((&mut *region_ptr, alloc_start));
                    }
                    Err(_) => {
                        prev = &mut region.next as *mut Option<&'static mut ListNode>;
                    }
                }
            }
        }
        None
    }

    /// Try to allocate from a region
    fn alloc_from_region(region: &ListNode, size: usize, align: usize) -> Result<usize, ()> {
        let alloc_start = align_up(region.start_addr(), align);
        let alloc_end = alloc_start.checked_add(size).ok_or(())?;

        if alloc_end > region.end_addr() {
            return Err(());
        }

        let excess_size = region.end_addr() - alloc_end;
        if excess_size > 0 && excess_size < mem::size_of::<ListNode>() {
            return Err(());
        }

        Ok(alloc_start)
    }

    /// Adjust layout to meet minimum size requirements
    fn size_align(layout: Layout) -> (usize, usize) {
        let layout = layout
            .align_to(mem::align_of::<ListNode>())
            .expect("adjusting alignment failed")
            .pad_to_align();
        let size = layout.size().max(mem::size_of::<ListNode>());
        (size, layout.align())
    }
}

unsafe impl GlobalAlloc for Locked<LinkedListAllocator> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let (size, align) = LinkedListAllocator::size_align(layout);
        let mut allocator = self.lock();

        if let Some((region, alloc_start)) = allocator.find_region(size, align) {
            let alloc_end = alloc_start.checked_add(size).expect("overflow");
            let excess_size = region.end_addr() - alloc_end;

            if excess_size > 0 {
                allocator.add_free_region(alloc_end, excess_size);
            }

            alloc_start as *mut u8
        } else {
            ptr::null_mut()
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let (size, _) = LinkedListAllocator::size_align(layout);
        self.lock().add_free_region(ptr as usize, size);
    }
}

/// Wrapper to add locking to allocator
pub struct Locked<A> {
    inner: Mutex<A>,
}

impl<A> Locked<A> {
    pub const fn new(inner: A) -> Self {
        Locked {
            inner: Mutex::new(inner),
        }
    }

    pub fn lock(&self) -> spin::MutexGuard<A> {
        self.inner.lock()
    }
}

/// Align address upwards to alignment
fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}

/// Global heap allocator instance
#[global_allocator]
static ALLOCATOR: Locked<LinkedListAllocator> = Locked::new(LinkedListAllocator::new());

/// Initialize the kernel heap
pub fn init_heap() {
    unsafe {
        let heap_start = &_heap_start as *const u8 as usize;
        let heap_end = &_heap_end as *const u8 as usize;
        let heap_size = heap_end - heap_start;

        println!("[HEAP] Initializing heap:");
        println!("  Start: {:#x}", heap_start);
        println!("  End:   {:#x}", heap_end);
        println!("  Size:  {} bytes ({} KB)", heap_size, heap_size / 1024);

        ALLOCATOR.lock().init(heap_start, heap_size);
    }
}
