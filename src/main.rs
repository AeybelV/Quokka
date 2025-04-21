// Copyright (c) 2025 Aeybel Varghese
//
// main.rs
//
// Kernel entry point
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![no_std]
#![no_main]

mod ktest;
mod serial;
use core::panic::PanicInfo;
use ktest::{run_all_ktests, KernelTest, KernelTestRegistry};
use serial::{
    amba_pl011::{PL011WriteByteTest, PL011, PL011_UART_BASE},
    SerialDevice,
};

// A panic handler function
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let uart_test = PL011WriteByteTest;
    let result = uart_test.run();
    loop {}
}
