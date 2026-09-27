use anyhow::Result;
use scripts_core::{AppCommand, run_in_terminal};

pub struct UpgradeMise;

impl AppCommand for UpgradeMise {
	fn label(&self) -> &str {
		"Update: mise"
	}

	fn execute(&self, _input: &str) -> Result<()> {
		run_in_terminal("brew", &["upgrade", "mise"])
	}
}
