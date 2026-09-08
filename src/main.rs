mod app;
mod commands;

use anyhow::Result;

fn main() -> Result<()> {
	app::run()
}
