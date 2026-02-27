use std::{
	path::PathBuf,
	process::{self, Command},
};

use crate::{
	errors::{ExecutorError, ExecutorErrorType},
	executor::{
		builtin_functions::{
			cd::evaluate_cd, get_env::get_env, if_function::evaluate_if,
			let_function::let_function, pipe::evaluate_pipe, set_env::set_env,
		},
		context::Context,
	},
	parser::{Expression, Func},
};

mod builtin_functions;
pub mod context;

enum Value {
	Command(process::Command),
	String(String),
	Cd(PathBuf),
	Let(String, String),
}

pub fn execute(func: Func, context: &mut Context) {
	match execute_with_result(func, context) {
		Ok(()) => {}
		Err(e) => eprintln!("{e}"),
	}
}

fn execute_with_result(func: Func, context: &mut Context) -> Result<(), ExecutorError> {
	let command_or_string = evaluate_func(Box::new(func), context)?;

	let mut command = match command_or_string {
		Value::Command(command) => command,
		Value::String(string) => {
			println!("{string}");
			return Ok(());
		}
		Value::Cd(path) => {
			context.working_dir = path;
			return Ok(());
		}
		Value::Let(key, value) => {
			context.vars.insert(key, value);
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
	match evaluate_expression(expr, context)? {
		Value::Command(mut command) => {
			let child = match command.output() {
				Ok(child) => child,
				Err(e) => {
					let binary_name = command.get_program().to_string_lossy().to_string();
					return Err(ExecutorError::from(e).with(binary_name));
				}
			};
			Ok(String::from_utf8_lossy(&child.stdout).trim().into())
		}
		Value::String(string) => Ok(string),
		Value::Cd(_) => Err(only_outermost_error("cd")),
		Value::Let(_, _) => Err(only_outermost_error("let")),
	}
}

fn only_outermost_error(function: &str) -> ExecutorError {
	ExecutorError::from_type(ExecutorErrorType::BuiltinExecutionError(format!(
		"Cannot use {function} as a value. {function} can only be used as the outermost function."
	)))
	.with(function.to_string())
}

fn evaluate_expression(expr: Expression, context: &Context) -> Result<Value, ExecutorError> {
	let string = match expr {
		Expression::String(str) => Value::String(str),
		Expression::Function(func) => evaluate_func(func, context)?,
		Expression::Variable(var) => get_var(var, context)?,
	};
	Ok(string)
}

fn evaluate_func(func: Box<Func>, context: &Context) -> Result<Value, ExecutorError> {
	let name = evaluate_expression_to_string(func.name, context)?;
	let result_string = match name.as_str() {
		"" => Value::String("".to_string()),
		"if" => evaluate_if(func.arguments, context)?,
		"pipe" | "|" => evaluate_pipe(func.arguments, context)?,
		"cd" => evaluate_cd(func.arguments, context)?,
		"set-env" => set_env(func.arguments, context)?,
		"get-env" => get_env(func.arguments, context)?,
		"let" => let_function(func.arguments, context)?,
		command => Value::Command(evalute_command(command, func.arguments, context)?),
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

fn get_var(var: String, context: &Context) -> Result<Value, ExecutorError> {
	match context.vars.get(&var) {
		//TODO Maybe don't copy here?
		Some(value) => Ok(Value::String(value.to_string())),
		None => Err(ExecutorError::from_type(
			ExecutorErrorType::VariableNotFound(var),
		)),
	}
}
