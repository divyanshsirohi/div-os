// kernel/drivers/uart.rs
//! UART driver for RISC-V (NS16550A compatible)
//! 
//! This driver provides character output through the UART serial port.
//! On QEMU virt machine, UART0 is memory-mapped at 0x10000000.
//! 
//! The NS16550A UART has several memory-mapped registers:
//! - THR (Transmit Holding Register): Write data to transmit
//! - LSR (Line Status Register): Check if transmitter is ready
//! - IER (Interrupt Enable Register): Configure interrupts
//! - LCR (Line Control Register): Configure data format

use core::fmt;
use spin::Mutex;

/// Base address of UART0 on QEMU virt machine
const UART_BASE: usize = 0x1000_0000;

/// UART register offsets
const UART_THR: usize = 0;  // Transmit Holding Register
const UART_LSR: usize = 5;  // Line Status Register
const UART_IER: usize = 1;  // Interrupt Enable Register
const UART_LCR: usize = 3;  // Line Control Register

/// Line Status Register bit for transmitter empty
const LSR_TX_IDLE: u8 = 1 << 5;

/// UART device structure
pub struct Uart {
    base_addr: usize,
}

impl Uart {
    /// Create a new UART instance
    pub const fn new(base_addr: usize) -> Self {
        Uart { base_addr }
    }

    /// Initialize the UART
    pub fn init(&mut self) {
        unsafe {
            // Disable all interrupts
            self.write_reg(UART_IER, 0x00);
            
            // Set word length to 8 bits, no parity, one stop bit
            self.write_reg(UART_LCR, 0x03);
        }
    }

    /// Write a byte to UART register
    unsafe fn write_reg(&mut self, offset: usize, value: u8) {
        let ptr = (self.base_addr + offset) as *mut u8;
        ptr.write_volatile(value);
    }

    /// Read a byte from UART register
    unsafe fn read_reg(&self, offset: usize) -> u8 {
        let ptr = (self.base_addr + offset) as *const u8;
        ptr.read_volatile()
    }

    /// Check if transmitter is ready
    fn is_tx_ready(&self) -> bool {
        unsafe {
            (self.read_reg(UART_LSR) & LSR_TX_IDLE) != 0
        }
    }

    /// Write a single byte to UART
    pub fn put_byte(&mut self, byte: u8) {
        // Wait until transmitter is ready
        while !self.is_tx_ready() {
            core::hint::spin_loop();
        }
        
        unsafe {
            self.write_reg(UART_THR, byte);
        }
    }

    /// Write a string to UART
    pub fn put_str(&mut self, s: &str) {
        for byte in s.bytes() {
            // Convert newline to carriage return + newline
            if byte == b'\n' {
                self.put_byte(b'\r');
            }
            self.put_byte(byte);
        }
    }
}

impl fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.put_str(s);
        Ok(())
    }
}

/// Global UART instance
pub static UART: Mutex<Uart> = Mutex::new(Uart::new(UART_BASE));

/// Initialize the global UART
pub fn init() {
    UART.lock().init();
}

/// Print function for internal use by macros
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    UART.lock().write_fmt(args).unwrap();
}

/// Print macro for kernel output
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::drivers::uart::_print(format_args!($($arg)*)));
}

/// Println macro for kernel output
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
