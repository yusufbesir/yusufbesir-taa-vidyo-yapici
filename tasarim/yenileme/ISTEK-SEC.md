---
description: SEC-FIX ajanının (dalga 4) koordinatöre istekleri. Sahip Yusuf; yazan [Claude#SEC-FIX Opus 5.5, yenileme-1.0, 2026-11-26]; yalnız yazan yapay zekâ ya da Yusuf'un izniyle değişir; silme yok.
---
# SEC-FIX istekleri (dalga 4)

## 1. .gitignore: kök dizindeki kurulum artıkları
Kök dizinde izlenmeyen `kaldir.exe`, `taa-vidyo-yapici.exe`, `simge.ico`, `kurulum.txt` var (kurulum artığı). Bu ajan dosyalara DOKUNMADI.
Öneri: `.gitignore` sonuna şu satırlar eklensin (yalnız ekleme; dosyalar yerinde kalır):

```
/kaldir.exe
/taa-vidyo-yapici.exe
/simge.ico
/kurulum.txt
```

## 2. Derleme: target\release\taa-vidyo-yapici.exe kilitli
Dalga 4 sırasında başka bir oturumun çalıştırdığı `target\release\taa-vidyo-yapici.exe` (çalışan süreç) yüzünden `cargo test` "Erişim engellendi (os error 5)" verdi.
SEC-FIX testlerini `--target-dir target/sec` ile koştu. Öneri: arayüzü denemek için çalıştırılan ikili `target` dışına kopyalanıp oradan açılsın.

## 3. Bilgi: oynatıcı ve diğer sayfalar
Ana sayfaya CSP eklendi (`src/arayuz.rs` SAYFA_CSP). `/oynatici` ve `/cikis` kendi başlıklarını `oynatici_http` içinde yazıyor (Oynatıcı ajanının alanı); bunlara dokunulmadı.
