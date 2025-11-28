use std::env::set_var;

use crate::{
	errors::{ExecutorError, ExecutorErrorType},
	executor::{context::Context, evaluate_expression_to_string, Value},
	parser::Expression,
};

pub fn set_env(mut args: Vec<Expression>, context: &Context) -> Result<Value, ExecutorError> {
	if args.len() != 2 {
		return Err(ExecutorError::from_type(
			ExecutorErrorType::IncorrectNumberOfArgsToBuiltinFunction,
		)
		.with("set-env".to_string()));
	}
	let name = evaluate_expression_to_string(args.remove(0), context)?;
	let value = evaluate_expression_to_string(args.remove(0), context)?;
	set_var(name, &value);
	// Unwrap an infalliable error.

	Ok(Value::String(value))
}
