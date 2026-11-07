# Dış LED ekran içerik araştırması (2026-11-07)

Kaynaklar vendör/ajans blogu; sayılar kural-ı başparmak, standart değil. Sahada denenmeli.

## Bulgular ve uygulanışı
| Bulgu | Kaynak | Programda |
|---|---|---|
| Mesaj 5-8 sn okunur, en çok ~7 kelime | [Blue Line Media](https://www.bluelinemedia.com/creative-tips), [Designshifu](https://www.designshifu.com/blog/digital-billboard-design-principles) | Her sahne 4,5 sn; mesajlar 3-4 kelime |
| Harf boyu: ~1 inç / 3 m (10 ft) civarı rehber; geniş piksel aralığında büyük yazı, basit grafik | [Impact LED](https://impactled.com/content-creation-for-led/), [LEDCraft](https://www.ledcraftinc.com/4-important-criteria-to-select-fonts-for-outdoor-advertising/) | Her satır genişliğe sığar (%84), en az 48 px uyarısı |
| Saf beyaz LED'de projektör gibi parlar, gece çevre yazıyı bastırır | [Socast DOOH](https://support.socastdigital.com/portal/en/kb/articles/dooh-billboard-ad-specifications-and-design-guidelines) | Beyaz palet kısıldı (214,214,208); TV açılış parlaması %100 yerine %60 |
| Kontrast en az 7:1, koyu zemin üstüne açık yazı; ince koyu kontur iyi | Socast, [Designshifu](https://www.designshifu.com/blog/digital-billboard-design-principles) | Siyah/çok koyu zemin, ekstrüzyon yan yüzü koyu kontur işlevi görür |
| Flaş/strobe yok; kısa, sade, akıcı hareket serbest; tek öğe hareketi iyi | [Socast](https://support.socastdigital.com/portal/en/kb/articles/dooh-billboard-ad-specifications-and-design-guidelines), [DOOH Marketing](https://doohmarketing.com/tips/creative-best-practices) | Yanıp sönme yok; pırıltılar küçük ve yavaş |
| Bazı kaynaklar: yavaş soldurma LED'de arıza gibi görünür; geçiş net olsun | [HostAd 2026](https://hostad.io/en/blog/dyzayn-kreatyvu-dlya-dooh-2026) | Geçişler soldurma değil kayma/çizgi; kapanış keskin |
| Döngü 10-15 sn ideal | Socast, DOOH Marketing | TV videosu 11,9 sn |
| İnce font geniş piksel aralığında dağılır, %5-15 harf aralığı ekle | [Muxwave](https://www.muxwave.com/how-to-design-content-for-transparent-led-film/) | Kalın font, %4 harf aralığı |
| İçeriği ekranın gerçek piksel boyutunda üret; oran değil piksel | [Pixelight](https://pixelight.fr/en/blog/en-led-screen-video-content-formats-resolution-2026) | 980x1800 varsayılan, ayarlanabilir |
| Yol ekranlarında tam video kısıtlı olabilir | Socast | Bina cephesi ise sorun değil; konum kurallarını Yusuf bilir |

## Bu çalışmada yapılan değişiklik
- Tarama çizgileri kaldırıldı: 3 px'lik çizgi, 10-17 mm pikselli ekranda düzensiz parlaklık (moiré) yapar.
- Köşe kararması azaltıldı (kenarda kontrast düşmesin).
- Beyaz kısıldı, parlama %60.

## Açık risk
Fiziksel boyut: ekranlar dikey ve 980x1800 ise piksel oranı 9:16 (0,54). Gerçek ekran 10x30 m (1:3) ise pikseller kare değildir ve yazı gerilmiş görünür. Gerçek en x boy (m) öğrenilmeli.
