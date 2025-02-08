#![allow(unused_macros)]

pub use ansi_term::Colour::{
	self, Blue, Cyan, Green, Purple, Red, White, Yellow,
};
pub use ansi_term::Style;

// make windows print colors
#[cfg(windows)]
#[ctor::ctor]
fn init() {
	let _ = output_vt100::try_init();
}

#[macro_export]
macro_rules! paint {
	($color:expr, $fmt:literal $($args:tt)*) => (
		if $crate::stdfiles::is_enabled() {
			print!($fmt $($args)*)
		} else {
			print!(concat!("{}", $fmt, "{}"), $color.prefix() $($args)*, $color.suffix())
		}
	)
}
#[macro_export]
macro_rules! paintln {
	($color:expr, $fmt:literal $($args:tt)*) => (
		if $crate::stdfiles::is_enabled() {
			println!($fmt $($args)*)
		} else {
			println!(concat!("{}", $fmt, "{}"), $color.prefix() $($args)*, $color.suffix())
		}
	)
}

#[macro_export]
macro_rules! epaint {
	($color:expr, $fmt:literal $($args:tt)*) => (
		if $crate::stdfiles::is_enabled() {
			eprint!($fmt $($args)*)
		} else {
			eprint!(concat!("{}", $fmt, "{}"), $color.prefix() $($args)*, $color.suffix())
		}
	)
}

#[macro_export]
macro_rules! epaintln {
	($color:expr, $fmt:literal $($args:tt)*) => (
		if $crate::stdfiles::is_enabled() {
			eprintln!($fmt $($args)*)
		} else {
			eprintln!(concat!("{}", $fmt, "{}"), $color.prefix() $($args)*, $color.suffix())
		}
	)
}

#[macro_export]
macro_rules! paint_err {
	($($args:tt)*) => (
		$crate::epaintln!($crate::paint::Red, $($args)*)
	)
}

#[macro_export]
macro_rules! paint_ok {
	($($args:tt)*) => (
		$crate::epaintln!($crate::paint::Green, $($args)*)
	)
}

#[macro_export]
macro_rules! paint_dbg {
	($($args:tt)*) => (
		$crate::epaintln!($crate::paint::White.dimmed(), $($args)*)
	)
}

/// paint action
#[macro_export]
macro_rules! paint_act {
	($($args:tt)*) => (
		$crate::epaintln!($crate::paint::Yellow, $($args)*)
	)
}
