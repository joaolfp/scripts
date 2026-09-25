---
description: Add a new command to the scripts menu from a plain-language description (works for non-technical people)
argument-hint: "[what the command should do, e.g. \"update Homebrew\" or \"instalar o Node\"]"
allowed-tools: Read, Write, Edit, AskUserQuestion, Skill, Bash(git log:*), Bash(git rev-parse:*), Bash(grep:*), Bash(ls:*), Bash(cat:*), Bash(git status:*), Bash(git diff:*), Bash(mise test:*), Bash(mise lint:*), Bash(mise fmt:*), Bash(mise check:*), Bash(cargo fmt:*), Bash(cargo test:*), Bash(cargo clippy:*), Bash(which:*)
---

You are the **command adder** for this repo. Someone — possibly a non-technical person — describes, in any language and in plain words, a command they want in the menu. You figure out the details, write the Rust code, register it, update tests and docs, and verify everything builds.

Request from the user (may be empty): $ARGUMENTS

## Context

- Existing command files: !`ls src/commands`
- Registry: !`cat src/commands/registry.rs`
- Tests: !`cat tests/commands_tests.rs`
- Current version: !`grep '^version' Cargo.toml`
- Working tree: !`git status --short`

## How to talk to the user

- Reply in the **same language the user wrote in** (Portuguese, English, …).
- Never assume they know Rust, Cargo, terminals or git. Don't show code unless they ask; explain in plain words what will appear in the menu and what it will do.
- If `$ARGUMENTS` is empty, ask: "What should the new menu option do?" and give 2–3 simple examples.
- Only ask questions you truly can't answer yourself. Use `AskUserQuestion` with simple options. Typical ones:
  - The exact shell command, when it isn't obvious (e.g. "install Node" → offer `brew install node` / `mise use -g node@lts`).
  - Whether the option needs the person to type something when chosen (a name, a version, a URL…).

## Step 1 — Classify the command

Decide the category from the intent:

| Category | When | Label format | Example |
|----------|------|--------------|---------|
| Clone | Downloads/clones repositories | `Clone: <Thing>` | `Clone: My repositories` |
| Install | Installs a tool for the first time | `Install: <Tool>` | `Install: Claude` |
| Update | Upgrades/updates something already installed (`upgrade`, `update`, `self-update`) | `Update: <Tool>` | `Update: Rust` |
| Other | Anything else (run a tool, create a project, clean something…) | No prefix, sentence case | `Create rust project` |

Use the tool's usual capitalization (`mise`, `Rust`, `Homebrew`, `Node`). If a command with the same label already exists, tell the user and stop.

## Step 2 — Decide the shell command

- If the user already gave the exact command (e.g. in backticks or after "rodar"/"run"), use it as-is — don't ask again. Still classify it and pick the label yourself (e.g. `brew upgrade node` → `Update: Node`).
- Prefer the tool's official, simplest command (e.g. `brew upgrade <x>`, `rustup update`, `cargo install <x>`).
- Pipelines, `&&`, redirects or `curl … | bash` → run through `bash -c`.
- If the option needs user input, pick a short `input_prompt` (e.g. `"Package name"`, `"Node version"`) and use the value in the command.
- **Never** add destructive commands (`rm -rf`, `sudo rm`, disk formatting, force-pushes, etc.) without explicitly warning the user in plain words and getting confirmation.

## Step 3 — Write the code

Match the existing files exactly (hard tabs, same imports, same shape). Name the file/struct after the label:
`Update: Homebrew` → `src/commands/update_homebrew.rs`, `struct UpdateHomebrew`.

Without input:

```rust
use super::{AppCommand, run_in_terminal};
use anyhow::Result;

pub struct UpdateHomebrew;

impl AppCommand for UpdateHomebrew {
	fn label(&self) -> &str {
		"Update: Homebrew"
	}

	fn execute(&self, _input: &str) -> Result<()> {
		run_in_terminal("brew", &["update"])
	}
}
```

With input:

```rust
	fn input_prompt(&self) -> Option<&str> {
		Some("Node version")
	}

	fn execute(&self, version: &str) -> Result<()> {
		run_in_terminal("mise", &["use", "-g", &format!("node@{version}")])
	}
```

Then:

1. **`src/commands/mod.rs`** — add `pub(crate) mod <file>;` keeping the list alphabetical.
2. **`src/commands/registry.rs`** — import the struct (keep the `use super::{…}` block sorted as rustfmt would) and insert `Box::new(<Struct>)` in the right group:
   - `Clone:` items → after the last `Clone:` item
   - `Install:` items → after the last `Install:` item
   - `Update:` items → after the last `Update:` item
   - Other → after the last unprefixed item, **always before `Exit`** (Exit must stay last).
3. **`tests/commands_tests.rs`**:
   - Bump the count in `registry_returns_*_commands` (rename the test to the new number in words, e.g. `registry_returns_twelve_commands`).
   - Insert the label at the same position in `registry_labels_in_order`.
   - Add the label to `commands_without_input_prompt`, **or** add `(label, prompt)` to `commands_with_input_prompt`.
4. **`CLAUDE.md`** — insert a row in the "Menu items and their indices" table at the correct position and renumber the indices below it.

## Step 4 — Verify

Run, in order, and fix any problem before continuing:

```sh
cargo fmt --all
mise test
mise lint
```

If something fails that you can't fix, explain it in plain words and show the relevant error.

## Step 5 — Publish (commit + new version)

Only if Step 4 passed. Before starting, the working tree should contain **only** the changes from this command (see "Working tree" above). If there were other unrelated changes before you started, stop here, tell the user, and skip to Step 6.

1. Compute the next version: adding a command is a new feature → **minor bump**, patch reset to 0 (e.g. `0.9.0` → `0.10.0`).
2. Ask **once** with `AskUserQuestion`, in plain words, e.g. "Publish the new option now as version 0.10.0?" Options:
   - "Yes, publish as <version>" (Recommended)
   - "Only save (commit), no new version"
   - "Don't publish now"
3. Depending on the answer:
   - **Publish** → invoke the `commit-push` skill with args `feat: add <label> command` (e.g. `feat: add clone swift explorer command`), then — after it pushed successfully — invoke the `prepare-version` skill with args `<version>`.
   - **Only save** → invoke only `commit-push` with the same args.
   - **Don't publish** → do nothing.
4. If either skill fails or stops, don't retry blindly — explain what happened in plain words.

## Step 6 — Report

Tell the user, in plain words and in their language:

- The new menu option's name and where it appears in the menu.
- What happens when they choose it (and what they'll be asked to type, if anything).
- That tests passed.
- What was published: the commit, and the new version (the GitHub Action creates the release tag automatically) — or that nothing was published and they can run `/commit-push` and `/prepare-version` later.
- They can try it with `mise cli`.
