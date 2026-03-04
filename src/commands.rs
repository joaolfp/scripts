use anyhow::Result;
use duct::cmd;
use std::process::Command;
use std::{env, fs};

pub struct CommandSpec {
	pub program: String,
	pub args: Vec<String>,
}

pub fn command_spec(selected: usize, input: &str) -> Result<Option<CommandSpec>> {
	let spec = match selected {
		0 => CommandSpec {
			program: "hoc".into(),
			args: vec!["clone".into()],
		},
		1 => CommandSpec {
			program: "releasor".into(),
			args: vec!["-f".into(), input.to_string()],
		},
		2 => {
			let script = format!(
				"cd {} && chmod +x rust_files.sh && ./rust_files.sh && rm rust_files.sh && rm -rf .git",
				input
			);
			CommandSpec {
				program: "bash".into(),
				args: vec!["-c".into(), script],
			}
		}
		3 => return Ok(None),
		4 => {
			let output = cmd!("xcodes", "version").read().unwrap_or_default();
			if output.contains("1.6.2") {
				CommandSpec {
					program: "bash".into(),
					args: vec![
						"-c".into(),
						format!("xcodes update && xcodes install {}", input),
					],
				}
			} else {
				CommandSpec {
					program: "brew".into(),
					args: vec!["install".into(), "xcodesorg/made/xcodes".into()],
				}
			}
		}
		5 => CommandSpec {
			program: "brew".into(),
			args: vec!["upgrade".into(), "mise".into()],
		},
		6 => CommandSpec {
			program: "bash".into(),
			args: vec!["release-rust.sh".into()],
		},
		_ => return Ok(None),
	};
	Ok(Some(spec))
}

pub fn prepare_rust_project(project_name: &str) -> Result<()> {
	cmd!("cargo", "new", project_name).run()?;
	let dest = env::current_dir()?.join(project_name).join("rust_files.sh");
	fs::write(&dest, include_str!("../rust_files.sh"))?;
	Ok(())
}

pub fn run_clone_my_repositories(repo: &str) -> Result<String> {
	use xx::git::CloneOptions;
	let opts = CloneOptions::default().branch("main");
	xx::git::clone(
		&format!("https://github.com/joaolfp/{repo}"),
		&format!("{repo}/"),
		&opts,
	)?;
	Ok(format!("Cloned: {}", repo))
}

pub fn run_in_terminal(spec: &CommandSpec) -> Result<std::process::ExitStatus> {
	Command::new(&spec.program)
		.args(&spec.args)
		.stdin(std::process::Stdio::inherit())
		.stdout(std::process::Stdio::inherit())
		.stderr(std::process::Stdio::inherit())
		.status()
		.map_err(Into::into)
}
