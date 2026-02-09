// kernel/memory/paging.rs
//! RISC-V Sv39 paging implementation
//! 
//! Sv39 is the standard virtual memory scheme for RISC-V 64-bit:
//! - 39-bit virtual addresses
//! - 3-level page table
//! - 4KB pages
//! - Page table entries are 8 bytes

use crate::println;
use crate::memory::frame_allocator::{self, PhysAddr};
use bitflags::bitflags;

/// Page size (4KB)
pub const PAGE_SIZE: usize = 4096;

/// Virtual address wrapper
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VirtAddr(pub usize);

impl VirtAddr {
    pub fn new(addr: usize) -> Self {
        // Ensure canonical address (sign-extend from bit 38)
        let addr = if addr & (1 << 38) != 0 {
            addr | 0xFFFF_FF80_0000_0000
        } else {
            addr & 0x0000_007F_FFFF_FFFF
        };
        VirtAddr(addr)
    }

    pub fn as_usize(&self) -> usize {
        self.0
    }

    /// Get page number
    pub fn page_number(&self) -> usize {
        self.0 / PAGE_SIZE
    }

    /// Get page offset
    pub fn page_offset(&self) -> usize {
        self.0 % PAGE_SIZE
    }

    /// Get level index for Sv39 page table
    pub fn vpn(&self, level: usize) -> usize {
        (self.0 >> (12 + level * 9)) & 0x1FF
    }
}

bitflags! {
    /// Page table entry flags
    pub struct PTEFlags: usize {
        const VALID = 1 << 0;       // V: Valid
        const READ = 1 << 1;        // R: Readable
        const WRITE = 1 << 2;       // W: Writable
        const EXECUTE = 1 << 3;     // X: Executable
        const USER = 1 << 4;        // U: User accessible
        const GLOBAL = 1 << 5;      // G: Global mapping
        const ACCESSED = 1 << 6;    // A: Accessed
        const DIRTY = 1 << 7;       // D: Dirty
    }
}

/// Page table entry (8 bytes)
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PageTableEntry {
    pub bits: usize,
}

impl PageTableEntry {
    pub const fn new() -> Self {
        PageTableEntry { bits: 0 }
    }

    pub fn is_valid(&self) -> bool {
        (self.bits & PTEFlags::VALID.bits()) != 0
    }

    pub fn is_leaf(&self) -> bool {
        (self.bits & (PTEFlags::READ.bits() | PTEFlags::WRITE.bits() | PTEFlags::EXECUTE.bits())) != 0
    }

    pub fn ppn(&self) -> usize {
        (self.bits >> 10) & 0x0FFF_FFFF_FFFF
    }

    pub fn flags(&self) -> PTEFlags {
        PTEFlags::from_bits_truncate(self.bits & 0xFF)
    }

    pub fn set_entry(&mut self, ppn: usize, flags: PTEFlags) {
        self.bits = (ppn << 10) | flags.bits();
    }

    pub fn phys_addr(&self) -> PhysAddr {
        PhysAddr::new(self.ppn() * PAGE_SIZE)
    }
}

/// Page table (512 entries)
#[repr(C, align(4096))]
pub struct PageTable {
    pub entries: [PageTableEntry; 512],
}

impl PageTable {
    pub const fn new() -> Self {
        PageTable {
            entries: [PageTableEntry::new(); 512],
        }
    }

    pub fn zero(&mut self) {
        for entry in self.entries.iter_mut() {
            entry.bits = 0;
        }
    }
}

/// Simple identity mapping for early boot
pub fn init() {
    println!("[PAGING] Initializing Sv39 paging");
    
    // For now, we're running with identity mapping set up by bootloader/firmware
    // In a full implementation, we would:
    // 1. Create a kernel page table
    // 2. Map kernel code/data
    // 3. Map UART and other devices
    // 4. Enable paging by setting satp register
    
    println!("[PAGING] Running with identity mapping");
}

/// Map a virtual page to a physical frame
pub fn map_page(page_table: &mut PageTable, virt: VirtAddr, phys: PhysAddr, flags: PTEFlags) -> Result<(), &'static str> {
    let vpn2 = virt.vpn(2);
    let vpn1 = virt.vpn(1);
    let vpn0 = virt.vpn(0);

    // Level 2
    let entry2 = &mut page_table.entries[vpn2];
    let table1_addr = if !entry2.is_valid() {
        // Allocate new page table
        let frame = frame_allocator::allocate_frame().ok_or("Out of memory")?;
        let ppn = frame.number;
        entry2.set_entry(ppn, PTEFlags::VALID);
        frame.start_address().as_usize()
    } else {
        entry2.phys_addr().as_usize()
    };

    // Level 1
    let table1 = unsafe { &mut *(table1_addr as *mut PageTable) };
    let entry1 = &mut table1.entries[vpn1];
    let table0_addr = if !entry1.is_valid() {
        let frame = frame_allocator::allocate_frame().ok_or("Out of memory")?;
        let ppn = frame.number;
        entry1.set_entry(ppn, PTEFlags::VALID);
        frame.start_address().as_usize()
    } else {
        entry1.phys_addr().as_usize()
    };

    // Level 0 (leaf)
    let table0 = unsafe { &mut *(table0_addr as *mut PageTable) };
    let entry0 = &mut table0.entries[vpn0];
    let ppn = phys.as_usize() / PAGE_SIZE;
    entry0.set_entry(ppn, flags | PTEFlags::VALID);

    Ok(())
}
