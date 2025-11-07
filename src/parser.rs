use crate::{errors::ParserError, lexer::Token};
use std::vec::IntoIter;

#[derive(Debug, PartialEq)]
pub enum Expression {
	String(String),
	Function(Box<Func>),
}

#[derive(Debug, PartialEq)]
pub struct Func {
	pub name: Expression,
	pub arguments: Vec<Expression>,
}

impl Func {
	fn empty() -> Self {
		return Func {
			name: Expression::String(String::new()),
			arguments: vec![],
		};
	}

	pub fn is_empty(&self) -> bool {
		self.name == Expression::String(String::new()) && self.arguments == vec![]
	}
}

pub fn parse(tokens: Vec<Token>) -> Result<Func, ParserError> {
	if tokens.is_empty() {
		return Ok(Func::empty());
	}
	let mut token_iterator = tokens.into_iter();
	parse_function(&mut token_iterator)
}

fn parse_function(tokens: &mut IntoIter<Token>) -> Result<Func, ParserError> {
	let token = match tokens.next() {
		Some(res) => res,
		None => return Err(ParserError::ExpectedFunctionNameGotEOF),
	};
	let fn_name = match token {
		Token::FunctionStart => Expression::Function(Box::new(parse_function(tokens)?)),
		Token::FunctionEnd => return Ok(Func::empty()),
		Token::Space => return parse_function(tokens), // Continue parsing without the space token.
		Token::String(string) => Expression::String(string),
	};
	let mut args = vec![];

	loop {
		let token = match tokens.next() {
			Some(res) => res,
			None => break,
		};
		let arg = match token {
			Token::FunctionStart => Expression::Function(Box::new(parse_function(tokens)?)),
			Token::FunctionEnd => break,
			Token::Space => continue,
			Token::String(string) => Expression::String(string),
		};
		args.push(arg);
	}

	Ok(Func {
		name: fn_name,
		arguments: args,
	})
}
