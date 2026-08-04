use crate::commands::AppCommand;
use anyhow::Result;
use dialoguer::{Input, Select, theme::DraculaTheme};

enum TopEntry {
	Direct(usize),
	Category(&'static str, Vec<usize>),
}

fn category_of(label: &str) -> Option<&'static str> {
	if label.starts_with("Clone:") {
		Some("Clone")
	} else if label.starts_with("Install:") {
		Some("Install")
	} else if label.starts_with("Update:") {
		Some("Update")
	} else {
		None
	}
}

fn build_top_entries(commands: &[Box<dyn AppCommand>]) -> Vec<TopEntry> {
	let mut entries: Vec<TopEntry> = Vec::new();

	for (index, command) in commands.iter().enumerate() {
		match category_of(command.label()) {
			Some(category) => match entries
				.iter_mut()
				.find(|entry| matches!(entry, TopEntry::Category(name, _) if *name == category))
			{
				Some(TopEntry::Category(_, indices)) => indices.push(index),
				_ => entries.push(TopEntry::Category(category, vec![index])),
			},
			None => entries.push(TopEntry::Direct(index)),
		}
	}

	entries
}

fn select_from(prompt: &str, labels: &[&str]) -> Result<usize> {
	Select::with_theme(&DraculaTheme::default())
		.with_prompt(prompt)
		.items(labels)
		.default(0)
		.interact()
		.map_err(Into::into)
}

pub fn show_menu(commands: &[Box<dyn AppCommand>]) -> Result<usize> {
	let entries = build_top_entries(commands);
	let labels: Vec<&str> = entries
		.iter()
		.map(|entry| match entry {
			TopEntry::Direct(index) => commands[*index].label(),
			TopEntry::Category(name, _) => name,
		})
		.collect();

	let selection = select_from("Scripts Menu", &labels)?;

	match &entries[selection] {
		TopEntry::Direct(index) => Ok(*index),
		TopEntry::Category(name, indices) => {
			let sub_labels: Vec<&str> = indices.iter().map(|&i| commands[i].label()).collect();
			let sub_selection = select_from(name, &sub_labels)?;
			Ok(indices[sub_selection])
		}
	}
}

pub fn get_user_input(command: &dyn AppCommand) -> Result<String> {
	let Some(prompt) = command.input_prompt() else {
		return Ok(String::new());
	};

	let input = Input::with_theme(&DraculaTheme::default())
		.with_prompt(prompt)
		.interact_text()?;

	Ok(input)
}
