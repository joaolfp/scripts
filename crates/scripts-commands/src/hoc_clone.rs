use anyhow::Result;
use scripts_core::AppCommand;

use crate::hoc::run_hoc;

pub struct HocClone;

impl AppCommand for HocClone {
	fn label(&self) -> &str {
		"Clone: HeroesOfCode's repositories"
	}

	fn execute(&self, _input: &str) -> Result<()> {
		run_hoc(&["clone"])
	}
}
