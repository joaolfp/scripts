use anyhow::Result;
use scripts_core::{AppCommand, run_in_terminal};

pub struct UpdateRust;

impl AppCommand for UpdateRust {
	fn label(&self) -> &str {
		"Update: Rust"
	}

	fn execute(&self, _input: &str) -> Result<()> {
		run_in_terminal("rustup", &["update"])
	}
}
