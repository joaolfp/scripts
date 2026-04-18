use super::{AppCommand, run_in_terminal};
use anyhow::Result;
use duct::cmd;
use std::{env, fs};

pub struct CreateRustProject;

impl AppCommand for CreateRustProject {
	fn label(&self) -> &str {
		"Create rust project"
	}

	fn input_prompt(&self) -> Option<&str> {
		Some("Project name")
	}

	fn execute(&self, project_name: &str) -> Result<()> {
		cmd!("cargo", "new", project_name).run()?;
		let dest = env::current_dir()?.join(project_name).join("rust_files.sh");
		fs::write(&dest, include_str!("../../rust_files.sh"))?;

		let script = format!(
			"cd {project_name} && chmod +x rust_files.sh && ./rust_files.sh && rm rust_files.sh && rm -rf .git"
		);

		run_in_terminal("bash", &["-c", &script])
	}
}
