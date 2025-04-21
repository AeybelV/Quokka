// Copyright (c) 2025 Aeybel Varghese
//
// Provides an API for implementing Serial Devices
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

/// Serial Device Trait
/// All Serial Device Drivers implement this trait
#[cfg(feature = "serialdevice")]
pub trait SerialDevice {
    /// Initializes the device
    fn init(&self);

    /// Transmits a byte
    ///
    /// * `byte`: byte to transmit
    fn write_byte(&self, byte: u8);

    /// Reads a byte from the device
    fn read_byte(&self) -> Option<u8>;

    /// Enables the device
    fn enable(&self);

    /// Disables the device
    fn disable(&self);
}

#[cfg(feature = "pl011")]
pub mod amba_pl011;
