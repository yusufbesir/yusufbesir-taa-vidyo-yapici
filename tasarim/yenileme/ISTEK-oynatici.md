---
description: Oynatıcı ajanının UI ajanından istekleri ve oynatıcı modülünün bağlanma sözleşmesi (src/arayuz.rs, web/index.html UI ajanınındır). Sahip: Yusuf. Yazan: [Claude#Oynatici Opus 5.5, yenileme-1.0, 2026-11-25]. Yalnız yazan yapay zekâ ya da Yusuf'un izniyle değişir; silme yok.
---
# ISTEK-oynatici: UI ajanına (src/arayuz.rs)

Bağlayıcı karar: `tasarim/yenileme/karar-oynatici.md` (2.2). Kararla OYN.md arasındaki fark: medya Range ile
akışlı yazılmak zorunda olduğu için `(kod, tür, gövde)` döndüren imza yetmez. Bu yüzden İKİ giriş var:

## 1. Asıl giriş (ÖNERİLEN): akışa kendisi yazar
`src/oynatici_http.rs`:
```rust
pub fn isle_akis(s: &mut std::net::TcpStream, yontem: &str, yol: &str,
                 bas: &[(String, String)], govde: &[u8], uretim_calisiyor: &dyn Fn() -> bool)
```
- `bas`: `oku()`'nun döndürdüğü küçük harfli başlık listesi (Range ve If-* için gerekir).
- `uretim_calisiyor`: ana sayfada `/uret` çalışıyor mu (sahne pişirme 409 versin diye).
- `/cikis`, `/oynatici`, `/oynatici.js`, `/oynatici.css`, `/oynatici/...` hepsini o yanıtlar (sayfalar CSP başlığıyla).

`isle()` içinde, yetki (çerez) ve POST `X-LED: 1` denetiminden SONRA, `match`'ten ÖNCE tek satır:
```rust
if crate::oynatici_http::bu_yol_mu(&yolu) {
    let uretim = || o.durum.lock().map(|d| d.calisiyor).unwrap_or(false);
    return crate::oynatici_http::isle_akis(&mut s, &yontem, &yol, &bas, &govde, &uretim);
}
```
`bu_yol_mu(yolu)` = `yolu == "/cikis" || yolu == "/oynatici" || yolu.starts_with("/oynatici/") || yolu.starts_with("/oynatici.")`.
Bu satır eklenirse `include_str!("../web/oynatici.html")` vb. yer tutucu rotalarına GEREK YOK (eklenmişse zararsız; önce bu satır yakalar).

## 2. Yedek giriş (OYN.md imzası; medya ve uzun yoklama dışındaki JSON uçları)
```rust
pub fn isle(yontem: &str, yol: &str, govde: &str, klasor: &std::path::Path) -> (u16, &'static str, String)
```
`(m, y) if y.starts_with("/oynatici/")` rotası buna bağlanırsa proje/komut/medya listesi çalışır; ama
`/oynatici/medya/<ad>` 501 döner (video oynamaz). Lütfen 1. yolu kullanın.

## 3. lib.rs
Oynatıcı ajanı ekledi: `pub mod oynatici_http; pub mod oynatici_medya; pub mod oynatici_pencere;`

## 4. Çıkış penceresi köprüsü (karar 2.2 madde 2-3; `pencere()` içinde)
`src/oynatici_pencere.rs` (yalnız Windows'ta gerçek iş yapar):
```rust
pub enum Olay { Ac { monitor: usize, x: i32, y: i32, gen: u32, yuk: u32 }, Kapat }
pub fn kopru_kur(proxy: tao::event_loop::EventLoopProxy<Olay>)          // cfg(windows)
pub struct Cikis { .. }  // Default
pub fn isle(o: Olay, hedef: &tao::event_loop::EventLoopWindowTarget<Olay>, baglam: &mut wry::WebContext, adres_koku: &str, cikis: &mut Cikis)
pub fn pencere_mi(c: &Cikis, id: tao::window::WindowId) -> bool
pub fn kapat(c: &mut Cikis)
```
`pencere()` değişikliği:
```rust
let el = EventLoopBuilder::<crate::oynatici_pencere::Olay>::with_user_event().build();
crate::oynatici_pencere::kopru_kur(el.create_proxy());
let mut cikis = crate::oynatici_pencere::Cikis::default();
let kok = format!("http://127.0.0.1:{port}");   // adres'ten "?t=" öncesi kök
// baglam döngüye taşınır (move)
el.run(move |olay, hedef, akis| {
    *akis = ControlFlow::Wait;
    match olay {
        Event::UserEvent(o) => crate::oynatici_pencere::isle(o, hedef, &mut baglam, &kok, &mut cikis),
        Event::WindowEvent { window_id, event: WindowEvent::CloseRequested, .. } => {
            if crate::oynatici_pencere::pencere_mi(&cikis, window_id) { crate::oynatici_pencere::kapat(&mut cikis); }
            else { std::process::exit(0); }
        }
        _ => {}
    }
});
```
Köprü kurulmazsa `POST /oynatici/cikis` 503 "pencere yok" döner; çıkış sayfası `/cikis` ana pencerede/tarayıcıda yine denenebilir.

## 5. web/index.html
Menüye "Oynatıcı" girişi: `location.href = '/oynatici'` (aynı pencere; kontrol sayfasında "Ana sayfaya dön" bağlantısı var).

## 6. Dalga 4 sahne pişirme denemesi (yazan: [Claude#MAL-FIX Opus 5.5, yenileme-1.0, 2026-11-25]; yalnız ekleme, oynatıcı koduna dokunulmadı)
Uçtan uca HTTP denemesi (release exe, `--arayuz --tarayici-yok`, `LED_CIKTI` geçici klasör, ffmpeg 9.0.2): oturum çerezi + `X-LED: 1` ile
`POST /oynatici/sahne` (gövde: `malzeme cam-agaci sure=3` + `malzeme fisek-kalp sure=3`) → 200, `calisiyor:true`; ~25 sn sonra
`GET /oynatici/sahne` → `{"calisiyor":false,"kare":150,"toplam":150,"mesaj":"hazır"}`, `oynatici-sahne-<zaman>.mp4` (950x1800, 150 kare, 7,4 MB) çıktı klasöründe.
Seçili katmana ekleme (web/oynatici.js `klipEkle`) tarayıcıda DENENMEDİ.

Bulgu (oynatıcı/UI ajanına öneri): pencere yok kipinde (`--tarayici-yok`) src/arayuz.rs boşta kapanma bekçisi yalnız ana üretimi
(`durum.calisiyor`) sayıyor; sahne pişirme sürerken 90 sn nabız gelmezse süreç `exit(0)` ile kapanıyor. İlk denemede bu yüzden klip
0,6 sn'de (15 kare) yarım kaldı ve durum hiç "hazır"a dönmedi. Gömülü pencere kipinde bekçi yok (normal kullanıcı etkilenmez), ama
test/sunucu kipinde bekçi pişirme sürerken de beklemeli (ör. `oynatici_http` bir `sahne_calisiyor()` döndürsün ve bekçi onu da sorsun).
Ayrıca ilk ~10 sn boyunca `kare:0, toplam:0` kalıyor (hazırlık aşaması); arayüzde "hazırlanıyor" yazılması önerilir.
