#!/usr/bin/env bash
# Windows paketini üretir: dist/LED-Ekran-Video-Kurulum.exe (kurulum + kaldırma), dist/led-ekran-video.exe
# Gerekenler: rustup target x86_64-pc-windows-gnu, ziglang (pip install ziglang), cargo-zigbuild.
set -euo pipefail
cd "$(dirname "$0")"
HEDEF=x86_64-pc-windows-gnu
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$PWD/target}"
cargo zigbuild --release --target "$HEDEF"
PAKET="$(mktemp -d)"
cp "$CARGO_TARGET_DIR/$HEDEF/release/led-ekran-video.exe" "$PAKET/"
cp logo/simge.ico README.md "$PAKET/"
mkdir "$PAKET/senaryolar" "$PAKET/animasyonlar" && cp senaryolar/*.txt "$PAKET/senaryolar/" && cp animasyonlar/*.txt "$PAKET/animasyonlar/"
LED_PAKET="$PAKET" cargo zigbuild --release --target "$HEDEF" --manifest-path kurulum/Cargo.toml --target-dir "$CARGO_TARGET_DIR/kurulum"
mkdir -p dist
cp "$CARGO_TARGET_DIR/$HEDEF/release/led-ekran-video.exe" dist/
cp "$CARGO_TARGET_DIR/kurulum/$HEDEF/release/led-ekran-video-kurulum.exe" dist/LED-Ekran-Video-Kurulum.exe
echo "Hazır: dist/"; ls -la dist
