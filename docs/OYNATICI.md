---
description: TAA Vidyo Yapıcı 1.0 katmanlı oynatıcı (Resolume Arena benzeri ekran oynatıcısı) kullanım, kurulum ve teknik belgesi.
sahip: Yusuf
yazan: "[Claude#Oynatici Opus 5.5, yenileme-1.0, 2026-11-25]"
degistirme-kurali: "Yalnız yazan yapay zekâ ya da Yusuf'un izniyle değişir; güncelleme üstü çizilerek yapılır; silme yok."
bagli-karar: tasarim/yenileme/karar-oynatici.md (KARAR-OYN ve Ek 1)
---

# Oynatıcı (katmanlı ekran oynatıcısı)

## Özet
- Program içindeki iki sayfa: `/oynatici` (kontrol, kumanda) ve `/cikis` (ekrana giden görüntü). Ayrı program yok, yeni bağımlılık yok.
- Oynatma motoru **çıkış** sayfasında çalışır. Kontrol penceresi küçültülse ya da donsa da ekran akmaya devam eder.
- Tek doğruluk kaynağı sunucudur: proje `oynatici.json` dosyasında kalıcıdır, anlık komutlar kuyruğa yazılır, çıkış uzun yoklamayla okur. BroadcastChannel yalnız sürgü hızlandırıcısıdır.
- Medya yalnız program çıktı klasöründen gelir (`%USERPROFILE%\Videos\TAA Vidyo Yapıcı`, `LED_CIKTI` ile değişir). Dosya silme, yükleme, alt klasör yok.
- **Uyarı:** 24 saatlik saha kabulü (aşağıda) geçilmeden "Arena'nın yerini aldı" denmez.

## Kullanım
1. Ana sayfadaki menüden **Oynatıcı**'ya girin (ya da adres çubuğu olmadan: program içinde `/oynatici`).
2. **Yeni katman** ile katman ekleyin (en çok 8). Üstteki katman ekranda öndedir.
3. Sağdaki **Medya** listesinden dosyayı katmana ya da klip listesine sürükleyin veya **Ekle**'ye basın. `.mov` (HAP) listede görünür ama eklenemez: mp4 olarak üretin.
4. Katman ayarları: Oynatma (Sıralı / Döngü / Rastgele), Liste bitince (**Son klipten başa** varsayılan / Son karede dur / Katmanı kapat), Hız (0,25×-2×), Karışım (Normal / Ekle / Çarp), Hedef ekran (İki ekran eşlenik / Sol / Sağ / İkisine yayılı), Sığdırma, Geçiş (0-2000 ms çapraz solma), Opaklık, Ses, Sessiz.
5. **Çıkışı aç**: çıkış penceresi seçili monitörde, kenarlıksız, en üstte ve 1:1 pikselle açılır. Görev çubuğunda simgesi yoktur.
6. **Karart**: bütün çıkış siyaha döner, oynatma sürer. **Hepsini duraklat/oynat** bütün katmanlara uygulanır.
7. **Sahneyi klip yap**: senaryo metni mevcut motorla `oynatici-sahne-<zaman>.mp4` olarak pişirilir ve seçili katmana eklenir. Ana sayfada üretim varken 409 döner.

"Listeden çıkar" yalnız listeyi değiştirir; dosya silinmez. 5 sn içinde **Geri al** ile geri gelir.

## Kısayollar (kontrol sayfası odaktayken; yazı alanında kapalı)
| Tuş | İş |
|---|---|
| Boşluk / Shift+Boşluk | Seçili katmanı / hepsini oynat-duraklat |
| Sağ ok / Sol ok | Sonraki / önceki klip |
| Shift+Sağ / Shift+Sol | 5 sn ileri / geri |
| Home | Listeyi başa al |
| Yukarı / Aşağı ok | Opaklık +5 / -5 |
| 1-8 / Shift+1-8 | Katman seç / aç-kapat |
| A, S, M | Aç-kapat, Solo, Sessiz |
| B | Karart |
| N | Yeni katman |
| Ctrl+Yukarı / Ctrl+Aşağı | Katmanı öne / arkaya al |
| Delete | Etkin klibi listeden çıkar (dosya silinmez) |
| O | Çıkışı aç/kapat |
| ? | Kısayol listesi |
Çıkış penceresinde yalnız **Ctrl+Shift+Q** (acil kapatma) çalışır; Esc pencereyi kapatmaz; fare imleci gizlidir.

## Kurulum ve ekran ayarı
- **Monitör seçimi:** "Çıkış ayarı" altında monitör listesi (ad, çözünürlük) ve X/Y kaydırma vardır. Pencere seçilen monitörün sol üst köşesi + kaydırma konumuna açılır. LED işlemcisi masaüstünün belli bir bölgesini alıyorsa o bölgenin konumunu X/Y olarak girin.
- **1:1 piksel:** Pencere boyutu kompozisyon kadardır (iki ekran 1900x1800, tek ekran 950x1800 fiziksel piksel). Sahne `scale(1/devicePixelRatio)` ile fiziksel piksele kilitlenir; Windows ölçeği %125 olsa da 950 piksel 950 LED pikseline düşer. Doğrulama: 1 piksellik dama deseni resmini katmana koyup ekranda bulanıklık olmadığına bakın (saha testi, yapılmadı).
- **Program açılınca çıkışı aç:** "Çıkış ayarı" altındaki onay kutusu. Açıksa program her açılışta kayıtlı monitör ve kaydırmayla çıkışı açar.
- **Güç ayarı:** Çıkış sayfası ekran uykusunu `navigator.wakeLock` ile engellemeye çalışır. Alınamazsa kontrol sayfası "Ekran uykusu engellenemedi" yazar; o zaman Windows **Ayarlar > Sistem > Güç** altında ekranı kapatma ve uyku süresini "Hiçbir zaman" yapın.
- **Ana pencere kapatılırsa:** Çıkış açıkken ana pencere kapatılırsa program kapanır ve ekran kararır (karar açık sorusu 3; Yusuf'un kararına kadar bu davranış kalır). Çıkış penceresi kapatılırsa yalnız o kapanır.
- **Ses:** Varsayılan bütün katmanlar sessizdir (cephe ekranı). Ses çıkış penceresinden çıkar; "ikisi" hedefinde yalnız ilk kopya ses verir.
- **Performans:** Program mp4'leri her karesi anahtar kare, yüksek bit hızlıdır. Yavaş diskte takılma olursa oynatma için ayrı, daha düşük bit hızlı dışa aktarım gerekir (sonraya). Aynı anda 4'ten çok video katmanı oynarsa kontrol sayfası uyarır.

## Kalıcılık ve günlük
- Proje: `%LOCALAPPDATA%\TAA Vidyo Yapici\oynatici.json` (`TAA_OYNATICI_DOSYASI` ile değişir). Atomik yazma (geçici dosya + yeniden adlandırma). Bozuk ya da 256 KB'yi aşan dosya varsayılan projeyle açılır; bozuk dosyaya dokunulmaz, yanına `.bozuk-<zaman>` kopyası konur.
- Kalıcı olanlar: bütün katman ayarları, klip listeleri, katman başına etkin klip ve oynuyor durumu, çıkış monitörü/kaydırması, açılışta çıkış açma. Klip içi konum kalıcı değildir.
- Günlük: `%LOCALAPPDATA%\TAA Vidyo Yapici\oynatici-gunluk.txt` (aç/kapa, atlanan klip, hata). 1 MB'yi geçince eskisi tarihli adla saklanır, silinmez.
- Dosyası kaybolan klip listede "bulunamadı" işaretiyle kalır, oynatmada atlanır; ekranda hata yazısı çıkmaz.

## Teknik
| Dosya | İş |
|---|---|
| `src/oynatici_http.rs` | Uçlar, JSON çözücü (derinlik ≤16), proje doğrulama, kalıcılık, komut kuyruğu (64), uzun yoklama (en çok 8 bekleyen), canlı durum, sahne pişirme, açılışta çıkış |
| `src/oynatici_medya.rs` | Medya listesi (≤2000), ad kuralı ve `canonicalize` ile yol geçişi koruması, tek aralıklı Range, 64 KB parçalı akış, eşzamanlı medya bağlantısı sınırı 32 (aşımda 503) |
| `src/oynatici_pencere.rs` | (Windows) çıkış penceresi: tao + wry, kenarlıksız, en üstte, görev çubuğunda yok, imleç gizli, gezinme kilidi (yalnız `http://127.0.0.1:<port>/`) |
| `web/oynatici.html/.js/.css` | Kontrol sayfası |
| `web/cikis.html`, `web/oynatici-cikis.js` | Çıkış sayfası ve DOM birleştirici (üst üste `<video>`/`<img>`, `opacity`, `mix-blend-mode`) |
| `web/oynatici-motor.js` | DOM'suz mantık (sıradaki klip, torba, bitince, solo, hedef kutuları, doğrulama); node testi de kullanır |
| `tests/oynatici.rs`, `tests/oynatici_test.js` | Rust uç testleri ve node motor testi |

### Uçlar
`GET /oynatici`, `GET /cikis` (CSP'li sayfalar) · `GET /oynatici/dosya/<ad>` (sabit beyaz liste) · `GET|POST /oynatici/proje` (`temelSurum`; 400/409) · `POST /oynatici/komut` · `GET /oynatici/olay?surum=&komut=` (uzun yoklama, 20 sn) · `GET|POST /oynatici/canli` · `GET /oynatici/medyalar` · `GET|HEAD /oynatici/medya/<ad>` (Range) · `GET /oynatici/kucuk/<ad>` (yalnız resim) · `POST|GET /oynatici/sahne`, `POST /oynatici/sahne-iptal` · `GET /oynatici/monitorler`, `POST /oynatici/cikis` (köprü yoksa 503).

### Güvenlik
- Yeni yetki yolu yok: bütün yollar `arayuz.rs` içinde `Host` + oturum çerezi + POST'ta `X-LED: 1` denetiminden sonra bu modüle gelir.
- CSP: `default-src 'none'; script-src 'self'; style-src 'self'; img-src 'self' blob:; media-src 'self'; connect-src 'self'; base-uri 'none'; form-action 'none'; frame-ancestors 'none'`. Satır içi betik/stil yok; metinler yalnız `textContent`.
- Medya adı: boş değil, ≤200 karakter, `/ \ : < > " | ? *` ve denetim karakteri yok, `.` ile başlamaz, `..` içermez, boşluk/nokta ile bitmez, uzantı mp4/webm/png/jpg/jpeg/webp; `canonicalize` sonrası üst klasör kök olmalı (bağlantıyla dışarı çıkış kapalı).

## Test
- Otomatik: `tasarim/yenileme/kilitli-cargo.sh test --release --test oynatici` (node motor testi de içinden çağrılır; node yoksa atlanır). Ayrıca `node tests/oynatici_test.js`.
- Elle tarayıcı: `taa-vidyo-yapici --arayuz --tarayici-yok --port 47963` (geçici `TAA_AYAR_DOSYASI`, `TAA_OYNATICI_DOSYASI`, `LED_CIKTI`); yazdırılan `/?t=<jeton>` adresi açılıp `/oynatici` ve `/cikis` iki sekmede denenir. Ekran görüntüleri `tasarim/yenileme/ekran/oynatici-*.png`.

## Saha kabul ölçütü (Arena yerine kullanmadan önce)
Gerçek ekran bilgisayarında 24 saat kesintisiz, düşen kare < %1, siyah kare ve donma yok; program yeniden açıldığında son proje ve katman durumu geri gelir. İkinci monitörde 1:1 dama deseni, 30 dk dayanıklılık ve karışım piksel denetimi de bu turda yapılmadı (karar Ek 1 A7).

## Sonraya
Gerçek zamanlı canlı sahne, WebGL birleştirici ve efektler, tersine oynatma, dilim/haritalama, takvim, otomatik yeniden açma, uzaktan kumanda/OSC/MIDI, HAP, dosya yükleme, canlı çıkış önizlemesi.
