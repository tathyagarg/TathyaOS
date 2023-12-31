#![no_std]
#![no_main]

mod vga_buf;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    panic!("I COULDNT FIND ANY TURTLES");
    println!("Never got a chance to formally introduce myself, did I?");
    loop {}
}
