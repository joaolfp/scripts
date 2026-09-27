use anyhow::Result;
use scripts_core::AppCommand;

pub struct Exit;

impl AppCommand for Exit {
	fn label(&self) -> &str {
		"Exit"
	}

	fn execute(&self, _input: &str) -> Result<()> {
		println!("Bye bye 👋");
		std::process::exit(0);
	}
}
