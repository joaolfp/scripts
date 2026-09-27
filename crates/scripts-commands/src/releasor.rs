use anyhow::Result;
use scripts_core::{AppCommand, run_in_terminal};

pub struct Releasor;

impl AppCommand for Releasor {
	fn label(&self) -> &str {
		"Releasor"
	}

	fn input_prompt(&self) -> Option<&str> {
		Some("Package name")
	}

	fn execute(&self, input: &str) -> Result<()> {
		run_in_terminal("releasor", &["-f", input])
	}
}
