use crate::throw;
use std::{ffi::OsStr, io};

pub fn parse(result_line: Result<String, io::Error>) -> Option<Func> {
	let line = result_line.unwrap_or_else(|e| throw!("Error reading line: {e}"));
	let mut tokens = line.split_whitespace();
	let name = tokens.next()?.to_owned();
	let arguments: Vec<Expression> = tokens
		.map(|str| Expression::String(str.to_owned()))
		.collect();
	Some(Func { name, arguments })
}

pub enum Expression {
	String(String),
	Function(Func),
}

impl Expression {
	fn to_string(&self) -> &str {
		match self {
			Self::String(str) => str,
			Self::Function(func) => func.to_string(),
		}
	}
}

impl AsRef<OsStr> for Expression {
	fn as_ref(&self) -> &OsStr {
		OsStr::new(self.to_string())
	}
}

pub struct Func {
	pub name: String,
	pub arguments: Vec<Expression>,
}

impl Func {
	fn to_string(&self) -> &str {
		format!("{}", self.name);
		"hello"
	}
}
