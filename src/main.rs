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

mod serial;
use core::panic::PanicInfo;
use serial::{serial::Serial, SystemConsole};

// A panic handler function
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let serial = SystemConsole::init();
    serial.enable();
    serial.write_string("\nBooting the Quokka Kernel...\n\n");
    serial.write_string("[Welcome to Quokka!]\n");

    loop {}
}
