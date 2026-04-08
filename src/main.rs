mod commands;
mod ui;

use anyhow::Result;

fn main() -> Result<()> {
	let selected = ui::show_menu()?;
	let input = ui::get_user_input(selected)?;
	commands::dispatch(selected, &input)
}
