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

  # Ensure parent dir exists (in case you use nested paths later)
  mkdir -p "$(dirname "$path")"

  # Write content exactly as provided
  cat > "$path" <<EOF
${content}
EOF

  echo "✅ Wrote: $path"
}

MISE_TOML_CONTENT='[tasks.lint]
run = "cargo clippy --all-targets --all-features"

[tasks.release]
run = "cargo build --release"

[tasks.build]
run = "cargo build"

[tasks.cli]
run = "cargo run -- -f releasor"

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
# remove the leading and trailing whitespace from the template
trim = true
# changelog footer
footer = """
"""
# postprocessors
postprocessors = [
  # { pattern = '\''<REPO>'\'', replace = "https://github.com/orhun/git-cliff" }, # replace repository URL
]
[git]
# parse the commits based on https://www.conventionalcommits.org
conventional_commits = true
# filter out the commits that are not conventional
filter_unconventional = true
# process each line of a commit as an individual commit
split_commits = false
# regex for preprocessing the commit messages
commit_preprocessors = [
  # { pattern = '\''\((\w+\s)?#([0-9]+)\)'\'', replace = "([#${2}](<REPO>/issues/${2}))"}, # replace issue numbers
]
# regex for parsing and grouping commits
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
# protect breaking changes from being skipped due to matching a skipping commit_parser
protect_breaking_commits = false
# filter out the commits that are not matched by commit parsers
filter_commits = false
# regex for matching git tags
tag_pattern = "v[0-9].*"

# regex for skipping tags
skip_tags = "v0.1.0-beta.1"
# regex for ignoring tags
ignore_tags = ""
# sort the tags topologically
topo_order = false
# sort the commits inside sections by oldest/newest order
sort_commits = "newest"
# limit the number of commits included in the changelog.
# limit_commits = 42
'

RUSTFMT_TOML_CONTENT='hard_tabs = true
tab_spaces = 2
imports_granularity = "Item"
group_imports = "StdExternalCrate"
use_field_init_shorthand = true
use_try_shorthand = true
format_code_in_doc_comments = true
'

write_file "mise.toml" "$MISE_TOML_CONTENT"
write_file "CHANGELOG.md" "$CHANGELOG_MD_CONTENT"
write_file "cliff.toml" "$CLIFF_TOML_CONTENT"
write_file ".rustfmt.toml" "$RUSTFMT_TOML_CONTENT"

echo "🎉 Done."
