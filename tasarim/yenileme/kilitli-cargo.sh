#!/usr/bin/env bash
# Açıklama: Aynı anda tek ağır derleme kuralı için cargo sarmalayıcısı. Kullanım: tasarim/yenileme/kilitli-cargo.sh test --release   (yalnız cargo alt komutları)
# Sahip: Yusuf. Yazan: [Claude#Sonnet-5.5, 2026-11-18]. Yalnız yazan yapay zekâ ya da Yusuf'un izniyle değişir; silme yok.
set -u
KILIT="${TEMP:-/tmp}/vidyo-cargo.kilit"
for i in $(seq 1 360); do
  if mkdir "$KILIT" 2>/dev/null; then break; fi
  # 20 dk'dan eski kilit takılı kalmıştır
  if [ -d "$KILIT" ] && [ -n "$(find "$KILIT" -maxdepth 0 -mmin +20 2>/dev/null)" ]; then rmdir "$KILIT" 2>/dev/null; fi
  sleep 5
done
trap 'rmdir "$KILIT" 2>/dev/null' EXIT
cd "$(dirname "$0")/../.." && cargo "$@"
