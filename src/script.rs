use std::fs::read_to_string;
use std::io;
use std::path::Path;

use rhai::packages::{Package, StandardPackage};
use rhai::{
	Array, CallFnOptions, Engine, EvalAltResult, FuncArgs, ParseError, Scope,
	AST,
};

use crate::stdfiles::{self, STDERR_FILE, STDOUT_FILE};

pub type Result<T> = std::result::Result<T, Error>;

pub type RhaiResult<T> = std::result::Result<T, Box<EvalAltResult>>;

#[derive(Debug)]
pub enum Error {
	Rhai(Box<EvalAltResult>),
	Parse(ParseError),
	Io(io::Error),
}

impl From<Box<EvalAltResult>> for Error {
	fn from(b: Box<EvalAltResult>) -> Self {
		Self::Rhai(b)
	}
}

impl From<ParseError> for Error {
	fn from(e: ParseError) -> Self {
		Self::Parse(e)
	}
}

impl From<io::Error> for Error {
	fn from(e: io::Error) -> Self {
		Self::Io(e)
	}
}

#[derive(Debug)]
pub struct Script {
	engine: Engine,
	scope: Scope<'static>,
	ast: AST,
}

impl Script {
	pub fn new(p: impl AsRef<Path>) -> Result<Self> {
		let engine = new_engine();
		let mut scope = Scope::new();
		let ctn = read_to_string(p)?;
		let ast = engine.compile_with_scope(&scope, &ctn)?;
		engine.eval_ast_with_scope(&mut scope, &ast)?;
		//ast.clear_statements();

		Ok(Self { engine, scope, ast })
	}

	fn call_fn(&mut self, name: &str, args: impl FuncArgs) -> Result<()> {
		let _ = self.engine.call_fn_with_options(
			CallFnOptions::new().eval_ast(false).rewind_scope(false),
			&mut self.scope,
			&self.ast,
			name,
			args,
		)?;

		Ok(())
	}

	pub fn execute(&mut self, cmd: &str, args: Vec<String>) -> Result<()> {
		self.call_fn(cmd, args)
	}

	pub fn execute_capture(&mut self, cmd: &str, args: Vec<String>) -> Output {
		stdfiles::enable();
		let r = self.call_fn(cmd, args);
		stdfiles::disable();

		Output {
			error: r.err(),
			stdout: String::from_utf8_lossy(&STDOUT_FILE.with(|f| f.take()))
				.into_owned(),
			stderr: String::from_utf8_lossy(&STDERR_FILE.with(|f| f.take()))
				.into_owned(),
		}
	}
}

#[derive(Debug)]
pub struct Output {
	pub error: Option<Error>,
	pub stdout: String,
	pub stderr: String,
}

fn new_engine() -> Engine {
	let mut engine = Engine::new_raw();

	engine.register_global_module(StandardPackage::new().as_shared_module());
	engine.set_fail_on_invalid_map_property(true);

	engine
		.on_debug(|s, src, pos| {
			paint_dbg!("{} @ {:?} > {}", src.unwrap_or("unkown"), pos, s)
		})
		.on_print(move |s| {
			println!("{}", s);
		})
		.register_fn("print", print_bool)
		.register_fn("print", print_arr)
		.register_fn("prompt", prompt)
		.register_fn("panic", panic);

	crate::api::cmd::add(&mut engine);
	crate::api::git::add(&mut engine);
	crate::api::fs::add(&mut engine);
	crate::api::regex::add(&mut engine);
	crate::api::other::add(&mut engine);
	crate::api::toml::add(&mut engine);
	crate::api::util::add(&mut engine);

	engine
}

fn print_arr(arr: Array) -> String {
	let mut s = String::new();
	for (i, el) in arr.into_iter().enumerate() {
		if i != 0 {
			s.push('\n');
		}

		match el.into_immutable_string() {
			Ok(i) => s.push_str(i.as_str()),
			Err(e) => {
				s.push_str("error: ");
				s.push_str(e);
			}
		}
	}

	s
}

fn print_bool(b: bool) -> String {
	if b { "true" } else { "false" }.to_string()
}

fn prompt(s: &str) -> bool {
	paint_act!("{}", s);
	println!("y/n");
	let mut input = String::new();
	io::stdin()
		.read_line(&mut input)
		.expect("could not read line");
	let s = input.trim();

	matches!(s, "y" | "Y")
}

fn panic(s: &str) {
	panic!("{}", s);
}
