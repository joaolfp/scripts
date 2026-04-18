mod clone_my_repo;
mod hoc_clone;
mod install_xcode;
pub mod registry;
mod releasor;
mod rust_project;
mod upgrade_mise;

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
