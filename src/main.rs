use std::io::{self, Write};

use crate::{lexer::lex, parser::parse};

mod errors;
mod executor;
mod lexer;
mod parser;

fn main() {
	print_prompt();
	let lines = io::stdin().lines();
	for line in lines {
		let tokens = match lex(line) {
			Ok(res) => res,
			Err(e) => {
				eprintln!("{e}");
				return;
			}
		};
		let parsed = match parse(tokens) {
			Ok(res) => res,
			Err(e) => {
				eprintln!("{e}");
				return;
			}
		};
		executor::execute(parsed);
		print_prompt();
	}
}

fn print_prompt() {
	print!("> ");
	match io::stdout().flush() {
		Ok(()) => {}
		Err(e) => throw!("Error flushing stdout: {e}"),
	}
}
