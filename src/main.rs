#![no_std] // don't link stdlib
#![no_main] // disable rust level entrypoints

mod vga_buffer;

use core::panic::PanicInfo;

#[panic_handler]
// the ! means it is a diverging function, which has not return.
fn panic(info: &PanicInfo) -> ! {
	println!("{}", info);
	loop {}
}

// this is the entry point, as the linker will look for a func called
// `_start` by default
#[no_mangle] // makes the function be really called _start
pub extern "C" fn _start() -> ! {
	println!("Welcome to BeckerOS {}\n\n", "0.0.1");
	println!("This OS does nothing yet and will never be useful at all");

	// panic!("aaaaaaaaa");

	loop {}
}
