use std::process::{Command, Stdio};

use crate::{
	errors::{ExecutorError, ExecutorErrorType},
	executor::{context::Context, evaluate_expression, Value},
	parser::Expression,
};

pub fn evaluate_pipe(mut args: Vec<Expression>, context: &Context) -> Result<Value, ExecutorError> {
	let mut prev = match evaluate_expression(args.remove(0), context)? {
		Value::String(s) => {
			// Probably not the best way of doing this, but it works for now.
			let mut command = Command::new("echo");
			command.arg(s);
			command
		}
		Value::Command(command) => command,
		Value::Cd(_) => {
			return Err(ExecutorErrorType::BuiltinExecutionError(
				"cd cannot be piped from.".to_string(),
			)
			.binary("pipe".to_string()))
		}
	};

	for arg in args {
		match evaluate_expression(arg, context)? {
			Value::String(s) => {
				return Err(ExecutorError::from_type(
					ExecutorErrorType::BuiltinExecutionError(format!(
						"Expected command but instead attempted to pipe into string '{s}'"
					)),
				))
			}
			Value::Command(mut command) => {
				prev.stdout(Stdio::piped());
				let child = prev.spawn()?;
				command.stdin(Stdio::from(child.stdout.unwrap()));
				prev = command;
			}
			Value::Cd(_) => {
				return Err(ExecutorErrorType::BuiltinExecutionError(
					"cd cannot be piped from or into.".to_string(),
				)
				.binary("pipe".to_string()))
			}
		}
	}
	Ok(Value::Command(prev))
}
