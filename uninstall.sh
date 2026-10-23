#!/usr/bin/env bash
# uninstall.sh - uninstaller for the Kiln programming language
# Usage: ./uninstall.sh [-y]
#   -y, --yes   don't ask for confirmation
# Removes the Kiln binary installed by install-kiln.sh and the cloned source
# in $KILN_SRC_DIR (default ~/.kiln/src). Does NOT remove git, Rust/cargo,
# or the repo checkout you ran the installer from.
set -euo pipefail

SRC_DIR="${KILN_SRC_DIR:-$HOME/.kiln/src}"
CARGO_BIN="${CARGO_HOME:-$HOME/.cargo}/bin"
ASSUME_YES=0

case "${1:-}" in
  -y|--yes) ASSUME_YES=1 ;;
  -h|--help) sed -n '2,7p' "$0" | sed 's/^# \{0,1\}//'; exit 0 ;;
  "") ;;
  *) echo "Unknown option: $1" >&2; exit 1 ;;
esac

if [ -t 1 ]; then
  YELLOW=$'\033[38;5;220m'; RED=$'\033[31m'; BOLD=$'\033[1m'; RESET=$'\033[0m'
else
  YELLOW=""; RED=""; BOLD=""; RESET=""
fi

info() { printf '%s==>%s %s\n' "$YELLOW" "$RESET" "$*"; }
fail() { printf '%sError:%s %s\n' "$RED" "$RESET" "$*" >&2; exit 1; }
have() { command -v "$1" >/dev/null 2>&1; }

# 1. Find what's installed
# The installer names the binary after the package in Cargo.toml (default "Kiln"),
# so match case-insensitively.
pkgs=()
if have cargo; then
  while IFS= read -r name; do
    [ -n "$name" ] && pkgs+=("$name")
  done < <(cargo install --list 2>/dev/null | sed -n 's/^\([Kk][Ii][Ll][Nn]\) v.*/\1/p')
fi

bins=()
for b in Kiln kiln; do
  [ -e "$CARGO_BIN/$b" ] || [ -L "$CARGO_BIN/$b" ] && bins+=("$CARGO_BIN/$b")
done

# Only delete the source dir if it really is a Kiln clone (guards a custom KILN_SRC_DIR)
remove_src=0
if [ -d "$SRC_DIR/.git" ] && { [ -f "$SRC_DIR/Cargo.toml" ] || [ -f "$SRC_DIR/Kiln/Cargo.toml" ]; }; then
  remove_src=1
fi

if [ "${#pkgs[@]}" -eq 0 ] && [ "${#bins[@]}" -eq 0 ] && [ "$remove_src" -eq 0 ]; then
  echo "Kiln doesn't appear to be installed. Nothing to do."
  exit 0
fi

# 2. Confirm
echo "This will remove:"
for p in "${pkgs[@]:-}"; do [ -n "$p" ] && echo "  - cargo package '$p'"; done
for b in "${bins[@]:-}"; do [ -n "$b" ] && echo "  - $b"; done
[ "$remove_src" -eq 1 ] && echo "  - $SRC_DIR (cloned source)"

if [ "$ASSUME_YES" -eq 0 ]; then
  read -r -p "Continue? [y/N] " reply
  case "$reply" in y|Y|yes|YES) ;; *) echo "Cancelled."; exit 0 ;; esac
fi

# 3. Remove
for p in "${pkgs[@]:-}"; do
  [ -n "$p" ] && { info "Running cargo uninstall $p..."; cargo uninstall "$p" || true; }
done

# Leftover binaries (cargo uninstall normally removes these already)
for b in "${bins[@]:-}"; do
  if [ -n "$b" ] && { [ -e "$b" ] || [ -L "$b" ]; }; then
    rm -f "$b" && info "Removed $b"
  fi
done

if [ "$remove_src" -eq 1 ]; then
  rm -rf "$SRC_DIR" && info "Removed $SRC_DIR"
  # Tidy up ~/.kiln if it is now empty
  rmdir "$(dirname "$SRC_DIR")" 2>/dev/null || true
fi

hash -r 2>/dev/null || true
printf '\n%sKiln has been uninstalled.%s\n' "$BOLD" "$RESET"
echo "Note: git and Rust/cargo were left installed. If you added $CARGO_BIN to your PATH, remove that line from your shell profile if you no longer need it."
