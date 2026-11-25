---
description: TAA Vidyo Yapıcı açılış animasyonları (0.9.5). Sahip: Yusuf. Yazan: [Claude#Sonnet-5.5, 2026-11-17]. Yalnız yazan yapay zekâ ya da Yusuf'un izniyle değişir; silme yok.
---
# web/acilis
- Her `*.html` tek başına açılabilen bir açılış animasyonudur; program bunlardan birini rastgele iframe'de oynatır (`/acilis/<ad>.html?uygulama=1`). Liste hem `web/index.html` (`ACILISLAR`) hem `src/arayuz.rs` (`ACILIS`) içinde tutulur; test ikisinin uyumunu denetler.
- `logo.js`: **İkiz Kabartma** logosunun açılışlardaki TEK tanımı. Logo değişirse bu dosya, `logo/logo.svg` ve `web/index.html` içindeki `<symbol id="ikiz-kabartma">` güncellenir.
- `?uygulama=1`: "Tekrar oynat" gizlenir; tıklama/Esc/Geç ana sayfaya `taa-acilis-gec` iletisi gönderir.
- Kaynak ve üretici: `tasarim/acilis-tur2/uret_web_acilis.py`.
