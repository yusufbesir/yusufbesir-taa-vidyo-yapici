---
description: TAA Vidyo Yapıcı için 7 uygulamanın (Shotcut, Kdenlive, OpenShot, Synfig, Natron, Blender, Olive) animasyon/geçiş/efekt taramasının ortak özeti.
sahip: Yusuf; yazan: Claude (2026-11-08)
---
# Ortak sonuçlar
Taranan: Shotcut 26.9.27, Kdenlive 26.08.3, OpenShot 4.1.1, Synfig 1.5.3, Natron 2.5.0, Blender 4.5.9, Olive 0.1.0. Ayrıntı her uygulamanın kendi dosyasında.

## Ortak animasyon fikirleri (LED için)
1. **Maske tabanlı geçiş:** gri tonlu bir görselle sınırsız geçiş (Kdenlive luma, OpenShot ek geçişler, Shotcut luma). Küçük maske klasörü + kullanıcının kendi maskesi.
2. **Hazır geçiş seti:** çapraz solma, itme (4 yön), kaydırma (4 yön), silme (4 yön + barn-door), daire, dikdörtgen, saat/iris. LED'de 500–800 ms.
3. **Giriş / bekleme / çıkış üçlüsü:** her nesneye hazır giriş ve çıkış hareketi (fade, blur, kaydırma, yakınlaştırma) — Shotcut/OpenShot/Olive'in ortak kalıbı.
4. **Kayan yazı ve dinamik metin:** daktilo, sayaç, saat, tarih, geri sayım; kayan yazı şablonu (hız, yön, döngü).
5. **Anahtar kareli konum/ölçek/opaklık + easing:** Bezier/Ease/Linear/Constant; Blender'ın Back, Bounce, Elastic, Expo eğrileri "hareket karakteri" olarak.
6. **Parametre kaynakları:** sinüs dalgası, rastgele, gürültü, zaman döngüsü (Synfig), sesle sürme (Natron AudioCurve).
7. **Prosedürel arka planlar:** gradyan, plazma, dalga, ızgara, çoğaltma (Synfig duplicate, Natron Shadertoy/Plasma/Ramp); video gerekmeden hareketli zemin.
8. **Piksel ve LED benzetimi:** pixelate, posterize, scanlines, grid, halftone, crossstitch — LED piksel izini yansıtan efektler.
9. **Adjustment/tüm ekran katmanı:** tek katmanla parlaklık, gamma, renk (Blender adjustment layer, Kdenlive efekt şablonları).
10. **Eklenti efektleri:** efekt = küçük shader + parametre tanımı (Olive modeli); kullanıcı yeni efekt ekleyebilsin.

## LED'e özgü güvenlik ve çıktı
- Yanıp sönme koruması (Kdenlive photosensitivity): saniyede en fazla 3 parlama, strobe/glitch'e uygulanır.
- Güvenli alan kılavuzu (Blender safe_areas) ve LED panel boyutları için hazır dışa aktarma profilleri (OpenShot presets).
- Dikişsiz döngü seçeneği (Synfig timeloop) ve solma eğrisi seçimi (doğrusal/üssel/logaritmik; LED gamma farkı için).

## Temizlik durumu
Kurulan beş uygulama kendi kaldırıcısıyla kaldırıldı (Natron kurulmadı, taşınabilir paket okundu); Blender mevcut taşınabilir kopya olarak kaldı. Yükleyiciler ve açılmış paketler Geri Dönüşüm Kutusu'nda; kalıcı silme yok.
