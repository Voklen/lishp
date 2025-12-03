use lishp::{
	errors::LexerError,
	lexer::{lex, Token},
};

#[test]
fn empty_string() {
	let lexed = lex("").unwrap();
	assert_eq!(lexed, vec![]);
}

#[test]
fn single_command() {
	let lexed = lex("ls").unwrap();
	assert_eq!(lexed, vec![Token::String("ls".to_string())]);
}

#[test]
fn single_command_with_args() {
	let lexed = lex("ls src target").unwrap();
	assert_eq!(
		lexed,
		vec![
			Token::String("ls".to_string()),
			Token::String("src".to_string()),
			Token::String("target".to_string())
		]
	);
}

#[test]
fn inserted_quote_error() {
	let lexed = lex("ls direc\"tory");
	assert_eq!(lexed, Err(LexerError::QuoteWithinArgument));
}

#[test]
fn inserted_open_parenthesis_error() {
	let lexed = lex("ls direc(tory");
	assert_eq!(lexed, Err(LexerError::OpenParethesisWithinArgument));
}

#[test]
fn inserted_close_parenthesis() {
	let lexed = lex("ls direc)tory").unwrap();
	assert_eq!(
		lexed,
		vec![
			Token::String("ls".to_string()),
			Token::String("direc".to_string()),
			Token::FunctionEnd,
			Token::String("tory".to_string()),
		]
	);
}

#[test]
fn surrounded_by_quotes() {
	let lexed = lex("\"ls\" \"src\" \"target\"").unwrap();
	assert_eq!(
		lexed,
		vec![
			Token::String("ls".to_string()),
			Token::String("src".to_string()),
			Token::String("target".to_string())
		]
	);
}

#[test]
fn function_call() {
	let lexed = lex("ls (echo src)").unwrap();
	assert_eq!(
		lexed,
		vec![
			Token::String("ls".to_string()),
			Token::FunctionStart,
			Token::String("echo".to_string()),
			Token::String("src".to_string()),
			Token::FunctionEnd,
		]
	);
}

#[test]
fn function_call_quoted() {
	let lexed = lex("\"ls\" (\"echo\" \"src\")").unwrap();
	assert_eq!(
		lexed,
		vec![
			Token::String("ls".to_string()),
			Token::FunctionStart,
			Token::String("echo".to_string()),
			Token::String("src".to_string()),
			Token::FunctionEnd,
		]
	);
}

#[test]
fn backslashes() {
	let lexed = lex("ls (echo weird\\ chars\\)\\(\\\"\\\\)").unwrap();
	assert_eq!(
		lexed,
		vec![
			Token::String("ls".to_string()),
			Token::FunctionStart,
			Token::String("echo".to_string()),
			Token::String("weird chars)(\"\\".to_string()),
			Token::FunctionEnd,
		]
	);
}

#[test]
fn trailing_backslash_error() {
	let lexed = lex("ls src\\");
	assert_eq!(lexed, Err(LexerError::TrailingBackslash));
}

#[test]
fn unclosed_quote_error() {
	let lexed = lex("ls \"src");
	assert_eq!(lexed, Err(LexerError::UnclosedQuote));
}
