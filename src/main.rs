mod app;
mod commands;

use anyhow::Result;

fn main() -> Result<()> {
	let commands = commands::registry::all();
	let selected = app::show_menu(&commands)?;
	let input = app::get_user_input(&*commands[selected])?;
	commands[selected].execute(&input)
}
