# Testing div-os in QEMU

This guide explains how to test the div-os RISC-V kernel in QEMU.

## Prerequisites

### Install QEMU

**Ubuntu/Debian:**
```bash
sudo apt update
sudo apt install qemu-system-misc
```

**macOS:**
```bash
brew install qemu
```

**Arch Linux:**
```bash
sudo pacman -S qemu-system-riscv
```

**Windows:**
Download from https://www.qemu.org/download/#windows

## Building the Kernel

1. Ensure you have Rust nightly and the RISC-V target:
```bash
rustup override set nightly
rustup component add rust-src
rustup target add riscv64gc-unknown-none-elf
```

2. Build the kernel:
```bash
make build
```

Or use cargo directly:
```bash
cargo build
```

The kernel binary will be at:
```
target/riscv64gc-unknown-none-elf/debug/div-os
```

## Running in QEMU

### Basic Run

```bash
make run
```

This executes:
```bash
qemu-system-riscv64 \
    -machine virt \
    -nographic \
    -bios none \
    -kernel target/riscv64gc-unknown-none-elf/debug/div-os \
    -smp 1 \
    -m 128M
```

### QEMU Parameters Explained

- `-machine virt`: Use the QEMU virt machine (generic RISC-V platform)
- `-nographic`: No graphical output, use terminal for serial console
- `-bios none`: Don't use a BIOS/firmware (we have our own bootloader)
- `-kernel`: Path to kernel ELF file
- `-smp 1`: Single CPU core
- `-m 128M`: 128MB of RAM

### Exiting QEMU

To exit QEMU when running in nographic mode:
1. Press `Ctrl-A`
2. Then press `X`

Or from QEMU monitor:
1. Press `Ctrl-A`
2. Then press `C` to enter monitor
3. Type `quit` and press Enter

## Expected Output

When the kernel boots successfully, you should see:

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
[PAGING] Initializing Sv39 paging
[PAGING] Running with identity mapping
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
[KERNEL] Heartbeat... (iteration 2)
...
```

## Debug Mode

To debug the kernel with GDB:

1. Start QEMU in debug mode:
```bash
make debug
```

This will start QEMU and wait for a GDB connection on port 1234.

2. In another terminal, connect GDB:
```bash
riscv64-unknown-elf-gdb target/riscv64gc-unknown-none-elf/debug/div-os

# In GDB:
(gdb) target remote :1234
(gdb) break kernel_main
(gdb) continue
```

## Troubleshooting

### Kernel doesn't boot

**Check the build:**
```bash
cargo build --verbose
```

Look for linker errors or assembly issues.

**Check QEMU machine:**
The kernel expects QEMU virt machine with:
- UART at 0x10000000
- RAM starting at 0x80000000
- CLINT at 0x2000000

### No output appears

**Check UART initialization:**
The kernel writes to UART0 at 0x10000000. If QEMU is configured differently, output won't appear.

**Try verbose QEMU:**
```bash
qemu-system-riscv64 -machine virt -nographic -bios none \
    -kernel target/riscv64gc-unknown-none-elf/debug/div-os \
    -d int,cpu_reset -D qemu.log
```

Check `qemu.log` for details.

### Kernel panics

If the kernel panics, you'll see:
```
╔════════════════════════════════════════╗
║          KERNEL PANIC!                 ║
╚════════════════════════════════════════╝
panicked at 'message': src/file.rs:line:col
```

Common causes:
- Out of memory
- Page fault
- Unhandled interrupt
- Assertion failure

## Advanced Testing

### Test with different memory sizes

```bash
qemu-system-riscv64 -machine virt -nographic -bios none \
    -kernel target/riscv64gc-unknown-none-elf/debug/div-os \
    -m 256M  # Try 256MB
```

### Test with multiple cores

```bash
qemu-system-riscv64 -machine virt -nographic -bios none \
    -kernel target/riscv64gc-unknown-none-elf/debug/div-os \
    -smp 2  # 2 cores (currently unsupported by kernel)
```

### Monitor mode

Access QEMU monitor for inspection:
```bash
# Press Ctrl-A then C during execution
# In monitor:
info registers
info mem
info tlb
```

## CI/CD Testing

For automated testing, use:

```bash
#!/bin/bash
set -e

# Build
cargo build

# Run with timeout (10 seconds)
timeout 10 qemu-system-riscv64 \
    -machine virt \
    -nographic \
    -bios none \
    -kernel target/riscv64gc-unknown-none-elf/debug/div-os \
    -serial mon:stdio \
    -smp 1 \
    -m 128M || true

# Check for successful boot in output
# (would need to capture and parse output)
```

## Performance Testing

To test boot time and initialization:

```bash
time make run
```

Expected boot time: < 1 second in QEMU

## Next Steps

Once the kernel boots successfully:

1. **Add tests**: Create integration tests for each subsystem
2. **Add user programs**: Implement ELF loading
3. **Add more drivers**: VirtIO block, network
4. **Add filesystem**: Simple ramfs or initramfs
5. **Add more debugging**: Kernel console, debug commands

---

Happy testing! 🚀
