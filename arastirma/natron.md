---
description: Natron 2.5.0 için TAA Vidyo Yapıcı (LED ekran video programı) araştırma notu. Yalnız ad listesi; kod/varlık/metin kopyalanmadı.
sahip: Yusuf; yazan: Claude (LED video animasyon araştırması, 2026-11-08)
---
# Natron 2.5.0 — tarama bulgusu
Kaynak: winget Natron.Natron (hash doğrulandı; paket zip içinde taşınabilir yapı + Setup.exe). Yöntem: zip açılıp eklenti paketlerinin ve ikon dosyalarının adları okundu. Program ve Setup.exe çalıştırılmadı, kurulum yapılmadı; açılan klasör Geri Dönüşüm Kutusu'na taşındı.
Not: Natron düğüm tabanlı kompozit (After Effects/Nuke benzeri); geçiş katalogu yok, geçişi düğümlerle kurarsın.

## Düğüm aileleri (OpenFX)
- Üretici: Constant, Ramp, Radial, Rectangle, CheckerBoard, ColorBars, ColorWheel, Noise, CImgPlasma, Text (iki ayrı), Texture, Shadertoy (GLSL), Roto
- Geçiş/karışım: Dissolve, Merge (over, plus, screen, multiply, min, max, in, out, difference), Switch, FrameBlend, Retime, FrameHold, FrameRange, timeOffset
- Hareket/geometri: Transform (+maskeli), CornerPin, Position, Crop, Reformat, Mirror, Tile, Roll, Polar, Swirl, Wave, Implode, Arc, Reflection, Polaroid, LensDistortion, IDistort, STMap, SlitScan
- Bulanıklık/ışık: Blur (CImg), DirBlur, GodRays, Glow, Bloom, DropShadow, LightWrap, EdgeBlur, AngleBlur, Oilpaint, Charcoal, Sketch, Edges
- Renk: Grade, ColorCorrect, HSVTool, HueCorrect, Saturation, Gamma, Invert, ColorLookup, ColorMatrix, ColorSpace (OCIO), Modulate, Premult/Unpremult
- Anahtar/maske: Keyer, ChromaKeyer, HueKeyer, PIK, Despill, ZMask, ZRemap, TrackerPM (izleyici)
- Ses/okuma-yazma: AudioCurve (sesle parametre), ReadSVG, ReadPSD, ReadFFmpeg, WriteFFmpeg, Read/Write PNG/EXR/OIIO
- G'MIC filtre koleksiyonu (yüzlerce hazır filtre) ayrıca paketli

## Hazır grup düğümleri (PyPlugs): AngleBlur, DropShadow, EdgeBlur, Fill, Glow, LightWrap, PIKColor, SplitAndJoin, ZMask, ZRemap

## TAA Vidyo Yapıcı'ya fikirler
1. Düğüm/katman grafiği: geçiş, efekt ve metni "kutular + oklar" olarak bağlama (ileri kullanıcı için); basit kullanıcıya hazır zincir şablonu sun, arkasında aynı yapı çalışsın.
2. AudioCurve: sesin yüksekliğiyle parlaklık/boyut/renk sürmek; müzikli LED ekran için doğrudan işe yarar.
3. Shadertoy düğümü: GLSL ile prosedürel arka plan (plazma, dalga, ızgara); LED panel için hafif ve sonsuz çeşitli desenler.
4. Üretici düğümler: Ramp, Radial, Plasma, CheckerBoard, ColorBars — video gerekmeden hareketli zemin; ColorBars/ColorWheel LED panel test deseni olarak da kullanılabilir.
5. Merge modları (plus, screen, multiply, max, min): katman karışımı için yedi-sekiz sabit mod yeterli.
6. Roll, Polar, Swirl, Wave, Tile gibi tek parametreli hareket bozucular: kaydırma tekrarı (Tile+Roll) LED'de kesintisiz döngü afişi için ideal.
7. SlitScan ve FrameBlend: iz bırakma ve hareket bulanıklığı efekti; hızlı hareketli yazılarda LED'de yumuşatma.
