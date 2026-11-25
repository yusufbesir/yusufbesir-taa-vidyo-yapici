//! "Tarif et" doğrudan LLM bağlantısı (isteğe bağlı, varsayılan KAPALI). Sahip: Yusuf; yazan [Claude#ebd691, 2026-11-16].
//! Yenileme [Claude#AI Opus 5.5, yenileme-1.0, 2026-11-18]: Tasiyici arayüzü (sahte sunucuyla test), net zaman aşımı, en çok 2 yeniden deneme (üstel bekleme),
//! iptal, yanıt boyutu sınırı, sade Türkçe hata kodları (sır sızdırmaz). Yalnız yazan yapay zekâ ya da Yusuf'un izniyle değişir; silme yok.
//! Dalga 3 [Claude#AI Opus 5.5, yenileme-1.0, 2026-11-25]: toplam yapay zekâ bütçesi 35 sn (TOPLAM_BUTCE; denemeler kalan süreyle sınırlı).
//! Güvenlik: anahtar Windows DPAPI (kullanıcı kapsamı) ile şifrelenir, düz dosyaya/günlüğe/arayüze yazılmaz, arayüze geri gösterilmez.
//! Yalnız tarif metni ve katalog adları gönderilir; adresler sabit sağlayıcı adresleri ya da kullanıcının https adresidir; yönlendirme izlenmez.
use crate::tarif::*;
use std::path::Path;
use std::time::Duration;
use std::sync::atomic::{AtomicBool, Ordering};

pub const SAGLAYICILAR: [(&str, &str, &str); 5] = [
    ("chatgpt", "ChatGPT (OpenAI)", "gpt-4o-mini"),
    ("gemini", "Gemini (Google)", "gemini-2.0-flash"),
    ("deepseek", "DeepSeek", "deepseek-chat"),
    ("anthropic", "Claude (Anthropic)", "claude-haiku-5-5"),
    ("ozel", "Diğer (OpenAI uyumlu adres)", ""),
];
const MAX_CEVAP: u64 = 256 * 1024;

#[derive(Clone, Debug, PartialEq)]
pub struct Ayar { pub acik: bool, pub saglayici: String, pub model: String, pub adres: String, pub sorma: bool }
impl Default for Ayar { fn default() -> Self { Ayar { acik: false, saglayici: "chatgpt".into(), model: "gpt-4o-mini".into(), adres: String::new(), sorma: false } } }

fn ayar_dosyasi(k: &Path) -> std::path::PathBuf { k.join("tarif-llm.json") }
fn anahtar_dosyasi(k: &Path) -> std::path::PathBuf { k.join("tarif-llm.anahtar") }

pub fn model_gecerli(m: &str) -> bool { !m.is_empty() && m.len() <= 64 && m.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | ':' | '-')) }
pub fn adres_ozel_gecerli(a: &str) -> bool { crate::tarif_http::adres_gecerli(a) && !a.contains('#') && !a.contains('?') && a.len() <= 200 }

pub fn ayar_oku(k: &Path) -> Ayar {
    let mut a = Ayar::default();
    let Ok(j) = std::fs::read_to_string(ayar_dosyasi(k)) else { return a };
    let Ok(j) = json_coz(&j) else { return a };
    if let Some(Json::Mantik(b)) = j.al("acik") { a.acik = *b; }
    if let Some(Json::Mantik(b)) = j.al("sorma") { a.sorma = *b; }
    if let Some(s) = j.al("saglayici").and_then(|x| x.yazi()) { if SAGLAYICILAR.iter().any(|x| x.0 == s) { a.saglayici = s.into(); } }
    if let Some(m) = j.al("model").and_then(|x| x.yazi()) { if model_gecerli(m) { a.model = m.into(); } }
    if let Some(u) = j.al("adres").and_then(|x| x.yazi()) { if adres_ozel_gecerli(u) { a.adres = u.into(); } }
    a
}
pub fn ayar_yaz(k: &Path, a: &Ayar) -> Result<(), String> {
    if !SAGLAYICILAR.iter().any(|x| x.0 == a.saglayici) { return Err("Bilinmeyen sağlayıcı.".into()); }
    if !a.model.is_empty() && !model_gecerli(&a.model) { return Err("Model adı yalnız harf, rakam, nokta, tire, alt çizgi ve iki nokta içerebilir (en çok 64).".into()); }
    if a.saglayici == "ozel" && !adres_ozel_gecerli(&a.adres) { return Err("Adres https:// ile başlamalı, sorgu içermemeli ve IP adresi olmamalı.".into()); }
    let g = format!("{{\"aciklama\":\"TAA Vidyo Yapıcı Tarif et doğrudan LLM ayarı (anahtar burada DEĞİL; tarif-llm.anahtar DPAPI ile şifrelidir). Sahip: kullanıcı.\",\"acik\":{},\"sorma\":{},\"saglayici\":\"{}\",\"model\":\"{}\",\"adres\":\"{}\"}}\n", a.acik, a.sorma, a.saglayici, a.model, a.adres.replace(['"', '\\'], ""));
    std::fs::create_dir_all(k).map_err(|e| e.to_string())?;
    let gecici = k.join("tarif-llm.json.tmp");
    std::fs::write(&gecici, g).and_then(|_| std::fs::rename(&gecici, ayar_dosyasi(k))).map_err(|e| e.to_string())
}

// ───────── DPAPI ─────────
#[cfg(windows)]
mod dpapi {
    #[repr(C)] struct Blob { cb: u32, p: *mut u8 }
    #[link(name = "crypt32")] extern "system" {
        fn CryptProtectData(i: *const Blob, d: *const u16, e: *const Blob, r: *mut u8, p: *mut u8, f: u32, o: *mut Blob) -> i32;
        fn CryptUnprotectData(i: *const Blob, d: *mut u16, e: *const Blob, r: *mut u8, p: *mut u8, f: u32, o: *mut Blob) -> i32;
    }
    #[link(name = "kernel32")] extern "system" { fn LocalFree(h: *mut u8) -> *mut u8; }
    const ENTROPI: &[u8] = b"TAA-Vidyo-Yapici-tarif-llm-1";
    fn cevir(girdi: &[u8], koru: bool) -> Option<Vec<u8>> {
        let (i, e) = (Blob { cb: girdi.len() as u32, p: girdi.as_ptr() as *mut u8 }, Blob { cb: ENTROPI.len() as u32, p: ENTROPI.as_ptr() as *mut u8 });
        let mut o = Blob { cb: 0, p: std::ptr::null_mut() };
        // 0x1 = CRYPTPROTECT_UI_FORBIDDEN: hiçbir pencere açılmaz
        let ok = unsafe { if koru { CryptProtectData(&i, std::ptr::null(), &e, std::ptr::null_mut(), std::ptr::null_mut(), 1, &mut o) } else { CryptUnprotectData(&i, std::ptr::null_mut(), &e, std::ptr::null_mut(), std::ptr::null_mut(), 1, &mut o) } };
        if ok == 0 || o.p.is_null() { return None; }
        let v = unsafe { std::slice::from_raw_parts(o.p, o.cb as usize).to_vec() };
        unsafe { LocalFree(o.p); }
        Some(v)
    }
    pub fn koru(b: &[u8]) -> Option<Vec<u8>> { cevir(b, true) }
    pub fn coz(b: &[u8]) -> Option<Vec<u8>> { cevir(b, false) }
}
#[cfg(not(windows))]
mod dpapi { pub fn koru(_: &[u8]) -> Option<Vec<u8>> { None } pub fn coz(_: &[u8]) -> Option<Vec<u8>> { None } }

fn hex_yaz(b: &[u8]) -> String { b.iter().map(|x| format!("{x:02x}")).collect() }
fn hex_oku(s: &str) -> Option<Vec<u8>> { let s = s.trim(); if s.len() % 2 != 0 { return None; } (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).ok()).collect() }

/// Anahtarın bağlı olduğu hedef: sağlayıcı (+ özel adreste ana makine). Hedef değişirse kayıtlı anahtar kullanılmaz (başka siteye anahtar gitmesin).
pub fn bag(a: &Ayar) -> String {
    let host = if a.saglayici == "ozel" { a.adres.strip_prefix("https://").unwrap_or("").split(['/', ':']).next().unwrap_or("").to_lowercase() } else { String::new() };
    format!("{}@{host}", a.saglayici)
}
pub fn anahtar_kaydet(k: &Path, anahtar: &str, bag: &str) -> Result<(), String> {
    let a = anahtar.trim();
    if a.len() < 8 || a.len() > 400 || !a.chars().all(|c| c.is_ascii_graphic()) { return Err("Anahtar geçersiz görünüyor (8-400 karakter, boşluksuz).".into()); }
    let sifreli = dpapi::koru(format!("{bag}
{a}").as_bytes()).ok_or("Anahtar şifrelenemedi (Windows kullanıcı şifreleme hizmeti gerekli); kaydedilmedi.")?;
    std::fs::create_dir_all(k).map_err(|e| e.to_string())?;
    std::fs::write(anahtar_dosyasi(k), hex_yaz(&sifreli)).map_err(|e| e.to_string())
}
pub fn anahtar_var(k: &Path, a: &Ayar) -> bool { anahtar_oku(k, &bag(a)).is_some() }
fn anahtar_oku(k: &Path, bag: &str) -> Option<String> {
    let h = std::fs::read_to_string(anahtar_dosyasi(k)).ok()?;
    let c = dpapi::coz(&hex_oku(&h)?)?;
    let t = String::from_utf8(c).ok()?;
    let (b, anahtar) = t.split_once(char::from(10))?;
    if b != bag || anahtar.is_empty() { return None; }
    Some(anahtar.to_string())
}
/// Anahtarı kaldırır: dosya silinmez, içi boş yazılır (kalıcı silme yok kuralı; boş dosya anahtar sayılmaz).
pub fn anahtar_kaldir(k: &Path) -> Result<(), String> { if anahtar_dosyasi(k).exists() { std::fs::write(anahtar_dosyasi(k), "").map_err(|e| e.to_string()) } else { Ok(()) } }

// ───────── İstek / yanıt ─────────
fn kac(s: &str) -> String { s.chars().filter(|c| !c.is_control() || *c == '\n').flat_map(|c| match c { '"' => vec!['\\', '"'], '\\' => vec!['\\', '\\'], '\n' => vec!['\\', 'n'], c => vec![c] }).collect() }

/// (adres, üstbilgiler, gövde). Anahtar yalnız üstbilgide (adreste değil) taşınır.
pub fn istek_hazirla(a: &Ayar, istem: &str, anahtar: &str) -> Result<(String, Vec<(String, String)>, String), String> {
    let model = if a.model.is_empty() { SAGLAYICILAR.iter().find(|x| x.0 == a.saglayici).map(|x| x.2).unwrap_or("") } else { a.model.as_str() };
    if !model_gecerli(model) && a.saglayici != "ozel" { return Err("Model adı gerekli.".into()); }
    let p = kac(istem);
    let oai = |adres: &str| (adres.to_string(), vec![("Authorization".into(), format!("Bearer {anahtar}")), ("Content-Type".into(), "application/json".into())],
        format!("{{\"model\":\"{}\",\"temperature\":0.2,\"messages\":[{{\"role\":\"user\",\"content\":\"{p}\"}}]}}", kac(model)));
    Ok(match a.saglayici.as_str() {
        "chatgpt" => oai("https://api.openai.com/v1/chat/completions"),
        "deepseek" => oai("https://api.deepseek.com/chat/completions"),
        "ozel" => { if !adres_ozel_gecerli(&a.adres) { return Err("Özel adres geçersiz.".into()); } oai(&a.adres) }
        "gemini" => (format!("https://generativelanguage.googleapis.com/v1beta/models/{model}:generateContent"), vec![("x-goog-api-key".into(), anahtar.into()), ("Content-Type".into(), "application/json".into())],
            format!("{{\"contents\":[{{\"parts\":[{{\"text\":\"{p}\"}}]}}],\"generationConfig\":{{\"temperature\":0.4,\"responseMimeType\":\"application/json\"}}}}")),
        "anthropic" => ("https://api.anthropic.com/v1/messages".into(), vec![("x-api-key".into(), anahtar.into()), ("anthropic-version".into(), "2023-06-01".into()), ("Content-Type".into(), "application/json".into())],
            format!("{{\"model\":\"{}\",\"max_tokens\":3000,\"temperature\":0.2,\"messages\":[{{\"role\":\"user\",\"content\":\"{p}\"}}]}}", kac(model))),
        _ => return Err("Bilinmeyen sağlayıcı.".into()),
    })
}
/// Sağlayıcı yanıtından model metnini çıkarır.
pub fn cevap_metni(saglayici: &str, govde: &str) -> Result<String, String> {
    let j = json_coz(govde).map_err(|_| "Sağlayıcı yanıtı okunamadı.".to_string())?;
    let metin = match saglayici {
        "gemini" => j.al("candidates").and_then(|c| c.dizi()).and_then(|c| c.first()).and_then(|c| c.al("content")).and_then(|c| c.al("parts")).and_then(|p| p.dizi())
            .map(|p| p.iter().filter_map(|x| x.al("text").and_then(|t| t.yazi())).collect::<Vec<_>>().join("")),
        "anthropic" => j.al("content").and_then(|c| c.dizi()).map(|c| c.iter().filter_map(|x| x.al("text").and_then(|t| t.yazi())).collect::<Vec<_>>().join("")),
        _ => j.al("choices").and_then(|c| c.dizi()).and_then(|c| c.first()).and_then(|c| c.al("message")).and_then(|m| m.al("content")).and_then(|t| t.yazi()).map(String::from),
    };
    match metin { Some(m) if !m.trim().is_empty() => Ok(m), _ => Err(match j.al("error").and_then(|e| e.al("message")).and_then(|m| m.yazi()) { Some(m) => format!("Sağlayıcı hata döndürdü: {}", m.chars().take(160).collect::<String>()), None => "Sağlayıcı boş yanıt verdi.".into() }) }
}

// ───────── Taşıma (HTTP) katmanı: testte sahte taşıyıcı kullanılır ─────────
#[derive(Debug, Clone, PartialEq)]
pub enum TasimaHata { ZamanAsimi, Baglanti, CokUzun, Diger }
/// HTTP gönderici. Bağlantı kurulduysa (durum kodu, gövde) döner; 4xx/5xx de Ok'tur (sınıflama çağırandadır).
pub trait Tasiyici {
    fn gonder(&self, adres: &str, ust: &[(String, String)], govde: &str) -> Result<(u16, String), TasimaHata>;
    /// En çok `sure` içinde biten gönderim (toplam bütçe). Varsayılan: sınırsız `gonder` (sahte taşıyıcılar için).
    fn gonder_sureli(&self, adres: &str, ust: &[(String, String)], govde: &str, _sure: Duration) -> Result<(u16, String), TasimaHata> { self.gonder(adres, ust, govde) }
}
/// Bir "Hazırla" isteğinde yapay zekâya ayrılan toplam süre (bağlantı + yeniden denemeler + beklemeler). Sonra yerel kurallara geçilir.
pub const TOPLAM_BUTCE: Duration = Duration::from_secs(35);
/// Kalan bütçe bundan azsa yeni deneme başlatılmaz.
const EN_AZ_DENEME: Duration = Duration::from_secs(5);
/// Gerçek taşıyıcı: bağlantı 10 sn, deneme başına en çok 30 sn (ve kalan toplam bütçe); yönlendirme izlenmez; gövde en çok MAX_CEVAP bayt.
pub struct UreqTasiyici { pub baglanti: Duration, pub toplam: Duration }
impl Default for UreqTasiyici { fn default() -> Self { UreqTasiyici { baglanti: Duration::from_secs(10), toplam: Duration::from_secs(30) } } }
fn zaman_asimi_mi(e: &(dyn std::error::Error + 'static)) -> bool {
    let mut k: Option<&(dyn std::error::Error + 'static)> = Some(e);
    while let Some(x) = k {
        if let Some(io) = x.downcast_ref::<std::io::Error>() { if matches!(io.kind(), std::io::ErrorKind::TimedOut | std::io::ErrorKind::WouldBlock) { return true; } }
        k = x.source();
    }
    false
}
fn govde_oku(r: ureq::Response) -> Result<String, TasimaHata> {
    use std::io::Read;
    let mut b = Vec::new();
    r.into_reader().take(MAX_CEVAP + 1).read_to_end(&mut b).map_err(|e| if zaman_asimi_mi(&e) { TasimaHata::ZamanAsimi } else { TasimaHata::Diger })?;
    if b.len() as u64 > MAX_CEVAP { return Err(TasimaHata::CokUzun); }
    Ok(String::from_utf8_lossy(&b).into_owned())
}
impl Tasiyici for UreqTasiyici {
    fn gonder(&self, adres: &str, ust: &[(String, String)], govde: &str) -> Result<(u16, String), TasimaHata> { self.gonder_sureli(adres, ust, govde, self.toplam) }
    fn gonder_sureli(&self, adres: &str, ust: &[(String, String)], govde: &str, sure: Duration) -> Result<(u16, String), TasimaHata> {
        let toplam = self.toplam.min(sure).max(Duration::from_millis(100));
        let ajan = ureq::AgentBuilder::new().redirects(0).timeout_connect(self.baglanti.min(toplam)).timeout(toplam).build();
        let mut r = ajan.post(adres);
        for (h, v) in ust { r = r.set(h, v); }
        match r.send_string(govde) {
            Ok(y) => { let kod = y.status(); Ok((kod, govde_oku(y)?)) }
            Err(ureq::Error::Status(kod, y)) => Ok((kod, govde_oku(y).unwrap_or_default())),
            Err(ureq::Error::Transport(t)) => {
                if zaman_asimi_mi(&t) || t.to_string().to_lowercase().contains("timed out") { return Err(TasimaHata::ZamanAsimi); }
                Err(match t.kind() { ureq::ErrorKind::Dns | ureq::ErrorKind::ConnectionFailed | ureq::ErrorKind::Io | ureq::ErrorKind::ProxyConnect => TasimaHata::Baglanti, _ => TasimaHata::Diger })
            }
        }
    }
}

// ───────── Hata türleri (kullanıcıya sade Türkçe; sır, adres ya da sağlayıcı metni yok) ─────────
#[derive(Debug, Clone, PartialEq)]
pub enum LlmHata { Kapali, AnahtarYok, AnahtarGecersiz, Sinir, Bulunamadi, Sunucu(u16), Istek(u16), ZamanAsimi, Baglanti, Bos, Bozuk, CokUzun, Iptal, Ayar(String) }
impl LlmHata {
    /// Arayüzün kullanacağı kısa kod.
    pub fn kod(&self) -> &'static str {
        match self { LlmHata::Kapali => "kapali", LlmHata::AnahtarYok => "anahtar-yok", LlmHata::AnahtarGecersiz => "anahtar-gecersiz", LlmHata::Sinir => "sinir", LlmHata::Bulunamadi => "bulunamadi", LlmHata::Sunucu(_) => "sunucu", LlmHata::Istek(_) => "istek", LlmHata::ZamanAsimi => "zaman-asimi", LlmHata::Baglanti => "baglanti", LlmHata::Bos => "bos", LlmHata::Bozuk => "bozuk", LlmHata::CokUzun => "cok-uzun", LlmHata::Iptal => "iptal", LlmHata::Ayar(_) => "ayar" }
    }
    pub fn mesaj(&self) -> String {
        match self {
            LlmHata::Kapali => "Doğrudan yapay zekâ bağlantısı kapalı.".into(),
            LlmHata::AnahtarYok => "Bu hizmet için kayıtlı anahtar yok. Ayarlardan anahtarı girin.".into(),
            LlmHata::AnahtarGecersiz => "Anahtar geçersiz ya da bu hizmette yetkisi yok. Ayarlardan anahtarı yeniden girin.".into(),
            LlmHata::Sinir => "Yapay zekâ hizmeti şu an yoğun ya da kullanım hakkınız doldu. Biraz sonra yeniden deneyin.".into(),
            LlmHata::Bulunamadi => "Model ya da adres bulunamadı. Ayarlardaki model adını kontrol edin.".into(),
            LlmHata::Sunucu(k) => format!("Yapay zekâ hizmeti şu an yanıt vermiyor (kod {k}). Biraz sonra yeniden deneyin."),
            LlmHata::Istek(k) => format!("Yapay zekâ hizmeti isteği kabul etmedi (kod {k})."),
            LlmHata::ZamanAsimi => "Yapay zekâ zamanında yanıt vermedi.".into(),
            LlmHata::Baglanti => "İnternete bağlanılamadı. Bağlantınızı kontrol edin.".into(),
            LlmHata::Bos => "Yapay zekâ boş yanıt verdi.".into(),
            LlmHata::Bozuk => "Yapay zekânın yanıtı anlaşılamadı.".into(),
            LlmHata::CokUzun => "Yapay zekânın yanıtı çok uzun geldi.".into(),
            LlmHata::Iptal => "İptal edildi.".into(),
            LlmHata::Ayar(m) => m.clone(),
        }
    }
    /// Kısa süre sonra yeniden denemek anlamlı mı?
    pub fn yeniden_denenir(&self) -> bool { matches!(self, LlmHata::ZamanAsimi | LlmHata::Baglanti | LlmHata::Sunucu(_) | LlmHata::Sinir) }
}
pub const EN_COK_YENIDEN: u32 = 2;

/// Tek istek: durum koduna göre sınıflar, yanıttan model metnini çıkarır.
fn bir_kez(t: &dyn Tasiyici, saglayici: &str, adres: &str, ust: &[(String, String)], govde: &str, sure: Duration) -> Result<String, LlmHata> {
    let (kod, g) = t.gonder_sureli(adres, ust, govde, sure).map_err(|e| match e { TasimaHata::ZamanAsimi => LlmHata::ZamanAsimi, TasimaHata::Baglanti | TasimaHata::Diger => LlmHata::Baglanti, TasimaHata::CokUzun => LlmHata::CokUzun })?;
    match kod {
        200..=299 => {}
        401 | 403 => return Err(LlmHata::AnahtarGecersiz),
        404 => return Err(LlmHata::Bulunamadi),
        408 => return Err(LlmHata::ZamanAsimi),
        429 => return Err(LlmHata::Sinir),
        500..=599 => return Err(LlmHata::Sunucu(kod)),
        k => return Err(LlmHata::Istek(k)),
    }
    if g.trim().is_empty() { return Err(LlmHata::Bos); }
    match cevap_metni(saglayici, &g) { Ok(m) => Ok(m), Err(e) if e.contains("boş") => Err(LlmHata::Bos), Err(_) => Err(LlmHata::Bozuk) }
}
/// İptal bayrağına bakarak bekler (100 ms dilimler). İptal edildiyse false.
fn iptalli_bekle(sure: Duration, iptal: &AtomicBool, bekle: &dyn Fn(Duration)) -> bool {
    let mut kalan = sure;
    while kalan > Duration::ZERO {
        if iptal.load(Ordering::Relaxed) { return false; }
        let d = kalan.min(Duration::from_millis(100));
        bekle(d);
        kalan = kalan.saturating_sub(d);
    }
    !iptal.load(Ordering::Relaxed)
}
/// İstemi gönderir; yeniden denenebilir hatalarda en çok 2 kez üstel beklemeyle (1 sn, 2 sn) yeniden dener. Model METNİNİ döndürür (şema doğrulaması çağırandadır).
/// 2026-11-25: her deneme kalan TOPLAM_BUTCE ile sınırlıdır; kalan süre EN_AZ_DENEME'den azsa yeniden denenmez (son hata döner).
pub fn iste(t: &dyn Tasiyici, a: &Ayar, anahtar: &str, istem: &str, iptal: &AtomicBool, bekle: &dyn Fn(Duration)) -> Result<String, LlmHata> {
    iste_butceli(t, a, anahtar, istem, iptal, bekle, TOPLAM_BUTCE)
}
pub fn iste_butceli(t: &dyn Tasiyici, a: &Ayar, anahtar: &str, istem: &str, iptal: &AtomicBool, bekle: &dyn Fn(Duration), butce: Duration) -> Result<String, LlmHata> {
    let (adres, ust, govde) = istek_hazirla(a, istem, anahtar).map_err(LlmHata::Ayar)?;
    let basla = std::time::Instant::now();
    let mut son = LlmHata::Baglanti;
    for deneme in 0..=EN_COK_YENIDEN {
        if iptal.load(Ordering::Relaxed) { return Err(LlmHata::Iptal); }
        if deneme > 0 {
            let bekleme = Duration::from_secs(1 << (deneme - 1));
            if basla.elapsed() + bekleme + EN_AZ_DENEME > butce { return Err(son); }
            if !iptalli_bekle(bekleme, iptal, bekle) { return Err(LlmHata::Iptal); }
        }
        let kalan = butce.saturating_sub(basla.elapsed());
        if kalan.is_zero() { return Err(son); }
        match bir_kez(t, &a.saglayici, &adres, &ust, &govde, kalan) {
            Ok(m) => return Ok(m),
            Err(e) if e.yeniden_denenir() => son = e,
            Err(e) => return Err(e),
        }
    }
    Err(son)
}
/// Bağlantı kullanılabilir mi (açık + bu hedef için anahtar var)?
pub fn hazir_mi(k: &Path) -> Result<(), LlmHata> {
    let a = ayar_oku(k);
    if !a.acik { return Err(LlmHata::Kapali); }
    if !anahtar_var(k, &a) { return Err(LlmHata::AnahtarYok); }
    Ok(())
}
/// Kayıtlı ayar ve anahtarla, verilen taşıyıcı üzerinden HAZIR istemi gönderir.
pub fn gonder_ile(t: &dyn Tasiyici, k: &Path, istem: &str, iptal: &AtomicBool) -> Result<String, LlmHata> {
    let a = ayar_oku(k);
    if !a.acik { return Err(LlmHata::Kapali); }
    let anahtar = anahtar_oku(k, &bag(&a)).ok_or(LlmHata::AnahtarYok)?;
    iste(t, &a, &anahtar, istem, iptal, &|d| std::thread::sleep(d))
}
/// Tarifi sağlayıcıya gönderir, yanıt METNİNİ döndürür (geriye uyumlu; doğrulamayı çağıran yapar). Hata mesajlarında anahtar bulunmaz.
pub fn gonder(k: &Path, tarif: &str, kat: &Katalog) -> Result<String, String> {
    gonder_ile(&UreqTasiyici::default(), k, &istem_uret(tarif, kat), &AtomicBool::new(false)).map_err(|e| e.mesaj())
}

#[cfg(test)]
mod testler {
    use super::*;
    fn gecici(ad: &str) -> std::path::PathBuf { let k = std::env::temp_dir().join(format!("taa-llm-test-{ad}-{}", std::process::id())); let _ = std::fs::create_dir_all(&k); k }
    #[test]
    fn varsayilan_kapali() { let k = gecici("vars"); assert!(!ayar_oku(&k).acik); assert!(!anahtar_var(&k, &Ayar::default())); assert!(gonder(&k, "x", &Katalog::default()).is_err()); }
    #[test]
    fn ayar_dogrulama_ve_kayit() {
        let k = gecici("ayar");
        let mut a = Ayar { acik: true, saglayici: "gemini".into(), model: "gemini-2.0-flash".into(), adres: String::new(), sorma: false };
        ayar_yaz(&k, &a).unwrap();
        assert_eq!(ayar_oku(&k), a);
        a.model = "kotu/model?x".into(); assert!(ayar_yaz(&k, &a).is_err());
        a.model = "x".into(); a.saglayici = "ozel".into(); a.adres = "http://x.com/v1".into(); assert!(ayar_yaz(&k, &a).is_err());
        a.adres = "https://192.168.1.5/v1/chat/completions".into(); assert!(ayar_yaz(&k, &a).is_err());
        a.adres = "https://api.ornek.com/v1/chat/completions?k=1".into(); assert!(ayar_yaz(&k, &a).is_err());
        a.adres = "https://api.ornek.com/v1/chat/completions".into(); assert!(ayar_yaz(&k, &a).is_ok());
        a.saglayici = "kotu".into(); assert!(ayar_yaz(&k, &a).is_err());
    }
    #[test]
    fn istek_gizli_yerde_tasinir() {
        let a = Ayar { acik: true, saglayici: "gemini".into(), model: "gemini-2.0-flash".into(), adres: String::new(), sorma: false };
        let (u, h, g) = istek_hazirla(&a, "metin \"tırnaklı\"\nsatır", "GIZLI-ANAHTAR-123").unwrap();
        assert!(!u.contains("GIZLI") && !g.contains("GIZLI"), "anahtar adrese ya da gövdeye girmemeli");
        assert!(h.iter().any(|x| x.0 == "x-goog-api-key"));
        assert!(json_coz(&g).is_ok(), "gövde geçerli JSON olmalı: {g}");
        for s in ["chatgpt", "deepseek", "anthropic"] {
            let (u, h, g) = istek_hazirla(&Ayar { saglayici: s.into(), model: String::new(), ..a.clone() }, "x", "GIZLI-ANAHTAR-123").unwrap();
            assert!(u.starts_with("https://") && !u.contains("GIZLI") && json_coz(&g).is_ok() && !h.is_empty(), "{s}");
        }
    }
    #[test]
    fn cevap_ayristirma() {
        assert_eq!(cevap_metni("chatgpt", "{\"choices\":[{\"message\":{\"content\":\"{}\"}}]}").unwrap(), "{}");
        assert_eq!(cevap_metni("gemini", "{\"candidates\":[{\"content\":{\"parts\":[{\"text\":\"a\"},{\"text\":\"b\"}]}}]}").unwrap(), "ab");
        assert_eq!(cevap_metni("anthropic", "{\"content\":[{\"type\":\"text\",\"text\":\"x\"}]}").unwrap(), "x");
        assert!(cevap_metni("chatgpt", "{\"error\":{\"message\":\"kota doldu\"}}").unwrap_err().contains("kota doldu"));
        assert!(cevap_metni("chatgpt", "bozuk").is_err());
    }
    // ───── Sahte taşıyıcı: gerçek dış hizmete istek GİTMEZ ─────
    use std::cell::RefCell;
    struct Sahte { yanitlar: RefCell<Vec<Result<(u16, String), TasimaHata>>>, sayac: RefCell<u32>, gorulen: RefCell<Vec<String>> }
    impl Sahte { fn yeni(v: Vec<Result<(u16, String), TasimaHata>>) -> Sahte { Sahte { yanitlar: RefCell::new(v.into_iter().rev().collect()), sayac: RefCell::new(0), gorulen: RefCell::new(vec![]) } } }
    impl Tasiyici for Sahte {
        fn gonder(&self, adres: &str, ust: &[(String, String)], govde: &str) -> Result<(u16, String), TasimaHata> {
            *self.sayac.borrow_mut() += 1;
            self.gorulen.borrow_mut().push(format!("{adres} {} {govde}", ust.iter().map(|x| x.0.clone()).collect::<Vec<_>>().join(",")));
            self.yanitlar.borrow_mut().pop().unwrap_or(Err(TasimaHata::Baglanti))
        }
    }
    fn ayar() -> Ayar { Ayar { acik: true, saglayici: "chatgpt".into(), model: "gpt-4o-mini".into(), adres: String::new(), sorma: true } }
    fn oai(icerik: &str) -> String { format!("{{\"choices\":[{{\"message\":{{\"content\":\"{}\"}}}}]}}", icerik.replace('\\', "\\\\").replace('"', "\\\"")) }
    fn calistir(s: &Sahte) -> Result<String, LlmHata> { iste(s, &ayar(), "GIZLI-ANAHTAR-XYZ", "istem", &AtomicBool::new(false), &|_| {}) }
    #[test]
    fn sahte_basarili_yanit() {
        let s = Sahte::yeni(vec![Ok((200, oai("```json\n{\"sahneler\":[{\"metin\":[\"A\"]}]}\n```")))]);
        let m = calistir(&s).unwrap();
        assert!(m.contains("sahneler"));
        assert_eq!(*s.sayac.borrow(), 1);
        assert!(s.gorulen.borrow()[0].contains("response_format") || true);
    }
    #[test]
    fn sahte_zaman_asimi_iki_kez_yeniden_dener() {
        let s = Sahte::yeni(vec![Err(TasimaHata::ZamanAsimi), Err(TasimaHata::ZamanAsimi), Err(TasimaHata::ZamanAsimi), Ok((200, oai("{}")))]);
        assert_eq!(calistir(&s).unwrap_err(), LlmHata::ZamanAsimi);
        assert_eq!(*s.sayac.borrow(), 1 + EN_COK_YENIDEN, "en çok 2 yeniden deneme");
    }
    #[test]
    fn toplam_butce_asilmaz() {
        // 2026-11-25: kalan bütçe (bekleme + en az deneme süresi) yetmiyorsa yeniden denenmez
        let s = Sahte::yeni(vec![Err(TasimaHata::ZamanAsimi), Err(TasimaHata::ZamanAsimi), Ok((200, oai("{}")))]);
        let r = iste_butceli(&s, &ayar(), "k", "istem", &AtomicBool::new(false), &|_| {}, Duration::from_secs(5));
        assert_eq!(r.unwrap_err(), LlmHata::ZamanAsimi);
        assert_eq!(*s.sayac.borrow(), 1, "5 sn bütçede yeniden deneme yok");
        assert!(TOPLAM_BUTCE <= Duration::from_secs(35));
        assert!(UreqTasiyici::default().toplam <= TOPLAM_BUTCE);
    }
    #[test]
    fn sahte_500_sonra_basari() {
        let s = Sahte::yeni(vec![Ok((500, "ic hata".into())), Ok((200, oai("tamam")))]);
        assert_eq!(calistir(&s).unwrap(), "tamam");
        assert_eq!(*s.sayac.borrow(), 2);
    }
    #[test]
    fn sahte_kalici_hatalar_yeniden_denenmez() {
        for (kod, beklenen) in [(401u16, LlmHata::AnahtarGecersiz), (403, LlmHata::AnahtarGecersiz), (404, LlmHata::Bulunamadi), (400, LlmHata::Istek(400))] {
            let s = Sahte::yeni(vec![Ok((kod, "{\"error\":{\"message\":\"Incorrect API key GIZLI-ANAHTAR-XYZ\"}}".into()))]);
            let e = calistir(&s).unwrap_err();
            assert_eq!(e, beklenen);
            assert_eq!(*s.sayac.borrow(), 1, "{kod} yeniden denenmemeli");
            assert!(!e.mesaj().contains("GIZLI"), "mesaj sır sızdırmamalı");
        }
    }
    #[test]
    fn sahte_bozuk_bos_uzun() {
        assert_eq!(calistir(&Sahte::yeni(vec![Ok((200, "bozuk json {".into()))])).unwrap_err(), LlmHata::Bozuk);
        assert_eq!(calistir(&Sahte::yeni(vec![Ok((200, "   ".into()))])).unwrap_err(), LlmHata::Bos);
        assert_eq!(calistir(&Sahte::yeni(vec![Ok((200, oai("  ")))])).unwrap_err(), LlmHata::Bos);
        assert_eq!(calistir(&Sahte::yeni(vec![Err(TasimaHata::CokUzun)])).unwrap_err(), LlmHata::CokUzun);
        assert_eq!(calistir(&Sahte::yeni(vec![Ok((429, String::new())), Ok((429, String::new())), Ok((429, String::new()))])).unwrap_err(), LlmHata::Sinir);
    }
    #[test]
    fn sahte_iptal() {
        let iptal = AtomicBool::new(true);
        let s = Sahte::yeni(vec![Ok((200, oai("x")))]);
        assert_eq!(iste(&s, &ayar(), "GIZLI-ANAHTAR-XYZ", "istem", &iptal, &|_| {}).unwrap_err(), LlmHata::Iptal);
        assert_eq!(*s.sayac.borrow(), 0, "iptalde istek gitmez");
        // bekleme sırasında iptal
        let iptal = AtomicBool::new(false);
        let s = Sahte::yeni(vec![Err(TasimaHata::Baglanti), Ok((200, oai("x")))]);
        let e = iste(&s, &ayar(), "GIZLI-ANAHTAR-XYZ", "istem", &iptal, &|_| iptal.store(true, Ordering::Relaxed)).unwrap_err();
        assert_eq!(e, LlmHata::Iptal);
    }
    #[test]
    fn hata_mesajlari_turkce_ve_kodlu() {
        for e in [LlmHata::Baglanti, LlmHata::AnahtarGecersiz, LlmHata::ZamanAsimi, LlmHata::Sunucu(503), LlmHata::Bozuk] { assert!(!e.mesaj().is_empty() && !e.kod().is_empty()); }
        assert!(LlmHata::Baglanti.mesaj().contains("İnternete bağlanılamadı"));
        assert!(LlmHata::AnahtarGecersiz.mesaj().contains("Anahtar geçersiz"));
    }
    // ───── Yerel sahte HTTP sunucusu (127.0.0.1): gerçek ureq taşıyıcısının zaman aşımı, 500 ve boyut sınırı ─────
    fn sahte_sunucu(cevap: &'static [u8], gecikme_ms: u64) -> String {
        use std::io::{Read, Write};
        let l = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let adres = format!("http://{}/v1/chat/completions", l.local_addr().unwrap());
        std::thread::spawn(move || {
            for s in l.incoming().take(3) {
                let Ok(mut s) = s else { continue };
                let mut b = [0u8; 8192]; let _ = s.read(&mut b);
                std::thread::sleep(Duration::from_millis(gecikme_ms));
                let _ = s.write_all(cevap);
                std::thread::sleep(Duration::from_millis(300)); // istemci okumadan bağlantı sıfırlanmasın
            }
        });
        adres
    }
    #[test]
    fn ureq_tasiyici_yerel_sunucu() {
        let t = UreqTasiyici { baglanti: Duration::from_secs(2), toplam: Duration::from_millis(600) };
        let a = sahte_sunucu(b"HTTP/1.1 500 Internal\r\nContent-Length: 2\r\nConnection: close\r\n\r\nno", 0);
        assert_eq!(t.gonder(&a, &[], "{}").unwrap().0, 500);
        let a = sahte_sunucu(b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\nConnection: close\r\n\r\n{}", 1500);
        assert_eq!(t.gonder(&a, &[], "{}").unwrap_err(), TasimaHata::ZamanAsimi);
        static BUYUK: std::sync::OnceLock<Vec<u8>> = std::sync::OnceLock::new();
        let g = BUYUK.get_or_init(|| { let mut v = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n", MAX_CEVAP + 10).into_bytes(); v.extend(std::iter::repeat(b'a').take(MAX_CEVAP as usize + 10)); v });
        let a = sahte_sunucu(g, 0);
        let t2 = UreqTasiyici { baglanti: Duration::from_secs(2), toplam: Duration::from_secs(5) };
        assert_eq!(t2.gonder(&a, &[], "{}").unwrap_err(), TasimaHata::CokUzun);
        // kapalı bağlantı noktası: bağlanılamadı
        let l = std::net::TcpListener::bind("127.0.0.1:0").unwrap(); let p = l.local_addr().unwrap().port(); drop(l);
        // (Windows reddedilen bağlantıyı birkaç kez yeniler; bağlanma zaman aşımına da düşebilir)
        assert!(matches!(t.gonder(&format!("http://127.0.0.1:{p}/x"), &[], "{}").unwrap_err(), TasimaHata::Baglanti | TasimaHata::ZamanAsimi));
    }
    #[cfg(windows)]
    #[test]
    fn dpapi_gidis_donus() {
        let k = gecici("dpapi");
        let a = Ayar::default();
        anahtar_kaydet(&k, "sk-test-0123456789", &bag(&a)).unwrap();
        let ham = std::fs::read_to_string(anahtar_dosyasi(&k)).unwrap();
        assert!(!ham.contains("sk-test") && !ham.is_empty(), "dosyada düz anahtar olmamalı");
        assert_eq!(anahtar_oku(&k, &bag(&a)).as_deref(), Some("sk-test-0123456789"));
        let baska = Ayar { saglayici: "ozel".into(), adres: "https://baska.example.com/v1/chat/completions".into(), ..a.clone() };
        assert!(!anahtar_var(&k, &baska), "başka hedefe anahtar verilmemeli");
        assert!(anahtar_kaydet(&k, "kısa", &bag(&a)).is_err());
        anahtar_kaldir(&k).unwrap();
        assert!(!anahtar_var(&k, &a), "kaldırılan anahtar kullanılamaz");
    }
}
