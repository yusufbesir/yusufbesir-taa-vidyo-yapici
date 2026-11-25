//! Dönüştürücü HTTP uçları. arayuz.rs `/donustur/` önekini buraya yollar (sorgu dizesi gelmez; parametreler JSON gövdede):
//!   GET  /donustur/donustur.js     → arayüz betiği (window.TaaDonustur)
//!   GET  /donustur/turler          → {"turler":[...],"paletler":[...],"tonlar":[...],"bloklar":[...],"ffmpeg":bool}
//!   GET  /donustur/kaynaklar       → {"kaynaklar":[{"yol","ad","klasor","boyut","resim"}]}
//!   POST /donustur/onizle  {kaynak,tur,...,t,kucuk} → {"tamam":true,"png":"data:image/png;base64,..."}
//!   POST /donustur/baslat  {kaynak,tur,...}         → {"tamam":true,"dosya":"..."}  (409: zaten iş var)
//!   GET  /donustur/durum           → {"calisiyor","kare","toplam","yuzde","asama","dosya","mesaj","hata"}
//!   POST /donustur/iptal           → {"tamam":true}
//!   POST /donustur/oynat           → son çıktıyı varsayılan oynatıcıda açar
//!   POST /donustur/klasor          → çıktı klasörünü açar
//! Hata gövdesi: {"tamam":false,"hata":"Türkçe mesaj"}.
//! Sahip Yusuf; yazan [Claude#Dönüştürücü Opus 5.5, yenileme-1.0, 2026-11-25]; yalnız yazan yapay zekâ ya da Yusuf'un izniyle değişir; silme yok.
use crate::donustur as d;
use std::path::Path;

pub const GOVDE_MAX: usize = 8 * 1024;
const JSON: &str = "application/json; charset=utf-8";
pub const BETIK: &str = include_str!("../web/donustur.js");

pub fn jk(s: &str) -> String {
    let mut o = String::with_capacity(s.len());
    for c in s.chars() {
        match c { '"' => o.push_str("\\\""), '\\' => o.push_str("\\\\"), '\n' => o.push_str("\\n"), '\r' => o.push_str("\\r"), '\t' => o.push_str("\\t"), c if (c as u32) < 0x20 => o.push_str(&format!("\\u{:04x}", c as u32)), c => o.push(c) }
    }
    o
}

fn hata(kod: u16, m: &str) -> (u16, &'static str, String) { (kod, JSON, format!("{{\"tamam\":false,\"hata\":\"{}\"}}", jk(m))) }

/// Düz JSON nesnesi (iç içe yok): değerler metin olarak döner (sayı/bool olduğu gibi). Bozuksa None.
pub fn json_duz(s: &str) -> Option<Vec<(String, String)>> {
    let b: Vec<char> = s.trim().chars().collect();
    if b.first() != Some(&'{') || b.last() != Some(&'}') { return None; }
    let mut i = 1;
    let mut v = vec![];
    let bosluk = |i: &mut usize| while *i < b.len() && b[*i].is_whitespace() { *i += 1 };
    fn metin(b: &[char], i: &mut usize) -> Option<String> {
        if b.get(*i) != Some(&'"') { return None; }
        *i += 1;
        let mut o = String::new();
        while *i < b.len() {
            let c = b[*i]; *i += 1;
            match c {
                '"' => return Some(o),
                '\\' => { let e = *b.get(*i)?; *i += 1; match e { 'n' => o.push('\n'), 't' => o.push('\t'), 'r' => o.push('\r'), 'b' | 'f' => {}, 'u' => { let h: String = b.get(*i..*i + 4)?.iter().collect(); *i += 4; o.push(char::from_u32(u32::from_str_radix(&h, 16).ok()?).unwrap_or('\u{fffd}')); } e => o.push(e) } }
                c => o.push(c),
            }
        }
        None
    }
    loop {
        bosluk(&mut i);
        if b.get(i) == Some(&'}') && v.is_empty() { i += 1; break; }
        let k = metin(&b, &mut i)?;
        bosluk(&mut i);
        if b.get(i) != Some(&':') { return None; }
        i += 1; bosluk(&mut i);
        let deger = if b.get(i) == Some(&'"') { metin(&b, &mut i)? } else {
            let bas = i;
            while i < b.len() && !matches!(b[i], ',' | '}') && !b[i].is_whitespace() { i += 1; }
            let t: String = b[bas..i].iter().collect();
            if t.is_empty() || t.starts_with('{') || t.starts_with('[') { return None; }
            if !(t == "true" || t == "false" || t == "null" || t.parse::<f64>().is_ok()) { return None; }
            t
        };
        v.push((k, deger));
        bosluk(&mut i);
        match b.get(i) { Some(',') => i += 1, Some('}') => { i += 1; break; } _ => return None }
    }
    bosluk(&mut i);
    (i == b.len()).then_some(v)
}

pub fn base64(v: &[u8]) -> String {
    const A: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut o = String::with_capacity(v.len().div_ceil(3) * 4);
    for p in v.chunks(3) {
        let n = (p[0] as u32) << 16 | (*p.get(1).unwrap_or(&0) as u32) << 8 | *p.get(2).unwrap_or(&0) as u32;
        o.push(A[(n >> 18) as usize & 63] as char);
        o.push(A[(n >> 12) as usize & 63] as char);
        o.push(if p.len() > 1 { A[(n >> 6) as usize & 63] as char } else { '=' });
        o.push(if p.len() > 2 { A[n as usize & 63] as char } else { '=' });
    }
    o
}

fn turler_json() -> String {
    let t: Vec<String> = d::TURLER.iter().map(|t| format!("{{\"id\":\"{}\",\"ad\":\"{}\",\"aciklama\":\"{}\"}}", t.id, jk(t.ad), jk(t.aciklama))).collect();
    let p: Vec<String> = d::PALETLER.iter().map(|p| format!("{{\"id\":\"{}\",\"ad\":\"{}\",\"renkler\":[{}]}}", p.0, jk(p.1), p.2.iter().map(|c| format!("\"#{c:06x}\"")).collect::<Vec<_>>().join(","))).collect();
    let o: Vec<String> = d::TONLAR.iter().map(|t| format!("{{\"id\":\"{}\",\"ad\":\"{}\",\"koyu\":\"#{:06x}\",\"acik\":\"#{:06x}\"}}", t.0, jk(t.1), t.2, t.3)).collect();
    let b: Vec<String> = d::BLOKLAR.iter().map(|b| b.to_string()).collect();
    format!("{{\"turler\":[{}],\"paletler\":[{}],\"tonlar\":[{}],\"bloklar\":[{}],\"ffmpeg\":{},\"w\":{},\"h\":{}}}", t.join(","), p.join(","), o.join(","), b.join(","), crate::ffmpeg::bul().is_some(), d::W, d::H)
}

fn kaynaklar_json(klasor: &Path) -> String {
    let k = d::kokler(klasor);
    let v: Vec<String> = d::kaynak_listesi(&k).into_iter().map(|(p, n)| {
        let ad = p.file_name().map(|x| x.to_string_lossy().to_string()).unwrap_or_default();
        let kl = p.parent().and_then(|x| x.file_name()).map(|x| x.to_string_lossy().to_string()).unwrap_or_default();
        format!("{{\"yol\":\"{}\",\"ad\":\"{}\",\"klasor\":\"{}\",\"boyut\":{n},\"resim\":{}}}", jk(&p.display().to_string()), jk(&ad), jk(&kl), d::resim_mi(&p))
    }).collect();
    format!("{{\"kaynaklar\":[{}]}}", v.join(","))
}

fn durum_json() -> String {
    let s = d::durum();
    let yuzde = if s.toplam > 0 { (s.kare as f64 * 100.0 / s.toplam as f64).min(100.0) } else { 0.0 };
    format!("{{\"calisiyor\":{},\"kare\":{},\"toplam\":{},\"yuzde\":{yuzde:.1},\"asama\":\"{}\",\"dosya\":\"{}\",\"mesaj\":\"{}\",\"hata\":{}}}", s.calisiyor, s.kare, s.toplam, jk(&s.asama), jk(&s.dosya), jk(&s.mesaj), s.hata)
}

/// İstek gövdesini çözer: (ayar, doğrulanmış kaynak, alanlar).
fn istek(govde: &str, klasor: &Path) -> Result<(d::Ayar, std::path::PathBuf, Vec<(String, String)>), String> {
    let alan = json_duz(govde).ok_or("bozuk istek (JSON nesnesi bekleniyor)")?;
    let a = d::ayar_coz(&alan)?;
    let kaynak = alan.iter().find(|(k, _)| k == "kaynak").map(|(_, v)| v.as_str()).unwrap_or("");
    let p = d::kaynak_dogrula(kaynak, &d::kokler(klasor))?;
    Ok((a, p, alan))
}

/// `/donustur/...` isteklerini işler. `yol`: önek sonrası. `klasor`: program çıktı klasörü. Dönüş: (kod, içerik türü, gövde).
pub fn isle(yontem: &str, yol: &str, govde: &str, klasor: &Path) -> (u16, &'static str, String) {
    if govde.len() > GOVDE_MAX { return hata(400, "istek çok büyük (en çok 8 KB)"); }
    match (yontem, yol) {
        ("GET", "donustur.js") => (200, "text/javascript; charset=utf-8", BETIK.to_string()),
        ("GET", "turler") => (200, JSON, turler_json()),
        ("GET", "kaynaklar") => (200, JSON, kaynaklar_json(klasor)),
        ("GET", "durum") => (200, JSON, durum_json()),
        ("POST", "onizle") => match istek(govde, klasor) {
            Ok((a, p, alan)) => {
                let t = alan.iter().find(|(k, _)| k == "t").and_then(|(_, v)| v.parse::<f32>().ok()).filter(|x| x.is_finite() && *x >= 0.0 && *x <= 36000.0).unwrap_or(0.0);
                let kucuk = alan.iter().any(|(k, v)| k == "kucuk" && v == "true");
                match d::onizle(&a, &p, t, kucuk) {
                    Ok(png) => (200, JSON, format!("{{\"tamam\":true,\"png\":\"data:image/png;base64,{}\"}}", base64(&png))),
                    Err(e) => hata(500, &e),
                }
            }
            Err(e) => hata(400, &e),
        },
        ("POST", "baslat") => match istek(govde, klasor) {
            Ok((a, p, _)) => match d::baslat(a, p, klasor) {
                Ok(c) => (200, JSON, format!("{{\"tamam\":true,\"dosya\":\"{}\"}}", jk(&c.display().to_string()))),
                Err(e) if e.contains("zaten") => hata(409, &e),
                Err(e) => hata(400, &e),
            },
            Err(e) => hata(400, &e),
        },
        ("POST", "iptal") => { d::iptal(); (200, JSON, "{\"tamam\":true}".into()) }
        ("POST", "oynat") => match d::son_cikti(klasor) {
            Some(p) => { d::sistemde_ac(&p); (200, JSON, "{\"tamam\":true}".into()) }
            None => hata(404, "oynatılacak dönüştürülmüş video yok"),
        },
        ("POST", "klasor") => { d::sistemde_ac(klasor); (200, JSON, "{\"tamam\":true}".into()) }
        _ => hata(404, "yok"),
    }
}

#[cfg(test)]
mod t {
    use super::*;
    #[test]
    fn json_cozumleme() {
        let v = json_duz(r#"{"tur":"piksel","blok":10,"kucuk":true,"kaynak":"C:\\a b\\c.mp4","x":"\u00fc"}"#).unwrap();
        assert_eq!(v[0], ("tur".into(), "piksel".into()));
        assert_eq!(v[1].1, "10"); assert_eq!(v[2].1, "true");
        assert_eq!(v[3].1, r"C:\a b\c.mp4"); assert_eq!(v[4].1, "ü");
        assert_eq!(json_duz("{}"), Some(vec![]));
        for bozuk in ["", "[]", "{", "{\"a\"}", "{\"a\":}", "{\"a\":1,}", "{\"a\":{\"b\":1}}", "{\"a\":[1]}", "{\"a\":abc}", "{\"a\":\"x}", "{\"a\":1} x"] {
            assert!(json_duz(bozuk).is_none(), "{bozuk}");
        }
    }
    #[test]
    fn base64_dogru() { assert_eq!(base64(b"Man"), "TWFu"); assert_eq!(base64(b"Ma"), "TWE="); assert_eq!(base64(b"M"), "TQ=="); assert_eq!(base64(b""), ""); }
    #[test]
    fn bozuk_istekler_400() {
        let k = std::env::temp_dir();
        assert_eq!(isle("POST", "onizle", "bozuk", &k).0, 400);
        assert_eq!(isle("POST", "baslat", "{\"tur\":\"piksel\"}", &k).0, 400, "kaynak yok");
        assert_eq!(isle("POST", "baslat", "{\"tur\":\"yok\",\"kaynak\":\"C:\\\\x.mp4\"}", &k).0, 400);
        assert_eq!(isle("POST", "onizle", &"x".repeat(GOVDE_MAX + 1), &k).0, 400);
        assert_eq!(isle("GET", "bilinmeyen", "", &k).0, 404);
        assert_eq!(isle("DELETE", "durum", "", &k).0, 404);
        let r = isle("POST", "onizle", "{\"tur\":\"cizgifilm\",\"kaynak\":\"C:\\\\Windows\\\\win.ini\"}", &k);
        assert_eq!(r.0, 400); assert!(r.2.contains("\"tamam\":false"));
    }
    #[test]
    fn turler_ve_durum_json() {
        let k = std::env::temp_dir();
        let (kod, tur, g) = isle("GET", "turler", "", &k);
        assert_eq!((kod, tur), (200, JSON));
        assert!(json_benzeri(&g) && g.contains("\"cizgifilm\"") && g.contains("\"retro16\"") && g.contains("\"lacivert-altin\""));
        let (_, _, dj) = isle("GET", "durum", "", &k);
        assert!(dj.contains("\"calisiyor\":false"));
        let (_, t, b) = isle("GET", "donustur.js", "", &k);
        assert!(t.starts_with("text/javascript") && b.contains("TaaDonustur"));
    }
    fn json_benzeri(s: &str) -> bool { s.starts_with('{') && s.ends_with('}') && s.matches('{').count() == s.matches('}').count() }
}
