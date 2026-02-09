// kernel/memory/mod.rs
//! Memory management module

pub mod allocator;
pub mod paging;
pub mod frame_allocator;

use crate::println;

/// Initialize memory management
pub fn init() {
    println!("[MEMORY] Initializing memory management");
    
    // Initialize frame allocator
    frame_allocator::init();
    
    // Initialize heap allocator
    allocator::init_heap();
    
    // Initialize paging (virtual memory)
    paging::init();
    
    println!("[MEMORY] Memory management initialized");
}
