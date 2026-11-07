---
description: Synfig Studio 1.5.3 için TAA Vidyo Yapıcı (LED ekran video programı) araştırma notu. Yalnız ad listesi; kod/varlık/metin kopyalanmadı.
sahip: Yusuf; yazan: Claude (LED video animasyon araştırması, 2026-11-08)
---
# Synfig Studio 1.5.3 — tarama bulgusu
Kaynak: winget Synfig.SynfigStudio (hash doğrulandı). Yöntem: `synfig --layers` katman adları + kurulu klasör adları. Tarama sonrası kendi kaldırıcısıyla kaldırılır.
Not: Synfig bir video düzenleyici değil, 2B vektör animasyon aracı; geçiş/efekt yerine katman ve parametre animasyonu sunar.

## Katmanlar (animasyon yapıtaşları)
- Şekil: circle, simple_circle, rectangle, polygon, star, region, outline, advanced_outline, plant (dallanan bitki), metaballs, skeleton (kemik)
- Dolgu/gradyan: solid_color, linear/radial/conical/spiral/curve gradient, checker_board, xor_pattern, noise, mandelbrot, julia
- Bozma/hareket: translate, rotate, zoom, stretch, warp, curve_warp, twirl, spherize, noise_distort, inside_out, skeleton_deformation
- Bulanıklık/filtre: blur, radial_blur, motion_blur, bevel, shade (gölge), halftone2/3, colorcorrect, clamp, chromakey, lumakey
- Zaman: timeloop, stroboscope, freetime, duplicate (çoğaltma), switch, group/filter_group
- Metin ve medya: text, import (görüntü/video), sound

## Animasyon mekanizması
- Anahtar kare (waypoint) ara değer türleri: TCB, Linear, Ease In/Out, Constant, Clamped
- "Dönüştürücü" (parametre bağlama): Linear, Composite, Random, Noise, Sine wave, Exponential, Time loop, Switch, Bone link vb. — bir değeri formülle canlandırır
- Skeleton: kemikle karakter/şekil hareketi; duplicate katmanı ile döngülü desen (parçacık benzeri)
- Dışa aktarma: GIF, PNG dizisi, ffmpeg ile video, SVG

## TAA Vidyo Yapıcı'ya fikirler
1. Parametre dönüştürücüler: "sinüs dalgası", "rastgele", "gürültü", "zaman döngüsü" gibi tek tıkla her özelliğe (konum, renk, boyut) bağlanabilen hareket kaynakları. Kaydırma yazı ve nefes alan parlaklık için çok uygun.
2. Duplicate (çoğaltma) katmanı: bir şekli N kez dizip her kopyada biraz değişen animasyon (dalga, çiçek, yağmur, kar). LED'de yoğun hareketli arka plan üretir.
3. Stroboscope ve timeloop: kontrollü yanıp sönme ve kusursuz döngü; LED döngü videosu için "dikişsiz döngü" seçeneği.
4. Gradyan katmanları: doğrusal, radyal, konik, spiral — hareketli renk arka planı (gökkuşağı akışı) için hafif ve LED'de çok iyi görünür.
5. Vektör kaynaklı (SVG, şekil) içerik: LED çözünürlüğüne göre kayıpsız ölçeklenir; piksel net çizim.
6. Waypoint türleri (Ease, Linear, Constant, Clamped): hareket "hissi" için küçük sabit seçici.
