#![no_main]
#![no_std]

static HELLO: &[u8] = b"Hello from Rust!";

//#[link_section = ".reset.boot"]
#[no_mangle]
pub extern "C" fn __start_rust() -> ! {
    print_hello();

    loop{}
}

fn print_hello() {
    let uart_16550 = 0x1001_3000  as *mut u8;
    for c in HELLO {
        unsafe {
            *uart_16550 = *c as u8;
        }
    }
}

use core::panic::PanicInfo;
#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    loop{}
}

#[no_mangle]
pub extern "C" fn abort() -> ! {
    loop{}
}
