---
description: Malzeme kitaplığı (LED içerik kitaplığı) sözleşmesi — senaryo dili eki, HTTP uçları, window.TaaMalzeme arayüzü, katalog, performans.
sahip: Yusuf
yazan: "[Claude#Malzeme Opus 5.5, yenileme-1.0, 2026-11-18]"
kural: Yalnız yazan yapay zekâ ya da Yusuf'un izniyle değişir; silme yok.
---
# Malzeme kitaplığı

## Özet
- 53 hazır LED malzemesi, 10 kategori: Oyun ve Karakter, Kayan Yazı, Sayaç ve Zaman, Kutlama, Doğa ve Hava, Kalp ve İkon, Kampanya Rozeti, Ok ve Yön, Logo ve Geçiş, Ses ve Ekolayzer.
- Özgün karakterler (telifli oyun kopyası yok): **Yutkun** (başında yeşil filiz olan, noktaları yutan sarı top), **Kovalayıcılar** (tek gözlü, antenli, üç ayaklı jöle yaratıklar; mor/yeşil/turuncu/mavi), **Robi** (piksel robot), **Zıpzıp**, **Ufocuk**, **Yılancık**.
- Malzeme senaryo dilinde bir sahne satırıdır; önizleme (`/onizle`), küçük resim ve MP4 (`/uret`, CLI `--senaryo`) aynı `senaryo::kare_uret` yolundan geçer.
- Harici yazı tipi/internet yok: 5x7 piksel yazı tipi (Türkçe Ç Ğ İ Ö Ş Ü dahil) ve vektör şekiller; rozet/logo yazıları gömülü Inter ile 3B.

## Senaryo dili eki (geriye uyumlu)
```
malzeme <id> [sure=S] [renk=PALET] [hiz=H] [deger=D] [yon=sag|sol|yukari|asagi] [metin="..."] [efekt=a,b]
```
- `sure` 2–120 sn, `hiz` 0,25–4 (animasyon zaman çarpanı), `renk` mevcut palet adları ya da `hex-rrggbb`.
- `deger`, `yon`, `metin` yalnız o malzeme kullanıyorsa geçerlidir; kullanmayan malzemede hata verir. Sınırlar katalogda (`sema`).
- `metin` en çok 120 karakter; `"` ve `#` kullanılamaz (üretici temizler). `{tarih}`, `{gun:YYYY-AA-GG}` çözülür.
- `efekt=` mevcut efekt/dekor adlarını malzemenin altına ekler.
- Eski `sahne "..."` satırları aynen çalışır; aynı senaryoda karışık kullanılabilir. Örnek: `senaryolar/malzeme-yutkun.txt`.

## HTTP uçları (src/malzeme_http.rs)
Hepsi oturum çerezi ister; POST'lar `x-led: 1` başlığı ister (arayuz.rs kuralı). Sorgu dizesi bu uçlara iletilmez: parametreler JSON gövdededir. Gövde sınırı 8 KB.

| Uç | İstek | Yanıt |
|---|---|---|
| `GET /malzeme/liste` | — | `200 application/json` katalog (aşağıda) |
| `POST /malzeme/uret` | `{"id":"…","sure":8,"renk":"…","hiz":1,"metin":"…","deger":3,"yon":"sag"}` (id dışı alanlar isteğe bağlı; `null` = varsayılan) | `200 {"tamam":true,"id":"…","senaryo":"malzeme …","onizleme":"ekran 950x1800\nfps 25\nmalzeme …\n"}`; hata `400 {"tamam":false,"hata":"…"}` (bozuk JSON, bilinmeyen alan/malzeme, sınır dışı, kullanılmayan parametre, 8 KB üstü) |
| `POST /malzeme/kucuk` | `/malzeme/uret` gövdesi + isteğe bağlı `"t":saniye` | `200 text/plain` → `data:image/png;base64,…` (190x360 tek kare, ~10–40 KB) |
| `GET /malzeme/kucuk/<id>` | — | aynı, varsayılan parametrelerle; bilinmeyen id `404` JSON |
| `GET /malzeme/malzeme.js` | — | `200 text/javascript` galeri betiği |
| diğer | — | `404`/`405` JSON |

Büyük önizleme için mevcut `POST /onizle?t=..` ucu `onizleme` metniyle çağrılır (malzeme satırları orada da çizilir).

### Katalog JSON biçimi
```
{"surum":1,"adet":53,
 "kategoriler":[{"id":"oyun","ad":"Oyun ve Karakter","adet":6},…],
 "paletler":[{"ad":"altin","acik":"…","yuz":"#rrggbb","zemin":"#rrggbb","vurgu":"#rrggbb"},…],
 "yonler":[{"id":"sag","ad":"Sağa"},…],
 "malzemeler":[{"id":"yutkun-labirent","ad":"Yutkun Labirentte","kategori":"oyun","etiketler":["yutkun",…],"aciklama":"…","canli":false,
   "varsayilan":{"sure":12,"renk":"gok-mavisi","hiz":1,"metin":"YUTKUN","deger":4,"yon":null},
   "sema":[{"ad":"sure","etiket":"Süre (sn)","tur":"sayi","min":2,"max":120,"adim":0.5,"varsayilan":12},
           {"ad":"renk","etiket":"Renk paleti","tur":"palet","varsayilan":"gok-mavisi"},
           {"ad":"hiz","etiket":"Hız","tur":"sayi","min":0.25,"max":4,"adim":0.25,"varsayilan":1},
           {"ad":"metin","etiket":"Yazı","tur":"metin","max":120,"varsayilan":"YUTKUN"},
           {"ad":"deger","etiket":"Koridor sayısı","tur":"sayi","min":2,"max":6,"adim":1,"varsayilan":4}],
   "senaryo":"malzeme yutkun-labirent sure=12 renk=gok-mavisi hiz=1 deger=4 metin=\"YUTKUN\""},…]}
```
`tur`: `sayi` | `metin` | `palet` | `yon`. `canli:true` (saat, takvim) üretildiği ana bağlıdır.

## window.TaaMalzeme (web/malzeme.js)
Yükleme: `<script src="/malzeme/malzeme.js"></script>` (arayuz.rs değişikliği gerekmez; betik malzeme_http.rs'ten sunulur).
```
TaaMalzeme.mount(el, app) → { hazir: Promise, sec(id) → bool, durum() → {secili,degerler,parca,kategori,arama,adet}, kaldir() }
TaaMalzeme.birakmaHedefiYap(el, app)   // el'e bırakılan malzeme kartı app.senaryoyaEkle(parca) ile eklenir
TaaMalzeme.MIME = "text/x-taa-malzeme" // sürükle-bırak veri türü (ayrıca text/plain = senaryo satırı)
TaaMalzeme._ic                          // saf yardımcılar (test): normal, filtrele, kategoriSayilari, varsayilanDegerler, sinirla, istekGovdesi, bekletici
```
`app` (yoksa `window.TaaApp`) kullanılan yöntemler: `app.onizle(senaryoMetni)` (parametre değişince 350 ms beklemeli, tam önizleme senaryosuyla), `app.senaryoyaEkle(satir)` ("Videoya ekle" ve bırakma). `senaryoMetni/senaryoDegistir/uret/durum` kullanılmaz.
Galeri: arama kutusu (Türkçe harf duyarsız), kategori çipleri (adetli), tembel yüklenen küçük resimli kartlar; karta dokununca parametre paneli (süre, palet, hız, yazı, değer, yön; şemadan), canlı küçük önizleme, "Videoya ekle" (56 px ana eylem), "Önizle", "Kapat". Kartlar sürüklenebilir. Tema yalnız `--bg --panel --panel2 --line --tx --mut --ac --onac --ok --uy` değişkenleri; dokunma hedefi ≥48 px, yazı ≥16 px (ikincil 13), görünür odak, `prefers-reduced-motion`.

## Performans (ölçüm, release, bu makine)
- 950x1800 tek kare çizimi: yoğun malzemelerde 14–42 ms (yutkun-labirent 42, gecis-daire 38, kovalamaca 34, rozet-firsat 32, konfeti 19, havai fişek 19, kar 17, kayan-led 14). Hazırlık+kare ortalaması 28 ms; logo malzemeleri yazı maskesi nedeniyle ilk karede ~100 ms.
- Örnek video (`senaryolar/malzeme-yutkun.txt`, 22,4 sn, 560 kare, 25 fps) CLI ile ~29 sn'de üretildi.
- Küçük resim (190x360) sunucuda ~4 ms, ~18 KB.

## Testler
- `cargo test --release --lib malzeme` (katalog kuralları, ayrıştırma hataları, üretici temizliği, JSON, uçlar).
- `cargo test --release --test malzeme_testleri` (her malzeme: senaryo üretir/ayrıştırılır, 950x1800 render, boş/tek renk değil, deterministik; parametre sınırları; bozuk istek 400; küçük PNG; eski senaryolar; `node tests/malzeme_test.js`).
- Görsel denetim: `MALZEME_CIKTI=klasör ... --test malzeme_testleri -- --ignored kontak` tüm malzemelerin ızgara PNG'sini yazar.
