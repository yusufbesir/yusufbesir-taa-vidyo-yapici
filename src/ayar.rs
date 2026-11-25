//! Kalıcı ayar deposu (0.9.8). Sahip: Yusuf. Yazan: [Claude#Sonnet-5.5, 2026-11-18]. Yalnız yazan yapay zekâ ya da Yusuf'un izniyle değişir; silme yok.
//!
//! Neden: arayüz her açılışta farklı bir yerel portta çalışır; tarayıcı localStorage'ı porta bağlı olduğundan ayarlar kaybolurdu.
//! Sayfa, `taa-` ile başlayan localStorage anahtarlarını bu depoya yazar (`GET/POST /ayar`) ve açılışta buradan geri yükler.
//! Dosya: %LOCALAPPDATA%\TAA Vidyo Yapici\ayar.json (TAA_AYAR_DOSYASI ortam değişkeniyle değiştirilebilir). Biçim: düz JSON nesnesi, anahtar -> metin.
//! Güvenlik: anahtar yalnız `taa-` + [a-z0-9-_]; boyut sınırları; bozuk/aşırı büyük dosya = boş ayar; yazma geçici dosya + yeniden adlandırma ile atomik.
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

pub const MAX_ANAHTAR: usize = 64;
pub const MAX_DEGER: usize = 16 * 1024;
pub const MAX_ADET: usize = 64;
pub const MAX_DOSYA: u64 = 256 * 1024;
static KILIT: Mutex<()> = Mutex::new(());

pub type Ayar = BTreeMap<String, String>;

pub fn varsayilan_yol() -> PathBuf {
    if let Some(p) = std::env::var_os("TAA_AYAR_DOSYASI") { return PathBuf::from(p); }
    let kok = std::env::var_os("LOCALAPPDATA").map(PathBuf::from).unwrap_or_else(std::env::temp_dir);
    kok.join("TAA Vidyo Yapici").join("ayar.json")
}

pub fn anahtar_gecerli(k: &str) -> bool {
    k.starts_with("taa-") && k.len() > 4 && k.len() <= MAX_ANAHTAR && k.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-' || b == b'_')
}

fn kacis(s: &str, o: &mut String) {
    o.push('"');
    for c in s.chars() {
        match c {
            '"' => o.push_str("\\\""),
            '\\' => o.push_str("\\\\"),
            '\n' => o.push_str("\\n"),
            '\r' => o.push_str("\\r"),
            '\t' => o.push_str("\\t"),
            c if (c as u32) < 0x20 => o.push_str(&format!("\\u{:04x}", c as u32)),
            c => o.push(c),
        }
    }
    o.push('"');
}

pub fn json_yaz(a: &Ayar) -> String {
    let mut o = String::from("{");
    for (i, (k, v)) in a.iter().enumerate() {
        if i > 0 { o.push(','); }
        kacis(k, &mut o); o.push(':'); kacis(v, &mut o);
    }
    o.push('}');
    o
}

struct Cozucu<'a> { b: &'a [u8], i: usize }
impl<'a> Cozucu<'a> {
    fn bosluk(&mut self) { while self.i < self.b.len() && matches!(self.b[self.i], b' ' | b'\t' | b'\r' | b'\n') { self.i += 1; } }
    fn bekle(&mut self, c: u8) -> Option<()> { self.bosluk(); if self.b.get(self.i) == Some(&c) { self.i += 1; Some(()) } else { None } }
    fn hex4(&mut self) -> Option<u32> {
        let s = self.b.get(self.i..self.i + 4)?;
        let t = std::str::from_utf8(s).ok()?;
        let v = u32::from_str_radix(t, 16).ok()?;
        self.i += 4;
        Some(v)
    }
    fn metin(&mut self) -> Option<String> {
        self.bekle(b'"')?;
        let mut o: Vec<u8> = Vec::new();
        loop {
            let c = *self.b.get(self.i)?;
            self.i += 1;
            match c {
                b'"' => return String::from_utf8(o).ok(),
                b'\\' => {
                    let e = *self.b.get(self.i)?;
                    self.i += 1;
                    let ch: char = match e {
                        b'"' => '"', b'\\' => '\\', b'/' => '/', b'n' => '\n', b'r' => '\r', b't' => '\t', b'b' => '\u{8}', b'f' => '\u{c}',
                        b'u' => {
                            let a = self.hex4()?;
                            if (0xD800..0xDC00).contains(&a) {
                                if self.b.get(self.i) != Some(&b'\\') || self.b.get(self.i + 1) != Some(&b'u') { return None; }
                                self.i += 2;
                                let b2 = self.hex4()?;
                                if !(0xDC00..0xE000).contains(&b2) { return None; }
                                char::from_u32(0x10000 + ((a - 0xD800) << 10) + (b2 - 0xDC00))?
                            } else { char::from_u32(a)? } // yalnız kalan alt vekil (DC00..DFFF) geçersiz sayılır
                        }
                        _ => return None,
                    };
                    let mut tmp = [0u8; 4];
                    o.extend_from_slice(ch.encode_utf8(&mut tmp).as_bytes());
                }
                c if c < 0x20 => return None,
                c => o.push(c),
            }
        }
    }
    fn bos_mu(&mut self) -> bool { self.bosluk(); if self.b.get(self.i..self.i + 4) == Some(b"null") { self.i += 4; true } else { false } }
}

/// Düz nesne ayrıştırır: değerler metin, `bos_izin` ise null da olabilir (None). Başka tür ya da fazla içerik = hata.
fn nesne_coz(s: &str, bos_izin: bool) -> Option<Vec<(String, Option<String>)>> {
    let mut c = Cozucu { b: s.as_bytes(), i: 0 };
    c.bekle(b'{')?;
    let mut v = Vec::new();
    c.bosluk();
    if c.b.get(c.i) == Some(&b'}') { c.i += 1; } else {
        loop {
            let k = c.metin()?;
            c.bekle(b':')?;
            c.bosluk();
            let d = if bos_izin && c.bos_mu() { None } else { Some(c.metin()?) };
            v.push((k, d));
            c.bosluk();
            match c.b.get(c.i) { Some(b',') => { c.i += 1; } Some(b'}') => { c.i += 1; break; } _ => return None }
        }
    }
    c.bosluk();
    if c.i != c.b.len() { return None; }
    Some(v)
}

/// Dosya içeriğini doğrular: geçerli anahtar, sınırlar. Geçersizse None.
pub fn json_coz(s: &str) -> Option<Ayar> {
    let v = nesne_coz(s, false)?;
    if v.len() > MAX_ADET { return None; }
    let mut a = Ayar::new();
    for (k, d) in v {
        let d = d?;
        if !anahtar_gecerli(&k) || d.len() > MAX_DEGER { return None; }
        a.insert(k, d);
    }
    Some(a)
}

/// POST gövdesi: {"k":"taa-...","v":"metin"} ya da {"k":"taa-...","v":null} (silme).
pub fn istek_coz(s: &str) -> Option<(String, Option<String>)> {
    let v = nesne_coz(s, true)?;
    if v.len() != 2 { return None; }
    let k = v.iter().find(|x| x.0 == "k")?.1.clone()?;
    let d = v.iter().find(|x| x.0 == "v")?.1.clone();
    if !anahtar_gecerli(&k) { return None; }
    if let Some(x) = &d { if x.len() > MAX_DEGER { return None; } }
    Some((k, d))
}

/// Dosyayı okur; yok, bozuk, çok büyük ya da geçersizse boş ayar (güvenli varsayılan).
pub fn oku(yol: &Path) -> Ayar {
    match std::fs::metadata(yol) { Ok(m) if m.len() <= MAX_DOSYA => {} _ => return Ayar::new() }
    std::fs::read_to_string(yol).ok().and_then(|s| json_coz(s.trim_start_matches('\u{feff}'))).unwrap_or_default()
}

/// Atomik yazma: aynı klasörde geçici dosyaya yaz, sonra hedefin üzerine yeniden adlandır.
pub fn yaz_atomik(yol: &Path, a: &Ayar) -> Result<(), String> {
    let govde = json_yaz(a);
    if govde.len() as u64 > MAX_DOSYA { return Err("ayar dosyası çok büyük".into()); }
    if let Some(k) = yol.parent() { std::fs::create_dir_all(k).map_err(|e| e.to_string())?; }
    let gecici = yol.with_extension("json.tmp");
    std::fs::write(&gecici, govde.as_bytes()).map_err(|e| e.to_string())?;
    std::fs::rename(&gecici, yol).map_err(|e| { let _ = std::fs::remove_file(&gecici); e.to_string() })
}

/// Tek anahtarı ekler/günceller (Some) ya da siler (None). Eşzamanlı çağrılar kilitle sıralanır.
pub fn degistir(yol: &Path, k: &str, v: Option<&str>) -> Result<(), String> {
    if !anahtar_gecerli(k) { return Err("geçersiz anahtar".into()); }
    if let Some(x) = v { if x.len() > MAX_DEGER { return Err("değer çok büyük".into()); } }
    let _g = KILIT.lock().unwrap_or_else(|e| e.into_inner());
    let mut a = oku(yol);
    match v {
        Some(x) => { if !a.contains_key(k) && a.len() >= MAX_ADET { return Err("çok fazla ayar".into()); } a.insert(k.to_string(), x.to_string()); }
        None => { a.remove(k); }
    }
    yaz_atomik(yol, &a)
}

#[cfg(test)]
mod t {
    use super::*;
    fn gecici(ad: &str) -> PathBuf {
        let k = std::env::temp_dir().join(format!("taa-ayar-test-{}-{ad}", std::process::id()));
        let _ = std::fs::remove_dir_all(&k);
        std::fs::create_dir_all(&k).unwrap();
        k.join("ayar.json")
    }
    #[test]
    fn json_gidis_donus_ve_kacisler() {
        let mut a = Ayar::new();
        a.insert("taa-a".into(), "düz \"tırnak\" \\ ters\nsatır\tsekme \u{1}kontrol ğüşıöç 😀".into());
        a.insert("taa-b_1".into(), "".into());
        let s = json_yaz(&a);
        assert_eq!(json_coz(&s).unwrap(), a);
        assert_eq!(json_coz("{}").unwrap(), Ayar::new());
        assert_eq!(json_coz(" { \"taa-x\" : \"\\u00e7\\ud83d\\ude00\\/\" } ").unwrap().get("taa-x").unwrap(), "ç😀/");
    }
    #[test]
    fn bozuk_girdiler_reddedilir() {
        for s in ["", "{", "[]", "null", "{\"taa-a\":1}", "{\"taa-a\":null}", "{\"taa-a\":\"x\"} fazla", "{\"taa-a\":\"x\",}", "{\"x\":\"y\"}", "{\"taa-A\":\"y\"}",
                  "{\"taa-a\":\"\\ud83d\"}", "{\"taa-a\":\"\\udc00\"}", "{\"taa-a\":\"\\q\"}", "{\"taa-a\":\"x\ny\"}", "{\"taa-a\" \"x\"}", "{'taa-a':'x'}"] {
            assert!(json_coz(s).is_none(), "kabul edildi: {s}");
        }
    }
    #[test]
    fn anahtar_kurallari() {
        assert!(anahtar_gecerli("taa-acilis-ayar") && anahtar_gecerli("taa-renk-a") && anahtar_gecerli("taa-bina4"));
        for k in ["", "taa", "acilis", "taa-", "taa-Ab", "taa-a b", "taa-a/b", "taa-ü", "../taa-a", &format!("taa-{}", "a".repeat(70))] {
            assert!(!anahtar_gecerli(k), "kabul edildi: {k}");
        }
    }
    #[test]
    fn istek_cozme() {
        assert_eq!(istek_coz("{\"k\":\"taa-a\",\"v\":\"1\"}"), Some(("taa-a".into(), Some("1".into()))));
        assert_eq!(istek_coz("{\"v\":null,\"k\":\"taa-a\"}"), Some(("taa-a".into(), None)));
        for s in ["{\"k\":\"taa-a\"}", "{\"k\":\"zzz\",\"v\":\"1\"}", "{\"k\":null,\"v\":\"1\"}", "{\"k\":\"taa-a\",\"v\":5}", "{\"k\":\"taa-a\",\"v\":\"1\",\"x\":\"y\"}", "x"] {
            assert!(istek_coz(s).is_none(), "kabul edildi: {s}");
        }
        assert!(istek_coz(&format!("{{\"k\":\"taa-a\",\"v\":\"{}\"}}", "x".repeat(MAX_DEGER + 1))).is_none());
    }
    #[test]
    fn dosya_okuma_yazma_ve_silme() {
        let y = gecici("rw");
        assert!(oku(&y).is_empty(), "dosya yokken boş olmalı");
        degistir(&y, "taa-acilis-ayar", Some("{\"tam\":false,\"kapali\":[\"led\"]}")).unwrap();
        degistir(&y, "taa-tema", Some("b")).unwrap();
        let a = oku(&y);
        assert_eq!(a.len(), 2);
        assert_eq!(a["taa-tema"], "b");
        assert_eq!(a["taa-acilis-ayar"], "{\"tam\":false,\"kapali\":[\"led\"]}");
        degistir(&y, "taa-tema", Some("a")).unwrap();
        assert_eq!(oku(&y)["taa-tema"], "a");
        degistir(&y, "taa-tema", None).unwrap();
        assert!(!oku(&y).contains_key("taa-tema") && oku(&y).len() == 1);
        assert!(!y.with_extension("json.tmp").exists(), "geçici dosya kalmamalı");
        assert!(degistir(&y, "kotu", Some("x")).is_err());
        assert!(degistir(&y, "taa-x", Some(&"x".repeat(MAX_DEGER + 1))).is_err());
    }
    #[test]
    fn bozuk_dosya_guvenli_varsayilan_ve_kurtarma() {
        let y = gecici("bozuk");
        std::fs::write(&y, "{bozuk json").unwrap();
        assert!(oku(&y).is_empty());
        degistir(&y, "taa-a", Some("1")).unwrap(); // bozuk dosyanın üzerine temiz yazar
        assert_eq!(oku(&y)["taa-a"], "1");
        std::fs::write(&y, vec![b'x'; (MAX_DOSYA + 1) as usize]).unwrap();
        assert!(oku(&y).is_empty(), "aşırı büyük dosya yok sayılmalı");
        std::fs::write(&y, "{\"yabanci\":\"x\"}").unwrap();
        assert!(oku(&y).is_empty(), "geçersiz anahtarlı dosya yok sayılmalı");
        std::fs::write(&y, "\u{feff}{\"taa-a\":\"b\"}").unwrap();
        assert_eq!(oku(&y)["taa-a"], "b", "BOM'lu dosya okunmalı");
    }
    #[test]
    fn adet_siniri_ve_eszamanli_yazma() {
        let y = gecici("adet");
        for i in 0..MAX_ADET { degistir(&y, &format!("taa-k{i}"), Some("v")).unwrap(); }
        assert!(degistir(&y, "taa-fazla", Some("v")).is_err());
        degistir(&y, "taa-k0", Some("yeni")).unwrap(); // var olanı güncellemek serbest
        assert_eq!(oku(&y).len(), MAX_ADET);
        let y2 = gecici("es");
        let ip: Vec<_> = (0..8).map(|i| { let y = y2.clone(); std::thread::spawn(move || { for j in 0..8 { degistir(&y, &format!("taa-e{i}-{j}"), Some("v")).unwrap(); } }) }).collect();
        for t in ip { t.join().unwrap(); }
        assert_eq!(oku(&y2).len(), 64, "8 iş parçacığı x 8 yazma: hiçbiri kaybolmamalı, dosya geçerli kalmalı");
    }
}
