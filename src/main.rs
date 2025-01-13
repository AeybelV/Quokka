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
