#!/bin/sh

set -eu

ROOT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
LOGO="$ROOT_DIR/crates/grammar_quest/assets/brand/grammar-quest-logo.png"
APP="$ROOT_DIR/dist/Grammar Quest.app"

test -f "$LOGO"
test -f "$ROOT_DIR/crates/grammar_quest/build.rs"
grep -q 'WINDOW_ICON_BIG' "$ROOT_DIR/crates/grammar_quest/build.rs"

grep -q 'grammar-quest-logo.png' "$ROOT_DIR/README.md"
test -x "$ROOT_DIR/scripts/build-macos-app.sh"
test -x "$APP/Contents/MacOS/grammar_quest"
test -f "$APP/Contents/Resources/AppIcon.icns"
plutil -lint "$APP/Contents/Info.plist" >/dev/null
grep -q '<string>Grammar Quest</string>' "$APP/Contents/Info.plist"

echo "Branding assets and macOS app bundle are valid."
