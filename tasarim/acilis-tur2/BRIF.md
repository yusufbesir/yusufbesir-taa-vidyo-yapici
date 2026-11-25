---
description: TAA Vidyo Yapıcı açılış animasyonu tur 2 brifi (sırsız). Her model tek animasyon, atanan konseptle. [Claude Opus 5.5, TAA Video Yapıcı arayüz başlığı, 2026-11-17]
---
Program: TAA Vidyo Yapıcı. Terzi mağazası çalışanının, bina cephesindeki iki dikey LED ekran (950x1800 piksel) için animasyonlu yazı videosu yaptığı Windows programı. Logo "İkiz Pile": birbirine bakan iki kumaş pilesi; aynı zamanda cephedeki iki eş ekranı anlatır. Düz biçim (128x128):
sol pile M16 16 H40 L56 64 L40 112 H16 L32 64 Z ; sağ pile M112 16 H88 L72 64 L88 112 H112 L96 64 Z
Ana renk vişne #9A1B3C (açık tonu #D2486A, koyu tonu #5A0C20). Zemin açık #F1F0EC ya da koyu #191826; birini seç.

İlk tur açılışlar BEĞENİLMEDİ: çok küçük, çekingen ve sıradandı (logo biraz kayıp beliriyordu). Bu tur sinematik, akılda kalan ve pencerenin tamamını kullanan bir tanıtım olsun. Yine de şatafat yok: neon, parıltı, lens ışığı, konfeti, zıplama yok.

SENİN KONSEPTİN (yalnız bunu uygula): {KONSEPT}

Kurallar:
- Tek, bağımsız HTML dosyası. Dış kaynak, CDN, font indirme, görsel dosya yok. Yazı tipi: "Segoe UI Variable", "Segoe UI", system-ui.
- Yalnız CSS animasyonu ve Web Animations API (element.animate). Kütüphane yok. Canvas serbest ama gerekmedikçe kullanma.
- Toplam süre 2,5 ile 4 saniye. Sonda logo, altında "TAA Vidyo Yapıcı" yazısı ve küçük "Yaz, önizle, tek tıkla videoyu yap." cümlesi durur.
- Sağ üstte "Geç" düğmesi (animasyonu anında son kareye getirir), sağ altta "Tekrar oynat" düğmesi.
- prefers-reduced-motion: reduce açıksa yalnız 120 ms solma ile son kare.
- Pencere 1366x768 ile 1920x1080 arası; her boyutta ortalı ve oranlı.
- Türkçe karakterler (ğ ş ı İ ç ö ü) düzgün görünmeli.
- 250 satırı geçme. Kod içinde Türkçe kısa yorumlar olabilir.

Yanıt biçimi: önce "AD: <iki kelime>" ve "FIKIR: <tek cümle>" satırları, sonra ```html ile tam dosya. Başka bir şey yazma.
