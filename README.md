# TAA Vidyo Yapıcı (TAA Paket programı, tek bilgisayarda kullanılır)

Bina dışı dikey LED ekranlar (Yusuf’un ekranı 950x1800, 9:16) için 3B yazı, parçacık ve geçişli, karanlıkta okunur MP4/HAP video üretir. Videolar bir **senaryo dosyası** ile tanımlanır.

## Kurulum (Windows)
`TAA-Vidyo-Yapici-Kurulum.exe` çalıştırılır: `D:\Programlar\TAA Vidyo Yapıcı\` klasörüne kurar (D: yoksa `%LOCALAPPDATA%\Programs`), masaüstü ve Başlat menüsü kısayolu ile Uygulamalar listesine kayıt ekler, `kaldir.exe` yazar. `--sessiz` sorusuz kurar, `--hedef KLASÖR` klasörü değiştirir, `--kisayolsuz` kısayol ve kayıt yazmaz.
Kaldırmak için Ayarlar > Uygulamalar'dan veya `kaldir.exe` ile; yalnız kendi dosyaları silinir, videolara ve kullanıcının eklediği senaryolara dokunulmaz.
**ffmpeg** programla gelmez; `ffmpeg.exe` programın klasörüne (veya altındaki `ffmpeg\bin`), `D:\Programlar\Araçlar Ve İndirilenler` altına konur ya da `LED_FFMPEG` ortam değişkeniyle yolu verilir; yoksa PATH aranır.

## Arayüz
`taa-vidyo-yapici.exe` argümansız çalışınca (veya kısayolla) yerel arayüz tarayıcıda açılır; yalnız bu bilgisayardan erişilir (127.0.0.1). Senaryo yazılır, örnekler yüklenir, "Sahne ekle" formu satır yazar, ekran oranında önizleme ve zaman çubuğu vardır; video `Videolar\TAA Vidyo Yapıcı` klasörüne MP4 veya HAP olarak üretilir (`LED_CIKTI` klasörü değiştirir). Sekme kapanınca program kendiliğinden kapanır; ayrıca "Kapat" düğmesi vardır. HAP için ekran ölçüsü 4'e bölünmelidir (950x1800 uygun değildir, MP4 kullanın).

## Senaryo ile video
```
taa-vidyo-yapici --senaryo senaryolar/tv-afis.txt --cikis kampanya.mp4
```
Senaryo dosyası (`#` yorum satırı):
```
ekran 950x1800
fiziksel 8x25        # gerçek ekran ölçüsü (metre); piksel kare değilse yazı gerilmez
fps 25
acilis tv            # tv | perde | yok
kapanis tv           # tv | perde | yok
sahne "PEŞİN|FİYATINA|*3|TAKSİT" renk=altin giris=toplan cikis=dagil sure=8 efekt=parlama,parcacik
```
- **renk:** altin, sari, beyaz (kısık), turuncu, cyan, magenta, lime (siyah/koyu zemin).
- **giris:** yok, donerek, zipla, yandan, zoom, alttan, ustten, belir (yavaşça belirir), elastik (yaylanarak büyür), daktilo (harf harf yazılır), sil (soldan sağa açılır), toplan (parçacıklardan toplanır). Ayrıca "Animasyon kütüphanesi" bölümündeki tüm adlar (spiral, salincak, neon, mercek, ...).
- **cikis:** yok, yukari, asagi, donerek, sondur, kucul (dönerek küçülür), sil (soldan sağa silinir), dagil (parçacıklara dağılır).
- **efekt:** parlama (yazıdan geçen ışık), pirilti (yıldızlar), cerceve (koşan ışıklı çerçeve), cift-cerceve (çift çizgi çerçeve), parcacik (süzülen parçacıklar), kivilcim (uçan kıvılcımlar), kar, yagmur, halka, kayan-yildiz; `efekt=yok` hepsini kapatır.
- **Düzen:** `sahne "PEŞİN|FİYATINA|*3|TAKSİT"`: `|` açık satır ayırır, `*` ile başlayan satır büyük rakamdır ve yanlarına çift çizgi gelir. `|` yoksa kelimeler alt alta dizilir.
- **Çizimler:** `@cicek`, `@papatya`, `@yildiz`, `@rozet`, `@cerceve`, `@elmas` (varsayılan senaryolarda yok, yalnızca istenirse eklenir; bkz. senaryolar/ornek-cicekli.txt).
- Hata olursa satır numarasıyla Türkçe mesaj verir.
- `--genislik/--yukseklik/--fps` senaryodaki değeri geçersiz kılar (genişlik çift sayı olmalı), `--duz` fiziksel düzeltmeyi kapatır, `--kodek hap` HAP üretir.

## Otomatik mod ve yeni arayüz (0.5.0)
Arayüz tek ekran: solda yazı kutusu, tarz kartları (Yumuşak, Enerjik, Minimal, Gösterişli, Şenlik), "Şaşırt beni", renk paletleri, yazı stilleri, animasyon galerisi ve öneriler; sağda büyük canlı önizleme. Önizleme iki görünüm içerir: tasarım ve "ekranda nasıl görünür" (8x25 m LED, nokta ve çıta benzetimi, 3 m / 15 m). Her değişiklikte alttaki klip yenilenir ve döngüde oynar; tam ekran düğmesi vardır.
- **Yalnız yazıyı yazın:** boş satır yeni mesajdır; yalnız rakam satırı büyük rakam olur; uzun satırlar 10 harfe bölünür. Düzen, font, palet, açılış, animasyonlar, geçişler, dekor ve kapanış otomatik kurulur (kural tabanlı; en çok 2 hareket ailesi, çıkış girişin tersi, açılış ve kapanış güçlü).
- **Sahne anahtarları:** `font=`, `palet=` (`renk=` da çalışır), `derinlik=0..3`.
- **Fontlar (10):** inter, anton, archivo, poppins, barlow, slab, russo, fjalla, bangers, bebas (hepsi SIL OFL). **Paletler (12):** altin-gece, sicak-beyaz, buz-cyan, cyan-vurgu, magenta-gece, pembe-parilti, gok-mavisi, altin-cyan, pembe-altin, mor-altin, tek-altin, sari-lacivert.
- **Açılış/kapanış:** tv, perde, kapi, iris, slayt, fade, neon, yok.
- **Dekor efektleri (`efekt=`):** cicek-yagmuru, papatya, yaprak, kelebek, ugur-bocegi, ari, ates-bocegi, kalp, yildiz-yagmuru, konfeti, balon, havai-fisek, pirilti-bulutu (mevcut parlama, pirilti, kar, kivilcim vb. ile birlikte).
- **Okunurluk denetimi:** en küçük harf ekran yüksekliğinin en az %5'i (~90 px), ana satırlar %7-9, satır başına en çok 10 harf, sahne en az 5 sn, kontrast 7:1; uygunsuzsa arayüzde uyarı çıkar (sade metin). Sert hareketler (yildirim, hizli-*, titre) için LED uyarısı verir.
- Giriş animasyonu sayısı 130'un üzerinde (25 yerleşik + dosyalardan: yön, dönüş, satır satır, vurgu); `animasyonlar/*.txt` ile eklenir.

## Animasyon kütüphanesi (çok sayıda, eklenebilir)
Her animasyon hem **giriş** hem **çıkış** olarak kullanılır (çıkışta ters oynar): `giris=spiral cikis=spiral`. Arayüzde açılır listeler kütüphaneden dolar; "Seçili giriş ve çıkışı oynat" düğmesi hemen önizletir.
- **Yerleşik (kodla):** dusus, sagdan, soldan, merdiven, cevir, salincak, buyu, patla, sarsil, dalga, yay-sag, yay-sol, ucarak, spiral, derinles, uzaktan, sinema, capraz, asilma, nabiz, neon, yavas-belir, yuksel, donerek-buyu, yildiz; ayrıca eski donerek, zipla, yandan, zoom, alttan, ustten, belir, elastik, daktilo, sil, toplan (parçacık) ve çıkışlar yukari, asagi, sondur, kucul, dagil.
- **Özel (metin dosyası):** `animasyonlar/temel.txt` ve `vurucu.txt` gömülüdür (mercek, gelgit, kapi, firlat, yaprak, sicra, yildirim, agirlik, firtina, mekik, uzay, ...).
- **Kendi animasyonunu ekle:** programı değiştirmeden `.txt` dosyası yaz ve şu klasörlerden birine koy: programın yanındaki `animasyonlar`, `Videolar\TAA Vidyo Yapıcı\animasyonlar` veya `LED_ANIMASYON` ortam değişkeniyle verilen klasör. Aynı ada sahip dosya yerleşik özelin yerine geçer. Biçim:
```
animasyon uzaydan-gel          # tek kelime ad
aciklama Uzaktan küçük gelir   # arayüzde görünür
sure 1.2                       # saniye (0.2-5)
gecikme 0.2                    # satırlar arası gecikme
egri yumusak                   # yumusak | dogrusal
anahtar 0.0 olcek=0.05 aci=360 parlak=0   # k=0 başlangıç
anahtar 0.7 olcek=1.15 aci=-20 parlak=1
anahtar 1.0                    # k=1 yerine oturmuş hâl (belirtilmezse eklenir)
```
Alanlar: `dx`, `dy` (ekran oranı; dx=-1 bir ekran genişliği sola), `olcek`, `aci` (derece, dikey eksende dönme), `parlak` (0-1), `derinlik` (3B kalınlık katsayısı). Hatalar satır numarasıyla Türkçe bildirilir.
- **Efektler (10):** parlama, pirilti, cerceve, cift-cerceve, parcacik, kivilcim, kar, yagmur, halka, kayan-yildiz.
- Hepsini görmek için: `senaryolar/animasyon-galerisi.txt`.

## Fiziksel ekran ölçüsü
`fiziksel 8x25` (metre, genişlik x yükseklik) verilirse program gerçek ekran oranında (kare piksel) çizer ve çıkışta dikeyde piksel yüksekliğine indirir. Örnek: 8 m / 950 px = 8,4 mm, 25 m / 1800 px = 13,9 mm; piksel oranı 1,65. Düzeltme olmazsa yazı ve çizimler ekranda 1,7 kat gerilir. Resolume’da kompozisyon 950x1800 olmalı ve video ölçeksiz oynatılmalı.

## Örnek senaryolar
`senaryolar/`: tv-referans, tv-afis, ornek-gibi, toplan-dagil, daktilo-kivilcim, elastik-parcacik, tv-klasik, tv-iki-mesaj, perde-hizli, ornek-cicekli.

## Eski hızlı modlar
`--mod sabit|kayan|3d|tv` ile tek komutta video (bkz. `--yardim`).

## Tasarım ilkeleri
ARASTIRMA-DIS-LED-EKRAN.md: beyaz kısık, flaş yok, tarama çizgisi yok, kalın font, 10-15 sn döngü.

## Derleme (Linux'tan Windows exe)
`./derle-windows.sh` (gerekenler: `rustup target add x86_64-pc-windows-gnu`, `pip install ziglang`, `cargo install cargo-zigbuild`) `dist/` altına `taa-vidyo-yapici.exe` ve `TAA-Vidyo-Yapici-Kurulum.exe` üretir. Simge `logo/simge.ico` (kısayollarda kullanılır; exe dosyasına gömülü değildir).

## Durum
Çekirdek, senaryo motoru, arayüz, kurulum ve kaldırma hazır (29 test; Linux'ta arayüz ve kurulum mantığı denendi). Render CPU'da çalışır (10 sn video yaklaşık 2,5-3 dk). Windows exe'nin kendisi bir Windows bilgisayarda henüz denenmedi.

## Yenilikler (0.6.0)
- Sadeleşen arayüz: 1 Ne yazsın, 2 Nasıl görünsün, 3 Hazır; gerisi kapalı "Ayrıntılar" altında.
- 18 hazır şablon, 16 yeni font, 18 yeni palet, özel renk (`renk=hex-ffcc00`).
- 10 çerçeve, 4 hareketli zemin, 7 yeni geçiş, 36 yeni animasyon, hareket eğrileri.
- Yanıp sönme koruması (`koruma acik|kapali`), canlı metin (`{tarih}`, `{gun:YYYY-MM-DD}`), güvenli alan çizgisi.
- Rehber: üstteki "Rehber" düğmesi, örnekleri canlı oynatır.

## Yenilikler (0.7.0)
- "Şablon kaydet": tasarımı adla saklar (Videolar\TAA Vidyo Yapıcı\Şablonlarım), şablon listesinde "Benim" başlığında görünür.
- Panel profili (Çıktı bölümü): sık ekran ölçüleri tek seçimle.
- Dikişsiz döngü (`dongu acik`): başı ve sonu yumuşak karartır.

## Bina önizleme (0.8.0)
Önizleme penceresinde **Düz | Bina karşıdan | Bina üstten** seçilir (seçim kalıcıdır). Varsayılan ölçüler (metre): cephe 31 yatay x 35 yükseklik, iki ekran 10 x 30 (ön sol ve ön sağ), orta bölüm 8 (baklava deseni ve krem çıtalar), logo Ø6 (merkez 18, 16-20 aralığı), ekran alt kenarı 3. Hepsi ayarlanır, "Ölçüleri varsayılana dön" bu değerlere döner. Ekran bina yüksekliğini ya da cephe genişliğini aşarsa kırmızı uyarı çıkar. Bina 2B kabul edilir, derinlik yoktur.
**Bina ölçüleri kaynağı: Yusuf, 2026-11-15/16** (31 m'nin yatay, 35 m'nin yükseklik sayılması bir yorumdur; ayardan değişir). Ekran alt kenarı gerçek değeri doğrulanmadı.

## Tarif et (0.9.0)
Ana ekranda **Tarif et** kartı: videoyu yazıyla anlatırsınız, program sahne listesini çıkarır, önizleme güncellenir; video yalnız **Videoyu yap** ile üretilir.
- **Kuraldan çöz** (internetsiz): yerleşik Türkçe kural motoru. Metni tırnak içinde ya da BÜYÜK HARFLE yazın. Anlamadığı kelimeyi sormaz, önizlemede vurgular; **Bunu kural olarak öğret** ile kalıcı kural eklenir. Kural listesi: kart içinde arama kutulu liste, düz belge `docs/TARIF-KURALLARI.md`.
- **İstemi kopyala / ChatGPT, Gemini, DeepSeek, Claude'da aç**: program kendi gerçek katalog adlarından "yalnız JSON döndür" istemi üretir ve panoya koyar; yanıtı kutuya yapıştırıp **Yanıtı uygula** dersiniz. Sahne listesi sıkı doğrulamadan geçer (en çok 12 sahne, sahne başına 6 satır, satır 40 karakter, sahne 1,5-15 sn, toplam 120 sn; HTML, bağlantı, dosya yolu ve komut benzeri metin temizlenir; bilinmeyen ad en yakın katalog adına eşlenir ve gösterilir). Hata olursa "Düzelt istemini kopyala" ile yapay zekâya geri verilir.
- Hiçbir şey çalıştırılmaz; dış siteler yalnız listedeki ada göre açılır. Yapay zekâya yalnız tarif metni ve katalog adları gider; kişisel bilgi yazmayın. Sonraki adımlar (pano algılama, isteğe bağlı doğrudan bağlantı) için `docs/TARIF-ET-LLM.md`.

## 0.9.4: Yapay zekâya yazdır, Hazır senaryolar, tam karşıdan bina, video temalı logo
- **Yapay zekâya yazdır** (ana ekranda, açık gelir): 1 videoyu yazın, 2 istemi kopyalayın, 3 yapay zekâya yapıştırın, 4 yanıtı yapıştırın, 5 uygulayın; adımlar kısa bir animasyonla vurgulanır; "istemi göster" gidecek metni (katalog adları + ekran/bina bilgisi + tarifiniz) gösterir; önizleme çubuğundan ve senaryo bölümünden de erişilir. Tarifiniz yalnız bir konuysa (örn. yaz indirimi, %30, 3 taksit) istem, yapay zekâdan reklam metinlerini de yazmasını ister.
- **Hazır senaryolar** (13 adet): kampanya, açılış, bayram, ürün tanıtımı vb.; her biri geçerli sahne listesidir, satırlar en çok 7 harf (3 m okunurluk testten geçer); metni düzeltip "Bu senaryoyu yükle", sonra "Videoyu yap". Kaynak: kurallar/hazir-senaryolar.json.
- **Bina karşıdan**: yalnız tam karşıdan düz ön cephe (bina üstten ve fotoğraf görünümü kaldırıldı); çıtalı ekranlar, krem baklava panel, bakır TAA logosu, çatı yazısı, tabela bandı, vitrin, kolonlar; Gündüz/Akşam; ölçüler aynı.
- **Logo**: film karesi + oynat üçgeni; eski logolar logo/eski/.

## 0.9.5: rastgele açılış animasyonu, İkiz Kabartma logosu
- **Açılış**: program her açılışta `web/acilis/` içindeki 14 animasyondan rastgele birini tam pencere oynatır (son oynayan bir sonrakinde tekrar gelmez; `localStorage taa-acilis-son`). En geç 4,5 sn sürer; Esc, tıklama ya da Geç anında geçer; `prefers-reduced-motion` açıksa animasyon oynamaz. Uygulama içinde "Tekrar oynat" görünmez (`?uygulama=1`); animasyon dosyaları tek başına tarayıcıda açılırsa görünür.
- **Logo: İkiz Kabartma** (iki kabartmalı kumaş pilesi): `web/index.html` içindeki `<symbol id="ikiz-kabartma">` (renkler vurgu renginden türer), `logo/logo.svg` (+ `logo.png`, `simge.ico`) ve `web/acilis/logo.js` (açılışlar). **Logoyu değiştirmek için yalnız bu üç yer güncellenir.** Eski logo: `logo/eski/`.
- Animasyonlar: tur 2 (zaman, gece, kalibrasyon, dikis, katlama), Opus (led, perde, kurgu, iki-ekran, isik) ve odak. Kaynaklar `tasarim/acilis-tur2/`; web sürümlerini `tasarim/acilis-tur2/uret_web_acilis.py` üretir.
- 0.9.6: Codex çıktısı üç açılış eklendi: zaman-sol (gpt-6.1-sol), gece-terra (gpt-6.1-terra), kalibrasyon-luna (gpt-6-luna); logoları İkiz Kabartma ile değiştirildi (`tasarim/acilis-tur2/uret_web_acilis2.py`).
- 0.9.7: açılış artık **karıştırılmış torba**: 14 animasyon karıştırılıp sırayla oynar, hepsi oynamadan hiçbiri tekrar gelmez, yeni turun ilki öncekinin sonuncusu olmaz (`localStorage taa-acilis-torba`). Ayrıntılar > **Program açılış animasyonları**: her açılış için aç/kapa + Önizle, Tümünü aç, Açılış animasyonunu tamamen kapat (`taa-acilis-ayar`); torba yalnız açık olanlarda döner, tek açıksa hep o, hiçbiri açık değilse program doğrudan açılır. Mantık testi: `node tests/torba_test.js` (cargo test de çalıştırır).
- 0.9.8: **kalıcı ayarlar**. Program her açılışta farklı portta çalıştığından localStorage ayarları (açılış ayarları ve torba, tema, renkler, şablon şeridi, önizleme görünümü, bina ölçüleri) kapat-aç arasında kayboluyordu. Artık `taa-` ile başlayan tüm localStorage anahtarları `%LOCALAPPDATA%\TAA Vidyo Yapiciyar.json` dosyasına yazılır (`GET/POST /ayar`; anahtar `taa-` + [a-z0-9-_], değer en çok 16 KB, en çok 64 anahtar, dosya en çok 256 KB; bozuk dosya = boş ayar; yazma atomik). Sayfa açılışta bu dosyadan geri yükler; sunucu okunamazsa localStorage'a, o da yoksa eski davranışa düşer. Başlangıç portu 47894'ten başlayıp sırayla denenir (47893 tek örnek kilididir). Dosya yolu `TAA_AYAR_DOSYASI` ortam değişkeniyle değiştirilebilir.
