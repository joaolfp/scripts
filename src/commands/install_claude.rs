use super::{AppCommand, run_in_terminal};
use anyhow::Result;

pub struct InstallClaude;

impl AppCommand for InstallClaude {
	fn label(&self) -> &str {
		"Install Claude"
	}

	fn execute(&self, _input: &str) -> Result<()> {
		run_in_terminal(
			"bash",
			&["-c", "curl -fsSL https://claude.ai/install.sh | bash"],
		)
	}
}
