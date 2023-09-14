use std::{io, process::Command};

mod errors;
mod parsing;

fn main() {
	let lines = io::stdin().lines();
	for line in lines {
		let parsed = parsing::parse(line);
		match parsed {
			Some(func) => {
				let mut child = Command::new(func.name).spawn().unwrap();
				child.wait();
			}
			None => {}
		};
	}
}
