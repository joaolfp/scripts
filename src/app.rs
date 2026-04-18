use crate::commands::AppCommand;
use anyhow::Result;
use dialoguer::{Input, Select, theme::ColorfulTheme};

pub fn show_menu(commands: &[Box<dyn AppCommand>]) -> Result<usize> {
	let labels: Vec<&str> = commands.iter().map(|c| c.label()).collect();

	let selection = Select::with_theme(&ColorfulTheme::default())
		.with_prompt("Scripts Menu")
		.items(&labels)
		.default(0)
		.interact()?;

	Ok(selection)
}

pub fn get_user_input(command: &dyn AppCommand) -> Result<String> {
	let Some(prompt) = command.input_prompt() else {
		return Ok(String::new());
	};

	let input = Input::with_theme(&ColorfulTheme::default())
		.with_prompt(prompt)
		.interact_text()?;

	Ok(input)
}
