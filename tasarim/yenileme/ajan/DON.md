---
description: Dönüştürücü ajanı görevi (dalga 3). Sahip: Yusuf. Yazan: [Claude#Sonnet-5.5 koordinatör, 2026-11-25]. Silme yok.
---
# Dönüştürücü ajanı
Videoyu çizgifilm/animasyon/piksel art'a çeviren araç. Sahip olduğun: yeni src/donustur.rs, src/donustur_http.rs, web/donustur.js, tests/donustur*, docs/DONUSTURUCU.md (src/lib.rs'e yalnız `pub mod`).
- Rust: isle(yontem, yol, govde, klasor) -> (kod, tür, gövde) (UI ajanı /donustur/ rotasını arayuz.rs'e bağlar; imzayı tasarim/yenileme/ISTEK-donusturucu.md'ye yaz). ffmpeg: src/ffmpeg.rs ve LED_FFMPEG/yan klasör/PATH bulma; ffmpeg yoksa net Türkçe mesaj. ffmpeg'i HER ZAMAN argüman dizisiyle çağır (kabuk yok, komut enjeksiyonu yok).
- Dönüştürme türleri (filtre zincirleri; birkaçını gerçek ffmpeg ile dene): çizgifilm (kenar + renk azaltma/posterize + yumuşatma), çizgi roman/halftone, boyama benzeri, piksel art (küçült + palet kısıtla + dithering seçenekli + büyüt; LED 950x1800), duotone, kontur/karakalem, retro 8-bit; neon OLMAYAN premium tonlar. 950x1800 dikey çıkış (kırp/sığdır/doldur), süre/FPS/yoğunluk/palet parametreleri.
- Hızlı tek kare PNG önizleme; ilerlemeli iş (başlat/durum/iptal; /uret mantığına benzer); çıktı program çıktı klasörüne; kaynak seçimi (çıktı klasörü ve kullanıcı klasörleri; yol geçişi koruması, yalnız video/resim uzantıları, boyut sınırı). Düşük öncelik: tek resim → hareketli piksel/çizgi giriş animasyonu.
- web/donustur.js: window.TaaDonustur = { mount(el, app) }: kaynak seç, tür kartları (küçük önizleme), parametre sürgüleri, önizle, başlat/ilerleme/iptal, bitince "Oynat / Klasörü aç / Oynatıcıya ekle"; ≥48 px hedefler, ≥16 px yazı.
- Testler: Rust birim testleri (filtre zinciri üretimi, parametre doğrulama, yol güvenliği, bozuk istek), ffmpeg varsa gerçek kısa dönüşüm testi (yoksa atla), Node mantık testi. Örnek: bir malzeme MP4'ünden çizgifilm ve piksel art dönüşümü üret (tasarim/yenileme/ornek/donusum-*.mp4 + kare PNG).
