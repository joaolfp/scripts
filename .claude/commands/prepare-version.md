---
description: Bump the crate version, refresh Cargo.lock via mise build, commit as "Prepare version to vX.Y.Z" and push (no git tag)
argument-hint: "<version>  e.g. 0.9.0  or  v0.9.0"
allowed-tools: Bash(git status:*), Bash(git diff:*), Bash(git add:*), Bash(git log:*), Bash(git commit:*), Bash(git push:*), Bash(git rev-parse:*), Bash(mise build:*), Bash(grep:*)
---

Prepare a new release version for this repo. A GitHub Action creates the git tag from the pushed commit, so **do not run `git tag`**.

## Input

Requested version: $ARGUMENTS

Normalize it:
- Accept `0.9.0` or `v0.9.0`.
- `CARGO_VERSION` = without the leading `v` (e.g. `0.9.0`) — used in `Cargo.toml`.
- `TAG_VERSION` = with a leading `v` (e.g. `v0.9.0`) — used in the commit message.
- If `$ARGUMENTS` is empty or not a valid `MAJOR.MINOR.PATCH`, stop and ask for the version.

## Context

- Current branch: !`git rev-parse --abbrev-ref HEAD`
- Working tree: !`git status --short`
- Current version: !`grep '^version' Cargo.toml`
- Recent version commits (style reference): !`git log --pretty=format:'%s' --grep='Prepare version' -5`

## Steps

1. If the working tree is not clean, stop and tell the user to commit or stash first (this commit must only touch `Cargo.toml` and `Cargo.lock`).
2. Confirm the current branch is `main`. If not, stop and ask.
3. Edit `Cargo.toml`: set the `[package]` `version` field to `CARGO_VERSION`. Only that line changes.
4. Run `mise build` so `Cargo.lock` picks up the new `scripts` package version.
5. `git add Cargo.toml Cargo.lock`. Verify with `git diff --cached --stat` that **only** those two files are staged and the diff is just the version change (plus any lock churn from the build). If anything else changed, stop and show the user.
6. Commit with exactly this subject, no body, no trailers:
   `Prepare version to <TAG_VERSION>`
7. `git push origin main`.
8. Report the commit hash and remind the user the tag `<TAG_VERSION>` will be created by the GitHub Action.
