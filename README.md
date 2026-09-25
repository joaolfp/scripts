> This tools was built for personal purpose, but feel free to contribute

```sh
cargo install --git https://github.com/joaolfp/scripts
```

## Run with Docker

```sh
docker build -t scripts .
docker run -it --rm scripts
```

## Add a new command

Anyone can add a new option to the menu — no Rust knowledge needed. Open [Claude Code](https://claude.com/claude-code) in this repository and run `/add-command` followed by what you want, in plain words or as the exact command:

```sh
/add-command update Homebrew
/add-command brew upgrade node
/add-command git clone https://github.com/heroesofcode/swift-explorer.git
/add-command install a Node version with `mise use -g node@<version>`, asking for the version
```

The agent will:

1. Pick the category (`Clone:`, `Install:`, `Update:` or other) and the menu label.
2. Write the code, register it in the menu and update the tests and docs.
3. Run the tests and lint.
4. Ask whether to publish — committing and pushing with `/commit-push`, then bumping the version with `/prepare-version`.

Try the new option with `mise cli`.
