---
description: KARAR-ANIM — Vidyo 1.0 dalga 3 animasyon/efekt paketi kararı (kış-yılbaşı, 3B/2B yazı animasyonu, piksel art, premium ağır efektler, video→çizgifilm süzgeç zincirleri). Uygulayıcılar: Malzeme ajanı ve Dönüştürücü ajanı.
sahip: Yusuf
yazan: "[Claude#KARAR-ANIM Fable 5.2, yenileme-1.0, 2026-11-25]"
kural: Yalnız yazan yapay zekâ ya da Yusuf'un izniyle değişir; silme yok (güncelleme üstü çizilerek).
kaynak: docs/MALZEME.md, src/malzeme.rs (53 kalem), src/anim.rs, src/animasyon.rs, src/efekt.rs, src/dekor.rs, src/malzeme_ciz.rs, src/senaryo.rs imzaları; ORTAK.md; PLAN.md. Ekip dosyaları (tasarim/yenileme/ekip/*.txt) YOKTU; GPT/Gemini görüşü alınmadı, karar kendi bilgimle verildi.
---
# Animasyon / efekt paketi kararı

## Özet
- Bu tur için karara bağlanan üst sınır: 27 yeni malzeme, 6 yeni giriş/çıkış animasyonu, 15 yeni `efekt=` adı, dönüştürücüde 5 stil + 1 ek anahtar. Kesin çekirdek (Ö1): 14 malzeme, 4 giriş/çıkış, 9 efekt, 3 dönüştürücü stili + ek anahtar. Sıra 9. bölümdeki Ö1 → Ö2 → Ö3 önceliğidir. Ajan süresi 60–80 dk olduğu için **Ö1 bitmeden Ö2'ye geçilmez**.
- Senaryo dili ve katalog şeması DEĞİŞMEZ (`sure renk hiz deger yon metin efekt`). Çeşitler ayrı `id` olur; yeni parametre anahtarı eklenmez. Yalnız iki kategori eklenir: `kis` "Kış ve Yılbaşı", `sahne` "Premium Sahne".
- Her şey **kapalı biçimli** (t'nin doğrudan fonksiyonu) ve tohumla deterministik çizilir; kareler arası durum tutulmaz. Sebep: `senaryo::kare_uret(s, hz, t, no)` rastgele erişimlidir (önizleme, küçük resim, MP4 aynı yol).
- "Premium ağır" görünüm ışıma/neon/bulanıklıkla değil; **katmanlı derinlik, sert gölge, 3 tonlu gölgeleme, bantlı (dither'li) ton geçişi, doku ve yavaş kütleli hareket** ile kurulur.
- Dönüştürücü stilleri: `piksel`, `ikiton`, `cizgifilm` (Ö1), `yarim-ton`, `karakalem` (Ö2), ek anahtar `animasyon-12`.
- Dönüştürücü 1.0'da kare işlemeyi FFmpeg süzgeç grafiğiyle yapar (Rust kare borusu sonraya). Süzgeç metni yalnız beyaz listeli adlardan ve sıkıştırılmış sayılardan kurulur.

## 1. Ortak teknik kurallar (bütün maddeler için bağlayıcı)
| Konu | Karar |
|---|---|
| Kare bütçesi | 950x1800 tek kare ortalama ≤ 45 ms, en kötü ≤ 70 ms (release, bugünkü tavan 42 ms). Aşan malzeme parça sayısını düşürür; bütçe testi `malzeme_testleri` içine eklenir (yalnız uyarı değil, üst sınır 120 ms ile başarısızlık). |
| Zaman | `z = yerel * hiz`. Konumlar kapalı biçimli: `p(z) = p0 + v·(1−e^(−k·z))/k + ½·g·z²` (sürtünmeli atış), salınım `sin`, düşüş `mod`. Simülasyon adımı, kare sayacı, önceki kare belleği YOK. |
| Rastgelelik | Yalnız `efekt::hash(tohum ⊕ i·asal)`. Aynı senaryo → aynı bayt (mevcut deterministik test geçerli kalır). |
| Döngü | "Ortam" malzemeleri (kar, aurora, sis, toz, ışık huzmesi, ızgara zemin, kristal dönüşü) **döngü güvenlidir**: tüm fazlar `sure`'ye tam bölünen periyotlarla kurulur, t=0 ile t=sure aynı karedir (oynatıcıda dikişsiz döngü için). Birikme, geri sayım, açılış türleri döngüsüzdür; katalog `aciklama` sonuna "(döngüsüz)" yazılır. |
| Alan (field) efektleri | Sis, aurora, sıvı metal, kumaş: 1/4 çözünürlükte (238x450) hesaplanır, en yakın komşu + 4x4 Bayer ile tam kareye bantlı yazılır. Tam karede piksel başına `sqrt/sin` yasak. |
| Parça tavanı | Kare başına çizilen parça ≤ 2 500 (iz kopyaları dahil); yarıçapı < 1,5 px olan parça tek `px` ile çizilir. |
| Maske hızı | Harf ve 3B yazı işleri maskeyi satır aralığı (RLE: satır başına `[x0,x1)` listesi) olarak bir kez hazırlar (`Hazir` içinde); her karede tam bbox taranmaz. |
| Karışım | Normal alfa. Yalnız havai fişek kıvılcımı ve ışık dizisi ampulü "toplamalı + 255'e kırpma" kullanır; çevresine hale/ışıma çizilmez. |
| LED güvenliği | Tam ekran flaş yok; ekranın >%25'inde kareler arası parlaklık sıçraması ≤ %20; 3 Hz üstü yanıp sönme yok; kare ortalama parlaklığı ≤ %45 (koyu zemin esas). Mevcut "flaş yok" kuralının sayısal hâli. |
| Ton | Her yüzey 3 ton: `yuz`, `yuz×0,62` (gölge), `yuz→beyaz %35` (ışık). Işık yönü sabit: sol üst. Gölge: sert, 45° sağ alt, bulanıksız. |
| Yazı güvenli alanı | Metin alan malzemelerde yazı kutusu ekranın %12–%88 yatay, kullanıcı metni `piksel_sigdir`/3B maske ile sığdırılır; Türkçe büyük harf `malzeme_ciz::buyuk`. |
| Etiket (yapay zekâ eşleşmesi) | Her yeni kalemde ≥ 6 Türkçe etiket, eş anlamlılar dahil (ör. `yilbasi,yeni yil,noel,kis,aralik,kutlama`). "İstemler alakasız" şikâyetinin malzeme tarafındaki çaresi budur; AI ajanı kataloğu `GET /malzeme/liste` üzerinden okur, ayrı sözlük tutmaz. |
| Yeni dosyalar | `src/malzeme_kis.rs` (kış çizimleri), `src/malzeme_parca.rs` (parça/fişek motoru), `src/anim_harf.rs` (harf maskeleri + harf animasyonu), `src/malzeme_sahne.rs` (premium sahneler), `src/malzeme_alan.rs` (1/4 çözünürlük alan yardımcıları). Hepsi Malzeme ajanının sahipliğinde. `malzeme_ciz::ciz` yalnız yönlendirme satırı alır. |

## 2. (a) Kış / yılbaşı paketi
Kategori `kis`. `deger` ve `yon` anlamı her satırda yazılı; diğer parametreler ortak (`sure renk hiz metin`).

| # | id | Ad | Nasıl çizilir (950x1800 yazılım raster) | Parametre | Tur | Zorluk |
|---|---|---|---|---|---|---|
| K1 | `kar-katmanli` | Katmanlı Kar | 3 derinlik katmanı: arka (r 1,5–3 px, hız 60 px/sn, alfa 0,45, tek `px`/küçük daire), orta (r 4–7, 130 px/sn, 0,75), ön (r 10–18, 240 px/sn, altı kollu tane: 3 `cizgi` + uç çatalları, yavaş döner). `y = (y0 + v·z) mod (h+pay)`; `x = x0 + A·sin(ω·z+φ) + R(z)·derinlik`. Rüzgâr `R(z) = yönişareti·(40 + 30·sin(2π·z/sure) + 15·sin(2π·3z/sure))` px: esinti kapalı biçimli ve döngü güvenli. | `deger` tane sayısı 40–400 (varsayılan 160, katmanlara 50/35/15 %); `yon` sag/sol rüzgâr | Ö1 | kolay |
| K2 | `kar-birikme` | Biriken Kar | K1 + alt kenarda kar örtüsü: `yük(x,z) = min(H, H·z/(0,8·sure))·(0,7 + 0,3·gürültü1B(x))`, üst kenar 2 oktav 1B değer gürültüsüyle tümsekli; örtü 3 ton (üst ışık bandı 6 px). Metin varsa 3B yazı örtünün üstünde durur. Döngüsüz. | `deger` en çok yükseklik %5–30 (varsayılan 14) | Ö1 | kolay |
| K3 | `cam-agaci` | Çam Ağacı | 5 kat üst üste üçgen (`cokgen`), her kat alt kenarı testere dişli; sol yarı ışık, sağ yarı gölge tonu (iki çokgen). Gövde `dikd`. Süsler: katların kenarında `daire` + sol üstte küçük ışık noktası (parlaklık değil, ton). Işık dizisi: katlar arasında sarkan yay boyunca ampuller; ampul parlaklığı `0,55 + 0,45·yumusak(üçgen dalga(z·0,8 − i/n))` (koşan, ≤ 1 Hz). Tepe yıldızı `yildiz` 5 köşe, ±8° salınır. Dipte kar tümseği + 2 hediye. Giriş: katlar alttan üste 0,25 sn arayla `geri` eğrisiyle oturur. Üstte isteğe bağlı piksel/3B yazı. | `deger` süs sayısı 6–40 (varsayılan 18); `metin` | Ö1 | orta |
| K4 | `fisek-kabarcik` | Küre Fişek | Ortak fişek motoru (3. bölüm), tür=küre: 90–140 kıvılcım, hızlar küre yüzeyinden eşit dağılımlı, iz 4 kopya. | `deger` roket 1–10 | Ö1 | orta (motor) |
| K5 | `fisek-sogut` | Söğüt Fişek | tür=söğüt: 60 kıvılcım, düşük ilk hız, yüksek yerçekimi, ömür 2,8 sn, iz 14 kopya (uzun sarkan kollar), renk altın→koyu turuncu. | aynı | Ö1 | kolay (motor varsa) |
| K6 | `fisek-halka` | Halka Fişek | tür=halka: 48 kıvılcım tek düzlemde çember; düzlem eğimi tohumdan (elips görünür), içte ikinci küçük halka. | aynı | Ö1 | kolay |
| K7 | `fisek-kalp` | Kalp Fişek | tür=kalp: hız vektörleri kalp eğrisinden (`x=16sin³u, y=13cosu−5cos2u−2cos3u−cos4u`) 56 örnek; düşük yerçekimi ki şekil 1 sn okunur. | aynı | Ö1 | kolay |
| K8 | `fisek-cift` | Çift Patlama | tür=çift: ilk küre (40 kıvılcım), 0,55 sn sonra her 5. kıvılcımın konumunda ikinci küçük patlama (12'şer kıvılcım, ikinci renk). | aynı | Ö1 | orta |
| K9 | `yilbasi-senlik` | Yılbaşı Şenliği | Karma gösteri: altta şehir silueti (tohumdan 14–20 `dikd` bina, sarı pencere kareleri, %6'sı 4 sn'de bir yanıp söner), üstte 5 türün sıralı karışımı (tür = hash), ortada 3B yazı. Sondaki 2 sn "final": art arda 6 roket. | `deger` roket 3–14 (varsayılan 8); `metin` "MUTLU YILLAR" | Ö1 | orta |
| K10 | `yilbasi-geri-sayim` | Yılbaşı Geri Sayımı | Mevcut `geri-sayim` düzeni + her rakam değişiminde rakam üstten düşüp `geri` eğrisiyle oturur, eski rakam alta kayarak söner; çevrede saniye halkası; arkada K1 arka katman. Sıfırda rakam yerini `metin` 3B yazıya bırakır ve 2 küre fişek + konfeti başlar. Süre = `deger` + 4 sn kutlama payı (üretici `sure`'yi buna yükseltir). Döngüsüz. | `deger` başlangıç 3–60 (varsayılan 10); `metin` | Ö1 | orta |
| K11 | `kardan-adam` | Kardan Adam | 3 `daire` (alt/orta/baş) + sağ altta gölge tonu hilali (aynı dairenin kaydırılmış koyu kopyası kırpılarak), kömür göz/düğme, havuç burun `cokgen`, silindir şapka, atkı (2 `yuvarlak` + ucu `sin` ile dalgalanan şerit), dal kol; sağ kol 1,2 sn'de bir el sallar (±18°). Zemin kar tümseği, arkada K1 arka+orta katman. | `metin` (üstte), `yon` bakış | Ö1 | kolay |
| K12 | `buz-kristali` | Buz Kristali | 6 kat simetri: bir kol = ana `cizgi` + 3 çift yan dal (dal boyu kola göre 0,45/0,35/0,22, 60°) + uçlarda küçük altıgen; büyüme `k = yumusak(z/2)` ile kollar uzar, sonra tüm kristal 12°/sn döner. 1 büyük + 4 küçük kristal (farklı faz). Çizgiler 2 ton (açık çekirdek 3 px, koyu kenar 7 px) → derinlik. | `deger` kristal sayısı 1–9 | Ö1 | kolay |
| K13 | `kis-manzarasi` | Kış Manzarası | 4 paralaks katman (arkadan öne): gökyüzü 5 bantlı dither'li ton + ay `daire` (kraterli); uzak dağlar (1B gürültü profili, kar tepeli); çam siluetleri sırası (K3 üçgenleri tek ton, 9–14 adet); ön tepe + kulübe (pencere sıcak sarı, baca dumanı: yükselip büyüyen 5 `daire`, alfa azalan). Katmanlar `yon`'a doğru 4/10/22/40 px/sn kayar (sarmalı, döngü güvenli). Üstüne K1. | `yon`, `metin` | Ö2 | orta |
| K14 | `hediye-yigini` | Hediye Yığını | 3–7 kutu sırayla üstten düşer (`sek` eğrisi), kutular `dikd` + kapak + çapraz kurdele + fiyonk (2 `elips` + `daire`); her kutu 3 ton (ön yüz, sağ yan koyu paralelkenar, üst açık paralelkenar → sahte 3B). Mevcut `hediye-kutusu` aynen kalır. | `deger` kutu 3–7; `metin` | Ö2 | kolay |
| K15 | `aurora` | Kuzey Işıkları | `malzeme_alan`: 3 dikey perde; perde parlaklığı `B(x,y) = bant(x + 60·sin(y/170 + z·0,5) + 30·sin(y/61 − z·0,8))·düşüş(y)`; 6 seviyeye nicemlenir, Bayer ile yazılır; altta çam silueti. | `renk`, `metin` | Ö2 | orta |
| K16 | `kis-suslu-yazi` | Karlı Yazı | 3B yazı + harflerin üstüne kar örtüsü: her x sütunu için maskenin en üst pikseli `ü(x)` bulunur, 9 px kayan ortalama ile yumuşatılır, üstüne `kalınlık(x,z)` yüksekliğinde beyaz şerit (üst kenar tümsekli), kenarlardan 2–3 buz sarkıtı (ince üçgen). | `metin`, `deger` kalınlık | Ö2 | orta |
| K17 | `yilbasi-susleri` | Sallanan Süsler | Üst kenardan iple sarkan 5–9 süs topu (sarkaç: `θ = θ0·sin(ω·z+φ)`), top üstünde bant deseni. | `deger` | sonraya | kolay |
| K18 | `ren-geyigi-kizak` | Kızak Geçişi | Ay önünden geçen kızak silueti (sprite/çokgen). | `yon` | sonraya | orta |
| K19 | `buzlu-cam` | Buzlu Cam Yazı | Kenarlardan içe büyüyen kırağı (dallanan büyüme) + silinerek açılan yazı. | — | sonraya | zor |
| K20 | `somine` | Şömine | Alev alan efekti + taş çerçeve. | — | sonraya | zor |

Yeni `efekt=` adları (her sahnenin üstüne eklenebilir; `dekor::ADLAR`'a eklenir): `kar-katmanli` (K1'in metinsiz hâli, Ö1), `kar-ortusu` (alt kenar birikmesi, Ö1), `isik-dizisi` (üst kenarda sarkan koşan ampuller, Ö1), `buz-koseler` (dört köşede kristal dalları, Ö2). Mevcut `kar` efekti ve `kar-yagisi`, `havai-fisek-gosteri` malzemeleri geriye uyum için aynen kalır.

## 3. Ortak parça / havai fişek motoru (`src/malzeme_parca.rs`)
Tek motor, beş tür; K4–K10 hepsi bunu çağırır.
- Roket `j`: fırlatma zamanı `tj = (j + hash)·(gösteri süresi/roket sayısı)`, yükseliş 0,9–1,3 sn, tepe noktası ekranın x %15–85, y %12–42 bandında (alt %40 yazıya kalır). Yükseliş izi: 10 px'lik kuyruk `cizgi` + arkada 6 sönük nokta. Tepede patlama; tam ekran flaş YOK, yalnız patlama merkezinde 0,12 sn'lik r=22 px açık ton daire.
- Kıvılcım `i`: tür tablosundan birim hız `u_i`; `p = merkez + v0·u_i·(1−e^(−k·τ))/k + (0, ½·g·τ²)`, `τ = z − tpatlama`. Ömür `L`; alfa `1 − (τ/L)²`; son %30'da kıvılcımların tohumdan seçilen yarısı 8 Hz yerine **2,5 Hz** göz kırpar (LED güvenliği).
- İz: aynı formül `τ − n·0,035` için `n=1..N` kopya, alfa ve yarıçap doğrusal azalır (bellek yok).
- Tür tablosu: `(adet, v0 px/sn, k, g px/sn², L sn, iz N, yarıçap)` → küre (120, 520, 2,2, 160, 1,7, 4, 3,5) · söğüt (60, 300, 1,1, 260, 2,8, 14, 3) · halka (48, 460, 2,4, 120, 1,5, 3, 4) · kalp (56, 26/birim, 2,6, 60, 1,8, 3, 4) · çift (40 + 8×12, 480, 2,2, 150, 1,2 + 0,9, 3, 3,5).
- Renk: palet `yuz` ve `vurgu` arası; kıvılcım çekirdeği açık ton, iz ana ton, son %25 koyu ton (3 ton kuralı). Toplamalı karışım + kırpma.
- Bütçe: aynı anda en çok 3 patlama görünür olacak biçimde `tj` aralığı `≥ L/3`; kare başına ≤ 2 500 parça.
- Test: her tür için t=patlama+0,4 sn karesinde boş olmayan piksel sayısı alt sınırı; kalp türünde yatay simetri farkı < %8; determinizm; bütçe.

## 4. (b) 3B / 2B yazı animasyonları
Altyapı kararı: **harf maskeleri yerleşim anında** üretilir (sütun boşluğundan bölme YOK: "AV", "LT" gibi çiftlerde ve Türkçe noktalı harflerde yanlış böler). `anim_harf::harf_maskeleri(font, w, h, satirlar, harf) -> Vec<HarfM>`; `HarfM { satir, sira, bx, rle }`. `Hazir`'a `harfler: Vec<HarfM>` eklenir, yalnız harf animasyonu/efekti isteyen sahnede doldurulur. Harf pozu mevcut `Poz` yapısıdır; her harfe `gecikme = sira·adım` uygulanır ve çizim `kare3d_kirpmali` eşdeğeri RLE yoluyla harf bbox'ında yapılır.

Sunum kararı: harf hareketleri **giriş/çıkış adı** (`giris=`/`cikis=`; `animasyon::YERLESIK`'e harf-bayraklı tanım), sürekli görünüm katmanları **`efekt=` adı**, tam sahne isteyenler **malzeme** (kategori `yazi`). `senaryo::giris_adlari()` genişlediği için AI tarifi yeni adları kendiliğinden görür; AI ajanı sabit liste tutmamalı.

| # | Ad (tür) | Nasıl | Tur | Zorluk |
|---|---|---|---|---|
| Y1 | `harf-kaskad` (giriş/çıkış) | Harfler soldan sağa 0,06 sn arayla alttan 120 px + 0,6 ölçekten `geri` eğrisiyle oturur; çıkışta ters sırayla yukarı. | Ö1 | orta (altyapı) |
| Y2 | `harf-dalga` (giriş + bekleme) | Oturduktan sonra `dy = 18·sin(2π(z·0,8 − sira/8))`, `aci = ±4°`; sürekli, döngü güvenli. | Ö1 | kolay |
| Y3 | `harf-oynak` (giriş + bekleme) | Kinetik oynaklık: her harf tohumdan fazla 0,9–1,4 sn'de bir küçük zıplar (`ziplama`, 22 px) ve ±6° yalpalar; aynı anda en çok harflerin 1/3'ü hareketli. | Ö1 | kolay |
| Y4 | `harf-dus` (giriş) | Harfler üstten rastgele sırayla düşer, `sek` ile iki kez seker; oturunca 6 px'lik sönümlü sahne sarsıntısı (yalnız yazı katmanı). | Ö1 | kolay |
| Y5 | `uzun-golge` (efekt) | Maskenin 45° sağ alta 1 px adımlı `derinlik×3` (en çok 90) kopyası tek koyu tonla; RLE ile satır kaydırma. Bulanıksız, kesik uçlu. | Ö1 | kolay |
| Y6 | `kabartma` (efekt) | Yüzün içinde: maske (−2,−2) kaydırılmış fark → ışık tonu kenar, (+2,+2) fark → gölge tonu kenar; 3 px. Metal/oyma hissi. | Ö1 | kolay |
| Y7 | `supurme-agir` (efekt) | Mevcut `isik_supurme` yalnız yüz maskesine kırpılır: 20° eğik, biri geniş (90 px, +%28 ton) biri ince (18 px, +%45) iki bant, 3,5 sn'de bir geçer; ton artışı, toplamalı değil → parlama/hale yok. | Ö1 | kolay |
| Y8 | `don-3b` (giriş) + `yazi-3b-don` (malzeme) | Dikey eksende ekstrüde dönüş: sütun eşlemesi `x' = cx + (x−cx)·cosθ / (1 + (x−cx)·sinθ/f)`, sütun dikey ölçeği `1/(1 + (x−cx)·sinθ/f)`, `f = 2,2·w`. Kalınlık: `derinlik` kadar (≤ 24) dilim arkadan öne `sinθ` yönünde 1 px kaydırılarak yan tonla, en üstte yüz; yan ton `0,45 + 0,35·|sinθ|` (Lambert, 4 seviye). |cosθ| < 0,08 iken yalnız yan şerit çizilir. Giriş: 540°→0° yavaşlayarak; malzeme: 8 sn'de bir tam tur, önde 2 sn bekler. | Ö2 | zor |
| Y9 | `derin-yaklas` (giriş) | Z'den gelme: ölçek 0,25→1 (`ustel`), gölge mesafesi 60→derinlik px, 3 kademeli hayalet kopya (alfa 0,25/0,15/0,08) geride kalır. | Ö2 | kolay |
| Y10 | `yansima` (efekt) | Yazının altına dikey çevrilmiş kopya: yükseklik %55'e sıkışık, satır alfa `0,42→0`, her 3 satırda 1 satır boş (tarama dokusu), satır x kayması `3·sin(y/9 + z·2)` (su). Araya 4 px zemin çizgisi. | Ö2 | orta |
| Y11 | `kinetik-yazi` (malzeme) | Kelime kelime kinetik tipografi; kelimeler `|` ya da boşlukla ayrılır, her kelime 0,7–1,1 sn ekranda. 4 yerleşim şablonu sırayla: tam genişlik vuruş (ölçek 1,6→1 + sarsıntı), 90° dönük sol kenardan kayma, iki satır zıt yönden çarpışma, harf-kaskad. Kelime değişiminde zemin/yazı tonları yer değiştirir (kesme, flaş değil: alan değişimi < %20 kuralı için renk yerine şerit silme geçişi). | Ö2 | orta |
| Y12 | `doku-metal` / `doku-ahsap` / `doku-tarama` (efekt) | Yüz içinde doku: metal = yatay fırça çizgileri (satır başına hash ton ±%10) + üstte ışık bandı; ahşap = 1B gürültüyle bükülen damar çizgileri; tarama = 45° 6 px aralıklı koyu çizgi. | Ö2 | kolay |
| Y13 | `dikis-yazi` (malzeme) | Basit sürüm: kumaş zemin (çapraz tarama dokusu), yazı yüzü düz ton, içte 8 px'te **iç kontur bandı** (maske ile 8 px ve 11 px aşındırılmış maskelerin farkı) "9 dolu / 6 boş" kesik desenle; desen soldan sağa `z` ile açılır (dikiliyor). İğne ucu küçük `cizgi`. | Ö3 | orta |
| Y14 | Gerçek kontur izlemeli dikiş / ip (marching squares → çoklu çizgi, ip sarkması) | Kontur boyunca sıralı dikiş, sarkan ip. | sonraya | zor |
| Y15 | `harf-cevir` (uçuş panosu / split-flap) | Yarım harf menteşe dönüşü. | sonraya | orta |
| Y16 | Yol üstünde yazı (dairesel / dalga yolu), sıvı-morf, el yazısı çizimi | — | sonraya | zor |

Yasak teyidi: yazıda neon tüp, dış ışıma, bulanık gölge, tam ekran parlama yok. Mevcut `parlama` efekti ve `Gecis::Neon` geriye uyum için kalır; yeni işlerde önerilmez, AI tarifinde varsayılan olmaz (AI ajanına not: 8. bölüm).

## 5. (c) Piksel art
| # | Madde | Karar | Tur | Zorluk |
|---|---|---|---|---|
| P1 | Video → piksel | Dönüştürücü, FFmpeg: `scale=W/h:H/h:flags=area` (hücre `h` = 6–32 px, varsayılan 10 → 95x180) → palet → `scale=950:1800:flags=neighbor`. | Ö1 | kolay |
| P2 | Palet kısıtlama | (i) **Uyarlamalı**: `palettegen=max_colors=N:stats_mode=full` ile video başına TEK palet (kare başına palet titreme yapar, yasak); N = 4/8/16/32. (ii) **Hazır**: Rust 16x1 PNG üretir (`png_sikistir` var), `paletteuse` girdi olur. Hazır paletler (marka adı yok): `oyun-16`, `cep-4` (dört yeşil ton), `ev-16`, `led-8` (doygun 8 renk), `tek-renk-2` (zemin+tek renk), `kis-8` (buz mavileri+kırmızı). | Ö1 (uyarlamalı + `led-8`, `cep-4`), Ö2 (diğerleri) | kolay |
| P3 | Dithering | Varsayılan **sıralı Bayer** (`paletteuse=dither=bayer:bayer_scale=3:diff_mode=rectangle`): zamansal olarak kararlı. Floyd–Steinberg videoda kaynaşır (karınca yürümesi), yalnız tek resim girdisinde seçenek. "Dither yok" seçeneği de var. | Ö1 | kolay |
| P4 | LED hücre ızgarası | İsteğe bağlı: büyütmeden sonra `drawgrid=w=h:h=h:t=1:c=black@0.55` (hücre arası koyu çizgi → tabela modülü görünümü). | Ö2 | kolay |
| P5 | Sprite malzemeleri | Mevcut `Tuval::sprite` ile `piksel-kis` (kardan adam, çam, hediye, kar tanesi; 16x16, 2–3 kare, 6 hücre/px yürüyen şerit) ve `piksel-fisek` (ızgaraya oturtulmuş 8 yönlü patlama). | Ö2 | kolay |
| P6 | Zamansal kararlılık (önceki dizin histerezisi), kenar çizgili piksel, piksel yazı fontu ile altyazı bindirme | Rust kare borusu gerektirir. | sonraya | orta |

## 6. (d) Premium ağır efektler
Kategori `sahne` (tam sahne) ve `efekt=` adları. Hepsi 1. bölüm kurallarına uyar (bantlı ton, sert gölge, döngü güvenli).

| # | id (tür) | Nasıl | Tur | Zorluk |
|---|---|---|---|---|
| A1 | `altin-toz` (efekt + malzeme) | 3 derinlikte yavaş yükselen sert kenarlı diskler (r 2/5/11 px; alfa 0,8/0,5/0,28), ön katman en yavaş değil en hızlı (paralaks); %10'u 4 köşeli pırıltı `yildiz`. | Ö1 | kolay |
| A2 | `vinyet` (efekt) | Kenarlardan içe 4 basamaklı koyulaşma (%0/12/24/38), basamak sınırları Bayer ile; her sahneye derinlik katar, en ucuz "ağırlık". Köşe mesafesi tablosu bir kez hesaplanır. | Ö1 | kolay |
| A3 | `isik-huzmesi` (efekt + malzeme) | Üst merkezden yayılan 7–9 kama `cokgen`, alfa 0,10–0,18, ±6° 9 sn periyotla salınır; kamalar üst üste binince doğal bantlanma. | Ö1 | kolay |
| A4 | `izgara-zemin` (malzeme) | Perspektif ızgara: ufuk y=%45; yatay çizgiler `y = ufuk + H/(n − kayma)` ile öne akar, dikeyler kaçış noktasından yelpaze; çizgi kalınlığı derinlikle 1→5 px; üstte 3B yazı + gökyüzünde bantlı ton. | Ö2 | kolay |
| A5 | `sis` (efekt) | `malzeme_alan`: 2 oktav değer gürültüsü, iki katman zıt yöne kayar, 4 seviye + Bayer, en çok alfa 0,35. | Ö2 | orta |
| A6 | `sivi-metal` (malzeme) | Alan: `f = sin(x/90+z) + sin(y/70−0,7z) + sin((x+y)/120+1,3z)`; 7 basamaklı palet rampasına nicemlenir (koyu→yüz→açık→yüz…), Bayer; üstünde `kabartma`'lı 3B yazı. Altın/gümüş/bakır palet. | Ö2 | kolay |
| A7 | `kumas-dalga` (malzeme) | Bayrak/kadife: satır kayması `dx = A·sin(y/λ − z)`, ton `yuz·(0,7 + 0,3·cos(y/λ − z))` 5 seviye; isteğe bağlı çizgili desen. Mevcut `logo-perde` ile aynı ton dili. | Ö3 | orta |
| A8 | `ucboyut-kutu` (malzeme) | Gerçek 3B düz gölgeli nesne: küçük yardımcı (`nokta3 → izdüşüm`, yüz normali Lambert 4 seviye, ressam sıralaması) ile dönen küp/hediye/prizma yıldız. 12–40 çokgen. | Ö3 | orta |
| A9 | `kamera-sarsinti` (iç yardımcı) | Vuruş anlarında yalnız ilgili katmana 6 px sönümlü kayma (Y4, Y11 kullanır). Tam kare sarsıntısı yok. | Ö1 | kolay |
| A10 | Mercek/cam kırılması, gerçek hacimsel ışık, akışkan/duman simülasyonu, parçacıkla yazı oluşumu (10 000+ parça) | Yazılım rasterde bütçeyi aşar ya da durum bellek ister. | sonraya | zor |
| A11 | Film greni / gürültü dokusu | **Yapılmaz**: LED'de kaynaşma ve bant sorunu yaratır. | — | — |

## 7. (e) Video → çizgifilm / animasyon süzgeç zincirleri (Dönüştürücü)
Ortak karar:
- Çözme, süzme, kodlama tek FFmpeg çağrısı; ses `-c:a copy` (yoksa atlanır); çıktı her zaman 950x1800 (ya da seçili ekran): `scale=…:force_original_aspect_ratio=increase,crop=950:1800` (doldur) ya da `decrease,pad` (sığdır) seçeneği.
- Süzgeç grafiği **yalnız** enum + `clamp` edilmiş sayılardan `format!` ile kurulur; kullanıcı metni grafiğe girmez; dosya yolları ayrı argüman, kabuk yok, giriş yolu izinli klasörlere kanonik yol denetimiyle sınırlı.
- Başlangıçta `ffmpeg -hide_banner -filters` bir kez okunur; eksik süzgeç varsa o stil arayüzde kapalı gösterilir, sessizce başka süzgece düşülmez. **Doğrulanmadı:** aşağıdaki süzgeçlerin projedeki FFmpeg sürümünde bulunduğu bu kararda sınanmadı; Dönüştürücü ajanı ilk iş bunu sınar.
- Her stilde `guc` (0–100) tek kaydırıcı + stil başına en çok 2 ek ayar. Önizleme: tek kare (`-ss t -frames:v 1`) aynı grafikle.

| # | Stil id | Zincir (özet) | Ayarlar | Tur | Zorluk |
|---|---|---|---|---|---|
| D1 | `cizgifilm` (kenar + posterize) | `split[a][b]`; `[a]` yumuşatma `hqdn3d=6:4:9:9` (varsa `bilateral`) → `eq=saturation=1.35:contrast=1.08` → posterize `lutrgb` ile kanal başına N seviye (`floor(val/s)*s + s/2`, N=4–8) `[renk]`; `[b]` `format=gray,edgedetect=low=0.08:high=0.22:mode=wires,negate` → kalınlaştırma `erosion` (negatifte çizgi kalınlaşır) `[cizgi]`; `[renk][cizgi]blend=all_mode=multiply`. | seviye N, çizgi kalınlığı 0–2 | Ö1 | orta |
| D2 | `piksel` | 5. bölüm P1–P3. | hücre, renk sayısı/palet, dither | Ö1 | kolay |
| D3 | `ikiton` (duotone) | `format=gray,eq=contrast=1.25,format=rgb24,lutrgb=r='Ar+(Br−Ar)*val/255':g=…:b=…` (A koyu, B açık renk; program paletlerinden `zemin`/`yuz`). Seçenek: 3 basamaklı sert eşik (posterize 3) → afiş görünümü. | palet, sert/yumuşak | Ö1 | kolay |
| D4 | `yarim-ton` (halftone) | Rust, 45° döndürülmüş nokta ekranı **eşik deseni** PNG'si üretir (hücre 8–24 px, döşenebilir). FFmpeg: `[video]format=gray[g]; [desen]tile/loop ile kareye döşe[d]; [g][d]blend=all_expr='if(gt(A,B),255,0)'` → ardından D3 `lutrgb` ile iki renge boyama. | nokta boyu, palet | Ö2 | orta |
| D5 | `animasyon-12` | Herhangi bir stile ek anahtar: `fps=12` → çıkışta 25 fps'e kare yineleme ("ikişerli" çizgi film hissi) + posterize. Tek başına stil değil, onay kutusu. | aç/kapa | Ö1 | kolay |
| D6 | `karakalem` | `edgedetect` negatifi + kâğıt tonu `lutrgb`; D1'in renk kolu olmadan. | kalınlık | Ö2 | kolay |
| D7 | Rotoskop / zamansal kararlı boyama, yağlı boya (kuwahara), yapay zekâ stil aktarımı | Rust kare borusu ya da dış model gerekir; 1.0 dışı. | — | sonraya | zor |

Test (Dönüştürücü): her stil için 1 sn'lik üretilmiş sentetik video (`testsrc2`) → çıktı 950x1800, kare sayısı korunur, `piksel`'de benzersiz renk ≤ N, `ikiton`'da her piksel A–B doğrusu üstünde (tolerans 3), süzgeç metni üreticisinde enjeksiyon denemesi (`;`, `[`, `'` içeren ayar) reddedilir.

## 8. Diğer ajanlara sözleşme notları
- **AI (tarif):** giriş/çıkış/efekt adlarını `senaryo::giris_adlari/cikis_adlari/efekt_listesi` ve `GET /malzeme/liste` üzerinden alır; sabit liste tutmaz. Kış/yılbaşı istemlerinde `kis` kategorisi önce denenir. `parlama` ve neon geçişi varsayılan seçilmez.
- **UI:** yeni kategoriler katalog JSON'undan kendiliğinden gelir; arayüzde değişiklik gerekmez. Kategori sırası: `kis` listenin başına (mevsim), `sahne` sona.
- **Oynatıcı:** döngü güvenli malzemeler `aciklama`'da "(döngüsüz)" yazmayanlardır; canlı sahne kaynağı olarak bunlar önerilir.
- **Belgeler/testler:** `docs/MALZEME.md` özetindeki adet (53/10) ve katalog testleri Malzeme ajanınca güncellenir.

## 9. Öncelik ve uygulama sırası
**Malzeme ajanı**
- Ö1 (bu tur, kesin): `malzeme_parca` motoru → K4–K9 · K1, K2 · K3 · K10 · K11 · K12 · efektler `kar-katmanli`, `kar-ortusu`, `isik-dizisi` · `anim_harf` altyapısı → Y1–Y4 · Y5, Y6, Y7 · A1, A2, A3, A9. (14 malzeme, 4 giriş/çıkış, 9 efekt)
- Ö2 (süre kalırsa, bu sırayla): K13 · K16 · K14 · Y10 · Y9 · Y8 (`don-3b`, `yazi-3b-don`) · Y11 · Y12 · K15 · A4 · A6 · A5 · P5 · `buz-koseler`.
- Ö3 (en son): Y13 · A7 · A8.
- Sonraya (1.0 dışı): K17–K20, Y14–Y16, A10, P6.

**Dönüştürücü ajanı**
- Ö1: süzgeç bulunurluk sınaması → D2 (`piksel`, uyarlamalı palet + `led-8`, `cep-4`, Bayer) → D3 → D1 → D5 (D5 stil değil, ek anahtar; stiller: piksel, ikiton, cizgifilm + Ö2'de yarim-ton, karakalem).
- Ö2: D4, D6, P4, kalan hazır paletler.
- Sonraya: D7, P6.

Kabul ölçütü (her kalem): katalogda ≥ 6 etiket; 950x1800 render boş/tek renk değil; deterministik; bütçe içinde; flaş kuralı (ardışık iki karede ortalama parlaklık farkı ≤ %20) otomatik testle; küçük resim (`temsil_t`) anlamlı bir anı gösterir (fişekte patlama anı, geri sayımda rakam).

## 10. Riskler ve açık noktalar
1. Ekip (GPT/Gemini) görüşü yok: Yusuf "animasyonlar için ekip" istemişti; bu karar tek aileden çıktı. Uygulamadan sonra farklı aileden inceleme önerilir (karar Yusuf'un).
2. `kare3d_kirpmali`'nin bbox mu tam kare mi taradığı bu kararda okunmadı; harf başına çağrı tam kare tarıyorsa Y1–Y4 bütçeyi aşar → RLE yolu zorunlu (1. bölüm).
3. Y8 perspektif dönüşü en riskli kalem; yetişmezse `don-3b` yalnız yatay sıkıştırma (perspektifsiz) + dilimli kalınlıkla çıkar, perspektif sonraya.
4. FFmpeg süzgeç adları/sözdizimi sürüme bağlı (özellikle `bilateral`, `erosion`, `blend` ifadesi hızı); D4 950x1800'de yavaş kalırsa Rust kare borusuna taşınır.
5. Ö1 kapsamı 60–80 dk için sıkı; kesinti gerekirse sırayla düşürülecekler: K12, K11, Y4, A3.
6. Katalog 53 → Ö1 ile 67, hepsi yapılırsa 80 kaleme çıkar; galeri küçük resim ilk yüklemesi uzar (tembel yükleme var, sorun beklenmez; ölçülmedi).

## 11. Ek 1: ikinci karar geçişi (2026-11-25 22:57; yazan [Claude#KARAR-ANIM Fable 5.2, yenileme-1.0])
Bu dosya ilk geçişte (22:34) yazılmıştı; uygulayıcı ajanlar ona göre çalışırken görev yeniden verildi. Dosyanın üstüne yazılmadı; 1–10. bölümler **geçerli karar olarak kalır**. Bu ek yalnız (i) açık kalan bir riski kapatır, (ii) çalışma ağacında görülen sapmaları karara bağlar. Kaynak: `src/anim.rs` 100–138, `src/malzeme.rs` KATALOG (22:57 hâli, commit edilmemiş), `src/dekor.rs` 422–423, `docs/DONUSTURUCU.md`. **Derleme/test çalıştırılmadı; aşağıdaki "var/yok" tespitleri yalnız kaynak metni aramasıdır, çalıştığının kanıtı değildir.**

### 11.1 Risk 2 kapandı (`kare3d_kirpmali`)
Okundu: işlev tam kareyi değil, dönüştürülmüş bbox + pay alanını tarar; ama bunu `derinlik + 1` kez (her kalınlık dilimi için bir geçiş) ve piksel başına `ornek` çağrısıyla yapar. Sonuç:
- Harf başına çağrı **zaman** açısından kabul edilir (harf bbox'larının toplamı ≈ satır bbox'ı; ek maliyet yok).
- Harf başına ayrı `Maske` **bellek** açısından yasak: `Maske` tam kare `Vec<f32>` taşır (950x1800 ≈ 6,8 MB); 20 harf ≈ 137 MB. Harfler ya kırpılmış (bbox boyutlu) maske/RLE ile, ya da tek ortak maske + sütun aralığı (`kirp` benzeri) ile çizilir.
- Maliyet `bbox alanı × (derinlik + 1)` olduğundan harf animasyonlarında `derinlik ≤ 16`, Y8 dönüşünde ≤ 24 (4. bölümdeki sınır) tutulur.

### 11.2 Görülen sapmalar ve karar
| Konu | Görülen | Karar |
|---|---|---|
| Y1–Y4 sunumu | `harf-kaskad/dalga/oynak/dus` **malzeme** (kategori `yazi`) olarak eklenmiş; `anim_harf.rs` yok, `giris=`/`cikis=` adı olarak yok. | 1.0 için **kabul**. 4. bölümdeki "giriş/çıkış adı" sunumu Ö2'ye iner: kullanıcının kendi `sahne "..."` satırında da kullanılabilmesi için. Yetişmezse sonraya; 1.0'ı engellemez. |
| Y5–Y7 efektleri | `uzun-golge`, `kabartma`, `supurme-agir` kaynakta **bulunamadı**. | Ö1'de **açık iş** olarak kalır; "premium ağır yazı" isteğinin en ucuz karşılığı bunlardır, Ö2 kalemlerinden önce yapılır. |
| A1–A3 | `altin-toz`, `isik-huzmesi`, `vinyet` hem efekt hem malzeme olarak var. | Karara uygun. |
| Ö2'den erken gelenler | `kis-manzarasi` (K13), `hediye-yigini` (K14), `izgara-zemin` (A4), `yazi-3b-don` (Y8), `yazi-3b-derin` (Y9'un malzeme hâli) var. | Kabul; Y8 için 10. bölüm 3. risk (perspektif) görsel denetimle doğrulanmalı. |
| Plansız kalemler | `yazi-2027`, `sahne-karli-isik`. | Kabul. Öneri (Malzeme ajanına, commit öncesi): `yazi-2027` kimliği yıla bağlı; bir yıl sonra eskir. Kimlik `yazi-yil`, varsayılan metin "üretim yılı + 1" olsun; metin zaten parametre. Kimlik commit'ten sonra değişirse eski senaryolar kırılır, bu yüzden şimdi karar verilmeli. |
| Başlamamış | `malzeme_alan.rs` yok → K15 `aurora`, A5 `sis`, A6 `sivi-metal`; ayrıca K16, `buz-koseler`, Y10–Y13. | Ö2/Ö3 sırası değişmez. |
| Piksel hücre boyu (P1) | Dönüştürücü yalnız 950 ve 1800'ü tam bölen 2, 5, 10, 25, 50 px kullanıyor. | **Kabul, P1'deki "6–32 px" aralığı pasif (2026-11-25); yerine bu küme geldi.** Tam bölenler kare pikseli korur; daha doğru. |
| Ek dönüştürücü stilleri | `retro8bit`, `piksel-giris`, boyama, kontur, çizgi roman 7. bölümdeki listenin dışında eklenmiş. | Kabul; koşul 7. bölümün ortak kuralları (beyaz listeli süzgeç metni, bulunurluk sınaması, enjeksiyon testi) her yeni stil için de geçerli. `karakalem`de kâğıt beyazının kısılması LED güvenliği kuralıyla uyumlu. |
| Belge | `docs/MALZEME.md` hâlâ "53 malzeme, 10 kategori" diyor; katalogda elle sayımla 78 kalem, 12 kategori var. | Commit öncesi Malzeme ajanı günceller (sayı testten alınmalı, elle sayım doğrulanmadı). |

### 11.3 Değişmeyenler
- Ekip (GPT/Gemini) görüşü hâlâ yok: `tasarim/yenileme/ekip/` klasörü var ama boş. 10. bölüm 1. risk aynen geçerli; farklı aileden inceleme kararı Yusuf'un.
- 1. bölümdeki bağlayıcı kurallar (kapalı biçim, determinizm, bütçe, LED güvenliği, ışıma/neon yasağı) ve 9. bölümdeki kabul ölçütü yeni ve plansız kalemler için de geçerlidir; bütçe ve flaş testleri bu kalemleri kapsamadan iş "tamam" sayılmaz.
