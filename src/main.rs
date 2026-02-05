use anyhow::Result;
use duct::cmd;
use std::env;
use std::fs;
use std::io::{self, Write};
use xx::git::CloneOptions;

#[derive(Debug, PartialEq)]
enum MenuOption {
	CloneRepos,
	Releasor,
	RustProject,
	CloneMyRepos,
	Xcode,
	UpgradeMise,
	Quit,
	Invalid,
}

fn parse_menu_option(input: &str) -> MenuOption {
	match input.trim() {
		"1" => MenuOption::CloneRepos,
		"2" => MenuOption::Releasor,
		"3" => MenuOption::RustProject,
		"4" => MenuOption::CloneMyRepos,
		"5" => MenuOption::Xcode,
		"6" => MenuOption::UpgradeMise,
		"q" | "quit" | "exit" => MenuOption::Quit,
		_ => MenuOption::Invalid,
	}
}

fn main() -> Result<()> {
	loop {
		println!();
		println!("1 - Clone HeroesOfCode's repositories");
		println!("2 - Releasor");
		println!("3 - Create rust project");
		println!("4 - Clone my repositories");
		println!("5 - Install Xcode");
		println!("6 - Upgrade mise");
		println!("q - Quit");

		println!();
		println!("Choose an option: ");
		io::stdout().flush()?;

		let mut input = String::new();
		io::stdin().read_line(&mut input)?;

		match parse_menu_option(&input) {
			MenuOption::CloneRepos => {
				cmd!("hoc", "clone").run()?;
			}
			MenuOption::Releasor => {
				setup_releasor()?;
			}
			MenuOption::RustProject => {
				setup_rust_project()?;
			}
			MenuOption::CloneMyRepos => {
				clone_my_repositories()?;
			}
			MenuOption::Xcode => {
				setup_xcode()?;
			}
			MenuOption::UpgradeMise => {
				cmd!("brew", "upgrade", "mise").run()?;
			}
			MenuOption::Quit => {
				println!("Bye 👋");
				break;
			}
			MenuOption::Invalid => {
				println!("❌ This option does not exist");
			}
		}
	}

	Ok(())
}

fn setup_releasor() -> Result<()> {
	let package_name = input_name(String::new(), "What's the package name: ")?;
	cmd!("releasor", "-f", package_name).run()?;
	Ok(())
}

fn setup_rust_project() -> Result<()> {
	let project_name = input_name(String::new(), "What's the project name: ")?;
	cmd!(
		"bash",
		"-c",
		format!("cargo new {} && cd {}", project_name, project_name)
	)
	.run()?;

	let cwd = env::current_dir()?;
	let dest = cwd.join(&project_name).join("rust_files.sh");

	let script = include_str!("../rust_files.sh");
	fs::write(&dest, script)?;

	cmd!(
		"bash",
		"-c",
		format!(
			"cd {} && chmod +x rust_files.sh && ./rust_files.sh && rm rust_files.sh && rm -rf .git",
			project_name
		)
	)
	.run()?;

	Ok(())
}

fn clone_my_repositories() -> Result<()> {
	let repo = input_name(String::new(), "Which repository do you want to clone: ")?;

	let opts = CloneOptions::default().branch("main");
	let url = format!("https://github.com/joaolfp/{repo}");
	let dest = format!("{repo}/");
	xx::git::clone(&url, &dest, &opts)?;
    
	Ok(())
}

fn setup_xcode() -> Result<()> {
	let output = cmd!("xcodes", "version").read()?;

	if output.contains("1.6.2") {
		let version = input_name(
			String::new(),
			"What's the Xcode version you want to install: ",
		)?;

		cmd!("xcodes", "update").run()?;
		cmd!("xcodes", "install", version).run()?;
	} else {
		cmd!("brew", "install", "xcodesorg/made/xcodes").run()?;
	}

	Ok(())
}

fn input_name(mut input_name: String, message: &str) -> Result<String> {
	println!();
	println!("{}", message);

	io::stdin().read_line(&mut input_name)?;

	let value = input_name.trim();
	Ok(value.to_string())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_parse_menu_option_numeric() {
		assert_eq!(parse_menu_option("1"), MenuOption::CloneRepos);
		assert_eq!(parse_menu_option("2"), MenuOption::Releasor);
		assert_eq!(parse_menu_option("3"), MenuOption::RustProject);
		assert_eq!(parse_menu_option("4"), MenuOption::CloneMyRepos);
		assert_eq!(parse_menu_option("5"), MenuOption::Xcode);
		assert_eq!(parse_menu_option("6"), MenuOption::UpgradeMise);
	}

	#[test]
	fn test_parse_menu_option_quit_aliases() {
		assert_eq!(parse_menu_option("q"), MenuOption::Quit);
		assert_eq!(parse_menu_option("quit"), MenuOption::Quit);
		assert_eq!(parse_menu_option("exit"), MenuOption::Quit);
	}

	#[test]
	fn test_parse_menu_option_trims_whitespace() {
		assert_eq!(parse_menu_option("1\n"), MenuOption::CloneRepos);
		assert_eq!(parse_menu_option("  2  "), MenuOption::Releasor);
		assert_eq!(parse_menu_option("q\n"), MenuOption::Quit);
	}

	#[test]
	fn test_parse_menu_option_invalid() {
		assert_eq!(parse_menu_option(""), MenuOption::Invalid);
		assert_eq!(parse_menu_option("9"), MenuOption::Invalid);
		assert_eq!(parse_menu_option("invalid"), MenuOption::Invalid);
		assert_eq!(parse_menu_option("Q"), MenuOption::Invalid);
	}
}
