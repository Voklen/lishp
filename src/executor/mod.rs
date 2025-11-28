use std::{
	path::PathBuf,
	process::{self, Command},
};

use crate::{
	errors::{ExecutorError, ExecutorErrorType},
	executor::{
		builtin_functions::{cd::evaluate_cd, if_function::evaluate_if, pipe::evaluate_pipe},
		context::Context,
	},
	parser::{Expression, Func},
};

mod builtin_functions;
pub mod context;

enum CommandOrString {
	Command(process::Command),
	String(String),
	Cd(PathBuf),
}

pub fn execute(func: Func, context: &mut Context) {
	match execute_with_result(func, context) {
		Ok(()) => {}
		Err(e) => {
			eprintln!("{e}");
		}
	}
}

fn execute_with_result(func: Func, context: &mut Context) -> Result<(), ExecutorError> {
	let command_or_string = evaluate_func(Box::new(func), context)?;

	let mut command = match command_or_string {
		CommandOrString::Command(command) => command,
		CommandOrString::String(string) => {
			println!("{string}");
			return Ok(());
		}
		CommandOrString::Cd(path) => {
			context.working_dir = match context.working_dir.join(path).canonicalize() {
				Ok(res) => res,
				Err(e) => {
					return Err(ExecutorErrorType::BuiltinExecutionError(format!(
						"Error canonicalizing path: {e}"
					))
					.binary("cd".to_string()))
				}
			};
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

fn evaluate_expression_to_string(
	expr: Expression,
	context: &Context,
) -> Result<String, ExecutorError> {
	let string = match evaluate_expression(expr, context)? {
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
		CommandOrString::Cd(_) => {
			return Err(
				ExecutorError::from_type(ExecutorErrorType::BuiltinExecutionError(
					"Cannot use cd as a value. cd can only be used as the outermost function."
						.to_string(),
				))
				.with("cd".to_string()),
			);
		}
	};
	Ok(string)
}

fn evaluate_expression(
	expr: Expression,
	context: &Context,
) -> Result<CommandOrString, ExecutorError> {
	let string = match expr {
		Expression::String(str) => CommandOrString::String(str),
		Expression::Function(func) => evaluate_func(func, context)?,
	};
	Ok(string)
}

fn evaluate_func(func: Box<Func>, context: &Context) -> Result<CommandOrString, ExecutorError> {
	let name = evaluate_expression_to_string(func.name, context)?;
	let result_string = match name.as_str() {
		"" => CommandOrString::String("".to_string()),
		"if" => evaluate_if(func.arguments, context)?,
		"pipe" | "|" => evaluate_pipe(func.arguments, context)?,
		"cd" => evaluate_cd(func.arguments, context)?,
		command => CommandOrString::Command(evalute_command(command, func.arguments, context)?),
	};
	Ok(result_string)
}

fn evalute_command(
	name: &str,
	args: Vec<Expression>,
	context: &Context,
) -> Result<Command, ExecutorError> {
	let args = evaluate_args(args, context)?;
	let mut command = Command::new(name);
	command.args(args);
	command.current_dir(&context.working_dir);
	Ok(command)
}

fn evaluate_args(func: Vec<Expression>, context: &Context) -> Result<Vec<String>, ExecutorError> {
	func.into_iter()
		.map(|e| evaluate_expression_to_string(e, context))
		.collect()
}
