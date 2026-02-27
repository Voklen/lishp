use std::{ffi::OsString, fs, path::PathBuf};

use lishp::{
	errors::LexerError,
	executor::context::Context,
	lexer::{lex, Token},
	KEYWORDS,
};
use reedline::{Completer, Span, Suggestion};

pub struct LishpCompleter {
	context: Context,
	commands: Vec<String>,
}

impl LishpCompleter {
	pub fn new(context: Context, mut executables: Vec<String>) -> Self {
		executables.extend(KEYWORDS.into_iter().map(String::from));
		executables.sort_unstable();
		Self {
			context,
			commands: executables,
		}
	}

	fn complete_tokens(
		&self,
		tokens: Vec<Token>,
		line: &str,
		pos: usize,
	) -> Vec<reedline::Suggestion> {
		match tokens.last() {
			Some(Token::FunctionStart) => {
				let span = Span {
					start: pos,
					end: pos,
				};
				suggest_commands(&self.commands, span)
			}
			Some(Token::FunctionEnd) => {
				if line.chars().nth(pos - 1) == Some(' ') {
					let span = Span {
						start: pos,
						end: pos,
					};
					return self.suggest_path(span);
				} else {
					// If there is no space after the end of the function, add one.
					let span = Span {
						start: pos,
						end: pos,
					};
					return self
						.suggest_path(span)
						.into_iter()
						.map(|mut s| {
							s.value = format!(" {}", s.value);
							s
						})
						.collect();
				}
			}
			Some(Token::String(string)) => {
				if line.chars().nth(pos - 1) == Some(' ') {
					// If the last character is a space, then this is an argument so return paths.
					let span = Span {
						start: pos,
						end: pos,
					};
					return self.suggest_path(span);
				}
				let span = Span {
					start: pos - string.len(),
					end: pos,
				};
				// Check second-to-last character
				if tokens.len() < 2 {
					// Just return a completion of there's no character before the last one.
					complete_command(string, &self.commands, span)
				} else {
					match tokens.get(tokens.len() - 2) {
						Some(Token::FunctionStart) => {
							complete_command(string, &self.commands, span)
						}
						Some(Token::FunctionEnd) | Some(Token::String(_)) => {
							let span = Span {
								start: pos - string.len(),
								end: pos,
							};
							self.complete_path(string, span)
						}
						None => {
							//NOTE For some reason if this panic is not in a block, rustfmt cannot format the match statment?
							panic!("Error accessing second to last token: This shouldn't be possible as we've just checked tokens has at least 2 items.")
						}
					}
				}
			}
			None => {
				let span = Span { start: 0, end: pos };
				suggest_commands(&self.commands, span)
			}
		}
	}

	fn suggest_path(&self, span: Span) -> Vec<Suggestion> {
		self.complete_path("", span)
	}

	fn complete_path(&self, incomplete_path: &str, span: Span) -> Vec<Suggestion> {
		let mut working_dir = self.context.working_dir.clone();
		// If the start of path is src/main.r then we want to search for files in
		// src/ and then match all the files that start with main.r
		let mut incomplete_dir = String::new();
		if !incomplete_path.is_empty() {
			let mut incomplete_dir_pathbuf = PathBuf::from(incomplete_path);
			if incomplete_path.chars().last() != Some('/') {
				// If the last char isn't a `/` then we want to pop whatever is after the
				// last `/` off so we just have the directory.
				incomplete_dir_pathbuf.pop();
			}
			working_dir.push(&incomplete_dir_pathbuf);
			incomplete_dir = match incomplete_dir_pathbuf.into_os_string().into_string() {
				Ok(res) => {
					if res.is_empty() || res.ends_with('/') {
						res
					} else {
						format!("{res}/")
					}
				}
				Err(_) => return vec![],
			};
		}

		let entries = match fs::read_dir(working_dir) {
			Ok(res) => res,
			// Silently ignore not being able to read directory.
			Err(_) => return vec![],
		};
		let files: Vec<String> = entries
			.flatten()
			.map(|entry| -> Result<String, OsString> {
				let name = entry.file_name().into_string()?;
				Ok(if entry.path().is_dir() {
					format!("{incomplete_dir}{name}/")
				} else {
					format!("{incomplete_dir}{name} ")
				})
			})
			.flatten()
			.collect();
		files
			.iter()
			.filter(|path| path.starts_with(&incomplete_path))
			.map(|path| Suggestion {
				value: path.to_string(),
				description: None,
				style: None,
				extra: None,
				span,
				append_whitespace: false,
				match_indices: None,
			})
			.collect()
	}
}

impl Completer for LishpCompleter {
	fn complete(&mut self, line: &str, pos: usize) -> Vec<reedline::Suggestion> {
		let (first_part, _second_part) = line.split_at(pos);
		match lex(first_part) {
			Ok(tokens) => self.complete_tokens(tokens, first_part, pos),
			Err(LexerError::TrailingBackslash) => vec![
				("\\\\".to_string(), "Backslash character".to_string()),
				("\\ ".to_string(), "Space character".to_string()),
				("\\\"".to_string(), "Double quote character".to_string()),
				("\\(".to_string(), "Open bracket character".to_string()),
				("\\)".to_string(), "Close bracket character".to_string()),
			]
			.into_iter()
			.map(|pair| to_backslash_suggestion(pair, pos))
			.collect(),
			Err(_) => todo!(),
		}
	}
}

fn to_backslash_suggestion(value_description_pair: (String, String), pos: usize) -> Suggestion {
	let (value, description) = value_description_pair;
	let span = Span {
		start: pos - 1,
		end: pos,
	};

	Suggestion {
		value,
		description: Some(description),
		style: None,
		extra: None,
		span,
		append_whitespace: false,
		match_indices: None,
	}
}

fn suggest_commands(commands: &Vec<String>, span: Span) -> Vec<Suggestion> {
	complete_command("", commands, span)
}

fn complete_command(start_of_command: &str, commands: &Vec<String>, span: Span) -> Vec<Suggestion> {
	let to_suggestion = |command: &str| {
		Suggestion {
			value: command.to_string(),
			// TODO Add descriptions for commands.
			description: None,
			style: None,
			extra: None,
			span,
			append_whitespace: true,
			match_indices: None,
		}
	};
	commands
		.iter()
		.filter(|command| command.starts_with(start_of_command))
		.map(|command| to_suggestion(command))
		.collect()
}
