---
description: TAA Vidyo Yapıcı 1.0 yenileme planı ve ajan sözleşmesi (dal yenileme-1.0). Sahip: Yusuf. Yazan: [Claude#Sonnet-5.5 koordinatör, 2026-11-18]. Yalnız yazan yapay zekâ ya da Yusuf'un izniyle değişir; silme yok.
---
# Hedef
Yusuf (2026-11-18): "yapay zeka özelliği iyi çalışmıyor; arayüzler işlenmemiş, menü yok, çok karışık, kullanıcı dostu değil; hazırlanan menü ve tasarımlar implemente edilsin; pacman gibi LED ekranda görünebilecek her türlü malzeme olsun; pürüzsüz ve çok profesyonel; sabaha (09:00 TR) kadar ayaklandırın." Motto: hızlı, konforlu, stabil, güvenli. Sürüm 1.0.0.

# Kaynaklar
- BAĞLAYICI tasarım: `tasarim/yenileme/karar-fable-tur2.md` (6 tema Vitrin/Vuruş/Cephe/Kadraj/Manşet/Eşlik, çekmece kuralı, jetonlar, logo kuralı). Ortak yerleşim, 5 kip (Yaz, Hazır, Yapay zekâ, Görünüm, Ayrıntılar), teknik kurallar: `karar-fable-tur1.md` 2. bölüm. Görsel taslak: `vidyo-taslak.html`.
- Kod: `web/index.html` (tek sayfa arayüz), `src/arayuz.rs` (HTTP sunucu, gömülü dosyalar), `src/senaryo.rs` `src/anim.rs` `src/animasyon.rs` `src/uret.rs` (motor), `src/tarif.rs` `src/llm.rs` `src/tarif_http.rs` (yapay zekâ/tarif), `src/ayar.rs` (kalıcı ayar: `taa-` localStorage anahtarları ayar.json'a yansır), `docs/`.
- Logo: İkiz Kabartma (`web/acilis/logo.js`, index.html `<symbol id="ikiz-kabartma">`). Mevcut 14 açılış animasyonu ve açılış ayarları korunur.

# Dosya sahipliği (aynı çalışma ağacında paralel; BAŞKASININ dosyasına dokunma)
| Ajan | Sahip olduğu dosyalar |
|---|---|
| UI (arayüz) | `web/index.html`, yeni `web/*.css`/`web/tema/*`/`web/yazi/*` (gömülecek yazı tipleri, lisans dosyalarıyla), `web/acilis/*`, `src/arayuz.rs`, `logo/*` |
| AI (yapay zekâ) | `src/tarif.rs`, `src/llm.rs`, `src/tarif_http.rs`, `kurallar/*`, `docs/TARIF-*`, `tests/tarif_dene.rs`, yeni `web/ai.js` |
| Malzeme (içerik kitaplığı) | `src/malzeme_http.rs`, yeni `src/malzeme*.rs`, `src/senaryo.rs`, `src/anim.rs`, `src/animasyon.rs`, `src/efekt.rs`, `src/dekor.rs`, `src/cizim.rs`, `src/uret.rs`, `animasyonlar/*`, `senaryolar/*`, yeni `web/malzeme.js`, `tests/*` (yeni dosyalar) |
Sınırı aşman gerekirse dosyayı değiştirme; `tasarim/yenileme/ISTEK-<ajan>.md` dosyasına ne istediğini yaz (koordinatör çözer). `src/lib.rs` ve `Cargo.toml` yalnız yeni `pub mod` / bağımlılık satırı eklemek için, en küçük değişiklikle düzenlenebilir.

# Ortak kurallar
- Dal `yenileme-1.0`. COMMIT ATMA, PUSH YOK (koordinatör commit eder). Kalıcı silme YOK (rm/del yok; gerekirse dosyayı olduğu gibi bırak). `D:\YusufII\HAFIZAM1\anahtarlar` klasörüne dokunma. Anahtar/sır koda ya da dosyaya yazma. Dış hizmete (GPT/Gemini vb.) soru gönderme; internetten indirme yapma (yazı tipi dosyaları yoksa Windows'taki yerleşik yazı tipleriyle devam et, gömme sonra ele alınır, ISTEK dosyasına yaz).
- Derleme ve test YALNIZ `tasarim/yenileme/kilitli-cargo.sh <cargo alt komutu>` ile (aynı anda tek ağır derleme; kilit sırada bekler). Örn. `tasarim/yenileme/kilitli-cargo.sh test --release`. Önce yalnız kendi alanının testlerini çalıştır (`... test --release <ad>`), tüm paketi yalnız işin sonunda bir kez.
- Her dosyanın başına kısa açıklama (ne, sahibi Yusuf, yazan imzan, değişiklik kuralı). İmza: `[Claude#<rolün> Opus 5.5, yenileme-1.0, 2026-11-18]`.
- Arayüz kuralları (karar-fable-tur1/2): dokunma hedefi ≥48 px (ana eylem 56), yazı ≥16 px (ikincil ≥13), kontrast ≥4,5:1, görünür odak, `prefers-reduced-motion`, Türkçe etiketler kesilmez, `tr-TR` büyük harf; hareket yalnız transform/opacity; ışıma/bulanıklık/gradyan/yaylanma/steps() YASAK; internetten font yok.
- LED içeriği 950x1800 dikey, iki eşlenik ekran: kalın, yüksek kontrast, uzaktan okunur.
- Gerçek ve doğrulanmış sonuç raporla; yapamadığını açıkça yaz; uydurma yok. Rapor Türkçe, özet (ham döküm değil): değişen dosyalar, yapılanlar, test sonucu, riskler, yapılamayanlar.
