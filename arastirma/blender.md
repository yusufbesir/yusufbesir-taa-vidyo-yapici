---
description: Blender 4.5.9 için TAA Vidyo Yapıcı (LED ekran video programı) araştırma notu. Yalnız ad listesi; kod/varlık/metin kopyalanmadı.
sahip: Yusuf; yazan: Claude (LED video animasyon araştırması, 2026-11-08)
---
# Blender 4.5.9 — tarama bulgusu
Kaynak: Yusuf'un D:\Programlar\Araçlar Ve İndirilenler\Blender altındaki mevcut taşınabilir kopyası (yeniden indirme/kurma/kaldırma yapılmadı, kopyaya dokunulmadı). Yöntem: bundled python betik adları (space_sequencer, bl_operators, presets klasörleri); program çalıştırılmadı.

## Video düzenleyici (VSE) geçiş ve efekt şeritleri
- Geçiş: Crossfade, Gamma Crossfade, Wipe (tür: tek, çift, iris, saat)
- Efekt: Transform, Speed Control, Glow, Gaussian Blur, Adjustment Layer (altındaki tüm şeritlere efekt), Color Mix (karışım modları), Alpha Over, Alpha Under, Add, Subtract, Multiply, Multicam
- Üretici şeritler: Color, Text (konum/hizalama/gölge/kutu)

## Hareket / başlık animasyonu
- Anahtar kare + F-curve: yumuşatma tipleri Sine, Quad, Cubic, Quart, Quint, Expo, Circ, Back, Bounce, Elastic (her biri In / Out / In-Out); Bezier, Linear, Constant
- Modifiers (F-curve): Noise, Cycles, Generator, Stepped, Limits
- Geometry Nodes: prosedürel metin/parçacık/ızgara animasyonu (3B başlık ve piksel panelleri için)
- Bundled işlevler: object_quick_effects (duman, sıvı, kıvılcım), rigidbody, node_wrangler, Text objesi (3B başlık)
- Ön ayar klasörleri: camera, cycles, eevee, ffmpeg, framerate, render, safe_areas (güvenli alan çizgileri), pixel_density

## TAA Vidyo Yapıcı'ya fikirler
1. Easing kütüphanesi: Back / Bounce / Elastic / Expo giriş-çıkış eğrilerini sabit bir "hareket karakteri" listesi olarak sun (LED'de net ve enerjik görünür).
2. Wipe türleri: tek, çift, iris, saat — dört kolay geçiş; dairesel (iris/saat) LED'de dikkat çeker.
3. Adjustment-layer fikri: tek bir "tüm ekran" efekt katmanı (parlaklık sınırı, renk düzeltme, blur) — LED parlaklık/gamma ayarı için ideal.
4. Safe-areas ön ayarı: LED panel kenar payı (yazı kesilmesin) için "güvenli alan" kılavuzu.
5. Üretici şeritler (Color, Text) ile şablon: logo + yazı + zemin otomatik kompozisyon.
6. Geometry Nodes benzeri parametrik animasyon: piksel ızgarası üzerinde dalga/rastgele yanıp sönme.


