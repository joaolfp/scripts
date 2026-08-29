mod app;
mod commands;

use anyhow::Result;

fn main() -> Result<()> {
	let commands = commands::registry::all();

	loop {
		let selected = match app::show_menu(&commands) {
			Ok(s) => s,
			Err(e) if e.to_string().contains("interrupted") => return Ok(()),
			Err(e) => return Err(e),
		};

		let input = match app::get_user_input(&*commands[selected]) {
			Ok(i) => i,
			Err(e) if e.to_string().contains("interrupted") => return Ok(()),
			Err(e) => return Err(e),
		};

		if let Err(e) = commands[selected].execute(&input) {
			eprintln!("Error: {e:?}");
		}
	}
}
