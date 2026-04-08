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
		0 => CommandSpec { program: "hoc".into(), args: vec!["clone".into()] },
		1 => CommandSpec { program: "releasor".into(), args: vec!["-f".into(), input.to_string()] },
		2 => rust_files_spec(input),
		3 => return Ok(None),
		4 => xcodes_spec(input),
		5 => CommandSpec { program: "brew".into(), args: vec!["upgrade".into(), "mise".into()] },
		_ => return Ok(None),
	};

	Ok(Some(spec))
}

fn rust_files_spec(project: &str) -> CommandSpec {
	let script = format!(
		"cd {project} && chmod +x rust_files.sh && ./rust_files.sh && rm rust_files.sh && rm -rf .git"
	);

	CommandSpec { program: "bash".into(), args: vec!["-c".into(), script] }
}

fn xcodes_spec(version: &str) -> CommandSpec {
	let has_xcodes = cmd!("xcodes", "version")
		.read()
		.map(|o| o.contains("1.6.2"))
		.unwrap_or(false);

	if has_xcodes {
		CommandSpec {
			program: "bash".into(),
			args: vec!["-c".into(), format!("xcodes update && xcodes install {version}")],
		}
	} else {
		CommandSpec {
			program: "brew".into(),
			args: vec!["install".into(), "xcodesorg/made/xcodes".into()],
		}
	}
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

pub fn dispatch(selected: usize, input: &str) -> Result<()> {
	if selected == 3 {
		run_clone_my_repositories(input)?;
		return Ok(());
	}

	if selected == 2 {
		prepare_rust_project(input)?;
	}

	if let Some(spec) = command_spec(selected, input)? {
		run_in_terminal(&spec)?;
	}

	Ok(())
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
