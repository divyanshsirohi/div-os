// kernel/memory/frame_allocator.rs
//! Physical frame allocator
//! 
//! Manages physical memory frames (4KB pages) for the kernel.
//! Uses a simple bitmap-based allocator.

use spin::Mutex;
use crate::println;

/// Frame size (4KB)
pub const FRAME_SIZE: usize = 4096;

/// Maximum number of frames we support (for 128MB of RAM)
const MAX_FRAMES: usize = (128 * 1024 * 1024) / FRAME_SIZE;

/// Bitmap to track free frames (1 = free, 0 = allocated)
static FRAME_BITMAP: Mutex<[u64; MAX_FRAMES / 64]> = Mutex::new([0; MAX_FRAMES / 64]);

/// Start of allocatable physical memory
static mut PHYS_MEM_START: usize = 0;
static mut PHYS_MEM_END: usize = 0;

/// Physical address wrapper
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PhysAddr(pub usize);

impl PhysAddr {
    pub fn new(addr: usize) -> Self {
        PhysAddr(addr)
    }

    pub fn as_usize(&self) -> usize {
        self.0
    }

    /// Get frame number from physical address
    pub fn frame_number(&self) -> usize {
        self.0 / FRAME_SIZE
    }
}

/// Physical frame
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Frame {
    pub number: usize,
}

impl Frame {
    pub fn containing_address(addr: PhysAddr) -> Self {
        Frame {
            number: addr.frame_number(),
        }
    }

    pub fn start_address(&self) -> PhysAddr {
        PhysAddr(self.number * FRAME_SIZE)
    }
}

/// Frame allocator
pub struct FrameAllocator {
    next_free: usize,
}

impl FrameAllocator {
    const fn new() -> Self {
        FrameAllocator { next_free: 0 }
    }

    /// Allocate a physical frame
    pub fn allocate_frame(&mut self) -> Option<Frame> {
        let mut bitmap = FRAME_BITMAP.lock();
        
        // Find a free frame
        for i in self.next_free..MAX_FRAMES {
            let word_index = i / 64;
            let bit_index = i % 64;
            
            if word_index >= bitmap.len() {
                break;
            }
            
            if (bitmap[word_index] & (1 << bit_index)) != 0 {
                // Mark as allocated
                bitmap[word_index] &= !(1 << bit_index);
                self.next_free = i + 1;
                return Some(Frame { number: i });
            }
        }
        
        None
    }

    /// Deallocate a physical frame
    pub fn deallocate_frame(&mut self, frame: Frame) {
        let mut bitmap = FRAME_BITMAP.lock();
        let word_index = frame.number / 64;
        let bit_index = frame.number % 64;
        
        if word_index < bitmap.len() {
            bitmap[word_index] |= 1 << bit_index;
            if frame.number < self.next_free {
                self.next_free = frame.number;
            }
        }
    }
}

static FRAME_ALLOCATOR: Mutex<FrameAllocator> = Mutex::new(FrameAllocator::new());

/// Initialize frame allocator
pub fn init() {
    extern "C" {
        static _kernel_end: u8;
    }

    unsafe {
        // Physical memory starts after kernel
        let kernel_end = &_kernel_end as *const u8 as usize;
        PHYS_MEM_START = align_up(kernel_end, FRAME_SIZE);
        
        // Assume 128MB of RAM (QEMU default)
        PHYS_MEM_END = 0x8000_0000 + (128 * 1024 * 1024);

        let total_frames = (PHYS_MEM_END - PHYS_MEM_START) / FRAME_SIZE;

        println!("[FRAME] Initializing frame allocator:");
        println!("  Kernel end: {:#x}", kernel_end);
        println!("  Phys start: {:#x}", PHYS_MEM_START);
        println!("  Phys end:   {:#x}", PHYS_MEM_END);
        println!("  Total frames: {}", total_frames);

        // Mark all frames as free
        let mut bitmap = FRAME_BITMAP.lock();
        for i in 0..total_frames.min(MAX_FRAMES) {
            let word_index = i / 64;
            let bit_index = i % 64;
            bitmap[word_index] |= 1 << bit_index;
        }
    }
}

/// Allocate a physical frame
pub fn allocate_frame() -> Option<Frame> {
    FRAME_ALLOCATOR.lock().allocate_frame()
}

/// Deallocate a physical frame
pub fn deallocate_frame(frame: Frame) {
    FRAME_ALLOCATOR.lock().deallocate_frame(frame);
}

/// Align address up to alignment
fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}
