pub const KEYWORDS: [&str; 5] = ["if", "pipe", "cd", "set-env", "get-env"];

pub mod errors;
pub mod executor;
pub mod lexer;
pub mod parser;
