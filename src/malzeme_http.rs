//! Malzeme (LED içerik kitaplığı) HTTP uçları: liste, senaryo üretimi, küçük önizleme karesi ve web/malzeme.js.
//! Uçlar (arayuz.rs `/malzeme/` önekini buraya yollar; sorgu dizesi gelmez, parametreler JSON gövdede):
//!   GET  /malzeme/liste            → katalog JSON
//!   POST /malzeme/uret   {id,...}  → {"tamam":true,"senaryo":"malzeme ...","onizleme":"ekran 950x1800\n..."} ya da 400 {"tamam":false,"hata":"..."}
//!   POST /malzeme/kucuk  {id,...,t}→ text/plain "data:image/png;base64,..." (190x360 tek kare)
//!   GET  /malzeme/kucuk/<id>       → aynı, varsayılan parametrelerle
//!   GET  /malzeme/malzeme.js       → galeri betiği (window.TaaMalzeme)
//! Sahip: Yusuf. İlk iskelet: [Claude#Sonnet-5.5, 2026-11-18]; içerik: [Claude#Malzeme Opus 5.5, yenileme-1.0, 2026-11-18]. Yalnız yazan yapay zekâ ya da Yusuf'un izniyle değişir; silme yok.
use crate::malzeme::{self, json_kac};

pub const GOVDE_MAX: usize = 8 * 1024;
pub const KUCUK_W: u32 = 190;
pub const KUCUK_H: u32 = 360;
const JSON: &str = "application/json; charset=utf-8";
const METIN: &str = "text/plain; charset=utf-8";
pub const BETIK: &str = include_str!("../web/malzeme.js");

fn hata(kod: u16, m: &str) -> (u16, &'static str, String) { (kod, JSON, format!("{{\"tamam\":false,\"hata\":\"{}\"}}", json_kac(m))) }

/// `/malzeme/...` isteklerini işler. `yol`: önek sonrası (örn. "liste"). Dönüş: (durum kodu, içerik türü, gövde).
pub fn isle(yontem: &str, yol: &str, govde: &str) -> (u16, &'static str, String) {
    if govde.len() > GOVDE_MAX { return hata(400, "istek çok büyük (en çok 8 KB)"); }
    match (yontem, yol) {
        ("GET", "liste") => (200, JSON, malzeme::liste_json()),
        ("GET", "malzeme.js") => (200, "text/javascript; charset=utf-8", BETIK.to_string()),
        ("POST", "uret") => match malzeme::istek_coz(govde).and_then(|q| malzeme::senaryo_parcasi(&q).map(|p| (q, p))) {
            Ok((q, p)) => (200, JSON, format!("{{\"tamam\":true,\"id\":\"{}\",\"senaryo\":\"{}\",\"onizleme\":\"{}\"}}", json_kac(&q.id), json_kac(&p), json_kac(&malzeme::onizleme_senaryosu(&p)))),
            Err(e) => hata(400, &e),
        },
        ("POST", "kucuk") => {
            let t = malzeme::json_nesne(govde).and_then(|v| v.into_iter().find(|(k, _)| k == "t")).and_then(|(_, v)| v.parse::<f32>().ok()).filter(|x| x.is_finite());
            match malzeme::istek_coz(govde).and_then(|q| malzeme::senaryo_parcasi(&q)) {
                Ok(p) => kucuk_yanit(&p, t),
                Err(e) => hata(400, &e),
            }
        }
        ("GET", y) if y.starts_with("kucuk/") => match malzeme::senaryo_parcasi(&malzeme::Istek { id: y[6..].to_string(), ..Default::default() }) {
            Ok(p) => kucuk_yanit(&p, None),
            Err(e) => hata(404, &e),
        },
        (_, "liste") | (_, "uret") | (_, "kucuk") => hata(405, "yöntem desteklenmiyor"),
        _ => hata(404, "malzeme ucu yok"),
    }
}

fn kucuk_yanit(parca: &str, t: Option<f32>) -> (u16, &'static str, String) {
    match std::panic::catch_unwind(|| kucuk_png(parca, t)).unwrap_or_else(|_| Err("önizleme üretilemedi".into())) {
        Ok(p) => (200, METIN, format!("data:image/png;base64,{}", base64(&p))),
        Err(e) => hata(400, &e),
    }
}

/// Malzemenin temsilî anı (küçük önizleme için): girişler oturmuş, hareket görünür.
pub fn temsil_t(sure: f32) -> f32 { (sure * 0.45).clamp(0.5, 3.2) }

/// Senaryo parçasını 190x360 tek kare olarak render edip PNG döndürür.
pub fn kucuk_png(parca: &str, t: Option<f32>) -> Result<Vec<u8>, String> {
    let metin = format!("ekran {KUCUK_W}x{KUCUK_H}\nfps 25\n{parca}\n");
    let s = crate::uret::senaryo_oku(&metin)?;
    let f = crate::uret::font();
    let hz = crate::senaryo::hazirla(&f, &s);
    let toplam = crate::senaryo::toplam_sn(&s);
    let t = t.unwrap_or_else(|| temsil_t(toplam)).clamp(0.0, (toplam - 0.04).max(0.0));
    let k = crate::senaryo::kare_uret(&s, &hz, t, (t * 25.0) as u32);
    Ok(png_sikistir(&k, s.w, s.h))
}

// ------------------------------------------------------------- küçük PNG kodlayıcı (Sub süzgeci + sabit Huffman deflate, uzaklık 1 tekrarları)
fn crc32(v: &[u8]) -> u32 { let mut c = !0u32; for &b in v { c ^= b as u32; for _ in 0..8 { c = if c & 1 != 0 { 0xEDB8_8320 ^ (c >> 1) } else { c >> 1 }; } } !c }
fn adler(v: &[u8]) -> u32 { let (mut a, mut b) = (1u32, 0u32); for &x in v { a = (a + x as u32) % 65521; b = (b + a) % 65521; } (b << 16) | a }
struct Bit { o: Vec<u8>, acc: u32, n: u32 }
impl Bit {
    fn yaz(&mut self, v: u32, n: u32) { self.acc |= v << self.n; self.n += n; while self.n >= 8 { self.o.push(self.acc as u8); self.acc >>= 8; self.n -= 8; } }
    fn huff(&mut self, kod: u32, n: u32) { let mut r = 0; for i in 0..n { if kod & (1 << i) != 0 { r |= 1 << (n - 1 - i); } } self.yaz(r, n); }
    fn sembol(&mut self, s: u32) { match s { 0..=143 => self.huff(0x30 + s, 8), 144..=255 => self.huff(0x190 + s - 144, 9), 256..=279 => self.huff(s - 256, 7), _ => self.huff(0xC0 + s - 280, 8) } }
}
const UZ_TABAN: [u32; 29] = [3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 15, 17, 19, 23, 27, 31, 35, 43, 51, 59, 67, 83, 99, 115, 131, 163, 195, 227, 258];
const UZ_EK: [u32; 29] = [0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 0];
fn deflate(v: &[u8]) -> Vec<u8> {
    let mut b = Bit { o: vec![0x78, 0x01], acc: 0, n: 0 };
    b.yaz(1, 1); b.yaz(1, 2);
    let mut i = 0;
    while i < v.len() {
        let mut run = 0;
        if i > 0 { while run < 258 && i + run < v.len() && v[i + run] == v[i - 1] { run += 1; } }
        if run >= 3 {
            let k = UZ_TABAN.iter().rposition(|&t| t <= run as u32).unwrap();
            b.sembol(257 + k as u32);
            if UZ_EK[k] > 0 { b.yaz(run as u32 - UZ_TABAN[k], UZ_EK[k]); }
            b.huff(0, 5);
            i += run;
        } else { b.sembol(v[i] as u32); i += 1; }
    }
    b.sembol(256);
    if b.n > 0 { b.o.push(b.acc as u8); }
    b.o.extend(adler(v).to_be_bytes());
    b.o
}
fn parca(o: &mut Vec<u8>, tur: &[u8; 4], v: &[u8]) { o.extend((v.len() as u32).to_be_bytes()); let mut t = tur.to_vec(); t.extend(v); o.extend(&t); o.extend(crc32(&t).to_be_bytes()); }
/// RGB24 → sıkıştırılmış PNG.
pub fn png_sikistir(rgb: &[u8], w: u32, h: u32) -> Vec<u8> {
    let sat = w as usize * 3;
    let mut ham = Vec::with_capacity((sat + 1) * h as usize);
    for y in 0..h as usize {
        ham.push(1);
        let r = &rgb[y * sat..(y + 1) * sat];
        for x in 0..sat { ham.push(r[x].wrapping_sub(if x >= 3 { r[x - 3] } else { 0 })); }
    }
    let mut o = vec![0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A];
    let mut ihdr = vec![]; ihdr.extend(w.to_be_bytes()); ihdr.extend(h.to_be_bytes()); ihdr.extend([8, 2, 0, 0, 0]);
    parca(&mut o, b"IHDR", &ihdr);
    parca(&mut o, b"IDAT", &deflate(&ham));
    parca(&mut o, b"IEND", &[]);
    o
}
pub fn base64(v: &[u8]) -> String {
    const A: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut o = String::with_capacity(v.len() * 4 / 3 + 4);
    for c in v.chunks(3) {
        let n = (c[0] as u32) << 16 | (*c.get(1).unwrap_or(&0) as u32) << 8 | *c.get(2).unwrap_or(&0) as u32;
        o.push(A[(n >> 18) as usize & 63] as char); o.push(A[(n >> 12) as usize & 63] as char);
        o.push(if c.len() > 1 { A[(n >> 6) as usize & 63] as char } else { '=' });
        o.push(if c.len() > 2 { A[n as usize & 63] as char } else { '=' });
    }
    o
}

#[cfg(test)]
mod t {
    use super::*;
    #[test]
    fn uclar() {
        let (k, tur, g) = isle("GET", "liste", "");
        assert_eq!(k, 200); assert!(tur.starts_with("application/json") && g.contains("\"malzemeler\""));
        let (k, _, g) = isle("POST", "uret", r#"{"id":"geri-sayim","deger":5,"metin":"HAZIR"}"#);
        assert_eq!(k, 200, "{g}"); assert!(g.contains("malzeme geri-sayim") && g.contains("\"tamam\":true"));
        assert_eq!(isle("POST", "uret", "{bozuk").0, 400);
        assert_eq!(isle("POST", "uret", r#"{"id":"yok-boyle"}"#).0, 400);
        assert_eq!(isle("POST", "uret", &"x".repeat(GOVDE_MAX + 1)).0, 400);
        assert_eq!(isle("GET", "uret", "").0, 405);
        assert_eq!(isle("GET", "bilinmez", "").0, 404);
        assert!(isle("GET", "malzeme.js", "").2.contains("TaaMalzeme"));
    }
    #[test]
    fn base64_dogru() { assert_eq!(base64(b"Man"), "TWFu"); assert_eq!(base64(b"Ma"), "TWE="); assert_eq!(base64(b"M"), "TQ=="); }
}
