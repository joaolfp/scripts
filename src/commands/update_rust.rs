use super::{AppCommand, run_in_terminal};
use anyhow::Result;

pub struct UpdateRust;

impl AppCommand for UpdateRust {
	fn label(&self) -> &str {
		"Update Rust"
	}

	fn execute(&self, _input: &str) -> Result<()> {
		run_in_terminal("rustup", &["update"])
	}
}
