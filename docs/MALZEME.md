---
description: Malzeme kitaplığı (LED içerik kitaplığı) sözleşmesi — senaryo dili eki, HTTP uçları, window.TaaMalzeme arayüzü, katalog, performans.
sahip: Yusuf
yazan: "[Claude#Malzeme Opus 5.5, yenileme-1.0, 2026-11-18]; dalga 3 güncellemesi [Claude#Malzeme Opus 5.5, yenileme-1.0, 2026-11-25]"
kural: Yalnız yazan yapay zekâ ya da Yusuf'un izniyle değişir; silme yok.
---
# Malzeme kitaplığı

## Özet
- 112 hazır LED malzemesi, 12 kategori (sayı `KATALOG.len()` testinden): **Kış ve Yılbaşı** (başta), Oyun ve Karakter, Kayan Yazı, Sayaç ve Zaman, Kutlama, Doğa ve Hava, Kalp ve İkon, Kampanya Rozeti, Ok ve Yön, Logo ve Geçiş, Ses ve Ekolayzer, **Premium Sahne** (sonda).
- Özgün karakterler (telifli oyun kopyası yok): **Yutkun** (dalga 3'te özgünleştirildi: yuvarlatılmış kare gövdeli mercan-turuncu yaratık, öne açılan dikdörtgen çene yarığı, tek iri göz, yeşil filiz; top/kama silüeti yok), **Kovalayıcılar** (kare kafalı robot-jöle: vizörlü, tek antenli, iki yuvarlak ayaklı; kubbe başlık ve dalgalı etek yok), **Robi** (piksel robot), **Zıpzıp**, **Ufocuk**, **Yılancık**. Labirent koridoru dolu koyu zemin şeridi + enine şeritler, yemler elmas, üst sayaç "PUAN".
- Dalga 3 paketi (karar: `tasarim/yenileme/karar-animasyon-paketi.md`): kış/yılbaşı (katmanlı ve biriken kar, çam ağacı, 5 havai fişek türü, şenlik, geri sayım, kardan adam, buz kristali, manzara, hediye, aurora, süsler, piksel kış/fişek, kızak, kar küresi, mum, kış penceresi, dönen noel yıldızı), 3B/2B yazı (dönen 3B, derinden gelen, harf kaskad/dalga/oynak/düşen, uzun gölge, kabartma, ağır süpürme, yansıma, metal/ahşap/tarama dokusu, dikiş, bayrak dalgası, kinetik tipografi) ve premium sahneler (altın toz, ışık huzmesi, perspektif ızgara, sıvı metal, sis, duman, kadife dalga, dönen 3B küp, parçacık girdabı, cam paneller, spot, tünel, yıldız alanı, sahne perdesi).
- "Premium ağır" görünüm ışıma/neon/bulanıklıkla değil; 3 tonlu gölgeleme, sert gölge, bantlı (Bayer/dama dither'li) ton, doku ve katmanlı derinlikle. Alan efektleri (aurora, sis, duman, sıvı metal) 1/4 çözünürlükte hesaplanıp 4x4 Bayer ile yazılır. Her şey kapalı biçimli ve tohumla deterministik; ortam sahneleri döngü güvenli, döngüsüz olanların açıklamasında "(döngüsüz)" yazar.
- Rozet yazıları LED okunurluğu için açık yüz + kalın koyu dış çizgi (eskiden koyu yazı).
- Malzeme senaryo dilinde bir sahne satırıdır; önizleme (`/onizle`), küçük resim ve MP4 (`/uret`, CLI `--senaryo`) aynı `senaryo::kare_uret` yolundan geçer.
- Harici yazı tipi/internet yok: 5x7 piksel yazı tipi (Türkçe Ç Ğ İ Ö Ş Ü dahil) ve vektör şekiller; rozet/logo yazıları gömülü Inter ile 3B.

## Senaryo dili eki (geriye uyumlu)
```
malzeme <id> [sure=S] [renk=PALET] [hiz=H] [deger=D] [yon=sag|sol|yukari|asagi] [metin="..."] [efekt=a,b]
```
- `sure` 2–120 sn, `hiz` 0,25–4 (animasyon zaman çarpanı), `renk` mevcut palet adları ya da `hex-rrggbb`.
- `deger`, `yon`, `metin` yalnız o malzeme kullanıyorsa geçerlidir; kullanmayan malzemede hata verir. Sınırlar katalogda (`sema`).
- `metin` en çok 120 karakter; `"` ve `#` kullanılamaz (üretici temizler). `{tarih}`, `{gun:YYYY-AA-GG}` çözülür.
- `efekt=` mevcut efekt/dekor adlarını malzemenin altına ekler. Dalga 3 efektleri: `kar-katmanli`, `kar-ortusu`, `isik-dizisi`, `altin-toz`, `isik-huzmesi`, `vinyet` (içerikten sonra çizilir), `buz-koseler`, `sis-alt`.
- Kimlik değişikliği: commit edilmemiş `yazi-2027` → `yazi-yil` (yıl metin parametresidir; kimlik eskimez).
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
{"surum":1,"adet":112,
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
`app` (yoksa `window.TaaApp`) kullanılan yöntemler (dalga 3): `app.geciciOnizle(metin)` / `app.geciciOnizleKapat()` — önizleme kullanıcının senaryosuna DOKUNMAZ (yoksa yalnız küçük resim gösterilir; `app.onizle` bilerek kullanılmaz), `app.senaryoyaEkle(satir, kaynak)` ("Videoya ekle" tek kez: işlem sürerken ve aynı parça eklendikten sonra düğme kapalı, "Eklendi ✓"), `app.senaryoMetni()` + `app.senaryoDegistir(metin, kaynak)` ("Geri al": senaryo eklemeden sonra değişmediyse eski hâline döner, değiştiyse uyarır). Seçili kategori çipi ve ana eylem dolu değil çerçeveli vurgu; köşeler `--r-btn` / `--r-kart`.
Galeri: arama kutusu (Türkçe harf duyarsız), kategori çipleri (adetli), tembel yüklenen küçük resimli kartlar; karta dokununca parametre paneli (süre, palet, hız, yazı, değer, yön; şemadan), canlı küçük önizleme, "Videoya ekle" (56 px ana eylem), "Önizle", "Kapat". Kartlar sürüklenebilir. Tema yalnız `--bg --panel --panel2 --line --tx --mut --ac --onac --ok --uy` değişkenleri; dokunma hedefi ≥48 px, yazı ≥16 px (ikincil 13), görünür odak, `prefers-reduced-motion`.

## Performans (ölçüm, release, bu makine)
- 950x1800 tek kare çizimi: yoğun malzemelerde 14–42 ms (yutkun-labirent 42, gecis-daire 38, kovalamaca 34, rozet-firsat 32, konfeti 19, havai fişek 19, kar 17, kayan-led 14). Hazırlık+kare ortalaması 28 ms; logo malzemeleri yazı maskesi nedeniyle ilk karede ~100 ms.
- Örnek video (`senaryolar/malzeme-yutkun.txt`, 22,4 sn, 560 kare, 25 fps) CLI ile ~29 sn'de üretildi.
- Küçük resim (190x360) sunucuda ~4 ms, ~18 KB.

### Dalga 3 ölçümü (2026-11-25, release, bu makine, ağır testler sırayla)
- `dalga3_kare_butcesi`: 112 kalemin 950x1800 kare çizimi (hazırlık hariç) ortalaması **26,3 ms** (hedef ≤ 45 ms karşılanıyor); 16 kalem tek başına 45 ms'yi aşıyor, en yavaşlar sahne-karli-isik 91, yazi-3b-derin 88, izgara-zemin 85, yazi-3b-don 79, isik-huzmesi ve altin-toz 73, yazi-yil 70, sivi-metal 70 ms. Maliyetin çoğu 3B ekstrüzyon (`kare3d`: bbox × (derinlik+1)); yeni kalemlerde derinlik 12 px. 120 ms üst sınırı testte zorunlu.
- Örnek videolar (CLI, 25 fps): `tasarim/yenileme/ornek/kis-yilbasi.mp4` (36,4 sn, 910 kare, ~58 sn'de üretildi; `senaryolar/malzeme-kis-yilbasi.txt`) ve `3b-yazi.mp4` (34,4 sn, 860 kare, ~65 sn'de; `senaryolar/malzeme-3b-yazi.txt`); örnek kareler `*-kare.png`.

## Testler
- `cargo test --release --lib malzeme` (katalog kuralları, ayrıştırma hataları, üretici temizliği, JSON, uçlar).
- `cargo test --release --test malzeme_testleri` (her malzeme: senaryo üretir/ayrıştırılır, 950x1800 render, boş/tek renk değil, deterministik; parametre sınırları; bozuk istek 400; küçük PNG; eski senaryolar; `node tests/malzeme_test.js`).
- Dalga 3: `dalga3_kare_butcesi` (bütçe), `dalga3_flas_ve_parlaklik` (ardışık karelerde ortalama parlaklık farkı ≤ %20; kış/sahne ve kinetik yazıda kare ortalaması ≤ %45); birim testler: uzun gölge, dönemli gürültü, döngü güvenli açısal hız, kar döngü dikişi, vinyet, malzeme sahnesinde "8-10 harf" önerisi çıkmaması.
- Görsel denetim: `MALZEME_CIKTI=klasör ... --test malzeme_testleri -- --ignored kontak` tüm malzemelerin ızgara PNG'sini yazar. `MALZEME_KARE="id@t,id@t"` ile seçili tam boy kareler de yazılır.
