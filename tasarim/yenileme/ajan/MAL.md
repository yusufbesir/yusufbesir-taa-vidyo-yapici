---
description: Malzeme ajanı görevi (dalga 3). Sahip: Yusuf. Yazan: [Claude#Sonnet-5.5 koordinatör, 2026-11-25]. Silme yok.
---
# Malzeme ajanı
Sahip olduğun: src/malzeme*.rs, src/senaryo.rs, src/anim*.rs, src/efekt.rs, src/dekor.rs, src/cizim.rs, src/uret.rs, src/oto.rs, web/malzeme.js, tests/malzeme*, docs/MALZEME.md, senaryolar/*, animasyonlar/*.

1) Fable bulguları (önce bunlar):
- KRİTİK: malzeme kartına tıklamak kullanıcının senaryosunu SİLİYOR. malzeme.js önizlemeyi senaryoya dokunmadan göstersin: TaaApp.geciciOnizle(metin) (UI ajanı ekliyor; yoksa /malzeme/kucuk karesiyle önizleme). "Videoya ekle" tek kez eklesin; görünür geri bildirim + geri al.
- .tm-ana ve seçili kategori çipi dolu vurgu olmasın (çerçeveli vurgu); çip/kartlar temanın --r-btn/--r-kart köşesini izlesin.
- Yutkun Pac-Man'e çok benziyor (sarı kama ağızlı top, kubbe başlı dalgalı etekli hayaletler, mavi çizgili koridor, SKOR): silüetleri özgünleştir (kare kafalı/yuvarlak ayaklı robot-jöle kovalayıcılar; Yutkun'un farklı renk/ağız biçimi; dolu zemin şeritli koridor; "SKOR"→"PUAN"); mantık kalsın. Karakter ölçeği ekran genişliğinin ≥%12-15'i (üst/alt üçte bir boş kalmasın).
- "İNDİRİM" vb. rozetlerde koyu yazı LED'de okunmuyor: açık renk/kalın dış çizgi. 53 malzemeyi 3 m LED benzetimiyle gözden geçir, zayıfları düzelt.
- oto.rs: malzeme sahnelerine "satır başına 8-10 harf" önerisi çıkmasın. Katalog etiketlerine eş anlamlı arama sözcükleri (pacman, oyun, labirent, kovalamaca, yemek yiyen).

2) YENİ PAKET. Karar dosyası: tasarim/yenileme/karar-animasyon-paketi.md (Fable yazıyor; 10-12 dk içinde belirir; yoksa ekip önerileriyle ilerle):
- KIŞ/YILBAŞI: kar yağışı (çok katmanlı, rüzgârlı, birikmeli kar yığını), çam ağacı + süsler + yıldız + ışık dizisi, havai fişek çeşitleri (kabarcık, söğüt, halka, kalp, çift patlama; yılbaşı gecesi sahnesi), yılbaşı geri sayımı, hediye kutuları, kardan adam, kış manzarası, buz kristalleri, "2027" yazısı.
- PREMIUM AĞIR EFEKTLER (hafif kaçmasın; parlama/neon değil derinlik, gölge, doku): parçacık, ışık süpürmesi, yansıma, cam/buz dokusu, kumaş dokusu, duman.
- 3B/2B YAZI ANİMASYONLARI: ekstrüde 3B harf döndürme (yazılım rasterleştirici), derinlik/perspektif, kinetik tipografi, dalga, parlak süpürme, kumaş ipi/dikiş yazı.
Hepsi mevcut motordan geçip önizlemede VE MP4'te görünsün; her biri için otomatik kare testi (boş değil, deterministik, doğru boyut). Hedef katalog ≥110 malzeme. Örnek MP4+kare: tasarim/yenileme/ornek/kis-yilbasi.mp4 ve 3b-yazi.mp4. Performansı ölç, raporla; docs/MALZEME.md güncelle.
