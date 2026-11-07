# LED Ekran Video (TAA Paket programı, tek bilgisayarda kullanılır)

Bina dışı dikey LED ekranlar (Yusuf’un ekranı 950x1800, 9:16) için 3B yazı, parçacık ve geçişli, karanlıkta okunur MP4/HAP video üretir. Videolar bir **senaryo dosyası** ile tanımlanır.

## Kurulum (Windows)
`LED-Ekran-Video-Kurulum.exe` çalıştırılır: `D:\Programlar\LED-Ekran-Video\` klasörüne kurar (D: yoksa `%LOCALAPPDATA%\Programs`), masaüstü ve Başlat menüsü kısayolu ile Uygulamalar listesine kayıt ekler, `kaldir.exe` yazar. `--sessiz` sorusuz kurar, `--hedef KLASÖR` klasörü değiştirir, `--kisayolsuz` kısayol ve kayıt yazmaz.
Kaldırmak için Ayarlar > Uygulamalar'dan veya `kaldir.exe` ile; yalnız kendi dosyaları silinir, videolara ve kullanıcının eklediği senaryolara dokunulmaz.
**ffmpeg** programla gelmez; `ffmpeg.exe` programın klasörüne (veya altındaki `ffmpeg\bin`), `D:\Programlar\Araçlar Ve İndirilenler` altına konur ya da `LED_FFMPEG` ortam değişkeniyle yolu verilir; yoksa PATH aranır.

## Arayüz
`led-ekran-video.exe` argümansız çalışınca (veya kısayolla) yerel arayüz tarayıcıda açılır; yalnız bu bilgisayardan erişilir (127.0.0.1). Senaryo yazılır, örnekler yüklenir, "Sahne ekle" formu satır yazar, ekran oranında önizleme ve zaman çubuğu vardır; video `Videolar\LED-Ekran-Video` klasörüne MP4 veya HAP olarak üretilir (`LED_CIKTI` klasörü değiştirir). Sekme kapanınca program kendiliğinden kapanır; ayrıca "Kapat" düğmesi vardır. HAP için ekran ölçüsü 4'e bölünmelidir (950x1800 uygun değildir, MP4 kullanın).

## Senaryo ile video
```
led-ekran-video --senaryo senaryolar/tv-afis.txt --cikis kampanya.mp4
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
- **giris:** yok, donerek, zipla, yandan, zoom, alttan, ustten, belir (yavaşça belirir), elastik (yaylanarak büyür), daktilo (harf harf yazılır), sil (soldan sağa açılır), toplan (parçacıklardan toplanır).
- **cikis:** yok, yukari, asagi, donerek, sondur, kucul (dönerek küçülür), sil (soldan sağa silinir), dagil (parçacıklara dağılır).
- **efekt:** parlama (yazıdan geçen ışık), pirilti (yıldızlar), cerceve (koşan ışıklı çerçeve), cift-cerceve (çift çizgi çerçeve), parcacik (süzülen parçacıklar), kivilcim (uçan kıvılcımlar); `efekt=yok` hepsini kapatır.
- **Düzen:** `sahne "PEŞİN|FİYATINA|*3|TAKSİT"`: `|` açık satır ayırır, `*` ile başlayan satır büyük rakamdır ve yanlarına çift çizgi gelir. `|` yoksa kelimeler alt alta dizilir.
- **Çizimler:** `@cicek`, `@papatya`, `@yildiz`, `@rozet`, `@cerceve`, `@elmas` (varsayılan senaryolarda yok, yalnızca istenirse eklenir; bkz. senaryolar/ornek-cicekli.txt).
- Hata olursa satır numarasıyla Türkçe mesaj verir.
- `--genislik/--yukseklik/--fps` senaryodaki değeri geçersiz kılar (genişlik çift sayı olmalı), `--duz` fiziksel düzeltmeyi kapatır, `--kodek hap` HAP üretir.

## Fiziksel ekran ölçüsü
`fiziksel 8x25` (metre, genişlik x yükseklik) verilirse program gerçek ekran oranında (kare piksel) çizer ve çıkışta dikeyde piksel yüksekliğine indirir. Örnek: 8 m / 950 px = 8,4 mm, 25 m / 1800 px = 13,9 mm; piksel oranı 1,65. Düzeltme olmazsa yazı ve çizimler ekranda 1,7 kat gerilir. Resolume’da kompozisyon 950x1800 olmalı ve video ölçeksiz oynatılmalı.

## Örnek senaryolar
`senaryolar/`: tv-referans, tv-afis, ornek-gibi, toplan-dagil, daktilo-kivilcim, elastik-parcacik, tv-klasik, tv-iki-mesaj, perde-hizli, ornek-cicekli.

## Eski hızlı modlar
`--mod sabit|kayan|3d|tv` ile tek komutta video (bkz. `--yardim`).

## Tasarım ilkeleri
ARASTIRMA-DIS-LED-EKRAN.md: beyaz kısık, flaş yok, tarama çizgisi yok, kalın font, 10-15 sn döngü.

## Derleme (Linux'tan Windows exe)
`./derle-windows.sh` (gerekenler: `rustup target add x86_64-pc-windows-gnu`, `pip install ziglang`, `cargo install cargo-zigbuild`) `dist/` altına `led-ekran-video.exe` ve `LED-Ekran-Video-Kurulum.exe` üretir. Simge `logo/simge.ico` (kısayollarda kullanılır; exe dosyasına gömülü değildir).

## Durum
Çekirdek, senaryo motoru, arayüz, kurulum ve kaldırma hazır (24 test; Linux'ta arayüz ve kurulum mantığı denendi). Render CPU'da çalışır (10 sn video yaklaşık 2,5-3 dk). Windows exe'nin kendisi bir Windows bilgisayarda henüz denenmedi.
