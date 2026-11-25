---
description: Yapay zekâ istemi ölçümünün sırsız girdi ve çıktıları (istem dosyaları ve model yanıtları). docs/TARIF-ISTEM-OPTIMIZASYON.md tablosu bu dosyalardan yeniden üretilir.
sahip: Yusuf
yazan: "[Claude#AI-FIX Opus 5.5, yenileme-1.0, 2026-11-25]"
kural: Yalnız yazan yapay zekâ ya da Yusuf'un izniyle değişir; silme yok. Buraya sır, anahtar, gerçek müşteri ya da işletme verisi konmaz.
---
# İstem ölçümü dosyaları

## Özet
Dalga 3 istem optimizasyonunda kullanılan istem dosyaları ve modellerin yanıtları. Tablo bu yanıtlardan yeniden hesaplanır:

```
tasarim/yenileme/kilitli-cargo.sh test --release --test tarif_dene istem_yanitlarini_degerlendir -- --ignored --nocapture
```
(`YANIT_DIZIN` verilmezse `tests/istem-olcum/yanit` kullanılır.)

## İçerik
- `istem/<sürüm>-<n>.txt`: yapay zekâya giden istem. Sürümler: `v1` (eski), `v2tr`, `v2en`, `v21tr` (uygulamadaki Türkçe 2.1). `<n>`: `tests/tarif_talepleri.txt` içindeki talep sırası (0'dan). Bu klasör, kış paketi kataloğa eklendikten sonraki istemlerdir; aynı dosyalar `istem_dosyalari_yaz` testiyle (`ISTEM_DIZIN`) yeniden üretilir.
- `yanit/<sürüm>-<model>-<n>.txt`: modelin ham yanıtı. Model: `sol` = GPT-6.1 Sol (codex exec), `gem` = Gemini 3.8 Flash (agy). Boş dosya = yanıt alınamadı (Gemini kotası doldu); değerlendirme boş dosyaları atlar.

## Sırsızlık
- İşletme adı yerine "Örnek Terzi" kullanıldı. Talepler uydurma mağaza cümleleridir. Dosyalarda anahtar, kullanıcı yolu ya da kişisel veri yoktur (kopyalanırken büyük/küçük harf duyarlı tarandı: `sk-…`, `AIza…`, kullanıcı klasörü, gerçek işletme adı).
- Codex/agy günlükleri (kullanıcı klasörü yolu içerir) bilerek konmadı.

## Not
Ölçüm yalnız yapay zekâ (LLM) yolunu kapsar. Anahtar yokken çalışan yerel yolun doğruluğu `tests/tarif_dene.rs` içindeki `yerel_yol_dogrulugu` ve `gercek_magaza_talepleri` testleriyle ölçülür.
