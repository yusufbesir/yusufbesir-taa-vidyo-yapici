---
description: Tarif et / Yapay zekâ HTTP uçları ve web/ai.js (window.TaaAI) arayüz sözleşmesi.
sahip: Yusuf
yazan: "[Claude#AI Opus 5.5, yenileme-1.0, 2026-11-18]; dalga 3 eki [Claude#AI Opus 5.5, 2026-11-25]; dalga 4 eki [Claude#AI-FIX Opus 5.5, 2026-11-25]"
kural: Yalnız yazan yapay zekâ ya da Yusuf'un izniyle değişir; silme yok.
---
# Tarif API (yenileme-1.0)

## Özet
Tüm uçlar `src/tarif_http.rs` içindedir, `src/arayuz.rs` `/tarif/` önekini yönlendirir. Yalnız 127.0.0.1, oturum çerezi (`taa-oturum`) ve `X-LED: 1` başlığıyla çalışır. Gövdeler UTF-8. Önerilen tek giriş: **POST /tarif/hazirla**.

## POST /tarif/hazirla (ana uç, yedek zinciri)
İstek (JSON) — düz metin gövde de kabul edilir (o zaman `metin` sayılır):
```json
{ "metin": "takım elbiselerde yüzde 20 indirim", "cesit": 0, "yz": true, "onay": false }
```
- `metin` (zorunlu, en çok 4000 karakter; boş olabilir → şablon)
- `cesit` (isteğe bağlı, 0+): "Başka öneri" için farklı stil/anlatım
- `yz` (varsayılan true): doğrudan yapay zekâ denensin mi
- `onay` (varsayılan false): ayarda "sorma" kapalıysa tarifin dış hizmete gitmesi için bu istekte onay

Zincir: (yz açık + anahtar var + (sorma ya da onay)) yapay zekâ → `dogrula` katı şema → olmazsa yerel niyet/kural → olmazsa güvenli şablon. **Her zaman `ok:true` ve geçerli senaryo döner.**

Yanıt (200, JSON):
```json
{ "ok": true,
  "senaryo": "# Tarif et ile üretildi\nekran 950x1800\n...",
  "gorunum": "duz", "sure": 11.0, "acilis": "fade", "kapanis": "fade",
  "sahneler": [ { "satirlar": ["%20","İNDİRİM"], "metin": "%20 / İNDİRİM", "giris": "zipla", "cikis": "sondur",
                  "efekt": "konfeti", "palet": "kirmizi-altin", "font": "anton", "sure": 4.0 } ],
  "notlar": [], "anladim": ["Konu: ..."], "anlasilmayan": [],
  "kaynak": "yapay-zeka | kural | sablon", "kaynak_ad": "Yapay zekâ | Kural | Şablon",
  "varsayimlar": ["Sona işletme adını (TERZİ ADEM ALTUN) ekledim."],
  "adimlar": [ { "ad": "yapay-zeka|kural|sablon", "durum": "basarili|hata|atlandi|onay-bekliyor|iptal", "mesaj": "sade Türkçe" } ],
  "yz_onay_gerekli": false }
```
`yz_onay_gerekli: true` → arayüz "Yapay zekâya gönder" düğmesi gösterir, aynı isteği `onay:true` ile yineler.

## POST /tarif/iptal
Gövde boş. Süren yapay zekâ isteğini (bekleme/yeniden deneme arasında) durdurur. Yanıt `ok` (text/plain). Zaten başlamış tek HTTP isteği en geç zaman aşımında biter (2026-11-25: deneme başına ≤30 sn, toplam ≤35 sn).

## Diğer uçlar (önceki sürümden, uyumlu)
| Uç | İstek | Yanıt |
|---|---|---|
| POST /tarif/istem | düz metin tarif | text/plain: sohbet sitesine yapıştırılacak istem |
| POST /tarif/uygula | yapıştırılan yanıt (kod çiti, açıklama, `[{...}]` dizi ya da tek sahne nesnesi toleranslı) | başarı: `hazirla` yanıtının ilk bölümü (kaynak alanları yok); hata: `{"ok":false,"ozet":"sade cümle","hatalar":[...],"duzelt":"düzelt istemi","anladim":[],"anlasilmayan":[]}` |
| POST /tarif/kural-coz | düz metin | eski kural motoru (yalnız metin aynen); `hazirla` önerilir |
| POST /tarif/llm-gonder | `{"metin":..,"onay":true}` | doğrudan gönderim; hata: `{"ok":false,"baglanti":true,"ozet":"..."}` |
| GET /tarif/siteler | — | `["ChatGPT","Gemini","DeepSeek","Claude", ...]` |
| POST /tarif/ac | site adı (yalnız listedeki ad) | `ok` / 404 |
| POST /tarif/site-ekle | `{"ad":..,"adres":"https://.."}` | `ok` / 400 |
| GET /tarif/llm-durum | — | `{"acik","saglayici","model","adres","sorma","anahtar_var","saglayicilar":[{"id","ad","model"}]}` (anahtar asla dönmez) |
| POST /tarif/llm-ayar | `{"acik","sorma","saglayici","model","adres","anahtar","anahtar_kaldir"}` | `ok` / 400 sade mesaj |
| GET /tarif/hazir, /tarif/katalog, /tarif/kurallar; POST /tarif/kural-ekle | önceki gibi | önceki gibi |

## Doğrudan bağlantı hata kodları (llm::LlmHata)
`kapali, anahtar-yok, anahtar-gecersiz (401/403), sinir (429), bulunamadi (404), sunucu (5xx), istek (diğer 4xx), zaman-asimi, baglanti, bos, bozuk, cok-uzun, iptal, ayar`. Yeniden denenenler: zaman-asimi, baglanti, sunucu, sinir (en çok 2 kez, 1 sn ve 2 sn bekleme). Mesajlar sade Türkçedir; sağlayıcı metni, adres ve anahtar içermez.

## web/ai.js — window.TaaAI
```js
window.TaaAI.mount(el, app) -> { hazirla(onay?), uygula(j), goster(j), yanitUygula(), istemKopyala(), siteAc(ad), el, durum, alan }
```
- `el`: paneli içine çizeceği öğe (içi temizlenir).
- `app` (yoksa `window.TaaApp`): `senaryoDegistir(metin)` "Uygula"da çağrılır (yoksa `senaryoyaEkle`); `onizle(metin)` varsa "Önizle" düğmesi çıkar. `senaryoMetni()`, `uret()`, `durum()` şu an kullanılmaz (ileride).
- Panel: "Videoyu anlatın" metin alanı, 8 örnek çip (tek tık doldurur), büyük "Hazırla" (Ctrl+Enter), "Durdur", sonuç kartı (kaynak rozeti Yapay zekâ/Kural/Şablon, sahne kutuları, varsayımlar, notlar), "Uygula" / "Başka öneri" / "Önizle"; açılır bölümler: "Sohbet sitesinde hazırlat" (istemi kopyala + siteyi aç + yanıtı yapıştır) ve "Doğrudan bağlantı (isteğe bağlı)".
- Tema: yalnız `--bg --panel --panel2 --line --tx --mut --ac --onac --ok --uy`; düğmeler ≥48 px (ana 56 px), yazı 16 px, görünür odak, `prefers-reduced-motion` uyumlu. Sunucu metni yalnız `textContent` ile yazılır.
- Test: `node tests/ai_test.js`.
- `TaaAI._ic`: test için saf yardımcılar (kaynakAdi, adimOzeti, hataMetni, istekGovdesi, sahneSatirlari, ORNEKLER).

## Dalga 3 eklemeleri (2026-11-25, yazan [Claude#AI Opus 5.5, yenileme-1.0])
- **POST /tarif/hazirla** yeni alan `isletme` (isteğe bağlı, en çok 80 karakter): yoksa varsayılan işletme adı; `""` ya da `"kapali"` → işletme adı sahnesi eklenmez; aksi halde o ad (`"A|B"` iki satır). Arayüz bunu `localStorage['taa-isletme']` ayarından gönderir (sunucuya `/ayar` ile kalıcı yazılır).
- `yz:false` → yapay zekâ atlanır (Durdur sonrası anında yerel sonuç). İptal bayrağı yalnız yapay zekâya gerçekten gidilecekse sıfırlanır; yani Durdur sonrası gelen yerel istek önceki iptali bozmaz.
- Sahne nesnesine `malzeme` alanı eklendi (boş ya da malzeme kitaplığı kimliği; kimlikler `crate::malzeme::KATALOG`tan dinamik okunur). Doluysa senaryoda `malzeme <id> ...` satırı olur (Malzeme modülünün `senaryo_parcasi` doğrulamasıyla).
- **Okunurluk**: her sonuç (yapay zekâ, kural, şablon, yapıştırılan yanıt) `okunur_yap`tan geçer: süre ≥5 sn, sahne başına en çok 3 satır, harf ekran yüksekliğinin ≥%5'i (gerekirse dar yazı tipi ya da heceden bölme), sert hareket (yıldırım/flaş/titreme) yumuşatılır, kontrastı 7:1'in altındaki renk en yakın okunur palete çevrilir. Değişiklik notlarda sade Türkçe yazılır. Ham ölçüm için `dogrula_ile(ham, kat, false)`.
- Tarifte oyun/pacman/labirent/kovalamaca/havai fişek/konfeti/kar/geri sayım/kalp/ok/rozet/ekolayzer/çam/yılbaşı geçerse ve model malzeme koymadıysa yerel eşleme malzeme sahnesi ekler (`niyet::malzeme_ekle`).
- **POST /tarif/istem** gövdesi düz metin (eski) ya da `{"metin":..,"isletme":..}`. İstem sürüm 2 (bkz. docs/TARIF-ISTEM-OPTIMIZASYON.md).
- **Bütçe**: yapay zekâya toplam en çok 35 sn (`llm::TOPLAM_BUTCE`; deneme başına kalan süre, kalan <5 sn ise yeniden deneme yok). Arayüz de 35 sn sonra isteği keser (AbortController) ve yerel sonuca geçer.
- **web/ai.js** `mount` dönüşü: `{ hazirla, durdur, uygula, geriAl, onizle, goster, yanitUygula, panodanUygula, kuraldanCoz, istemKopyala, istemGoster, kurallariYukle, kuralEkle, siteAc, siteEkle, isletmeKaydet, el, durum, alan }`. Adım göstergesi (Yaz → Hazırla → Uygula → Önizle); Hazırla metin kutusunun hemen altında; "Önizle" `TaaApp.geciciOnizle(metin)` (yoksa `onizle`); Uygula'dan sonra "Geri al (önceki senaryoya dön)"; kapalı "Diğer araçlar": işletme adı, İstemi göster, Kuraldan çöz, Panodaki tarifi uygula, Kural listesi/ekle, + Site; "Doğrudan bağlantı"da "Her seferinde sor" (varsayılan açık; "Kaydet ve aç" onayı kalıcı kapatmaz). `.taai-ana` çerçeveli vurgu (dolu değil).

## Dalga 4 düzeltmeleri (2026-11-25, yazan [Claude#AI-FIX Opus 5.5, yenileme-1.0])
- **Talimat/ilgisiz talep** (`niyet::talimat_mi`, `niyet::soru_ya_da_ilgisiz`): "önceki talimatları unut…", "şiir yaz", "Ignore previous instructions" gibi tarifler ekrana YAZILMAZ; yerel yol güvenli HOŞ GELDİNİZ + işletme adı şablonu döner (`kaynak:"sablon"`, nedeni `varsayimlar`da). Talimat içeren tarif yapay zekâya hiç gönderilmez (`adimlar`: yapay-zeka `atlandi`, "Tarifte talimat var"). Soru/ilgisiz konu yalnız hiçbir mağaza niyeti bulunamazsa şablona düşer. Tırnak içindeki metin de talimat içeriyorsa yazılmaz. `POST /tarif/kural-coz` (eski "Kuraldan çöz", metni aynen yazar) bu denetimi yapmaz.
- **Sadakat**: talepte geçmeyen İNDİRİM/FIRSAT/BÜYÜK yazılmaz (oran yoksa ve "büyük" yazılmamışsa yalnız "İNDİRİM"; "fırsat"/"kampanya" yazılmışsa o sözcük). "son 3 gün", "son iki saat", "son hafta", "son gün" tanınır (sayı yazıldığı gibi). "yılbaşı kampanyası" → YILBAŞI / KAMPANYASI. Havai fişek, kar, çam gibi belirli görsel istenince genel yılbaşı (kar) görseli eklenmez; kar efekti yalnız kar/kış istenince.
- **web/ai.js**: site düğmeleri doğru ekle ("DeepSeek'te aç", "Claude'da aç"; `TaaApp.bulunmaEki`, yoksa aynı kural yerelde). `anlasilmayan` doluysa sonuç kartında görünür uyarı (`.taai-uyari.taai-anlasilmayan`). `TaaAI._ic.bulunmaEki` eklendi.
- **Anahtar yeri** (güvenlik #6): şifreli anahtar `%LOCALAPPDATA%\TAA Vidyo Yapici\tarif-llm.anahtar`. Eski yerdeki (çıktı klasörü) dosya ilk okumada kopyalanır ve doğrulanır; eski dosya silinmez. Ayar dosyası (`tarif-llm.json`, sır içermez) çıktı klasöründe kalır.

## UI ajanından beklenen (ISTEK-AI.md)
`src/arayuz.rs`: `GET /ai.js` → `include_str!("../web/ai.js")`, `text/javascript; charset=utf-8`. `web/index.html`: Yapay zekâ kipinde `<div id="aiPanel"></div>` ve `<script src="/ai.js"></script>` sonrası `TaaAI.mount(document.getElementById('aiPanel'), window.TaaApp)`.
