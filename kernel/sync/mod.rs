// kernel/sync/mod.rs
//! Synchronization primitives
//! 
//! This module provides synchronization primitives for the kernel.
//! Currently, we re-export primitives from the spin crate.

pub use spin::{Mutex, MutexGuard};

// In a full implementation, we would add:
// - Semaphores
// - Condition variables
// - Read-write locks
// - Barriers
