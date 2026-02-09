# div-os Architecture Diagram

## System Boot Flow

```
┌─────────────────────────────────────────────────────────────┐
│                     Power On / Reset                         │
└───────────────────────┬─────────────────────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────────────┐
│                  bootloader/boot.S (_start)                  │
│  • Disable interrupts                                        │
│  • Setup stack pointer (sp = _stack_end)                     │
│  • Setup global pointer (gp = __global_pointer$)             │
│  • Clear BSS section (zero uninitialized data)               │
│  • Setup trap vector (stvec = _trap_vector)                  │
└───────────────────────┬─────────────────────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────────────┐
│                    kernel_main() [Rust]                      │
│  1. Initialize UART driver                                   │
│  2. Print boot banner                                        │
│  3. Initialize architecture (RISC-V timer)                   │
│  4. Initialize memory management                             │
│  5. Enable interrupts                                        │
│  6. Initialize scheduler                                     │
│  7. Run demo tasks                                           │
│  8. Enter main kernel loop                                   │
└───────────────────────┬─────────────────────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────────────┐
│                     Main Kernel Loop                         │
│  • Wait for interrupts (wfi)                                 │
│  • Handle timer ticks                                        │
│  • Schedule tasks                                            │
│  • Never returns                                             │
└─────────────────────────────────────────────────────────────┘
```

## Memory Subsystem Architecture

```
┌──────────────────────────────────────────────────────────────┐
│                     Memory Management                         │
└─────┬────────────────────────────────────────────────────────┘
      │
      ├─► ┌────────────────────────────────────────────────┐
      │   │         Frame Allocator                         │
      │   │  • Manages physical 4KB frames                  │
      │   │  • Bitmap-based allocation                      │
      │   │  • Used for page tables                         │
      │   │  • Located in: memory/frame_allocator.rs        │
      │   └────────────────────────────────────────────────┘
      │
      ├─► ┌────────────────────────────────────────────────┐
      │   │         Heap Allocator                          │
      │   │  • Manages kernel heap (16MB)                   │
      │   │  • Linked-list allocator                        │
      │   │  • First-fit strategy                           │
      │   │  • Implements GlobalAlloc trait                 │
      │   │  • Located in: memory/allocator.rs              │
      │   └────────────────────────────────────────────────┘
      │
      └─► ┌────────────────────────────────────────────────┐
          │         Paging (Sv39)                           │
          │  • 3-level page tables                          │
          │  • 39-bit virtual addresses                     │
          │  • 4KB pages                                    │
          │  • Identity mapping (currently)                 │
          │  • Located in: memory/paging.rs                 │
          └────────────────────────────────────────────────┘
```

## Interrupt & Trap Flow

```
┌─────────────────────────────────────────────────────────────┐
│                     Hardware Event                           │
│  (Timer, Exception, External Interrupt)                      │
└───────────────────────┬─────────────────────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────────────┐
│                CPU Trap (stvec points here)                  │
└───────────────────────┬─────────────────────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────────────┐
│              _trap_vector [Assembly]                         │
│  • Save all 32 registers to stack                            │
│  • Call trap_handler with context pointer                    │
│  • Located in: bootloader/boot.S                             │
└───────────────────────┬─────────────────────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────────────┐
│              trap_handler() [Rust]                           │
│  • Read scause, sepc, stval registers                        │
│  • Determine trap type                                       │
│  • Located in: interrupt/mod.rs                              │
└───────┬───────────────────────────────────────┬─────────────┘
        │                                       │
        ▼                                       ▼
┌──────────────────┐                  ┌──────────────────────┐
│  Timer Interrupt │                  │     Exception        │
│  • Set next timer│                  │  • Handle fault      │
│  • Call scheduler│                  │  • Update sepc       │
└──────────────────┘                  └──────────────────────┘
        │                                       │
        └───────────────┬───────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────────────┐
│              Return from trap_handler                        │
└───────────────────────┬─────────────────────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────────────┐
│              _trap_vector [Assembly]                         │
│  • Restore all 32 registers from stack                       │
│  • Execute sret (return from supervisor trap)                │
└───────────────────────┬─────────────────────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────────────┐
│              Resume execution at sepc                        │
└─────────────────────────────────────────────────────────────┘
```

## Task Scheduler Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                        Scheduler                             │
│  • Round-robin policy                                        │
│  • Timer-driven preemption (10ms quantum)                    │
│  • Task queue (VecDeque)                                     │
│  • Located in: scheduler/mod.rs                              │
└───────────┬────────────────────────────────────┬────────────┘
            │                                    │
            │                                    │
     ┌──────▼──────┐                      ┌──────▼──────┐
     │   Task 1    │                      │   Task 2    │
     │  - ID: 1    │                      │  - ID: 2    │
     │  - State    │                      │  - State    │
     │  - Context  │◄────── switch ──────►│  - Context  │
     │  - Stack    │      context()       │  - Stack    │
     └─────────────┘                      └─────────────┘

Context Switch (task/mod.rs):
  1. Save current task: ra, sp, s0-s11
  2. Switch to next task
  3. Restore next task: ra, sp, s0-s11
  4. Return to new task (ret instruction)
```

## Driver Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                          Drivers                             │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ▼
          ┌──────────────────────────────┐
          │        UART Driver            │
          │  • NS16550A compatible        │
          │  • Memory-mapped at 0x1000000 │
          │  • Console output             │
          │  • print!/println! macros     │
          │  • Located in: drivers/uart.rs│
          └──────────────────────────────┘

Memory Map (QEMU virt machine):
  0x0000_0000 - Debug/Test interface
  0x0200_0000 - CLINT (timer)
  0x1000_0000 - UART0 (serial console)  ◄── We use this
  0x8000_0000 - RAM start
  0x8020_0000 - Kernel load address     ◄── We are here
```

## Module Dependencies

```
main.rs
  ├─► arch/riscv/
  │     ├─► timer.rs        (CLINT timer)
  │     └─► registers.rs    (CSR access)
  │
  ├─► memory/
  │     ├─► allocator.rs    (Heap allocator)
  │     ├─► frame_allocator.rs (Physical frames)
  │     └─► paging.rs       (Virtual memory)
  │
  ├─► interrupt/
  │     └─► mod.rs          (Trap handler)
  │
  ├─► drivers/
  │     └─► uart.rs         (Serial console)
  │
  ├─► scheduler/
  │     └─► mod.rs          (Task scheduler)
  │
  ├─► task/
  │     └─► mod.rs          (TCB, context switch)
  │
  └─► sync/
        └─► mod.rs          (Mutex, locks)
```

## Build Process

```
┌─────────────────────────────────────────────────────────────┐
│                     cargo build                              │
└───────────────────────┬─────────────────────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────────────┐
│  1. Build core, compiler_builtins, alloc from source         │
│     (no_std environment)                                     │
└───────────────────────┬─────────────────────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────────────┐
│  2. Compile Rust source files                                │
│     • src/**/*.rs → .o files                                 │
│     • Target: riscv64gc-unknown-none-elf                     │
└───────────────────────┬─────────────────────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────────────┐
│  3. Assemble bootloader                                      │
│     • bootloader/boot.S → .o file                            │
└───────────────────────┬─────────────────────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────────────┐
│  4. Link everything with rust-lld                            │
│     • Use linker.ld script                                   │
│     • Resolve symbols                                        │
│     • Apply memory layout                                    │
└───────────────────────┬─────────────────────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────────────┐
│  5. Generate ELF binary                                      │
│     • target/riscv64gc-unknown-none-elf/debug/div-os         │
│     • Ready to run in QEMU!                                  │
└─────────────────────────────────────────────────────────────┘
```

## Key Technologies Used

- **Language**: Rust nightly (no_std)
- **Target**: RISC-V RV64GC (riscv64gc-unknown-none-elf)
- **Assembler**: GNU as (via Rust build)
- **Linker**: rust-lld (LLVM linker)
- **Emulator**: QEMU virt machine
- **Build Tool**: Cargo + Make

## Memory Safety Features

```
Rust's Ownership System:
  ├─► No null pointer dereferences
  ├─► No use-after-free
  ├─► No data races
  ├─► Memory safety without GC
  └─► Compile-time guarantees

Unsafe code limited to:
  ├─► Hardware register access
  ├─► Memory-mapped I/O
  ├─► Assembly integration
  └─► Allocator implementation
```

---

This architecture provides a solid foundation for learning OS concepts while maintaining clean, modular design principles.
