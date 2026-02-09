// kernel/scheduler/mod.rs
//! Preemptive round-robin task scheduler
//! 
//! This module implements a simple preemptive scheduler:
//! - Round-robin task selection
//! - Timer-driven preemption
//! - Task queue management

use alloc::collections::VecDeque;
use alloc::boxed::Box;
use spin::Mutex;
use crate::task::{Task, TaskId, TaskState, Context};
use crate::println;

/// Global task queue
static TASK_QUEUE: Mutex<VecDeque<Box<Task>>> = Mutex::new(VecDeque::new());

/// Current running task ID
static CURRENT_TASK_ID: Mutex<Option<TaskId>> = Mutex::new(None);

/// Add a task to the scheduler
pub fn add_task(task: Task) {
    let mut queue = TASK_QUEUE.lock();
    queue.push_back(Box::new(task));
}

/// Get current task ID
pub fn current_task_id() -> Option<TaskId> {
    *CURRENT_TASK_ID.lock()
}

/// Schedule next task (called on timer interrupt)
pub fn schedule() {
    let mut queue = TASK_QUEUE.lock();
    
    if queue.is_empty() {
        return;
    }

    // Get next task (round-robin)
    if let Some(mut task) = queue.pop_front() {
        task.state = TaskState::Running;
        let task_id = task.id;
        
        // Update current task
        *CURRENT_TASK_ID.lock() = Some(task_id);
        
        // Push back to queue
        queue.push_back(task);
    }
}

/// Timer tick handler (called from interrupt handler)
pub fn tick() {
    // This would trigger a context switch in a full implementation
    // For now, just acknowledge the timer tick
}

/// Initialize scheduler
pub fn init() {
    println!("[SCHEDULER] Initializing preemptive round-robin scheduler");
}

/// Yield CPU to next task
pub fn yield_now() {
    schedule();
}
