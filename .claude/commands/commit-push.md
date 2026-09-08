---
description: Commit staged/unstaged changes and push straight to main following the repo commit conventions
argument-hint: "[optional: extra context / hint about the change]"
allowed-tools: Bash(git status:*), Bash(git diff:*), Bash(git add:*), Bash(git log:*), Bash(git commit:*), Bash(git push:*), Bash(git branch:*), Bash(git rev-parse:*)
---

Automate a commit + push directly to `main` for this repo.

## Context

- Current branch: !`git rev-parse --abbrev-ref HEAD`
- Status: !`git status --short`
- Staged diff: !`git diff --cached --stat`
- Unstaged diff: !`git diff --stat`
- Recent commits (for style reference): !`git log --pretty=format:'%s' -15`

Extra hint from the user (may be empty): $ARGUMENTS

## Commit message conventions used in this repo

Look at the recent commits above. The rules are:

- **Conventional Commits**, lowercase after the type: `feat:`, `fix:`, `refactor:`, `chore:`, `docs:`.
- Optional scope in parentheses: `feat(theme): ...`, `chore(deps): ...`.
- Short, imperative, English description. No trailing period.
- One logical change per commit. Do **not** add `Co-Authored-By` or other trailers — keep the message a single line matching the existing history.
- Never use `Prepare version to vX.Y.Z` here — that belongs to `/prepare-version`.

## Steps

1. If there are no changes at all, stop and say so.
2. Review the full diff (`git diff` and `git diff --cached`) to understand what changed.
3. If nothing is staged, stage all tracked changes with `git add -A`. If only part of the tree should go in, stage selectively based on the user's hint.
4. Choose the single best conventional-commit subject line. Use the user's hint if provided.
5. If the changes clearly split into unrelated groups, make more than one commit, each staged and messaged separately.
6. Commit.
7. Confirm the local branch is `main` (this repo commits straight to main). If it is not `main`, stop and ask before pushing.
8. `git push origin main`.
9. Report the commit hash(es) and the push result.
