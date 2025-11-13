use std::process::{Child, Command};

use crate::{
	errors::ExecutorError,
	parser::{Expression, Func},
};

pub fn execute(func: Func) {
	if func.is_empty() {
		return;
	}
	let mut child = match spawn_command(func) {
		Ok(res) => res,
		Err(e) => {
			eprintln!("{e}");
			return;
		}
	};
	let exit_code = child.wait();
	exit_code.expect("Error, command was not running");
}

fn spawn_command(func: Func) -> Result<Child, ExecutorError> {
	let name = evaluate_expression(func.name)?;
	let args = evaluate_args(func.arguments)?;

	match Command::new(name.clone()).args(args).spawn() {
		Ok(child) => Ok(child),
		Err(e) => Err(ExecutorError::from(e).with(name)),
	}
}

fn evaluate_args(func: Vec<Expression>) -> Result<Vec<String>, ExecutorError> {
	func.into_iter().map(evaluate_expression).collect()
}

fn evaluate_expression(expr: Expression) -> Result<String, ExecutorError> {
	match expr {
		Expression::String(str) => Ok(str),
		Expression::Function(func) => evaluate_func(func),
	}
}

fn evaluate_func(func: Box<Func>) -> Result<String, ExecutorError> {
	let name = evaluate_expression(func.name)?;
	let args = evaluate_args(func.arguments)?;

	let child = match Command::new(name.clone()).args(args).output() {
		Ok(child) => child,
		Err(e) => return Err(ExecutorError::from(e).with(name)),
	};

	Ok(String::from_utf8_lossy(&child.stdout).trim().into())
}
