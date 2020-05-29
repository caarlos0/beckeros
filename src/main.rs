#![no_std] // don't link stdlib
#![no_main] // disable rust level entrypoints

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	// the ! means it is a diverging function, which has not return.
	loop {}
}

static HELLO: &[u8] = b"Hello World!";

#[no_mangle] // makes the function be really called _start
pub extern "C" fn _start() -> ! {
	// this is the entry point, as the linker will look for a func called
	// `_start` by default

	// 0xb8000 is the VGA buffer addr, this converts it into a raw pointer
	let vga_buffer = 0xb8000 as *mut u8;

	for (i, &byte) in HELLO.iter().enumerate() {
		unsafe {
			*vga_buffer.offset(i as isize * 2) = byte;
			*vga_buffer.offset(i as isize * 2 + 1) = 0xb;
		}
	}

	loop {}
}
