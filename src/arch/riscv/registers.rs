// kernel/arch/riscv/registers.rs
//! RISC-V register access utilities
//! 
//! Provides safe wrappers around RISC-V CSR (Control and Status Registers)

/// Read stack pointer
#[inline(always)]
pub fn read_sp() -> usize {
    let sp: usize;
    unsafe {
        core::arch::asm!("mv {}, sp", out(reg) sp);
    }
    sp
}

/// Read global pointer
#[inline(always)]
pub fn read_gp() -> usize {
    let gp: usize;
    unsafe {
        core::arch::asm!("mv {}, gp", out(reg) gp);
    }
    gp
}

/// Read thread pointer
#[inline(always)]
pub fn read_tp() -> usize {
    let tp: usize;
    unsafe {
        core::arch::asm!("mv {}, tp", out(reg) tp);
    }
    tp
}
