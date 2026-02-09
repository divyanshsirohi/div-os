// kernel/main.rs
//! div-os: A bare-metal RISC-V operating system kernel
//!
//! This kernel demonstrates core OS concepts:
//! - Boot process and hardware initialization
//! - Interrupt and trap handling
//! - Virtual memory management (Sv39 paging)
//! - Physical memory allocation
//! - Kernel heap allocation
//! - Preemptive multitasking
//! - Timer-driven scheduling
//! - UART console output

#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

extern crate alloc;

use core::arch::global_asm;
use core::panic::PanicInfo;

// Kernel modules
mod arch;
mod drivers;
mod interrupt;
mod memory;
mod scheduler;
mod sync;
mod task;

// Include boot assembly
global_asm!(include_str!("../bootloader/boot.S"));

/// Kernel entry point called from assembly
#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    // Initialize UART for early output
    drivers::uart::init();

    println!("\n╔════════════════════════════════════════╗");
    println!("║        div-os RISC-V Kernel v0.2      ║");
    println!("║    Bare-metal OS written in Rust      ║");
    println!("╚════════════════════════════════════════╝\n");

    println!("[BOOT] Kernel starting...");

    // Initialize architecture-specific features
    arch::riscv::init();

    // Initialize memory management
    memory::init();

    // Initialize interrupt handling
    interrupt::init();

    // Initialize scheduler
    scheduler::init();

    println!("\n[BOOT] Kernel initialization complete!");
    println!("[BOOT] System is ready.\n");

    // Demo: Create some sample tasks
    demo_tasks();

    // Main kernel loop
    println!("[KERNEL] Entering main loop...\n");
    main_loop();
}

/// Demonstrate task creation
fn demo_tasks() {
    println!("[DEMO] Creating sample tasks...");

    // In a full implementation, we would create and schedule actual tasks here
    // For this demo, we'll just print some information

    use alloc::vec::Vec;

    println!("[DEMO] Testing heap allocation...");
    let mut vec = Vec::new();
    for i in 0..10 {
        vec.push(i);
    }
    println!("[DEMO] Heap allocation successful: {:?}", vec);

    println!("[DEMO] Task demonstration complete.");
}

/// Main kernel loop
fn main_loop() -> ! {
    let mut counter = 0;

    loop {
        // Demonstrate periodic output
        if counter % 10000000 == 0 {
            println!("[KERNEL] Heartbeat... (iteration {})", counter / 10000000);
        }
        counter += 1;

        // Wait for interrupt
        unsafe {
            riscv::asm::wfi();
        }
    }
}

/// Panic handler
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("\n╔════════════════════════════════════════╗");
    println!("║          KERNEL PANIC!                 ║");
    println!("╚════════════════════════════════════════╝");
    println!("{}", info);

    loop {
        unsafe {
            riscv::asm::wfi();
        }
    }
}

/// Out of memory handler
#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("Allocation error: {:?}", layout);
}
