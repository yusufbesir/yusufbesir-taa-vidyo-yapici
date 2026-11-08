---
description: TAA Vidyo Yapıcı menü/arayüz tasarım araştırması ve yeni içerik önerileri (şablon, çerçeve, animasyon, geçiş, font, renk). Öneri notudur, kodu değiştirmez.
sahip: Yusuf; yazan: Claude (2026-11-08)
---
# Menü / arayüz araştırması
Kaynak kalitesi: arama sonuçları çoğunlukla satıcı sayfaları ve katalog yazılarıdır; bulgular yön gösterir, kesin kanıt sayılmaz.

## Bulgular
1. **Kademeli açma (progressive disclosure):** Az özellikle başlayıp gelişmiş olanı "Ayrıntılar" altına koymak, karmaşıklığı düşüren yaygın kalıp. Önemli koşul: gelişmiş özelliğin açılma düğmesi açık görünmeli.
2. **Şablon-önce akış:** Shotstack, CE.SDK, Wave.video gibi araçlar boş sayfa yerine hazır şablonla başlatıyor; kullanıcı yalnız metni ve rengi değiştiriyor.
3. **Dijital tabela araçları (Yodeck, Rise Vision):** yüzlerce hazır şablon (800+ / 600–750), sektöre göre sınıflı; tek bölgeli ve çok bölgeli yerleşimler; zamanlama; kurumsal planda yazı tipi, renk ve logo kilitli, düzenleyici yalnız izin verilen alanı değiştirir.
4. **Sahne bölme (PlayPlay):** uzun işi küçük sahnelere bölmek karışıklığı azaltır. Bizde "mesaj" = sahne.

## Mevcut menüdeki sorun (ekran görüntüsü, 0.5.0)
Sol sütunda aynı anda: metin, 5 tarz, şaşırt, öneriler, 12 palet, 12 font, derinlik, 122 animasyon, giriş/çıkış/açılış/kapanış, efekt çipleri, üretim, senaryo kutusu. Hepsi hep açık; yeni kullanıcı hangisinin gerekli olduğunu ayıramıyor.

## Öneri: 3 adım + Ayrıntılar
1. **Ne yazsın?** büyük metin kutusu (+ hazır şablon seç düğmesi).
2. **Nasıl görünsün?** 5 tarz kartı + "Şaşırt beni"; 6 renk dairesi. Başka her şey kapalı.
3. **Hazır:** canlı önizleme (tasarım | ekranda 3 m/15 m), tek "Videoyu yap", ilerleme, "Klasörü aç".
- **Ayrıntılar ▾** (varsayılan kapalı): animasyon galerisi, font, derinlik, açılış/kapanış, geçiş, çerçeve, efekt, gelişmiş senaryo.
- Okunabilirlik uyarısı: önizlemenin üstünde tek satır, yeşil (iyi) / sarı (uyarı); ayrıntı tıklanınca açılır.
- Kullanıcının en son seçtikleri hatırlansın; "Sıfırla" tek düğme.
- Klavye: Ctrl+Enter = videoyu yap; boşluk = önizlemeyi oynat/durdur.

## Yeni içerik önerileri (kod değişikliği değil; kodun sahibi oturum karar verir)
Kullanıcı isteği: hazır tasarım şablonları, çerçeveler, yeni animasyonlar, geçişler, fontlar, yazı renkleri.
- **Hazır şablonlar (sektöre göre, 15–25 adet):** İndirim/kampanya, Taksit, Yeni ürün, Açılış, Menü/fiyat, Duyuru/saat, Bayram/kutlama, Randevu/çalışma saati, Emlak, Araç, Eczane/nöbetçi, Düğün/organizasyon. Her şablon: metin yer tutucusu + palet + tarz + animasyon + geçiş + çerçeve paketi. Senaryo dosyası (txt) olarak saklanır, kullanıcı kendi şablonunu da kaydedebilir.
- **Çerçeveler:** ince çizgi, çift çizgi, köşe işaretleri, LED ışık yürüyen çerçeve (chaser), neon, şerit/bant, yıldız/ışıltı köşe, altın kenar. Kenar payı LED güvenli alanına uyumlu.
- **Geçişler:** araştırmadaki ortak set — çapraz solma, itme (4 yön), kaydırma (4 yön), silme (4 yön + barn-door), daire/iris, saat, panjur, damalı, spiral, perde. Maske tabanlı yapı (gri tonlu PNG) ile sınırsız ek.
- **Yeni animasyonlar:** (araştırmadan) dalga harf, Elastic/Bounce giriş, daktilo, sayaç/geri sayım, piksel dağılma, glitch (güvenlik sınırlı), nefes alan parlaklık, zaman döngüsü, kar/yağmur/ışıltı arka planı, plazma/gradyan akışı, kayan manşet.
- **Fontlar:** mevcut 12'ye ek, LED'de kalın ve net: yoğun sans (Bebas Neue, Oswald, Anton benzeri), geniş display (Archivo Black, Montserrat ExtraBold), köşeli/teknolojik (Orbitron, Rajdhani), el yazısı vurgu (yalnız tek satır). Türkçe karakter (İ, ı, Ş, Ğ) desteği zorunlu; lisans kontrolü (OFL).
- **Yazı renkleri:** palet dışında tek tıkla "yazı rengi" seçici; LED için doygun, kontrast sınırı olan 12–16 renk + degrade yazı (altın, gökkuşağı, buz); koyu zeminde WCAG kontrast oranı otomatik uyarı.
- **Güvenlik:** photosensitivity sınırı (saniyede en fazla 3 parlama) tüm efekt ve animasyonlarda varsayılan açık.
- **Önceliklendirme:** (1) menü sadeleştirme, (2) şablonlar, (3) geçişler, (4) çerçeveler, (5) font ve renk, (6) yeni animasyonlar.

Not: Kaynak kodu başka bir oturumun işi olduğu için ben eklemedim; yukarıdaki liste o oturuma ve Yusuf'a öneridir.
