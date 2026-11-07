---
description: Shotcut 26.9.27 (MLT tabanlı) için TAA Vidyo Yapıcı (LED ekran video programı) araştırma notu. Yalnız ad listesi; kod/varlık/metin kopyalanmadı.
sahip: Yusuf; yazan: Claude (LED video animasyon araştırması, 2026-11-08)
---
# Shotcut 26.9.27 — tarama bulgusu
Kaynak: resmi GitHub sürümü (winget Meltytech.Shotcut, SHA256 doğrulandı). Yöntem: kurulu `melt -query` çıktısı + kurulum klasörü adları. Tarama sonrası kaldırıldı.

## Geçişler (MLT transition)
- Çekirdek: luma (wipe şablonlu), mix (çapraz solma), composite, matte, affine, qtblend, movit.luma_mix, movit.mix, movit.overlay
- frei0r sleid0r: push-up/down/left/right, slide-up/down/left/right, wipe-up/down/left/right, wipe-barn-door-h/v, wipe-circle, wipe-rect
- frei0r karışım modları (geçiş katmanı): addition, burn, darken, difference, divide, dodge, hardlight, lighten, multiply, overlay, screen, softlight, subtract, xfade0r

## Efektler (filtre) — görüntü
- Solma: fadein/fadeout (parlaklık ve movit opaklık), video_autofade, brightness from_black/to_black
- Bulanıklık: blur (üssel, gauss, alçak geçiren), movit.blur, box blur
- Renk: color, contrast, saturation, sepia, invert, posterize, levels, lift-gamma-gain, hue-lightness-saturation, vibrance, white balance, gradientmap, tcolor
- Stil: glow, softglow, vignette, halftone, sketch, charcoal, cartoon, outline, mosaic/pixelize, scanlines, dither, grain, oldfilm, dust, lines
- Hareket/bozma: wave, distort, fisheye, mirror, reflect, flip, rotate, shake, camerashake, vertigo, nervous, choppy, strobe, trails, dance, glitch, rgbsplit, kaleidoscope, burningtv
- Konum/boyut: size_position (anahtar kareli), crop (daire/dikdörtgen), corners (perspektif), reframe, elastic_scale, pillar_echo, zoompan
- Maske/anahtar: chroma key, lumakey, mask (şekil/alfa), tracker, rotoscoping
- Ses görselleştirme: audiospectrum, audiowaveform, audiolevelgraph, lightshow
- Zaman: speed, time_remap, freeze, timer

## Başlık / metin hareketi
- Filtreler: Text (basit), Rich Text, Dynamic Text (zaman/veri değişkenli), Typewriter (daktilo efekti), Subtitle, Timer
- Ön ayar mantığı: filtre başına "fade_in / fade_out / fade_in_out" ön ayarları; blur_in / blur_out / blur_in_out; yalnız bu on ön ayar var, başlık animasyonu esas olarak anahtar karelerle yapılıyor.

## TAA Vidyo Yapıcı'ya fikirler
1. Geçişi ve süresini ayrı bir "geçiş seçici" olarak sun: çapraz solma, itme (4 yön), kaydırma (4 yön), silme (4 yön + barn-door + daire + dikdörtgen); LED'de 500–800 ms ideal.
2. Daktilo ve dinamik metin: sayaç, saat, tarih, kayan değer (fiyat/duyuru panosu) tek "dinamik metin" nesnesi olsun.
3. Her nesne için hazır "giriş / çıkış / giriş-çıkış" üçlüsü (fade, blur, kaydırma); Shotcut'ın ön ayar kalıbı sade ve LED için yeterli.
4. Anahtar kareli konum/ölçek/opaklık (size_position kalıbı) — kaydırma yazı ve logo animasyonu için tek temel.
5. LED'e özgü: strobe/glitch/nervous gibi bozma efektlerine "fotosensitif güvenlik" sınırı (saniyede en fazla 3 parlama) koy; scanlines/grid LED piksel izi benzetimi için kullanılabilir.
6. Maske (şekil/alfa) ile logo içinden video göstermek, ses spektrumu ile müzikli ekran.


