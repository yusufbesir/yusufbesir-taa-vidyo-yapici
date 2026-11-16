//! "Tarif et" doğrudan LLM bağlantısı (isteğe bağlı, varsayılan KAPALI). Sahip: Yusuf; yazan [Claude#ebd691, 2026-11-16].
//! Güvenlik: anahtar Windows DPAPI (kullanıcı kapsamı) ile şifrelenir, düz dosyaya/günlüğe/arayüze yazılmaz, arayüze geri gösterilmez.
//! Yalnız tarif metni ve katalog adları gönderilir; adresler sabit sağlayıcı adresleri ya da kullanıcının https adresidir; yönlendirme izlenmez.
use crate::tarif::*;
use std::path::Path;
use std::time::Duration;

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
            format!("{{\"contents\":[{{\"parts\":[{{\"text\":\"{p}\"}}]}}],\"generationConfig\":{{\"temperature\":0.2}}}}")),
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

/// Tarifi sağlayıcıya gönderir, yanıt METNİNİ döndürür (doğrulamayı çağıran yapar). Hata mesajlarında anahtar bulunmaz.
pub fn gonder(k: &Path, tarif: &str, kat: &Katalog) -> Result<String, String> {
    let a = ayar_oku(k);
    if !a.acik { return Err("Doğrudan bağlantı kapalı.".into()); }
    let anahtar = anahtar_oku(k, &bag(&a)).ok_or("Bu hizmet/adres için kayıtlı anahtar yok; anahtarı yeniden girin.")?;
    let (adres, ust, govde) = istek_hazirla(&a, &istem_uret(tarif, kat), &anahtar)?;
    let ajan = ureq::AgentBuilder::new().redirects(0).timeout(Duration::from_secs(60)).build();
    let mut r = ajan.post(&adres);
    for (h, v) in &ust { r = r.set(h, v); }
    let temizle = |s: String| s.replace(&anahtar, "***");
    match r.send_string(&govde) {
        Ok(y) => {
            use std::io::Read;
            let mut s = String::new();
            y.into_reader().take(MAX_CEVAP).read_to_string(&mut s).map_err(|_| "Yanıt okunamadı.".to_string())?;
            cevap_metni(&a.saglayici, &s).map_err(temizle)
        }
        Err(ureq::Error::Status(kod, y)) => {
            let mut s = String::new();
            { use std::io::Read; let _ = y.into_reader().take(8 * 1024).read_to_string(&mut s); }
            let ayrinti = json_coz(&s).ok().and_then(|j| j.al("error").and_then(|e| e.al("message")).and_then(|m| m.yazi()).map(|m| m.chars().take(160).collect::<String>())).unwrap_or_default();
            Err(temizle(format!("Sağlayıcı {kod} kodu döndürdü{}{}", if kod == 401 || kod == 403 { " (anahtar reddedildi olabilir)" } else if kod == 429 { " (istek sınırı ya da kredi)" } else { "" }, if ayrinti.is_empty() { String::new() } else { format!(": {ayrinti}") })))
        }
        Err(e) => Err(temizle(format!("Bağlanılamadı ({}). Kopyala-yapıştır yolunu kullanabilirsiniz.", e.to_string().chars().take(120).collect::<String>()))),
    }
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
