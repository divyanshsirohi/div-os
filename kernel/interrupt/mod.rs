// kernel/interrupt/mod.rs
//! Interrupt and trap handling for RISC-V
//! 
//! This module handles:
//! - Trap/exception handling
//! - Timer interrupts from CLINT
//! - External interrupts
//! - Register context save/restore

use crate::println;

/// Saved register context during trap
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TrapContext {
    pub x: [usize; 32],  // General purpose registers x0-x31
}

impl TrapContext {
    pub const fn new() -> Self {
        TrapContext { x: [0; 32] }
    }
}

/// Trap cause types (scause register values)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrapCause {
    Interrupt(Interrupt),
    Exception(Exception),
    Unknown(usize),
}

/// Interrupt types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Interrupt {
    SupervisorSoftware,
    SupervisorTimer,
    SupervisorExternal,
    Unknown(usize),
}

/// Exception types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Exception {
    InstructionAddressMisaligned,
    InstructionAccessFault,
    IllegalInstruction,
    Breakpoint,
    LoadAddressMisaligned,
    LoadAccessFault,
    StoreAddressMisaligned,
    StoreAccessFault,
    UserEnvCall,
    SupervisorEnvCall,
    InstructionPageFault,
    LoadPageFault,
    StorePageFault,
    Unknown(usize),
}

impl TrapCause {
    /// Parse trap cause from scause register
    pub fn from_scause(scause: usize) -> Self {
        let is_interrupt = (scause >> 63) != 0;
        let code = scause & 0x7FFFFFFFFFFFFFFF;

        if is_interrupt {
            let interrupt = match code {
                1 => Interrupt::SupervisorSoftware,
                5 => Interrupt::SupervisorTimer,
                9 => Interrupt::SupervisorExternal,
                _ => Interrupt::Unknown(code),
            };
            TrapCause::Interrupt(interrupt)
        } else {
            let exception = match code {
                0 => Exception::InstructionAddressMisaligned,
                1 => Exception::InstructionAccessFault,
                2 => Exception::IllegalInstruction,
                3 => Exception::Breakpoint,
                4 => Exception::LoadAddressMisaligned,
                5 => Exception::LoadAccessFault,
                6 => Exception::StoreAddressMisaligned,
                7 => Exception::StoreAccessFault,
                8 => Exception::UserEnvCall,
                9 => Exception::SupervisorEnvCall,
                12 => Exception::InstructionPageFault,
                13 => Exception::LoadPageFault,
                15 => Exception::StorePageFault,
                _ => Exception::Unknown(code),
            };
            TrapCause::Exception(exception)
        }
    }
}

/// Main trap handler called from assembly
#[no_mangle]
pub extern "C" fn trap_handler(context: *mut TrapContext) {
    use riscv::register::{scause, sepc, stval};

    let scause_val = scause::read().bits();
    let sepc_val = sepc::read();
    let stval_val = stval::read();

    let cause = TrapCause::from_scause(scause_val);

    match cause {
        TrapCause::Interrupt(Interrupt::SupervisorTimer) => {
            // Handle timer interrupt
            handle_timer_interrupt();
        }
        TrapCause::Exception(Exception::Breakpoint) => {
            println!("[TRAP] Breakpoint at {:#x}", sepc_val);
            // Move past ebreak instruction
            unsafe {
                sepc::write(sepc_val + 4);
            }
        }
        _ => {
            println!("[TRAP] Unhandled trap:");
            println!("  Cause: {:?}", cause);
            println!("  sepc: {:#x}", sepc_val);
            println!("  stval: {:#x}", stval_val);
            panic!("Unhandled trap!");
        }
    }
}

/// Handle timer interrupt
fn handle_timer_interrupt() {
    use crate::arch::riscv::timer;
    
    // Acknowledge timer interrupt by setting next timer
    timer::set_next_timer();
    
    // Notify scheduler if available
    #[cfg(feature = "scheduler")]
    crate::scheduler::tick();
}

/// Initialize interrupt system
pub fn init() {
    use riscv::register::{sie, sstatus};
    
    unsafe {
        // Enable supervisor interrupts
        sstatus::set_sie();
        
        // Enable timer, software, and external interrupts
        sie::set_stimer();
        sie::set_ssoft();
        sie::set_sext();
    }
    
    println!("[INTERRUPT] Interrupt system initialized");
}
