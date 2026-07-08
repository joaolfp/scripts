pub(crate) mod clone_my_repo;
pub(crate) mod hoc_clone;
pub(crate) mod install_releasor;
pub(crate) mod install_xcode;
pub mod registry;
pub(crate) mod releasor;
pub(crate) mod rust_project;
pub(crate) mod update_claude;
pub(crate) mod update_rust;
pub(crate) mod upgrade_mise;

use anyhow::Result;

pub trait AppCommand {
	fn label(&self) -> &str;
	fn input_prompt(&self) -> Option<&str> {
		None
	}
	fn execute(&self, input: &str) -> Result<()>;
}

pub(crate) fn run_in_terminal(program: &str, args: &[&str]) -> Result<()> {
	std::process::Command::new(program)
		.args(args)
		.stdin(std::process::Stdio::inherit())
		.stdout(std::process::Stdio::inherit())
		.stderr(std::process::Stdio::inherit())
		.status()?;

	Ok(())
}
