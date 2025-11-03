use std::io::{self, Write};

mod errors;
mod executor;
mod parsing;

fn main() {
	print_prompt();
	let lines = io::stdin().lines();
	for line in lines {
		let parsed = match parsing::parse(line) {
			Some(res) => res,
			None => continue,
		};
		executor::executor(parsed);
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
