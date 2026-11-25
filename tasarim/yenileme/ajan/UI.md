---
description: UI ajanı görevi (dalga 3). Sahip: Yusuf. Yazan: [Claude#Sonnet-5.5 koordinatör, 2026-11-25]. Silme yok.
---
# UI ajanı
Sahip olduğun: web/index.html, tests/arayuz_ui.rs, src/arayuz.rs. index.html'de değişiklikleri hedefli Edit ile yap (tüm dosyayı yeniden yazma).

Fable bulgularını kapat:
1. (1-host) window.TaaApp'e geciciOnizle(metin:string):boolean ve geciciOnizleKapat():void ekle: senaryoyu DEĞİŞTİRMEDEN önizleme karesini gösterir; kapatınca ya da yeni senaryo işlemi olunca eski senaryonun önizlemesine döner. TaaApp.onizle aynı kalsın. İmza sabit (Malzeme ve AI ajanları kullanacak).
2. (4) "Tasarım" önizlemesi 190x360 pikselde kalıyor: canvas iç çözünürlüğü kutuya göre ayarlansın, genişlik aspect-ratio 950/1800 ile yükseklikten türesin, çekmece geçişi bitince bir kez yeniden ölçülsün (1366x768'de kare ≈270x515, 1920x1080'de ≈420x800). "Ekranda nasıl görünür" de net.
3. (12) AI/malzeme senaryoyu değiştirince Yaz kipinde net bildirim + "Önceki senaryoya dön" her zaman görünür.
4. (15) 48 px altı hedefler 48'e (Eşlik kip düğmeleri, Görünüm renk noktaları, Düz/Bina ve 3m/15m, paneli kapat, durum hapı). (16) Durum hapı metni üç noktayla kesilmesin (iki satıra esnesin). (17) .ilerleme genişlik geçişi transform(scaleX); hareketi azaltta geçişler 120 ms solma. (18) Gündüz/Akşam Düz görünümde gizlenmesin: devre dışı + neden yazısı. (14) Üst çubuk sürümü /bilgi'den gelsin. (19) "DeepSeek'de aç"→"DeepSeek'te aç", "Claude'de aç"→"Claude'da aç", "Gosterisli"→"Gösterişli".

Yeni entegrasyon:
5. Üst çubukta/menüde "Oynatıcı" girişi: /oynatici sayfasını yeni pencerede/sekmede açar (Oynatıcı ajanı sayfayı yazar; src/arayuz.rs'te /oynatici, /oynatici.js, /oynatici.css, /cikis rotalarını ekle; include_str için yer tutucu dosyaları web/oynatici.html, oynatici.js, oynatici.css, cikis.html olarak oluştur (yoksa; dolu dosyalara dokunma, SİLME/yeniden adlandırma yok) ve (m, y) if y.starts_with("/oynatici/") rotasını crate::oynatici_http::isle(...) çağrısına bağla (modül yoksa Oynatıcı ajanı ekler; derleme kırılmasın: ISTEK dosyasından oku).
6. Hazır kipinde üçüncü sekme "Dönüştürücü": <div id="donusturMount">, window.TaaDonustur.mount(el, window.TaaApp) varsa çağır; /donustur.js ve /donustur/ rotası (crate::donustur_http::isle) için aynı yöntem (yer tutucu web/donustur.js).
7. Yapay zekâ kipi kapsayıcısında AI ajanının web/ai.js adım göstergesi için yer bırak.

Testleri tests/arayuz_ui.rs'e ekle; tarayıcıda (--arayuz --tarayici-yok --port 47961; TAA_AYAR_DOSYASI ile geçici ayar dosyası; gerçek ayar.json'a dokunma) 1366x768 ve 1920x1080'de altı tema ve her kipi sına; ekran görüntülerini tasarim/yenileme/ekran/ altına yaz.
