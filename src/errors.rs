use std::{fmt::Display, io};

/// Print the error and stop the program, if running in debug mode it will panic
/// but in release builds it will print `<program name>: <error>` to stderr and
/// exit with exit code 1.
/// The macro will also format the string within it
/// ```
/// match value {
/// 	Ok(x) => x,
/// 	Err(err) => {
/// 		throw!("Error reading line: {err}")
/// 	}
/// }
/// ```
#[macro_export]
macro_rules! throw {
    ($($message:tt)*) => {{
		use	crate::errors::throw_error_fuction;
        let res = format!($($message)*);
        throw_error_fuction(res)
    }}
}

pub fn throw_error_fuction(error_message: String) -> ! {
	#[cfg(not(debug_assertions))]
	exit_production(error_message);
	#[cfg(debug_assertions)]
	panic!("{error_message}");
}

#[allow(dead_code)]
fn exit_production(error_message: String) -> ! {
	let program_name = env!("CARGO_PKG_NAME");
	eprintln!("{program_name}: {error_message}");
	std::process::exit(1);
}

#[derive(Debug)]
pub enum LexerError {
	TrailingBackslash,
}

impl Display for LexerError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let message = match self {
			LexerError::TrailingBackslash => "Single backslash at the end of the command.",
		};
		write!(f, "Lexer Error: {message}")
	}
}

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
