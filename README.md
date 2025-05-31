# 🦀 Div-OS

**Div-OS** is a minimal, bare-metal operating system written in [Rust](https://www.rust-lang.org/), designed to explore and implement core operating system concepts from scratch. Built with no standard library or external OS dependencies, it runs directly on hardware or in a virtualized environment.

This project is both a technical deep dive and a passion project—an exercise in systems programming, low-level architecture, and writing safe, performant code close to the metal.

---

## 🚀 Project Goals

- Build a fully bare-metal OS using Rust
- Explore the x86_64 architecture, boot process, and memory layout
- Implement a modular and testable kernel architecture
- Gain hands-on experience with systems-level programming

---

## ✨ Current Features

- **Pure Rust Kernel**: No `std`, no `main`, 100% `#![no_std]`
- **Custom Bootloader**: Built using `bootimage` for bootable binary generation
- **VGA Text Mode Output**: Direct text output via memory-mapped I/O
- **Interrupt Descriptor Table (IDT)**: Basic CPU exception and interrupt handling
- **Inline Assembly**: Hardware-level I/O with safe Rust abstraction
- **QEMU Virtualization**: Fast testing and emulation
- **Bare-Metal Unit Testing**: Custom test framework in a `no_std` environment

---

## 🔧 In Progress / Planned

- 🧠 **Paging & Memory Management**: Frame allocator, heap support
- ⌨️ **Keyboard Input Driver**
- 💻 **Interactive Shell**
- 📁 **Basic Filesystem Driver**
- 🧵 **Multitasking & Cooperative Scheduling**
- 🌐 **Basic Networking Stack** (Experimental)

---

## 🧪 Skills & Technologies Used

- **Rust** (nightly toolchain, `#![no_std]`, `unsafe`, `xbuild`, `bootimage`)
- **x86_64 Architecture**
- **Operating System Concepts**: memory layout, interrupt handling, paging, I/O ports
- **Inline Assembly** for hardware interaction
- **Binary formats & boot process** (ELF format, multiboot concepts)
- **Tooling**: `cargo`, `qemu`, `bootimage`, `cargo-xbuild`
- **Bare-metal testing & debugging** with no OS or standard runtime

---

## 📦 Prerequisites

To build and run Div-OS locally:

```bash
rustup override set nightly
rustup component add rust-src llvm-tools-preview
cargo install cargo-xbuild bootimage
cargo bootimage
qemu-system-x86_64 -drive format=raw,file=target/x86_64-custom/debug/bootimage-div_os.bin
```

# 👤 About Me

I'm a systems-oriented developer with a passion for building things from scratch and understanding every layer of the stack—from silicon to software.

Whether it's writing a kernel in Rust, implementing a bootloader, or working with memory-mapped I/O, I enjoy solving problems that require a deep understanding of how computers work at a fundamental level.

I'm currently looking for opportunities where I can contribute to low-level, performance-critical, or systems-heavy projects—especially in areas like OS development, embedded systems, compilers, or secure infrastructure.

Feel free to reach out if you're hiring or just want to chat about systems programming!

- 🔗 LinkedIn: [linkedin.com/in/divyanshsirohi]([https://linkedin.com/in/YOURUSERNAME](https://www.linkedin.com/in/divyansh-sirohi-796aa824a/))

# 💡 Motivation & Learning

This project started with a simple question: *How does an operating system actually work beneath all the abstractions?*

Div-OS is my attempt to answer that question by building something from the ground up—starting with a bootable binary, writing my own kernel, and handling real hardware interactions with zero runtime or standard library.

What I’ve learned so far:

- How to build a `#![no_std]` kernel in Rust from scratch
- How the CPU boots and transfers control to an operating system
- How interrupts, paging, and memory protection actually work at the hardware level
- How to set up virtual memory and interact with hardware through I/O ports and memory-mapped registers
- How to use tools like QEMU and bootimage to create a smooth build-test-debug loop

This project has significantly deepened my confidence in:
- Low-level systems thinking
- Writing and debugging unsafe Rust safely
- Working with custom target specs, toolchains, and ELF binaries

It’s a work-in-progress, but one that continues to teach me something new with every line of code.


