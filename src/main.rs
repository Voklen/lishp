use std::io;

mod errors;
mod executor;
mod parsing;

fn main() {
	let lines = io::stdin().lines();
	for line in lines {
		let parsed = parsing::parse(line);
		executor::executor(parsed);
	}
}
