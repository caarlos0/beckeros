// needed because we only write to the VGA buffer, and never read from it
// rust might decide to optimize away the writes.
// volatile prevents that.
use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

#[allow(dead_code)] // shrug
#[derive(Debug, Clone, Copy, PartialEq, Eq)] // enable copy semantics, making it printable and comparable
#[repr(u8)] // u8 because rust doesn't have a u4 type
pub enum Color {
	Black = 0,
	Blue = 1,
	Green = 2,
	Cyan = 3,
	Red = 4,
	Magenta = 5,
	Brown = 6,
	LightGray = 7,
	DarkGray = 8,
	LightBlue = 9,
	LightGreen = 10,
	LightCyan = 11,
	LightRed = 12,
	Pink = 13,
	Yellow = 14,
	White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)] // ensure it has the same mem layout as its single field
struct ColorCode(u8);

impl ColorCode {
	fn new(foreground: Color, background: Color) -> ColorCode {
		ColorCode((background as u8) << 4 | (foreground as u8))
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)] // guarantees field ordering, whatever that means
struct ScreenChar {
	ascii_character: u8,
	color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

// #[repr(transparent)]
struct Buffer {
	chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
	// chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
	column_position: usize,
	color_code: ColorCode,
	buffer: &'static mut Buffer, // static so it lives through the whole execution of the program
}

impl Writer {
	pub fn write_string(&mut self, s: &str) {
		for byte in s.bytes() {
			match byte {
				// printable ascii byte OR a newline
				// ascii only supports code page 437, but rust strings
				// are utf8, that's why this is needed
				0x20..=0x7e | b'\n' => self.write_byte(byte),
				// isn't in the printable ascii range
				// 0xfe is a ■
				_ => self.write_byte(0xfe),
			}
		}
	}

	pub fn write_byte(&mut self, byte: u8) {
		match byte {
			b'\n' => self.new_line(),
			byte => {
				if self.column_position >= BUFFER_WIDTH {
					self.new_line();
				}

				let row = BUFFER_HEIGHT - 1;
				let col = self.column_position;

				let color_code = self.color_code;
				self.buffer.chars[row][col].write(ScreenChar {
					ascii_character: byte,
					color_code,
				});
				self.column_position += 1;
			}
		}
	}

	fn new_line(&mut self) {
		// move all chars one row up
		for row in 1..BUFFER_HEIGHT {
			for col in 0..BUFFER_WIDTH {
				let character = self.buffer.chars[row][col].read();
				self.buffer.chars[row - 1][col].write(character);
			}
		}
		self.clear_row(BUFFER_HEIGHT - 1);
		self.column_position = 0;
	}

	fn clear_row(&mut self, row: usize) {
		let blank = ScreenChar {
			ascii_character: b' ',
			color_code: self.color_code,
		};
		// writes spaces on the entire line
		for col in 0..BUFFER_WIDTH {
			self.buffer.chars[row][col].write(blank);
		}
	}
}

impl fmt::Write for Writer {
	fn write_str(&mut self, s: &str) -> fmt::Result {
		self.write_string(s);
		Ok(())
	}
}

lazy_static! {
	pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
		column_position: 0,
		color_code: ColorCode::new(Color::Green, Color::Black),
		// 0xb8000 is the VGA buffer addr, this converts it into a raw pointer
		// and then into a mutable reference
		buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
	});
}

#[macro_export]
macro_rules! print {
	($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
	() => ($crate::print!("\n"));
	($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
	use core::fmt::Write;
	WRITER.lock().write_fmt(args).unwrap();
}

#[test_case]
fn test_println_simple() {
	println!("test_println_simple output")
}

#[test_case]
fn test_println_output() {
	let s = "some string";
	println!("{}", s);
	for (i, c) in s.chars().enumerate() {
		let screen_char = WRITER.lock().buffer.chars[BUFFER_HEIGHT - 2][i].read();
		assert_eq!(char::from(screen_char.ascii_character), c);
	}
}
