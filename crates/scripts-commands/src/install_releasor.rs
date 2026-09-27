use anyhow::Result;
use scripts_core::{AppCommand, run_in_terminal};

pub struct InstallReleasor;

impl AppCommand for InstallReleasor {
	fn label(&self) -> &str {
		"Install: Releasor"
	}

	fn execute(&self, _input: &str) -> Result<()> {
		run_in_terminal("cargo", &["install", "releasor"])
	}
}
