---
description: TAA Vidyo Yapıcı yeni yerleşim ve 3 tema için BAĞLAYICI tasarım kararı (Fable 5.2). Üç ekip yanıtının (Fable, GPT Astra, Gemini) birleşimi. Kişisel veri, anahtar içermez.
---
# KARAR: TAA Vidyo Yapıcı, tek ortak yerleşim ve 3 tema

Okunanlar: `BRIF-3TEMA-2026-11-17.md`, `ekip/fable-yanit.md`, `ekip/gpt-astra-yanit.md`, `ekip/gemini-yanit.md`.
Dürüstlük notu: Kontrast WCAG 2 formülüyle betikle hesaplandı; tarayıcıda ölçülmedi. İlham sitelerine bu oturumda erişilmedi. Paketteki öteki programın temalarının jetonlarını görmedim; yalnız brifteki yasak adlarla karşılaştırdım.

Bu karar bağlayıcıdır. Uygulayan ekip aşağıdaki "Açık kalanlar" dışında yorum yapmaz; sapma gerekiyorsa önce bu dosyaya not düşer.

## 1. Sonuç (kısa)

- **Yerleşim:** sahne ortada ve sabit, 5 kip, aynı anda tek kip açık, altta sabit eylem çubuğu, ekranda tek dolu düğme (Videoyu yap).
- **Temalar:** **Vitrin** (açık, krem/bordo), **Vuruş** (koyu, patlıcan/ıhlamur), **Cephe** (renkli koyu, petrol/mercan).
- **Varsayılan tema:** Vitrin.
- **Elenenler:** Reji ve Prizma (koyu nötr; brifteki "koyu-nötr, kimliği yok" şikâyetini yineliyor ve çizgili/köşeli dili yasak listedeki adlara yaklaşıyor), İkiz ve Mercek (ikinci ve üçüncü açık tema gereksiz; fikirleri aşağıda başka yerlere taşındı), Kozalar ve Kütle (ad olarak elendi; yüzen ada, patlıcan zemin ve kalın çerçeve fikirleri alındı).

## 2. Ortak yerleşim (üç temada aynı)

### 2.1 Bölgeler

| Bölge | Karar | Kimden |
|---|---|---|
| Üst çubuk, 48 px | Program adı ve sürüm; sağda Yeni video, Rehber, Ayarlar | Fable (48 px, Rehber), GPT (Yeni video üstte) |
| Kip seçici | 5 kip, tek tıkla, sayfa değişmez. Konumu temaya göre değişir (bkz. 3) | Fable (5 kip, konum temaya göre), GPT (aynı fikir: iş seçici) |
| Çalışma paneli | Seçili kipin içeriği. Yalnız bu panel kendi içinde kayar; sayfa kaymaz | Fable, GPT (ortak) |
| Sahne (merkez) | "Tasarım" ve "Ekranda nasıl görünür" yan yana, 950:1800 oranı korunur. Üstünde tek satır araç: Düz / Bina karşıdan, Gündüz / Akşam, 3 m / 15 m, Güvenli alan, Tam ekran | Fable (araç satırı), GPT (oran kilidi, cephe kırpılmaz) |
| Aktarım satırı, 56 px (sahnenin altı) | Oynat/duraklat, zaman çubuğu, geçen/toplam süre; sağ ucunda durum hapı | Fable (aktarım), Gemini (durum önizlemenin altında hap) |
| Eylem çubuğu, 72 px (alt, sabit) | Sağda **Videoyu yap** (56 px, tek dolu düğme) ve ilerleme; solunda çerçeveli ikinciller: Son videoyu oynat, Klasörü aç | Fable (tek dolu düğme), GPT ve Gemini (alt sabit eylem) |

### 2.2 Kipler (ad, sıra, simge, kısayol üç temada AYNI)

| Sıra | Ad | Simge | Kısayol | İçerik |
|---|---|---|---|---|
| 1 | **Yaz** | kalem | Alt+1 | Ne yazsın? metin kutusu ("Boş satır yeni mesaj başlatır"), şablon şeridi, geri al/yinele |
| 2 | **Hazır** | dörtlü ızgara | Alt+2 | 13 senaryo kartı: küçük önizleme, metni düzelt, "Bu senaryoyu yükle" |
| 3 | **Yapay zekâ** | konuşma balonu | Alt+3 | 5 adım: Yaz, İstemi kopyala, Yapay zekâya yapıştır, Yanıtı yapıştır, Uygula |
| 4 | **Görünüm** | palet | Alt+4 | 5 stil, renk noktaları, 12 palet, Sıfırla |
| 5 | **Ayrıntılar** | sürgüler | Alt+5 | Animasyon (122, arama, çip), Yazı ve renk (10 font), Açılış ve kapanış, Çerçeve/zemin/efekt, Çıktı, Gelişmiş senaryo metni |

- Beş kip ve sıra: Fable. "Yapay zekâ" adı: GPT (Fable'ın "Tarif et" adı elendi; teknik olmayan kullanıcı için ne olduğu daha açık). Panel içi başlık "Yapay zekâya yazdır" olarak kalır.
- Fable'ın Cephe için önerdiği "5 kipi 3 adıma toplayan şerit" **reddedildi**: kip adı ve sırası temalar arasında değişmez kuralını bozuyor, tek tık kuralını riske atıyor (riski Fable kendisi de yazmıştı).
- Gemini'nin "yapay zekâ varsayılan kapalı çekmece" fikri kip olarak karşılandı: açılışta Yaz kipi açık, metin kutusu odaklı (Fable).

### 2.3 Kip içi kurallar

- **Yapay zekâ:** tek satır adım göstergesi; yalnız etkin adımın düğmeleri görünür (Fable). Tüm düğmeler korunur: Kuraldan çöz, İstemi kopyala, ChatGPT'de aç, Gemini'de aç, DeepSeek'te aç, Claude'da aç, + Site, Panodaki tarifi uygula; "İstemi göster", "Doğrudan bağlantı (isteğe bağlı)", "Kural listesi" kapalı açılır bölüm (GPT listesi, Fable yerleşimi).
- **Hazır:** kart tıklanınca sahnede önizlenir (Fable). 13 senaryo için arama ve kategori süzgeci **eklenmez** (GPT önerisi reddedildi: 13 kart için gereksiz yük); iç içe pencere yok (GPT).
- **Görünüm:** stil ve palet seçeneklerinin üstünde örnek "TAA" yazısı (GPT). "Şaşırt beni" bugün programda varsa korunur, yoksa eklenmez (Fable yanıtında geçiyor, brifte yok; bkz. Açık kalanlar).
- **Ayrıntılar:** altı bölüm akordeon (Gemini, GPT). Neon yalnız video efekti olarak kalır, arayüzde yok (GPT).
- **Modülerlik:** bölümler katlanır, gizlenir, iğnelenir, sıralanır; düzen localStorage'da, "Düzeni sıfırla" her zaman görünür (Fable). Sıralama için sürüklemeye ek **Yukarı taşı / Aşağı taşı** düğmeleri zorunlu (GPT; klavye ve dokunmatik için).
- **Durum hapı:** metinli ("Okunabilirlik uygun" ya da uyarı cümlesi), simgeli, renk tek başına anlam taşımaz (GPT). Tıklanınca uyarı listesi ve öneriler açılır (Fable). Yeri aktarım satırının sağ ucu (Gemini).
- **Düğme hiyerarşisi:** bir dolu düğme (Videoyu yap); kip içi birincil işlem çerçeveli vurgu; gerisi düz (Fable).
- **Üretim:** gerçek ilerleme, aşama adı ve İptal; sahte yüzde yok (GPT).
- **Yeni video:** kaydedilmemiş değişiklik varsa işlem geri alınabilir (GPT).
- **Kullanılamayan görünüm seçeneği** gizlenmez; neden kullanılamadığı yazılır (GPT).

### 2.4 Çözünürlük

| | 1366x768 | 1920x1080 |
|---|---|---|
| Çalışma paneli | 400 px (Cephe'de etiketli ray 132 + panel 360) | 480 px |
| Sahne çerçeveleri | her biri yaklaşık 270x515 px | yaklaşık 420x800 px |
| İkinci panel | yok; Ayrıntılar ve Görünüm kip olarak açılır | sağda 360 px iğneli panel; varsayılan Görünüm iğneli, kullanıcı değiştirir |
| Sığmayanlar | Güvenli alan ve Tam ekran "…" menüsüne; Vitrin'de ikincil eylemler "…" menüsüne | hepsi açık |

- 1366 ölçüleri ve "sahne küçülmez, ikinciller menüye iner" kuralı: Fable. 1920'de üç sütun (sol içerik, orta sahne, sağ görünüm): Gemini ve Fable ortak. Windows ölçeklemesinde piksel değil kullanılabilir CSS alanı esas alınır; panel daralırsa çekmeceye döner ve sahnenin üstünü kapatmaz: GPT.
- Gemini'nin 1366'da "sahnenin üstünde yüzen paneller" önerisi **reddedildi** (önizleme her zaman görünür kuralını bozar).

### 2.5 Ortak teknik kurallar

- Mevcut JS id'leri (`uret`, `tarifCoz`, `hazirListe` vb.) korunur; yalnız kapsayıcılar değişir (Fable).
- Tema `html[data-tema]`; seçim ve tema başına renk geçersiz kılmaları localStorage'da (GPT, brif).
- Tema kartı: "Aa", Türkçe örnek "İğne, ölçü, ŞİMDİ", 4 renk noktası (bg, panel, tx, ac), canlı mini arayüz (GPT).
- Özel vurgu/zemin rengi 4,5 altına düşürürse `--onac` ve `--tx` otomatik siyah ya da beyaza çekilir, kullanıcıya soru sorulmaz (Fable, GPT).
- Dokunma hedefi en az 48 px, ana eylem 56 px; gövde 16 px, ikincil en az 13 px (koddaki 11,5 ve 12 px'ler 13'e çıkar: Fable). Görünür odak halkası: 2 px `--ac` + 2 px `--bg` boşluk.
- Düğmeler metne göre esner, gerekirse iki satır, üç nokta ile kesme yok (üçü ortak). Büyük harf `toLocaleUpperCase('tr-TR')` ya da `lang="tr"` (üçü ortak).
- Hareket yalnız `transform` ve `opacity`. `backdrop-filter`, bulanıklık, ışıma, canlandırılan gölge, yaylanma, `steps()` ve ölçekle geri çekilme **yasak** (Fable, GPT; Gemini'nin Kozalar yaylanması, Kütle bulanıklığı ve Prizma `steps()` hareketi bu yüzden reddedildi).
- "Hareketi azalt": imza hareket yerine 120 ms solma; tanıtım tek durağan kare (üçü ortak).
- Tanıtım ilk açılışta bir kez oynar, "Geç" düğmesi her zaman vardır, yüklemeyi geciktirmez; sonra yalnız Ayarlar'dan (Fable, GPT).
- İki önizleme aynı video karesini paylaşır; LED dokusu önbellekli canvas dokusudur (GPT).
- Gömülen yazı tipleri pakete lisans dosyasıyla girer, internetten yükleme yok (GPT, brif).

## 3. Üç tema

### 3.1 Vitrin (açık, VARSAYILAN)

- **Fikir:** mağaza vitrini gibi aydınlık; krem kumaş zemin, bordo tek vurgu. (Fable)
- **Jetonlar:** `--bg:#F6F1E7; --panel:#FFFDF8; --panel2:#EDE6D8; --line:#857B69; --tx:#1F1B16; --mut:#5E564A; --ac:#9B1C31; --onac:#FFFFFF; --ok:#1E6B3A; --uy:#8A4B00` (Fable; `--line` bu kararda #8F8573'ten #857B69'a koyulaştırıldı, panel2 üstünde de 3:1 geçsin diye)
- **Yazı:** başlık IBM Plex Serif (gömülü), gövde Segoe UI Variable, yedek Segoe UI. (Fable; yedek GPT)
- **Köşe:** kart 18 px, düğme 14 px, kip çubuğu tam hap. (Fable)
- **Yüzey:** çizgisiz kartlar; birbirine değmeyen, aralarında 16 px boşluk olan adalar; durağan, tek katman yumuşak gölge (`0 2px 12px` tx rengi %8). Giriş kutularında 1 px `--line`. (Fable gölge, Gemini "yüzen adalar")
- **Gezinme iskeleti:** kip seçici **altta ortalanmış yüzen çubuk** (simge + etiket); çalışma paneli solda; Videoyu yap sağ altta ayrı hap. (Fable)
- **İmza hareket:** perde; panel içeriği ortadan iki yana açılan düz renk perdeyle değişir, 220 ms. (Fable)
- **Yükleme:** krem zeminde iki bordo dikey şerit (iki ekran) perde gibi açılır, arasından "TAA Vidyo Yapıcı" görünür; altında gerçek hazırlık durumu metni. (Fable; durum metni GPT)
- **Tanıtım, 4 sn:** perde açılır; "Yaz", "Seç", "Videoyu yap" sırayla yerine oturur; son kare arayüzün kendisidir. (Fable; son karenin arayüzle birleşmesi GPT)

### 3.2 Vuruş (koyu)

- **Fikir:** videonun zaman duygusu; koyu patlıcan zemin, ıhlamur yeşili vurgu, güçlü tipografik duraklar. (ad, fikir GPT; zemin ailesi Gemini Kütle)
- **Jetonlar:** `--bg:#1B1924; --panel:#252233; --panel2:#2E2A40; --line:#7D779C; --tx:#EFEDF7; --mut:#B5B0CC; --ac:#CBDA82; --onac:#1A1F08; --ok:#86DCC6; --uy:#F5B971`
  - bg, panel, panel2: Gemini Kütle, aynen. GPT'nin vişne zemini elendi: Vitrin'in bordosuyla aynı renk ailesine düşüyordu.
  - `--ac`: Gemini'nin neon limonu #D9F854 **yumuşatıldı** (#CBDA82, doygunluk yarıya); ışıma yok, yalnız düz dolgu ve 2 px çizgi olarak kullanılır.
  - `--ok`: Gemini'nin "yeşil yerine camgöbeği" fikri yumuşatılarak nane-turkuaz yapıldı; vurgu ıhlamurundan ayrışsın diye. `--uy`: Gemini turuncusu #FB923C yumuşatıldı.
  - `--line`, `--tx`, `--mut`: bu kararda kontrast için açıldı (Gemini'nin #4E496A çizgisi bg üstünde 3:1 altındaydı).
- **Yazı:** Bahnschrift (başlık ve gövde); süre, kare ve ölçülerde IBM Plex Mono (gömülü). (GPT; Fable Reji de Bahnschrift + tek aralıklı sayı önermişti)
- **Köşe:** 2 px; hap ve kapsül yok. (GPT, Fable Reji)
- **Yüzey:** düz renk blokları; gölge ve gradyan yok; paneller pencere kenarına kadar uzanır, aralarında boşluk yok, ayrım ton farkı ve tek 1 px çizgiyle. (GPT blok, Gemini Prizma "uçtan uca")
- **Gezinme iskeleti:** sol ray yok; **üstte geniş, metinli yatay kip sekmeleri**; çalışma paneli **sağda**, sahne solda geniş. (GPT)
- **İmza hareket:** tek vuruş; seçilen sekmenin 2 px alt çizgisi 160 ms'de soldan sağa tamamlanır, panel içeriği aynı anda belirir; yanıp sönme yok. (GPT; süpürme yönü Fable Reji "oynatma kafası")
- **Yükleme:** büyük "TAA", altında gerçek hazırlık aşamasıyla dolan tek yatay çizgi ve tek aralıklı zaman kodu; bitince "HAZIR". (GPT çizgi, Fable Reji zaman kodu)
- **Tanıtım, 4 sn:** "Yaz.", "Gör.", "Üret." üç durakta belirir; son durak Videoyu yap düğmesine bağlanır. (GPT)

### 3.3 Cephe (renkli koyu)

- **Fikir:** akşamüstü bina cephesi; petrol mavisi zemin, krem yazı, bakır logoya yakın mercan vurgu; arayüz binanın simetrisini taşır. (Fable)
- **Jetonlar:** `--bg:#102A33; --panel:#173843; --panel2:#1F4653; --line:#7FA5B1; --tx:#F3EDE2; --mut:#B4C8CD; --ac:#FF9A82; --onac:#3A0F06; --ok:#8FE0B0; --uy:#FFD166` (Fable; `--line` bu kararda #5F8794'ten #7FA5B1'e açıldı)
- **Yazı:** Space Grotesk (gömülü) başlık ve gövde; ğ ş ı İ gömmeden önce sınanır, eksikse Inter (gömülü). (Fable; Gemini Kütle de Space Grotesk önermişti)
- **Köşe:** 10 px. (Fable)
- **Yüzey:** gölgesiz ton katmanı (bg, panel, panel2). Etkileşimli ve seçili öğelerde 2 px çerçeve; seçili kart ve seçili palet ayrıca 3 px kaydırılmış düz (bulanık olmayan, durağan) alt blokla öne çıkar. (ton katmanı Fable; 2 px çerçeve ve düz sert gölge Gemini Kütle)
- **Gezinme iskeleti:** **solda etiketli dikey ray** (132 px, simge + ad; seçili kipin yanında iki kısa dikey çizgi, iki ekrana gönderme); çalışma paneli rayın yanında; 1920'de sağdaki iğneli panelle sahne tam ortada, **simetrik**. (etiketli ray ve çift çizgi GPT İkiz; simetri Fable)
- **İmza hareket:** yükseliş; yeni içerik aşağıdan yukarı 16 px kayarak gelir, 200 ms. (Fable)
- **Yükleme:** iki dikey dikdörtgen alttan yukarı krem renkle dolar, ortada düz baklava biçimi belirir (logo kopyası değil). (Fable)
- **Tanıtım, 5 sn:** cephe silueti çizilir, iki ekran sırayla dolar, içlerinden "HOŞ GELDİNİZ" yükselir, görüntü arayüze açılır; kamera hareketi yok. (Fable; "baş döndüren kamera yok" GPT Mercek)

## 4. Kontrast (hesaplanmış)

| Çift | Eşik | Vitrin | Vuruş | Cephe |
|---|---|---|---|---|
| tx / bg | 4,5 | 15,21 | 14,97 | 12,86 |
| tx / panel2 | 4,5 | 13,79 | 11,91 | 8,75 |
| mut / bg | 4,5 | 6,42 | 8,30 | 8,63 |
| mut / panel | 4,5 | 7,11 | 7,41 | 7,18 |
| mut / panel2 | 4,5 | 5,82 | 6,60 | 5,87 |
| onac / ac | 4,5 | 8,06 | 11,17 | 8,15 |
| ac / bg | 4,5 | 7,16 | 11,47 | 7,28 |
| ac / panel | 4,5 | 7,93 | 10,25 | 6,06 |
| ac / panel2 | 4,5 | 6,49 | 9,13 | 4,95 |
| ok / panel2 | 4,5 | 5,25 | 8,59 | 6,54 |
| uy / panel2 | 4,5 | 5,48 | 7,93 | 7,07 |
| line / bg | 3 | 3,71 | 4,12 | 5,65 |
| line / panel | 3 | 4,10 | 3,68 | 4,71 |
| line / panel2 | 3 | 3,36 | 3,28 | 3,85 |

Sonuç: istenen altı çiftin tamamı üç temada geçiyor. Fable yanıtındaki tek eksik (line / panel2 3:1 altı) bu kararda kapandı. En dar pay: Cephe'de ac / panel2 = 4,95; mercan renkli küçük yazı panel2 üstünde kullanılabilir ama rengi açılmamalı.

Ekip önerilerinde bulunan ve bu yüzden olduğu gibi alınmayan değerler: Kozalar `--line: transparent` (sınır yok), Kozalar beyaz üstü #FF6B6B vurgu ve #34D399 / #FBBF24 durum renkleri (açık zeminde 4,5 altı), Prizma #343438 ve Kütle #4E496A çizgileri, GPT'nin açık temalardaki #BAC8BF ve #A8B9D0 çizgileri (hepsi 3:1 altı). Gemini'nin "tamamı 4,5:1 testinden geçti" cümlesi hesapla doğrulanmadı.

## 5. Fark tablosu (en az 4 eksen şartı)

| Eksen | Vitrin | Vuruş | Cephe |
|---|---|---|---|
| Zemin | açık krem | koyu patlıcan | petrol mavisi |
| Vurgu | bordo | ıhlamur yeşili | mercan |
| Yazı | Plex Serif + Segoe UI Variable | Bahnschrift + Plex Mono | Space Grotesk |
| Köşe | 18 px, hap | 2 px | 10 px |
| Yüzey | gölgeli, ayrık adalar | bitişik düz bloklar, 1 px çizgi | ton katmanı, 2 px çerçeve, düz alt blok |
| Gezinme | alt yüzen çubuk, panel solda | üst sekmeler, panel sağda | sol etiketli ray, simetrik |
| İmza hareket | perde (yatay açılma) | vuruş (çizgi süpürmesi) | yükseliş (dikey) |

Yedi eksenin yedisinde de üç tema birbirinden farklı. Bir açık (Vitrin), iki koyu (Vuruş, Cephe).

Yasak ad denetimi: Vitrin, Vuruş, Cephe brifteki 15 Telefon Köprüsü adıyla ve üç eski tema adıyla çakışmıyor. Amber, neon, gradyan yok. Vuruş'un köşeli ve çizgili yanı yasak listedeki "Çizgi" ve "Keskin" ile ana fikir olarak karışmasın diye temanın kimliği köşe değil, zaman/ritim ve patlıcan-ıhlamur rengi olarak tanımlandı; uygulayan ekip o iki temanın görünümüyle yan yana bir kez karşılaştırmalı.

## 6. Varsayılan tema: Vitrin

Gerekçe: kullanıcı teknik olmayan mağaza çalışanı ve program gündüz, aydınlık mağazada kullanılıyor; açık zemin en tanıdık olanı. Alt çubuk ve tek hap düğme en az öğrenme gerektiren iskelet. Üç ekip yanıtında da açık tema en sakin seçenek olarak sunulmuş. Koyu isteyen Vuruş'a, renkli isteyen Cephe'ye geçer.

## 7. Açık kalanlar (karar dışı, uygulamada sınanacak)

1. Space Grotesk ve IBM Plex Serif'in gömülecek dosyalarında ğ ş ı İ ç ö ü sınanmadı.
2. "Şaşırt beni" düğmesi Fable yanıtında geçiyor, brifte yok; mevcut programda varsa korunur. `web/index.html` bu karar oturumunda açılmadı.
3. Vitrin'de 1366x768 alt satırına kip çubuğu ve Videoyu yap hapı birlikte sığıyor mu, ilk prototipte ölçülmeli; sığmazsa ikincil eylemler "…" menüsünde kalır, kip etiketleri kısaltılmaz.
4. Vuruş'ta üst sekme satırı sahneden yaklaşık 48 px yükseklik alır (çerçeve 1366x768'de yaklaşık 250x470 px'e iner); kabul edildi, ama 470 px altına inmemeli.
5. Kontrast tarayıcıda ve gerçek ekranda doğrulanmadı.

## 8. Aday notlar

- aday not: Vidyo Yapıcı bağlayıcı yerleşimi: sahne ortada sabit, 5 kip (Yaz, Hazır, Yapay zekâ, Görünüm, Ayrıntılar; ad, sıra, simge temalar arasında aynı), altta sabit eylem çubuğu, tek dolu düğme Videoyu yap. (kaynak: KARAR-FABLE.md, 2026-11-17)
- aday not: Vidyo Yapıcı'nın kesin üç teması Vitrin (varsayılan, krem/bordo, alt çubuk), Vuruş (patlıcan/ıhlamur, üst sekmeler), Cephe (petrol/mercan, sol etiketli ray); jetonlar ve kontrast tablosu KARAR-FABLE.md'de. (kaynak: aynı)
- aday not: Reji, Prizma, İkiz, Mercek, Kozalar, Kütle adları Vidyo Yapıcı'da kullanılmadı; paketin başka programlarında ad olarak boşta. (kaynak: aynı)
- aday not: Arayüzde neon renk, ışıma, bulanıklık, yaylanma yasak; Gemini'nin #D9F854 limonu #CBDA82'ye yumuşatılarak Vuruş vurgusu oldu, #00E5FF kullanılmadı. (kaynak: aynı)

[Claude Fable 5.2, TAA Video Yapıcı arayüz başlığı, 2026-11-17]
