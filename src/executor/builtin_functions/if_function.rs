use crate::{
	errors::{ExecutorError, ExecutorErrorType},
	executor::{context::Context, evaluate_expression, evaluate_expression_to_string, Value},
	parser::Expression,
};

pub fn evaluate_if(mut args: Vec<Expression>, context: &Context) -> Result<Value, ExecutorError> {
	if args.len() != 3 {
		return Err(ExecutorError::from_type(
			ExecutorErrorType::IncorrectNumberOfArgsToBuiltinFunction,
		)
		.with("if".to_string()));
	}
	let predicate = args.remove(0);
	let true_expression = args.remove(0);
	let false_expression = args.remove(0);
	match evaluate_expression_to_string(predicate, context)?.as_str() {
		"true" => evaluate_expression(true_expression, context),
		"false" => evaluate_expression(false_expression, context),
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
