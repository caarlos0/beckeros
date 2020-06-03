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
	println!("Welcome to BeckerOS {}\n\n", "0.0.1");
	println!("This OS does nothing yet and will never be useful at all");

	#[cfg(test)] // conditional compilation
	test_main();

	loop {}
}
