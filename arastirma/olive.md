---
description: Olive 0.1.0 (eski 0.1 çizgisi) için TAA Vidyo Yapıcı (LED ekran video programı) araştırma notu. Yalnız ad listesi; kod/varlık/metin kopyalanmadı.
sahip: Yusuf; yazan: Claude (LED video animasyon araştırması, 2026-11-08)
---
# Olive 0.1.0 — tarama bulgusu
Kaynak: winget OliveTeam.OliveVideoEditor (hash doğrulandı; olivevideoeditor.org). Yöntem: kurulu `effects\` klasöründeki gölgelendirici (shader) dosya adları. Tarama sonrası kendi kaldırıcısıyla kaldırılır.
Not: Yerleşik efektlerin bir kısmı (Transform, Text, Solid, Shake, Mask, Cross Dissolve vb.) programın içinde derlenmiş; aşağıda "(yerleşik, dosyadan doğrulanmadı)" işaretli olanlar uygulamanın belgelenen özellikleridir.

## Efektler (GLSL shader dosyaları — doğrulandı)
boxblur, gaussianblur, directionalblur, radialblur, bulge, fisheye, sphere, swirl, ripple, wave, tile, flip, crop, crossstitch, chromaticaberration, chromakey, lumakey, despill, colorcorrection, colorsel, huesatbri, invert, posterize, pixelate, noise, emboss, toonify, vignette, volumetriclight (ışık hüzmesi); kapalı gelenler: dropshadow, findedges

## Geçiş ve hareket (yerleşik, dosyadan doğrulanmadı)
- Geçişler: Cross Dissolve, Linear Fade, Exponential Fade, Logarithmic Fade (ses geçişleri de aynı aileden)
- Üretici/hareket: Transform (konum, ölçek, döndürme, saydamlık anahtar karelerle), Text, Solid, Shake, Mask, Timecode
- Anahtar kare ara değerleri: doğrusal, yavaşlayan/hızlanan (ease), bezier, tutma

## TAA Vidyo Yapıcı'ya fikirler
1. Shader (GLSL) tabanlı efekt dosyaları: her efekt küçük bir `.frag` + `.xml` parametre tanımıdır. TAA Vidyo Yapıcı'da efektleri aynı şekilde dışarıdan eklenebilir (eklenti klasörü) yap; kullanıcı yeni LED efekti ekleyebilsin.
2. Piksel odaklı efektler: pixelate, posterize, crossstitch, toonify — LED'in "piksel" görünümünü gerçekçi yansıtan hazır set.
3. Dalga ailesi: wave, ripple, swirl, bulge, fisheye, sphere — düşük maliyetli hareketli bozulma; tek parametre (genlik, hız) yeterli.
4. Işık efektleri: vignette, volumetriclight, chromaticaberration — küçük bir "sinema" paketi; LED'de parlak içerik için abartı yapmadan.
5. Anahtar: chromakey + lumakey + despill üçlüsü; yeşil perde olmadan logo çıkarmak için lumakey (siyah/beyaz zemin) LED içeriğinde sık gerekir.
6. Linear / Exponential / Logarithmic fade ayrımı: üç solma eğrisi; LED'de gamma farkı nedeniyle doğrusal solma "geç kalkar", eğri seçimi görünür fark yaratır.
