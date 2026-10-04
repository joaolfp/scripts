#!/usr/bin/env bash
set -euo pipefail

echo "🦀 Installing Rust..."
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# shellcheck disable=SC1091
source "$HOME/.cargo/env"

echo "📦 Installing scripts..."
cargo install --git https://github.com/joaolfp/scripts

echo "✅ Done."
