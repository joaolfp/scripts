use anyhow::Result;
use scripts_core::{AppCommand, run_in_terminal};

pub struct UpdateOmarchy;

impl AppCommand for UpdateOmarchy {
	fn label(&self) -> &str {
		"Update: Omarchy"
	}

	fn execute(&self, _input: &str) -> Result<()> {
		run_in_terminal("omarchy-update", &[])
	}
}
