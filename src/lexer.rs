use crate::{errors::LexerError, throw};
use std::io;

#[derive(Debug)]
pub enum Token {
	FunctionStart,
	FunctionEnd,
	Space,
	String(String),
}

pub fn lex(result_line: Result<String, io::Error>) -> Result<Vec<Token>, LexerError> {
	let line = result_line.unwrap_or_else(|e| throw!("Error reading line: {e}"));
	let mut chars = line.chars();

	let mut tokens = vec![];

	loop {
		let char = match chars.next() {
			Some(res) => res,
			None => break,
		};
		match char {
			'(' => tokens.push(Token::FunctionStart),
			')' => tokens.push(Token::FunctionEnd),
			' ' => tokens.push(Token::Space),
			'\\' => {
				let next_char = match chars.next() {
					Some(res) => res,
					None => return Err(LexerError::TrailingBackslash),
				};
				push_to_string(&mut tokens, next_char);
			}
			char => push_to_string(&mut tokens, char),
		}
	}
	Ok(tokens)
}

fn push_to_string(tokens: &mut Vec<Token>, char: char) {
	match tokens.last_mut() {
		Some(Token::String(string)) => string.push(char),
		_ => tokens.push(Token::String(char.to_string())),
	}
}
