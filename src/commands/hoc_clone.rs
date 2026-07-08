use super::{AppCommand, run_in_terminal};
use anyhow::Result;

pub struct HocClone;

impl AppCommand for HocClone {
	fn label(&self) -> &str {
		"Clone: HeroesOfCode's repositories"
	}

	fn execute(&self, _input: &str) -> Result<()> {
		run_in_terminal("hoc", &["clone"])
	}
}
