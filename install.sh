#!/usr/bin/env sh
# Build mrkdwn and install (or uninstall) it.
#
#   ./install.sh                  install to ~/.local/bin
#   PREFIX=/usr/local ./install.sh   install to /usr/local/bin (may need sudo)
#   ./install.sh --uninstall      remove the installed binary

set -eu

PREFIX="${PREFIX:-$HOME/.local}"
BIN_DIR="$PREFIX/bin"
TARGET="$BIN_DIR/mrkdwn"

cd "$(dirname "$0")"

if [ "${1:-}" = "--uninstall" ]; then
    if [ -e "$TARGET" ]; then
        rm -f "$TARGET"
        echo "Removed $TARGET"
    else
        echo "Nothing to remove: $TARGET does not exist"
    fi
    exit 0
fi

if ! command -v cargo >/dev/null 2>&1; then
    echo "error: cargo not found. Install Rust from https://rustup.rs" >&2
    exit 1
fi

echo "Building mrkdwn (release)..."
cargo build --release --locked

install -Dm755 target/release/mrkdwn "$TARGET"
echo "Installed $TARGET"

case ":$PATH:" in
    *":$BIN_DIR:"*) ;;
    *) echo "note: $BIN_DIR is not in your PATH. Add it to use 'mrkdwn' directly." ;;
esac
