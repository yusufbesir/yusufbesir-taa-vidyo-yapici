---
description: Kdenlive 26.08.3 için TAA Vidyo Yapıcı (LED ekran video programı) araştırma notu. Yalnız ad listesi; kod/varlık/metin kopyalanmadı.
sahip: Yusuf; yazan: Claude (LED video animasyon araştırması, 2026-11-08)
---
# Kdenlive 26.08.3 — tarama bulgusu
Kaynak: winget KDE.Kdenlive (hash doğrulandı). Yöntem: kurulu veri klasörlerindeki dosya adları (transitions, lumas, titles, effects). Tarama sonrası kendi kaldırıcısıyla kaldırılır.
Not: Motor MLT, Shotcut ile ortak; burada yalnız farklı ve Kdenlive'e özgü olanlar vurgulandı.

## Geçişler
- Çekirdek: dissolve (çapraz solma), luma (wipe şablonlu), wipe, slide, composite, affine, qtblend, matte, mix, region
- Kaydırma/itme: push (4 yön), slide (4 yön); silme: wipe (4 yön), barn-door (yatay/dikey), daire, dikdörtgen
- Luma silme şablonları (18): bi-linear x/y, burst, checkerboard, clock, symmetric clock, cloud, curtain, horizontal blinds, linear x/y, radial, radial-bars, spiral, spiral2, square, square2, square2-bars
- Karışım modu geçişleri: addition, burn, darken, difference, divide, dodge, grain extract/merge, hardlight, lighten, multiply, overlay, screen, softlight, subtract

## Efektler (görüntü; ses hariç)
- Solma: fade from/to black, fadein/fadeout, opacity (movit), brightness
- Hareket/konum: pan_zoom, rotation (anahtar kareli), shear, scroll, zoompan, elastic_scale, corners (perspektif), bezier curves
- Stil/bozma: glow, softglow, vignette, charcoal, cartoon, emboss, edgeglow, oldfilm, scratchlines, dust, grain, scanlines, pixelize, glitch, rgbsplit, nervous, vertigo, wave, mirror, kaleidoscope, burningtv, strobe, shake
- Renk: levels, curves, lift-gamma-gain, hsl primaries/range, colorize, tint, sepia, posterize, gradientmap, color balance/temperature, selective color
- Maske/anahtar: chroma key, lumakey, rotoscoping, shape, alphaspot, mask_start/mask_apply, tracker
- Görselleştirme: audio spectrum/waveform/level graph, lightshow, vectorscope/waveform/RGB parade
- Güvenlik: photosensitivity (ışık sıçraması sınırlayıcı)
- Zaman: speed, freeze, timer

## Başlık / metin hareketi
- Başlık şablonları: simple, simple-with-date, simple-scroll (kayan yazı)
- Metin efektleri: dynamictext, typewriter, subtitles, timer
- Not: Başlıklar ayrı bir başlık düzenleyicide (kutu, kaydırma, kenarlık) hazırlanıp klibe dönüşüyor.

## TAA Vidyo Yapıcı'ya fikirler
1. Luma şablon mantığı: bir gri tonlu görselle sınırsız geçiş (radial, spiral, perde, panjur, damalı). LED için küçük bir "geçiş şablonu" klasörü ve kullanıcı kendi PNG'sini ekleyebilsin.
2. Kayan yazı şablonu (simple-scroll): LED ekranın en sık işi; hız, yön, döngü ve boşluk ayarlı hazır şablon olsun.
3. Tarihli başlık (simple-with-date): tarih/saat/geri sayım alanı otomatik dolsun.
4. Photosensitivity sınırlayıcı: bütün efektlerin sonuna otomatik uygulanabilen yanıp sönme koruması; LED'de güvenlik özelliği olarak öne çıkar.
5. Efekt şablonları (effect-templates) gibi "ikinci renk düzeltme" ve "tek tuşla kapat" kalıbı: efekt grubunu tek seferde aç/kapa.
6. Efektler anahtar kareli ve kategorili (renk, stil, hareket, maske) listelensin; arama kutusu ile hızlı bulunsun.
