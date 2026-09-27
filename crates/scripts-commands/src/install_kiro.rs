use anyhow::Result;
use scripts_core::{AppCommand, run_in_terminal};

pub struct InstallKiro;

impl AppCommand for InstallKiro {
	fn label(&self) -> &str {
		"Install: Kiro"
	}

	fn execute(&self, _input: &str) -> Result<()> {
		run_in_terminal(
			"bash",
			&["-c", "curl -fsSL https://cli.kiro.dev/install | bash"],
		)
	}
}
