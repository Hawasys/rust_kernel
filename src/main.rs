// disable rust standard library
#![no_std]
// disables Rust runtime init,
#![no_main]

//enable inline assembly
#![feature(asm)]

use core::panic::PanicInfo;

#[path = "./video/vga_buffer.rs"]
pub mod vga_buffer;

pub mod io;
/// The name **must be** `_start`, otherwise the compiler throws away all code as unused. 
/// The name can be changed by passing a different entry symbol as linker argument.

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    let bananas = 'W';
    let testcolor = vga_buffer::ColorCode::new(vga_buffer::ColorForeground::Red, vga_buffer::ColorBackground::Blue);
    vga_buffer::write_char(&bananas, testcolor);
    const AZ: &str = "abc";
    vga_buffer::write_string(AZ, testcolor);
    vga_buffer::write_char(&bananas, testcolor);
    loop {}
}



#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
    }
}


