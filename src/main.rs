use duct::cmd;
use anyhow::Result;

fn main() -> Result<()> {
    // TODO: Tomorrow I need to add all commands I'm going to use
    cmd!("git", "status").run()?;
    Ok(())
}
