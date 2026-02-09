// kernel/arch/riscv/mod.rs
//! RISC-V architecture-specific code

pub mod timer;
pub mod registers;

use crate::println;

/// Initialize RISC-V architecture
pub fn init() {
    println!("[ARCH] Initializing RISC-V architecture");
    timer::init();
    println!("[ARCH] RISC-V architecture initialized");
}
