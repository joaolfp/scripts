mod commands;
mod ui;

use anyhow::Result;
use crossterm::{
	execute,
	terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io::{self, Write};
use ui::App;

fn main() -> Result<()> {
	enable_raw_mode()?;
	let mut stdout = io::stdout();
	execute!(stdout, EnterAlternateScreen)?;

	let backend = CrosstermBackend::new(stdout);
	let mut terminal = Terminal::new(backend)?;

	let result = run_app(&mut terminal);

	disable_raw_mode()?;
	execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

	if let Err(e) = result {
		eprintln!("Error: {}", e);
	}

	Ok(())
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
	let mut app = App::new();

	loop {
		terminal.draw(|f| app.render(f))?;

		if app.handle_input().is_ok() {
			disable_raw_mode()?;
			execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

			let input = get_user_input(app.selected)?;

			if app.selected == 3 {
				commands::run_clone_my_repositories(&input)?;
				return Ok(());
			}

			if app.selected == 2 {
				commands::prepare_rust_project(&input)?;
			}

			if let Some(spec) = commands::command_spec(app.selected, &input)? {
				commands::run_in_terminal(&spec)?;
			}
			return Ok(());
		}

		if app.should_quit {
			break;
		}
	}

	Ok(())
}

fn get_user_input(selected: usize) -> Result<String> {
	if !matches!(selected, 1 | 2 | 3 | 4) {
		return Ok(String::new());
	}

	let prompt = match selected {
		1 => "Package name: ",
		2 => "Project name: ",
		3 => "Repository: ",
		4 => "Xcode version: ",
		_ => "",
	};

	print!("{}", prompt);
	io::stdout().flush()?;
	let mut input = String::new();
	io::stdin().read_line(&mut input)?;
	Ok(input.trim().to_string())
}
