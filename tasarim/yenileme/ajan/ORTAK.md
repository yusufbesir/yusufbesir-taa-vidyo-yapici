---
description: Vidyo 1.0 dalga 3 ajanları için ortak bağlam ve kurallar. Sahip: Yusuf. Yazan: [Claude#Sonnet-5.5 koordinatör, 2026-11-25]. Yalnız yazan yapay zekâ ya da Yusuf'un izniyle değişir; silme yok.
---
# Ortak bağlam
Proje: D:\Programlar\TAA Vidyo Yapıcı (Rust + web arayüz, dal yenileme-1.0, hedef sürüm 1.0.0). Önce tasarim/yenileme/PLAN.md oku. Dalga 1 commit edildi (git log -1; ayrıntı: docs/MALZEME.md, docs/TARIF-API.md). Fable bağımsız denetimi dalga 1 için KALDI dedi (1 kritik, 5 yüksek bulgu); ilgili bulgular rol dosyanda.

Sahip Yusuf'un isteği (2026-11-24): "istemler çok alakasız sonuç veriyor, optimize edilsin; animasyonlar, yazılı animasyonlar (3B/2B yazılar düzgün animasyonlarla), videoları çizgifilm/animasyon/piksel art'a çeviren dönüştürücü, premium ağır efektler (hafif kaçmasın): kış (kar yağışı, çam ağacı, yılbaşı havai fişekleri); dünyadaki benzer LED ekran çalışmalarına benzer içerik; tam donanımlı program (başka programa ihtiyaç kalmasın); ekranları yönetme ve yansıtma için Resolume Arena yerine kullanılacak oynatıcı: bağımsız katmanlar, yeni katman, kapatılabilir, sıralı oynatma, son videodan başa alma. Animasyonlar için GPT ve Gemini ile birlikte çalışın (ekip)."

# Ekip (GPT + Gemini) önerileri
tasarim/yenileme/ekip/gpt-sol-animasyon.txt ve gemini-animasyon.txt varsa oku (yoksa kendin sırsız brifle codex exec -m gpt-6.1-sol ve agy -p ... --model gemini-3.8-flash-high ile sor; komut kalıpları ~/.claude/skills/codex-cli ve agy-antigravity; kota doluysa atla ve bildir).

# Kurallar
- Commit yok, push yok, kalıcı silme yok (rm/del yok), D:\YusufII\HAFIZAM1\anahtarlar'a dokunma, anahtar/sır koda ya da dosyaya yazılmaz.
- Derleme ve test YALNIZ `tasarim/yenileme/kilitli-cargo.sh <cargo alt komutu>` ile (aynı anda tek ağır derleme; kilit sırada bekler). Önce yalnız kendi alanının testlerini, tüm paketi işin sonunda bir kez çalıştır.
- Dış AI'ya (codex/agy) yalnız sırsız, kişisel veri içermeyen metin gönder.
- Dosya sahipliği (başkasının dosyasına dokunma; gerekirse tasarim/yenileme/ISTEK-<ajan>.md'ye yaz): UI = web/index.html, tests/arayuz_ui.rs, src/arayuz.rs · AI = src/tarif.rs, src/tarif/*, src/llm.rs, src/tarif_http.rs, kurallar/*, tests/tarif_dene.rs, tests/ai_test.js, web/ai.js, docs/TARIF-* · Malzeme = src/malzeme*.rs, src/senaryo.rs, src/anim*.rs, src/efekt.rs, src/dekor.rs, src/cizim.rs, src/uret.rs, src/oto.rs, web/malzeme.js, tests/malzeme*, docs/MALZEME.md · Dönüştürücü = yeni src/donustur*.rs, web/donustur.js, tests/donustur*, docs/DONUSTURUCU.md · Oynatıcı = yeni src/oynatici*.rs, web/oynatici*.html/js/css, web/cikis.html, tests/oynatici*, docs/OYNATICI.md. src/lib.rs ve Cargo.toml'a yalnız yeni `pub mod` / bağımlılık satırı ekleme (hedefli Edit).
- Yeni dosyaların başına kısa açıklama + imza ("sahip Yusuf; yazan [Claude#<rol> Opus 5.5, yenileme-1.0, 2026-11-25]; yalnız yazan yapay zekâ ya da Yusuf'un izniyle değişir; silme yok").
- Arayüz kuralları: dokunma hedefi ≥48 px (ana eylem 56), yazı ≥16 px (ikincil ≥13), kontrast ≥4,5:1, görünür odak, prefers-reduced-motion, Türkçe etiket kesilmez; hareket yalnız transform/opacity; ışıma/bulanıklık/gradyan/yaylanma/steps() yasak; yalnız CSS değişkenleri (--bg --panel --panel2 --line --tx --mut --ac --onac --ok --uy --r-btn --r-kart); dikey 950x1800, iki eşlenik ekran, kalın ve yüksek kontrast.
- Rapor Türkçe ve özet (ham döküm değil): tamamlanan, değişen dosyalar, test sonucu, yapılamayan, riskler, sözleşme. Doğrulanmamışı "tamam" yazma. Süre: yaklaşık 60-80 dakika; çalışır parçayı önce bitir, sonra cila.
