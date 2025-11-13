use std::str::Chars;

use crate::errors::LexerError;

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
	FunctionStart,
	FunctionEnd,
	String(String),
}

/// Lex a string of lishp into a vector of tokens.
///
/// ```
/// use lishp::lexer::{lex, Token};
///
/// let lexed = lex("ls (echo src)".to_string()).unwrap();
/// assert_eq!(
/// 	lexed,
/// 	vec![
/// 		Token::String("ls".to_string()),
/// 		Token::FunctionStart,
/// 		Token::String("echo".to_string()),
/// 		Token::String("src".to_string()),
/// 		Token::FunctionEnd,
/// 	]
/// );
/// ```
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
			'"' => {
				let quoted_string = handle_quoted_string(&mut chars)?;
				tokens.push(Token::String(quoted_string));
			}
			' ' => continue,
			'\\' => {
				let next_char = match chars.next() {
					Some(res) => res,
					None => return Err(LexerError::TrailingBackslash),
				};
				let mut arg = handle_argument(&mut chars, next_char)?;
				tokens.append(&mut arg);
			}
			char => {
				let mut arg = handle_argument(&mut chars, char)?;
				tokens.append(&mut arg);
			}
		};
	}
	Ok(tokens)
}

fn handle_argument(chars: &mut Chars<'_>, char: char) -> Result<Vec<Token>, LexerError> {
	let mut arg = char.to_string();
	loop {
		let next_char = match chars.next() {
			Some(res) => res,
			// Just exit normally and let the outer function exit cleanly.
			None => break,
		};
		match next_char {
			'(' => return Err(LexerError::OpenParethesisWithinArgument),
			')' => return Ok(vec![Token::String(arg), Token::FunctionEnd]),
			'"' => return Err(LexerError::QuoteWithinArgument),
			'\\' => {
				let next_char = match chars.next() {
					Some(res) => res,
					None => return Err(LexerError::TrailingBackslash),
				};
				arg.push(next_char);
			}
			' ' => break,
			c => arg.push(c),
		};
	}
	Ok(vec![Token::String(arg)])
}

/// Handles a string that starts with a quote.
fn handle_quoted_string(chars: &mut Chars<'_>) -> Result<String, LexerError> {
	let mut string = String::new();
	loop {
		let next_char = match chars.next() {
			Some(res) => res,
			None => return Err(LexerError::UnclosedQuote),
		};
		match next_char {
			'\\' => {
				let next_char = match chars.next() {
					Some(res) => res,
					None => return Err(LexerError::TrailingBackslash),
				};
				string.push(next_char);
			}
			'"' => break,
			c => string.push(c),
		};
	}
	Ok(string)
}
