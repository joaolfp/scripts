mod commands;
mod ui;

use anyhow::Result;

fn main() -> Result<()> {
	let commands = commands::registry::all();
	let selected = ui::show_menu(&commands)?;
	let input = ui::get_user_input(&*commands[selected])?;
	commands[selected].execute(&input)
}
