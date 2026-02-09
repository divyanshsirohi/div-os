// kernel/task/mod.rs
//! Task management and context switching
//! 
//! This module provides:
//! - Task Control Block (TCB) structure
//! - Task state management
//! - Context switching support

use core::sync::atomic::{AtomicUsize, Ordering};

/// Task ID type
pub type TaskId = usize;

/// Atomic task ID counter
static NEXT_TASK_ID: AtomicUsize = AtomicUsize::new(1);

/// Task state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskState {
    Ready,
    Running,
    Blocked,
}

/// Saved CPU context for a task
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Context {
    /// Return address (ra/x1)
    pub ra: usize,
    /// Stack pointer (sp/x2)
    pub sp: usize,
    /// Saved registers (s0-s11)
    pub s: [usize; 12],
}

impl Context {
    pub const fn new() -> Self {
        Context {
            ra: 0,
            sp: 0,
            s: [0; 12],
        }
    }
}

/// Task Control Block
pub struct Task {
    pub id: TaskId,
    pub state: TaskState,
    pub context: Context,
    pub stack: [u8; TASK_STACK_SIZE],
}

/// Task stack size (64KB per task)
pub const TASK_STACK_SIZE: usize = 64 * 1024;

impl Task {
    /// Create a new task
    pub fn new(entry: fn() -> !) -> Self {
        let id = NEXT_TASK_ID.fetch_add(1, Ordering::Relaxed);
        let mut task = Task {
            id,
            state: TaskState::Ready,
            context: Context::new(),
            stack: [0; TASK_STACK_SIZE],
        };

        // Set up initial stack
        let stack_top = task.stack.as_ptr() as usize + TASK_STACK_SIZE;
        task.context.sp = stack_top;
        task.context.ra = entry as usize;

        task
    }

    /// Get task ID
    pub fn id(&self) -> TaskId {
        self.id
    }
}

/// Context switch between tasks
/// 
/// This function saves the current task's context and restores the next task's context.
/// It's written in assembly for precise control over register saving/restoring.
#[unsafe(naked)]
pub unsafe extern "C" fn switch_context(current: *mut Context, next: *const Context) {
    core::arch::naked_asm!(
        // Save current context
        "sd ra, 0(a0)",       // Save return address
        "sd sp, 8(a0)",       // Save stack pointer
        "sd s0, 16(a0)",      // Save s0
        "sd s1, 24(a0)",      // Save s1
        "sd s2, 32(a0)",
        "sd s3, 40(a0)",
        "sd s4, 48(a0)",
        "sd s5, 56(a0)",
        "sd s6, 64(a0)",
        "sd s7, 72(a0)",
        "sd s8, 80(a0)",
        "sd s9, 88(a0)",
        "sd s10, 96(a0)",
        "sd s11, 104(a0)",
        
        // Restore next context
        "ld ra, 0(a1)",       // Restore return address
        "ld sp, 8(a1)",       // Restore stack pointer
        "ld s0, 16(a1)",      // Restore s0
        "ld s1, 24(a1)",      // Restore s1
        "ld s2, 32(a1)",
        "ld s3, 40(a1)",
        "ld s4, 48(a1)",
        "ld s5, 56(a1)",
        "ld s6, 64(a1)",
        "ld s7, 72(a1)",
        "ld s8, 80(a1)",
        "ld s9, 88(a1)",
        "ld s10, 96(a1)",
        "ld s11, 104(a1)",
        
        "ret",                // Return to new task
    )
}
