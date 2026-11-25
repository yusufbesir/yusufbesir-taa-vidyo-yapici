//! Oynatıcı medya sunumu: çıktı klasörü listesi, güvenli ad çözme (yol geçişi koruması), Range ayrıştırma, akışlı yanıt.
//! Sahip: Yusuf. Yazan: [Claude#Oynatici Opus 5.5, yenileme-1.0, 2026-11-25]. Yalnız yazan yapay zekâ ya da Yusuf'un izniyle değişir; silme yok.
//! Bağlayıcı karar: tasarim/yenileme/karar-oynatici.md 2.6. Dosya belleğe alınmaz; 64 KB parçalarla yazılır. Silme ucu YOK.
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

pub const UZANTILAR: &[&str] = &["mp4", "webm", "png", "jpg", "jpeg", "webp"];
pub const MAX_LISTE: usize = 2000;
pub const MAX_BAGLANTI: usize = 32;
pub const MAX_AD: usize = 200;
const PARCA: usize = 64 * 1024;
static ACIK_BAGLANTI: AtomicUsize = AtomicUsize::new(0);

/// Uzantıya göre (MIME, "video" | "resim"). Beyaz liste dışı = None.
pub fn tur(ad: &str) -> Option<(&'static str, &'static str)> {
    let uz = ad.rsplit_once('.')?.1.to_lowercase();
    Some(match uz.as_str() {
        "mp4" => ("video/mp4", "video"),
        "webm" => ("video/webm", "video"),
        "png" => ("image/png", "resim"),
        "jpg" | "jpeg" => ("image/jpeg", "resim"),
        "webp" => ("image/webp", "resim"),
        _ => return None,
    })
}

/// Yüzde kodunu BİR kez çözer ('+' olduğu gibi kalır: yol bileşeni).
pub fn yuzde_coz(s: &str) -> String {
    let b = s.as_bytes();
    let (mut o, mut i) = (Vec::with_capacity(b.len()), 0);
    while i < b.len() {
        if b[i] == b'%' && i + 2 < b.len() {
            if let Some(v) = std::str::from_utf8(&b[i + 1..i + 3]).ok().and_then(|h| u8::from_str_radix(h, 16).ok()) { o.push(v); i += 3; continue; }
        }
        o.push(b[i]);
        i += 1;
    }
    String::from_utf8_lossy(&o).into_owned()
}

/// Karar 2.6 madde 2-3: ad kuralları (kök dışına çıkışı daha dosya sistemine gitmeden keser).
pub fn ad_gecerli(ad: &str) -> bool {
    if ad.is_empty() || ad.chars().count() > MAX_AD { return false; }
    if ad.chars().any(|c| matches!(c, '/' | '\\' | ':' | '<' | '>' | '"' | '|' | '?' | '*') || c.is_control()) { return false; }
    if ad.starts_with('.') || ad.contains("..") || ad.ends_with(' ') || ad.ends_with('.') { return false; }
    tur(ad).is_some()
}

/// Adı çıktı kökünde gerçek dosyaya çözer. Bağlantı (symlink/junction) ile kök dışına çıkış da kapanır: canonicalize sonrası üst klasör kök olmalı.
pub fn coz(kok: &Path, ad: &str) -> Option<PathBuf> {
    if !ad_gecerli(ad) { return None; }
    let kok = std::fs::canonicalize(kok).ok()?;
    let tam = std::fs::canonicalize(kok.join(ad)).ok()?;
    if tam.parent()? != kok.as_path() || !tam.is_file() { return None; }
    Some(tam)
}

#[derive(Debug, PartialEq)]
pub enum Aralik { Tam, Parca(u64, u64), Gecersiz }

/// Tek aralık: `bytes=a-b`, `bytes=a-`, `bytes=-n`. Çoklu ya da bozuk başlık = Tam (200). Dosya dışı = Gecersiz (416).
pub fn aralik_coz(baslik: Option<&str>, boyut: u64) -> Aralik {
    let Some(h) = baslik else { return Aralik::Tam };
    let h = h.trim();
    let Some(r) = h.strip_prefix("bytes=").or_else(|| h.strip_prefix("Bytes=")) else { return Aralik::Tam };
    if r.contains(',') { return Aralik::Tam; }
    let Some((a, b)) = r.trim().split_once('-') else { return Aralik::Tam };
    let (a, b) = (a.trim(), b.trim());
    let sayi = |x: &str| if !x.is_empty() && x.bytes().all(|c| c.is_ascii_digit()) { x.parse::<u64>().ok() } else { None };
    if a.is_empty() {
        let Some(n) = sayi(b) else { return Aralik::Tam };
        if n == 0 || boyut == 0 { return Aralik::Gecersiz; }
        return Aralik::Parca(boyut.saturating_sub(n), boyut - 1);
    }
    let Some(bas) = sayi(a) else { return Aralik::Tam };
    if bas >= boyut { return Aralik::Gecersiz; }
    if b.is_empty() { return Aralik::Parca(bas, boyut - 1); }
    let Some(son) = sayi(b) else { return Aralik::Tam };
    if son < bas { return Aralik::Gecersiz; }
    Aralik::Parca(bas, son.min(boyut - 1))
}

fn durum_adi(kod: u16) -> &'static str {
    match kod { 200 => "OK", 206 => "Partial Content", 404 => "Not Found", 416 => "Range Not Satisfiable", 503 => "Service Unavailable", _ => "Error" }
}

fn kisa<W: Write>(w: &mut W, kod: u16, ek: &str) {
    let g = match kod { 404 => "yok", 416 => "aralik gecersiz", 503 => "mesgul", _ => "hata" };
    let _ = write!(w, "HTTP/1.1 {kod} {}\r\nContent-Type: text/plain; charset=utf-8\r\nContent-Length: {}\r\nX-Content-Type-Options: nosniff\r\nCache-Control: no-store\r\n{ek}Connection: close\r\n\r\n{g}", durum_adi(kod), g.len());
}

struct Sayac;
impl Sayac {
    fn al() -> Option<Sayac> {
        if ACIK_BAGLANTI.fetch_add(1, Ordering::SeqCst) >= MAX_BAGLANTI { ACIK_BAGLANTI.fetch_sub(1, Ordering::SeqCst); return None; }
        Some(Sayac)
    }
}
impl Drop for Sayac { fn drop(&mut self) { ACIK_BAGLANTI.fetch_sub(1, Ordering::SeqCst); } }

/// Medya yanıtı. `ham_ad`: yoldaki `/oynatici/medya/` sonrası (sorgusuz, çözülmemiş). `yalniz_resim`: küçük resim ucu için.
pub fn yaz<W: Write>(w: &mut W, kok: &Path, ham_ad: &str, head: bool, range: Option<&str>, yalniz_resim: bool) {
    let ad = yuzde_coz(ham_ad.split('?').next().unwrap_or(""));
    let Some(yol) = coz(kok, &ad) else { return kisa(w, 404, "") };
    let (mime, t) = tur(&ad).unwrap_or(("application/octet-stream", ""));
    if yalniz_resim && t != "resim" { return kisa(w, 404, ""); }
    let Some(_s) = Sayac::al() else { return kisa(w, 503, "") };
    let Ok(mut f) = std::fs::File::open(&yol) else { return kisa(w, 404, "") };
    let boyut = f.metadata().map(|m| m.len()).unwrap_or(0);
    let (kod, bas, son) = match aralik_coz(range, boyut) {
        Aralik::Tam => (200u16, 0u64, boyut.saturating_sub(1)),
        Aralik::Parca(a, b) => (206, a, b),
        Aralik::Gecersiz => return kisa(w, 416, &format!("Content-Range: bytes */{boyut}\r\n")),
    };
    let uzun = if boyut == 0 { 0 } else { son - bas + 1 };
    let ek = if kod == 206 { format!("Content-Range: bytes {bas}-{son}/{boyut}\r\n") } else { String::new() };
    let b = format!("HTTP/1.1 {kod} {}\r\nContent-Type: {mime}\r\nContent-Length: {uzun}\r\nAccept-Ranges: bytes\r\n{ek}X-Content-Type-Options: nosniff\r\nCache-Control: no-store\r\nConnection: close\r\n\r\n", durum_adi(kod));
    if w.write_all(b.as_bytes()).is_err() || head || uzun == 0 { return; }
    if f.seek(SeekFrom::Start(bas)).is_err() { return; }
    let mut kalan = uzun;
    let mut tampon = vec![0u8; PARCA];
    while kalan > 0 {
        let n = (kalan.min(PARCA as u64)) as usize;
        match f.read(&mut tampon[..n]) {
            Ok(0) | Err(_) => return,
            Ok(k) => { if w.write_all(&tampon[..k]).is_err() { return; } kalan -= k as u64; }
        }
    }
    let _ = w.flush();
}

pub fn json_kacis(s: &str) -> String {
    let mut o = String::with_capacity(s.len() + 2);
    for c in s.chars() {
        match c { '"' => o.push_str("\\\""), '\\' => o.push_str("\\\\"), '\n' => o.push_str("\\n"), '\r' => o.push_str("\\r"), '\t' => o.push_str("\\t"), c if (c as u32) < 0x20 => o.push_str(&format!("\\u{:04x}", c as u32)), c => o.push(c) }
    }
    o
}

/// Çıktı klasörü listesi (JSON dizi): ad, tur, boyut, zaman (unix sn), oynatilabilir, neden. Yeni dosya önce. En çok 2000.
pub fn liste(kok: &Path) -> String {
    let mut v: Vec<(u64, String)> = vec![];
    for e in std::fs::read_dir(kok).into_iter().flatten().flatten() {
        let Ok(m) = e.metadata() else { continue };
        if !m.is_file() { continue; }
        let ad = e.file_name().to_string_lossy().to_string();
        let zaman = m.modified().ok().and_then(|z| z.duration_since(std::time::UNIX_EPOCH).ok()).map(|d| d.as_secs()).unwrap_or(0);
        let alt = ad.to_lowercase();
        let satir = if alt.ends_with(".mov") {
            format!("{{\"ad\":\"{}\",\"tur\":\"video\",\"boyut\":{},\"zaman\":{zaman},\"oynatilabilir\":false,\"neden\":\"HAP .mov bu oynatıcıda çözülmez; mp4 olarak üretin.\"}}", json_kacis(&ad), m.len())
        } else if let Some((_, t)) = tur(&ad) {
            if !ad_gecerli(&ad) {
                format!("{{\"ad\":\"{}\",\"tur\":\"{t}\",\"boyut\":{},\"zaman\":{zaman},\"oynatilabilir\":false,\"neden\":\"Dosya adı desteklenmiyor (nokta/boşlukla bitiyor ya da özel karakter var).\"}}", json_kacis(&ad), m.len())
            } else {
                format!("{{\"ad\":\"{}\",\"tur\":\"{t}\",\"boyut\":{},\"zaman\":{zaman},\"oynatilabilir\":true}}", json_kacis(&ad), m.len())
            }
        } else { continue };
        v.push((zaman, satir));
    }
    v.sort_by(|a, b| b.0.cmp(&a.0));
    v.truncate(MAX_LISTE);
    format!("[{}]", v.into_iter().map(|x| x.1).collect::<Vec<_>>().join(","))
}

#[cfg(test)]
mod t {
    use super::*;
    #[test]
    fn ad_tablosu() {
        for kotu in ["..\\x.mp4", "a/b.mp4", "C:x.mp4", "x.mp4.", "x.mp4 ", ".gizli.mp4", "x.exe", "", "a\0.mp4", "x", "a|b.mp4"] { assert!(!ad_gecerli(kotu), "{kotu:?} kabul edilmemeli"); }
        assert!(!ad_gecerli(&format!("{}.mp4", "a".repeat(197))));
        assert!(ad_gecerli(&format!("{}.mp4", "a".repeat(196))));
        for iyi in ["x.MP4", "Kış Kampanyası.mp4", "a b.webm", "r.jpeg", "r+1.png"] { assert!(ad_gecerli(iyi), "{iyi:?} kabul edilmeli"); }
        assert_eq!(yuzde_coz("%2e%2e%2fx.mp4"), "../x.mp4");
        assert!(!ad_gecerli(&yuzde_coz("%2e%2e%2fx.mp4")));
        assert_eq!(yuzde_coz("%252e%252e"), "%2e%2e");
        assert_eq!(yuzde_coz("a+b%20c%"), "a+b c%");
        assert_eq!(yuzde_coz("K%C4%B1%C5%9F.mp4"), "Kış.mp4");
    }
    #[test]
    fn aralik_tablosu() {
        let n = 1000;
        assert_eq!(aralik_coz(None, n), Aralik::Tam);
        assert_eq!(aralik_coz(Some("bytes=0-"), n), Aralik::Parca(0, 999));
        assert_eq!(aralik_coz(Some("bytes=0-0"), n), Aralik::Parca(0, 0));
        assert_eq!(aralik_coz(Some("bytes=-500"), n), Aralik::Parca(500, 999));
        assert_eq!(aralik_coz(Some("bytes=-5000"), n), Aralik::Parca(0, 999));
        assert_eq!(aralik_coz(Some("bytes=500-"), n), Aralik::Parca(500, 999));
        assert_eq!(aralik_coz(Some("bytes=10-5000"), n), Aralik::Parca(10, 999));
        assert_eq!(aralik_coz(Some("bytes=1000-"), n), Aralik::Gecersiz);
        assert_eq!(aralik_coz(Some("bytes=5-2"), n), Aralik::Gecersiz);
        assert_eq!(aralik_coz(Some("bytes=-0"), n), Aralik::Gecersiz);
        assert_eq!(aralik_coz(Some("bytes=0-1,5-6"), n), Aralik::Tam);
        assert_eq!(aralik_coz(Some("bytes=x-y"), n), Aralik::Tam);
        assert_eq!(aralik_coz(Some("items=0-1"), n), Aralik::Tam);
        assert_eq!(aralik_coz(Some("bytes=0-"), 0), Aralik::Gecersiz);
    }
}
