use std::{io::ErrorKind, process::Command};

use anyhow::Result;
use scripts_core::run_in_terminal;

const HOC_REPO: &str = "https://github.com/heroesofcode/heroesofcode-git";

/// Runs `hoc` with the given args, installing it first via cargo if it is missing.
pub fn run_hoc(args: &[&str]) -> Result<()> {
	if !is_installed() {
		println!("hoc not found, installing from {HOC_REPO}...");
		run_in_terminal("cargo", &["install", "--git", HOC_REPO])?;
	}

	run_in_terminal("hoc", args)
}

fn is_installed() -> bool {
	match Command::new("hoc").arg("--version").output() {
		Ok(_) => true,
		Err(err) => err.kind() != ErrorKind::NotFound,
	}
}
