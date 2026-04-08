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

This is a terminal UI (TUI) tool built with [ratatui](https://github.com/ratatui-org/ratatui) and crossterm. It presents a menu of developer scripts and runs the selected command.

**Flow:**
1. `main.rs` — initializes crossterm raw mode + alternate screen, creates the ratatui `Terminal`, runs the event loop via `run_app`, then cleans up terminal state on exit.
2. `ui.rs` — `App` struct holds selection state and `should_quit`. `render()` draws header/menu/footer. `handle_input()` returns `Ok(())` on Enter (selection confirmed) or sets `should_quit` on `q`/`Esc`.
3. `commands.rs` — maps the selected menu index (0–6) to a `CommandSpec` (program + args) or runs special logic directly. After the TUI exits, `main.rs` calls `get_user_input()` for items that need text input, then dispatches to the appropriate command function.

**Menu items and their indices:**
| Index | Label | Command/Logic |
|-------|-------|---------------|
| 0 | Clone HeroesOfCode's repositories | `hoc clone` |
| 1 | Releasor | `releasor -f <package>` |
| 2 | Create rust project | `cargo new <name>` + copies `rust_files.sh` |
| 3 | Clone my repositories | `xx::git::clone` from `github.com/joaolfp/<repo>` |
| 4 | Install Xcode | `xcodes install <version>` (or installs xcodes via brew) |
| 5 | Upgrade mise | `brew upgrade mise` |
| 6 | Release Rust | runs embedded `release-rust.sh` script |

`rust_files.sh` and `release-rust.sh` are embedded into the binary via `include_str!` and written to disk at runtime when needed.

## Code Style

Formatting is enforced via `.rustfmt.toml`: hard tabs, 2-space tab width, imports grouped as `StdExternalCrate`.
