---
description: "Yapay zekâ / Tarif et" özelliğinin tanısı (2026-11-18): 31 gerçek mağaza talebiyle deneme, bulunan zayıflıklar ve yapılan düzeltmeler.
sahip: Yusuf
yazan: "[Claude#AI Opus 5.5, yenileme-1.0, 2026-11-18]"
kural: Yalnız yazan yapay zekâ ya da Yusuf'un izniyle değişir; silme yok (yalnız Geri Dönüşüm Kutusu, Yusuf'un kararıyla).
---
# Tarif et tanısı (2026-11-18)

## Özet
Yusuf'un şikâyeti: "yapay zeka özelliği iyi çalışmıyor". Kural motoru (src/tarif.rs) 31 gerçek, teknik olmayan mağaza talebiyle denendi. Sorunun kökü: motor yalnız tırnak içi ya da BÜYÜK HARFLİ metinde iyi çalışıyordu; kullanıcının düz cümlesini (ör. "takım elbiselerde yüzde 20 indirim var") **reklam metnine çevirmiyor, sözcükleri olduğu gibi ekrana basıyor ya da önemli kısmı atıyordu**. Her sonuç tek sahne, "belir + altın" ile tekdüzeydi. Doğrudan bağlantıda yeniden deneme, iptal ve anlaşılır hata yoktu; boş tarifte hata dönüyordu. Düzeltmeler aşağıda; hepsi testlidir.

## Deneme yöntemi
- `tests/tarif_dene.rs` içinde `tani_yazdir` (elle çalıştırılır: `--ignored --nocapture`) ve kalıcı `gercek_magaza_talepleri` testi (31 örnek: indirim, bayram, açılış, taksit, çalışma saati, iki mesaj, renk/animasyon isteği, yazım hatalı, belirsiz, çok uzun, boş).
- Doğrudan bağlantı: sahte taşıyıcı (trait) ve 127.0.0.1 üzerinde sahte HTTP sunucusu; gerçek dış hizmete istek gönderilmedi.

## Önceki davranış: kötü sonuç veren örnekler ve nedeni
| Tarif | Eski sonuç | Neden |
|---|---|---|
| takım elbiselerde yüzde 20 indirim var | yalnız "%20 / İNDİRİM"; ürün atıldı | desen bulunca kalan sözcükler "anlaşılmayan" sayılıyor |
| Gömlek 499 TL | yalnız "TL" | "TL" BÜYÜK HARFLİ metin sanılıyordu (hata) |
| Sevgililer gününe özel %15 | yalnız "%15" | aynı neden |
| çalışma saatlerimiz 09:00 - 20:00 | "ÇALIŞMA SAATLERİMİZ 09/00 20 00" | ":" ve "-" ayırıcı sayılıyor, saat bozuluyor |
| hafta içi 9-19 cumartesi 10-18 açığız | tek sahnede 6 sözcük, saatler bozuk | saat aralığı tanınmıyor |
| yeni mağazamız açıldı herkesi bekleriz | tek sahnede 5 sözcük | reklam kurgusu yok, uzaktan okunmaz |
| indrim yap yüzde otuz | "İNDRİM YÜZDE OTUZ" | yazım hatası ve "otuz" sayısı tanınmıyor |
| zıplıyarak gelsin hoşgeldiniz | animasyon tanınmadı, "ZIPLIYARAK" ekrana yazıldı | ek/yazım hatası toleransı yok |
| dükkan bugün kapalı | "DÜKKAN BUGÜN" | "kapalı" yok sayıldı |
| pacman gibi bir animasyon olsun | ekrana "PACMAN ANİMASYON" | istek metin sanıldı |
| güzel bir şey yap / çok renkli olsun | "GÜZEL ŞEY" ekrana yazıldı | içeriksiz sözcükler metin sanıldı |
| ekranda saat ve tarih gözüksün | "SAAT TARİH GÖZÜKSÜN" | desteklenmeyen istek bildirilmedi |
| (boş) | hata: "metin bulunamadı… tırnak içinde yazın" | kullanıcıya teknik talimat; video yok |
| "bir sürü yazı" x80 | tek sahnede 16 kez "SÜRÜ" | yineleme ayıklanmıyor |
| indirim var kırmızı olsun yanıp sönsün | "İNDİRİM VAR YANIP" | "yanıp sön" kuralı yok |

Genel: tüm sonuçlar tek sahne, aynı giriş (belir) ve renk (altın); "Başka öneri" yok.

## Doğrudan LLM bağlantısı (src/llm.rs) zayıflıkları
1. Tek zaman aşımı (60 sn), bağlanma zaman aşımı ayrı değil; yeniden deneme yok.
2. İptal edilemiyordu.
3. Yanıt boyutu sessizce kırpılıyordu (256 KB'tan uzunu bozuk JSON gibi görünür).
4. Hata metinleri teknik ve sağlayıcının ham mesajını içeriyordu ("Sağlayıcı 401 kodu döndürdü: Incorrect API key …"); sağlayıcı anahtarın bir kısmını geri yazarsa sızıntı riski (yalnız tam eşleşme gizleniyordu).
5. Ağ hatası "Bağlanılamadı (Transport error …)" gibi İngilizce/teknik.
6. JSON kipi istenmiyordu (Gemini `responseMimeType` yok).
7. Test edilebilir taşıma katmanı yoktu (gerçek ağ olmadan hata yolları denenemiyordu).
Anahtar saklama (DPAPI, kullanıcı kapsamı, hedefe bağlı) sağlamdı; korundu.

## İstem kalitesi
Eski istem doğru bilgiyi içeriyordu ama uzun ve dağınıktı; "belirsizse soru sorma, varsayım yap", "satır başına en çok 10 harf", "3-6 sahne", "farklı animasyonlar" gibi okunurluk/kurgu kuralları net değildi. Yapıştırılan yanıt üst düzey dizi (`[{...}]`) ya da tek sahne nesnesi olunca reddediliyordu.

## Hata mesajlarının anlaşılırlığı
"JSON okunamadı: beklenmeyen karakter 'x' (konum 3)" gibi teknik metinler doğrudan kullanıcıya gidiyordu.

## Yapılan düzeltmeler (2026-11-18)
1. **Niyet şablonları** (`src/tarif/niyet.rs`): indirim (oran, ürün), taksit (sayı, kredi kartı/peşin), fiyat, bayram (Ramazan/Kurban/Şeker/Cumhuriyet, mübarek/kutlu), anneler/babalar/sevgililer/öğretmenler günü, yılbaşı, okul, açılış, çalışma saatleri (gün etiketli), kapalıyız (neden, bugün/yarın), yeni sezon/yeni gelenler, hizmetler (tadilat, ütü, paça…), karşılama, teşekkür, davet. Kısa satırlı (≤10 harf) 2-6 sahne + sonda işletme adı. Kullanıcının yazdığı renk/animasyon/efekt/süre/geçiş korunur.
2. **Varsayım, soru yok**: oran yoksa "BÜYÜK İNDİRİM", metin yoksa güvenli karşılama şablonu; hepsi `varsayimlar` listesinde gösterilir.
3. **Yazım hatası toleransı**: 5+ harfli sözcükte tek ünlü farkı ("zıplıyarak"→zıpla), 6+ harfte tek eksik harf ("indrim"→indirim). Ünsüz farkı kabul edilmez ("kapalı"≠"kapak").
4. **Hata düzeltmeleri**: "TL/KDV/XL" büyük harfli metin sayılmaz; saat aralıkları korunur; yinelenen metin sadeleştirilir; uzun metin 3 sözcüklük sahnelere bölünür.
5. **Yedek zinciri**: `POST /tarif/hazirla` → (izin varsa) yapay zekâ → katı şema → kural/niyet → şablon. Her zaman geçerli senaryo; `kaynak: yapay-zeka | kural | sablon`.
6. **LLM bağlantısı**: `Tasiyici` arayüzü, bağlanma 10 sn / toplam 45 sn, en çok 2 yeniden deneme (1 sn, 2 sn), iptal (`POST /tarif/iptal`), boyut sınırı aşımında açık hata, sade Türkçe hata kodları (sağlayıcı metni ve sır yok), Gemini JSON kipi.
7. **İstem** sadeleşti ve sağlamlaştı; yapıştırılan yanıtta dizi/tek sahne toleransı; `ozet` alanında tek cümlelik sade hata.
8. **Kurallar**: "yanıp sön" → nabiz, "kayarak/soldan sağa" → kayar-sol, "renkli/rengarenk" → gun-batimi (yalnız ekleme).
9. **web/ai.js**: tek "Hazırla" düğmeli yeni panel (bkz. docs/TARIF-API.md).

## Kalan sınırlar
- Canlı saat/tarih gösterimi yok (not olarak bildiriliyor).
- "pacman" gibi katalogda olmayan animasyon adı: not + benzer hareket. Malzeme ajanı kataloğa eklerse otomatik kural adı kendiliğinden tanır.
- Ürün adı çıkarımı sezgiseldir (bulunma ekli sözcük ve öncesi); karmaşık cümlelerde ürün atlanabilir, metin yine geçerli olur.
