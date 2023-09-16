use crate::throw;
use std::{io, iter::Peekable, str::Chars};

pub fn parse(result_line: Result<String, io::Error>) -> Option<Func> {
	let line = result_line.unwrap_or_else(|e| throw!("Error reading line: {e}"));
	let mut chars = line.chars().peekable();
	let name = next_word(&mut chars);

	let mut args = vec![];
	loop {
		let result = get_next_expression(&mut chars);
		match result {
			Some(expr) => args.push(expr),
			None => break,
		}
	}
	println!("name: {} args: {:?}", name, args);
	Some(Func {
		name: name.to_owned(),
		arguments: args,
	})
}

fn next_word(chars: &mut Peekable<Chars<'_>>) -> String {
	let reference = chars.by_ref();
	reference.take_while(|char| !char.is_whitespace()).collect()
}

/// Returns None if end of function
fn get_next_expression(chars: &mut Peekable<Chars<'_>>) -> Option<Expression> {
	let first_char = chars.peek()?;
	match first_char {
		'(' => Some(parse_function(chars)),
		')' => {
			chars.next();
			None
		}
		_ => Some(Expression::String(next_word(chars))),
	}
}

fn parse_function(chars: &mut Peekable<Chars<'_>>) -> Expression {
	chars.next();
	let name = next_word(chars);
	let mut args = vec![];

	loop {
		let result = get_next_expression(chars);
		match result {
			Some(expr) => args.push(expr),
			None => break,
		}
	}

	let as_func = Func {
		name: name.to_owned(),
		arguments: args,
	};
	Expression::Function(as_func)
}

#[derive(Debug)]
pub enum Expression {
	String(String),
	Function(Func),
}

#[derive(Debug)]
pub struct Func {
	pub name: String,
	pub arguments: Vec<Expression>,
}
