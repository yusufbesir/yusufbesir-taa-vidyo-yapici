---
description: "KARAR-OYN: TAA Vidyo Yapıcı 1.0 katmanlı oynatıcı (Resolume Arena yerine kullanılacak ekran oynatıcısı) mimari kararı. Oynatıcı ajanı ve UI ajanı için bağlayıcı sözleşme."
sahip: Yusuf
yazan: "[Claude#Fable-5.2 karar ajanı, yenileme-1.0, 2026-11-25]"
degistirme-kurali: "Yalnız yazan yapay zekâ ya da Yusuf'un izniyle değişir; güncelleme üstü çizilerek yapılır; silme yok."
dogrulama-durumu: "Kod okunarak yazıldı (src/arayuz.rs, src/ffmpeg.rs, src/uret.rs, src/ayar.rs, src/tarif.rs, web/index.html, wry 0.57.2 ve tao 0.37.2 kaynakları). Hiçbir şey derlenmedi ve çalıştırılmadı; 'doğrulanmadı' yazan maddeler sahada denenmeli. Son söz Yusuf'un."
---

# KARAR-OYN: Katmanlı oynatıcı mimarisi

## Özet
- Oynatıcı programın içine gömülür: iki yeni sayfa, `/oynatici` (kontrol) ve `/cikis` (ekrana giden çıkış). Ayrı program, ayrı süreç, yeni bağımlılık yok.
- Çıkış, Rust tarafında açılan ikinci bir tao penceresidir (kenarlıksız, en üstte, seçilen monitörde, 1:1 piksel). Tarayıcı penceresi kullanılmaz.
- Oynatma motoru ÇIKIŞ sayfasında çalışır; kontrol sayfası yalnız kumandadır. Kontrol penceresi küçültülse ya da donsa bile ekran akmaya devam eder.
- Tek doğruluk kaynağı sunucudur (kalıcı proje + komut kuyruğu, uzun yoklama). BroadcastChannel yalnız hızlandırıcıdır; tasarım ona bağımlı değildir.
- Birleştirme (compositor) 1.0'da DOM ile yapılır: üst üste `<video>`/`<img>` + CSS `opacity` + `mix-blend-mode`. Canvas/WebGL birleştirici sonraya kalır.
- "Canlı sahne" 1.0'da gerçek zamanlı değildir: senaryo mevcut motorla mp4'e pişirilir ve klip olarak eklenir.
- Araştırmanın uyarısı geçerliliğini korur (arastirma/RESOLUME-ARASTIRMASI-2026-11-16.md: "yerine geçen oynatıcı önerilmez"). Yusuf yine de istedi (2026-11-24); karar buna göre verildi, ama 1.0 kapsamı dar tutuldu ve 5. bölümdeki saha kabul ölçütü geçilmeden Arena'nın yerine konmamalı.

## 0. Koddan çıkan kısıtlar (kararı belirleyenler)
| Bulgu | Kaynak | Sonuç |
|---|---|---|
| Sunucu her bağlantıda tek istek işler (`Connection: close`), gövdeyi belleğe alır, Range yok, istek gövdesi en çok 1 MB | `src/arayuz.rs` `oku`, `yanit` | Medya için ayrı, akışlı (dosyadan parça parça) ve Range destekli yanıt yazılmalı. Dosya yükleme (upload) 1.0'da yok. |
| Yetki: `Host` denetimi + `taa-oturum` çerezi (HttpOnly, SameSite=Strict); çerez yalnız `GET /?t=<jeton>` ile verilir; POST'ta `X-LED: 1` şart | `isle` | Yeni sayfalar ve medya aynı kapıdan geçer; yeni yetki yolu AÇILMAZ. Çıkış penceresi ana pencereyle aynı `WebContext`'i kullanır, çerez hazırdır. |
| `pencere()` tek pencere açar ve HERHANGİ bir `CloseRequested` olayında `process::exit(0)` yapar | `pencere` | İkinci pencere için olay döngüsü değişmeli (pencere kimliğine bakmalı). Bu dosya UI ajanınındır: ISTEK gerekir (2.2). |
| wry 0.57.2: `autoplay` varsayılanı `true`; Windows'ta bu `--autoplay-policy=no-user-gesture-required` argümanına dönüşür | wry `lib.rs:881`, `webview2/mod.rs:315` | Çıkış penceresinde sesli video kullanıcı tıklaması olmadan başlar. İki görünüm aynı veri klasörünü paylaştığı için tarayıcı argümanları AYNI kalmalı: hiçbirine `with_additional_browser_args` eklenmez. |
| wry'de Windows için arka plan kısıtlamasını kapatma yok (`with_background_throttling` desteklenmiyor) | wry `lib.rs:1490` | Motor, görünür olan çıkış sayfasında çalışmalı; kontrol sayfasındaki zamanlayıcılara güvenilmez. |
| serde yok; genel JSON çözücü var: `crate::tarif::json_coz` (`Json` enum) | `src/tarif.rs:20,116` | Proje JSON'u bununla çözülür ve doğrulanır. Yeni bağımlılık eklenmez (internetten indirme yasağı). |
| Program mp4'ü: H.264, yuv420p, `-g 1` (her kare anahtar kare), crf 12 | `src/ffmpeg.rs` | Sarma ve döngü için ideal; bit hızı yüksek. HAP `.mov` WebView2'de ÇÖZÜLMEZ: listede "oynatılamaz" diye gösterilir. |
| Çıktı klasörü: `cikti_klasoru()` (`%USERPROFILE%\Videos\TAA Vidyo Yapıcı`, `LED_CIKTI` ile değişir) | `src/arayuz.rs:94` | Tek medya kökü budur. |
| `uret::video(metin, cikis, kodek, ilerleme, iptal)` ve `uret::onizleme(metin, t)` hazır | `src/uret.rs:55,69` | Senaryo pişirme ve klip küçük resmi bunlarla yapılır. |

## 1. En küçük işe yarar özellik kümesi (1.0)

### 1.1 Kompozisyon
- İki kip: `ikili` = 1900x1800 (iki 950x1800 ekran yan yana) ve `tek` = 950x1800. Ölçüler projede sayı olarak tutulur (`ekranGen` 950, `ekranYuk` 1800, `ekranSayisi` 1 ya da 2); 980 genişlik sorusu çözülene kadar sabit yazılmaz.
- Çıkış 1:1 pikseldir (ölçekleme yok). Sahne kökü `transform: scale(1 / devicePixelRatio)` ile fiziksel piksele kilitlenir; böylece Windows ölçeği %125 olsa bile 950 piksel 950 LED pikseline düşer.
- Katman hedefi (katman başına): `sol`, `sag`, `ikisi` (aynı klip iki ekranda, eşlenik), `yayili` (klip 1900 genişliğe yayılır). `tek` kipte hedef yok sayılır.
- Sığdırma (katman başına): `doldur` (kırparak, varsayılan), `sigdir` (boşluklu), `esnet`. Zemin her zaman siyah.

### 1.2 Katman
- En çok 8 katman. Aynı anda oynayan video katmanı 4'ü geçince kontrol sayfası uyarır (sert sınır değil, ölçüm sonrası güncellenir).
- Alanlar: `ad`, `acik` (kapalıysa çizilmez ve video duraklatılır), `solo`, `opaklik` 0-1, `karisim` (`normal` | `ekle` | `carp`), `hedef`, `sigdir`, `kip`, `bitince`, `hiz`, `ses`, `sessiz`, `gecisMs`, `klipler[]`, `etkin` (etkin klip sırası), `oynuyor`.
- Karışım eşlemesi: `normal` → `mix-blend-mode: normal`; `ekle` → `plus-lighter` (toplamalı; yoksa `screen`'e düşer ve kontrol sayfası bunu yazar); `carp` → `multiply`.
- Solo: herhangi bir katmanda solo açıksa yalnız solo katmanlar çizilir.
- İşlemler: ekle, sil (yalnız listeden; dosyaya dokunulmaz), yukarı/aşağı sırala, ad değiştir, çoğalt. Katman 1 en alttadır.

### 1.3 Klip ve oynatma listesi
- Katman başına en çok 200 klip. Klip: `{tur: 'video'|'resim', ad, sure}`; `sure` yalnız resimde (varsayılan 8 sn, 1-3600).
- Kaynaklar: çıktı klasöründeki `.mp4`, `.webm`, `.png`, `.jpg`, `.jpeg`, `.webp`. `.mov` (HAP) listede görünür ama eklenemez, nedeni yazılır.
- "Canlı sahne" = pişirilen sahne: kontrol sayfasında senaryo metni (ya da ana sayfadaki güncel senaryo) "Sahneyi klip yap" ile `uret::video` üzerinden `oynatici-sahne-<zaman>.mp4` olarak çıktı klasörüne üretilir, bitince listeye eklenir. İlerleme gösterilir, iptal edilir. Ana sayfadaki `/uret` çalışıyorsa 409 döner (iki ağır üretim aynı anda yok). Üretim sırasında oynatma takılabilir: kontrol sayfası uyarır.
- Oynatma kipi (`kip`): `dongu` (etkin klip tekrar eder), `sirali` (liste sırayla), `rastgele` (torba: liste bitmeden aynı klip gelmez).
- Liste bitince (`bitince`, yalnız `sirali` ve `rastgele`): `basa` (son klipten sonra ilk klibe döner; varsayılan, Yusuf'un "son videodan başa alma" isteği), `dur` (son karede kalır), `kapat` (katman saydamlaşır).
- Taşıma: oynat/duraklat, sonraki, önceki, başa al (listenin ilk klibi), klibi tetikle (listeden tıklayarak), klip içinde sar (kaydırma çubuğu), 5 sn ileri/geri.
- "Geri" = önceki klip demektir. Tersine oynatma `<video>` ile yapılamaz: sonraya.
- Hız: 0,25 / 0,5 / 0,75 / 1 / 1,25 / 1,5 / 2 (`playbackRate`). Resimde süre hıza bölünür.
- Geçiş: `gecisMs` 0 (kes) ile 2000 arası çapraz solma; yalnız `opacity`. Katman başına iki ortam öğesi (A/B) tutulur, sıradaki klip B'ye önceden yüklenir, geçiş anında yer değiştirirler. Tek klipli `dongu` için `video.loop = true`.
- Ses: katman `ses` 0-1 ve `sessiz`; ana ses `anaSes` 0-1. Varsayılan: bütün katmanlar sessiz (cephe ekranı). Ses çıkış penceresinden çıkar.
- Genel: "Karart" (bütün çıkış siyah, oynatma sürer), "Hepsini duraklat/oynat".

### 1.4 Kalıcılık
- Proje dosyası: `%LOCALAPPDATA%\TAA Vidyo Yapici\oynatici.json` (`TAA_OYNATICI_DOSYASI` ile değişir). `ayar.json` kullanılmaz (değer sınırı 16 KB).
- Yazma atomik: geçici dosya + yeniden adlandırma (`ayar.rs` kalıbı). Bozuk ya da 256 KB'yi aşan dosya = boş varsayılan proje; bozuk dosyaya dokunulmaz, yanına `.bozuk-<zaman>` adıyla kopyalanır (silme yok).
- Kalıcı olan: bütün proje alanları, katman başına `etkin` ve `oynuyor`, çıkış penceresi ayarı (`monitor`, `x`, `y`, `kip`), `acilistaCikisAc`. Klip içi konum kalıcı DEĞİL (açılışta klip baştan başlar).
- Dosyası kaybolmuş klip listede "bulunamadı" işaretiyle kalır, oynatmada atlanır; kendiliğinden silinmez.

## 2. Mimari

### 2.1 Dosyalar (Oynatıcı ajanı sahibidir)
| Dosya | İçerik |
|---|---|
| `src/oynatici.rs` | HTTP uçları, proje doğrulama, kalıcılık, komut kuyruğu, uzun yoklama |
| `src/oynatici_medya.rs` | Medya listesi, güvenli ad çözme, Range ayrıştırma, akışlı yanıt |
| `src/oynatici_pencere.rs` | (yalnız Windows) çıkış penceresi açma/kapama, monitör listesi; olay döngüsüne köprü |
| `web/oynatici.html`, `web/oynatici.js`, `web/oynatici.css` | Kontrol sayfası |
| `web/cikis.html`, `web/oynatici-cikis.js` | Çıkış sayfası |
| `web/oynatici-motor.js` | DOM'suz saf mantık (sıradaki klip, torba, bitince, doğrulama); hem çıkış sayfası hem node testi kullanır |
| `tests/oynatici.rs`, `tests/oynatici_test.js`, `docs/OYNATICI.md` | Testler ve belge |
`src/lib.rs`'e yalnız üç `pub mod` satırı eklenir. `Cargo.toml` değişmez.

### 2.2 UI ajanından istenenler (`tasarim/yenileme/ISTEK-UI.md`'ye yazılacak; `src/arayuz.rs` ve `web/index.html` onundur)
1. Yönlendirme, `isle` içinde yetki denetiminden SONRA, `match`'ten önce: `if yolu == "/cikis" || yolu.starts_with("/oynatici") { return crate::oynatici::isle(&mut s, &yontem, &yol, &bas, &govde, &o_bilgi); }`. Oynatıcı yanıtı akışa kendisi yazar (Range ve ek başlıklar için `yanit()` yetmez). `o_bilgi`: üretim çalışıyor mu bilgisini okuyan küçük bir kapanış ya da `&AtomicBool`.
2. Olay döngüsü: `EventLoopBuilder::<crate::oynatici_pencere::Olay>::with_user_event()`; `create_proxy()` sonucu `crate::oynatici_pencere::kopru_kur(proxy)` ile modüle verilir; döngüde `Event::UserEvent(o)` gelince `crate::oynatici_pencere::isle(o, hedef, &mut baglam)` çağrılır. `baglam` (WebContext) döngü kapanışına taşınmalı.
3. `CloseRequested` pencere kimliğine bakmalı: çıkış penceresi kapanırsa yalnız o kapanır; ana pencere kapanırsa bugünkü gibi süreç biter. Öneri (P1): çıkış açıkken ana pencere kapatılmak istenirse süreç bitmez, ana pencere simge durumuna küçülür ve sayfaya "Çıkış açık. Önce çıkışı kapatın ya da Programı kapat'ı kullanın." bildirimi gider. Karar Yusuf'a sorulacak (aşağıda açık soru 3).
4. `web/index.html`: menüye "Oynatıcı" girişi (ana pencerede `/oynatici` adresine gider; geri dönüş bağlantısı kontrol sayfasında).
Bu dört madde gelene kadar Oynatıcı ajanı uçlarını `--tarayici-yok` hizmetinde ve node testleriyle geliştirir; pencere kısmı "bağlanmadı" diye raporlanır.

### 2.3 Çıkış penceresi: Rust'ta ikinci tao penceresi (karar)
| Seçenek | Değerlendirme |
|---|---|
| İkinci tao penceresi + wry görünümü (SEÇİLDİ) | Monitör, konum, kenarlık, en üstte kalma tam denetimde (`available_monitors`, `with_position`, `with_decorations(false)`, `with_always_on_top(true)`, `with_skip_taskbar`, `set_cursor_visible(false)`; hepsi tao 0.37.2 kaynağında var). Aynı `WebContext` = aynı çerez, aynı otomatik oynatma politikası. |
| `window.open` / varsayılan tarayıcı | Reddedildi. Program kuralı "varsayılan tarayıcı hiç açılmaz"; tam ekran için kullanıcı hareketi gerekir; monitör seçilemez; jetonun adres çubuğuna ve geçmişe düşmesi gerekirdi. |
- Pencere `with_fullscreen` ile DEĞİL, kenarlıksız pencere olarak açılır: boyutu kompozisyon kadar (1900x1800 ya da 950x1800 fiziksel piksel), konumu seçilen monitörün sol üstü + `x`,`y` kaydırması. Gerekçe: LED işlemcileri genelde masaüstünün belli bir bölgesini alır; özel tam ekran pencere odak kaybında küçülür ve 1:1 pikseli bozabilir.
- Açma/kapama yalnız sunucu ucundan olur (`POST /oynatici/cikis`); çıkış sayfasında Esc pencereyi KAPATMAZ (yanlışlıkla kapanmasın). Acil kapatma: çıkış penceresinde Ctrl+Shift+Q.
- Tek çıkış penceresi. Açıkken yeniden "aç" gelirse yeni ayarla taşınır.
- Ekran uykusu: çıkış sayfası `navigator.wakeLock.request('screen')` ister (127.0.0.1 güvenli bağlam sayılır); alınamazsa kontrol sayfasında uyarı çıkar. Doğrulanmadı; alınamıyorsa Windows güç ayarı elle yapılır (belgeye yazılır).
- Çökme koruması: çıkış sayfası her saniye canlı durum bildirir; 5 sn bildirim gelmezse kontrol sayfası "Çıkış yanıt vermiyor" der ve "Çıkışı yeniden aç" düğmesi sunar. Otomatik yeniden açma sonraya.

### 2.4 İletişim: sunucu yetkili, BroadcastChannel hızlandırıcı
- İki tür veri vardır: (a) proje (kalıcı durum, `surum` sayacı), (b) komut (anlık: sonraki, önceki, başa al, tetikle, sar, karart; artan `no`).
- Çıkış sayfası `GET /oynatici/olay?surum=<s>&komut=<n>` ile uzun yoklama yapar: yeni proje ya da komut varsa hemen, yoksa en geç 20 sn'de boş döner; hemen yeniden bağlanır. Sunucu bağlantı başına iş parçacığı açtığı için bekleyen istek başkasını tıkamaz. Yerelde gecikme birkaç milisaniyedir.
- Kontrol sayfası değişikliği sunucuya yazar; sunucu sürümü artırır ve bekleyen yoklamaları uyandırır (`Condvar`).
- BroadcastChannel (`taa-oynatici`) yalnız iki yüksek hızlı iş için: opaklık/ses sürgüsü sürüklenirken ara değerler (kontrol → çıkış) ve konum/kare bilgisi (çıkış → kontrol). Sürgü bırakılınca son değer her zaman sunucuya yazılır. Kanal çalışmazsa sürgü 10 Hz'e kısılmış sunucu yazımıyla, konum bilgisi `/oynatici/canli` ile çalışır. İki WebView2 görünümünün aynı veri klasöründe BroadcastChannel paylaştığı bu makinede doğrulanmadı; tasarım bu yüzden kanala bağımlı değil.
- Çakışma: `POST /oynatici/proje` gövdesinde `temelSurum` gönderilir; sunucudaki sürümle uyuşmazsa 409 + güncel proje döner, kontrol sayfası yeniden uygular. (Tek kontrol sayfası olağan durumdur; bu yalnız emniyettir.)

### 2.5 Uçlar
| Yöntem ve yol | İş |
|---|---|
| `GET /oynatici` , `GET /cikis` | Sayfalar (gömülü). `GET /oynatici/dosya/<ad>`: gömülü js/css (sabit beyaz liste). |
| `GET /oynatici/proje` | `{surum, proje}` |
| `POST /oynatici/proje` | Tam proje + `temelSurum`; doğrular, yazar, sürümü artırır. 400 / 409. |
| `POST /oynatici/komut` | `{tur, katman?, deger?}`; `tur` beyaz listeden. Kuyruk son 64 komutu tutar. |
| `GET /oynatici/olay?surum=&komut=` | Uzun yoklama (yalnız çıkış sayfası kullanır). |
| `POST /oynatici/canli` , `GET /oynatici/canli` | Çıkışın bildirdiği anlık durum (katman başına klip, konum, düşen kare, hata). Bellekte, kalıcı değil. |
| `GET /oynatici/medyalar` | Çıktı klasörü listesi: ad, tür, boyut, değişme zamanı, oynatılabilir mi. En çok 2000 kayıt. |
| `GET` ve `HEAD /oynatici/medya/<ad>` | Dosya, Range destekli. |
| `GET /oynatici/kucuk/<ad>` | Resimler için dosyanın kendisi; video küçük resmi 1.0'da çıkış/kontrol sayfasında `<video>` ilk karesinden alınır (sunucuda ffmpeg çağrısı yok). |
| `POST /oynatici/sahne` , `GET /oynatici/sahne` , `POST /oynatici/sahne-iptal` | Senaryoyu klibe pişirme, ilerleme, iptal. |
| `GET /oynatici/monitorler` , `POST /oynatici/cikis` | Monitör listesi (ad, konum, boyut, ölçek); çıkışı aç/kapat/taşı `{ac, monitor, x, y}`. Pencere köprüsü yoksa 503 "pencere yok". |

### 2.6 Medya sunumu ve yol geçişi koruması
- Ad çözme sırası (hepsi geçmeli, yoksa 404; ayrıntı sızdırılmaz):
  1. Yoldan `/oynatici/medya/` sonrası alınır, sorgu atılır, `url_coz` BİR kez uygulanır.
  2. Ad boş değil, en çok 200 karakter; `/`, `\`, `:`, NUL ve denetim karakteri içermez; `.` ile başlamaz; `..` içermez; boşluk ya da nokta ile bitmez (Windows adı kırpar).
  3. Uzantı beyaz listede: mp4, webm, png, jpg, jpeg, webp (küçük harfe çevrilerek).
  4. `kok = canonicalize(cikti_klasoru())`, `tam = canonicalize(kok.join(ad))`; `tam.parent() == kok` ve `tam.is_file()` olmalı. Böylece bağlantı (symlink/junction) ile kök dışına çıkış da kapanır. Alt klasör yok.
- Range: yalnız tek aralık. `bytes=a-b`, `bytes=a-`, `bytes=-n` desteklenir → 206 + `Content-Range` + `Accept-Ranges: bytes`. Geçersiz ya da dosya sonunu aşan başlangıç → 416 + `Content-Range: bytes */<boyut>`. Çoklu aralık → başlık yok sayılır, 200 tam dosya. Range yoksa 200 + `Accept-Ranges: bytes`.
- Gövde dosyadan 64 KB parçalarla yazılır; dosya belleğe alınmaz. Yazma hatası (istemci bağlantıyı kesti) sessizce biter. Yazma zaman aşımı 30 sn.
- Başlıklar: doğru `Content-Type`, `Content-Length`, `X-Content-Type-Options: nosniff`, `Cache-Control: no-store`, `Connection: close`. Chromium her aralık için yeni bağlantı açar; yerelde sorun değildir.
- Eşzamanlı medya bağlantısı sınırı: 32 (aşılırsa 503); kaçak bağlantı birikmesine karşı.

### 2.7 Performans ve birleştirici
- 1.0 kararı DOM birleştirici: her katman mutlak konumlu bir kutu; içinde A/B ortam öğeleri. Chromium videoyu donanımda çözer ve GPU'da birleştirir; kare kopyası yoktur. İki 950x1800 video + 1-2 üst katman için en düşük riskli yol budur.
- Canvas 2D reddedildi (her karede `drawImage` kopyası, 1900x1800'de gereksiz yük). WebGL birleştirici sonraya: gerekçesi ancak klip başına efekt, maske ya da özel geçiş istenirse doğar.
- `ikisi` (eşlenik) hedefi: aynı kaynak için iki `<video>` açılır ve ikinci çözme yapılır (basit ve sağlam). İkisi aynı anda başlatılır; kayma 1 kareyi geçerse sağdaki soldakine eşitlenir (`currentTime`), denetim 2 sn'de bir. Ölçümde yük fazla çıkarsa ikinci kopya `requestVideoFrameCallback` + küçük canvas aynasına çevrilir (sonraya, ölçüme bağlı).
- Yalnız `transform` ve `opacity` canlandırılır. Kapalı ya da opaklığı 0 olan katmanın videosu duraklatılır (çözme yükü kalkar).
- Önceden yükleme: sıradaki klip B öğesine `preload="auto"` ile, etkin klibin bitimine 3 sn kala yüklenir; daha erken yüklenmez (bellek).
- Ölçüm: çıkış sayfası `getVideoPlaybackQuality()` ile düşen kareyi katman başına sayar, `/oynatici/canli` ile bildirir; kontrol sayfası gösterir.
- Ortam hatası (`error`, `stalled` > 5 sn): klip atlanır, olay canlı duruma yazılır, katman sıradaki klibe geçer; liste tamamen bozuksa katman saydamlaşır. Ekran asla hata iletisi göstermez.
- Kontrol sayfası önizlemesi: gerçek videoları İKİNCİ kez çözmez; her katmanın etkin klibinin küçük resmi + ad + konum çubuğu gösterilir. Canlı çıkış önizlemesi sonraya.
- Beklenti (doğrulanmadı): tümleşik GPU'lu sıradan bir bilgisayarda 2 x 950x1800 @25 fps H.264 + 2 resim katmanı sorunsuz. `-g 1`, crf 12 dosyalar yüksek bit hızlıdır; yavaş diskte takılma olursa belge "oynatma için ayrı, daha düşük bit hızlı dışa aktarma" önerir (üretim ayarı Malzeme/UI alanı: sonraya).

### 2.8 Güvenlik
- Mevcut kapılar aynen: yalnız 127.0.0.1, `Host` denetimi, oturum çerezi, POST'ta `X-LED: 1`. `/cikis` ve `/oynatici` için jetonlu yeni adres YOK.
- Oynatıcı sayfalarında `Content-Security-Policy: default-src 'none'; script-src 'self'; style-src 'self'; img-src 'self' blob:; media-src 'self'; connect-src 'self'; base-uri 'none'; form-action 'none'; frame-ancestors 'none'`. Satır içi betik ve stil yok (ayrı dosyalar).
- Dosya adları ve kullanıcı metni arayüze yalnız `textContent` ile basılır; `innerHTML` yok.
- Proje doğrulama sunucuda (`crate::tarif::json_coz` sonrası): gövde ≤ 256 KB, katman ≤ 8, klip ≤ 200, metin alanları ≤ 200 karakter, sayılar aralığa kırpılır, bilinmeyen alan atılır, `tur`/`kip`/`karisim`/`hedef` beyaz listeden. Klip `ad`'ı 2.6'daki ad kuralından geçer; geçmeyen klip reddedilir (400).
- Çıkış penceresi yalnız kendi `/cikis` adresini açar; köprü olayı adres taşımaz (yalnız `Ac{monitor,x,y,gen,yuk}` / `Kapat`). Gezinme kilidi: çıkış görünümünde `with_navigation_handler` ile yalnız aynı köken kabul edilir.
- Sahne pişirme çıktı adı sunucuda üretilir (`oynatici-sahne-<YYYYAAGG-SSDDSS>.mp4`); kullanıcı adı alınmaz. Dosya silme ucu YOK (silme kararı Yusuf'un; klasörü aç düğmesi yeter).
- Günlük: oynatıcı olayları (aç/kapa, hata, atlanan klip) `%LOCALAPPDATA%\TAA Vidyo Yapici\oynatici-gunluk.txt` dosyasına eklenir; 1 MB'yi geçince yeni dosyaya geçilir, eskisi silinmez. Dosya adı dışında kişisel veri yazılmaz.

## 3. Kısayollar (kontrol sayfası odaktayken; `event.code` ile, Türkçe Q ve F klavyede aynı tuş yerinde; yazı alanında yazarken kapalı)
| Tuş | İş |
|---|---|
| Boşluk | Seçili katmanı oynat/duraklat |
| Shift+Boşluk | Hepsini oynat/duraklat |
| Sağ ok / Sol ok | Sonraki / önceki klip |
| Shift+Sağ / Shift+Sol | 5 sn ileri / geri sar |
| Home | Listeyi başa al |
| Yukarı / Aşağı ok | Opaklık +5 / -5 |
| 1-8 | Katman seç |
| Shift+1-8 | Katmanı aç/kapat |
| KeyA | Seçili katmanı aç/kapat |
| KeyS | Solo |
| KeyM | Sessiz |
| KeyB | Karart (aç/kapat) |
| KeyN | Yeni katman |
| Ctrl+Yukarı / Ctrl+Aşağı | Katmanı yukarı/aşağı taşı |
| Delete | Seçili klibi listeden çıkar (dosya silinmez; 5 sn "Geri al") |
| KeyO | Çıkış penceresini aç/kapat |
| ? | Kısayol listesi |
Çıkış penceresinde yalnız Ctrl+Shift+Q (acil kapat) çalışır; fare imleci gizlidir. Kısayollar ekranda da düğme olarak bulunur (dokunma hedefi ≥48 px, ana eylem 56 px); kısayol tek yol değildir.

## 4. Test planı
### 4.1 Otomatik (Rust, `tests/oynatici.rs` + modül içi; `kilitli-cargo.sh test --release oynatici`)
- Ad çözme tablosu: `..\x.mp4`, `a/b.mp4`, `%2e%2e%2fx.mp4`, `%252e%252e`, `C:x.mp4`, `x.mp4.`, `x.mp4 `, `.gizli.mp4`, `x.exe`, `x.MP4` (kabul), Türkçe ad `Kış Kampanyası.mp4` (kabul), 201 karakter, NUL, kök dışına bağlantı (oluşturulabiliyorsa).
- Range: `bytes=0-`, `0-0`, `-500`, `500-`, `a-b` (b ≥ boyut → kırpılır), başlangıç ≥ boyut → 416, `bytes=5-2` → 416, çoklu → 200, bozuk başlık → 200; 206 gövde uzunluğu ve `Content-Range` doğru; 5 MB'lik geçici dosyada parça parça okuma tam dosyayla aynı.
- Proje doğrulama: sınırlar (9 katman, 201 klip, 257 KB), bilinmeyen alan atılır, sayılar kırpılır, geçersiz `kip` → 400, kötü klip adı → 400.
- Kalıcılık: yaz-oku aynı; bozuk dosya → varsayılan + `.bozuk-` kopyası; atomik yazma yarıda kalırsa eski dosya sağlam.
- Sürüm ve komut: `temelSurum` uyuşmazlığı 409; komut `no` artar; kuyruk 64'te döner; uzun yoklama yeni komutta < 100 ms'de, boşta 20 sn'de döner (testte süre parametreyle kısaltılır).
- Yetki (gerçek soket, `--tarayici-yok` benzeri iç başlatma): çerezsiz 401, yanlış `Host` 403, `X-LED`'siz POST 403, `HEAD` gövdesiz.
- Gömülü sayfa sözleşmesi: `oynatici.html` ve `cikis.html` satır içi betik/stil içermez, yalnız izinli CSS değişkenleri, yasaklı CSS (gradient, blur, box-shadow ışıma, `steps(`) yok, `prefers-reduced-motion` var.
### 4.2 Otomatik (node, `tests/oynatici_test.js`; Rust testi çağırır, node yoksa atlanır)
- `oynatici-motor.js`: `dongu`/`sirali`/`rastgele` sıradaki klip; `bitince` üç durumu; torba tekrar etmez; bulunamayan klip atlanır; hepsi bozuksa katman kapanır; solo kuralı; `ikisi` hedefi iki kutu üretir; proje doğrulama sunucuyla aynı sınırlar.
### 4.3 Elle / tarayıcı otomasyonu (Playwright yerel, 1900x1800 görünüm)
- Çıktı klasörüne 3 kısa test mp4'ü + 1 png konur (test bitince Geri Dönüşüm Kutusu'na taşınması Yusuf'a bırakılır). İki katman, `sirali` + `basa`: son klipten sonra ilk klibe döndüğü, geçişte siyah kare olmadığı ekran görüntüsüyle doğrulanır.
- Karışım: beyaz üstüne `carp`, siyah üstüne `ekle` için piksel renk denetimi.
- Çıkış penceresi (yalnız gerçek pencerede): ikinci monitörde açılır, 1:1 piksel (1 piksellik dama deseni resmiyle), ana pencere küçültülünce oynatma sürer, çıkış penceresi kapanınca program kapanmaz.
- Dayanıklılık: 30 dk kesintisiz iki video; düşen kare oranı < %1, bellek artışı < 100 MB. Oynarken dosya klasörden taşınır: katman sıradaki klibe geçer, ekranda hata yazısı çıkmaz.
- Erişilebilirlik: klavyeyle tam kullanım, görünür odak, 48/56 px hedefler, kontrast ≥ 4,5:1, `prefers-reduced-motion`'da arayüz hareketsiz (çıkış içeriği etkilenmez).
### 4.4 Kabul ölçütü (Arena yerine kullanmadan önce, sahada)
Gerçek ekran bilgisayarında 24 saat kesintisiz, düşen kare < %1, siyah kare ve donma yok, program yeniden açıldığında son proje ve katman durumu geri gelir. Bu geçilmeden "Arena'nın yerini aldı" denmez.

## 5. Kapsam
### 1.0'da (bu tur)
1. Uçlar, proje doğrulama, kalıcılık, komut kuyruğu, uzun yoklama. (kolay-orta)
2. Medya listesi + Range'li akış + ad koruması. (orta; en riskli güvenlik parçası, önce bu ve testleri)
3. Çıkış sayfası + motor: katmanlar, A/B geçiş, kipler, bitince, hız, ses, karışım, hedef, karart. (orta-zor)
4. Kontrol sayfası: katman ızgarası, klip listesi, taşıma, sürgüler, kısayollar, medya seçici, canlı durum. (orta)
5. Çıkış penceresi köprüsü (UI ajanı ISTEK'i gelince). (orta; gelmezse "bağlanmadı" diye raporlanır, sayfa `/cikis` olarak ana pencerede denenebilir)
6. Sahne pişirme. (kolay; zaman kalmazsa ilk kesilecek madde)
Sıra: 2 → 1 → 3 → 4 → 5 → 6. Çalışan en küçük dilim: tek katman, sıralı liste, başa dönme, çıkış sayfası.

### Sonraya
- Gerçek zamanlı canlı sahne (senaryonun tarayıcıda canvas ile çizilmesi) ve canlı veri (saat, sayaç, hava).
- WebGL birleştirici, klip/katman efektleri, maske, özel geçişler, tersine oynatma, vuruşa (BPM) bağlı süre.
- Dilim/haritalama (Advanced Output karşılığı), ikiden fazla ekran, kenar düzeltme, renk/parlaklık kalibrasyonu.
- Saat bazlı takvim (sabah aç, gece karart, kampanya tarihi), Windows açılışında otomatik başlatma (`--oynatici` bayrağı; `src/main.rs` sahibi belirsiz), çökmede otomatik yeniden açma, uzaktan/telefonla kumanda, OSC/MIDI.
- HAP oynatma, dosya yükleme, alt klasörler, çıktı klasörü dışından medya, canlı çıkış önizlemesi, birden çok proje/deck, oynatma için ayrı dışa aktarma ön ayarı.

## Riskler
1. `src/arayuz.rs` değişikliği UI ajanına bağlı; gecikirse çıkış penceresi 1.0'a yetişmez. Azaltma: ISTEK hemen yazılır, köprü arayüzü yukarıda sabitlendi.
2. BroadcastChannel ve Wake Lock'ın gömülü WebView2'de davranışı doğrulanmadı; ikisi de zorunlu yol değil.
3. Performans rakamı ölçülmedi; hedef bilgisayar (GPU, disk) bilinmiyor.
4. 24/7 dayanıklılık artık bizim sorumluluğumuz (araştırmanın ana uyarısı). 1.0'da otomatik toparlanma yok.
5. Bağlantı başına iş parçacığı + uzun yoklama: en çok birkaç bekleyen bağlantı olur; sınır 2.6'da medya için kondu, yoklama için istemci başına tek bekleyen istek kuralı testle korunur.

## Yusuf'a açık sorular (karar bekleyenler)
1. Ekran bilgisayarı iki ekranı tek 1900x1800 yüzey olarak mı görüyor, iki ayrı çıkış mı? Genişlik 950 mi 980 mi? (Araştırmadaki 3. ve 4. sorular hâlâ yanıtsız.) Yanıt gelene kadar varsayım: tek yüzey, 950.
2. Cephede ses var mı? Yoksa ses denetimleri 1.0 arayüzünde "Ayrıntılar" altına gizlenir.
3. Çıkış açıkken ana pencere kapatılırsa: program kapansın mı (bugünkü davranış), küçülsün mü (öneri)?
4. Oynatıcı, video üreten bilgisayarda mı çalışacak, ekran bilgisayarında mı? İkincisiyse o bilgisayara kurulum ve WebView2 gerekir.

---

## Ek 1 (2026-11-25 23:00): karar ile uygulamanın karşılaştırması
Yazan: [Claude#Fable-5.2 karar ajanı, yenileme-1.0, 2026-11-25]. Yalnız ekleme; yukarıdaki metin değişmedi. Yöntem: çalışma ağacındaki kod OKUNDU (`src/arayuz.rs`, `src/oynatici_http.rs` imzaları, `src/oynatici_medya.rs` imzaları, `src/oynatici_pencere.rs` tamamı, `web/oynatici*.js` aramayla). Hiçbir şey derlenmedi, test çalıştırılmadı, pencere açılmadı. Aşağıdaki "uyuyor" sözü "kodda karşılığı görüldü" demektir, "çalıştığı doğrulandı" demek değildir.

### E1.1 Kararın beş maddesi geçerli; değişen karar yok
(1) özellik kümesi, (2) mimari, (3) kısayollar, (4) test planı, (5) kapsam yukarıdaki haliyle bağlayıcıdır. Bu ekte yalnız ad uyarlamaları, kabul edilen sapmalar ve kapanmamış açıklar var.

### E1.2 Kabul edilen sapmalar (karar bunlara göre okunur)
| Karar metni | Uygulama | Hüküm |
|---|---|---|
| 2.1: `src/oynatici.rs` | `src/oynatici_http.rs` (uçlar, doğrulama, kalıcılık, kuyruk) | Kabul. Ad farkı; içerik aynı sorumluluk. |
| 2.2: `crate::oynatici::isle(...)` | `crate::oynatici_http::bu_yol_mu` + `isle_akis(s, yontem, yol, bas, govde, &uretim)`; `arayuz.rs` içinde yetki ve `X-LED` denetiminden sonra, `match`'ten önce bağlı | Kabul. Kararın istediği sıra korunmuş. |
| 2.5: `GET /oynatici/dosya/<ad>` | `/oynatici.js`, `/oynatici.css` (ve `bu_yol_mu` ile `/oynatici.*`) | Kabul. Sabit beyaz liste koşulu aynen geçerli: `/oynatici.<bilinmeyen>` 404 dönmeli (teste eklenmeli). |
| 2.2 madde 2: köprü olayı yalnız `Ac`/`Kapat` | Üçüncü olay `Monitorler(Sender<String>)` | Kabul. Adres ya da yol taşımıyor; 2.8'deki "köprü adres taşımaz" kuralı bozulmadı. |
| 2.5: kullanıcı metinli JSON için `crate::tarif::json_coz` | Oynatıcıda ayrı `json_coz` (`oynatici_http.rs`) | Kabul (AI ajanının dosyasına bağımlılığı kaldırıyor). Koşul: 4.1'deki doğrulama testleri bu çözücü üzerinden koşmalı; iç içe derinlik sınırı olmalı (bakılmadı, doğrulanmadı). |

### E1.3 Kodda karşılığı görülenler
- Çıkış penceresi: kenarlıksız, en üstte, yeniden boyutlanmaz, fiziksel piksel boyut ve konum, imleç gizli, aynı `WebContext`, gezinme kilidi (`with_navigation_handler`, yalnız `http://127.0.0.1:<port>/` öneki), boyut 64-8192 arasına kırpılıyor.
- Olay döngüsü: `CloseRequested` pencere kimliğine bakıyor; çıkış kapanınca yalnız o kapanıyor.
- Çıkış sayfası: `1 / devicePixelRatio` ölçeği, `plus-lighter` yoksa `screen`, BroadcastChannel `try/catch` içinde (yokluğunda çalışır), Wake Lock isteği, Ctrl+Shift+Q acil kapatma, Esc kapatmıyor.
- Sınırlar: 8 katman, 200 klip, 256 KB proje, 200 karakter metin, 64 komut, 8 bekleyen yoklama.
- Test dosyaları var: `tests/oynatici.rs` (13 fonksiyon), `tests/oynatici_test.js`. Sonuçları bu ek için ÇALIŞTIRILMADI.

### E1.4 Kapanmamış açıklar (Oynatıcı ajanına; 1.0 çıkmadan)
| No | Açık | Önem | Sahip |
|---|---|---|---|
| A1 | 2.6: eşzamanlı medya bağlantısı sınırı (32, aşımda 503) kodda aramayla bulunamadı. Varsa belgede yeri yazılsın, yoksa eklensin. | Orta | Oynatıcı |
| A2 | 2.3: `with_skip_taskbar(true)` çıkış penceresinde yok. Görev çubuğunda ikinci simge yanlışlıkla kapatmaya davet eder. | Düşük | Oynatıcı |
| A3 | `docs/OYNATICI.md` yok (2.1'de sayıldı). Kurulum, monitör seçimi, 1:1 piksel, güç ayarı ve 4.4 kabul ölçütü yazılmalı. | Orta | Oynatıcı |
| A4 | `arayuz.rs` içinde `("GET", "/oynatici")`, `/oynatici.js`, `/oynatici.css`, `/cikis` ve `(m, y) if y.starts_with("/oynatici/")` rotaları artık ulaşılamaz (önce `bu_yol_mu` yakalıyor). Zararsız ama CSP başlığı OLMAYAN ikinci bir yol gibi duruyor; `bu_yol_mu` satırı bir gün kaldırılırsa sayfalar CSP'siz ve medya 501 ile sunulur. Öneri: ölü rotalar yorumla işaretlensin ya da kaldırılsın (silme değil, kod sadeleştirme; UI ajanının dosyası). | Düşük | UI |
| A5 | Çıkış açıkken ana pencere kapatılınca süreç bitiyor (ekran kararır). Açık soru 3 yanıtlanana kadar bu davranış kalır; kontrol sayfasında ve belgede açıkça yazılmalı. | Orta | Yusuf kararı |
| A6 | `acilistaCikisAc` alanı projede var; program açılışında çıkışı gerçekten açan kod görülmedi (bakılmadı, doğrulanmadı). Yoksa arayüzde bu seçenek gösterilmemeli. | Düşük | Oynatıcı |
| A7 | 4.3 elle testleri (ikinci monitör, 1:1 dama deseni, 30 dk dayanıklılık, karışım piksel denetimi) ve 4.4 saha kabulü yapılmadı. Bunlar geçmeden rapora "Arena yerine kullanılabilir" yazılmaz. | Yüksek | Koordinatör + Yusuf |

### E1.5 Güvenlik kararı (değişmedi, yeniden teyit)
Yeni yetki yolu açılmadı: oynatıcı yolları `Host` + oturum çerezi + POST'ta `X-LED: 1` kapısının arkasında. Medya kökü tek klasör, alt klasör yok, dosya silme ucu yok, dosya yükleme yok. Bu dört kural 1.0 boyunca gevşetilmez; gevşetme isteği gelirse karar yeniden Fable'a ve Yusuf'a gelir. Commit öncesi `guvenlik-denetcisi` ve farklı aileden inceleme (Sol ya da Gemini) zorunlu; özellikle `oynatici_medya.rs` `ad_gecerli`/`coz`/`aralik_coz` ve `oynatici_http.rs` `json_coz`/`dogrula`.
