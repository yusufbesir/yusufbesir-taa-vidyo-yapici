---
description: TAA Vidyo Yapıcı arayüz temalarının gömülü yazı tipleri (woff2, latin + latin-ext + Türkçe alt kümesi). Sahip: Yusuf. Yazan: [Claude#UI Opus 5.5, yenileme-1.0, 2026-11-18]. Yalnız UI ajanı ya da Yusuf'un izniyle değişir; silme yok.
---
# Gömülü yazı tipleri
Kaynak: `D:\Programlar\Araçlar Ve İndirilenler\Yazi-Tipleri-OFL\woff2` (Telefon Köprüsü için google/fonts'tan indirilip Türkçe dahil alt kümesi çıkarılmış dosyalar; bu iş için internetten indirme yapılmadı).
Lisans: hepsi SIL Open Font License 1.1; lisans metinleri bu klasörde `OFL-<ad>.txt`.

| Dosya | Tema |
|---|---|
| spacegrotesk.woff2 | Cephe |
| archivo.woff2 | Manşet |
| nunito.woff2 | Eşlik |
| atkinsonhyperlegible-regular/bold.woff2 | Kadraj (karar "Next" sürümünü istiyor; bilgisayarda yalnız ilk sürüm vardı) |
| ibmplexmono-regular.woff2 | Vuruş (süre, kare, ölçü) |

Vitrin Segoe UI Variable, Vuruş Bahnschrift kullanır (Windows'ta yerleşik). Sunucu `src/arayuz.rs` içinde `/yazi/<dosya>` yolundan sunar.
