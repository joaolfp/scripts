use super::{AppCommand, run_in_terminal};
use anyhow::Result;

pub struct UpgradeMise;

impl AppCommand for UpgradeMise {
	fn label(&self) -> &str {
		"Update: mise"
	}

	fn execute(&self, _input: &str) -> Result<()> {
		run_in_terminal("brew", &["upgrade", "mise"])
	}
}
