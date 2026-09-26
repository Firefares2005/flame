#!/usr/bin/env bash
# Flame installer for Linux / macOS
set -e

REPO="Firefares2005/flame"
INSTALL_DIR="$HOME/.local/bin"
BIN="$INSTALL_DIR/flame"

echo "Installing Flame..."

mkdir -p "$INSTALL_DIR"

OS="$(uname -s)"
case "$OS" in
  Linux*)  PATTERN="linux" ;;
  Darwin*) PATTERN="macos" ;;
  *)       echo "Unsupported OS: $OS"; exit 1 ;;
esac

URL=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" \
  | grep "browser_download_url" \
  | grep "$PATTERN" \
  | head -n 1 \
  | cut -d '"' -f 4)

if [ -z "$URL" ]; then
  echo "No binary found for $OS"
  exit 1
fi

echo "Downloading from $URL"
curl -L -o "$BIN" "$URL"
chmod +x "$BIN"

if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
  SHELL_RC="$HOME/.bashrc"
  [ -n "$ZSH_VERSION" ] && SHELL_RC="$HOME/.zshrc"
  echo "export PATH=\"$INSTALL_DIR:$PATH\"" >> "$SHELL_RC"
  echo "Added to PATH in $SHELL_RC"
fi

echo ""
echo "Flame installed!"
echo "   Run: flame"
