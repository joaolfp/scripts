use anyhow::Result;
use duct::cmd;
use std::io::{self, Write};

fn main() -> Result<()> {
    println!();
    println!("1 - Clone HeroesOfCode's repositories");
    println!("2 - Releasor");
    println!("3 - Create rust project");
    println!("q - Quit");

    println!();
    println!("Choose an option: ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let option = input.trim();

    match option {
        "1" => {
            cmd!("hoc", "clone").run()?;
        }
        "2" => {
            setup_releasor()?;
        }
        "3" => {
            setup_rust_project()?;
        }
        "q" | "quit" | "exit" => {
            println!("Bye 👋");
            return Ok(());
        }
        _ => {
            println!("❌ This option does not exist");
        }
    }

    Ok(())
}

fn setup_releasor() -> Result<()> {
    let package_name = input_name(String::new(), "What's the package name: ")?;
    cmd!("releasor", "-f", package_name).run()?;
    Ok(())
}

fn setup_rust_project() -> Result<()> {
    let project_name = input_name(String::new(), "What's the project name: ")?;
    cmd!(
        "bash",
        "-c",
        format!("cd .. && cargo new {} && cd scripts", project_name)
    )
    .run()?;
    Ok(())
}

fn input_name(mut input_name: String, message: &str) -> Result<String> {
    println!();
    println!("{}", message);

    io::stdin().read_line(&mut input_name)?;

    let value = input_name.trim();
    Ok(value.to_string())
}
