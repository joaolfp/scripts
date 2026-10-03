use anyhow::Result;
use scripts_core::AppCommand;

use crate::hoc::run_hoc;

pub struct CloneMyRepo;

impl AppCommand for CloneMyRepo {
	fn label(&self) -> &str {
		"Clone: My repositories"
	}

	fn execute(&self, _input: &str) -> Result<()> {
		run_hoc(&["m"])
	}
}
