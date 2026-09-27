use anyhow::Result;
use scripts_core::{AppCommand, run_in_terminal};

pub struct CloneMyRepo;

impl AppCommand for CloneMyRepo {
	fn label(&self) -> &str {
		"Clone: My repositories"
	}

	fn execute(&self, _input: &str) -> Result<()> {
		run_in_terminal("hoc", &["m"])
	}
}
