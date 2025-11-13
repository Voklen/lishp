use std::{fmt::Display, io};

#[derive(Debug, PartialEq, Eq)]
pub enum LexerError {
	TrailingBackslash,
	UnclosedQuote,
	QuoteWithinArgument,
	OpenParethesisWithinArgument,
}

impl Display for LexerError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let message = match self {
    LexerError::TrailingBackslash => "Single backslash at the end of the command.",
    LexerError::UnclosedQuote => "Start of quoted string without end quote.",
    LexerError::QuoteWithinArgument => "Quote found within argument. Either replace it with \\\"  or add a space before it if this is meant as a seperate argument.",
    LexerError::OpenParethesisWithinArgument => "Open parenthesis '(' found within argument. Either replace it with \\(  or add a space before it if this is meant to be the start of a subcommand.",
};
		write!(f, "Lexer Error: {message}")
	}
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParserError {
	ExpectedFunctionNameGotEOF,
	EndOfFunctionWhileStillTokens,
}

impl Display for ParserError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let message = match self {
			ParserError::ExpectedFunctionNameGotEOF => {
				"Expected a function, but instead got end of command."
			}
			ParserError::EndOfFunctionWhileStillTokens => {
				"End of function but still more text after it. Hint: Do you have too many ')'?"
			}
		};
		write!(f, "Parser Error: {message}")
	}
}

#[derive(Debug)]
pub struct ExecutorError {
	error_type: ExecutorErrorType,
	binary_name: Option<String>,
}

impl ExecutorError {
	pub fn with(mut self, binary_name: String) -> Self {
		self.binary_name = Some(binary_name);
		self
	}
}

impl Display for ExecutorError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let message = match &self.error_type {
			ExecutorErrorType::CommandStart(e) => e,
		};
		match &self.binary_name {
			Some(name) => write!(f, "{name}: {message}"),
			None => write!(f, "{message}"),
		}
	}
}

impl From<io::Error> for ExecutorError {
	fn from(value: io::Error) -> Self {
		let error_type = ExecutorErrorType::CommandStart(value);
		Self {
			error_type,
			binary_name: None,
		}
	}
}

#[derive(Debug)]
pub enum ExecutorErrorType {
	CommandStart(io::Error),
}
