#!/usr/bin/env bash
# Packages DraftCraft release binaries for Linux as a tarball, .deb, .rpm and AppImage.
#
#   packaging/linux/package.sh <x86_64|aarch64> <bin-dir> [out-dir]
#
# <bin-dir> holds release builds of `draftcraft` and `draftcraft-cli` for that architecture.
# Needs: dpkg-deb, rpmbuild (Debian/Ubuntu package `rpm`), desktop-file-validate
# (`desktop-file-utils`), curl. The AppImage step downloads
# appimagetool (pinned below) and runs it without FUSE.
set -euo pipefail

ARCH="${1:?usage: package.sh <x86_64|aarch64> <bin-dir> [out-dir]}"
BIN="$(cd "${2:?missing bin-dir}" && pwd)"
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
OUT="$(mkdir -p "${3:-$ROOT/dist}" && cd "${3:-$ROOT/dist}" && pwd)"
HERE="$ROOT/packaging/linux"
APPIMAGETOOL_VERSION="1.9.0"

case "$ARCH" in
  x86_64) DEB_ARCH=amd64 ;;
  aarch64) DEB_ARCH=arm64 ;;
  *) echo "unknown architecture: $ARCH" >&2; exit 2 ;;
esac

VERSION=$(awk -F'"' '/^version = /{print $2; exit}' "$ROOT/Cargo.toml")
NAME="draftcraft-linux-${ARCH}"
DOCS=(README.md LICENSE-MIT LICENSE-APACHE NOTICE ATTRIBUTION.md)
WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT

for b in draftcraft draftcraft-cli; do
  [ -x "$BIN/$b" ] || { echo "missing $BIN/$b" >&2; exit 1; }
done

# One file tree shared by the .deb, .rpm and AppImage: /usr/bin, the desktop entry, AppStream
# metadata and the icon (scalable SVG plus a 512 px PNG).
stage_usr() {
  local root="$1"
  install -Dm755 "$BIN/draftcraft" "$root/usr/bin/draftcraft"
  install -Dm755 "$BIN/draftcraft-cli" "$root/usr/bin/draftcraft-cli"
  install -Dm644 "$HERE/draftcraft.desktop" "$root/usr/share/applications/draftcraft.desktop"
  install -Dm644 "$HERE/io.github.hlophlopgaming.draftcraft.metainfo.xml" \
    "$root/usr/share/metainfo/io.github.hlophlopgaming.draftcraft.metainfo.xml"
  install -Dm644 "$ROOT/docs/images/icon.svg" "$root/usr/share/icons/hicolor/scalable/apps/draftcraft.svg"
  install -Dm644 "$ROOT/docs/images/icon-512.png" "$root/usr/share/icons/hicolor/512x512/apps/draftcraft.png"
  for d in "${DOCS[@]}"; do
    install -Dm644 "$ROOT/$d" "$root/usr/share/doc/draftcraft/$d"
  done
}

# --- tar.gz: the two binaries, docs and the desktop files, unpack-and-run ---
T="$WORK/$NAME"
mkdir -p "$T"
install -m755 "$BIN/draftcraft" "$BIN/draftcraft-cli" "$T/"
for d in "${DOCS[@]}"; do cp "$ROOT/$d" "$T/"; done
cp "$HERE/draftcraft.desktop" "$ROOT/docs/images/icon.svg" "$T/"
tar -C "$WORK" -czf "$OUT/$NAME.tar.gz" "$NAME"

# --- .deb ---
D="$WORK/deb"
stage_usr "$D"
mkdir -p "$D/DEBIAN"
SIZE=$(du -sk "$D/usr" | cut -f1)
cat > "$D/DEBIAN/control" <<EOF
Package: draftcraft
Version: ${VERSION}
Section: editors
Priority: optional
Architecture: ${DEB_ARCH}
Installed-Size: ${SIZE}
Depends: libc6, libgcc-s1, libxkbcommon0, libgl1 | libvulkan1
Recommends: libgtk-3-0, xdg-desktop-portal, espeak-ng | speech-dispatcher
Maintainer: DraftCraft Contributors <https://github.com/hlophlopgaming/draftcraft>
Homepage: https://hlophlopgaming.github.io/draftcraft/
Description: Open-source screenwriting app
 DraftCraft is a clean-room screenwriting editor, CLI and MCP server written in Rust,
 with industry-standard pagination and Final Draft (.fdx) and Fountain support.
EOF
dpkg-deb --build --root-owner-group "$D" "$OUT/$NAME.deb"

# --- .rpm (prebuilt binaries; no %build) ---
R="$WORK/rpm"
mkdir -p "$R"/{BUILD,RPMS,SOURCES,SPECS,SRPMS}
stage_usr "$R/root"
cat > "$R/SPECS/draftcraft.spec" <<EOF
Name:           draftcraft
Version:        ${VERSION}
Release:        1
Summary:        Open-source screenwriting app
License:        MIT OR Apache-2.0
URL:            https://hlophlopgaming.github.io/draftcraft/
Requires:       libxkbcommon
Recommends:     gtk3
Recommends:     xdg-desktop-portal
Recommends:     espeak-ng

%global debug_package %{nil}
%global _build_id_links none
%global __strip /bin/true

%description
DraftCraft is a clean-room screenwriting editor, CLI and MCP server written in Rust,
with industry-standard pagination and Final Draft (.fdx) and Fountain support.

%install
cp -a ${R}/root/. %{buildroot}/

%files
/usr/bin/draftcraft
/usr/bin/draftcraft-cli
/usr/share/applications/draftcraft.desktop
/usr/share/metainfo/io.github.hlophlopgaming.draftcraft.metainfo.xml
/usr/share/icons/hicolor/scalable/apps/draftcraft.svg
/usr/share/icons/hicolor/512x512/apps/draftcraft.png
%doc /usr/share/doc/draftcraft
EOF
rpmbuild --define "_topdir $R" --target "$ARCH" -bb "$R/SPECS/draftcraft.spec"
cp "$R"/RPMS/*/draftcraft-*.rpm "$OUT/$NAME.rpm"

# --- AppImage ---
A="$WORK/AppDir"
stage_usr "$A"
cp "$HERE/draftcraft.desktop" "$A/draftcraft.desktop"
cp "$ROOT/docs/images/icon-512.png" "$A/draftcraft.png"
ln -s draftcraft.png "$A/.DirIcon"
cat > "$A/AppRun" <<'EOF'
#!/bin/sh
HERE="$(dirname "$(readlink -f "$0")")"
case "$(basename "${ARGV0:-$0}")" in
  draftcraft-cli*) exec "$HERE/usr/bin/draftcraft-cli" "$@" ;;
esac
exec "$HERE/usr/bin/draftcraft" "$@"
EOF
chmod +x "$A/AppRun"
TOOL="$WORK/appimagetool"
curl -fsSL -o "$TOOL" \
  "https://github.com/AppImage/appimagetool/releases/download/${APPIMAGETOOL_VERSION}/appimagetool-${ARCH}.AppImage"
chmod +x "$TOOL"
APPIMAGE_EXTRACT_AND_RUN=1 ARCH="$ARCH" VERSION="$VERSION" "$TOOL" --no-appstream "$A" "$OUT/$NAME.AppImage"

ls -l "$OUT"/"$NAME".*
