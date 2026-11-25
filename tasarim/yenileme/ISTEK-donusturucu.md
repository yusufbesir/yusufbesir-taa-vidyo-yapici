---
description: Dönüştürücü ajanının UI ajanına bağlama isteği (dalga 3). Sahip Yusuf; yazan [Claude#Dönüştürücü Opus 5.5, yenileme-1.0, 2026-11-25]; yalnız yazan yapay zekâ ya da Yusuf'un izniyle değişir; silme yok.
---
# İstek: /donustur/ rotasını donustur_http'ye bağla (UI ajanı, src/arayuz.rs)

Modül hazır: `src/donustur.rs`, `src/donustur_http.rs` (lib.rs'e `pub mod donustur; pub mod donustur_http;` eklendi).

İmza:
```rust
pub fn crate::donustur_http::isle(yontem: &str, yol: &str, govde: &str, klasor: &std::path::Path) -> (u16, &'static str, String)
```
- `yol`: "/donustur/" sonrası, sorgu dizesi YOK (parametreler JSON gövdede). `govde`: istek gövdesi (UTF-8). `klasor`: `cikti_klasoru()`.
- Gövde her zaman metin (JSON ya da JS); PNG önizleme JSON içinde data URL olarak döner.

arayuz.rs'teki 503 kolunun yerine önerilen satır:
```rust
(m, y) if y.starts_with("/donustur/") => { let (kod, tur, g) = crate::donustur_http::isle(m, &y[10..], &yazi, &cikti_klasoru()); yanit(&mut s, kod, tur, g.as_bytes()) }
```
`GET /donustur.js` mevcut gömülü kol kalabilir; ayrıca `GET /donustur/donustur.js` de aynı betiği verir.

Uçlar: turler, kaynaklar, durum (GET); onizle, baslat, iptal, oynat, klasor (POST, x-led:1 başlığı arayüzce zaten zorunlu). Ayrıntı: docs/DONUSTURUCU.md.

"Oynatıcıya ekle" düğmesi: `window.TaaApp.oynaticiyaEkle(yol)` varsa çağrılır ve her durumda `document` üzerinde `taa:oynaticiya-ekle` olayı (`detail.dosya` = tam yol) gönderilir. Oynatıcı ajanı/UI bu olayı dinleyebilir.
