use std::{
	env,
	fs::{self, DirEntry},
	io::Error,
};

use nu_ansi_term::{Color, Style};
use reedline::{
	default_emacs_keybindings, ColumnarMenu, DefaultHinter, Emacs, KeyCode, KeyModifiers,
	MenuBuilder, Reedline, ReedlineEvent, ReedlineMenu, Signal,
};

use lishp::{
	executor::{context::Context, execute},
	lexer::lex,
	parser::parse,
};

use crate::{completer::LishpCompleter, prompt::LishpPrompt};

mod completer;
mod prompt;

fn main() {
	let mut line_editor = get_line_editor();
	let mut context = match Context::new() {
		Ok(res) => res,
		Err(e) => {
			eprintln!("Error creating context: {e}");
			return;
		}
	};

	loop {
		let prompt = LishpPrompt::new(&context);
		let line = match line_editor.read_line(&prompt) {
			Ok(Signal::Success(line)) => line,
			Ok(Signal::CtrlC) | Ok(Signal::CtrlD) => {
				println!("Exiting, have a nice day :)");
				break;
			}
			Err(e) => {
				eprintln!("Error reading line: {e}");
				continue;
			}
		};

		let tokens = match lex(&line) {
			Ok(res) => res,
			Err(e) => {
				eprintln!("{e}");
				continue;
			}
		};
		let parsed = match parse(tokens) {
			Ok(res) => res,
			Err(e) => {
				eprintln!("{e}");
				continue;
			}
		};
		execute(parsed, &mut context);
	}
}

fn get_line_editor() -> Reedline {
	let executables = executables_in_path();
	let completer = Box::new(LishpCompleter::new(executables));
	let completion_menu = Box::new(ColumnarMenu::default().with_name("completion_menu"));

	let mut keybindings = default_emacs_keybindings();
	keybindings.add_binding(
		KeyModifiers::NONE,
		KeyCode::Tab,
		ReedlineEvent::UntilFound(vec![
			ReedlineEvent::Menu("completion_menu".to_string()),
			ReedlineEvent::MenuNext,
		]),
	);

	let edit_mode = Box::new(Emacs::new(keybindings));

	Reedline::create()
		.with_completer(completer)
		.with_menu(ReedlineMenu::EngineCompleter(completion_menu))
		.with_edit_mode(edit_mode)
		.with_hinter(Box::new(
			DefaultHinter::default().with_style(Style::new().italic().fg(Color::LightGray)),
		))
}

fn executables_in_path() -> Vec<String> {
	let path = match env::var("PATH") {
		Ok(res) => res,
		Err(_) => "/bin".to_string(),
	};

	// Ignore all errors and just collect as many executables as you can for autocompletion.
	let mut executables = vec![];
	for p in path.split(":") {
		let files = match fs::read_dir(p) {
			Ok(res) => res,
			Err(_) => continue,
		};
		for file_res in files {
			if let Some(file) = executable_name_from_entry(file_res) {
				executables.push(file);
			}
		}
	}
	executables
}

fn executable_name_from_entry(dir_entry: Result<DirEntry, Error>) -> Option<String> {
	let file = match dir_entry {
		Ok(res) => res,
		Err(_) => return None,
	};
	let (name, metadata) = match (file.file_name().into_string(), file.metadata()) {
		(Ok(name), Ok(metadata)) => (name, metadata),
		_ => return None,
	};
	if !metadata.is_file() {
		return None;
	};
	Some(name)
}
