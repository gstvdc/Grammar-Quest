#!/bin/sh

set -eu

ROOT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
APP="$ROOT_DIR/dist/Grammar Quest.app"
ICONSET="$ROOT_DIR/dist/Grammar Quest.iconset"
LOGO="$ROOT_DIR/crates/grammar_quest/assets/brand/grammar-quest-logo.png"
BIN="$ROOT_DIR/target/release/grammar_quest"
PLIST="$ROOT_DIR/scripts/macos/Info.plist"

if test -e "$APP"; then
    echo "Refusing to overwrite $APP. Move it aside and run this script again."
    exit 1
fi

mkdir -p "$APP/Contents/MacOS" "$APP/Contents/Resources" "$ICONSET"

for spec in \
    "16 icon_16x16.png" \
    "32 icon_16x16@2x.png" \
    "32 icon_32x32.png" \
    "64 icon_32x32@2x.png" \
    "128 icon_128x128.png" \
    "256 icon_128x128@2x.png" \
    "256 icon_256x256.png" \
    "512 icon_256x256@2x.png" \
    "512 icon_512x512.png" \
    "1024 icon_512x512@2x.png"; do
    set -- $spec
    sips -z "$1" "$1" "$LOGO" --out "$ICONSET/$2" >/dev/null
done

iconutil -c icns "$ICONSET" -o "$APP/Contents/Resources/AppIcon.icns"
cargo build --release -p grammar_quest
cp "$BIN" "$APP/Contents/MacOS/grammar_quest"
cp "$PLIST" "$APP/Contents/Info.plist"
mkdir -p "$APP/Contents/Resources/Licenses"
cp "$ROOT_DIR/crates/grammar_quest/assets/fonts/OFL.txt" \
    "$APP/Contents/Resources/Licenses/PressStart2P-OFL.txt"
cp "$ROOT_DIR/crates/grammar_quest/assets/ATTRIBUTION.md" \
    "$APP/Contents/Resources/Licenses/ATTRIBUTION.md"
codesign --force --deep --sign - "$APP"
codesign --verify --deep --strict "$APP"

rm -rf "$ICONSET"
echo "Created $APP"
