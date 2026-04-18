use super::{AppCommand, run_in_terminal};
use anyhow::Result;
use duct::cmd;

pub struct InstallXcode;

impl AppCommand for InstallXcode {
	fn label(&self) -> &str {
		"Install Xcode"
	}

	fn input_prompt(&self) -> Option<&str> {
		Some("Xcode version")
	}

	fn execute(&self, version: &str) -> Result<()> {
		let has_xcodes = cmd!("xcodes", "version")
			.read()
			.map(|o| o.contains("1.6.2"))
			.unwrap_or(false);

		if has_xcodes {
			run_in_terminal(
				"bash",
				&["-c", &format!("xcodes update && xcodes install {version}")],
			)
		} else {
			run_in_terminal("brew", &["install", "xcodesorg/made/xcodes"])
		}
	}
}
