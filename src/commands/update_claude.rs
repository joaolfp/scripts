use super::{AppCommand, run_in_terminal};
use anyhow::Result;

pub struct UpdateClaude;

impl AppCommand for UpdateClaude {
	fn label(&self) -> &str {
		"Update: Claude"
	}

	fn execute(&self, _input: &str) -> Result<()> {
		run_in_terminal("claude", &["update"])
	}
}
