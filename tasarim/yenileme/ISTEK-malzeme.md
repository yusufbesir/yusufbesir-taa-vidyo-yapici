---
description: Malzeme ajanının diğer ajanlardan/koordinatörden istekleri (dosya sahipliği dışında kalan değişiklikler).
sahip: Yusuf
yazan: "[Claude#Malzeme Opus 5.5, yenileme-1.0, 2026-11-18]"
kural: Yalnız yazan yapay zekâ ya da Yusuf'un izniyle değişir; silme yok.
---
# Malzeme ajanının istekleri

## Özet
Malzeme galerisini arayüze bağlamak için UI ajanının web/index.html'de iki küçük ekleme yapması gerekiyor; arayuz.rs değişikliği gerekmez. Ayrıca oto.rs ve proje kökündeki bir test artığı için koordinatör kararı gerekiyor.

## UI ajanı (web/index.html)
1. Betiği yükleyin: `<script src="/malzeme/malzeme.js"></script>` (malzeme_http.rs sunuyor, çerezle).
2. "Hazır" (ya da uygun) kipinde bir kap açıp `TaaMalzeme.mount(kap, window.TaaApp)` çağırın. Kullanılan TaaApp yöntemleri: `onizle(senaryoMetni)` ve `senaryoyaEkle(satir)`.
3. İsteğe bağlı: senaryo/zaman çizelgesi öğesine `TaaMalzeme.birakmaHedefiYap(el, window.TaaApp)` (kart sürükle-bırak).
Ayrıntılı sözleşme: docs/MALZEME.md.

## AI ajanı (isteğe bağlı)
Tarif/LLM çıktısı `malzeme <id> ...` satırı üretebilir; geçerli id ve parametreler `GET /malzeme/liste` ya da `crate::malzeme::KATALOG`, doğrulama `crate::malzeme::senaryo_parcasi`.

## Koordinatör
- `src/oto.rs` (sahibi tanımsız): malzeme sahnelerinde metin uzun olduğunda (kayan yazı) "satır başına 8-10 harf" önerisi çıkar. İstenirse `if sh.malzeme.is_some() { continue-benzeri atlama }` eklenebilir; ben dokunmadım.
- Proje kökünde `cikti.mp4` (2026-11-18 04:52, 2,6 MB): sunucuyu denerken `--arayuz` bayrağını unuttuğum için program varsayılan CLI kipinde bu dosyayı üretti. Bir test artığı; silme/taşıma kararı Yusuf'un. Geri Dönüşüm Kutusu'na taşınmasını öneriyorum.
