#!/usr/bin/env bash
# install-kiln.sh - installer for the Kiln programming language
# Repo: https://github.com/Azure-6-lgtm/Kiln-language
set -euo pipefail

REPO_URL="https://github.com/Azure-6-lgtm/Kiln-language.git"
SRC_DIR="${KILN_SRC_DIR:-$HOME/.kiln/src}"

if [ -t 1 ]; then
  ORANGE=$'\033[38;5;208m'; YELLOW=$'\033[38;5;220m'; DIM=$'\033[2m'
  RED=$'\033[31m'; BOLD=$'\033[1m'; RESET=$'\033[0m'
else
  ORANGE=""; YELLOW=""; DIM=""; RED=""; BOLD=""; RESET=""
fi

logo() {
  #printf '%s' "$ORANGE"
  cat <<'LOGO'

        ,-~~~-.
      ,'  ) (  '.
     /   (   )   \
    |   .-----.   |
    |  /  ~ ~  \  |
  __|_|_________|_|__

 _    _ _
| | _(_) |_ __
| |/ / | | '_ \
|   <| | | | | |
|_|\_\_|_|_| |_|
   source in, solid out

  Kiln version 0.1.0 alpha pre-release

LOGO
 # printf '%s' "$RESET"
}

info() { printf '%s==>%s %s\n' "$YELLOW" "$RESET" "$*"; }
fail() { printf '%sError:%s %s\n' "$RED" "$RESET" "$*" >&2; exit 1; }
have() { command -v "$1" >/dev/null 2>&1; }

install_pkgs() {
  # $@ = logical needs: git, rust
  if [ -n "${TERMUX_VERSION:-}" ] || [ -d /data/data/com.termux ]; then
    pkg install -y git rust
  elif have apt-get; then
    sudo apt-get update && sudo apt-get install -y git cargo
  elif have dnf; then
    sudo dnf install -y git cargo
  elif have pacman; then
    sudo pacman -S --noconfirm git rust
  elif have brew; then
    brew install git rust
  else
    return 1
  fi
}

logo

# 1. Dependencies (only the Rust toolchain is needed)
if ! have cargo; then
  info "Installing dependencies (git, Rust/cargo)..."
  if ! install_pkgs; then
    if have curl; then
      info "No supported package manager found, using rustup..."
      curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
      # shellcheck disable=SC1091
      . "$HOME/.cargo/env"
    else
      fail "Please install git and Rust (cargo) manually, then re-run."
    fi
  fi
fi
have cargo || fail "cargo is still missing."

# 2. Locate the Kiln source (this script ships with the repo)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]:-$0}")" && pwd)"

find_project() {
  local d
  for d in "$SCRIPT_DIR" "$SCRIPT_DIR/Kiln" "$SCRIPT_DIR/.."; do
    if [ -f "$d/Cargo.toml" ] && grep -qi 'kiln' "$d/Cargo.toml"; then
      (cd "$d" && pwd)
      return 0
    fi
  done
  return 1
}

if ! PROJECT_DIR="$(find_project)"; then
  # Fallback: script was run on its own (e.g. downloaded or piped), so fetch the repo.
  have git || fail "git is required to download Kiln. Please install it and re-run."
  mkdir -p "$(dirname "$SRC_DIR")"
  if [ -d "$SRC_DIR/.git" ]; then
    info "Updating Kiln source in $SRC_DIR..."
    git -C "$SRC_DIR" pull --ff-only
  else
    info "Cloning Kiln into $SRC_DIR..."
    git clone --depth 1 "$REPO_URL" "$SRC_DIR"
  fi
  if [ -f "$SRC_DIR/Cargo.toml" ]; then
    PROJECT_DIR="$SRC_DIR"
  elif [ -f "$SRC_DIR/Kiln/Cargo.toml" ]; then
    PROJECT_DIR="$SRC_DIR/Kiln"
  else
    fail "Could not find Cargo.toml in the Kiln repository."
  fi
fi
info "Using Kiln source at $PROJECT_DIR"

# 3. Build and install
BIN_NAME="$(sed -n 's/^name *= *"\(.*\)".*/\1/p' "$PROJECT_DIR/Cargo.toml" | head -1)"
BIN_NAME="${BIN_NAME:-Kiln}"

info "Building Kiln (release mode)..."
cargo install --path "$PROJECT_DIR" --force

CARGO_BIN="${CARGO_HOME:-$HOME/.cargo}/bin"
case ":$PATH:" in
  *":$CARGO_BIN:"*) ;;
  *)
    printf '\n%sNote:%s add this to your shell profile so Kiln is on your PATH:\n' "$BOLD" "$RESET"
    printf '  export PATH="%s:$PATH"\n' "$CARGO_BIN"
    ;;
esac

printf '\n%sKiln installed successfully!%s\n' "$BOLD" "$RESET"
printf 'Check the version:  %s --ver\n' "$BIN_NAME"
printf 'Run a program:      %s file.kiln\n\n' "$BIN_NAME"

