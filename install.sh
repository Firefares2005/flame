#!/usr/bin/env bash
set -e

INSTALL_DIR="$HOME/.local/bin"
BIN="$INSTALL_DIR/flame"

echo "Installing Flame..."

mkdir -p "$INSTALL_DIR"

OS="$(uname -s)"
case "$OS" in
  Linux*)  FILE="flame-linux" ;;
  Darwin*) FILE="flame-macos" ;;
  *)       echo "Unsupported OS: $OS"; exit 1 ;;
esac

URL="https://github.com/Firefares2005/flame/releases/latest/download/$FILE"

echo "Downloading $FILE..."
curl -L -o "$BIN" "$URL"
chmod +x "$BIN"

if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
  SHELL_RC="$HOME/.bashrc"
  [ -n "$ZSH_VERSION" ] && SHELL_RC="$HOME/.zshrc"
  echo "export PATH=\"$INSTALL_DIR:\$PATH\"" >> "$SHELL_RC"
  echo "Added to PATH in $SHELL_RC"
fi

echo ""
echo "Flame installed!"
echo "   Run: flame"