// Copyright (c) 2025 Aeybel Varghese
//
// amba_pl011.rs
//
// Minimal SerialDevice driver for the AMBA PL011 UART Peripheral
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::ktest::{KernelTest, KernelTestResult};

use super::SerialDevice;
use volatile_register::{RO, RW, WO};

// ========== PL011 Device Driver ==========

#[repr(C)]
/// PrimeCell UART (PL011) AMBA Registers
///
/// * `dr`: Data Register
/// * `rsr_ecr`: Receive Status/Error Clear Register
/// * `_reserved1`:  Reserved
/// * `fr`:  Flag Register
/// * `_reserved2`:  Reserved
/// * `ilpr`: IrDA Low-Power Register
/// * `ibrd`: Integer Baud Rate Register
/// * `fbrd`: Fractional Baud Rate Register
/// * `lcr_h`: Line Control Register
/// * `cr`:  Control Register
/// * `ifls`: Interrupt FIFO Level Select Register
/// * `imsc`: Interrupt Mask Set/Clear Register
/// * `ris`:  Raw Interrupt Status Register
/// * `mis`:  Masked Interrupt Status Register
/// * `icr`: Interrupt Clear Register
/// * `dmacr`: DMA Control Register
/// * `_reserved3`:  Reserved
/// * `periph_id`:  Peripheral ID Registers
/// * `pcell_id`: PrimeCell ID Registers
pub struct PL011Regs {
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
// TODO: [Serial:PL011] This must be removed to be hardware agnostic
pub const PL011_UART_BASE: u32 = 0x09000000;

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

/// PL011 Device
///
/// * `regs`: MIO Device Registers
pub struct PL011 {
    regs: &'static mut PL011Regs,
}

impl PL011 {
    /// Instatiates a MIO PL011 at the specified MIO base address for the Peripheral
    ///
    /// * `base_addr`: MIO Base address of the peripheral
    pub fn new(base_addr: u32) -> Self {
        Self {
            regs: unsafe { &mut *(base_addr as *mut PL011Regs) },
        }
    }

    /// Transmits a byte
    ///
    /// * `c`: Character to write
    fn pl011_write_byte(&self, c: u8) {
        unsafe {
            // Wait until the UART is not full
            while self.regs.fr.read() & fr_masks::TXFF != 0 {}
            self.regs.dr.write(c as u32);
        }
    }

    /// Receive a character
    fn pl011_read_byte(&self) -> u8 {
        // Wait until the UART is not empty
        while self.regs.fr.read() & fr_masks::RXFE != 0 {}
        self.regs.dr.read() as u8
    }

    /// Enable UART
    fn pl011_enable(&self) {
        unsafe {
            self.regs
                .cr
                .write(cr_masks::UARTEN | cr_masks::TXE | cr_masks::RXE);
        }
    }

    /// Disable UART
    fn pl011_disable(&self) {
        unsafe {
            self.regs.cr.write(0);
        }
    }
}

// ========== ========== ==========

// ========== Serial Device ==========

// Implements the SerialDevice trait to expose it to the SerialDevice subsystem
impl SerialDevice for PL011 {
    fn init(&self) {
        // Does nothing for now
    }
    fn write_byte(&self, byte: u8) {
        self.pl011_write_byte(byte);
    }
    fn read_byte(&self) -> Option<u8> {
        Some(self.pl011_read_byte())
    }
    fn enable(&self) {
        self.pl011_enable();
    }
    fn disable(&self) {
        self.pl011_disable();
    }
}

// ========== ========== ==========

// ========== Tests ==========

pub struct PL011WriteByteTest;

impl KernelTest for PL011WriteByteTest {
    fn name(&self) -> &'static str {
        "Tests the PL011"
    }

    fn run(&self) -> KernelTestResult {
        let uart0 = PL011::new(PL011_UART_BASE);
        uart0.write_byte('a' as u8);
        KernelTestResult::pass(self.name())
    }
}

// ========== ========== ==========
