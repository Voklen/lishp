use std::process::Command;

use crate::{
	parsing::{Expression, Func},
	throw,
};

pub fn execute(func: Func) {
	if func.is_empty() {
		return;
	}
	let name = evaluate_expression(func.name);
	let args = evaluate_args(func.arguments);

	let startup_result = Command::new(name.clone()).args(args).spawn();
	let mut child =
		startup_result.unwrap_or_else(|e| throw!("Could not start command {}: {e}", name));
	let exit_code = child.wait();
	exit_code.unwrap_or_else(|e| throw!("Exited with code: {e}"));
}

fn evaluate_args(func: Vec<Expression>) -> Vec<String> {
	func.into_iter().map(evaluate_expression).collect()
}

fn evaluate_expression(expr: Expression) -> String {
	match expr {
		Expression::String(str) => str,
		Expression::Function(func) => evaluate_func(func),
	}
}

fn evaluate_func(func: Box<Func>) -> String {
	let name = evaluate_expression(func.name);
	let args = evaluate_args(func.arguments);

	let startup_result = Command::new(name).args(args).output();
	let child = startup_result.unwrap_or_else(|e| throw!("Could not start command: {e}"));

	String::from_utf8_lossy(&child.stdout).trim().into()
}
