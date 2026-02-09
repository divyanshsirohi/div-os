# Makefile for div-os RISC-V Kernel
# 
# This Makefile provides commands to build and run the kernel

# Kernel binary name
KERNEL := div-os

# Build mode (debug or release)
MODE := debug

# Target architecture
TARGET := riscv64gc-unknown-none-elf

# Build directory
BUILD_DIR := target/$(TARGET)/$(MODE)
KERNEL_BIN := $(BUILD_DIR)/$(KERNEL)
KERNEL_ELF := $(BUILD_DIR)/$(KERNEL)

# QEMU settings
QEMU := qemu-system-riscv64
QEMU_ARGS := -machine virt \
             -nographic \
             -bios none \
             -kernel $(KERNEL_ELF) \
             -smp 1 \
             -m 128M

# Rust flags
export RUSTFLAGS := -C link-arg=-Tlinker.ld

.PHONY: all build run clean help qemu

all: build

# Build the kernel
build:
	@echo "Building div-os kernel..."
	@cargo build --manifest-path=Cargo.toml

# Build release version
release:
	@echo "Building div-os kernel (release mode)..."
	@cargo build --manifest-path=Cargo.toml --release

# Run in QEMU
run: build
	@echo "Starting QEMU..."
	@echo "Press Ctrl-A then X to exit QEMU"
	@$(QEMU) $(QEMU_ARGS)

# Run release version in QEMU
run-release: release
	@echo "Starting QEMU (release mode)..."
	@echo "Press Ctrl-A then X to exit QEMU"
	@$(QEMU) $(QEMU_ARGS)

# Debug with QEMU (waits for GDB connection)
debug: build
	@echo "Starting QEMU in debug mode..."
	@echo "Waiting for GDB connection on localhost:1234"
	@$(QEMU) $(QEMU_ARGS) -s -S

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	@cargo clean

# Show help
help:
	@echo "div-os Makefile Commands:"
	@echo "  make build        - Build the kernel (debug mode)"
	@echo "  make release      - Build the kernel (release mode)"
	@echo "  make run          - Build and run in QEMU"
	@echo "  make run-release  - Build release and run in QEMU"
	@echo "  make debug        - Build and run in QEMU debug mode"
	@echo "  make clean        - Clean build artifacts"
	@echo "  make help         - Show this help message"
	@echo ""
	@echo "QEMU Controls:"
	@echo "  Ctrl-A then X     - Exit QEMU"
	@echo "  Ctrl-A then C     - Switch to QEMU monitor"
