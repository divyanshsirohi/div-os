/* bootloader/boot.S
 * RISC-V Assembly Boot Code
 * 
 * This is the entry point for the kernel. It:
 * 1. Sets up the stack pointer
 * 2. Clears the BSS section
 * 3. Initializes trap vector
 * 4. Transfers control to Rust kernel
 */

.section .text.boot
.global _start

_start:
    # Disable interrupts during boot
    csrw sie, zero
    csrw sip, zero
    
    # Set up stack pointer
    # The linker script provides _stack_end symbol
    la sp, _stack_end
    
    # Set up global pointer (for relaxation)
    .option push
    .option norelax
    la gp, __global_pointer$
    .option pop
    
    # Clear BSS section
    # BSS contains uninitialized global/static variables
    la t0, _bss_start
    la t1, _bss_end
    
clear_bss:
    bgeu t0, t1, clear_bss_done
    sd zero, 0(t0)
    addi t0, t0, 8
    j clear_bss
    
clear_bss_done:
    # Set up trap vector
    # Point stvec to our trap handler
    la t0, _trap_vector
    csrw stvec, t0
    
    # Jump to Rust kernel main
    call kernel_main
    
    # If kernel_main returns (it shouldn't), loop forever
_halt:
    wfi
    j _halt

.section .text
.align 4
.global _trap_vector

_trap_vector:
    # Save all registers to stack
    addi sp, sp, -256
    
    # Save x registers (skip x0 which is always zero)
    sd x1, 0(sp)
    sd x2, 8(sp)
    sd x3, 16(sp)
    sd x4, 24(sp)
    sd x5, 32(sp)
    sd x6, 40(sp)
    sd x7, 48(sp)
    sd x8, 56(sp)
    sd x9, 64(sp)
    sd x10, 72(sp)
    sd x11, 80(sp)
    sd x12, 88(sp)
    sd x13, 96(sp)
    sd x14, 104(sp)
    sd x15, 112(sp)
    sd x16, 120(sp)
    sd x17, 128(sp)
    sd x18, 136(sp)
    sd x19, 144(sp)
    sd x20, 152(sp)
    sd x21, 160(sp)
    sd x22, 168(sp)
    sd x23, 176(sp)
    sd x24, 184(sp)
    sd x25, 192(sp)
    sd x26, 200(sp)
    sd x27, 208(sp)
    sd x28, 216(sp)
    sd x29, 224(sp)
    sd x30, 232(sp)
    sd x31, 240(sp)
    
    # Call Rust trap handler
    # Argument: pointer to saved context on stack
    mv a0, sp
    call trap_handler
    
    # Restore all registers
    ld x1, 0(sp)
    ld x2, 8(sp)
    ld x3, 16(sp)
    ld x4, 24(sp)
    ld x5, 32(sp)
    ld x6, 40(sp)
    ld x7, 48(sp)
    ld x8, 56(sp)
    ld x9, 64(sp)
    ld x10, 72(sp)
    ld x11, 80(sp)
    ld x12, 88(sp)
    ld x13, 96(sp)
    ld x14, 104(sp)
    ld x15, 112(sp)
    ld x16, 120(sp)
    ld x17, 128(sp)
    ld x18, 136(sp)
    ld x19, 144(sp)
    ld x20, 152(sp)
    ld x21, 160(sp)
    ld x22, 168(sp)
    ld x23, 176(sp)
    ld x24, 184(sp)
    ld x25, 192(sp)
    ld x26, 200(sp)
    ld x27, 208(sp)
    ld x28, 216(sp)
    ld x29, 224(sp)
    ld x30, 232(sp)
    ld x31, 240(sp)
    
    addi sp, sp, 256
    
    # Return from trap
    sret
