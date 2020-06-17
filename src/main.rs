#![no_std] // don't link stdlib
#![no_main] // disable rust level entrypoints
#![feature(custom_test_frameworks)]
#![test_runner(beckeros::test_runner)]
#![reexport_test_harness_main = "test_main"] // renaames the main tedst main fnc

use beckeros::println;
use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
// the ! means it is a diverging function, which has not return.
fn panic(info: &PanicInfo) -> ! {
	println!("{}", info);
	loop {}
}

#[cfg(test)]
#[panic_handler]
// the ! means it is a diverging function, which has not return.
fn panic(info: &PanicInfo) -> ! {
	beckeros::test_panic_handler(info)
}

// this is the entry point, as the linker will look for a func called
// `_start` by default
#[no_mangle] // makes the function be really called _start
pub extern "C" fn _start() -> ! {
	beckeros::init();

	println!("Welcome to BeckerOS {}\n\n", "0.0.1");

	x86_64::instructions::interrupts::int3();

	unsafe {
		// 0xdeadbeef is an invalid mem addr
		// this will trigger a page fault!
		// we not yet have a page fault interrupt handler, so this will trigger
		// a double fault.
		*(0xdeadbeef as *mut u64) = 42;
	};

	println!("This OS does nothing yet and will never be useful at all");

	#[cfg(test)] // conditional compilation
	test_main();

	loop {}
}
