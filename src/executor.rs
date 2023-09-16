use std::process::Command;

use crate::{
	parsing::{Expression, Func},
	throw, unwrap_or_return,
};

pub fn executor(option_func: Option<Func>) {
	let func = unwrap_or_return!(option_func);

	let args = get_args(func.arguments);
	println!("{:?}", args);
	let startup_result = Command::new(func.name).args(args).spawn();
	let mut child = startup_result.unwrap_or_else(|e| throw!("Could not start command: {e}"));
	let exit_code = child.wait();
	exit_code.unwrap_or_else(|e| throw!("Exited with code: {e}"));
}

fn get_args(func: Vec<Expression>) -> Vec<String> {
	func.into_iter().map(evaluate_expression).collect()
}

fn evaluate_expression(expr: Expression) -> String {
	match expr {
		Expression::String(str) => str,
		Expression::Function(func) => evaluate_func(func),
	}
}

fn evaluate_func(func: Func) -> String {
	let args = get_args(func.arguments);
	let startup_result = Command::new(func.name).args(args).output();
	let child = startup_result.unwrap_or_else(|e| throw!("Could not start command: {e}"));

	String::from_utf8_lossy(&child.stdout).trim().into()
}
