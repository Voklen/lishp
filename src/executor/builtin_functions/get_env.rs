use std::env::{self, VarError};

use crate::{
	errors::{ExecutorError, ExecutorErrorType},
	executor::{context::Context, evaluate_expression_to_string, Value},
	parser::Expression,
};

pub fn get_env(mut args: Vec<Expression>, context: &Context) -> Result<Value, ExecutorError> {
	if args.len() != 1 {
		return Err(
			ExecutorErrorType::IncorrectNumberOfArgsToBuiltinFunction.binary("get-env".to_string())
		);
	}
	let name = evaluate_expression_to_string(args.remove(0), context)?;
	let value = match env::var(name) {
		Ok(res) => res,
		Err(VarError::NotPresent) => {
			//TODO Potentially return null?
			return Err(ExecutorErrorType::BuiltinExecutionError(
				"Variable not present".to_string(),
			)
			.binary("get-env".to_string()));
		}
		Err(VarError::NotUnicode(_)) => {
			return Err(ExecutorErrorType::BuiltinExecutionError(
				"Variable not valid unicode".to_string(),
			)
			.binary("get-env".to_string()));
		}
	};
	Ok(Value::String(value))
}
