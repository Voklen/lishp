use std::{borrow::Cow, env};

use reedline::{Prompt, PromptEditMode, PromptHistorySearchStatus, PromptViMode};

use lishp::executor::context::Context;

// Default prompt indicators
pub static DEFAULT_PROMPT_INDICATOR: &str = "〉";
pub static DEFAULT_VI_INSERT_PROMPT_INDICATOR: &str = ": ";
pub static DEFAULT_VI_NORMAL_PROMPT_INDICATOR: &str = "〉";
pub static DEFAULT_MULTILINE_INDICATOR: &str = "::: ";

pub struct LishpPrompt<'a> {
	context: &'a Context,
}

impl<'a> LishpPrompt<'a> {
	pub fn new(context: &'a Context) -> Self {
		LishpPrompt { context }
	}
}

impl<'a> Prompt for LishpPrompt<'a> {
	fn render_prompt_left(&self) -> std::borrow::Cow<'_, str> {
		let path_str = self.context.working_dir.display().to_string();
		let normalised = normalise_path(path_str);
		Cow::Owned(normalised)
	}

	fn render_prompt_right(&self) -> std::borrow::Cow<'_, str> {
		Cow::Owned("".to_string())
	}

	fn render_prompt_indicator(
		&self,
		prompt_mode: reedline::PromptEditMode,
	) -> std::borrow::Cow<'_, str> {
		match prompt_mode {
			PromptEditMode::Default | PromptEditMode::Emacs => DEFAULT_PROMPT_INDICATOR.into(),
			PromptEditMode::Vi(vi_mode) => match vi_mode {
				PromptViMode::Normal => DEFAULT_VI_NORMAL_PROMPT_INDICATOR.into(),
				PromptViMode::Insert => DEFAULT_VI_INSERT_PROMPT_INDICATOR.into(),
			},
			PromptEditMode::Custom(str) => format!("({str})").into(),
		}
	}

	fn render_prompt_multiline_indicator(&self) -> std::borrow::Cow<'_, str> {
		Cow::Borrowed(DEFAULT_MULTILINE_INDICATOR)
	}

	fn render_prompt_history_search_indicator(
		&self,
		history_search: reedline::PromptHistorySearch,
	) -> std::borrow::Cow<'_, str> {
		// Taken from the default prompt:
		let prefix = match history_search.status {
			PromptHistorySearchStatus::Passing => "",
			PromptHistorySearchStatus::Failing => "failing ",
		};
		// NOTE: magic strings, given there is logic on how these compose I am not sure if it
		// is worth extracting in to static constant
		Cow::Owned(format!(
			"({}reverse-search: {}) ",
			prefix, history_search.term
		))
	}
}

fn normalise_path(path: String) -> String {
	let homedir = match get_home_dir() {
		Some(res) => res,
		None => return path,
	};
	if path != homedir {
		path.replace(&homedir, "~")
	} else {
		path
	}
}

fn get_home_dir() -> Option<String> {
	if let Ok(mac_linux_home) = env::var("HOME") {
		return Some(mac_linux_home);
	}
	if let Ok(windows_home) = env::var("USERPROFILE") {
		return Some(windows_home);
	}
	None
}
