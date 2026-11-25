---
description: TAA Vidyo Yapıcı tema turu 2 için BAĞLAYICI karar (Fable 5.2). Üç mevcut temanın rafine sürümü, tek çekmece kuralı, üç yeni tema, İkiz Pile logosunun renklenmesi. Kişisel veri, anahtar içermez.
---
# KARAR TUR 2: çekmece kuralı ve 6 tema

Okunanlar: `BRIF-TUR2.md`, `KARAR-FABLE.md`, `gpt-astra-tur2.md`, `gemini-tur2.md`.
Dürüstlük notu: Kontrast WCAG 2 formülüyle betikle hesaplandı; tarayıcıda ve gerçek ekranda ölçülmedi. Hiçbir şey prototiplenmedi; piksel ölçüleri hesaptır. İlham sitelerine erişilmedi. Yasak listedeki temaların görünümünü görmedim; yalnız adları ve adın çağrıştırdığı fikirle karşılaştırdım. Gömülecek yazı tiplerinde Türkçe harfler sınanmadı.

Bu karar bağlayıcıdır ve `KARAR-FABLE.md`'nin 3. bölümünün (üç tema) yerine geçer. O kararın 2. bölümü (ortak yerleşim, 5 kip, kip içi kurallar, teknik kurallar) aşağıda açıkça değiştirilmeyen her yerde geçerlidir.

## 1. Sonuç (kısa)

- **Çekmece:** çalışma paneli her temada, hangi yanda olursa olsun, aynı kuralla kapanır; kapanınca yerinde hiçbir şey kalmaz. İlk açılışta açık.
- **Rafine:** Vitrin (krem ve serif gitti, vişne kaldı), Vuruş (limon doygunlaştı, çizgi azaldı), Cephe (132 px ray 76 px'e indi ve çekmecenin içine girdi).
- **Yeni:** Kadraj (buz mavisi/kobalt, GPT), Manşet (sarı/lacivert, GPT), Eşlik (erik/pembe, GPT; iskeleti değiştirildi).
- **Elenen:** Tezgâh (koyu nötr gri + kehribar: önceki turda elenen "koyu nötr kimliksiz" ve yasak amber; basınca küçülme hareketi yasak), Kılavuz (mimari plan/eskiz fikri yasak "Plan" ile aynı), Katman (nötr gri + mavi kimliksiz; sert gölge Cephe'de, üst sekme Vuruş'ta zaten var; yazı tipi ve ölçü çizgisi dışında yeni eksen getirmiyor).
- **Varsayılan tema:** Vitrin (değişmedi). Üç açık (Vitrin, Kadraj, Manşet), üç koyu (Vuruş, Cephe, Eşlik).

## 2. Çekmece kuralı (altı temada TEK davranış)

| Konu | Karar | Kimden |
|---|---|---|
| Çekmece nedir | Çalışma paneli ve, kip seçici panelin içindeyse (Cephe, Kadraj, Manşet), onunla birlikte kip seçici. Panel sağdaysa (Vuruş, Kadraj) kural aynen, ayna olarak uygulanır | GPT (sağ panel de kapanır); Fable |
| Düğme yeri | Üst çubuğun en solunda, program adından önce, 48x48 px, her temada aynı yerde. Panel başlığında ayrıca 48 px "Paneli kapat" düğmesi | GPT, Gemini (ortak) |
| Düğme etiketi | Simge + etkin kipin adı ("Yaz" gibi); erişilebilir ad "Paneli aç" / "Paneli kapat"; `aria-expanded`, `aria-controls`; araç ipucunda kısayol yazar | GPT (aria); kip adı Fable |
| Kısayol | **Ctrl+B**. Alt+1…5 kapalı çekmeceyi o kipte açar | Gemini (Ctrl+B). GPT'nin Ctrl+Shift+B'si elendi: üç tuş gereksiz |
| Kapalı hal | **Tamamen gizli, 0 px.** Simge rayı, şerit, tutamak kalmaz. Kip seçici panelin dışındaysa (Vitrin alt çubuk, Vuruş sekmeler, Eşlik üst çubuk) yerinde kalır ve bir kipe basmak çekmeceyi o kipte açar. Açıkken etkin kipe yeniden basmak kapatmaz | GPT (tam gizli) |
| Sahne | Boşalan genişliği alır ve yeniden ortalanır; kareler 950:1800 oranıyla, yükseklik sınırına kadar büyür. Çekmece sahnenin üstüne binmez, her zaman iter | GPT; önceki karar |
| Kalıcılık | `localStorage` `taa.cekmece` = `acik` / `kapali`; temadan bağımsız tek değer. Kullanılabilir genişlik 1200 CSS px altındaysa ayrı anahtar `taa.cekmece.dar` | Fable. GPT'nin "tema başına ayrı" önerisi elendi: tema değişince çekmece kendiliğinden açılıp kapanmamalı |
| İlk açılış | **1366x768'de AÇIK**, 1920x1080'de açık; Yaz kipi, metin kutusu odakta. Yalnız 1200 CSS px altında (ör. %150 ölçekleme) kapalı başlar | Fable |
| Genişlik | 1366'da 400 px, 1920'de 480 px (Cephe'de bunun 76 px'i ray). 1920'deki sağ iğneli panel kendi iğnesiyle kapanır, Ctrl+B ona dokunmaz | önceki karar |
| Animasyon | Sütun genişliği tek adımda değişir (tek yeniden yerleşim); panel 180 ms `translateX` ile girer/çıkar, sahne yeni yerine aynı sürede `translateX` ile kayar, `cubic-bezier(.2,0,0,1)`. Önizleme canvas'ı geçiş bitince bir kez yeniden ölçülür | GPT (kısa kayma, ölçüm sonda) |
| Hareketi azalt | Kayma yok; panel 120 ms solar | GPT, brif |
| Odak | Kapanırken odak paneldeyse açma düğmesine döner. Odak tuzağı yok; Esc kapatmaz (kalıcı panel, iletişim kutusu değil) | GPT |

Gerekçeler:
- **Neden 1366'da açık (iki öneri de "kapalı" dedi, reddedildi):** 768 px yükseklikte kareleri sınırlayan genişlik değil yüksekliktir. Hesap: 768 − 48 (üst) − 48 (araç) − 56 (aktarım) − 72 (eylem) − boşluk ≈ 515 px kare yüksekliği, yani kare 272 px en; iki kare + ara ≈ 570 px. Panel açıkken sahneye 966 px kalıyor; bu zaten yetiyor. Çekmeceyi kapatmak Düz görünümde kareyi büyütmez, yalnız çevre boşluğu ve "Bina karşıdan" görünümü kazanır. Buna karşılık kapalı başlamak, teknik olmayan kullanıcıyı ilk ekranda metin kutusuz bırakır ve "tek tık" kuralını bozar. Gemini'nin "%10-15 büyür" sayısı bu hesapla doğrulanmadı.
- **Neden 64 px simge rayı yok (Gemini, reddedildi):** program sahibi "kapatılabilir" dedi; yarım kapanan ray iki ayrı hal öğretir, etiketsiz simge teknik olmayan kullanıcıya uymaz ve 64 px'i sahneden almaya devam eder.
- **Neden sahne üstünde kip şeridi yok (GPT, reddedildi):** kapalıyken ikinci bir gezinme takımı doğar ve 768'de sahneden yükseklik alır; yükseklik burada en kıt şeydir.
- **Neden 300 ms genişlik geçişi yok (Gemini, reddedildi):** genişlik canlandırmak her karede yeniden yerleşim ve canvas ölçümü demektir; ortak kural hareketi `transform` ve `opacity` ile sınırlar. Gemini'nin önerdiği `transform: scale()` ile önizlemeyi büyütme de alınmadı (LED noktaları bulanır).

## 3. Altı temada ortak, bu turda eklenenler

- **Önizleme pervazı:** iki önizleme karesinin çevresinde her temada aynı, 6 px, nötr koyu pervaz (`#15171A`). Tema rengi (özellikle Manşet sarısı) video piksellerine değmez; renk yargısı temadan etkilenmez. (Fable)
- **Seçili öğe:** yalnız renkle değil; çerçeve + onay simgesi + "Seçili" etiketi. (GPT)
- **Dolu vurgu** yalnız Videoyu yap'ta; seçili sekme/kipte çizgi ve kalın etiket. (GPT; önceki karar)
- **Sürekli büyük harf yok**; büyük harf yalnız kısa başlıkta, `lang="tr"` ile. (GPT)
- Yasaklar aynen: ışıma, bulanıklık, gradyan, yaylanma, `steps()`, ölçekle basma/geri çekilme, titreme. Bu yüzden alınmayanlar: Gemini Vuruş "glitch", Tezgâh "scale 0.95 çökme" ve içe oyuk gölge, Cephe "buzlu cam" saydam sınır.

## 4. Mevcut üç tema, rafine

### 4.1 Vitrin (açık, VARSAYILAN)

- **Fikir:** aydınlık mağaza vitrini; porselen beyazı zemin, birbirine değmeyen adalar, tek vişne vurgu.
- **Jetonlar:** `--bg:#F1F0EC; --panel:#FFFFFF; --panel2:#E6E4DE; --line:#7C776E; --tx:#1D1B1A; --mut:#5A564F; --ac:#9A1B3C; --onac:#FFFFFF; --ok:#1E6B3A; --uy:#8A4B00`
- **Ne değişti:** krem (#F6F1E7) sarısı alınmış nötr porselene, fildişi panel saf beyaza döndü; serif tümüyle çıktı (yüklemede de yok); bordo kırmızısı azıcık soğutulup vişneye çekildi. Kalıbın üç ayağından ikisi (krem, serif) gitti, kimliği taşıyan üçüncüsü (kırmızı perde rengi) kaldı. Zemin yönü GPT (#F3F1EC'ye yakın); Gemini'nin kiremit vurgusu elendi (kiremit + taş + serif de aynı ölçüde görülmüş bir kalıp ve beyaz üstünde 4,56 ile sınırda), Instrument Serif + Geist elendi (serif kalıyor).
- **Yazı:** Segoe UI Variable (başlık Display 600, gövde Text 400), yedek Segoe UI. Gömülü yazı tipi yok. (Fable; GPT'nin Plex Sans'ı yerine yerleşik olan: varsayılan tema en hafif olsun)
- **Köşe:** kart 16 px, düğme 12 px, kip çubuğu tam hap (18'den indi; GPT 12 dedi, hap çubukla uyum için 16).
- **Yüzey:** adalar arası 12 px boşluk; üst üste gölge yok, tek keskin gölge `0 1px 3px` tx %10 + 1 px `--panel2` kenar (Gemini "ince keskin gölge", GPT "tek gölge"). Giriş kutularında 1 px `--line`.
- **Gezinme iskeleti:** kip seçici sahnenin altında ortalanmış yüzen hap çubuk (çekmecenin dışında, hep görünür); çalışma paneli **solda çekmece** (ada olarak). Gemini'nin "çubuğu sola 64 px ada yap" önerisi elendi: Cephe ile aynı iskelete düşer.
- **İmza hareket:** perde; yalnız kip değişiminde, panelin üstünde iki düz renk yarım ortadan iki yana `translateX` ile açılır, 200 ms. Akordeon açılışında yinelenmez (GPT).
- **Yükleme:** İkiz Pile'nin iki pilesi ortada bitişik durur, perde gibi iki yana açılır, arasından "TAA Vidyo Yapıcı" görünür; altında gerçek hazırlık metni.
- **Tanıtım, 4 sn:** perde açılır; "Yaz", "Seç", "Videoyu yap" sırayla ilgili gerçek kontrolün üstünde belirir; son kare arayüzdür.
- **Logo:** iki pile de `--ac` (vişne), beyaz ya da porselen üstünde.

### 4.2 Vuruş (koyu)

- **Fikir:** videonun ritmi; koyu patlıcan zemin, tok ıhlamur-limon vurgu, tipografik duraklar.
- **Jetonlar:** `--bg:#191826; --panel:#232234; --panel2:#2D2B42; --line:#807C9E; --tx:#F0EEF8; --mut:#B7B2CE; --ac:#C2DC55; --onac:#1A2006; --ok:#7FDCC4; --uy:#F7B466`
- **Ne değişti:** soluk #CBDA82 doygunlaştı (#C2DC55; Gemini'nin ilk turdaki neon #D9F854'ü ile eski soluk ton arasında), yalnız düz dolgu ve 2 px çizgi olarak. GPT'nin sarısı #F2CC4D elendi (Manşet'in sarısıyla ve yasak amberle karışır). Gemini'nin antrasit + zümrüt önerisi elendi: koyu nötr zemin önceki turda "kimliksiz" diye reddedildi, `--ok` ile `--ac` aynı renkti, çizgisi 1,61. Zemin Eşlik'in eriğinden ayrışsın diye bir tık maviye çekildi.
- **Yazı:** Bahnschrift; süre, kare ve çıktı ölçülerinde IBM Plex Mono (gömülü). Sürekli büyük harf yok. (GPT)
- **Köşe:** 2 px; hap yok.
- **Yüzey:** bitişik düz bloklar; her bloğu çerçevelemek yerine bölümler arasında 24 px boşluk ve bölge başına tek 1 px ayırıcı (GPT). Gemini'nin "yalnız sol ve üst kenara çizgi" önerisi alınmadı.
- **Gezinme iskeleti:** üst çubuğun altında tam genişlikte metinli sekme satırı (çekmecenin dışında); çalışma paneli **sağda çekmece**, sahne solda.
- **İmza hareket:** tek vuruş; seçilen sekmenin 2 px alt çizgisi 140 ms'de soldan sağa tamamlanır, içerik aynı anda belirir.
- **Yükleme:** İkiz Pile, altında gerçek hazırlık aşamasıyla dolan tek yatay çizgi ve tek aralıklı zaman kodu; bitince "Hazır".
- **Tanıtım, 4 sn:** "Yaz.", "Gör.", "Üret." üç durakta; son durak Videoyu yap'a bağlanır.
- **Logo:** sol pile `--tx`, sağ pile `--ac`.

### 4.3 Cephe (renkli koyu)

- **Fikir:** akşamüstü bina cephesi; petrol zemin, krem yazı, bakıra yakın mercan; arayüz binanın simetrisini taşır.
- **Jetonlar:** `--bg:#112B34; --panel:#193A45; --panel2:#224854; --line:#7FA5B1; --tx:#F4EEE3; --mut:#B7CAD0; --ac:#FFA58C; --onac:#3A1108; --ok:#8FE0B0; --uy:#FFD166`
- **Ne değişti:** ray 132 px'ten **76 px**'e indi (simge üstte, 13 px etiket altta; "Yapay zekâ" iki satır) ve çekmecenin içine girdi: 1366'da ray + panel 492 px'ten 400 px'e düştü, kapanınca ikisi birlikte gider. Yüzeyler bir tık açıldı (Gemini "çok karanlık"), mercan açıldı (ac/panel2 4,95'ten 5,19'a). Gemini'nin #0B1921 zemini elendi (daha da karanlık, çizgisi 2,12).
- **Yazı:** Space Grotesk (gömülü), başlık 500, kontroller 400 (GPT, Gemini: ağırlıklar inceldi); eksik harf çıkarsa Inter.
- **Köşe:** 10 px.
- **Yüzey:** gölgesiz ton katmanı. 2 px çerçeve yalnız sahne kabuğunda ve seçili öğede (GPT: "her yerde çerçeve yorucu"); seçili kart ve palette 3 px kaydırılmış düz alt blok kalır.
- **Gezinme iskeleti:** **sol çekmecenin içinde** dış kenarda 76 px etiketli dikey ray + yanında panel; seçili kipin yanında iki kısa dikey çizgi; 1920'de sağ iğneli panelle sahne tam ortada, simetrik.
- **İmza hareket:** yükseliş; yalnız panel içeriği aşağıdan 12 px kayarak gelir, 180 ms (16 px/200 ms'den kısaldı; GPT).
- **Yükleme:** İkiz Pile'nin iki pilesi alttan yukarı mercanla dolar; altında gerçek hazırlık metni.
- **Tanıtım, 5 sn:** cephe silueti çizilir, iki ekran sırayla dolar, içlerinden "HOŞ GELDİNİZ" yükselir, görüntü arayüze açılır; kamera hareketi yok.
- **Logo:** iki pile de `--ac` (mercan/bakır), petrol üstünde.

## 5. Üç yeni tema

### 5.1 Kadraj (açık) — GPT'den, iskeleti değişti

- **Fikir:** videoyu temiz bir izleme penceresi olarak sunmak; soğuk, sakin, en okunaklı tema.
- **Jetonlar:** `--bg:#EAF2FA; --panel:#FFFFFF; --panel2:#D9E7F4; --line:#60758A; --tx:#122C43; --mut:#41566A; --ac:#174BD1; --onac:#FFFFFF; --ok:#24613D; --uy:#795000` (GPT, aynen; hesapta hepsi geçti)
- **Yazı:** Atkinson Hyperlegible Next (gömülü, SIL OFL), yedek Segoe UI. (GPT; "Next" sürümü bu kararda belirtildi, ilk sürümün dil kapsamı dar)
- **Köşe:** 4 px (GPT 6 dedi; Cephe'nin 10'undan uzaklaşsın diye 4).
- **Yüzey:** gölgesiz; buz mavisi çevre, beyaz iç yüzey, 1 px `--line` iç kenar yalnız sahneyi ve etkin bölümü toplar. Izgara, cetvel, plan çizimi yok.
- **Gezinme iskeleti:** çalışma paneli **sağda çekmece**; 5 kip panelin başlığında yatay 5 bölmeli seçici (simge üstte, 13 px etiket altta, 56 px yüksek). GPT'nin "üstte seçici + sağ panel"i Vuruş ile aynı iskelet olduğu için seçici panelin içine alındı.
- **İmza hareket:** köşe yerleşimi; seçilen öğenin ve kip değişiminde sahnenin dört köşe işareti 8 px dışarıdan içeri oturur, 140 ms; sürekli animasyon yok.
- **Yükleme:** iki dikey boş pencere, köşe işaretleri oturur, ortada İkiz Pile; altında gerçek hazırlık metni.
- **Tanıtım, 4 sn:** metin → iki önizleme → "Videoyu yap"; her adım gerçek kontrolde.
- **Logo:** iki pile de `--ac` (kobalt), beyaz üstünde.

### 5.2 Manşet (açık, renkli) — GPT'den, iskeleti netleşti

- **Fikir:** mağazanın söyleyeceği cümleyi başrole alan tok ve neşeli duyuru dili; sarı alan, mürekkep yazı, lacivert tek eylem.
- **Jetonlar:** `--bg:#F5DC72; --panel:#FFF9E5; --panel2:#E9CD61; --line:#7A6C2E; --tx:#24221B; --mut:#514927; --ac:#173C58; --onac:#FFFFFF; --ok:#205D3D; --uy:#754800` (GPT; `--line` bu kararda #897A3B'den #7A6C2E'ye koyulaştı: panel2 üstünde 2,72 idi)
- **Yazı:** Archivo (gömülü, SIL OFL), başlık 650, arayüz 400. (GPT)
- **Köşe:** 0 px.
- **Yüzey:** geniş düz renk alanları, gölge yok; ayrım alan rengi ve akordeon satırları arasında 2 px `--tx` çizgiyle. Baskı dokusu, tram, mürekkep kayması yok (yasak Risograf'a yaklaşmasın); amber yok, sarı düz.
- **Gezinme iskeleti:** **sol çekmece, dikey akordeon**: 5 kip beş büyük metin satırı (seçili 56 px, ötekiler 48 px); seçilen satır yerinde açılır, içerik başlığının altındadır; seçili satırın solunda kısa dolu dikdörtgen. (GPT'nin 240 px metin menüsü + ayrı panel yerine tek sütun; Fable)
- **İmza hareket:** satır açılması; yeni içerik bir satırlık kırpma alanından yukarıdan aşağı 160 ms'de görünür (maske içinde `translateY`); arayüz yerinden oynamaz.
- **Yükleme:** sarı alanda mürekkep İkiz Pile ve tek satır gerçek aşama metni; sahte yüzde yok.
- **Tanıtım, 3 sn:** "Yaz / Önizle / Videoyu yap" satır satır açılır, her biri gerçek kontrolü gösterir.
- **Logo:** iki pile de `--tx` (mürekkep), sarı üstünde; lacivert yalnız Videoyu yap'a kalır.

### 5.3 Eşlik (koyu, renkli) — GPT'den, iskeleti ve yazısı değişti

- **Fikir:** yazılan mesaj ile LED'deki karşılığını yan yana tutan sıcak gece arayüzü; erik zemin, pudra pembe vurgu, büyük yumuşak kapsayıcılar.
- **Jetonlar:** `--bg:#351D2A; --panel:#452A39; --panel2:#57384A; --line:#B08CA2; --tx:#FFF3F9; --mut:#D8BFCD; --ac:#F4C3DA; --onac:#3A1428; --ok:#B8DFC7; --uy:#F4D198` (GPT; zemin ailesi Vuruş'un patlıcanından ayrışsın diye bir tık kırmızıya çekildi, öteki jetonlar buna göre ayarlandı)
- **Yazı:** Nunito (gömülü, SIL OFL), yedek Segoe UI. (Fable; GPT'nin Noto Sans'ı Vitrin'in Segoe'siyle ayırt edilemiyordu; yuvarlak uçlu yazı 24 px köşeyle aynı dili konuşur)
- **Köşe:** kapsayıcı 24 px, kontrol 12 px.
- **Yüzey:** gölgesiz, çizgisiz; iç içe ton kapsayıcıları (bg, panel, panel2). `--line` yalnız giriş kutusu ve odakta.
- **Gezinme iskeleti:** 5 kip **üst çubuğun ortasında** hap grubu (ayrı satır yok, çekmecenin dışında); çalışma paneli **solda çekmece**. GPT'nin "sahne altında 152 px çalışma bandı" **reddedildi**: 768'de sahneden yaklaşık 150 px yükseklik alır (kare 515'ten 360 px dolayına iner), "sahne küçülmez" kuralını bozar ve 122 animasyon ile büyük metin kutusu o banda sığmaz.
- **İmza hareket:** eşzamanlı belirme; değişen kontrol ile sahnedeki karşılığının etiketi aynı anda 140 ms'de belirir (yalnız `opacity`).
- **Yükleme:** iki pile aynı anda belirir; altında gerçek hazırlık metni.
- **Tanıtım, 4 sn:** bir mesaj yazılır, iki ekranda aynı anda belirir, ardından gündüz/akşam karşılaştırması.
- **Logo:** sol pile `--ac` (pembe), sağ pile `--tx`.

## 6. Logo: İkiz Pile (program sahibinin seçimi)

`viewBox="0 0 64 64"`; yollar: `M8 8 H20 L28 32 L20 56 H8 L16 32 Z` ve `M44 8 H56 L48 32 L56 56 H44 L36 32 Z`. Her pile tek düz dolgu; çizgi, gradyan, gölge yok; en küçük 24 px. Dolu vurgu yüzeyinin üstünde iki pile de `--onac`. Özel vurgu rengi seçilirse logo onu izler.

| Tema | Sol pile | Sağ pile | Zemin |
|---|---|---|---|
| Vitrin | `--ac` #9A1B3C | `--ac` | panel/bg |
| Vuruş | `--tx` #F0EEF8 | `--ac` #C2DC55 | bg |
| Cephe | `--ac` #FFA58C | `--ac` | bg |
| Kadraj | `--ac` #174BD1 | `--ac` | panel |
| Manşet | `--tx` #24221B | `--tx` | bg |
| Eşlik | `--ac` #F4C3DA | `--tx` #FFF3F9 | bg |

## 7. Kontrast (hesaplanmış)

| Çift | Eşik | Vitrin | Vuruş | Cephe | Kadraj | Manşet | Eşlik |
|---|---|---|---|---|---|---|---|
| tx / bg | 4,5 | 15,05 | 15,24 | 12,82 | 12,66 | 11,64 | 14,29 |
| tx / panel2 | 4,5 | 13,49 | 11,91 | 8,57 | 11,37 | 10,13 | 9,39 |
| mut / bg | 4,5 | 6,40 | 8,56 | 8,72 | 6,72 | 6,59 | 9,01 |
| mut / panel | 4,5 | 7,29 | 7,61 | 7,14 | 7,60 | 8,55 | 7,46 |
| mut / panel2 | 4,5 | 5,74 | 6,69 | 5,83 | 6,04 | 5,73 | 5,92 |
| onac / ac | 4,5 | 8,09 | 10,90 | 8,72 | 7,09 | 11,52 | 10,43 |
| ac / bg | 4,5 | 7,09 | 11,39 | 7,76 | 6,27 | 8,42 | 10,04 |
| ac / panel | 4,5 | 8,09 | 10,12 | 6,36 | 7,09 | 10,93 | 8,32 |
| ac / panel2 | 4,5 | 6,36 | 8,90 | 5,19 | 5,63 | 7,33 | 6,60 |
| ok / panel2 | 4,5 | 5,13 | 8,42 | 6,34 | 5,85 | 4,96 | 6,97 |
| uy / panel2 | 4,5 | 5,35 | 7,60 | 6,86 | 5,64 | 4,98 | 6,97 |
| line / bg | 3 | 3,90 | 4,41 | 5,58 | 4,21 | 3,83 | 5,23 |
| line / panel | 3 | 4,45 | 3,92 | 4,57 | 4,76 | 4,97 | 4,33 |
| line / panel2 | 3 | 3,50 | 3,45 | 3,73 | 3,78 | 3,33 | 3,44 |

Sonuç: istenen altı çift (tx/bg, mut/bg, mut/panel2, onac/ac, ac/panel ≥ 4,5; line/bg ≥ 3) altı temada geçiyor; tablodaki öteki çiftler de eşiğin üstünde. En dar paylar: Manşet ok/panel2 4,96 ve uy/panel2 4,98; Cephe ac/panel2 5,19. Bu renkler açılmamalı.

Önerilerde hesapla tutmayan ve bu yüzden alınmayan değerler: Gemini'nin altı temasının tamamında `--line` / bg 3'ün altında (1,17 ile 2,12 arası); Katman mut/bg 3,84 ve durum renkleri panel2 üstünde 2,9 ve 3,0; Kılavuz mut/bg 4,31, mut/panel2 3,85; Tezgâh mut/panel2 4,11; rafine Vitrin'de uy/panel2 3,51. Gemini'nin "tamamı WCAG AA'dan geçer" cümlesi doğrulanmadı. GPT'nin setleri kendi saydığı üç çiftte geçiyor; tek eksik Manşet line/panel2 2,72 idi, düzeltildi.

## 8. Fark tablosu (6 tema)

| Eksen | Vitrin | Vuruş | Cephe | Kadraj | Manşet | Eşlik |
|---|---|---|---|---|---|---|
| Zemin | açık porselen | koyu patlıcan | petrol | açık buz mavisi | sarı | koyu erik |
| Vurgu | vişne | ıhlamur-limon | mercan | kobalt | lacivert | pudra pembe |
| Yazı | Segoe UI Variable | Bahnschrift + Plex Mono | Space Grotesk | Atkinson Hyperlegible Next | Archivo | Nunito |
| Köşe | 16 px, hap | 2 px | 10 px | 4 px | 0 px | 24 / 12 px |
| Yüzey | ayrık adalar, tek ince gölge | bitişik düz blok, tek ayırıcı | ton katmanı, seçilide 2 px çerçeve + alt blok | beyaz yüzey, 1 px iç kenar | geniş renk alanı, 2 px mürekkep çizgi | çizgisiz iç içe kapsayıcı |
| Gezinme | alt yüzen çubuk, sol çekmece | üst sekme satırı, sağ çekmece | sol çekmecede 76 px ray | sağ çekmece, başlıkta 5 bölme | sol çekmece, akordeon | üst çubukta hap, sol çekmece |
| İmza hareket | perde (yatay açılma) | vuruş (çizgi süpürme) | yükseliş (dikey kayma) | köşe yerleşimi | satır açılması (maske) | eşzamanlı belirme |

Yedi eksenin altısında altı tema da birbirinden farklı. Tek yakınlık: Kadraj ile Manşet'in vurgusu ikisi de mavi ailesinden (kobalt, lacivert); bu çift öteki altı eksende ayrışıyor, yani her tema çifti en az altı eksende farklı (şart dört).

Yasak ad denetimi: Kadraj, Manşet, Eşlik brifteki 15 Telefon Köprüsü adıyla, İlmek/Kalıp/Mezura ile ve eski üç tema adıyla çakışmıyor; Reji, Prizma, Kozalar geri gelmedi. Fikir yakınlığı riski olan iki yer: Manşet (baskı çağrışımı, Risograf) ve Kadraj (çerçeve çizgisi, Çizgi/Plan); ikisi de yukarıda dokusuz ve ızgarasız tanımlandı, uygulayan ekip o temalarla yan yana bir kez karşılaştırmalı.

## 9. Açık kalanlar (uygulamada sınanacak)

1. Atkinson Hyperlegible Next, Archivo, Nunito, Space Grotesk, IBM Plex Mono dosyalarında ğ ş ı İ ç ö ü sınanmadı; eksik çıkan temada yedek Segoe UI Variable.
2. Manşet akordeonu: 768 px yükseklikte beş başlık yaklaşık 250 px alır, içerik için yaklaşık 350 px kalır ve kendi içinde kayar. Ayrıntılar kipinde dar gelirse seçili olmayan başlıklar 48 px'te kalır, başka çözüm bu dosyaya not düşülerek alınır.
3. Manşet sarısının uzun kullanımda yorup yormadığı ve pervazın renk yargısını gerçekten koruduğu gerçek ekranda bakılmalı.
4. Kadraj başlık seçicisi ve Cephe rayında "Yapay zekâ" ile "Ayrıntılar" 13 px etiketle sığıyor mu, prototipte ölçülmeli; kısaltma yok, gerekirse iki satır.
5. Sağ çekmeceli temalarda (Vuruş, Kadraj) açma düğmesi yine sol üsttedir; tutarlılık için böyle karar verildi, kullanıcı şaşırırsa ilk geri bildirimde yeniden bakılır.
6. 1366'da kare ölçüleri hesaptır (pencere başlığı ve görev çubuğu düşülmedi; gerçek kare 470 px dolayına inebilir).
7. Altı temada üç gömülü yazı ailesi daha eklendi; paket boyutu ölçülmedi.

## 10. Aday notlar

- aday not: Vidyo Yapıcı çekmece kuralı: düğme üst çubuğun en solunda, Ctrl+B, kapalıyken 0 px (ray kalmaz), tek kalıcı değer `taa.cekmece`, 1366 ve 1920'de açık başlar, 180 ms translateX, hareketi azaltta 120 ms solma. (kaynak: KARAR-FABLE-TUR2.md, 2026-11-17)
- aday not: Vidyo Yapıcı'nın altı teması: Vitrin (varsayılan), Vuruş, Cephe, Kadraj, Manşet, Eşlik; jetonlar ve kontrast tablosu KARAR-FABLE-TUR2.md'de. (kaynak: aynı)
- aday not: Vitrin artık krem/serif değil: porselen zemin #F1F0EC, Segoe UI Variable, vişne #9A1B3C. (kaynak: aynı)
- aday not: Tezgâh, Kılavuz, Katman adları Vidyo Yapıcı'da kullanılmadı. (kaynak: aynı)
- aday not: Program logosu İkiz Pile; her pile tek düz dolgu, temaya göre `--ac` ya da `--tx`. (kaynak: aynı)

[Claude Fable 5.2, TAA Video Yapıcı arayüz başlığı, 2026-11-17]
