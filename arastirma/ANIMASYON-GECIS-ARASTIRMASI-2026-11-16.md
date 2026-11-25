---
description: TAA Vidyo Yapıcı için animasyon, geçiş, efekt, yazı ve arka plan araştırma raporu (web kaynaklı, 194 yeni öneri; mevcut katalog envanteri, uygulama sırası, tarif kuralı bağlama ve geçiş editörü tasarımı). Salt öneridir, kod değişmedi. Sahip Yusuf; yazan [Claude#ebd691 alt ajan, 2026-11-16]. Silme yok; güncelleme üstü çizilerek yapılır.
sahip: Yusuf
yazan: "[Claude#ebd691 alt ajan, 2026-11-16]"
---
# Animasyon, efekt ve geçiş araştırması (2026-11-16)

## Özet
- Bu rapor yalnız öneridir; programın kodu, kataloğu ve kural dosyaları değiştirilmedi.
- Mevcut katalog (dosyalardan sayıldı): 19 geçiş seçeneği (18 gerçek + "yok"), 186 giriş, 182 çıkış (aynı 174 animasyon kütüphanesi + yerleşik), 41 efekt (10 yazı efekti + 31 dekor), 7 hareket eğrisi, 46 palet, 26 font. Kural dosyasında 87 giriş, 14 çıkış, 41 efekt, 18 geçiş, 46 palet, 26 font kuralı var.
- Yeni öneri sayısı: **194** (geçiş 93, giriş 28, çıkış 5, vurgu 2, efekt 24, yazı 18, arka plan 10, eğri 14). Hiçbiri mevcut ad listesiyle çakışmaz (betikle denetlendi).
- Öneri sırası: (1) ffmpeg xfade'in resmi 58 geçişinden programda olmayan 53 tanesi (5 tanesi mevcutla eşdeğer olduğu için alınmadı); (2) anahtar kare dosyalarıyla yazılan yeni giriş/çıkış/vurgu animasyonları (yeni kod gerekmez, yalnız .txt); (3) piksel kodu gerektiren efekt, yazı ve arka plan öğeleri; (4) gl-transitions'tan seçilmiş 40 geçiş; (5) yeni eğriler.
- LED için ana kural: ana mesaj durağan veya yavaş hareketli kalmalı, tek öğe hareket etmeli, hızlı kesme ve ani flaş olmamalı [14] [16]; bu kural programın "strobe yok" ilkesiyle uyumludur.
- Ayrı bir "geçiş editörü" önerisi 7. bölümdedir (zaman çizelgesi, iki klip arası geçiş seçimi, önizleme); tavsiye: önce mevcut sahne listesine "sahneler arası geçiş" alanı eklemek, editörü ikinci aşamada yapmak.

## 1. Yöntem ve sınırlar
- Önce kataloğu dosyalardan okudum: `src/senaryo.rs` (GECISLER, GIRISLER, CIKISLAR, EFEKTLER), `src/dekor.rs` (ADLAR), `animasyonlar/*.txt` (9 dosya), `kurallar/tarif-kurallari.json`, `docs/TARIF-ET-LLM.md`, `src/anim.rs` (EGRILER, egri_k).
- Sonra web araştırması: ffmpeg resmi dokümanı (xfade listesi bire bir alındı), gl-transitions deposu (isimler GitHub API'den), easings.net, MDN, Adobe/DaVinci/Canva/CapCut açıklamaları ve LED/DOOH rehberleri. Kaynak numaraları 6. bölümde.
- Doğrulanamayanlar (uydurmadım, "doğrulanmadı" diye işaretledim) en altta ayrı listelenmiştir.
- Önerilerin ad stili mevcut katalog gibi ASCII kebab-case'tir. Ad çakışmasını önlemek için bazı adlara ek verildi (ör. `ruzgar-savur`).

## 2. Mevcut katalog envanteri (sayılarla)
| Grup | Sayı | Nerede | Adlar |
|---|---|---|---|
| Geçiş (açılış/kapanış) | 19 (18 + yok) | `senaryo.rs` GECISLER | yok, tv, perde, kapi, iris, slayt, fade, neon, sil, jaluzi, mozaik, dalga, cubuk, elmas, kare, saat, capraz, damali, yanlar |
| Giriş (yerleşik) | 12 | GIRISLER | yok, donerek, zipla, yandan, zoom, alttan, ustten, belir, elastik, daktilo, sil, toplan |
| Çıkış (yerleşik) | 8 | CIKISLAR | yok, yukari, asagi, donerek, sondur, kucul, sil, dagil |
| Anahtar kareli animasyon | 174 | `animasyonlar/*.txt` (donus, sirali, temel, vurgu, vurucu, yeni, yeni2, yeni3, yon) | hem giriş hem çıkış olarak kullanılır; yönlü aileler (-sol/-sag/-ust/-alt), tek-tek-*, vurgu-*, kavis/firlat/suzul/yatik, kayar/yaylan/yavas/hizli/donen/zoomlu, capraz-* vb. |
| Toplam giriş / çıkış listesi | 186 / 182 | giris_listesi(), cikis_listesi() | |
| Yazı efekti | 10 | EFEKTLER | parlama, pirilti, cerceve, cift-cerceve, parcacik, kivilcim, kar, yagmur, halka, kayan-yildiz |
| Dekor (efekt/zemin/çerçeve) | 31 | `dekor.rs` ADLAR | 13 süs efekti (cicek-yagmuru ... pirilti-bulutu), çerçeve-* (ince, cift, kose, nokta, neon, yildiz, dalga, isik, altin, serit), zemin-* (serit, yildizli, nabiz, sutun, gradyan, dalga, izgara, plazma) |
| Toplam efekt listesi | 41 | efekt_listesi() | |
| Hareket eğrisi | 7 | `anim.rs` EGRILER | yok, yavasla, hizlan, geri, esnek, sek, ustel |
| Anahtar kare alanları | 6 | `temel.txt` başlığı | dx, dy, olcek, aci, parlak, derinlik (ek: gecikme, sure, egri) |
| Tarif kuralları | 232 | `tarif-kurallari.json` | giris 87, palet 46, efekt 41, font 26, gecis 18, cikis 14, gorunum 2 |

Gözlemler:
- Geçişlerin hepsi ya "kapanan/açılan" maske ya da silme; kaydırma (slide/push/cover/reveal) ve köşe tabanlı çapraz geçiş ailesi, blur, dilim, rüzgar, sıkıştırma ve şekil maskeleri eksik.
- Çıkış listesi kural dosyasında yalnız 14 kural içeriyor; 174 animasyon kütüphanesinin çoğu tariften seçilemiyor (eşleme eksik).
- Yazı için harf/kelime düzeyinde animasyon yok; "daktilo" ve "toplan" dışında yazıya özel hareket neredeyse yok.
- Eğriler Penner ailesinin yalnız 6'sını içeriyor (easings.net'te 30 eğri var [4]).

## 3. YENİ ÖNERİLER (194 madde)
Kısaltmalar: zorluk "kolay" = yalnız .txt veya birkaç satır kod; "orta" = yeni piksel kodu veya küçük yeni alan; "zor" = karmaşık maske, 3B, gürültü veya yeni motor alanı. "Kaynak" sütunu 6. bölümdeki numaralara bağlıdır. "Mevcutla eşdeğer olduğu için alınmayanlar": xfade `fade` (fade), `wiperight` (sil), `circleopen` (iris), `radial` (saat), `horzopen` (kapi).

| No | Ad | Tür | Açıklama | Uygulama yolu | LED okunabilirlik notu | Zorluk | Kaynak |
|---|---|---|---|---|---|---|---|
| 1 | `sil-sol` | geçiş | Sağdan sola silme ile yeni sahne açılır | ffmpeg xfade `wipeleft`; veya kendi piksel maskemiz (iki karenin kaynak karışımı) | Sert kenar, LED’de net; iki sahne de okunur kalır | kolay | [1] |
| 2 | `sil-yukari` | geçiş | Alttan yukarı silme | ffmpeg xfade `wipeup`; veya kendi piksel maskemiz (iki karenin kaynak karışımı) | Sert kenar, LED’de net; iki sahne de okunur kalır | kolay | [1] |
| 3 | `sil-asagi` | geçiş | Üstten aşağı silme | ffmpeg xfade `wipedown`; veya kendi piksel maskemiz (iki karenin kaynak karışımı) | Sert kenar, LED’de net; iki sahne de okunur kalır | kolay | [1] |
| 4 | `kaydir-sol` | geçiş | Yeni sahne sağdan girip eskiyi sola iter | ffmpeg xfade `slideleft`; veya kendi piksel maskemiz (iki karenin kaynak karışımı) | Tüm kare kayar; yavaş (0.8 sn ve üstü) ise okunur, hızlıysa bulanıklaşır | kolay | [1] |
| 5 | `kaydir-sag` | geçiş | Yeni sahne soldan girip eskiyi sağa iter | ffmpeg xfade `slideright`; veya kendi piksel maskemiz (iki karenin kaynak karışımı) | Tüm kare kayar; yavaş (0.8 sn ve üstü) ise okunur, hızlıysa bulanıklaşır | kolay | [1] |
| 6 | `kaydir-yukari` | geçiş | Yeni sahne alttan girip eskiyi yukarı iter | ffmpeg xfade `slideup`; veya kendi piksel maskemiz (iki karenin kaynak karışımı) | Tüm kare kayar; yavaş (0.8 sn ve üstü) ise okunur, hızlıysa bulanıklaşır | kolay | [1] |
| 7 | `kaydir-asagi` | geçiş | Yeni sahne üstten girip eskiyi aşağı iter | ffmpeg xfade `slidedown`; veya kendi piksel maskemiz (iki karenin kaynak karışımı) | Tüm kare kayar; yavaş (0.8 sn ve üstü) ise okunur, hızlıysa bulanıklaşır | kolay | [1] |
| 8 | `daire-kirp` | geçiş | Daire maske önce küçülür, yeni sahne dairede büyür | ffmpeg xfade `circlecrop`; veya kendi piksel maskemiz (iki karenin kaynak karışımı) | Şekil maske; kenar net, okunaklı | orta | [1] |
| 9 | `dikdortgen-kirp` | geçiş | Dikdörtgen kırpma ile yeni sahneye geçer | ffmpeg xfade `rectcrop`; veya kendi piksel maskemiz (iki karenin kaynak karışımı) | Şekil maske; kenar net, okunaklı | orta | [1] |
| 10 | `uzaklik` | geçiş | Piksel renk uzaklığına göre karışım | ffmpeg xfade `distance`; veya kendi piksel maskemiz (iki karenin kaynak karışımı) | Özel; önizlemede LED’de test edin | orta | [1] |
| 11 | `karartma-gec` | geçiş | Siyaha sönüp yeni sahne siyahtan gelir | ffmpeg xfade `fadeblack`; veya kendi piksel maskemiz (iki karenin kaynak karışımı) | Renk üzerinden geçiş; geniş beyaz alan LED’de göz yorar, parlaklık sınırlı | kolay | [1] |
| 12 | `beyaza-gec` | geçiş | Beyaza parlayıp yeni sahne beyazdan gelir | ffmpeg xfade `fadewhite`; veya kendi piksel maskemiz (iki karenin kaynak karışımı) | Renk üzerinden geçiş; geniş beyaz alan LED’de göz yorar, parlaklık sınırlı | kolay | [1] |
| 13 | `yumusak-sol` | geçiş | Yumuşak kenarlı sola silme | ffmpeg xfade `smoothleft`; veya kendi piksel maskemiz (iki karenin kaynak karışımı) | Yumuşak kenar; LED’de kenar bantlaşması olabilir, 0.8 sn üstü önerilir | kolay | [1] |
| 14 | `yumusak-sag` | geçiş | Yumuşak kenarlı sağa silme | ffmpeg xfade `smoothright`; veya kendi piksel maskemiz (iki karenin kaynak karışımı) | Yumuşak kenar; LED’de kenar bantlaşması olabilir, 0.8 sn üstü önerilir | kolay | [1] |
| 15 | `yumusak-yukari` | geçiş | Yumuşak kenarlı yukarı silme | ffmpeg xfade `smoothup`; veya kendi piksel maskemiz (iki karenin kaynak karışımı) | Yumuşak kenar; LED’de kenar bantlaşması olabilir, 0.8 sn üstü önerilir | kolay | [1] |
| 16 | `yumusak-asagi` | geçiş | Yumuşak kenarlı aşağı silme | ffmpeg xfade `smoothdown`; veya kendi piksel maskemiz (iki karenin kaynak karışımı) | Yumuşak kenar; LED’de kenar bantlaşması olabilir, 0.8 sn üstü önerilir | kolay | [1] |
| 17 | `daire-kapan` | geçiş | Daire dışarıdan merkeze kapanarak geçiş | ffmpeg xfade `circleclose`; veya kendi piksel maskemiz (iki karenin kaynak karışımı) | Şekil maske; kenar net, okunaklı | kolay | [1] |
| 18 | `dikey-kapi` | geçiş | Dikey eksende iki yarıya ayrılıp açılır | ffmpeg xfade `vertopen`; veya kendi piksel maskemiz (iki karenin kaynak karışımı) | Şekil maske; kenar net, okunaklı | kolay | [1] |
| 19 | `dikey-kapi-kapan` | geçiş | Üst ve alttan ortaya kapanır | ffmpeg xfade `vertclose`; veya kendi piksel maskemiz (iki karenin kaynak karışımı) | Şekil maske; kenar net, okunaklı | kolay | [1] |
| 20 | `yatay-kapi-kapan` | geçiş | Sol ve sağdan ortaya kapanır (yanlar ile yakın) | ffmpeg xfade `horzclose`; veya kendi piksel maskemiz (iki karenin kaynak karışımı) | Şekil maske; kenar net, okunaklı | kolay | [1] |
| 21 | `nokta-erime` | geçiş | Rastgele piksellerle erime geçişi | ffmpeg xfade `dissolve`; veya kendi piksel maskemiz (iki karenin kaynak karışımı) | Özel; önizlemede LED’de test edin | orta | [1] |
| 22 | `pikselle` | geçiş | Büyüyen bloklara pikselleşir, sonra yeni sahne netleşir | ffmpeg xfade `pixelize`; veya kendi piksel maskemiz (iki karenin kaynak karışımı) | Özel; önizlemede LED’de test edin | orta | [1] |
| 23 | `diyagonal-sol-ust` | geçiş | Sol üst köşeden çapraz açılış | ffmpeg xfade `diagtl`; veya kendi piksel maskemiz (iki karenin kaynak karışımı) | Çapraz sert kenar; metin satırlarını keser ama kısa sürede biter | kolay | [1] |
| 24 | `diyagonal-sag-ust` | geçiş | Sağ üst köşeden çapraz açılış | ffmpeg xfade `diagtr`; veya kendi piksel maskemiz (iki karenin kaynak karışımı) | Çapraz sert kenar; metin satırlarını keser ama kısa sürede biter | kolay | [1] |
| 25 | `diyagonal-sol-alt` | geçiş | Sol alt köşeden çapraz açılış | ffmpeg xfade `diagbl`; veya kendi piksel maskemiz (iki karenin kaynak karışımı) | Çapraz sert kenar; metin satırlarını keser ama kısa sürede biter | kolay | [1] |
| 26 | `diyagonal-sag-alt` | geçiş | Sağ alt köşeden çapraz açılış | ffmpeg xfade `diagbr`; veya kendi piksel maskemiz (iki karenin kaynak karışımı) | Çapraz sert kenar; metin satırlarını keser ama kısa sürede biter | kolay | [1] |
| 27 | `dilim-sol` | geçiş | Yatay dilimler soldan sırayla gelir | ffmpeg xfade `hlslice`; veya kendi piksel maskemiz (iki karenin kaynak karışımı) | Dilim hareketi; ince dilimler LED’de titreyebilir, en az 8 piksel kalınlık | orta | [1] |
| 28 | `dilim-sag` | geçiş | Yatay dilimler sağdan sırayla gelir | ffmpeg xfade `hrslice`; veya kendi piksel maskemiz (iki karenin kaynak karışımı) | Dilim hareketi; ince dilimler LED’de titreyebilir, en az 8 piksel kalınlık | orta | [1] |
| 29 | `dilim-yukari` | geçiş | Dikey dilimler alttan yukarı gelir | ffmpeg xfade `vuslice`; veya kendi piksel maskemiz (iki karenin kaynak karışımı) | Dilim hareketi; ince dilimler LED’de titreyebilir, en az 8 piksel kalınlık | orta | [1] |
| 30 | `dilim-asagi` | geçiş | Dikey dilimler üstten aşağı gelir | ffmpeg xfade `vdslice`; veya kendi piksel maskemiz (iki karenin kaynak karışımı) | Dilim hareketi; ince dilimler LED’de titreyebilir, en az 8 piksel kalınlık | orta | [1] |
| 31 | `yatay-bulanik` | geçiş | Yatay hareket bulanıklığıyla geçiş | ffmpeg xfade `hblur`; veya kendi piksel maskemiz (iki karenin kaynak karışımı) | Özel; önizlemede LED’de test edin | orta | [1] |
| 32 | `gri-gecis` | geçiş | Gri tonlamaya düşüp yeni sahneye karışır | ffmpeg xfade `fadegrays`; veya kendi piksel maskemiz (iki karenin kaynak karışımı) | Renk üzerinden geçiş; geniş beyaz alan LED’de göz yorar, parlaklık sınırlı | orta | [1] |
| 33 | `kose-sol-ust` | geçiş | Sol üst köşeden köşeye silme | ffmpeg xfade `wipetl`; veya kendi piksel maskemiz (iki karenin kaynak karışımı) | Çapraz sert kenar; metin satırlarını keser ama kısa sürede biter | kolay | [1] |
| 34 | `kose-sag-ust` | geçiş | Sağ üst köşeden köşeye silme | ffmpeg xfade `wipetr`; veya kendi piksel maskemiz (iki karenin kaynak karışımı) | Çapraz sert kenar; metin satırlarını keser ama kısa sürede biter | kolay | [1] |
| 35 | `kose-sol-alt` | geçiş | Sol alt köşeden köşeye silme | ffmpeg xfade `wipebl`; veya kendi piksel maskemiz (iki karenin kaynak karışımı) | Çapraz sert kenar; metin satırlarını keser ama kısa sürede biter | kolay | [1] |
| 36 | `kose-sag-alt` | geçiş | Sağ alt köşeden köşeye silme | ffmpeg xfade `wipebr`; veya kendi piksel maskemiz (iki karenin kaynak karışımı) | Çapraz sert kenar; metin satırlarını keser ama kısa sürede biter | kolay | [1] |
| 37 | `sikis-yatay` | geçiş | Eski sahne yatayda sıkışır, yeni sahne genişler | ffmpeg xfade `squeezeh`; veya kendi piksel maskemiz (iki karenin kaynak karışımı) | Özel; önizlemede LED’de test edin | orta | [1] |
| 38 | `sikis-dikey` | geçiş | Eski sahne dikeyde sıkışır, yeni sahne genişler | ffmpeg xfade `squeezev`; veya kendi piksel maskemiz (iki karenin kaynak karışımı) | Özel; önizlemede LED’de test edin | orta | [1] |
| 39 | `yakin-gec` | geçiş | Eski sahneye yaklaşılırken yeni sahneye karışılır | ffmpeg xfade `zoomin`; veya kendi piksel maskemiz (iki karenin kaynak karışımı) | Özel; önizlemede LED’de test edin | orta | [1] |
| 40 | `hizli-fade` | geçiş | Başta hızlı, sonda yavaş karışım | ffmpeg xfade `fadefast`; veya kendi piksel maskemiz (iki karenin kaynak karışımı) | Renk üzerinden geçiş; geniş beyaz alan LED’de göz yorar, parlaklık sınırlı | kolay | [1] |
| 41 | `yavas-fade` | geçiş | Başta yavaş, sonda hızlı karışım | ffmpeg xfade `fadeslow`; veya kendi piksel maskemiz (iki karenin kaynak karışımı) | Renk üzerinden geçiş; geniş beyaz alan LED’de göz yorar, parlaklık sınırlı | kolay | [1] |
| 42 | `ruzgar-sol` | geçiş | Rüzgar çizgileri soldan geçerek açar | ffmpeg xfade `hlwind`; veya kendi piksel maskemiz (iki karenin kaynak karışımı) | Rüzgar çizgileri; ince çizgi dokusu LED’de titreşir, kısa tutun | orta | [1] |
| 43 | `ruzgar-sag` | geçiş | Rüzgar çizgileri sağdan geçerek açar | ffmpeg xfade `hrwind`; veya kendi piksel maskemiz (iki karenin kaynak karışımı) | Rüzgar çizgileri; ince çizgi dokusu LED’de titreşir, kısa tutun | orta | [1] |
| 44 | `ruzgar-yukari` | geçiş | Dikey rüzgar çizgileri alttan geçer | ffmpeg xfade `vuwind`; veya kendi piksel maskemiz (iki karenin kaynak karışımı) | Rüzgar çizgileri; ince çizgi dokusu LED’de titreşir, kısa tutun | orta | [1] |
| 45 | `ruzgar-asagi` | geçiş | Dikey rüzgar çizgileri üstten geçer | ffmpeg xfade `vdwind`; veya kendi piksel maskemiz (iki karenin kaynak karışımı) | Rüzgar çizgileri; ince çizgi dokusu LED’de titreşir, kısa tutun | orta | [1] |
| 46 | `ort-sol` | geçiş | Yeni sahne sağdan gelip eskinin üstünü örter (eski yerinde kalır) | ffmpeg xfade `coverleft`; veya kendi piksel maskemiz (iki karenin kaynak karışımı) | Yeni sahne eskinin üstüne biner; metin bozulmaz, en güvenli aile | kolay | [1] |
| 47 | `ort-sag` | geçiş | Yeni sahne soldan gelip eskiyi örter | ffmpeg xfade `coverright`; veya kendi piksel maskemiz (iki karenin kaynak karışımı) | Yeni sahne eskinin üstüne biner; metin bozulmaz, en güvenli aile | kolay | [1] |
| 48 | `ort-yukari` | geçiş | Yeni sahne alttan gelip eskiyi örter | ffmpeg xfade `coverup`; veya kendi piksel maskemiz (iki karenin kaynak karışımı) | Yeni sahne eskinin üstüne biner; metin bozulmaz, en güvenli aile | kolay | [1] |
| 49 | `ort-asagi` | geçiş | Yeni sahne üstten gelip eskiyi örter | ffmpeg xfade `coverdown`; veya kendi piksel maskemiz (iki karenin kaynak karışımı) | Yeni sahne eskinin üstüne biner; metin bozulmaz, en güvenli aile | kolay | [1] |
| 50 | `ac-sol` | geçiş | Eski sahne sola kayıp altındaki yeniyi açığa çıkarır | ffmpeg xfade `revealleft`; veya kendi piksel maskemiz (iki karenin kaynak karışımı) | Eski sahne kayıp alttakini açar; okunaklı | kolay | [1] |
| 51 | `ac-sag` | geçiş | Eski sahne sağa kayıp yeniyi açar | ffmpeg xfade `revealright`; veya kendi piksel maskemiz (iki karenin kaynak karışımı) | Eski sahne kayıp alttakini açar; okunaklı | kolay | [1] |
| 52 | `ac-yukari` | geçiş | Eski sahne yukarı kayıp yeniyi açar | ffmpeg xfade `revealup`; veya kendi piksel maskemiz (iki karenin kaynak karışımı) | Eski sahne kayıp alttakini açar; okunaklı | kolay | [1] |
| 53 | `ac-asagi` | geçiş | Eski sahne aşağı kayıp yeniyi açar | ffmpeg xfade `revealdown`; veya kendi piksel maskemiz (iki karenin kaynak karışımı) | Eski sahne kayıp alttakini açar; okunaklı | kolay | [1] |
| 54 | `zoom-capraz` | geçiş | Eski sahneye hızla yaklaşırken yeni sahneye zoom ile çıkılır (CrossZoom) | gl-transitions CrossZoom; kendi piksel kodu | Hareket bulanıklığı LED’de bantlaşır; süre 1 sn ve üstü, kuvvet düşük | orta | [2], [3] |
| 55 | `ruya` | geçiş | Yumuşak bulanık, rüya gibi karışım (Dreamy) | gl-transitions Dreamy | Yumuşak; metin kısa süre okunmaz, kısa tutun | orta | [2], [3] |
| 56 | `ruya-zoom` | geçiş | Rüya karışımı ve hafif zoom (DreamyZoom) | gl-transitions DreamyZoom | Bulanıklık fazlaysa metin kaybolur | orta | [2], [3] |
| 57 | `yon-bukum` | geçiş | Yönlü bükülme ile geçiş (DirectionalWarp) | gl-transitions DirectionalWarp | Bükülme metni bozar; kısa süre | orta | [2], [3] |
| 58 | `girdap` | geçiş | Merkezden dönen girdap (Swirl) | gl-transitions Swirl | Güçlü dönme LED’de rahatsız edebilir; 1 sn altı yok | zor | [2], [3] |
| 59 | `dalgacik` | geçiş | Su dalgacığı yayılır (Ripple) | gl-transitions Ripple | Yüksek frekanslı halka LED’de titrer; genlik düşük | orta | [2], [3] |
| 60 | `ruzgargulu` | geçiş | Pervane gibi dönen dilimler (Pinwheel) | gl-transitions Pinwheel | Net kenar; okunaklı | orta | [2], [3] |
| 61 | `altigen` | geçiş | Altıgen bloklara bölünerek geçiş (Hexagonalize) | gl-transitions Hexagonalize | Blok büyüklüğü 24 piksel ve üstü | orta | [2], [3] |
| 62 | `petek` | geçiş | Petek hücreleri sırayla açılır (Honeycomb) | gl-transitions Honeycomb | Net kenar; okunaklı | orta | [2], [3] |
| 63 | `kup-3b` | geçiş | 3B küp dönüşü (Cube) | gl-transitions Cube | Perspektif metni eğer; 1 sn üstü, dönüş yavaş | zor | [2], [3] |
| 64 | `papyon` | geçiş | Papyon şeklinde açılır (BowTie) | gl-transitions BowTie | Şekil maske; okunaklı | orta | [2], [3] |
| 65 | `kaleydoskop` | geçiş | Simetrik kaleydoskop kırılması (Kaleidoscope) | gl-transitions Kaleidoscope | Yoğun desen LED’de karışık; kısa ve düşük parlaklık | zor | [2], [3] |
| 66 | `puantiye` | geçiş | Büyüyen daire noktalarla perde (PolkaDotsCurtain) | gl-transitions PolkaDotsCurtain | Noktalar 30 piksel ve üstü; net | orta | [2], [3] |
| 67 | `ruzgar-savur` | geçiş | Rüzgar savurması (Wind) | gl-transitions Wind | İnce çizgi dokusu titrer | orta | [2], [3] |
| 68 | `su-damlasi` | geçiş | Su damlası çarpması ve halka (WaterDrop) | gl-transitions WaterDrop | Halka genliği düşük tutulmalı | orta | [2], [3] |
| 69 | `kalp-silme` | geçiş | Kalp şeklinde büyüyerek açılır (HeartWipe) | gl-transitions HeartWipe | Net şekil; kampanya temalı | orta | [2], [3] |
| 70 | `yildiz-silme` | geçiş | Yıldız şeklinde büyüyerek açılır (StarWipe) | gl-transitions StarWipe | Net kenar; okunaklı | orta | [2], [3] |
| 71 | `sarmal-silme` | geçiş | Sarmal desende kaplar (SpiralWipe) | gl-transitions SpiralWipe | Sert kenar; orta hız | orta | [2], [3] |
| 72 | `cicek-silme` | geçiş | Çiçek yaprakları şeklinde açılır (FlowerWipe) | gl-transitions FlowerWipe | Net; bahar kampanyası uygun | orta | [2], [3] |
| 73 | `anahtar-deligi` | geçiş | Anahtar deliği biçiminde daralıp açılır (KeyholeWipe) | gl-transitions KeyholeWipe | Net; merkezi odak | orta | [2], [3] |
| 74 | `erime-sutun` | geçiş | Sütunlar farklı hızla aşağı eriyip yeni sahneyi açar (DoomMelt) | gl-transitions DoomMelt | Sütun genişliği 16 piksel ve üstü | orta | [2], [3] |
| 75 | `sayfa-kivrim` | geçiş | Sayfa köşeden kıvrılır (InvertedPageCurl) | gl-transitions InvertedPageCurl | Gölge degradesi bantlaşır | zor | [2], [3] |
| 76 | `kitap-cevir` | geçiş | Kitap sayfası gibi çevrilir (BookFlip) | gl-transitions BookFlip | Perspektif; yavaş | zor | [2], [3] |
| 77 | `katla` | geçiş | Sahne yatay katlanır (Fold) | gl-transitions Fold | Net; okunaklı | orta | [2], [3] |
| 78 | `bolme-levha` | geçiş | Havaalanı panosu gibi levhalar döner (SplitFlap) | gl-transitions SplitFlap | Pano görünümü, yavaş çevirme; tabela hissine çok uygun | zor | [2], [3] |
| 79 | `led-duvar` | geçiş | LED duvar karoları sırayla yanar (LedWall) | gl-transitions LedWall | LED ekran temalı; karo 20 piksel ve üstü | orta | [2], [3] |
| 80 | `rastgele-kare` | geçiş | Rastgele karelerle dolar (RandomSquares) | gl-transitions RandomSquares | Kare 24 piksel ve üstü; titreme riski düşük | kolay | [2], [3] |
| 81 | `blok-ort` | geçiş | Bloklar sırayla örter (BlockCover) | gl-transitions BlockCover | Net; okunaklı | kolay | [2], [3] |
| 82 | `daire-zoom` | geçiş | İç içe daireler zoom ile açılır (ZoomInCircles) | gl-transitions ZoomInCircles | Halka sayısı az olsun | orta | [2], [3] |
| 83 | `film-yanik` | geçiş | Film yanığı turuncu parlama (FilmBurn) | gl-transitions FilmBurn | Parlak turuncu alan geniş; parlaklık sınırlı | orta | [2], [3] |
| 84 | `renk-evre` | geçiş | Renk kanalları kayarak geçer (ColorPhase) | gl-transitions ColorPhase | Renk titreşimi; kısa | orta | [2], [3] |
| 85 | `luma-silme` | geçiş | Gri tonlu bir desen haritasına göre açılır (LumaWipe) | gl-transitions LumaWipe; harita PNG | Harita yumuşaksa bantlaşır | zor | [2], [3] |
| 86 | `yilan-blok` | geçiş | Bloklar yılan gibi ilerler (SnakeBlockWipe) | gl-transitions SnakeBlockWipe | Net; orta hız | orta | [2], [3] |
| 87 | `panjur-silme` | geçiş | Deklanşör kanatları gibi açılır (ShutterWipe) | gl-transitions ShutterWipe | Net | orta | [2], [3] |
| 88 | `testere-silme` | geçiş | Testere dişli kenarla silme (SawtoothWipe) | gl-transitions SawtoothWipe | Diş 20 piksel ve üstü | kolay | [2], [3] |
| 89 | `arti-silme` | geçiş | Artı şeklinde büyür (PlusWipe) | gl-transitions PlusWipe | Net | kolay | [2], [3] |
| 90 | `tarama-erime` | geçiş | Düzenli desenle erime (DitherDissolve) | gl-transitions DitherDissolve | Gürültü LED’de gren; kısa, kare ızgaralı seçin | orta | [2], [3] |
| 91 | `gurultu-erime` | geçiş | Gürültü haritasıyla erime (NoiseDissolve) | gl-transitions NoiseDissolve | Gren/titreme riski; düşük frekans | orta | [2], [3] |
| 92 | `perlin-erime` | geçiş | Perlin gürültüsü ile organik erime (PerlinDissolve) | gl-transitions PerlinDissolve | Yumuşak; bantlaşma olabilir | orta | [2], [3] |
| 93 | `tarama-cizgi-gec` | geçiş | Çapraz tarama çizgileriyle geçiş (Crosshatch) | gl-transitions Crosshatch | İnce çizgi titreşir; kalın çizgi seçin | orta | [2], [3] |
| 94 | `pop-yay` | giriş | Küçükten fırlayıp hafif taşar, iki küçük salınımla oturur | olcek 0.2, 1.15, 0.96, 1 | Hafif salınım okunaklı | kolay | [11] |
| 95 | `drift-sag` | giriş | Sağdan çok yavaş süzülerek gelir | dx=0.35 parlak=0; sure 2.4 | Çok okunaklı, sakin | kolay | [11] |
| 96 | `drift-sol` | giriş | Soldan çok yavaş süzülerek gelir | dx=-0.35 parlak=0; sure 2.4 | Çok okunaklı | kolay | [11] |
| 97 | `tumble-ters` | giriş | Baş aşağı başlayıp dönerek düşer ve oturur | aci=180 dy=-0.8 | Dönüş sırasında okunmaz; 1.2 sn | kolay | [11] |
| 98 | `baseline-hizli` | giriş | Hızlı ve kısa bir yükseliş | dy=0.25; sure 0.6 | Çok okunaklı | kolay | [11] |
| 99 | `yukselis-blok` | giriş | Metin bloğu bütün olarak alttan yükselir | dy=0.5; sure 1.0 | İyi | kolay | [11] |
| 100 | `yukari-ascend` | giriş | Alttan hafif büyüyerek yükselir | dy=0.4 olcek=0.9 | İyi | kolay | [11] |
| 101 | `yuvarlan-gir` | giriş | Soldan dönerek yuvarlanır gelir | dx=-1.1 aci=-360; sure 1.4 | Dönüş uzunsa okunmaz | kolay | [11] |
| 102 | `kayma-skate` | giriş | Zemin boyunca kayıp durur | dx=-1.0 dy=0.1 olcek=0.95; egri yavasla | İyi | kolay | [11] |
| 103 | `dam-dusus` | giriş | Üstten düşüp iki kez zıplar | dy=-1.2; 0.5: dy=0.04; 0.7: dy=-0.02 | Zıplama okunaklı, strobe yok | orta | [11], [17] |
| 104 | `yay-yagi` | giriş | Yaylanarak yukarıdan gelir, küçük yalpa | dy=-0.9 aci=3 | İyi | kolay | [11] |
| 105 | `basin-ezme` | giriş | Büyük boyuttan ezilerek oturur (Stomp) | olcek 1.6 parlak 0.3, sonra 0.95, sonra 1 | Güçlü vurgu; tek sefer | kolay | [11] |
| 106 | `levha-kayma` | giriş | İki levha gibi iki yönden kayıp birleşir | satır dx işareti dönüşümlü | Satır ayrımı gerekir | orta | [11] |
| 107 | `yuvarlan-sol-kucuk` | giriş | Sol kenardan dönerek ve büyüyerek gelir | dx=-1 aci=-180 olcek=0.6 | Dönüş yavaş olsun | kolay | [12] |
| 108 | `katla-ac` | giriş | Dikey eksende katlanıp açılır | aci=90 olcek=0.8 derinlik=1.5 | Katlanırken ince çizgi titreşebilir | orta | [12] |
| 109 | `ters-ucus` | giriş | Ters köşeden çapraz uçarak gelir | dx=0.8 dy=-0.8 aci=20 | İyi | kolay | [12] |
| 110 | `flutter-kanat` | giriş | Kanat çırpar gibi küçük açısal salınımla gelir | aci +8, -8, +4; dy=0.2 | Hafif salınım iyi | orta | [12] |
| 111 | `ince-yaya` | giriş | Ortadan iki yana yatay genişler | yatay ölçek 0.2 den 1 e (yeni alan: olcek-x) | Yatay ölçek anahtarı yoksa yeni alan gerekir | orta | [6] |
| 112 | `basamak-yan` | giriş | Satırlar yandan merdiven gibi kayarak gelir | tek-tek mantığı, dx kademeli | Çok okunaklı | kolay | [18] |
| 113 | `satir-ziplat` | giriş | Satırlar sırayla zıplayarak gelir | gecikme 0.3; dy salınım | Sıralı; okunaklı | kolay | [18] |
| 114 | `kelime-kelime-gir` | giriş | Sözcükler tek tek belirir | kelime ayırma gerekir (yeni alan) | En okunaklı yöntem | orta | [18] |
| 115 | `sarkac-yavas` | giriş | Büyük genlikli sarkaç yavaş yavaş durur | aci 18, -12, 7, -3, 0 | Sürekli salınım okunaklı | kolay | [11] |
| 116 | `sis-acil` | giriş | Sisin içinden netleşir (parlaklık ve ölçek) | olcek 1.3 den 1; parlak 0.2 den 1; sure 2 | Yumuşak; parlaklık adımı 0.05 ve üstü | kolay | [6] |
| 117 | `delip-gec` | giriş | Küçük merkez noktadan büyür | olcek 0.02, 1.1, 1 | İyi | kolay | [8] |
| 118 | `isik-yuvarlak` | giriş | Işık topundan açılarak büyür | olcek 0.1 parlak 1 | Parlak çekirdek; parlaklık sınırlı | orta | [6] |
| 119 | `bukul-gir` | giriş | Yaprak gibi bükülerek gelir | aci 30, -10, 0; dx=0.5 | İyi | kolay | [12] |
| 120 | `yoyo-gir` | giriş | Yukarı çıkıp aşağı dönerek iner | dy -0.2, 0.1, 0 | İyi | kolay | [11] |
| 121 | `zipla-sag-yavas` | giriş | Sağdan yavaşça zıplayarak girer | dx=1; üç azalan salınım | İyi | kolay | [11] |
| 122 | `cikis-ucar` | çıkış | Hızlanarak üste uçup kaybolur | dy=-1.3; egri hizlan | İyi | kolay | [11] |
| 123 | `cikis-eriyor` | çıkış | Aşağı ağırlıkla eriyerek kaybolur | dy=0.3 parlak 1 den 0; sure 1.6 | Yumuşak | kolay | [12] |
| 124 | `cikis-sicra` | çıkış | Hafifçe büyüyüp küçülerek çıkar | olcek 1.1 den 0 | Güçlü; tek sefer | kolay | [12] |
| 125 | `cikis-katla` | çıkış | Katlanıp yok olur | aci=90 olcek 0.7 | Katlanırken ince çizgi | orta | [12] |
| 126 | `cikis-sag-dalga` | çıkış | Sağa dalgalanarak çıkar | dx=1.2 ve dy salınım | İyi | kolay | [11] |
| 127 | `vurgu-nabiz-yavas` | vurgu | Metin yavaş nefes alır, 2 sn döngü, titreşimsiz | olcek 1, 1.03, 1 | Çok yumuşak, sürekli vurgu | kolay | [14], [15] |
| 128 | `vurgu-renk-gecis` | vurgu | Metin rengi paletin iki vurgu rengi arasında yumuşak gezinir | renk karışımı (yeni alan) | Kontrast korunmalı | orta | [13], [14] |
| 129 | `tarama-cizgisi` | efekt | Yatay ince ışık çizgisi yukarıdan aşağı süpürür | kendi piksel kodu (dekor.rs tarzı) | Çizgi 6 piksel ve üstü; sürekli değil | kolay | [13] |
| 130 | `radar-tarama` | efekt | Merkezden dönen radar kolu ve yumuşak iz | kendi piksel kodu | Yavaş (5 sn tur); titreme yok | orta | [13] |
| 131 | `isik-huzmesi` | efekt | Çapraz ışık huzmeleri yavaşça kayar | kendi piksel kodu; alfa maske | Metne parlak vurmasın | orta | [14] |
| 132 | `projektor-isik` | efekt | Spot ışık metnin üzerinde gezer | kendi piksel kodu | Kontrastı artırır | orta | [14] |
| 133 | `lens-parlama` | efekt | Mercek parlaması metnin üstünden geçer | kendi piksel kodu; halkalar ve çizgi | Parlak alan sınırlı | orta | [7] |
| 134 | `kar-yogun` | efekt | Yoğun kar fırtınası (iki derinlik katmanı) | mevcut kar kodu çoğaltılır | Taneler 3 piksel ve üstü; metni örtmesin | kolay | [13] |
| 135 | `sis-akis` | efekt | Alttan yükselen yumuşak sis | kendi piksel kodu | Kontrast düşer, opaklık sınırlı | orta | [14] |
| 136 | `ates-yalin` | efekt | Alt kenarda yumuşak ateş yalımı | kendi piksel kodu; gürültü | Turuncu/kırmızı LED’de iyi; titreme kontrollü | zor | [8] |
| 137 | `duman-kivrim` | efekt | Duman kıvrımları yükselir | kendi piksel kodu | Hafif; arka plan katmanı | zor | [8] |
| 138 | `aurora-serit` | efekt | Aurora ışık şeritleri akar | kendi piksel kodu; sinüs bantlar | Bantlaşma riski; dither şart | zor | [14] |
| 139 | `yildiz-alan` | efekt | Parlayan yıldız alanı, derin paralaks | kendi piksel kodu | Küçük noktalar LED’de kaybolur; 3 piksel ve üstü | kolay | [13] |
| 140 | `ekolayzer` | efekt | Alt kenarda hareketli çubuklar | kendi piksel kodu | Çubuk 16 piksel ve üstü; hız düşük | kolay | [14] |
| 141 | `matris-yagmuru` | efekt | Düşen harf/rakam sütunları (CodeRain benzeri) | kendi piksel kodu, font | İnce harf okunmaz, düşük opaklık | orta | [3] |
| 142 | `sabun-kopugu` | efekt | Yükselen yarı saydam köpükler | balon efektinin türevi | Hafif | kolay | [14] |
| 143 | `isik-damla` | efekt | Yukarıdan akan ışık damlaları | kendi piksel kodu | Hafif | kolay | [14] |
| 144 | `yazi-golge-uzun` | efekt | Metin için uzun, yumuşak gölge (derinlik hissi) | maske kaydırma | Okunurluğu artırır | kolay | [6] |
| 145 | `yazi-neon-hale` | efekt | Metin çevresinde neon hale | bulanık maske | Parlaklık sınırı; halo geniş olmasın | orta | [11] |
| 146 | `yazi-cizgi-ic` | efekt | Metin içi çizgi dolgu animasyonu | kendi piksel kodu | İnce çizgi titreşir | orta | [6] |
| 147 | `yazi-izi` | efekt | Hareketli metin arkasında sönen iz (motion trail) | önceki karelerin alfa birikimi | Hızlı harekette okunurluğu artırır | orta | [17] |
| 148 | `kromatik-hafif` | efekt | Sürekli çok hafif RGB kayması (1-2 piksel) | kanal kayması | LED alt pikselleriyle çakışabilir; test şart | orta | [7] |
| 149 | `sparkle-dalga` | efekt | Pırıltı dalgası yazı boyunca gezer | pirilti türevi; maske | İyi | kolay | [11] |
| 150 | `halka-cift` | efekt | Büyüyen çift halka, yavaş | halka türevi | İyi | kolay | [14] |
| 151 | `serit-isik` | efekt | Kenar boyunca ışık şeridi akar | cerceve-isik türevi | İyi | kolay | [14] |
| 152 | `kor-parcacik` | efekt | Yükselen kor parçacıkları | parçacık türevi, turuncu | İyi | kolay | [8] |
| 153 | `yazi-kelime-kelime` | yazı | Sözcükler tek tek belirir (sıralı) | kelime ayırma ve gecikme | En okunaklı; reklam metni için | orta | [17], [18] |
| 154 | `yazi-harf-merdiven` | yazı | Harfler birbiri ardına yukarı çıkar | harf maskesi ve kademeli dy | Kısa sözcük (8 harf ve altı) | orta | [17], [18] |
| 155 | `yazi-harf-dalga` | yazı | Harfler sırayla dalga gibi kalkıp iner | harf maskesi ve sinüs dy | Sürekli ise 2 sn döngü | orta | [17] |
| 156 | `yazi-maske-blok` | yazı | Blok metni örter, kayarak ortaya çıkarır | dikdörtgen maske | Çok okunaklı, net | kolay | [11], [17] |
| 157 | `yazi-maske-yukari` | yazı | Satırlar alt kenar maskesinden yükselerek ortaya çıkar | satır başına kırpma ve dy | Editoryal; net | kolay | [17] |
| 158 | `yazi-fosforlu` | yazı | Sözcüğün arkasına fosforlu kalem şeridi çizilir | dikdörtgen dolgu | Şerit rengi kontrast koşulu | kolay | [17] |
| 159 | `yazi-altcizgi` | yazı | Alt çizgi soldan sağa çizilir | çizgi uzunluğu k | Vurgu için ideal | kolay | [17] |
| 160 | `yazi-karaoke` | yazı | Sözcükler sırayla renk dolgusu alır | kelime sırasına göre renk | Okuma ritmi | orta | [17] |
| 161 | `yazi-egim` | yazı | Metin eğik (skew) kayıp düzelir | yatay shear ve dx | Eğikken okunmaz; kısa | orta | [6] |
| 162 | `yazi-flip-harf` | yazı | Harfler sırayla döner (flip) | harf maskesi ve aci | Kısa kelime; yavaş | orta | [11] |
| 163 | `yazi-cozulme` | yazı | Karışık harfler çözülüp doğru yazıya dönüşür (decode) | harf sırasıyla rastgele karakter | Karışık aşama kısa (0.6 sn altı) | orta | [3] |
| 164 | `yazi-sayac` | yazı | Rakam artarak hedef fiyata ulaşır (₺ tutar) | sayı enterpolasyonu | Hedef değer uzun süre sabit durmalı | orta | [13] |
| 165 | `yazi-fiyat-vur` | yazı | Fiyat büyük boyuttan vurularak yerine oturur | basin-ezme ve gölge | Güçlü vurgu; tek sefer | kolay | [11] |
| 166 | `yazi-kayan-yazi` | yazı | Tek satır yatay kayan yazı (ticker, sağdan sola) | piksel kaydırma, sonsuz | LED’in klasik biçimi; hız en çok 150 piksel/sn (önerilen, ölçülmedi) | kolay | [14], [15] |
| 167 | `yazi-dikey-kayan` | yazı | Dikey akan satırlar (alttan yukarı) | piksel kaydırma | Satır arası boşluk; hız düşük | kolay | [14], [15] |
| 168 | `yazi-gradyan-akan` | yazı | Metin içinde yavaş akan renk geçişi | metin maskesi ve gradyan | Kontrast kontrolü; bantlaşma | orta | [6] |
| 169 | `yazi-boyut-dalga` | yazı | Harf boyutları sırayla büyüyüp küçülür | harf maskesi ve ölçek | Taşma kontrolü | orta | [17] |
| 170 | `yazi-satir-ayri` | yazı | Her satır farklı yönden gelir (sol-sağ dönüşümlü) | satır başına dx işareti | Okunaklı | kolay | [11], [18] |
| 171 | `zemin-bokeh` | arka plan | Yumuşak bokeh daireleri yavaşça süzülür | kendi piksel kodu | Hafif; metin kontrastı korunmalı | orta | [14] |
| 172 | `zemin-ucgen` | arka plan | Üçgen mozaik, yavaş renk değişimi | kendi piksel kodu | Düz alanlar; bantlaşma az | orta | [14] |
| 173 | `zemin-altigen` | arka plan | Altıgen hücreler yavaşça yanıp söner | kendi piksel kodu | Yavaş (3 sn üstü) | orta | [3] |
| 174 | `zemin-sinus` | arka plan | İnce sinüs çizgileri yatay akar | kendi piksel kodu | İnce çizgi titreşir; 6 piksel ve üstü | kolay | [14] |
| 175 | `zemin-radyal` | arka plan | Merkezden ışın ışın yayılan hafif ışık | kendi piksel kodu | Hafif | kolay | [14] |
| 176 | `zemin-sivi` | arka plan | Sıvı gibi akan renk lekeleri (düşük frekans) | gürültü alanı ve palet | Bantlaşma için dither | zor | [14] |
| 177 | `zemin-akis-cizgi` | arka plan | Hız çizgileri soldan sağa akar | kendi piksel kodu | Hız düşük | kolay | [14] |
| 178 | `zemin-kare-nabiz` | arka plan | Kareler merkezden dışa nabız gibi | kendi piksel kodu | Yavaş | kolay | [14] |
| 179 | `zemin-yildiz-kayma` | arka plan | Üç katmanlı yıldız kayması (paralaks) | kendi piksel kodu | Noktalar 3 piksel ve üstü | kolay | [13] |
| 180 | `zemin-gradyan-donen` | arka plan | Yavaş dönen ikili renk gradyanı | kendi piksel kodu | Bantlaşma: dither kullanılmalı | kolay | [14] |
| 181 | `sinus-cik` | eğri | Sine out: yumuşak yavaşlama | anim::egri_k yeni kol | Genel kullanım iyi | kolay | [4] |
| 182 | `sinus-gir-cik` | eğri | Sine in-out | egri_k | İyi | kolay | [4] |
| 183 | `kuad-gir-cik` | eğri | Quad in-out: dengeli | egri_k | İyi | kolay | [4] |
| 184 | `kubik-gir-cik` | eğri | Cubic in-out: belirgin ivme ve yavaşlama | egri_k | İyi | kolay | [4] |
| 185 | `kuart-cik` | eğri | Quart out: çok yumuşak duruş | egri_k | Okunaklı | kolay | [4] |
| 186 | `kuint-cik` | eğri | Quint out: daha uzun yumuşak duruş | egri_k | Okunaklı | kolay | [4] |
| 187 | `daire-cik` | eğri | Circ out: son anda hızlı frenleme | egri_k | Sert duruş; dikkat | kolay | [4] |
| 188 | `geri-gir` | eğri | Back in: geri çekilip hızla fırlama | egri_k | Güçlü; kısa | kolay | [4] |
| 189 | `geri-gir-cik` | eğri | Back in-out: iki uçta taşma | egri_k | Hareket çok; kısa | kolay | [4] |
| 190 | `esnek-gir` | eğri | Elastic in | egri_k | Titrer; küçük genlik | orta | [4] |
| 191 | `sek-gir` | eğri | Bounce in | egri_k | Okunaklı | orta | [4] |
| 192 | `ustel-gir-cik` | eğri | Expo in-out | egri_k | Hızlı; dikkat | kolay | [4] |
| 193 | `adim-4` | eğri | Basamaklı hareket (CSS steps benzeri), LED için kare kare akış | egri_k; kare sayısı parametre | Kasıtlı basamaklı; kare aralığı 2 ve üstü | kolay | [5] |
| 194 | `bezier-ozel` | eğri | Kullanıcı tanımlı cubic-bezier(x1,y1,x2,y2) eğrisi | egri_k ve yeni egri bezier satırı | Usta ayarı | orta | [5] |

## 4. Uygulama yolları ve sıralama
### 4.1 Sıra
1. **ffmpeg xfade geçişleri (53 adet).** Resmi listede 58 hazır geçiş vardır [1]; mevcut 19 geçişle eşdeğer olan 5'i hariç kalan 53 tanesi önerildi. Hepsi tek formül ailesidir, aynı kod iskeleti ile gelir.
2. **Anahtar kare animasyonları (35 adet).** `animasyonlar/yeni4.txt` gibi yeni bir dosya ve biçim bilinen (anahtar k alan=değer). Derleme gerekmez.
3. **Yazı ve efektler (42 adet).** `dekor.rs`'ye yeni fonksiyonlar; harf/kelime maskesi için `anim::maske` genişletilir.
4. **Arka planlar (10 adet).** `dekor.rs` zemin-*; düşük frekanslı, dither ile.
5. **gl-transitions seçkisi (40 adet).** GLSL kodunu Rust piksel koduna çevirmek gerekir; depo 100+ geçiş, topluluk yazımı [2] (lisans: doğrulanmadı, kullanmadan önce depodaki lisans dosyası okunmalı).
6. **Eğriler (14 adet).** `egri_k` içine yeni kollar; easings.net adları [4], css cubic-bezier ve steps mantığı [5].

### 4.2 Teknik yol seçimi (xfade)
- **Yol A: ffmpeg xfade filtresi.** Her sahne ayrı klip render edilir, `xfade=transition=ADI:duration=D:offset=O` ile birleştirilir [1]. Artı: 58 geçiş hazır, kod yazmak yok. Eksi: iki girdinin aynı çözünürlük, piksel biçimi, kare hızı ve zaman tabanı olması gerekir [1]; ikinci kodlama kalite kaybı yapar; sahne bazlı hızlı önizleme zor.
- **Yol B: kendi piksel kodumuz.** Geçiş formülü Rust'ta iki kare arasında hesaplanır (mevcut 19 geçiş zaten böyle). Artı: tek geçişte kodlama, tam denetim (LED titreme/dither), önizleme aynı koddan. Eksi: her formülü yazmak gerekir. xfade'in kaynak kodunun (libavfilter, vf_xfade.c) formülleri yol gösterebilir; dosyayı bu çalışmada okumadım, lisansını doğrulamadım (doğrulanmadı).
- **Tavsiye:** Yol B, önce 10 geçişlik bir "ailelerden birer örnek" ile (sil-yukari, kaydir-sol, ort-sol, ac-sol, yumusak-sol, dilim-sol, diyagonal-sol-ust, daire-kapan, ruzgar-sol, nokta-erime); formül ailesi aynı olduğu için kalanı kolaydır. Yol A "hızlı deneme" için yedek olabilir.

### 4.3 Riskler
| Yol | CPU süresi | Okunabilirlik | LED titreme / bantlaşma |
|---|---|---|---|
| xfade ailesi (sil, kaydır, ört, aç) | Düşük (maske + kopya) | Yüksek; ört/aç ailesi metni bozmaz [1] | Düşük; yumuşak kenarlı olanlarda bantlaşma, 0.8 sn üstü ve dither önerilir |
| xfade dilim/rüzgar | Düşük | Orta | İnce şerit titreşebilir; dilim kalınlığı en az 8 piksel |
| xfade blur/sıkıştır/zoom | Orta (ölçekleme, bulanıklık) | Orta | Bulanıklıkta bantlaşma; kısa süre |
| gl-transitions (gürültü, girdap, kaleydoskop, 3B) | Orta-yüksek (piksel başına gürültü, perspektif) | Düşük-orta | Gren ve titreme riski; yalnız kısa ve düşük kontrastla; kullanıcı onaylı "dikkatli" etiketi |
| Anahtar kare animasyonları | Düşük (mevcut motor) | Yüksek | Dönüş ve zıplamada kısa süre; strobe yok |
| Efekt/dekor katmanları | Orta (her kare parçacık/gürültü) | Metni örtmeyecek opaklık | Küçük parçacık LED'de kaybolur (3 piksel altı); sürekli yanıp sönme yok |
| Yazı harf/kelime animasyonları | Orta (harf maskesi) | Kelime düzeyi yüksek, harf düzeyi uzun metinde düşük [18] | Harf başına 30-50 ms web kuralı [17]; LED'de ritim yavaş tutulmalı |
| Eğriler | Ihmal edilebilir | Aşırı taşan eğriler (geri, esnek) dikkat dağıtır | Basamaklı eğri (`adim-4`) bilinçli LED hissi verir |

Genel kısıtlar: ana mesaj durağan, tek öğe hareketli, hızlı kesme ve kısa metin flaşı yok [14] [16]; en az 4.5:1 kontrast, hedef 7:1 [14]; metin yüksekliği izleme mesafesine göre [13] (rakamlar satıcı kaynaklı, ölçülmedi).

### 4.4 Anahtar kare dosyası örneği
Mevcut biçimle (temel.txt) aynıdır; yeni dosya `animasyonlar/yeni4.txt` olabilir:
```
animasyon pop-yay
aciklama Küçükten fırlayıp hafif taşar, iki küçük salınımla oturur
sure 1.3
gecikme 0.15
anahtar 0 olcek=0.2 parlak=0
anahtar 0.55 olcek=1.15 parlak=1
anahtar 0.78 olcek=0.96
anahtar 1
```
Yeni alan gerektirenler (`olcek-x`, harf/kelime sırası, vurgu renk karışımı) "orta" olarak işaretlendi.

## 5. "Tarif ile video" bölümü
Mevcut `kurallar/tarif-kurallari.json` biçimi: `{"k":"kategori","h":"katalog-adı","i":["ifade1","ifade2"]}`; her `h` gerçek katalogda olmak zorundadır (cargo test denetler). Yeni her öğe için ayrı kural satırı eklenir; kural dosyası yalnız büyür (silme yok).

### 5.1 Genel bağlama adımları
1. Öğe önce kataloğa girer (GECISLER / .txt dosyası / EFEKTLER-dekor ADLAR / EGRILER); aksi hâlde kural testi reddeder.
2. `tarif-kurallari.json` içine `{k,h,i}` satırı eklenir. Yeni kategori gerekenler: `yazi` (yazı animasyonu), `egri` (hareket eğrisi); `zemin` ise mevcut `efekt` kategorisine konabilir (dekor zaten orada).
3. Ek ve çekim toleransı ayrıştırıcıdadır; ifadelere yalnız kök ve bir iki çekim yazmak yeter (ı/i, ş/s farkı yok sayılır).
4. LLM istem üreticisi (/bilgi verisi) kataloğu otomatik okur; ek bir iş yok.
5. Yeni geçişlerin tarif eşlemesi: "A'dan B'ye ... geçişle" kalıbı ile `gecis` kategorisi (şimdilik açılış/kapanış; sahneler arası geçiş alanı 7. bölümdeki editörle birlikte gelir).
6. Altın test: her kategoriden en az bir örnek tarif beklenen sahne listesine eklenir.

### 5.2 Kategori başına örnek ifadeler ve kural satırları
**Geçiş** (k=gecis)
Örnek ifadeler: "sağdan kayarak geçsin", "yukarı doğru silinsin", "yumuşak silinerek geçsin", "köşeden çapraz açılsın", "daire kapanarak geçsin", "rüzgar gibi geçsin", "pikselleşerek geçsin".
```
{"k":"gecis","h":"kaydir-sol","i":["sağdan kay","sola kay","sağdan itsin","kayarak geç"]},
{"k":"gecis","h":"sil-yukari","i":["yukarı sil","alttan yukarı sil","yukarı doğru silin"]},
{"k":"gecis","h":"yumusak-sol","i":["yumuşak sil","yumuşak silinerek","yumuşak kenar"]},
{"k":"gecis","h":"diyagonal-sol-ust","i":["köşeden çapraz","sol üstten aç","çapraz aç"]},
{"k":"gecis","h":"daire-kapan","i":["daire kapan","daireyle kapan","iris kapan"]},
{"k":"gecis","h":"pikselle","i":["pikselleş","pikselleşerek","blok blok"]}
```
Örnek tarif -> sahne: "İlk sahne İNDİRİM yukarıdan gelsin, sonra yumuşak silinerek ikinci sahneye geçsin" -> sahne1 giris=ustten, gecis=yumusak-sol; sahne2 ...

**Giriş** (k=giris; öğe animasyon dosyasında)
Örnek ifadeler: "yaylanarak pop gibi gelsin", "sağdan çok yavaş süzülsün", "baş aşağı düşsün", "sisin içinden belirsin", "satır satır zıplasın", "sözcük sözcük gelsin".
```
{"k":"giris","h":"pop-yay","i":["pop gibi","yaylanarak","patlayıp otur"]},
{"k":"giris","h":"drift-sag","i":["sağdan süzül","sağdan yavaş","süzülerek gel"]},
{"k":"giris","h":"tumble-ters","i":["baş aşağı","ters düş","takla"]},
{"k":"giris","h":"sis-acil","i":["sisin içinden","sis gibi","netleşerek"]},
{"k":"giris","h":"satir-ziplat","i":["satır satır zıpla","satırlar zıplasın"]}
```
Örnek: "ÇOK ÖZEL FİYAT sisin içinden belirsin, 5 sn" -> metin ["ÇOK ÖZEL FİYAT"], giris=sis-acil, sure=5.

**Çıkış** (k=cikis)
Örnek ifadeler: "yukarı uçarak çıksın", "aşağı doğru erisin", "küçülüp patlasın", "katlanarak kaybolsun", "sağa dalgalanarak çıksın".
```
{"k":"cikis","h":"cikis-ucar","i":["uçarak çık","yukarı uç","hızla yukarı çık"]},
{"k":"cikis","h":"cikis-eriyor","i":["erisin","eriyerek","aşağı eriyerek"]},
{"k":"cikis","h":"cikis-sicra","i":["büyüyüp sön","patlayarak sön"]},
{"k":"cikis","h":"cikis-katla","i":["katlanarak","katlan"]},
{"k":"cikis","h":"cikis-sag-dalga","i":["sağa dalgalan","dalgalanarak sağa"]}
```
Not: bugün 174 animasyonun çıkış kuralı yok; kural dosyası "giris" ile aynı adlar için "çıkış" kategorisini de tarayacak şekilde genişletilmeli (ayrıştırıcı tarafı).

**Efekt** (k=efekt; yazı efektleri, dekor ve arka plan ayrı alt kümeler)
Örnek ifadeler: "tarama çizgisi geçsin", "aurora arka plan", "ateş yalımı olsun", "sis yükselsin", "yıldızlar paralaks kaysın", "neon halesi olsun", "kıvılcımlı kor".
```
{"k":"efekt","h":"tarama-cizgisi","i":["tarama çizgisi","tarayıcı çizgi","ışık çizgisi"]},
{"k":"efekt","h":"aurora-serit","i":["aurora","kuzey ışığı","ışık şeritleri"]},
{"k":"efekt","h":"ates-yalin","i":["ateş yalımı","alev","ateşli"]},
{"k":"efekt","h":"sis-akis","i":["sis","sisli","yükselen sis"]},
{"k":"efekt","h":"yazi-neon-hale","i":["neon hale","neon halesi","neon parıltı"]},
{"k":"efekt","h":"zemin-bokeh","i":["bokeh","yumuşak daireler","bokeh zemin"]}
```
Örnek: "Yıldızlı zemin, neon halesi" -> efekt=[zemin-yildiz-kayma, yazi-neon-hale].

**Yazı** (yeni k=yazi; sahnede "yazi=<ad>" alanı gerekir, 0.9.x sahne şemasına eklenmeli)
Örnek ifadeler: "sözcük sözcük gelsin", "harfler dalga yapsın", "alt çizgisi çizilsin", "fiyat vurarak otursun", "rakam sayarak artsın", "yatay kayan yazı olsun".
```
{"k":"yazi","h":"yazi-kelime-kelime","i":["sözcük sözcük","kelime kelime","tek tek kelime"]},
{"k":"yazi","h":"yazi-harf-dalga","i":["harfler dalga","dalgalı harf","harf dalgası"]},
{"k":"yazi","h":"yazi-altcizgi","i":["alt çizgi","altı çizilsin","altını çiz"]},
{"k":"yazi","h":"yazi-fiyat-vur","i":["fiyat vur","fiyat vurarak","fiyatı vur"]},
{"k":"yazi","h":"yazi-sayac","i":["sayarak artsın","rakam sayaç","sayaç gibi"]},
{"k":"yazi","h":"yazi-kayan-yazi","i":["kayan yazı","ticker","şerit yazı"]}
```
Örnek: "Kayan yazı: YENİ SEZON GELDİ" -> yazi=yazi-kayan-yazi, metin tek satır.

**Arka plan** (k=efekt, h=zemin-*; istenirse yeni k=zemin ile ayrılabilir)
Örnek ifadeler: "yavaş dönen gradyan zemin", "sıvı gibi akan renkler", "altıgen zemin", "bokeh arka plan", "hız çizgili zemin", "yıldız kayması".
```
{"k":"efekt","h":"zemin-gradyan-donen","i":["dönen gradyan","dönen renk geçişi"]},
{"k":"efekt","h":"zemin-sivi","i":["sıvı gibi","akan renkler","sıvı zemin"]},
{"k":"efekt","h":"zemin-altigen","i":["altıgen zemin","petek zemin"]},
{"k":"efekt","h":"zemin-akis-cizgi","i":["hız çizgili","hız çizgileri","akış çizgisi"]},
{"k":"efekt","h":"zemin-yildiz-kayma","i":["yıldız kayması","kayan yıldız alanı"]}
```

**Vurgu** (k=giris veya yeni k=vurgu; vurgu-* zaten giris kategorisinde animasyondur)
Örnek ifadeler: "yavaş nefes alsın", "rengi gezsin", "vurgulansın", "nabız gibi atsın", "dikkat çeksin".
```
{"k":"giris","h":"vurgu-nabiz-yavas","i":["yavaş nefes","nefes alsın","sakin vurgu"]},
{"k":"giris","h":"vurgu-renk-gecis","i":["rengi gezsin","renk değiştirsin","renk gezinsin"]}
```

**Eğri** (yeni k=egri; mevcut `egri` alanıyla bağlanır)
Örnek ifadeler: "yumuşak dursun", "geri çekilip fırlasın", "basamak basamak ilerlesin", "çok yumuşak yavaşlasın", "esneyerek otursun".
```
{"k":"egri","h":"kuint-cik","i":["çok yumuşak dur","uzun yavaşla","çok yumuşak yavaşla"]},
{"k":"egri","h":"geri-gir","i":["geri çekilip fırla","geri çekil"]},
{"k":"egri","h":"adim-4","i":["basamak basamak","kademeli","adım adım ilerle"]},
{"k":"egri","h":"sinus-cik","i":["yumuşak dur","sinüs gibi"]},
{"k":"egri","h":"esnek-gir","i":["esneyerek başla","yay gibi başla"]}
```

### 5.3 Örnek tarif -> sahne eşlemesi (şema: docs/TARIF-ET-LLM.md bölüm 3)
Tarif: "Üç sahne. İlk sahnede HAFTANIN FIRSATI sözcük sözcük gelsin, 4 saniye. İkincisinde %30 İNDİRİM fiyat vurarak otursun, yıldız kayması zemin olsun, 5 saniye. Üçüncüsünde ÇOK YAKINDA yavaş nefes alsın, 4 saniye. Sahneler arası yumuşak silme, açılışta daire kapan."
```
{ "surum": 1, "ekran": "dikey", "gorunum": "duz",
  "sahneler": [
   { "metin": ["HAFTANIN","FIRSATI"], "giris": "kelime-kelime-gir", "cikis": "yok", "efekt": [], "sure": 4 },
   { "metin": ["%30","İNDİRİM"], "yazi": "yazi-fiyat-vur", "efekt": ["zemin-yildiz-kayma"], "sure": 5, "gecis_sonra": "yumusak-sol" },
   { "metin": ["ÇOK","YAKINDA"], "giris": "vurgu-nabiz-yavas", "sure": 4 } ],
  "acilis": "daire-kapan", "kapanis": "fade" }
```
`yazi` ve `gecis_sonra` alanları yeni şema alanlarıdır; sürüm numarası 2 olarak artırılmalı, sürüm 1 geriye uyumlu kalmalıdır (alan yoksa varsayılan).

## 6. Kaynaklar listesi
1. ffmpeg Filters Documentation, xfade bölümü (58 geçiş adı, duration/offset/expr, girdi koşulları): https://ffmpeg.org/ffmpeg-filters.html#xfade
2. gl-transitions deposu README (100+ geçiş, GLSL, spesifikasyon): https://raw.githubusercontent.com/gl-transitions/gl-transitions/master/README.md
3. gl-transitions geçiş dosyası listesi (GitHub API, bu çalışmada alındı; dosya adları): https://api.github.com/repos/gl-transitions/gl-transitions/contents/transitions
4. Easing Functions Cheat Sheet (30 eğri adı; formüller sayfada boş geldi): https://easings.net/
5. MDN, CSS easing-function (linear, cubic-bezier, steps, anahtar kelimeler): https://developer.mozilla.org/en-US/docs/Web/CSS/easing-function
6. Adobe, After Effects metin animasyon ön ayarları (Animate In / Animate Out): https://www.adobe.com/in/learn/after-effects/web/animate-text-presets
7. Adobe, Premiere Pro modern geçişler (Push, Split, Wave Impacts ve klasik geçişler): https://helpx.adobe.com/cz/premiere/desktop/add-video-effects/types-of-effects/transitions.html
8. Adobe, Premiere Elements geçişleri uygulama (Cross Dissolve, Dip to Black/White, Iris, Push, Split, Wipe, Page Peel): https://helpx.adobe.com/premiere-elements/desktop/applying-transitions/applying-transitions-clips.html
9. DaVinci Resolve geçişler (Dissolve, Iris, Motion, Shape, Wipe, Fusion kutuları): https://tryuncle.com/learn/davinci-resolve/transitions
10. DaVinci Resolve Club, geçişler: https://davinciresolveclub.com/davinci-resolve-transitions/
11. Canva metin animasyonları (Block, Typewriter, Ascend, Bounce, Burst, Roll, Shift, Skate, Breathe, Fade, Pan, Rise, Tumble, Drift, Stomp, Tectonic, Baseline, Pop, Neon, Scrapbook; üçüncü taraf anlatım): https://www.fontbundles.net/design-school/canva-tutorials/how-to-animate-text-in-canva
12. CapCut metin animasyonları (In/Out/Loop yapısı): https://www.capcut.com/resource/text-animation-online
13. Place Exchange DOOH Creative Best Practices (okunabilir yazı yüksekliği, kısa metin; PDF doğrudan açılamadı, arama özetinden alındı): https://www.placeexchange.com/assets/marketing/PX_Creative_Best_Practices.pdf
14. Rise Vision, dijital tabela tasarım kuralları (kontrast, tek hareketli öğe, durağan ana mesaj): https://www.risevision.com/blog/digital-signage-design
15. Mood Media, Digital Signage Best Practices (döngü süresi, bekleme süresi): https://us.moodmedia.com/wp-content/uploads/2023/03/Digital-Signage-Best-Practices.pdf
16. Vistar Media, açık hava reklam yaratıcı sorular (kısa mesaj, 3-7 sn maruz kalma): https://www.vistarmedia.com/blog/top-questions-design-ooh
17. Studiomeyer, kinetik tipografi (harf başına 30-50 ms akıcı, 100 ms ve üstü dramatik): https://studiomeyer.io/en/blog/kinetische-typografie.md
18. Cursa, okunabilirlik için metin animasyonu (kelime/satır sıralaması, harf düzeyinde sınır): https://cursa.app/en/page/text-animation-for-readability-kinetic-type-without-overcomplication
19. gl-transitions ana sayfa (içerik boş döndü, ayrıntı alınamadı): https://gl-transitions.com/
20. AdQuick DOOH yaratıcı belirtimleri (1080x1920 dikey ve 15 sn varsayılan; arama özetinden): https://www.adquick.com/answers/ooh-basics/digital-out-of-home-dooh-creative-specs-and-lead-times

(20 kaynak; birinci elden resmi olanlar: 1, 2, 3, 5, 7, 8. Satıcı/blog kaynakları olan 11, 12, 13-20 kural önerisi sayılır, ölçülmüş gerçek değil.)

### Doğrulanamayanlar ve uyarılar
- xfade `custom` ifadesi ile elle yazılabilecek geçişlerin çıktısı denenmedi; yalnız dokümandaki değişkenler (X, Y, W, H, P, A, B, a0..a3, b0..b3) alındı [1].
- xfade'in iç formülleri (vf_xfade.c) ve lisansı okunmadı; "formül buradan alınır" ifadesi yön gösterir, doğrulanmamıştır.
- gl-transitions lisansı ve tek tek geçiş kodları incelenmedi; 3. kaynak yalnız dosya adlarıdır. Adların bir kısmı GLSL'de gürültü, girdap, 3B içerir; LED'de görünümü denenmedi.
- Easing formülleri easings.net'te boş geldi; kodlama sırasında standart Penner denklemleri ayrıca kaynak gösterilerek alınmalı (doğrulanmadı).
- Canva ve CapCut animasyon adları üçüncü taraf anlatımlara dayanır; "Skate", "Stomp" gibi adların davranışı net bulunamadı, önerilerde yalnız ad ilhamı alındı, davranış kendi tasarımımızdır.
- After Effects'in yerleşik metin ön ayar klasör adları (Blurs, Fire, Paths vb.) doğrulanamadı; yalnız Animate In / Animate Out yapısı doğrulandı [6].
- LED "en çok 150 piksel/sn kayan yazı hızı", "8 piksel dilim", "3 piksel parçacık", "0.8 sn geçiş" gibi eşikler bu raporun önerisidir, ölçülmedi; gerçek LED ekranda denenip ayarlanmalı. DOOH'daki 3-7 sn, 2-3 sn gibi süreler kaynaklarda birbirinden farklıdır [13-16].
- Mevcut katalog sayıları dosyalardan elle sayıldı; derleme çıktısıyla karşılaştırılmadı.

## 7. Ayrı geçiş editörü: 1 sayfalık tasarım önerisi
**Amaç.** Kullanıcı "Detaylı bir geçiş video programı" istiyor. Bugün geçiş yalnız açılış/kapanış için seçiliyor; sahneler arası geçiş yok. İki seçenek: (a) ana programa "sahneler arası geçiş" alanı eklemek, (b) ayrı bir geçiş editörü. Tavsiye: önce (a), sonra (b) aynı kod üzerine.

**Veri modeli (iki seçenek için ortak).** Sahne listesine iki alan: `gecis_sonra` (katalog adı) ve `gecis_sure` (0.4-2.0 sn; varsayılan 1.2 sn, mevcut). Senaryo metin biçiminde `gecis <ad> <sn>` satırı. İsteğe bağlı eğri: `gecis_egri`. Süre sahne toplamına dahil edilir (örtüşmeli: çıkış ve giriş birlikte oynar, xfade mantığı [1]); örtüşmesiz seçenek "önce çıkış, sonra giriş" kalır.

**Ekran düzeni (editör, tek pencere, 3 bölge).**
1. **Zaman çizelgesi (alt, tam genişlik).** Her sahne bir blok (genişlik = süre). Bloklar arasında küçük bir "geçiş düğümü" (elmas); tıklayınca sağ panelde geçiş seçilir. Blok kenarı sürüklenerek süre değişir, düğümün iki yanı sürüklenerek geçiş süresi değişir. Zoom, kare/saniye cetveli, oynatma imleci.
2. **Önizleme (üst orta).** Seçili düğümün iki sahnesini kare kare oynatır; geçiş süresi üzerinde döngü. LED önizleme modu: gerçek LED piksel ızgarası ve düşük parlaklık simülasyonu (bantlaşma ve titreme uyarısı rozeti). Oynat/duraklat/kare ileri-geri; mevcut yerel sunucunun önizleme yolu yeniden kullanılır.
3. **Geçiş kütüphanesi (sağ panel).** Aile sekmeleri: Silme, Kaydırma, Ört/Aç, Şekil, Dilim, Rüzgar, Blur, Köşe, Özel (gl). Her kart 1 sn'lik küçük bir döngü; LED etiketi (yeşil: güvenli, sarı: dikkat, kırmızı: titreme riski); arama kutusu; "favori".

**Alt şerit.** Seçili geçişin ayarları: süre, eğri (egri listesi), yön, "tüm geçişlere uygula", "rastgele ata (güvenli aileler)". Tarif kutusu burada da var: "geçişler yumuşak sağa kaysın" -> 5. bölümdeki kurallar.

**Tek tıkla iş akışı.** Sahne listesinden gelir; geçiş yoksa "yok". "Tümünü aynı yap", "dönüşümlü yön" kısayolları. Geri al/yinele (en az 20 adım). Proje dosyası mevcut senaryo metin biçimi + `gecis` satırları; eski dosyalar aynen açılır (alan yoksa "yok").

**Teknik.** Çerçeve çizimi aynı motordan (`senaryo.rs` render yolu) gelir; geçişler `GECISLER` kataloğundan ve yeni `gecis_ciz(ad, kare_a, kare_b, p)` imzasıyla çağrılır; önizleme ve video aynı fonksiyonu kullanır (görüntü eşitliği). Önizlemede alt örnekleme (örn. 1/4 çözünürlük) CPU'yu azaltır. Önbellek: sahne A/B kareleri bir kez üretilir, geçiş her p için hesaplanır.

**Aşamalar.** (1) `gecis_sonra` alanı ve 10 xfade aile örneği; (2) önizleme penceresinde geçiş seçici (editörsüz); (3) zaman çizelgesi ve sürükleme; (4) gl seçkisi ve LED uyarı rozetleri; (5) tarif bağlama (`gecis` kategorisi sahneler arası).

**Riskler.** Önizleme ile video arasında görüntü farkı (aynı fonksiyon kullanılarak engellenir); süre örtüşmesinin toplam süre hesabını bozması (sahne süresi + geçiş mantığı belgelenmeli); kütüphane kartlarının her biri için küçük önizleme üretmek açılışı yavaşlatır (tembel yükleme ve önbellek).

*Not: Bu belge öneridir; karar Yusuf'undur. Kod ve katalog değiştirilmedi.*
