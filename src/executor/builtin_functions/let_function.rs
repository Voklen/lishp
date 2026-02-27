use crate::{
	errors::{ExecutorError, ExecutorErrorType},
	executor::{context::Context, evaluate_expression_to_string, Value},
	parser::Expression,
};

pub fn let_function(mut args: Vec<Expression>, context: &Context) -> Result<Value, ExecutorError> {
	if args.len() != 2 {
		return Err(ExecutorError::from_type(
			ExecutorErrorType::IncorrectNumberOfArgsToBuiltinFunction,
		)
		.with("let".to_string()));
	}
	let name = evaluate_expression_to_string(args.remove(0), context)?;
	let value = evaluate_expression_to_string(args.remove(0), context)?;
	Ok(Value::Let(name, value))
}
