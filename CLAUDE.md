# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

All tasks are defined in `mise.toml` and run via `mise <task>`:

```sh
mise build       # cargo build
mise release     # cargo build --release
mise cli         # cargo fmt --all, then cargo run (launch the TUI)
mise test        # cargo test
mise check       # cargo check
mise lint        # cargo clippy --all-targets --all-features
mise fmt         # cargo fmt --all -- --check
mise doc         # cargo doc --no-deps
mise changelog   # git cliff -o CHANGELOG.md
```

To run a single test: `cargo test <test_name>`

To add a new menu command from a plain-language description, use the `/add-command` slash command (`.claude/commands/add-command.md`).

## Architecture

This is a CLI tool that presents an interactive menu of developer scripts using [dialoguer](https://github.com/console-rs/dialoguer). The selected command is then executed.

The project is a Cargo workspace (root `Cargo.toml` holds `[workspace.package]` version/edition and shared `[workspace.dependencies]`) with three crates under `crates/`:

| Crate | Kind | Contents |
|-------|------|----------|
| `scripts-core` | lib | `AppCommand` trait and the shared `run_in_terminal` helper |
| `scripts-commands` | lib | One file per command, plus `all()` in `lib.rs` returning the ordered list of boxed commands |
| `scripts` | bin | `main.rs` + `app.rs` — the interactive menu |

Dependency direction: `scripts` → `scripts-commands` → `scripts-core`.

**Flow:**
1. `crates/scripts/src/main.rs` — calls `app::run()`.
2. `crates/scripts/src/app.rs` — gets the list from `scripts_commands::all()`, renders a `dialoguer::Select` menu (`show_menu()`), a `dialoguer::Input` prompt if the command has an `input_prompt` (`get_user_input()`), then calls `execute()` on the selected command.
3. `crates/scripts-core/src/lib.rs` — defines the `AppCommand` trait (`label`, `input_prompt`, `execute`) and `run_in_terminal`.
4. `crates/scripts-commands/src/lib.rs` — declares the command modules and `all()`; re-exports `AppCommand`.

**`AppCommand` trait:**
```rust
pub trait AppCommand {
    fn label(&self) -> &str;
    fn input_prompt(&self) -> Option<&str> { None }
    fn execute(&self, input: &str) -> Result<()>;
}
```

**Menu items and their indices:**

Labels are grouped by category prefix (`Clone:`, `Install:`, `Update:`) so related actions sit together in the `Select` list; standalone commands are unprefixed and listed last.

| Index | Label | Command/Logic |
|-------|-------|---------------|
| 0 | Clone: HeroesOfCode's repositories | `hoc clone` |
| 1 | Clone: My repositories | `hoc m` |
| 2 | Install: Releasor | `cargo install releasor` |
| 3 | Install: Claude | `curl -fsSL https://claude.ai/install.sh \| bash` |
| 4 | Install: Kiro | `curl -fsSL https://cli.kiro.dev/install \| bash` |
| 5 | Update: mise | `brew upgrade mise` |
| 6 | Update: Rust | `rustup update` |
| 7 | Update: Claude | `claude update` |
| 8 | Releasor | `releasor -f <package>` |
| 9 | Create rust project | `cargo new <name>` + copies `rust_files.sh` |
| 10 | Exit | prints `Bye bye 👋`, then `std::process::exit(0)` — quits the app |

`crates/scripts-commands/rust_files.sh` is embedded into the binary via `include_str!` and written to disk at runtime when needed.

## Testing

Integration tests live in `crates/scripts-commands/tests/commands_tests.rs` and use `scripts_commands::all()`. They verify the registry length, label order, and which commands require an input prompt.

## Code Style

Formatting is enforced via `.rustfmt.toml`: hard tabs, 2-space tab width, imports grouped as `StdExternalCrate`.
