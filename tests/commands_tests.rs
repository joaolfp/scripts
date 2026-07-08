use scripts::commands::registry;

#[test]
fn registry_returns_ten_commands() {
	assert_eq!(registry::all().len(), 10);
}

#[test]
fn registry_labels_in_order() {
	let commands = registry::all();
	let labels: Vec<&str> = commands.iter().map(|c| c.label()).collect();

	assert_eq!(
		labels,
		[
			"Clone: HeroesOfCode's repositories",
			"Clone: My repositories",
			"Install: Releasor",
			"Install: Xcode",
			"Install: Claude",
			"Update: mise",
			"Update: Rust",
			"Update: Claude",
			"Releasor",
			"Create rust project",
		]
	);
}

#[test]
fn commands_without_input_prompt() {
	let commands = registry::all();

	let no_prompt = [
		"Clone: HeroesOfCode's repositories",
		"Install: Releasor",
		"Update: mise",
		"Update: Rust",
		"Install: Claude",
		"Update: Claude",
	];

	for cmd in &commands {
		if no_prompt.contains(&cmd.label()) {
			assert!(
				cmd.input_prompt().is_none(),
				"{} should have no prompt",
				cmd.label()
			);
		}
	}
}

#[test]
fn commands_with_input_prompt() {
	let commands = registry::all();

	let expected = [
		("Releasor", "Package name"),
		("Create rust project", "Project name"),
		("Clone: My repositories", "Repository"),
		("Install: Xcode", "Xcode version"),
	];

	for (label, prompt) in expected {
		let cmd = commands
			.iter()
			.find(|c| c.label() == label)
			.unwrap_or_else(|| panic!("command '{label}' not found"));

		assert_eq!(
			cmd.input_prompt(),
			Some(prompt),
			"wrong prompt for '{label}'"
		);
	}
}
