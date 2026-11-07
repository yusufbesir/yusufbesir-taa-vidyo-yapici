# LED Ekran Video (TAA Paket programı, tek bilgisayarda kullanılır)

Bina dışı LED ekranlar (1800x980, eşlenik) için 3B yazı ve çizimli, karanlıkta okunur MP4/HAP video üretir.

## Kullanım
```
led-ekran-video --mod 3d --yazi "PEŞİN FİYATINA 3 TAKSİT|VADE FARKSIZ 3 TAKSİT|@cicek|TERZİ ADEM ALTUN" --sure 5 --cikis kampanya.mp4
```
- `--yazi` mesajları `|` ile ayırır; her mesaj ayrı sahne. `@cicek`, `@papatya`, `@yildiz`, `@rozet`, `@cerceve`, `@elmas` çizim sahnesidir.
- `--palet sari,cyan,turuncu,lime,magenta,beyaz` mesajlara sırayla renk verir (siyah zemin).
- `--kodek mp4` (varsayılan, her kare anahtar kare, CRF 12) veya `hap`.
- Düz modlar: `--mod sabit|kayan`. `--yardim` ile tüm seçenekler.
- Geçiş: dönerek giriş, sallanarak bekleme, dönerek çıkış. Ekrana açılı bakanda yan yüzey okunur kalsın diye ekstrüzyon hep görünür.
- Render CPU'da çalışır: 15 sn 1800x980 video yaklaşık 2 dk.

## Durum
Çekirdek hazır, testli (8 test). Eksik: arayüz, logo, kurulum/kaldırma exe, paket kaydı (`program.toml`).
