# 🦀 div-os: RISC-V Bare-Metal Operating System Kernel

**div-os** is a production-quality, bare-metal operating system kernel written entirely in Rust for the RISC-V RV64 architecture. Built from scratch with no standard library dependencies, it demonstrates core OS concepts including interrupt handling, virtual memory, preemptive multitasking, and more.

---

## 🎯 Project Overview

This project implements a minimal but fully functional OS kernel that runs directly on RISC-V hardware or in QEMU emulation. It showcases:

- **Custom bootloader** with assembly entry point
- **Interrupt and trap handling** for RISC-V
- **Sv39 paging-based virtual memory**
- **Preemptive multitasking scheduler**
- **Timer interrupts via CLINT**
- **Context switching** between tasks
- **Kernel heap allocator** (linked-list based)
- **UART-based console output**
- **Frame allocator** for physical memory management

### Target Platform

- **Architecture**: RISC-V RV64GC (riscv64gc-unknown-none-elf)
- **Machine**: QEMU virt machine
- **Language**: Rust nightly (no_std environment)
- **Build System**: Cargo + Make

---

## 📁 Project Structure

```
div-os/
├── bootloader/
│   └── boot.S                 # Assembly boot code and trap vector
├── src/                       # Main kernel source
│   ├── main.rs               # Kernel entry point
│   ├── arch/
│   │   └── riscv/            # RISC-V architecture code
│   │       ├── mod.rs
│   │       ├── timer.rs      # CLINT timer driver
│   │       └── registers.rs  # Register access utilities
│   ├── memory/
│   │   ├── mod.rs
│   │   ├── paging.rs         # Sv39 paging implementation
│   │   ├── allocator.rs      # Kernel heap allocator
│   │   └── frame_allocator.rs # Physical frame allocator
│   ├── interrupt/
│   │   └── mod.rs            # Trap and interrupt handling
│   ├── scheduler/
│   │   └── mod.rs            # Round-robin scheduler
│   ├── drivers/
│   │   ├── mod.rs
│   │   └── uart.rs           # NS16550A UART driver
│   ├── task/
│   │   └── mod.rs            # Task control blocks and context switching
│   └── sync/
│       └── mod.rs            # Synchronization primitives
├── linker.ld                  # Linker script for memory layout
├── Makefile                   # Build automation
├── Cargo.toml                 # Rust dependencies
└── README.md                  # This file
```

---

## 🚀 Quick Start

### Prerequisites

1. **Rust nightly toolchain**:
   ```bash
   rustup override set nightly
   rustup component add rust-src
   rustup target add riscv64gc-unknown-none-elf
   ```

2. **QEMU** (for testing):
   ```bash
   # Ubuntu/Debian
   sudo apt install qemu-system-riscv64
   
   # macOS
   brew install qemu
   
   # Arch Linux
   sudo pacman -S qemu-system-riscv
   ```

### Building

```bash
# Build debug version
make build

# Build release version (optimized)
make release
```

### Running in QEMU

```bash
# Run the kernel in QEMU
make run

# Run release version
make run-release

# Exit QEMU: Press Ctrl-A, then X
```

### Example Output

When you run the kernel, you should see:

```
╔════════════════════════════════════════╗
║        div-os RISC-V Kernel v0.2      ║
║    Bare-metal OS written in Rust      ║
╚════════════════════════════════════════╝

[BOOT] Kernel starting...
[ARCH] Initializing RISC-V architecture
[ARCH] RISC-V architecture initialized
[MEMORY] Initializing memory management
[FRAME] Initializing frame allocator:
  Kernel end: 0x802xxxxx
  Phys start: 0x802xxxxx
  Phys end:   0x88000000
  Total frames: XXXX
[HEAP] Initializing heap:
  Start: 0x802xxxxx
  End:   0x812xxxxx
  Size:  16777216 bytes (16384 KB)
[MEMORY] Memory management initialized
[INTERRUPT] Interrupt system initialized
[SCHEDULER] Initializing preemptive round-robin scheduler

[BOOT] Kernel initialization complete!
[BOOT] System is ready.

[DEMO] Creating sample tasks...
[DEMO] Testing heap allocation...
[DEMO] Heap allocation successful: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
[DEMO] Task demonstration complete.
[KERNEL] Entering main loop...

[KERNEL] Heartbeat... (iteration 0)
[KERNEL] Heartbeat... (iteration 1)
...
```

---

## 🧠 Architecture Deep Dive

### Boot Process

1. **Assembly Entry** (`bootloader/boot.S`):
   - CPU starts execution at `_start`
   - Sets up stack pointer (`sp`) to end of stack region
   - Initializes global pointer (`gp`) for data access relaxation
   - Clears BSS section (uninitialized data)
   - Sets up trap vector register (`stvec`)
   - Jumps to Rust `kernel_main()`

2. **Kernel Initialization** (`src/main.rs`):
   - Initialize UART for console output
   - Initialize architecture-specific features (timer)
   - Set up memory management (frame allocator + heap)
   - Enable interrupts
   - Initialize scheduler
   - Enter main kernel loop

### Memory Layout

The linker script (`linker.ld`) defines the memory layout:

```
0x8020_0000  ┌─────────────┐
             │  .text      │  Kernel code
             ├─────────────┤
             │  .rodata    │  Read-only data
             ├─────────────┤
             │  .data      │  Initialized data
             │  (gp here)  │  Global pointer
             ├─────────────┤
             │  .bss       │  Uninitialized data
             ├─────────────┤
             │  .stack     │  512KB stack (grows down)
             ├─────────────┤
             │  .heap      │  16MB heap
             └─────────────┘
```

**Key Memory Regions**:
- **Code**: 0x8020_0000 - Executable instructions
- **Data**: After code - Global and static variables
- **Stack**: 512 KB - Function calls and local variables
- **Heap**: 16 MB - Dynamic allocations

### Interrupt & Trap Flow

```
Hardware Event → CPU → stvec → _trap_vector (ASM)
                                     ↓
                              Save registers to stack
                                     ↓
                              trap_handler() (Rust)
                                     ↓
                    ┌────────────────┴────────────────┐
                    │                                 │
            Timer Interrupt                    Exception
                    │                                 │
          handle_timer_interrupt()           Handle exception
                    │                                 │
          set_next_timer()                    sepc += 4
                    │                                 │
                    └────────────────┬────────────────┘
                                     ↓
                              Restore registers
                                     ↓
                                   sret
```

**Trap Types Handled**:
- **Supervisor Timer Interrupt**: For preemptive scheduling
- **Supervisor Software Interrupt**: For IPIs (future)
- **Supervisor External Interrupt**: For device I/O
- **Exceptions**: Breakpoint, page faults, illegal instructions

### Virtual Memory (Sv39 Paging)

RISC-V Sv39 provides 3-level page tables with 39-bit virtual addresses:

```
Virtual Address (39 bits):
┌─────┬─────┬─────┬────────┐
│ VPN2│ VPN1│ VPN0│ Offset │
└─────┴─────┴─────┴────────┘
  9bit  9bit  9bit   12bit

Page Table Entry (64 bits):
┌────────┬─────┬─────────────┐
│   PPN  │ RSW │    Flags    │
└────────┴─────┴─────────────┘
  44bit   2bit     10bit
```

**PTE Flags**:
- **V**: Valid
- **R**: Readable
- **W**: Writable
- **X**: Executable
- **U**: User accessible
- **G**: Global mapping
- **A**: Accessed
- **D**: Dirty

### Scheduler Design

The kernel implements a preemptive round-robin scheduler:

1. **Task Structure**: Each task has a Task Control Block (TCB) with:
   - Task ID
   - State (Ready/Running/Blocked)
   - Saved context (registers)
   - 64KB stack

2. **Context Switching**:
   ```rust
   Current Task ────┐
                    │ Save: ra, sp, s0-s11
                    ▼
              ┌──────────┐
              │ Context  │
              └──────────┘
                    │
                    │ Restore: ra, sp, s0-s11
                    ▼
   Next Task ───────┘
   ```

3. **Preemption**: Timer interrupts trigger the scheduler every 10ms

### Memory Allocators

**Frame Allocator** (`frame_allocator.rs`):
- Manages physical 4KB frames
- Bitmap-based tracking (1 bit per frame)
- Used for page table allocation

**Heap Allocator** (`allocator.rs`):
- Linked-list based allocator
- First-fit allocation strategy
- Supports `alloc` crate (Vec, Box, etc.)
- 16MB heap size

### UART Driver

Memory-mapped UART (NS16550A compatible) at 0x1000_0000:

```
Register Map:
+0  THR  Transmit Holding Register (write)
+1  IER  Interrupt Enable Register
+3  LCR  Line Control Register
+5  LSR  Line Status Register (read)
```

---

## 🔧 Development

### Project Files

- **linker.ld**: Defines memory layout and section placement
- **boot.S**: Assembly bootstrap code
- **Cargo.toml**: Rust dependencies and build config
- **Makefile**: Build automation scripts

### Adding New Features

1. **New Device Driver**: Add to `src/drivers/`
2. **New System Call**: Add to interrupt handler
3. **New Scheduler Policy**: Modify `src/scheduler/mod.rs`
4. **New Memory Region**: Update `linker.ld`

### Debugging

```bash
# Debug with GDB
make debug

# In another terminal:
riscv64-unknown-elf-gdb target/riscv64gc-unknown-none-elf/debug/div-os
(gdb) target remote :1234
(gdb) break kernel_main
(gdb) continue
```

---

## 📚 Technical Concepts Explained

### Why RISC-V?

RISC-V is an open, free ISA that's gaining traction in systems programming:
- Clean, simple instruction set
- Excellent documentation
- Good tooling support (QEMU, GCC, LLVM)
- Privilege levels (M/S/U modes)

### Why `no_std`?

The Rust standard library (`std`) assumes an operating system exists. When writing an OS kernel:
- No heap (until we implement it)
- No threads (until we implement them)
- No file I/O
- No network stack
- Must use `core` (platform-agnostic) and `alloc` (with custom allocator)

### Why Nightly Rust?

We use unstable features:
- `alloc_error_handler`: Custom out-of-memory handler
- `global_asm`: Inline assembly files
- `build-std`: Build core/alloc from source

---

## 🎓 Learning Resources

To understand this kernel better, study these topics:

1. **RISC-V ISA**: [RISC-V Spec](https://riscv.org/technical/specifications/)
2. **OS Development**: [OSDev Wiki](https://wiki.osdev.org/)
3. **Rust Embedded**: [Embedded Rust Book](https://docs.rust-embedded.org/book/)
4. **Virtual Memory**: Understanding page tables and address translation
5. **Interrupt Handling**: Trap vectors, exception handling

---

## 🚧 Future Enhancements

Potential additions for learning:

- [ ] User mode support with syscall interface
- [ ] Multiple hart (CPU core) support
- [ ] VirtIO device drivers (block, network)
- [ ] ELF loader for user programs
- [ ] Basic filesystem (initramfs)
- [ ] SBI (Supervisor Binary Interface) calls
- [ ] Advanced scheduler (priority-based, CFS)
- [ ] Symmetric multiprocessing (SMP)

---

## 🤝 Contributing

This is a learning/portfolio project. Feel free to:
- Report issues
- Suggest improvements
- Ask questions about the implementation
- Fork and experiment

---

## 📜 License

This project is open source. See LICENSE file for details.

---

## 👤 Author

**Divyansh Sirohi**
- Portfolio/Resume project demonstrating systems programming skills
- LinkedIn: [linkedin.com/in/divyansh-sirohi-796aa824a](https://www.linkedin.com/in/divyansh-sirohi-796aa824a/)

---

## 🙏 Acknowledgments

- **RISC-V Foundation** for excellent documentation
- **Rust Embedded Working Group** for embedded Rust support
- **OSDev Community** for OS development resources
- **QEMU Project** for virtualization support

---

## 📊 Project Stats

- **Language**: Rust (no_std)
- **Lines of Code**: ~2,500+ lines
- **Architecture**: RISC-V RV64GC
- **Features**: 
  - Bootloader ✓
  - Interrupts ✓
  - Virtual Memory ✓
  - Scheduler ✓
  - Heap Allocator ✓
  - UART Driver ✓
  - Timer Support ✓
  - Context Switching ✓

---

**Built with ❤️ in Rust for RISC-V**
