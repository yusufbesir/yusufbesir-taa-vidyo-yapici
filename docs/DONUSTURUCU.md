---
description: TAA Vidyo Yapıcı Dönüştürücü belgesi (videoyu/resmi çizgi film, piksel art vb. görünüme çevirme; uçlar, türler, güvenlik, testler).
sahip: Yusuf
yazan: "[Claude#Dönüştürücü Opus 5.5, yenileme-1.0, 2026-11-25]"
kural: Yalnız yazan yapay zekâ ya da Yusuf'un izniyle değişir; silme yok.
---
# Dönüştürücü

## Özet
Video ya da resmi LED ekran için 950x1800 dikey MP4'e çevirir: çizgi film, çizgi roman (halftone), boyama, piksel art,
retro 8-bit, duotone, karakalem, kontur ve piksel giriş animasyonu. İşi ffmpeg yapar; program ffmpeg'i yalnız argüman
dizisiyle çağırır (kabuk yok). Kod: `src/donustur.rs` (filtre zincirleri, doğrulama, iş), `src/donustur_http.rs` (uçlar),
`web/donustur.js` (arayüz, `window.TaaDonustur.mount(el, app)`).

## Türler
| id | Görünüm | Filtre özeti | Kullanılan ayarlar |
|---|---|---|---|
| cizgifilm | Kalın kontur, az renk | bilateral yumuşatma + posterize (10..4 seviye) + edgedetect mürekkep çizgisi (çarpma) | yoğunluk |
| cizgiroman | Mürekkep + halftone | posterize 4 + geq nokta deseni (hücre 8..16 px) + kalın kontur | yoğunluk |
| boyama | Yağlı boya | median (yarıçap 2..7) + smartblur + unsharp + doygunluk | yoğunluk |
| piksel | Piksel art | küçült (blok) + palet (otomatik ya da sabit) + dithering seçenekli + keskin büyüt | palet, renk, dither, blok |
| retro8bit | 8-bit oyun | piksel + varsayılan Retro 16 palet + piksel ızgarası + en çok 15 fps | palet, renk, dither, blok |
| duotone | İki ton afiş | gri + iki ton arası doğrusal eşleme | ton |
| karakalem | Kurşun kalem | gri + dodge eskiz; kâğıt beyazı kısık sıcak tona çekilir (LED'de göz kamaştırmasın) | yoğunluk |
| kontur | Koyu zeminde çizgi | edgedetect + ton rengi | yoğunluk, ton |
| piksel-giris | Netleşen giriş | blok boyutu zamanla 50→1 px (sürenin %80'inde); resim için ideal | yoğunluk, süre |

Ortak ayarlar: `sigdir` (kirp: ekranı doldurur, kenar kırpılır · sigdir: siyah kenarla sığar · doldur: bulanık koyu arka planla sığar),
`sure` (0 = tamamı; resimde 5 sn; en çok 600), `baslangic` (sn), `fps` (1..60), `yogunluk` (0..100).
Piksel blokları yalnız 950 ve 1800'ü tam bölen 2, 5, 10, 25, 50 px'tir (kare pikseller eşit kalır; 10 px → 95x180 ızgara).
Paletler (neon yok): otomatik, Retro 16, Game Boy yeşili, sepya, kehribar, lacivert-altın, gri. Tonlar: lacivert-altın,
bordo-krem, petrol-şeftali, grafit-kehribar, orman-krem. Çıktı: H.264 MP4, yuv420p, CRF 16, sessiz.

## Uçlar (arayüz `/donustur/` önekini `donustur_http::isle` işlevine yollar)
İmza: `isle(yontem, yol, govde, klasor) -> (kod, tür, gövde)`; `yol` önek sonrası, sorgu dizesi yok; `klasor` = çıktı klasörü.
| Yöntem | Yol | Gövde | Yanıt |
|---|---|---|---|
| GET | turler | — | türler, paletler, tonlar, bloklar, ffmpeg var mı |
| GET | kaynaklar | — | izinli köklerdeki video/resim dosyaları (yeniden eskiye, en çok 300) |
| POST | onizle | {kaynak, tur, ..., t, kucuk} | `{"tamam":true,"png":"data:image/png;base64,..."}` (950x1800; kucuk=true ise 190x360) |
| POST | baslat | {kaynak, tur, ...} | `{"tamam":true,"dosya":"..."}`; zaten iş varsa 409 |
| GET | durum | — | calisiyor, kare, toplam, yuzde, asama, dosya, mesaj, hata |
| POST | iptal | — | ffmpeg süreci durdurulur |
| POST | oynat | — | son çıktıyı varsayılan oynatıcıda açar (yol yalnız sunucu kaydından) |
| POST | klasor | — | çıktı klasörünü açar |
| GET | donustur.js | — | arayüz betiği |
Hata: `{"tamam":false,"hata":"Türkçe mesaj"}` (400 bozuk istek/doğrulama, 404, 409, 500 ffmpeg hatası).

İş akışı: otomatik palette önce kaynaktan palet çıkarılır (1. geçiş, bellek dostu), sonra dönüştürülür. Çıktı önce geçici
klasöre yazılır, başarıda çıktı klasörüne `<kaynak>-<tür>.mp4` adıyla taşınır; aynı ad varsa `-2`, `-3` eklenir (hiçbir dosya ezilmez).
İptalde ya da hatada yalnız programın kendi geçici dosyası kaldırılır; çıktı klasörüne yarım dosya bırakılmaz.

## Güvenlik
- ffmpeg her zaman `Command::args` dizisiyle; kabuk, `cmd /c` yok. Dosya/klasör açma `explorer <yol>` tek argümanla.
- Filtre zincirine yalnız doğrulanmış sayılar ve sabit listeden adlar girer; serbest metin girmez.
- Kaynak: mutlak yol, `canonicalize` sonrası izinli kök altında (çıktı klasörü, Videolar, Resimler, Masaüstü, İndirilenler),
  yalnız video (mp4 mov mkv webm avi m4v gif, en çok 4 GB) ya da resim (png jpg jpeg bmp webp, en çok 50 MB) uzantısı; `..` ile yol geçişi reddedilir.
- İstek gövdesi en çok 8 KB; düz JSON nesnesi dışındaki her şey 400.
- ffmpeg yoksa Türkçe mesaj (`ffmpeg::YOK_MESAJI`); arama: LED_FFMPEG, program klasörü, `D:\Programlar\Araçlar Ve İndirilenler`, PATH.

## Arayüz (web/donustur.js)
Kaynak listesi (arama), tür kartları (seçilen kaynaktan sırayla küçük önizleme), türe göre ayarlar (sürgü/seçim),
önizleme, Dönüştür / İptal, ilerleme çubuğu (transform ile), bitince Oynat / Klasörü aç / Oynatıcıya ekle.
Oynatıcıya ekle: `app.oynaticiyaEkle(yol)` varsa çağrılır ve `document` üzerinde `taa:oynaticiya-ekle` olayı
(`detail.dosya`) gönderilir. Hedefler ≥48 px (ana eylem 56), yazı ≥16 px (ikincil 13-14), yalnız tema değişkenleri, görünür odak.

## Testler
- `cargo test --lib donustur`: ayar doğrulama, filtre zinciri üretimi (her tür × sığdırma × yoğunluk), posterize sınırı,
  palet PNG, süre ayıklama, argüman dizisi, yol güvenliği, JSON çözücü, bozuk istekler.
- `tests/donustur_testleri.rs` (ffmpeg yoksa gerçek testler atlanır): her tür için gerçek PNG önizleme, örnek dönüşümler
  (`tasarim/yenileme/ornek/donusum-*.mp4` + `-kare.png`), resimden piksel giriş, başlat/durum/409/iptal, kaynak güvenliği, Node testi.
- `node tests/donustur_test.js`: web/donustur.js saf mantığı.
Ölçüm (yutkun.mp4, 950x1800, bu makine): önizleme 0,15-0,8 sn; 3 sn çizgi film 3-4 sn, piksel 1-2 sn sürer.
