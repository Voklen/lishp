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
	let full_path = match context.working_dir.join(&path).canonicalize() {
		Ok(res) => res,
		Err(e) => {
			return Err(
				ExecutorErrorType::BuiltinExecutionError(e.to_string()).binary("cd".to_string())
			)
		}
	};
	if !full_path.is_dir() {
		return Err(
			ExecutorErrorType::BuiltinExecutionError(format!("{path}: Not a directory"))
				.binary("cd".to_string()),
		);
	}
	Ok(Value::Cd(full_path))
}
