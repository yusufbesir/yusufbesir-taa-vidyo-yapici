---
description: AI ajanının UI ajanından / koordinatörden istekleri (dosya sahipliği dışında kalan değişiklikler).
sahip: Yusuf
yazan: "[Claude#AI Opus 5.5, yenileme-1.0, 2026-11-18]"
kural: Yalnız yazan yapay zekâ ya da Yusuf'un izniyle değişir; silme yok.
---
# ISTEK-AI

## Özet
`web/ai.js` hazır ve testli (`node tests/ai_test.js`). Durum (2026-11-18, son kontrol): 1. ve 2. madde UI ajanınca zaten yapılmış (arayuz.rs `/ai.js`, index.html `TaaAI.mount`). Kalan: TaaApp'in `senaryoDegistir` sağladığının doğrulanması ve 3. madde.

1. `src/arayuz.rs` (UI): gömülü dosya ve yol ekleyin:
   - `const AI_JS: &str = include_str!("../web/ai.js");`
   - eşleşmeye: `("GET", "/ai.js") => yanit(&mut s, 200, "text/javascript; charset=utf-8", AI_JS.as_bytes()),`
2. `web/index.html` (UI): "Yapay zekâ" kipinde eski tarif kartı yerine `<div id="aiPanel"></div>`; sayfa sonunda `<script src="/ai.js"></script>` ve `window.TaaApp` tanımlandıktan sonra `TaaAI.mount(document.getElementById('aiPanel'), window.TaaApp)`.
   - TaaApp'ten kullanılanlar: `senaryoDegistir(metin)` (Uygula), `onizle(metin)` (varsa Önizle düğmesi).
   - `tests/arayuz_ui.rs` eski tarif kimliklerini (tarifKart, tarifCoz …) arıyor; eski kart kaldırılırsa o testin UI ajanınca güncellenmesi gerekir.
3. Koordinatör: commit öncesi `guvenlik-denetcisi` (llm.rs ağ katmanı değişti: zaman aşımı, yeniden deneme, iptal; anahtar saklama aynen korundu).
