use std::fmt::Display;

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
	ExpectedArgumentGotEOF,
	FunctionEndOutsideFunction,
}

impl Display for ParserError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let message = match self {
			ParserError::ExpectedFunctionNameGotEOF => {
				"Expected a function, but instead got end of command."
			}
			ParserError::ExpectedArgumentGotEOF => {
				"Expected an argument, but instead got end of command."
			}
			ParserError::FunctionEndOutsideFunction => {
				"Found an end of function outside a function."
			}
		};
		write!(f, "Parser Error: {message}")
	}
}
