#!/bin/sh

set -eu

ROOT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
APP="$ROOT_DIR/dist/Grammar Quest.app"
DMG="$ROOT_DIR/dist/Grammar-Quest-v0.1.0-macos.dmg"
STAGING_DIR=$(mktemp -d /private/tmp/grammar-quest-dmg.XXXXXX)

cleanup() {
    rm -rf "$STAGING_DIR"
}
trap cleanup EXIT HUP INT TERM

if ! test -d "$APP"; then
    echo "Missing $APP. Run scripts/build-macos-app.sh first." >&2
    exit 1
fi

if test -e "$DMG"; then
    echo "Refusing to overwrite $DMG. Move it aside and run this script again." >&2
    exit 1
fi

ditto "$APP" "$STAGING_DIR/Grammar Quest.app"
ln -s /Applications "$STAGING_DIR/Applications"

hdiutil create \
    -volname "Grammar Quest" \
    -srcfolder "$STAGING_DIR" \
    -format UDZO \
    "$DMG"
hdiutil verify "$DMG"

echo "Created $DMG"
