use std::{collections::HashMap, env, io, path::PathBuf};

#[derive(Clone)]
pub struct Context {
	pub working_dir: PathBuf,
	pub vars: HashMap<String, String>,
}

impl Context {
	pub fn new() -> io::Result<Self> {
		let working_dir = env::current_dir()?;
		let vars = HashMap::new();
		Ok(Context { working_dir, vars })
	}
}
