use reedline::{DefaultPrompt, Reedline, Signal};

use crate::{executor::execute, lexer::lex, parser::parse};

mod errors;
mod executor;
mod lexer;
mod parser;

fn main() {
	let mut line_editor = Reedline::create();
	let prompt = DefaultPrompt::default();

	loop {
		let line = match line_editor.read_line(&prompt) {
			Ok(Signal::Success(line)) => line,
			Ok(Signal::CtrlC) | Ok(Signal::CtrlD) => {
				println!("Exiting, have a nice day :)");
				break;
			}
			Err(e) => {
				eprintln!("Error reading line: {e}");
				continue;
			}
		};

		let tokens = match lex(line) {
			Ok(res) => res,
			Err(e) => {
				eprintln!("{e}");
				continue;
			}
		};
		let parsed = match parse(tokens) {
			Ok(res) => res,
			Err(e) => {
				eprintln!("{e}");
				continue;
			}
		};
		execute(parsed);
	}
}
