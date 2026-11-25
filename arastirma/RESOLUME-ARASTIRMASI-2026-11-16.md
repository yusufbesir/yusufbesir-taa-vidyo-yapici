---
description: "Resolume Arena ile oynatılan bina cephesi LED ekran (2 dikey ekran, ~950x1800 px) için ayrı bir yan uygulama yapmanın mantıklı olup olmadığı araştırması: Arena yetenekleri, riskler, 4+ yan araç seçeneği, yol haritası, numaralı kaynaklar. Salt araştırma raporudur; kod veya ayar değiştirmez."
sahip: Yusuf
yazan: "[Claude#ebd691 alt ajan, 2026-11-16]"
tarih: 2026-11-16
degistirme-kurali: "Yalnız ekleme; üstü çizerek güncelle. Silme yok (Yusuf kararı). Başka yapay zekâ bu dosyayı değiştirmez, öneri olarak sunar."
dogrulama-durumu: "Web kaynakları 2026-11-16'da okundu; doğrulanamayanlar metinde 'doğrulanmadı' diye işaretli. Resolume gerçek kurulumu/ekranı bu bilgisayarda incelenmedi."
---

# Resolume ve LED ekran: yan uygulama araştırması (16.11.2026)

## Özet
- Resolume yerine geçen bir oynatıcı yapmak mantıklı değil. Arena zaten olgun bir medya sunucusu (haritalama, çoklu çıkış, REST/WebSocket/OSC, autopilot). Sıfırdan yazmak stabilite kazandırmaz, risk getirir.
- "Yan araç" yapmak mantıklı. En çok değer: (A) Resolume uyumlu video hazırlama (kodek, çözünürlük, dosya adı) ve (D) sağlık denetimi/izleme + otomatik yeniden başlatma. Bunlar Arena'nın bıraktığı boşlukları (24/7 gözetim, medya doğrulama) doldurur.
- Önce Yusuf'un bugünkü sorunlarını öğrenmek gerekir (bölüm 3 soru listesi). "Daha stabil" hedefinin asıl nedeni Arena değil de video biçimi (H.264) veya Windows ortamı olabilir.

---

## 1. Kısa cevap ve öneri

**Yeni uygulama yapmaya değer mi?** Evet, ama "yerine geçen" olarak hayır, "yan araç" olarak evet.

| Yaklaşım | Karar | Gerekçe |
|---|---|---|
| Resolume yerine geçen oynatıcı | Önerilmez | Arena'nın ekran haritalama, LED işlemci çıkışı, çoklu çıkış ve 7.x güncellemeleri tek başına büyük iş [15][16][17]. Ekran 24/7 çalışacağı için yeni oynatıcının çökme/ısı/GPU hataları tamamen bizim sorumluluğumuz olur. |
| Yan araç (hazırlayıcı + izleyici) | Önerilir | Arena'nın kontrol arayüzleri (REST/WebSocket/OSC) dışarıdan izleme ve yönetim için yeterli [4][5][7]. |
| Hiçbir şey yapmama | Seçenek | Yalnız bugünkü iş akışında sorun yoksa. Videolar zaten H.264 MP4 üretiyor; DXV3/HAP'a geçiş tek başına bir iyileştirme olabilir [12][13]. |

**Önerilen ilk adım:** Önce yan araç A'nın küçük bir sürümü: "Resolume için hazırla" modu (HAP veya DXV hedefli, doğru çözünürlük, toplu dışa aktarma). Aynı anda Yusuf'a bölüm 3'teki soruları sormak.

---

## 2. Arena'nın bizim işimizle ilgili yetenekleri

| Alan | Bulgu | Durum | Kaynak |
|---|---|---|---|
| Sürüm | Arena 7 (7.x çizgisi). 7.25 ve 7.26 Mart 2026 civarı çıkmış; 7.26'da MCP sunucuları ve REST API yenilikleri var. 7.28 başlığı bir blog listesinde görüldü; tarih ve içerik doğrulanmadı. En güncel sürüm resmi indirme sayfasından kontrol edilmeli. | Kısmen doğrulandı | [19][21] |
| Lisans/fiyat | Resmi mağazada yenileme: Arena 12 ay yenileme 219 euro, Avenue 79 euro; Avenue'den Arena'ya geçiş 500 euro. Yeni Arena 7 lisansının resmi güncel fiyatı bulunamadı. Bayi fiyatları değişken (tek bilgisayar için yaklaşık 1.744 dolar bir bayide). Lisans bir bilgisayar + yedek bilgisayar şeklinde tarif edilmiş (bayi açıklaması). | Resmi yeni fiyat: doğrulanmadı | [1][2] |
| Sistem gereksinimi | Önerilen Windows: Windows 11, i7 6 çekirdek 3,7 GHz, Nvidia RTX 4070, M2 SSD, 16 GB RAM. Minimum (7.21 sayfası): Windows 10, Nvidia veya AMD GPU, 8 GB RAM. | Doğrulandı (arama özeti) | [18] |
| Advanced Output | Arena'da tam özellik. Ekranlar (screens) çıkıştır; dilimler (slice) giriş seçiminde konumlanır, ölçeklenir, döndürülür. LED işlemciler için tek dilim ve "Output Transformation" aşamasında işlemcinin beklediği sinyale göre yeniden dilimleme anlatılıyor. Ekran ayarları kompozisyondan bağımsız preset olarak .xml kaydedilir. Kenar yumuşatma (edge blending) projektörler içindir; LED duvarda genelde gerekmez. Playback card çıkışı yalnız Arena'da. | Doğrulandı | [15][16][17] |
| Çoklu çıkış | Birden çok ekran/çıkışa kompozisyon parçaları yönlendirilebilir. "Lots of outputs" konusunda sayı limiti bilgisi yok. | Limit: doğrulanmadı | [15] |
| Yapı | Composition > Group > Layer > Clip, kolonlar (columns) ve decks. Composition; klip setlerini, efektleri, parametreleri ve kontrol kısayollarını tutar. | Doğrulandı (sözlük) | [28] |
| Codec | Resolume'un kendi anlatımı: DXV en hızlı seçenek, çünkü kareler GPU'da çözülüyor. Ses için sıkıştırılmamış PCM wav önerilir; ses ve video ayrı render edilir. DXV3'ün H.264'e göre avantajı forumda sık tekrarlanıyor: H.264 doğrusal oynatım için tasarlanmış, uzun dosyada takılma raporları var. HAP ile DXV performansı için kaynaklar çelişiyor. Resolume'un DXV3 yazısı: 4K'da yaklaşık %30 iyileşme, düşük çözünürlükte yeniden kodlama fazla fark yaratmaz; DXV3 HQ dosya boyutunu yaklaşık iki katına çıkarır. | Resmi sayfa DXV3/HAP/H.264'ü ayrıntılı vermiyor; DXV3 resmi belge sayfası: doğrulanmadı | [11][12][13][14] |
| Dosya biçimi | .avc composition dosyası XML; Resolume çalışanı programla üretilebileceğini söylüyor. Eleman adları (layer/clip) bu araştırmada doğrulanmadı; örnek dosya incelenmeli. Klip küçük resimleri ayrı XML dosyalarında (base64 PNG). | Kısmen | [23] |
| Dosya yönetimi | "Media Manager"/Alley (dönüştürme) ile ilgili resmi ayrıntı bu araştırmada alınamadı. | Doğrulanmadı | [11] |
| OSC | Preferences > OSC'de açılır, varsayılan giriş portu 7000. Her kontrolün sabit adresi var (örn. /composition/layers/1/video/opacity). Okuma için "?" ile sorgu, çıkış için OSC çıkış presetleri. | Doğrulandı | [7] |
| REST + WebSocket API | Preferences > Webserver'dan açılır, varsayılan port 8080, varsayılan dinleme adresi 0.0.0.0 (yalnız yerel için 127.0.0.1 yazılabilir). WebSocket aynı portta: ws://adres:port/api/v1; eylemler: subscribe, unsubscribe, set, get, reset, trigger. Sütun/katman/grup ekleme-silme, efekt yönetimi, klip küçük resmi alma, bileşim değişikliklerine abone olma. 7.26: çıkış görüntü yakalama (monitors, PNG/JPEG), parametre animasyonu. Swagger belgesi Preferences > Webserver'dan. | Doğrulandı | [4][5][6][19] |
| MIDI / DMX / klavye | Klavye, MIDI, OSC, DMX kısayol düzenleyicileri var. Art-Net/sACN girişinden bu sayfada söz edilmiyor; DMX Output makalesi ayrı. | Art-Net/sACN giriş ayrıntısı: doğrulanmadı | [8] |
| Eklentiler / paylaşım | FFGL eklentileri (.dll); ISF desteği 7.8'den beri (eski forum başlığı); NDI, Syphon, Spout harici akış kaynağı. Resolume Wire ayrı ürün (düğüm tabanlı görsel programlama), kendi REST API v2 ve MCP sunucusu var. | Doğrulandı (genel) | [22][29][19] |
| Zamanlama | Resmi belgede "Scheduler" (takvimli çalıştırma) bulunamadı. Autopilot var: ileri/geri/rastgele (Any/Other/Bag), süre (saniye, vuruş, klip sonu), döngü, klip ve kolon eylemleri, Master Layer, öncelik sırası. Katman başına "Trigger first clip on load". Saat bazlı takvim için dışarıdan kontrol (OSC/REST) gerekir. | Scheduler yok/doğrulanmadı | [9][10][24] |
| BPM / transport | Autopilot süreleri global BPM'ye bağlanabilir; klip transport sonuna kadar oynatma seçeneği var. Dashboard ayrıntısı bu araştırmada ayrıca doğrulanmadı. | Kısmen | [9] |
| MCP | 7.26 ile Arena/Avenue için yerel MCP sunucusu: bileşim kurma/düzenleme (dosya yükleme, efekt, katman, kolon, grup). Slice/screen, MIDI/DMX/OSC eşlemesi, preset, kayıt/render desteklenmiyor. Canlı performans için değil. "DXV olmayan video dosyalarını bul" örnek istemi resmi sayfada geçiyor. | Doğrulandı | [19][20] |
| Kararlılık notları | Resmi sürüm notlarında çökme düzeltmeleri var (örn. 7.25: dikey MP4 açarken çökme düzeltmesi; 7.26.2: klipleri temizlerken olası çökme). Forumda: Task Scheduler ile açılan Arena'nın 3-5 günde donması ve günlük yeniden başlatma önerisi (anekdot); düzensiz kapanışta advanced.xml bozulması (eski sürüm raporu, doğrulanmadı). | Anekdot / kısmen | [21][19][24] |

---

## 3. Bugünkü iş akışı riskleri (VARSAYIM; Yusuf'a doğrulatılacak)

Yerel inceleme:
- Program ekranı 950x1800 (9:16), 25 fps olarak tanımlıyor; MP4 varsayılan, `--kodek hap` ile HAP (.mov) üretebiliyor (README.md, src/ffmpeg.rs, src/uret.rs).
- H.264 çıkışı all-intra benzeri ayarlı (`-g 1`, crf 12, yuv420p); bu, geriye sarma/atlama için iyi. Hızlı önizleme yolunda `ultrafast, crf 24` kullanılıyor.
- HAP için kod, genişlik ve yüksekliğin 4'e bölünmesini şart koşuyor. 950 sayısı 4'e bölünmüyor (950 mod 4 = 2), dolayısıyla 950x1800 ile HAP üretimi hata verir. 948, 952 veya 980 gibi bir genişlik gerekir. (Koddaki hata iletisi 980x1800 örneğini veriyor.) Bu bir bulgu, kesin bir hata değil: gerçek davranış programda denenmedi.
- Bu bilgisayarda Resolume kurulu görünmüyor: `C:\Program Files\Resolume*` yok, `C:\Program Files (x86)\Resolume*` yok, `Documents\Resolume Arena` yok (yalnız klasör varlığı bakıldı, içerik okunmadı). Resolume muhtemelen ekranı süren ayrı bir bilgisayarda. Bu, "yan aracın" ağ üzerinden (REST/OSC) konuşması gerektiği anlamına gelir.
- Eski araştırma notu (ARASTIRMA-DIS-LED-EKRAN.md) 980x1800 diyor, README 950x1800 diyor; tutarsızlık var, hangisi doğru Yusuf'a sorulmalı. Ayrıca fiziksel 10x30 m oran (1:3) ile piksel oranı (950:1800 yaklaşık 1:1,9) uyuşmuyor; programda "fiziksel düzeltme" bu yüzden var (README).

Varsayılan riskler:
1. H.264 uzun/yoğun oynatmada takılabilir ve CPU'ya yük bindirir; DXV3/HAP GPU çözümü ile daha öngörülebilir [11][12]. (Varsayım: bugün H.264 oynatılıyor.)
2. Piksel ızgarası uyumsuzluğu: video boyutu, Arena kompozisyon boyutu ve LED işlemcinin giriş çözünürlüğü aynı değilse ölçekleme bulanıklığı veya kayma olur [26][16].
3. Resolume'un 24/7 gözetimi yok: çökme veya donma sonrası ekran siyah/dondurulmuş kalabilir; otomatik yeniden başlatma Windows tarafında kurulmalı [24][27].
4. Zaman çizelgesi: saat bazlı program (sabah aç, gece kapat, kampanya değişimi) için Arena'da doğrudan "Scheduler" bulunamadı; elle veya dışarıdan yönetim gerekir.
5. Dosya adı/sürüm karışıklığı: hangi video hangi kolona gitti, eski sürüm mü oynuyor. Takip aracı yoksa insan hatası riski.
6. Güvenlik: REST/WebSocket sunucusu varsayılan 0.0.0.0 dinler; kimlik doğrulama bilgisi bulunamadı (belgede şifre/anahtar anlatımı yok, doğrulanmadı). Ağda herkes erişirse ekran kontrol edilebilir [4].
7. Kompozisyon/ekran ayarı yedeği: advanced output preset (.xml) ve .avc yedeklenmiyor olabilir [15].
8. Ses: LED cephede ses yoksa sorun değil; varsa ses ve video ayrı render önerilir [11].

**Yusuf'a sorulacak sorular**
1. Arena hangi sürüm ve hangi lisansla çalışıyor (Arena mı Avenue mu)? Lisans bilgisayarı hangisi (bu PC mi, ayrı sunucu mu)?
2. Videolar bugün Resolume'a hangi biçimde yükleniyor (H.264 MP4 mi, HAP mi, DXV mi)? Takılma, siyah ekran, kare atlama veya çökme yaşıyor musunuz? Ne sıklıkta?
3. Gerçek kompozisyon ve LED işlemci giriş çözünürlüğü kaç (950x1800 mü 980x1800 mü)? İki ekran tek kompozisyonda yan yana mı, iki ayrı çıkış mı?
4. LED işlemci/denetleyici markası ve modeli (Novastar, Colorlight, Linsn)? Bilgisayardan işlemciye bağlantı: HDMI/DVI mi, ağ mı?
5. Ekran 7/24 mi çalışıyor, belli saatlerde mi? Gece kapanıyor mu? Program/kampanya kim, ne sıklıkla değiştiriyor?
6. Resolume bilgisayarı kimin elinde, uzaktan erişim (AnyDesk, TeamViewer) var mı? Ağ güvenli mi?
7. Sunucu bilgisayarın özellikleri (GPU, RAM, disk) ve Windows güncelleme/otomatik yeniden başlatma politikası?
8. Ses var mı? Canlı veri (hava durumu, saat, sayaç) veya canlı kamera ister misiniz?
9. En çok neyi kolaylaştırmak istersiniz: video hazırlama, zamanlama, izleme/alarm, yoksa hepsi mi?

---

## 4. Önerilen yan araç seçenekleri

### A. Resolume uyumlu video hazırlayıcı ve toplu dışa aktarma
- **Fayda:** Doğru kodek (HAP veya DXV3 hedefli; DXV için Resolume'un kendi kodeki gerekir), piksel ızgarası kontrolü (4'e bölünebilir, kompozisyon boyutuna eşit), tutarlı fps (25), dosya adı kuralı (kampanya_tarih_sürüm), toplu dışa aktarma ve "video ekrana uygun mu" denetimi (ffprobe ile boyut/fps/kodek kontrolü).
- **Yapılabilirlik:** Yüksek. Programın içinde zaten ffmpeg ile HAP çıkışı var (src/ffmpeg.rs); eksik olan çözünürlük kontrolü ve toplu iş. DXV3 üretimi ffmpeg'de yok; DXV için Resolume Alley gerekir, otomasyonu doğrulanmadı [11].
- **Risk:** Düşük. HAP/DXV dosyaları büyük (disk); HAP ile DXV performansı kaynaklarda çelişiyor, kendi ekranda denenmeli [14].
- **İş büyüklüğü:** Küçük (yaklaşık 2-4 gün: ffprobe doğrulayıcı, 948/952 genişlik önerisi, toplu liste, HAP varsayılan seçeneği).
- **Güvenlik notu:** Yalnız yerel dosya işler; ağ yok. Dosya adlarında kişisel veri tutulmamalı.

### B. Composition/clip üreticisi (.avc XML)
- **Fayda:** Yeni kampanya videolarını otomatik klip olarak bir .avc dosyasına yerleştirmek; kolon/katman düzenini şablondan üretmek; elle sürükle-bırak hatasını azaltmak.
- **Yapılabilirlik:** Orta-zor. .avc XML'dir ve programla üretilebileceği Resolume tarafından söylenmiş [23], ama biçim belgesi yok; sürüm bağımlı olabilir. Daha güvenli alternatif: Arena çalışırken REST API ile kolon/katman/klip eklemek [4][6] veya 7.26 MCP sunucusu [20].
- **Risk:** Orta-yüksek. Bozuk XML Arena'nın başlangıçta takılmasına yol açabilir (forumda kompozisyon/ayar dosyası kaynaklı başlangıç takılması örnekleri var; eski sürüm raporu) [24]. Mutlaka kopya üzerinde çalışılmalı, orijinal değiştirilmemeli.
- **İş büyüklüğü:** Orta (1-2 hafta; örnek .avc analizi, şablon motoru, doğrulama).
- **Güvenlik notu:** Üretilen dosya Arena'ya yüklenmeden önce insan onayı. Yedek zorunlu.

### C. OSC/REST ile zamanlayıcı ve izleme paneli
- **Fayda:** Saat bazlı program (sabah aç, kampanya değişimi, gece karartma) için Arena dışında bir zamanlayıcı; hangi klip çalıyor, kaç fps, hangi kolon gibi durumun panelde görülmesi. REST/WebSocket ile abone olma ve durum okuma mümkün [4][5]; çıkış önizleme görüntüsü 7.26 ile geliyor [19].
- **Yapılabilirlik:** Yüksek-orta. API belgeli ve açık. Tuzaklar: "set" mesajındaki değer anahtar adı ve ayrıntılar Swagger'dan alınmalı (bu araştırmada doğrulanmadı) [5]; 7.26'da API'de yük sorunu düzeltmesi olması büyük kompozisyonda dikkat gerektirir [19].
- **Risk:** Orta. Zamanlayıcı yanlış tetiklerse ekranda yanlış içerik; bu yüzden "son bilinen iyi durum" ve elle devralma düğmesi şart.
- **İş büyüklüğü:** Orta (1-2 hafta ilk sürüm).
- **Güvenlik notu:** REST/WebSocket varsayılan olarak tüm ağ arayüzlerini dinler [4]; yalnız yerel ağ, güvenlik duvarı kuralı ve mümkünse 127.0.0.1 + aynı makinede çalışan araç. Parola/anahtar belgelenmemiş; ağ dışına açılmamalı.

### D. Sağlık denetimi, izleme ve otomatik yeniden başlatma
- **Fayda:** Arena çökerse/donarsa/çıkış siyah kalırsa fark edip yeniden başlatmak, Windows açılışında Arena'yı başlatmak, e-posta/telefon uyarısı. 24/7 ekran için en doğrudan kazanım.
- **Yapılabilirlik:** Yüksek. Süreç denetimi + REST'e periyodik istek + (isteğe bağlı) çıkış görüntüsü alıp kare değişiyor mu kontrolü [19]. Ticari alternatifler var (AlwaysUp, Acquire Digital System Watchdog); önce bunları denemek mantıklı [27].
- **Risk:** Orta. Yeniden başlatma sonrası Arena'nın doğru kolonu açıp Advanced Output'u eski haline getirmesi sorun olabilir: forumda ekranlar bağlı değilse yeniden atama gerektiği, bazı kullanıcıların AutoHotkey kullandığı bildirilmiş (anekdot) [24]. "Trigger first clip on load" seçeneği bu işi kısmen görür [10].
- **İş büyüklüğü:** Küçük-orta (yaklaşık 1 hafta; süreç izleme + REST sağlık kontrolü + yeniden başlatma + günlük).
- **Güvenlik notu:** Yeniden başlatma yetkisi yüksek; yalnız yerel çalışsın, günlük tutsun, "devre dışı bırak" düğmesi olsun.

### E. Tarif ile video köprüsü (Tarif Et/LLM -> video -> Arena klibi)
- **Fayda:** TAA Vidyo Yapıcı'nın senaryo/tarif sistemiyle (docs/TARIF-ET-LLM.md) üretilen videonun otomatik "Resolume hazır" çıktıya dönüşmesi ve (A+C ile birleşince) otomatik kolona konulması: tarif -> senaryo -> HAP -> klasör -> Arena.
- **Yapılabilirlik:** Orta. Üretim tarafı hazır; teslim tarafı için A (doğrulama) ve C veya B (klibi Arena'ya koyma) gerekir.
- **Risk:** Orta-yüksek. Otomatik içerik yayını; yanlış/uygunsuz içerik doğrudan cepheye gidebilir. "İnsan onayı olmadan yayına alma" kuralı olmalı. 7.26 MCP sunucusu ayrıca yapay zekâyla bileşim düzenlemeyi mümkün kılıyor ama canlı kullanım için önerilmiyor [20].
- **İş büyüklüğü:** Orta (A ve C/B'nin üstüne ek 1 hafta).
- **Güvenlik notu:** LLM/dış hizmet çıktısı veridir, talimat değildir; yayın öncesi insan onayı, sır ve kişisel veri üretim tariflerinde bulunmamalı.

### F. (Ek) Piksel haritası ve kurulum doğrulayıcı
- **Fayda:** Tek sayfalık kurulum kartı: ekran fiziksel boyutu, piksel pitch, video boyutu, kompozisyon boyutu, işlemci girişi, fps. Uyumsuzluğu yazdırır. Mevcut "fiziksel düzeltme" mantığına bağlanır [26].
- **Yapılabilirlik:** Yüksek, çok küçük. **Risk:** Düşük. **İş:** Çok küçük (1 gün). **Güvenlik:** Yerel, ağ yok.

**Sıralama önerisi:** F + A (ilk) -> D (ikinci) -> C (üçüncü) -> B ve E (yalnız ihtiyaç doğrulanırsa).

---

## 5. Önerilen ilk adım ve 3 aşamalı yol haritası

**İlk adım (bu hafta):** (1) bölüm 3'teki soruları Yusuf'a sormak; (2) kodda 950x1800 ve HAP 4'e bölünme çelişkisini netleştirmek (Yusuf'un ekranı 950 mi 952 mi 980 mi); (3) ağda kullanılan Arena bilgisayarında REST'in açık olup olmadığını Yusuf'un kendisinin kontrol etmesini istemek (Preferences > Webserver) [4]. Bu bilgisayarda hiçbir Resolume ayarı değiştirilmedi.

| Aşama | Süre (tahmini) | İçerik | Başarı ölçütü |
|---|---|---|---|
| 1. Hazırlık ve doğrulama | 1-2 hafta | F (kurulum kartı) + A (ffprobe ile uygunluk kontrolü, HAP varsayılanı, toplu dışa aktarma, dosya adı kuralı). Sahada denenmiş bir test videosu (H.264 vs HAP) ile oynatma karşılaştırması. | Aynı video Arena'da takılmadan, ölçeklemesiz oynar |
| 2. İzleme ve dayanıklılık | 1-2 hafta | D (süreç izleme, REST sağlık kontrolü, yeniden başlatma, günlük). Önce ticari izleyicileri (AlwaysUp vb.) karşılaştır. Yedekleme betiği (preset .xml ve .avc kopyası, kopyalama yöntemiyle). | Çökme/donma sonrası 5 dakika içinde otomatik toparlanma; günlükte kayıt |
| 3. Zamanlama ve entegrasyon | 2-4 hafta | C (zamanlayıcı + panel; yalnız yerel ağ), isteğe bağlı E (Tarif -> Arena köprüsü, insan onaylı), B (yalnız ihtiyaç varsa). | Programlı kampanya değişimi insan müdahalesi olmadan, geri alma düğmesiyle |

Genel kurallar: Arena'ya yazan her araç önce kopya/yedekle çalışır; silme yok; Yusuf onayı olmadan canlı ekrana otomatik içerik gitmez.

---

## 6. Kaynaklar (numaralı)

Not: Çoğu sayfa WebFetch ile okundu. "(arama özeti)" yazanlar yalnız arama sonucunda özetle görüldü, sayfanın tamamı okunmadı.

1. Resolume lisans yenileme ve yükseltme fiyatları (arama özeti): https://www.resolume.com/shop/upgrade
2. Bayi listesi, Arena 7 tek bilgisayar (örnek fiyat; resmi değil): https://audioeffetti.com/en/category/video/media-servers/resolume-arena7x1-resolume-arena-7-for-1-computer-p8521
3. Resolume Avenue ve Arena 7 duyurusu (arama özeti): https://resolume.com/blog/18695
4. REST API ve Webserver (resmi): https://resolume.com/support/en/restapi
5. WebSocket API (resmi): https://resolume.com/support/en/websocket-api
6. Arena ve Avenue REST API OpenAPI belgesi (arama özeti): https://resolume.com/docs/restapi/
7. OSC (resmi): https://resolume.com/support/en/osc
8. Kısayollar: klavye, MIDI, OSC, DMX (resmi): https://resolume.com/support/en/shortcuts
9. Autopilot (resmi, v7.22): https://resolume.com/support/en/autopilot
10. Resolume 7.22 sürüm yazısı, kolon autopilot (arama özeti): https://resolume.com/blog/27125
11. Preparing Media: DXV, ses, codec kurulumu (resmi): https://resolume.com/support/en/preparing-media
12. Forum: DXV3 ve H.264 (topluluk/çalışan yanıtı, arama özeti): https://resolume.com/forum/viewtopic.php?t=15826
13. Resolume blog: DXV3 yazısı (arama özeti): https://resolume.com/blog/11923
14. Forum: DXV ve HAP performansı (topluluk, çelişkili, arama özeti): https://resolume.com/forum/viewtopic.php?t=26109
15. Advanced Output (resmi, v6 sayfası): https://resolume.com/article/21
16. Input Maps (resmi, arama özeti): https://resolume.com/article/32
17. Output Transformation (resmi, arama özeti): https://resolume.com/article/52
18. Tech Specs (resmi, arama özeti): https://resolume.com/support/tech-specs
19. Resolume 7.26: MCP sunucuları ve REST API yenilikleri (resmi): https://resolume.com/blog/34484
20. MCP Servers (resmi): https://resolume.com/support/en/mcp-servers
21. Resolume 7.25 sürüm yazısı (arama özeti): https://resolume.com/blog/34172
22. Avenue ve Arena ürün sayfası: NDI, Syphon, Spout (arama özeti): https://resolume.com/software/avenue-arena
23. Forum: .avc XML ve programla bileşim üretme (Resolume yanıtı, arama özeti): https://www.resolume.com/forum/viewtopic.php?p=77918
24. Forum: sabit kurulum, otomatik başlatma, 7/24 notları (topluluk, anekdot, arama özeti): https://resolume.com/forum/viewtopic.php?p=82178
25. Resolume destek dizini: NDI, Syphon/Spout, ISF, Wire girişleri: https://resolume.com/support
26. Pixelight: LED ekran içerik biçimi, çözünürlük, fps, codec (üçüncü taraf): https://pixelight.fr/en/blog/en-led-screen-video-content-formats-resolution-2026
27. AlwaysUp SSS: uygulama izleme/yeniden başlatma (üçüncü taraf, ticari): https://www.coretechnologies.com/products/AlwaysUp/AlwaysUp_FAQ.html
28. Resolume sözlük (Basic Vocabulary): https://resolume.com/support/en/vocabulary
29. Forum: ISF desteği 7.8 (eski, arama özeti): https://resolume.com/forum/viewtopic.php?t=14394
30. NovaLCT kılavuzu (yalnız içindekiler görüldü; yapılandırma bilgisi alınamadı): https://cdn.shopify.com/s/files/1/0282/1616/4457/files/NovaLCT-LED-Configuration-Tool-for-Multimedia-Player-User-Manual-V5.4.4.pdf

**Yerel dosyalar (salt okunur incelendi):** `D:\Programlar\TAA Vidyo Yapıcı\README.md`, `...\ARASTIRMA-DIS-LED-EKRAN.md`, `...\src\ffmpeg.rs`, `...\src\uret.rs`, `...\docs` klasörü (yalnız TARIF-ET-LLM.md ve TARIF-KURALLARI.md var; video biçimi bilgisi içermiyor).

## Doğrulanamayanlar (özet)
- Yeni Arena 7 lisansının resmi güncel fiyatı ve lisansın tam koşulları.
- Arena'nın en güncel sürümü (7.28 yalnız bir blog listesinde görüldü) ve 7.23/7.24 ayrıntıları.
- Resmi "DXV3/HAP/H.264 önerisi" sayfası; HAP ve DXV performansı (kaynaklar çelişiyor).
- .avc XML eleman yapısı; Media Manager ayrıntısı.
- Scheduler (takvim) özelliği: resmi belgede bulunamadı.
- Art-Net/sACN giriş ayrıntısı, REST için kimlik doğrulama, maksimum çıkış sayısı.
- Colorlight ve Linsn için Resolume'a özgü rehber bulunamadı; Novastar için yalnız forum kullanıcı anlatımı (HDMI ile işlemciye, giriş çözünürlüğü sınırları işlemciye bağlı).
- Resolume bu bilgisayarda kurulu görünmüyor (klasör yok); gerçek kurulum başka bilgisayarda olabilir.
