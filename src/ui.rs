use anyhow::Result;
use dialoguer::{Input, Select, theme::ColorfulTheme};

const MENU_ITEMS: &[&str] = &[
	"Clone HeroesOfCode's repositories",
	"Releasor",
	"Create rust project",
	"Clone my repositories",
	"Install Xcode",
	"Upgrade mise",
];

pub fn show_menu() -> Result<usize> {
	let selection = Select::with_theme(&ColorfulTheme::default())
		.with_prompt("Scripts Menu")
		.items(MENU_ITEMS)
		.default(0)
		.interact()?;

	Ok(selection)
}

pub fn get_user_input(selected: usize) -> Result<String> {
	if !matches!(selected, 1 | 2 | 3 | 4) {
		return Ok(String::new());
	}

	let prompt = match selected {
		1 => "Package name",
		2 => "Project name",
		3 => "Repository",
		4 => "Xcode version",
		_ => "",
	};

	let input = Input::with_theme(&ColorfulTheme::default())
		.with_prompt(prompt)
		.interact_text()?;

	Ok(input)
}
