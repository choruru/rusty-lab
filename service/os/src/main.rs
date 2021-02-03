#![no_std]
#![no_main]
mod vga_buffer;
use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
// The linker looks for a function named `_start` by default.
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
