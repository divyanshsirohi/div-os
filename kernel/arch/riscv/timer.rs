// kernel/arch/riscv/timer.rs
//! RISC-V timer support using CLINT (Core Local Interruptor)
//! 
//! The CLINT provides:
//! - Machine-mode timer (mtime, mtimecmp)
//! - Software interrupts
//! 
//! On QEMU virt machine:
//! - CLINT base address: 0x2000000
//! - mtime: offset 0xbff8
//! - mtimecmp: offset 0x4000

use core::ptr;

/// CLINT base address on QEMU virt machine
const CLINT_BASE: usize = 0x200_0000;

/// mtime register offset (current timer value)
const MTIME_OFFSET: usize = 0xbff8;

/// mtimecmp register offset for hart 0 (timer compare value)
const MTIMECMP_OFFSET: usize = 0x4000;

/// Timer interval (10ms at 10MHz clock)
const TIMER_INTERVAL: u64 = 100_000;

/// Read current time value
pub fn read_mtime() -> u64 {
    unsafe {
        let addr = (CLINT_BASE + MTIME_OFFSET) as *const u64;
        ptr::read_volatile(addr)
    }
}

/// Write timer compare value
pub fn write_mtimecmp(value: u64) {
    unsafe {
        let addr = (CLINT_BASE + MTIMECMP_OFFSET) as *mut u64;
        ptr::write_volatile(addr, value);
    }
}

/// Set the next timer interrupt
pub fn set_next_timer() {
    let current = read_mtime();
    write_mtimecmp(current + TIMER_INTERVAL);
}

/// Initialize timer
pub fn init() {
    // Set initial timer interrupt
    set_next_timer();
}
