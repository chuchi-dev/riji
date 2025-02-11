use std::{
	cell::{Cell, RefCell, RefMut},
	io::{Cursor, Write},
};

thread_local! {
	pub static ENABLED: Cell<bool> = Cell::new(false);
	pub static STDOUT_FILE: StdFile = StdFile::new();
	pub static STDERR_FILE: StdFile = StdFile::new();
}

pub fn is_enabled() -> bool {
	ENABLED.with(|e| e.get())
}

pub fn enable() {
	ENABLED.set(true);
}

pub fn disable() {
	ENABLED.set(false);
}

#[derive(Debug)]
pub struct StdFile {
	file: RefCell<Cursor<Vec<u8>>>,
}

impl StdFile {
	pub fn new() -> Self {
		Self {
			file: RefCell::new(Cursor::new(Vec::new())),
		}
	}

	pub fn borrow_mut(&self) -> RefMut<Cursor<Vec<u8>>> {
		self.file.borrow_mut()
	}

	pub fn take(&self) -> Vec<u8> {
		self.file.take().into_inner()
	}

	pub fn write(&self, buf: &[u8]) {
		self.file.borrow_mut().write_all(buf).unwrap();
	}
}

#[macro_export]
macro_rules! eprint {
	($($args:tt)*) => {
		if $crate::stdfiles::is_enabled() {
			$crate::stdfiles::STDERR_FILE.with(|f| {
				use std::io::Write;
				write!(f.borrow_mut(), $($args)*).unwrap();
			});
		} else {
			::std::eprint!($($args)*);
		}
	}
}

#[macro_export]
macro_rules! eprintln {
	($($args:tt)*) => {
		if $crate::stdfiles::is_enabled() {
			$crate::stdfiles::STDERR_FILE.with(|f| {
				use std::io::Write;
				writeln!(f.borrow_mut(), $($args)*).unwrap();
			});
		} else {
			::std::eprintln!($($args)*);
		}
	}
}

#[macro_export]
macro_rules! print {
	($($args:tt)*) => {
		if $crate::stdfiles::is_enabled() {
			$crate::stdfiles::STDOUT_FILE.with(|f| {
				use std::io::Write;
				write!(f.borrow_mut(), $($args)*).unwrap();
			});
		} else {
			::std::print!($($args)*);
		}
	}
}

#[macro_export]
macro_rules! println {
	($($args:tt)*) => {
		if $crate::stdfiles::is_enabled() {
			$crate::stdfiles::STDOUT_FILE.with(|f| {
				use std::io::Write;
				writeln!(f.borrow_mut(), $($args)*).unwrap();
			});
		} else {
			::std::println!($($args)*);
		}
	}
}
