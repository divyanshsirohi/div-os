# 🦀 Div-OS

**Div-OS** is a hobby operating system written in [Rust](https://www.rust-lang.org/), aiming to explore the inner workings of low-level systems with a focus on safety, performance, and modularity. It is designed to boot on real or virtual hardware and operate without any standard library or existing OS.

This project serves both as a learning journey and a practical dive into systems programming—building an OS kernel from the ground up with control over every byte of execution.

---

## 🚀 Goals

- Build a fully bare-metal OS with Rust
- Emphasize safety and memory correctness without garbage collection
- Understand the boot process, memory management, and hardware communication
- Create a foundation for experimentation with OS-level features

---

## ✨ Key Features

- **Pure Rust Kernel** (`#![no_std]`, `#![no_main]`)
- **Custom Bootloader** using the `bootimage` toolchain
- **VGA Text Mode Output** for early-stage user interaction
- **Interrupt Descriptor Table (IDT)** and basic CPU exception handling
- **Inline Assembly Support** for port I/O and hardware-level tasks
- **QEMU-compatible** for fast and safe development iterations
- **Unit Testing in Kernel Code** under a no-OS context

---

## 🔧 Planned Enhancements

- 🧠 **Memory Management**: Frame allocator, paging, and heap
- ⌨️ **Keyboard Input**: Basic input device handling
- 💻 **Command-line Shell**: Minimal shell for interaction
- 📄 **File System Support**: Read/write support for a simple FS
- 🧵 **Multitasking**: Cooperative threading and process scheduling
- 🌐 **Networking (experimental)**: Exploring TCP/IP stack basics

---

## 📦 Requirements

To build and run Div-OS, you'll need:

- Rust nightly toolchain (`rustup override set nightly`)
- `cargo-xbuild` and `bootimage`
- `qemu-system-x86_64` (or real hardware with USB booting)
- Familiarity with low-level concepts (paging, interrupts, I/O ports)

---

## 🧪 Building & Running

```bash
rustup override set nightly
rustup component add rust-src llvm-tools-preview
cargo install cargo-xbuild bootimage
cargo bootimage
qemu-system-x86_64 -drive format=raw,file=target/x86_64-custom/debug/bootimage-div_os.bin
