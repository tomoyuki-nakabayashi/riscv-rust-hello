#![no_std]

static HELLO: &[u8] = b"Hello from Rust!";

#[no_mangle]
pub fn __start_rust() -> ! {
    let uart_16550 = 0x1001_3000  as *mut u8;
    for c in HELLO {
        unsafe {
            *uart_16550 = *c as u8;
        }
    }

    loop {}
}

use core::panic::PanicInfo;
#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    loop{}
}