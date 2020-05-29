#![no_std] // don't link stdlib
#![no_main] // disable rust level entrypoints

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	// the ! means it is a diverging function, which has not return.
	loop {}
}

#[no_mangle] // makes the function be really called _start
pub extern "C" fn _start() -> ! {
	// this is the entry point, as the linker will look for a func called
	// `_start` by default
	loop {}
}
