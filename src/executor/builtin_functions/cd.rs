use std::{path::PathBuf, str::FromStr};

use crate::{
	errors::{ExecutorError, ExecutorErrorType},
	executor::{context::Context, evaluate_expression_to_string, Value},
	parser::Expression,
};

pub fn evaluate_cd(
	mut arguments: Vec<Expression>,
	context: &Context,
) -> Result<Value, ExecutorError> {
	let path = match arguments.len() {
		0 => "~".to_string(),
		1 => evaluate_expression_to_string(arguments.remove(0), context)?,
		_ => {
			return Err(
				ExecutorErrorType::BuiltinExecutionError("Too many arguments".to_string())
					.binary("cd".to_string()),
			)
		}
	};
	// Unwrap an infalliable error.
	let pathbuf = PathBuf::from_str(&path).unwrap();
	if !pathbuf.is_dir() {
		return Err(
			ExecutorErrorType::BuiltinExecutionError(format!("{path}: Not a directory"))
				.binary("cd".to_string()),
		);
	}
	Ok(Value::Cd(pathbuf))
}
