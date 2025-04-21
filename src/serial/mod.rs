pub mod serial;
// Copyright (c) 2025 Aeybel Varghese
//
// Provides an API for implementing Serial Devices
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.


#[cfg(feature = "pl011")]
pub mod amba_pl011;

#[cfg(feature = "system_console_pl011")]
pub type SystemConsole = amba_pl011::PL011Uart;
