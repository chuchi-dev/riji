#![doc = include_str!("../README.md")]

#[macro_use]
#[doc(hidden)]
pub mod stdfiles;
#[macro_use]
pub mod paint;

mod script;
pub use script::{Error, Result, Script};

mod api;

mod ref_cell;
