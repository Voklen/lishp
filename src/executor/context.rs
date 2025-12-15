use std::{env, io, path::PathBuf};

#[derive(Clone)]
pub struct Context {
	pub working_dir: PathBuf,
}

impl Context {
	pub fn new() -> io::Result<Self> {
		let working_dir = env::current_dir()?;
		Ok(Context { working_dir })
	}
}
