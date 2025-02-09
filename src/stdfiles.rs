use std::{
	io::{Cursor, Write},
	sync::{
		atomic::{AtomicBool, Ordering},
		Mutex, MutexGuard,
	},
};

use lazy_static::lazy_static;

lazy_static! {
	pub static ref ENABLED: AtomicBool = AtomicBool::new(false);
	pub static ref STDOUT_FILE: StdFile = StdFile::new();
	pub static ref STDERR_FILE: StdFile = StdFile::new();
}

pub fn is_enabled() -> bool {
	ENABLED.load(Ordering::Relaxed)
}

pub fn enable() {
	ENABLED.store(true, Ordering::Relaxed);
}

pub fn disable() {
	ENABLED.store(false, Ordering::Relaxed);
}

#[derive(Debug)]
pub struct StdFile {
	file: Mutex<Cursor<Vec<u8>>>,
}

impl StdFile {
	pub fn new() -> Self {
		Self {
			file: Mutex::new(Cursor::new(Vec::new())),
		}
	}

	pub fn lock(&self) -> MutexGuard<Cursor<Vec<u8>>> {
		self.file.lock().unwrap()
	}

	pub fn take(&self) -> Vec<u8> {
		let mut file = self.file.lock().unwrap();
		let mut buf = Vec::new();
		std::mem::swap(&mut buf, &mut file.get_mut());
		buf
	}

	pub fn write(&self, buf: &[u8]) {
		let mut file = self.file.lock().unwrap();
		file.write_all(buf).unwrap();
	}
}

#[macro_export]
macro_rules! eprint {
	($($args:tt)*) => {
		if $crate::stdfiles::is_enabled() {
			use std::io::Write;
			let mut file = $crate::stdfiles::STDERR_FILE.lock();
			write!(file, $($args)*).unwrap();
		} else {
			::std::eprint!($($args)*);
		}
	}
}

#[macro_export]
macro_rules! eprintln {
	($($args:tt)*) => {
		if $crate::stdfiles::is_enabled() {
			use std::io::Write;
			let mut file = $crate::stdfiles::STDERR_FILE.lock();
			writeln!(file, $($args)*).unwrap();
		} else {
			::std::eprintln!($($args)*);
		}
	}
}

#[macro_export]
macro_rules! print {
	($($args:tt)*) => {
		if $crate::stdfiles::is_enabled() {
			use std::io::Write;
			let mut file = $crate::stdfiles::STDOUT_FILE.lock();
			write!(file, $($args)*).unwrap();
		} else {
			::std::print!($($args)*);
		}
	}
}

#[macro_export]
macro_rules! println {
	($($args:tt)*) => {
		if $crate::stdfiles::is_enabled() {
			use std::io::Write;
			let mut file = $crate::stdfiles::STDOUT_FILE.lock();
			writeln!(file, $($args)*).unwrap();
		} else {
			::std::println!($($args)*);
		}
	}
}
