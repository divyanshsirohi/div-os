/* RISC-V Linker Script for div-os kernel
 * Target: QEMU virt machine
 * Memory layout:
 *   - DRAM starts at 0x80000000
 *   - Kernel is loaded at 0x80200000 (standard QEMU virt location)
 */

OUTPUT_ARCH(riscv)
ENTRY(_start)

/* Memory regions for QEMU virt machine */
MEMORY
{
    /* QEMU virt machine has RAM starting at 0x80000000 
     * We load kernel at 0x80200000 to leave space for bootloader/SBI
     */
  RAM : ORIGIN = 0x80000000, LENGTH = 128M
}

SECTIONS
{
    . = ORIGIN(RAM);
    
    /* Kernel start address */
    PROVIDE(_kernel_start = .);
    
    /* Text section - executable code */
    .text : ALIGN(4K) {
        PROVIDE(_text_start = .);
        *(.text.boot)  /* Boot code first */
        *(.text .text.*)
        PROVIDE(_text_end = .);
    } > RAM
    
    /* Read-only data section */
    .rodata : ALIGN(4K) {
        PROVIDE(_rodata_start = .);
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
        PROVIDE(_rodata_end = .);
    } > RAM
    
    /* Data section - initialized data */
    .data : ALIGN(4K) {
        PROVIDE(_data_start = .);
        . = ALIGN(8);
        PROVIDE(__global_pointer$ = . + 0x800);  /* Global pointer for relaxation */
        *(.data .data.*)
        *(.sdata .sdata.*)
        PROVIDE(_data_end = .);
    } > RAM
    
    /* BSS section - uninitialized data */
    .bss : ALIGN(4K) {
        PROVIDE(_bss_start = .);
        *(.bss .bss.*)
        *(.sbss .sbss.*)
        *(COMMON)
        PROVIDE(_bss_end = .);
    } > RAM
    
    /* Stack section (grows downward) */
    .stack (NOLOAD) : ALIGN(4K) {
        PROVIDE(_stack_start = .);
        . += 512K;  /* 512KB stack */
        PROVIDE(_stack_end = .);
    } > RAM
    
    /* Heap section */
    .heap (NOLOAD) : ALIGN(4K) {
        PROVIDE(_heap_start = .);
        . += 16M;  /* 16MB heap */
        PROVIDE(_heap_end = .);
    } > RAM
    
    /* Kernel end marker */
    PROVIDE(_kernel_end = .);
    
    /* Discard unnecessary sections */
    /DISCARD/ : {
        *(.comment)
        *(.eh_frame)
        *(.note*)
    }
}
