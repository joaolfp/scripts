# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

All tasks are defined in `mise.toml` and run via `mise <task>`:

```sh
mise build       # cargo build
mise release     # cargo build --release
mise cli         # cargo run (launch the TUI)
mise test        # cargo test
mise check       # cargo check
mise lint        # cargo clippy --all-targets --all-features
mise fmt         # cargo fmt --all -- --check
mise doc         # cargo doc --no-deps
mise changelog   # git cliff -o CHANGELOG.md
```

To run a single test: `cargo test <test_name>`

## Architecture

This is a CLI tool that presents an interactive menu of developer scripts using [dialoguer](https://github.com/console-rs/dialoguer). The selected command is then executed.

**Flow:**
1. `main.rs` — calls `commands::registry::all()` to get the command list, delegates to `app::show_menu()` for selection, `app::get_user_input()` for any required text input, then calls `execute()` on the selected command.
2. `app.rs` — `show_menu()` renders a `dialoguer::Select` prompt. `get_user_input()` renders a `dialoguer::Input` prompt if the command has an `input_prompt`.
3. `commands/mod.rs` — defines the `AppCommand` trait (`label`, `input_prompt`, `execute`) and the shared `run_in_terminal` helper.
4. `commands/registry.rs` — `all()` returns the ordered list of boxed commands.
5. `lib.rs` — exposes `pub mod commands` for use in integration tests.

**`AppCommand` trait:**
```rust
pub trait AppCommand {
    fn label(&self) -> &str;
    fn input_prompt(&self) -> Option<&str> { None }
    fn execute(&self, input: &str) -> Result<()>;
}
```

**Menu items and their indices:**
| Index | Label | Command/Logic |
|-------|-------|---------------|
| 0 | Clone HeroesOfCode's repositories | `hoc clone` |
| 1 | Releasor | `releasor -f <package>` |
| 2 | Install Releasor | `cargo install releasor` |
| 3 | Create rust project | `cargo new <name>` + copies `rust_files.sh` |
| 4 | Clone my repositories | `xx::git::clone` from `github.com/joaolfp/<repo>` |
| 5 | Install Xcode | `xcodes install <version>` (or installs xcodes via brew) |
| 6 | Update mise | `brew upgrade mise` |
| 7 | Update Rust | `rustup update` |
| 8 | Update Claude | `claude update` |

`rust_files.sh` and `release-rust.sh` are embedded into the binary via `include_str!` and written to disk at runtime when needed.

## Testing

Integration tests live in `tests/commands_tests.rs` and import via the `scripts` crate (`lib.rs`). They verify the registry length, label order, and which commands require an input prompt.

## Code Style

Formatting is enforced via `.rustfmt.toml`: hard tabs, 2-space tab width, imports grouped as `StdExternalCrate`.
