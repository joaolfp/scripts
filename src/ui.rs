use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
	Frame,
	layout::{Constraint, Layout, Rect},
	style::{Color, Modifier, Style},
	text::{Line, Span},
	widgets::{Block, Borders, List, ListItem, Paragraph},
};

pub struct App {
	pub selected: usize,
	pub should_quit: bool,
}

impl App {
	pub fn new() -> Self {
		Self {
			selected: 0,
			should_quit: false,
		}
	}

	pub fn handle_input(&mut self) -> anyhow::Result<()> {
		if let Event::Key(key) = event::read()? {
			if key.kind == KeyEventKind::Press {
				match key.code {
					KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
					KeyCode::Down | KeyCode::Char('j') => self.next(),
					KeyCode::Up | KeyCode::Char('k') => self.previous(),
					KeyCode::Enter => return Ok(()),
					_ => {}
				}
			}
		}
		Err(anyhow::anyhow!("Continue"))
	}

	fn next(&mut self) {
		self.selected = (self.selected + 1).min(5);
	}

	fn previous(&mut self) {
		self.selected = self.selected.saturating_sub(1);
	}

	pub fn render(&self, frame: &mut Frame) {
		let area = frame.area();
		let chunks = Layout::vertical([
			Constraint::Length(3),
			Constraint::Min(0),
			Constraint::Length(3),
		])
		.split(area);

		self.render_header(frame, chunks[0]);
		self.render_menu(frame, chunks[1]);
		self.render_footer(frame, chunks[2]);
	}

	fn render_header(&self, frame: &mut Frame, area: Rect) {
		let title = Paragraph::new("Scripts Menu")
			.style(
				Style::default()
					.fg(Color::Cyan)
					.add_modifier(Modifier::BOLD),
			)
			.block(Block::default().borders(Borders::ALL));
		frame.render_widget(title, area);
	}

	fn render_menu(&self, frame: &mut Frame, area: Rect) {
		let items = vec![
			"Clone HeroesOfCode's repositories",
			"Releasor",
			"Create rust project",
			"Clone my repositories",
			"Install Xcode",
			"Upgrade mise",
		];

		let list_items: Vec<ListItem> = items
			.iter()
			.enumerate()
			.map(|(i, item)| {
				let style = if i == self.selected {
					Style::default()
						.fg(Color::Yellow)
						.add_modifier(Modifier::BOLD)
				} else {
					Style::default()
				};
				ListItem::new(Line::from(vec![
					Span::raw(format!("{}. ", i + 1)),
					Span::styled(*item, style),
				]))
			})
			.collect();

		let list = List::new(list_items).block(Block::default().borders(Borders::ALL).title("Options"));
		frame.render_widget(list, area);
	}

	fn render_footer(&self, frame: &mut Frame, area: Rect) {
		let help = Paragraph::new("↑/k: Up | ↓/j: Down | Enter: Select | q/Esc: Quit")
			.style(Style::default().fg(Color::DarkGray))
			.block(Block::default().borders(Borders::ALL));
		frame.render_widget(help, area);
	}
}
