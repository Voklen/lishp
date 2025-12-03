use std::fs;

use lishp::{
	errors::LexerError,
	executor::context::Context,
	lexer::{lex, Token},
	KEYWORDS,
};
use reedline::{Completer, Span, Suggestion};

pub struct LishpCompleter {
	commands: Vec<String>,
}

impl LishpCompleter {
	pub fn new(mut executables: Vec<String>) -> Self {
		executables.extend(KEYWORDS.into_iter().map(String::from));
		executables.sort_unstable();
		Self {
			commands: executables,
		}
	}
}

impl Completer for LishpCompleter {
	fn complete(&mut self, line: &str, pos: usize) -> Vec<reedline::Suggestion> {
		let (first_part, _second_part) = line.split_at(pos);
		match lex(first_part) {
			Ok(tokens) => match tokens.last() {
				Some(Token::FunctionStart) => {
					let span = Span {
						start: pos,
						end: pos,
					};
					suggest_commands(&self.commands, span)
				}
				Some(Token::FunctionEnd) => {
					if first_part.get(pos - 1..pos) == Some(" ") {
						return vec![]; // TODO path completions
					} else {
						return vec![]; // TODO path completion with space
					}
				}
				Some(Token::String(string)) => {
					let span = Span {
						start: pos - string.len(),
						end: pos,
					};
					if first_part.get(pos - 1..pos) == Some(" ") {
						return vec![]; // TODO path completions
					}
					// Check second-to-last character
					if tokens.len() < 2 {
						// Just return a completion of there's no character before the last one.
						complete_command(string, &self.commands, span)
					} else {
						match tokens.get(tokens.len() - 2) {
							Some(Token::FunctionStart) => complete_command(string, &self.commands, span),
							Some(Token::FunctionEnd) | Some(Token::String(_)) => vec![], // TODO path completions
							None => panic!("Error accessing second to last token: This shouldn't be possible as we've just checked tokens has at least 2 items."),
						}
					}
				}
				None => {
					let span = Span { start: 0, end: pos };
					suggest_commands(&self.commands, span)
				}
			},
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
		}
	};
	commands
		.iter()
		.filter(|command| command.starts_with(start_of_command))
		.map(|command| to_suggestion(command))
		.collect()
}

// TODO For these next two function to be usable, we need to be able to access
// Context in the complete method.

#[allow(dead_code)]
fn suggest_path(span: Span, context: &Context) -> Vec<Suggestion> {
	complete_path("", span, context)
}

fn complete_path(start_of_path: &str, span: Span, context: &Context) -> Vec<Suggestion> {
	let to_suggestion = |path: &str| Suggestion {
		value: path.to_string(),
		description: None,
		style: None,
		extra: None,
		span,
		append_whitespace: true,
	};
	let entries = match fs::read_dir(&context.working_dir) {
		Ok(res) => res,
		// Silently ignore not being able to read directory.
		Err(_) => return vec![],
	};
	let files: Vec<String> = entries
		.flatten()
		.map(|entry| entry.path().into_os_string())
		.map(|os_string| os_string.into_string())
		.flatten()
		.collect();
	files
		.iter()
		.filter(|command| command.starts_with(start_of_path))
		.map(|command| to_suggestion(command))
		.collect()
}
