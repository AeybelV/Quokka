#![no_std]
#![no_main]
use core::panic::PanicInfo;
use volatile_register::{RO, RW, WO};

#[repr(C)]
pub struct PL011Uart {
    pub dr: RW<u32>,             // 0x000: Data Register
    pub rsr_ecr: RW<u32>,        // 0x004: Receive Status/Error Clear Register
    _reserved1: [u8; 0x10],      // 0x008 - 0x017: Reserved
    pub fr: RO<u32>,             // 0x018: Flag Register
    _reserved2: [u8; 0x4],       // 0x01C - 0x01F: Reserved
    pub ilpr: RW<u32>,           // 0x020: IrDA Low-Power Register
    pub ibrd: RW<u32>,           // 0x024: Integer Baud Rate Register
    pub fbrd: RW<u32>,           // 0x028: Fractional Baud Rate Register
    pub lcr_h: RW<u32>,          // 0x02C: Line Control Register
    pub cr: RW<u32>,             // 0x030: Control Register
    pub ifls: RW<u32>,           // 0x034: Interrupt FIFO Level Select Register
    pub imsc: RW<u32>,           // 0x038: Interrupt Mask Set/Clear Register
    pub ris: RO<u32>,            // 0x03C: Raw Interrupt Status Register
    pub mis: RO<u32>,            // 0x040: Masked Interrupt Status Register
    pub icr: WO<u32>,            // 0x044: Interrupt Clear Register
    pub dmacr: RW<u32>,          // 0x048: DMA Control Register
    _reserved3: [u8; 0xF90],     // 0x04C - 0xFDF: Reserved
    pub periph_id: [RO<u32>; 4], // 0xFE0 - 0xFEC: Peripheral ID Registers
    pub pcell_id: [RO<u32>; 4],  // 0xFF0 - 0xFFC: PrimeCell ID Registers
}

/// Base address of the UART0 register for the qemu-virt.
const PL011_UART_BASE: u32 = 0x09000000;

/// Masks for the UARTFR (Flag Register)
pub mod fr_masks {
    pub const TXFF: u32 = 1 << 5; // Transmit FIFO Full
    pub const RXFE: u32 = 1 << 4; // Receive FIFO Empty
    pub const BUSY: u32 = 1 << 3; // UART Busy
}

/// Masks for the UARTLCR_H (Line Control Register)
pub mod lcr_h_masks {
    pub const WLEN_8: u32 = 0b11 << 5; // 8-bit word length
    pub const FEN: u32 = 1 << 4; // Enable FIFOs
}

/// Masks for the UARTCR (Control Register)
pub mod cr_masks {
    pub const UARTEN: u32 = 1 << 0; // UART Enable
    pub const TXE: u32 = 1 << 8; // Transmit Enable
    pub const RXE: u32 = 1 << 9; // Receive Enable
}

impl PL011Uart {
    /// Get a reference to the UART registers
    pub fn instance() -> &'static Self {
        unsafe { &*(PL011_UART_BASE as *const PL011Uart) }
    }

    /// Write a string slice to the UART
    pub fn transmit_string(&self, s: &str) {
        for c in s.chars() {
            self.transmit(c as u8);
        }
    }

    /// Transmit a character
    pub fn transmit(&self, c: u8) {
        unsafe {
            // Wait until the UART is not full
            while self.fr.read() & fr_masks::TXFF != 0 {}
            self.dr.write(c as u32);
        }
    }

    /// Receive a character
    pub fn receive(&self) -> u8 {
        // Wait until the UART is not empty
        while self.fr.read() & fr_masks::RXFE != 0 {}
        self.dr.read() as u8
    }

    /// Enable UART
    pub fn enable_uart(&self) {
        unsafe {
            self.cr
                .write(cr_masks::UARTEN | cr_masks::TXE | cr_masks::RXE);
        }
    }

    /// Disable UART
    pub fn disable_uart(&self) {
        unsafe {
            self.cr.write(0);
        }
    }
}

// A panic handler function
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    PL011Uart::instance().transmit_string("QUOKKA PANIC");
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    PL011Uart::instance().transmit_string("Booting kernel");
    PL011Uart::instance().transmit_string("[Welcome to Quokka]");
    loop {}
}
