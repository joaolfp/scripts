use super::{AppCommand, run_in_terminal};
use anyhow::Result;

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
