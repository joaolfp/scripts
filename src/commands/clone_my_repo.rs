use super::AppCommand;
use anyhow::Result;

pub struct CloneMyRepo;

impl AppCommand for CloneMyRepo {
	fn label(&self) -> &str {
		"Clone my repositories"
	}

	fn input_prompt(&self) -> Option<&str> {
		Some("Repository")
	}

	fn execute(&self, repo: &str) -> Result<()> {
		use xx::git::CloneOptions;

		let opts = CloneOptions::default().branch("main");
		
        xx::git::clone(
			&format!("https://github.com/joaolfp/{repo}"),
			format!("{repo}/"),
			&opts,
		)?;

		Ok(())
	}
}
