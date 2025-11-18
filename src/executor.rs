use std::process::{self, Command};

use crate::{
	errors::{ExecutorError, ExecutorErrorType},
	parser::{Expression, Func},
};

enum CommandOrString {
	Command(process::Command),
	String(String),
}

pub fn execute(func: Func) {
	match execute_with_result(func) {
		Ok(()) => {}
		Err(e) => {
			eprintln!("{e}");
		}
	}
}

fn execute_with_result(func: Func) -> Result<(), ExecutorError> {
	let command_or_string = evaluate_func(Box::new(func))?;

	let mut command = match command_or_string {
		CommandOrString::Command(command) => command,
		CommandOrString::String(string) => {
			println!("{string}");
			return Ok(());
		}
	};

	let mut child = match command.spawn() {
		Ok(child) => child,
		Err(e) => {
			let binary_name = command.get_program().to_string_lossy().to_string();
			return Err(ExecutorError::from(e).with(binary_name));
		}
	};
	let exit_code = child.wait();
	exit_code.expect("Error, command was not running");
	Ok(())
}

fn evaluate_args(func: Vec<Expression>) -> Result<Vec<String>, ExecutorError> {
	func.into_iter()
		.map(evaluate_expression_to_string)
		.collect()
}

fn evaluate_expression_to_string(expr: Expression) -> Result<String, ExecutorError> {
	let string = match evaluate_expression(expr)? {
		CommandOrString::Command(mut command) => {
			let child = match command.output() {
				Ok(child) => child,
				Err(e) => {
					let binary_name = command.get_program().to_string_lossy().to_string();
					return Err(ExecutorError::from(e).with(binary_name));
				}
			};
			String::from_utf8_lossy(&child.stdout).trim().into()
		}
		CommandOrString::String(string) => string,
	};
	Ok(string)
}

fn evaluate_expression(expr: Expression) -> Result<CommandOrString, ExecutorError> {
	let string = match expr {
		Expression::String(str) => CommandOrString::String(str),
		Expression::Function(func) => evaluate_func(func)?,
	};
	Ok(string)
}

fn evaluate_func(func: Box<Func>) -> Result<CommandOrString, ExecutorError> {
	let name = evaluate_expression_to_string(func.name)?;
	let result_string = match name.as_str() {
		"" => CommandOrString::String("".to_string()),
		"if" => evaluate_if(func.arguments)?,
		command => CommandOrString::Command(evalute_command(command, func.arguments)?),
	};
	Ok(result_string)
}

fn evalute_command(name: &str, args: Vec<Expression>) -> Result<Command, ExecutorError> {
	let args = evaluate_args(args)?;
	let mut command = Command::new(name);
	command.args(args);
	Ok(command)
}

fn evaluate_if(mut args: Vec<Expression>) -> Result<CommandOrString, ExecutorError> {
	if args.len() != 3 {
		return Err(ExecutorError::from_type(
			ExecutorErrorType::IncorrectNumberOfArgsToBuiltinFunction,
		)
		.with("if".to_string()));
	}
	let predicate = args.remove(0);
	let true_expression = args.remove(0);
	let false_expression = args.remove(0);
	match evaluate_expression_to_string(predicate)?.as_str() {
		"true" => evaluate_expression(true_expression),
		"false" => evaluate_expression(false_expression),
		arg => {
			return Err(
				ExecutorError::from_type(ExecutorErrorType::BuiltinExecutionError(format!(
					"First argument must be true or false but was '{arg}'"
				)))
				.with("if".to_string()),
			)
		}
	}
}
