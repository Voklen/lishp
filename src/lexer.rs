use std::str::Chars;

use crate::errors::LexerError;

#[derive(Debug)]
pub enum Token {
	FunctionStart,
	FunctionEnd,
	String(String),
}

pub fn lex(line: String) -> Result<Vec<Token>, LexerError> {
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
			' ' => handle_space(&mut tokens, &mut chars)?,
			'\\' => {
				let next_char = match chars.next() {
					Some(res) => res,
					None => return Err(LexerError::TrailingBackslash),
				};
				push_to_string(&mut tokens, next_char);
			}
			char => push_to_string(&mut tokens, char),
		};
	}
	Ok(tokens)
}

fn push_to_string(tokens: &mut Vec<Token>, char: char) {
	match tokens.last_mut() {
		Some(Token::String(string)) => string.push(char),
		_ => tokens.push(Token::String(char.to_string())),
	}
}

/// Handles the space and handles the next char except adding it to a new
/// string token instead of pushing.
fn handle_space(tokens: &mut Vec<Token>, chars: &mut Chars<'_>) -> Result<(), LexerError> {
	let next_char = match chars.next() {
		Some(res) => res,
		// Just return normally and let the next iteration exit cleanly.
		None => return Ok(()),
	};
	match next_char {
		'(' => tokens.push(Token::FunctionStart),
		')' => tokens.push(Token::FunctionEnd),
		'\\' => {
			let next_char = match chars.next() {
				Some(res) => res,
				None => return Err(LexerError::TrailingBackslash),
			};
			tokens.push(Token::String(next_char.to_string()));
		}
		' ' => return handle_space(tokens, chars),
		c => tokens.push(Token::String(c.to_string())),
	};
	Ok(())
}
