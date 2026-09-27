use anyhow::Result;

pub trait AppCommand {
	fn label(&self) -> &str;
	fn input_prompt(&self) -> Option<&str> {
		None
	}
	fn execute(&self, input: &str) -> Result<()>;
}

pub fn run_in_terminal(program: &str, args: &[&str]) -> Result<()> {
	std::process::Command::new(program)
		.args(args)
		.stdin(std::process::Stdio::inherit())
		.stdout(std::process::Stdio::inherit())
		.stderr(std::process::Stdio::inherit())
		.status()?;

	Ok(())
}
