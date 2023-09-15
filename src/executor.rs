use std::process::Command;

use crate::{parsing::Func, throw, unwrap_or_return};

pub fn executor(option_func: Option<Func>) {
	let func = unwrap_or_return!(option_func);

	let startup_result = Command::new(func.name).args(func.arguments).spawn();
	let mut child = startup_result.unwrap_or_else(|e| throw!("Could not start command: {e}"));
	let exit_code = child.wait();
	exit_code.unwrap_or_else(|e| throw!("Exited with code: {e}"));
}
