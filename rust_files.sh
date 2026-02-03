#!/usr/bin/env bash
set -euo pipefail

FORCE=false

usage() {
  cat <<'EOF'
Usage:
  ./init_rust_files.sh
  ./init_rust_files.sh --force
EOF
}

if [[ "${1:-}" == "--help" || "${1:-}" == "-h" ]]; then
  usage
  exit 0
fi

if [[ "${1:-}" == "--force" ]]; then
  FORCE=true
elif [[ "${1:-}" != "" ]]; then
  echo "❌ Unknown argument: ${1}"
  usage
  exit 1
fi

write_file() {
  local path="$1"
  shift
  local content="$1"

  if [[ -e "$path" && "$FORCE" == "false" ]]; then
    echo "⚠️  Skipping (already exists): $path"
    return 0
  fi

  mkdir -p "$(dirname "$path")"

  cat > "$path" <<EOF
${content}
EOF

  echo "✅ Created: $path"
}

MISE_TOML_CONTENT='[tasks.lint]
run = "cargo clippy --all-targets --all-features"

[tasks.release]
run = "cargo build --release"

[tasks.build]
run = "cargo build"

[tasks.cli]
run = "cargo run"

[tasks.test]
run = "cargo test"

[tasks.check]
run = "cargo check"

[tasks.doc]
run = "cargo doc --no-deps"

[tasks.fmt]
run = "cargo fmt --all -- --check"

[tasks.changelog]
run = "git cliff -o CHANGELOG.md"
'

CHANGELOG_MD_CONTENT='# Changelog
'

CLIFF_TOML_CONTENT='# git-cliff ~ default configuration file
# https://git-cliff.org/docs/configuration
#
# Lines starting with "#" are comments.
# Configuration options are organized into tables and keys.
# See documentation for more information on available options.

[changelog]
# changelog header
header = """
# Changelog

"""
# template for the changelog body
# https://keats.github.io/tera/docs/#introduction
body = """
{% if version %}\
    ## [{{ version | trim_start_matches(pat="v") }}] - {{ timestamp | date(format="%Y-%m-%d") }}
{% else %}\
    ## [unreleased]
{% endif %}\
{% for group, commits in commits | group_by(attribute="group") %}
    ### {{ group | upper_first }}
    {% for commit in commits %}
        - {% if commit.breaking %}[**breaking**] {% endif %}{{ commit.message | upper_first }}\
    {% endfor %}
{% endfor %}\n
"""
trim = true
footer = """
"""
postprocessors = [
  # { pattern = '\''<REPO>'\'', replace = "https://github.com/orhun/git-cliff" },
]
[git]
conventional_commits = true
filter_unconventional = true
split_commits = false
commit_preprocessors = [
  # { pattern = '\''\((\w+\s)?#([0-9]+)\)'\'', replace = "([#${2}](<REPO>/issues/${2}))"},
]
commit_parsers = [
  { message = "^feat", group = "Features" },
  { message = "^fix", group = "Bug Fixes" },
  { message = "^doc", group = "Documentation" },
  { message = "^perf", group = "Performance" },
  { message = "^refactor", group = "Refactor" },
  { message = "^style", group = "Styling" },
  { message = "^test", group = "Testing" },
  { message = "^chore\\(deps\\)", group = "Dependency Updates" },
  { message = "^revert", group = "Revert" },
]
protect_breaking_commits = false
filter_commits = false
tag_pattern = "v[0-9].*"
skip_tags = "v0.1.0-beta.1"
ignore_tags = ""
topo_order = false
sort_commits = "newest"
'

RUSTFMT_TOML_CONTENT='hard_tabs = true
tab_spaces = 2
imports_granularity = "Item"
group_imports = "StdExternalCrate"
use_field_init_shorthand = true
use_try_shorthand = true
format_code_in_doc_comments = true
'

CI_YML_CONTENT='name: CI
on: [push]

jobs:
  build:
    runs-on: ubuntu-latest

    steps:
      - uses: actions/checkout@0c366fd6a839edf440554fa01a7085ccba70ac98

      - name: Mise install
        uses: jdx/mise-action@10eed64f1fcad9eea08c7c683b0a6fa8889d51e7
        with:
          cache: true
          experimental: true

      - uses: Swatinem/rust-cache@11da8522bc3856a8fbc565f1d1530989c793d67d

      - name: Run build
        run: mise build

      - name: Run tests
        run: mise test
'

write_file "mise.toml" "$MISE_TOML_CONTENT"
write_file "CHANGELOG.md" "$CHANGELOG_MD_CONTENT"
write_file "cliff.toml" "$CLIFF_TOML_CONTENT"
write_file ".rustfmt.toml" "$RUSTFMT_TOML_CONTENT"
write_file ".github/workflows/CI.yml" "$CI_YML_CONTENT"

echo "🎉 Done."