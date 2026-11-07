---
description: OpenShot 4.1.1 için TAA Vidyo Yapıcı (LED ekran video programı) araştırma notu. Yalnız ad listesi; kod/varlık/metin kopyalanmadı.
sahip: Yusuf; yazan: Claude (LED video animasyon araştırması, 2026-11-08)
---
# OpenShot 4.1.1 — tarama bulgusu
Kaynak: winget OpenShot.OpenShot (hash doğrulandı). Yöntem: kurulu klasördeki dosya adları (transitions, effects/icons, titles, blender). Tarama sonrası kendi kaldırıcısıyla kaldırılır.

## Geçişler
- Ortak (7): fade, circle in→out, circle out→in, wipe (sol→sağ, sağ→sol, üst→alt, alt→üst)
- Ek geçiş şablonu ailesi (~400 gri tonlu maske): barr_ripple, board, boxes, checked, clock, cross, deform, distortion, dissolve, fish-eyes, flower, fractal, frame, future, hatched, hourglass, luminous_boxes, luminous_spiral, luminous_frame, mosaic, ondulation, puzzle, ray_light, ripple, spiral, star, sunlight, triangle, twirl, vertical/horizontal blinds, wandering, wave, whirlpool, wipe_diagonal
- Yapı: her geçişin yönü ve "yumuşaklık" ayarı var, ters çevrilebilir.

## Efektler (görüntü)
analogtape, bars (letterbox), blur, brightness/kontrast, caption, chromakey, colorgrade, colormap (LUT), colorshift, crop, deinterlace, denoise, displace, distortion, filmgrain, glow, hue, lensflare, mask, negate, noise, objectdetection, objectmask, outline, pixelate, saturation, shadow, sharpen, shift, sphericalprojection (360), stabilizer, timer, tracker, wave
(ses: audiovisualization, beatsync, compressor, delay, echo, expander, normalize, parametriceq, robotization, whisperization)

## Başlık / metin
- Hazır SVG başlıklar (50): Bar 1-3, Box, Bubbles, Cloud, Flames, Flare, Footer 1-3, Gold, Gray Box 1-4, Header 1-3, Oval 1-4, Post-it, Ribbon 1-3, Smoke 1-3, Solid Color, Standard 1-4, Sunset, TV Rating, Film Rating 1-4
- Animasyonlu 3B başlıklar (Blender ile üretilir, 19): blinds, colors, dissolve, earth, explode, fly_by_1, fly_by_two_titles, glare, glass_slider, lens_flare, magic_wand, neon_curves, picture_frames_4, rotate_360, slide_left_to_right, snow, spacemovie_intro, wireframe_text, zoom_clapboard
- Klip animasyonu: konum, ölçek, döndürme, saydamlık anahtar kareli; hazır "giriş/çıkış/yakınlaştırma/kaydırma" kısa yolları; presets klasöründe 64 dışa aktarma profili

## TAA Vidyo Yapıcı'ya fikirler
1. Maske tabanlı geçiş kütüphanesi: gri tonlu görüntüyle geçiş ve "yumuşaklık" kaydırıcısı; LED'in düşük çözünürlüğünde (örn. 128×64) küçük maskeler hafif çalışır. Kullanıcı kendi maskesini ekleyebilsin.
2. Başlık şablonu galerisi: bar, kutu, oval, afiş, şerit; renk/yazı yerine koyma ile LED duyurusu hazırlanır. Şablonlar SVG, yani LED çözünürlüğüne net ölçeklenir.
3. Animasyonlu başlık ailesi (blinds, dissolve, slide_left_to_right, rotate_360, zoom): 3B yerine LED için 2B "hazır hareket" seti; her biri yazı + giriş + bekleme + çıkış.
4. Hazır "giriş/çıkış" kısa yolları: tek tıkla fade, zoom, kayma.
5. Efekt vitrini: her efektin küçük ikonu ve canlı önizleme; kategorili.
6. Dışa aktarma profilleri: LED panel boyutları için hazır profil (çözünürlük, fps, renk derinliği, parlaklık sınırı).
