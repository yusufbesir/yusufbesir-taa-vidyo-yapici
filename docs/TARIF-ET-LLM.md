---
description: Vidyo Yapıcı "Tarif et" özelliği ana brifi (LLM istem üretici + çevrimdışı kural ayrıştırıcı + anlat akışı + arayüz/logo yenileme sırası). Sahip Yusuf; yazan [Claude#ebd691, 2026-11-16]. Kod yazılırken güncellenir; silme yok.
---
# Tarif et (0.9.0) ana brif
Kaynak istekler (Yusuf, 2026-11-16, koordinatör aktarımıyla): kullanıcı videoyu TARİF eder, program tarife göre sahne listesi çıkarır, önizleme ve tek tık onayla video üretilir. İki yol, TEK şema: (A) dış LLM'e istem + yanıtı geri alma (ChatGPT, Gemini, DeepSeek, Claude, yeni eklenebilir), (B) çevrimdışı kural tabanlı Türkçe ayrıştırıcı. İsteğe bağlı doğrudan LLM bağlantısı varsayılan KAPALI. "Anlat" akışı Tutaa'dakiyle tek özellik (ayrı ikinci ekran yok).

## Aşamalar
0.8.2 (bitti, yerel) -> 0.9.0 Tarif et (LLM + çevrimdışı + anlat) -> 0.9.1 arayüz ve logo yenileme -> araştırma raporları. İTME/BİRLEŞTİRME YOK (Yusuf onayı). Silme yok.

## 1. Ana ekran akışı (en çok 3 tık)
Ana ekranda "Tarif et" düğmesi (1 tık) -> tarif kutusu: kullanıcı yazar -> "İstemi kopyala" (2) -> LLM sitesinde yapıştır, yanıtı kopyala -> programa dönünce "Panodaki tarifi uygula" (3) -> önizleme -> "Videoyu yap". Çevrimdışı: tarif kutusu -> "Kuraldan çöz" -> önizleme. Düğmeler 48 px, ana eylem 56 px.

## 2. Katalogdan istem üretici
Program kendi GERÇEK katalog adlarını (giriş/çıkış animasyonları, geçişler, efektler, açılış/kapanış, palet, font, dekor) /bilgi verisinden okuyup istem üretir; katalog büyüdükçe istem otomatik güncellenir. İstem: "Yalnız bu listeden seç, yalnız JSON döndür, kod yok", sahne listesi şeması ve örnek. Sır ve müşteri verisi istemde yok.
LLM düğmeleri: ChatGPT / Gemini / DeepSeek / Claude sitelerini varsayılan tarayıcıda AÇAR (yalnız dış site; programın kendisi tarayıcıda çalışmaz). Adresler Ayarlar'da düzenlenebilir listede, yeni LLM eklenebilir.

## 3. Sahne listesi şeması (iki yol aynı şemayı kullanır)
```
{ "surum": 1, "ekran": "dikey", "gorunum": "duz|bina",
  "sahneler": [ { "metin": ["SATIR1","SATIR2"], "giris": "<katalog>", "cikis": "<katalog>", "efekt": ["<katalog>"],
                  "palet": "<katalog>", "font": "<katalog>", "sure": 4.5 } ],
  "acilis": "<geçiş>", "kapanis": "<geçiş>" }
```
Sınırlar: en çok 12 sahne, sahne başına en çok 6 satır, satır en çok 40 karakter, sahne süresi 1-15 sn, toplam en çok 120 sn.

## 4. Yanıtı otomatik alma
Yalnız program penceresi odaktayken ve kullanıcı "Tarif et" ekranındayken pano okunur (pano olayında); JSON bulunur: kod çiti ```json, öncesi/sonrası metin, Türkçe karakter ve tırnak sorunlarına toleranslı. "Panodaki tarifi uygula" düğmesi çıkar; yapıştırma kutusu da vardır. Pano hiçbir yerde saklanmaz/günlüğe yazılmaz.

## 5. Doğrulama (güvenlik, zorunlu)
Sıkı şema; bilinmeyen animasyon/geçiş adı reddedilmez, en yakın katalog adına eşlenir ve kullanıcıya gösterilir; metin/sahne/süre sınırları; HTML, komut, dosya yolu, URL içeren metin temizlenir; hiçbir şey çalıştırılmaz; okunabilirlik denetimi (harf boyutu, süre, kontrast) devrede. Hata: okunur Türkçe açıklama + "Düzelt istemi" (LLM'ye geri yapıştırılacak metin). Onaysız video üretilmez.

## 6. Doğrudan LLM bağlantısı (isteğe bağlı, varsayılan KAPALI)
Ayarlar'da sağlayıcı (OpenAI uyumlu: ChatGPT/DeepSeek/diğer; Gemini; Anthropic) + anahtar. Anahtar dosyaya DÜZ yazılmaz: Windows Kimlik Bilgisi Yöneticisi ya da DPAPI ile şifreli; günlüğe yazılmaz; arayüze geri gösterilmez. Yalnız tarif metni ve katalog gider; ilk kullanımda "şu metin gönderilecek" onayı; zaman aşımı, hata ve çevrimdışında otomatik kopyala-yapıştır yoluna düşer. Yeni bağımlılık minimum (ureq/rustls ya da mevcut). Bu bölüm güvenlik açısından ayrı inceleme ister (guvenlik-denetcisi) ve anahtar yönetimi Yusuf kararıdır; ilk uygulamada yalnız arayüz iskeleti ve kapalı varsayılan gelir.

## 7. Çevrimdışı kural listesi (detaylı)
Veri odaklı kural dosyası: kurallar/tarif-kurallari.json (programa gömülü varsayılan + kullanıcı kuralları yanına eklenir; yönetici/kullanıcı düzenler). Kapsam en az 300 Türkçe ifade/kök, ek ve çekim toleranslı (zıpla/zıplayarak/zıplayan/zıplatarak), ı/i ş/s ASCII toleransı, büyük/küçük harf. Kategoriler: animasyon giriş/çıkış, yön, geçiş (ffmpeg xfade adları dahil), efekt, renk (en az 40 ad ve niteleme), font/stil, süre ("iki saniye", "2 sn", "yarım saniye", kısa/uzun/hızlı/yavaş), sıra bağlaçları (önce, sonra, ardından, en son, aynı anda, arada), metin çıkarma (tırnak içi, BÜYÜK HARFLİ metin, %30, ₺ tutar, "3 taksit" desenleri), şablon ipuçları, ekran modu (bina, dikey). Belirsiz kelimede SORMAZ: önizlemede "şunu anladım" listesi gösterir, anlaşılmayan kelimeleri vurgular, "Bunu kural olarak öğret" düğmesiyle kalıcı yeni eşleme eklenir. Her kural bir katalog adına bağlıdır; katalogda olmayan ad kural dosyası doğrulamasında reddedilir (CI testi). En az 40 örnek tarif -> beklenen sahne listesi (altın test) cargo test'te çalışır. Kural listesi düz Türkçe belgeye de dökülür: docs/TARIF-KURALLARI.md; Ayarlar > "Kural listesi" ekranında arama kutulu liste.

## 8. "Anlat" akışı birleşimi
Önce D:\Programlar\Tutaa içinde (yalnız okunur) yapay zekâya yapıştırılacak "anlat" özelliği incelenir (arayüz akışı, istem üretimi, kopyalama, yanıtı geri alma, dil/ton); aynı kullanıcı deneyimi ve dil Vidyo "Tarif et" ile birleştirilir; ortak parça TAA Arayuz Ortak'a konur; Tutaa'dakinden iyi/eksik farklar durum dosyasına yazılır.

## 9. Test
Şema doğrulama birim testleri (bozuk JSON, kod çiti, bilinmeyen ad, aşırı uzun metin, HTML enjeksiyon denemesi), 5 örnek tarif/yanıt çifti, 40 altın örnek; arayüz kontrolü: Tarif et ekranı en çok 3 tık, düğme 48/56 px; cargo test geçmeli.

## 10. Arayüz ve logo yenileme (0.9.1, ayrı commit)
GPT + Gemini ortak (sırsız brif, kalıcı izinle); brif ve yanıtlar tasarim\ortak-gpt-gemini\. Hedef: profesyonel, sade, şatafatsız, bol boşluk, iş aracı hissi ("çalışma masası"); mevcut özellikler kaybolmaz; varsayılan tema sade koyu-nötr ve açık-nötr iki seçenek, parlak amber/neon/gradyan kalkar; tek aile tipografi; çizgi ikon. Logo: video/geçiş/zaman çizelgesi/kare kare fikri, düz tek renk + tek vurgu, 16/32/256 px ve .ico, SVG kaynağı; Gemini ile en az 4 aday, GPT eleştirir, Claude seçer, adaylar Artifact sayfasında; Yusuf seçer, o zamana dek en iyi aday varsayılan; eski logo eski\ klasörüne taşınır (silme yok). Stitch tasarım turu.

## Açık kararlar (Yusuf)
Doğrudan LLM bağlantısında anahtar saklama yöntemi (Credential Manager önerilir), pano okuma izni ilk kullanımda onay, hangi LLM sitelerinin varsayılan listede olacağı.
