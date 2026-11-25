//! Katmanlı oynatıcı (Resolume Arena yerine) HTTP uçları: sayfalar, proje doğrulama ve kalıcılık, komut kuyruğu, uzun yoklama,
//! canlı durum, medya (oynatici_medya), sahne pişirme, çıkış penceresi köprüsü (oynatici_pencere).
//! Sahip: Yusuf. Yazan: [Claude#Oynatici Opus 5.5, yenileme-1.0, 2026-11-25]. Yalnız yazan yapay zekâ ya da Yusuf'un izniyle değişir; silme yok.
//! Bağlayıcı karar: tasarim/yenileme/karar-oynatici.md. Bağlanma: tasarim/yenileme/ISTEK-oynatici.md.
//! Güvenlik: yetki (Host, çerez, X-LED) arayuz.rs'te bu modüle gelmeden yapılır; burada yeni yetki yolu AÇILMAZ.
use crate::oynatici_medya as medya;
use std::collections::VecDeque;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Condvar, Mutex, OnceLock};
use std::time::{Duration, Instant};

pub const MAX_KATMAN: usize = 8;
pub const MAX_KLIP: usize = 200;
pub const MAX_PROJE: usize = 256 * 1024;
pub const MAX_METIN: usize = 200;
pub const MAX_KOMUT: usize = 64;
pub const MAX_CANLI: usize = 32 * 1024;
pub const MAX_BEKLEYEN: usize = 8;
pub const KIPLER: &[&str] = &["dongu", "sirali", "rastgele"];
pub const BITINCE: &[&str] = &["basa", "dur", "kapat"];
pub const KARISIM: &[&str] = &["normal", "ekle", "carp"];
pub const HEDEF: &[&str] = &["ikisi", "sol", "sag", "yayili"];
pub const SIGDIR: &[&str] = &["doldur", "sigdir", "esnet"];
pub const KOMUTLAR: &[&str] = &["sonraki", "onceki", "basa", "tetikle", "sar", "ileri", "geri", "yenile"];

const HTML_KONTROL: &str = include_str!("../web/oynatici.html");
const HTML_CIKIS: &str = include_str!("../web/cikis.html");
const DOSYALAR: &[(&str, &str, &str)] = &[
    ("oynatici.js", "text/javascript; charset=utf-8", include_str!("../web/oynatici.js")),
    ("oynatici.css", "text/css; charset=utf-8", include_str!("../web/oynatici.css")),
    ("oynatici-motor.js", "text/javascript; charset=utf-8", include_str!("../web/oynatici-motor.js")),
    ("oynatici-cikis.js", "text/javascript; charset=utf-8", include_str!("../web/oynatici-cikis.js")),
];
pub const CSP: &str = "default-src 'none'; script-src 'self'; style-src 'self'; img-src 'self' blob:; media-src 'self'; connect-src 'self'; base-uri 'none'; form-action 'none'; frame-ancestors 'none'";

// ---------------------------------------------------------------- JSON (256 KB'ye kadar; tarif::json_coz 64 KB ile sınırlı)
#[derive(Clone, Debug, PartialEq)]
pub enum J { Bos, M(bool), S(f64), Y(String), D(Vec<J>), N(Vec<(String, J)>) }
impl J {
    pub fn al(&self, k: &str) -> Option<&J> { if let J::N(v) = self { v.iter().find(|x| x.0 == k).map(|x| &x.1) } else { None } }
    fn sayi(&self) -> Option<f64> { if let J::S(x) = self { if x.is_finite() { Some(*x) } else { None } } else { None } }
    fn mantik(&self) -> Option<bool> { if let J::M(x) = self { Some(*x) } else { None } }
    fn yazi(&self) -> Option<&str> { if let J::Y(x) = self { Some(x) } else { None } }
}
struct Ayr<'a> { b: &'a [u8], i: usize, d: usize }
impl<'a> Ayr<'a> {
    fn bos(&mut self) { while self.i < self.b.len() && matches!(self.b[self.i], b' ' | b'\t' | b'\r' | b'\n') { self.i += 1; } }
    fn deger(&mut self) -> Option<J> {
        self.bos();
        if self.d > 16 { return None; }
        match *self.b.get(self.i)? {
            b'{' => {
                self.i += 1; self.d += 1;
                let mut v = vec![];
                self.bos();
                if self.b.get(self.i) == Some(&b'}') { self.i += 1; self.d -= 1; return Some(J::N(v)); }
                loop {
                    self.bos();
                    let k = self.metin()?;
                    self.bos();
                    if self.b.get(self.i) != Some(&b':') { return None; }
                    self.i += 1;
                    let d = self.deger()?;
                    if !v.iter().any(|(a, _): &(String, J)| *a == k) { v.push((k, d)); }
                    self.bos();
                    match self.b.get(self.i)? { b',' => self.i += 1, b'}' => { self.i += 1; self.d -= 1; return Some(J::N(v)); } _ => return None }
                }
            }
            b'[' => {
                self.i += 1; self.d += 1;
                let mut v = vec![];
                self.bos();
                if self.b.get(self.i) == Some(&b']') { self.i += 1; self.d -= 1; return Some(J::D(v)); }
                loop {
                    v.push(self.deger()?);
                    self.bos();
                    match self.b.get(self.i)? { b',' => self.i += 1, b']' => { self.i += 1; self.d -= 1; return Some(J::D(v)); } _ => return None }
                }
            }
            b'"' => self.metin().map(J::Y),
            b't' if self.b[self.i..].starts_with(b"true") => { self.i += 4; Some(J::M(true)) }
            b'f' if self.b[self.i..].starts_with(b"false") => { self.i += 5; Some(J::M(false)) }
            b'n' if self.b[self.i..].starts_with(b"null") => { self.i += 4; Some(J::Bos) }
            b'-' | b'0'..=b'9' => {
                let b = self.i;
                while self.i < self.b.len() && matches!(self.b[self.i], b'-' | b'+' | b'.' | b'e' | b'E' | b'0'..=b'9') { self.i += 1; }
                std::str::from_utf8(&self.b[b..self.i]).ok()?.parse::<f64>().ok().filter(|x| x.is_finite()).map(J::S)
            }
            _ => None,
        }
    }
    fn hex4(&mut self) -> Option<u32> {
        let s = std::str::from_utf8(self.b.get(self.i..self.i + 4)?).ok()?;
        self.i += 4;
        u32::from_str_radix(s, 16).ok()
    }
    fn metin(&mut self) -> Option<String> {
        if self.b.get(self.i) != Some(&b'"') { return None; }
        self.i += 1;
        let mut o: Vec<u8> = vec![];
        loop {
            let c = *self.b.get(self.i)?;
            self.i += 1;
            match c {
                b'"' => return String::from_utf8(o).ok(),
                b'\\' => {
                    let e = *self.b.get(self.i)?;
                    self.i += 1;
                    let ch = match e {
                        b'"' => '"', b'\\' => '\\', b'/' => '/', b'b' => '\u{8}', b'f' => '\u{c}', b'n' => '\n', b'r' => '\r', b't' => '\t',
                        b'u' => {
                            let a = self.hex4()?;
                            if (0xD800..0xDC00).contains(&a) {
                                if self.b.get(self.i..self.i + 2) != Some(b"\\u") { return None; }
                                self.i += 2;
                                let b = self.hex4()?;
                                if !(0xDC00..0xE000).contains(&b) { return None; }
                                char::from_u32(0x10000 + ((a - 0xD800) << 10) + (b - 0xDC00))?
                            } else { char::from_u32(a)? }
                        }
                        _ => return None,
                    };
                    let mut t = [0u8; 4];
                    o.extend_from_slice(ch.encode_utf8(&mut t).as_bytes());
                }
                c if c < 0x20 => return None,
                c => o.push(c),
            }
        }
    }
}
pub fn json_coz(s: &str) -> Option<J> {
    let mut a = Ayr { b: s.as_bytes(), i: 0, d: 0 };
    let d = a.deger()?;
    a.bos();
    if a.i != a.b.len() { return None; }
    Some(d)
}
fn k(s: &str) -> String { medya::json_kacis(s) }
fn sayi_yaz(x: f64) -> String { if x.fract() == 0.0 && x.abs() < 1e15 { format!("{}", x as i64) } else { format!("{:.4}", x).trim_end_matches('0').trim_end_matches('.').to_string() } }

// ---------------------------------------------------------------- Proje modeli ve doğrulama
#[derive(Clone, Debug, PartialEq)]
pub struct Klip { pub tur: String, pub ad: String, pub sure: f64 }
#[derive(Clone, Debug, PartialEq)]
pub struct Katman {
    pub id: u32, pub ad: String, pub acik: bool, pub solo: bool, pub opaklik: f64, pub karisim: String, pub hedef: String, pub sigdir: String,
    pub kip: String, pub bitince: String, pub hiz: f64, pub ses: f64, pub sessiz: bool, pub gecis_ms: u32, pub klipler: Vec<Klip>, pub etkin: usize, pub oynuyor: bool,
}
#[derive(Clone, Debug, PartialEq)]
pub struct Proje {
    pub ekran_gen: u32, pub ekran_yuk: u32, pub ekran_sayisi: u32, pub ana_ses: f64, pub karart: bool,
    pub cikis_monitor: u32, pub cikis_x: i32, pub cikis_y: i32, pub acilista_cikis_ac: bool, pub katmanlar: Vec<Katman>,
}
impl Default for Proje {
    fn default() -> Self {
        Proje { ekran_gen: 950, ekran_yuk: 1800, ekran_sayisi: 2, ana_ses: 1.0, karart: false, cikis_monitor: 0, cikis_x: 0, cikis_y: 0, acilista_cikis_ac: false, katmanlar: vec![katman_varsayilan(1, "Katman 1")] }
    }
}
pub fn katman_varsayilan(id: u32, ad: &str) -> Katman {
    Katman { id, ad: ad.into(), acik: true, solo: false, opaklik: 1.0, karisim: "normal".into(), hedef: "ikisi".into(), sigdir: "doldur".into(), kip: "sirali".into(), bitince: "basa".into(), hiz: 1.0, ses: 1.0, sessiz: true, gecis_ms: 0, klipler: vec![], etkin: 0, oynuyor: true }
}
fn kirp(x: f64, a: f64, b: f64) -> f64 { x.max(a).min(b) }
fn metin_kirp(s: &str) -> String { s.chars().filter(|c| !c.is_control()).take(MAX_METIN).collect() }
fn secim(j: Option<&J>, liste: &[&str], alan: &str) -> Result<String, String> {
    match j {
        None | Some(J::Bos) => Ok(liste[0].to_string()),
        Some(J::Y(s)) if liste.contains(&s.as_str()) => Ok(s.clone()),
        _ => Err(format!("{alan} geçersiz")),
    }
}
fn sayi_al(j: Option<&J>, vars: f64, a: f64, b: f64) -> f64 { kirp(j.and_then(|x| x.sayi()).unwrap_or(vars), a, b) }
fn mantik_al(j: Option<&J>, vars: bool) -> bool { j.and_then(|x| x.mantik()).unwrap_or(vars) }

/// Sunucu tarafı doğrulama (karar 2.8): sınırlar, beyaz listeler, sayılar kırpılır, bilinmeyen alan atılır, kötü klip adı reddedilir.
pub fn dogrula(j: &J) -> Result<Proje, String> {
    if !matches!(j, J::N(_)) { return Err("proje nesne olmalı".into()); }
    let d = Proje::default();
    let mut p = Proje {
        ekran_gen: sayi_al(j.al("ekranGen"), 950.0, 64.0, 4096.0).round() as u32,
        ekran_yuk: sayi_al(j.al("ekranYuk"), 1800.0, 64.0, 4096.0).round() as u32,
        ekran_sayisi: if sayi_al(j.al("ekranSayisi"), 2.0, 1.0, 2.0) < 1.5 { 1 } else { 2 },
        ana_ses: sayi_al(j.al("anaSes"), 1.0, 0.0, 1.0),
        karart: mantik_al(j.al("karart"), false),
        cikis_monitor: 0, cikis_x: 0, cikis_y: 0,
        acilista_cikis_ac: mantik_al(j.al("acilistaCikisAc"), false),
        katmanlar: vec![],
    };
    if let Some(c) = j.al("cikis") {
        p.cikis_monitor = sayi_al(c.al("monitor"), 0.0, 0.0, 16.0) as u32;
        p.cikis_x = sayi_al(c.al("x"), 0.0, -20000.0, 20000.0) as i32;
        p.cikis_y = sayi_al(c.al("y"), 0.0, -20000.0, 20000.0) as i32;
    }
    let katmanlar = match j.al("katmanlar") { Some(J::D(v)) => v.as_slice(), None => &[], _ => return Err("katmanlar dizi olmalı".into()) };
    if katmanlar.len() > MAX_KATMAN { return Err(format!("en çok {MAX_KATMAN} katman")); }
    let mut kullanilan: Vec<u32> = vec![];
    for (i, kj) in katmanlar.iter().enumerate() {
        if !matches!(kj, J::N(_)) { return Err("katman nesne olmalı".into()); }
        let mut id = kj.al("id").and_then(|x| x.sayi()).filter(|x| *x >= 1.0 && *x <= 2_147_483_647.0).map(|x| x as u32).unwrap_or(0);
        if id == 0 || kullanilan.contains(&id) { id = (1..).find(|n| !kullanilan.contains(n) && !katmanlar.iter().any(|q| q.al("id").and_then(|x| x.sayi()) == Some(*n as f64))).unwrap_or(1); }
        kullanilan.push(id);
        let klipler_j = match kj.al("klipler") { Some(J::D(v)) => v.as_slice(), None => &[], _ => return Err("klipler dizi olmalı".into()) };
        if klipler_j.len() > MAX_KLIP { return Err(format!("katman başına en çok {MAX_KLIP} klip")); }
        let mut klipler = vec![];
        for c in klipler_j {
            let ad = c.al("ad").and_then(|x| x.yazi()).unwrap_or("");
            if !medya::ad_gecerli(ad) { return Err(format!("klip adı geçersiz: {}", metin_kirp(ad))); }
            let tur = secim(c.al("tur"), &["video", "resim"], "klip türü")?;
            if medya::tur(ad).map(|x| x.1) != Some(tur.as_str()) { return Err(format!("klip türü uzantıyla uyuşmuyor: {}", metin_kirp(ad))); }
            klipler.push(Klip { sure: if tur == "resim" { sayi_al(c.al("sure"), 8.0, 1.0, 3600.0) } else { 0.0 }, tur, ad: ad.to_string() });
        }
        let ad = kj.al("ad").and_then(|x| x.yazi()).map(metin_kirp).filter(|s| !s.trim().is_empty()).unwrap_or_else(|| format!("Katman {}", i + 1));
        let etkin = sayi_al(kj.al("etkin"), 0.0, 0.0, MAX_KLIP as f64) as usize;
        p.katmanlar.push(Katman {
            id, ad,
            acik: mantik_al(kj.al("acik"), true), solo: mantik_al(kj.al("solo"), false),
            opaklik: sayi_al(kj.al("opaklik"), 1.0, 0.0, 1.0),
            karisim: secim(kj.al("karisim"), KARISIM, "karışım")?, hedef: secim(kj.al("hedef"), HEDEF, "hedef")?,
            sigdir: secim(kj.al("sigdir"), SIGDIR, "sığdırma")?, kip: secim(kj.al("kip").or(None), &["sirali", "dongu", "rastgele"], "kip")?,
            bitince: secim(kj.al("bitince"), BITINCE, "bitince")?,
            hiz: sayi_al(kj.al("hiz"), 1.0, 0.25, 2.0), ses: sayi_al(kj.al("ses"), 1.0, 0.0, 1.0), sessiz: mantik_al(kj.al("sessiz"), true),
            gecis_ms: sayi_al(kj.al("gecisMs"), 0.0, 0.0, 2000.0).round() as u32,
            etkin: if klipler.is_empty() { 0 } else { etkin.min(klipler.len() - 1) }, klipler,
            oynuyor: mantik_al(kj.al("oynuyor"), true),
        });
    }
    let _ = d;
    Ok(p)
}

pub fn proje_json(p: &Proje) -> String {
    let katmanlar: Vec<String> = p.katmanlar.iter().map(|c| {
        let klipler: Vec<String> = c.klipler.iter().map(|x| if x.tur == "resim" { format!("{{\"tur\":\"resim\",\"ad\":\"{}\",\"sure\":{}}}", k(&x.ad), sayi_yaz(x.sure)) } else { format!("{{\"tur\":\"video\",\"ad\":\"{}\"}}", k(&x.ad)) }).collect();
        format!("{{\"id\":{},\"ad\":\"{}\",\"acik\":{},\"solo\":{},\"opaklik\":{},\"karisim\":\"{}\",\"hedef\":\"{}\",\"sigdir\":\"{}\",\"kip\":\"{}\",\"bitince\":\"{}\",\"hiz\":{},\"ses\":{},\"sessiz\":{},\"gecisMs\":{},\"etkin\":{},\"oynuyor\":{},\"klipler\":[{}]}}",
            c.id, k(&c.ad), c.acik, c.solo, sayi_yaz(c.opaklik), c.karisim, c.hedef, c.sigdir, c.kip, c.bitince, sayi_yaz(c.hiz), sayi_yaz(c.ses), c.sessiz, c.gecis_ms, c.etkin, c.oynuyor, klipler.join(","))
    }).collect();
    format!("{{\"ekranGen\":{},\"ekranYuk\":{},\"ekranSayisi\":{},\"anaSes\":{},\"karart\":{},\"acilistaCikisAc\":{},\"cikis\":{{\"monitor\":{},\"x\":{},\"y\":{}}},\"katmanlar\":[{}]}}",
        p.ekran_gen, p.ekran_yuk, p.ekran_sayisi, sayi_yaz(p.ana_ses), p.karart, p.acilista_cikis_ac, p.cikis_monitor, p.cikis_x, p.cikis_y, katmanlar.join(","))
}

// ---------------------------------------------------------------- Kalıcılık
pub fn varsayilan_yol() -> PathBuf {
    if let Some(p) = std::env::var_os("TAA_OYNATICI_DOSYASI") { return PathBuf::from(p); }
    let kok = std::env::var_os("LOCALAPPDATA").map(PathBuf::from).unwrap_or_else(std::env::temp_dir);
    kok.join("TAA Vidyo Yapici").join("oynatici.json")
}
/// Yok = varsayılan. Bozuk ya da 256 KB üstü = varsayılan; bozuk dosyaya dokunulmaz, yanına `.bozuk-<zaman>` kopyası konur (silme yok).
pub fn yukle(yol: &Path) -> Proje {
    let Ok(m) = std::fs::metadata(yol) else { return Proje::default() };
    let sonuc = if m.len() as usize > MAX_PROJE { None } else { std::fs::read_to_string(yol).ok().and_then(|s| json_coz(s.trim_start_matches('\u{feff}'))).and_then(|j| dogrula(&j).ok()) };
    match sonuc {
        Some(p) => p,
        None => {
            let mut kopya = yol.as_os_str().to_owned();
            kopya.push(format!(".bozuk-{}", zaman_damgasi()));
            let _ = std::fs::copy(yol, PathBuf::from(kopya));
            gunluk(&format!("proje dosyası bozuk, varsayılan proje açıldı: {}", yol.display()));
            Proje::default()
        }
    }
}
pub fn kaydet(yol: &Path, p: &Proje) -> Result<(), String> {
    if let Some(u) = yol.parent() { std::fs::create_dir_all(u).map_err(|e| e.to_string())?; }
    let mut gecici = yol.as_os_str().to_owned();
    gecici.push(".gecici");
    let gecici = PathBuf::from(gecici);
    std::fs::write(&gecici, proje_json(p)).map_err(|e| e.to_string())?;
    std::fs::rename(&gecici, yol).map_err(|e| e.to_string())
}

/// UTC zaman damgası YYYYAAGG-SSDDSS (harici kitaplık yok).
pub fn zaman_damgasi() -> String {
    let s = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0) as i64;
    let (gun, sn) = (s.div_euclid(86400), s.rem_euclid(86400));
    // Howard Hinnant'ın civil_from_days algoritması
    let z = gun + 719468;
    let era = z.div_euclid(146097);
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = yoe + era * 400 + if m <= 2 { 1 } else { 0 };
    format!("{y:04}{m:02}{d:02}-{:02}{:02}{:02}", sn / 3600, sn % 3600 / 60, sn % 60)
}

/// Olay günlüğü (yalnız ekleme; 1 MB'yi geçince eskisi yeniden adlandırılır, silinmez). Dosya adı dışında kişisel veri yazılmaz.
pub fn gunluk(satir: &str) {
    let yol = varsayilan_yol().with_file_name("oynatici-gunluk.txt");
    if std::fs::metadata(&yol).map(|m| m.len() > 1_000_000).unwrap_or(false) {
        let _ = std::fs::rename(&yol, yol.with_file_name(format!("oynatici-gunluk-{}.txt", zaman_damgasi())));
    }
    if let Some(u) = yol.parent() { let _ = std::fs::create_dir_all(u); }
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(&yol) {
        let temiz: String = satir.chars().filter(|c| !c.is_control()).take(500).collect();
        let _ = writeln!(f, "{} {}", zaman_damgasi(), temiz);
    }
}

// ---------------------------------------------------------------- Ortak durum
pub struct Durum {
    pub surum: u64, pub proje: Proje, komutlar: VecDeque<(u64, String)>, pub komut_no: u64,
    canli: Option<(String, Instant)>, bekleyen: usize, etkin_kirli: bool, son_kayit: Instant, yol: PathBuf,
}
static ORTAK: OnceLock<(Mutex<Durum>, Condvar)> = OnceLock::new();
fn ortak() -> &'static (Mutex<Durum>, Condvar) {
    ORTAK.get_or_init(|| {
        let yol = varsayilan_yol();
        let proje = yukle(&yol);
        (Mutex::new(Durum { surum: 1, proje, komutlar: VecDeque::new(), komut_no: 0, canli: None, bekleyen: 0, etkin_kirli: false, son_kayit: Instant::now(), yol }), Condvar::new())
    })
}
fn kilit() -> std::sync::MutexGuard<'static, Durum> { ortak().0.lock().unwrap_or_else(|e| e.into_inner()) }

/// `{temelSurum, proje}` → Ok(yeni sürüm) | Err((kod, gövde)).
pub fn proje_yaz(govde: &str) -> Result<u64, (u16, String)> {
    if govde.len() > MAX_PROJE + 1024 { return Err((400, "proje çok büyük (en çok 256 KB)".into())); }
    let j = json_coz(govde).ok_or((400, "JSON çözülemedi".to_string()))?;
    let temel = j.al("temelSurum").and_then(|x| x.sayi()).map(|x| x as u64);
    let mut p = dogrula(j.al("proje").unwrap_or(&J::Bos)).map_err(|e| (400, e))?;
    let mut d = kilit();
    if temel != Some(d.surum) { return Err((409, durum_json(&d))); }
    // Etkin klip sırası çıkışın kendi bilgisidir (canlı bildirimle gelir); kontrol sayfası onu ezmez. Yeni katmanda gelen değer kullanılır.
    for c in p.katmanlar.iter_mut() {
        if let Some(e) = d.proje.katmanlar.iter().find(|x| x.id == c.id) { c.etkin = if c.klipler.is_empty() { 0 } else { e.etkin.min(c.klipler.len() - 1) }; }
    }
    kaydet(&d.yol, &p).map_err(|e| (500, format!("kaydedilemedi: {e}")))?;
    d.proje = p;
    d.surum += 1;
    d.etkin_kirli = false;
    d.son_kayit = Instant::now();
    let s = d.surum;
    drop(d);
    ortak().1.notify_all();
    Ok(s)
}
fn durum_json(d: &Durum) -> String { format!("{{\"surum\":{},\"komutNo\":{},\"proje\":{}}}", d.surum, d.komut_no, proje_json(&d.proje)) }
pub fn proje_oku() -> String { durum_json(&kilit()) }

/// `{tur, katman?, deger?}` → komut no.
pub fn komut_ekle(govde: &str) -> Result<u64, String> {
    let j = json_coz(govde).ok_or("JSON çözülemedi")?;
    let tur = j.al("tur").and_then(|x| x.yazi()).filter(|t| KOMUTLAR.contains(t)).ok_or("komut türü geçersiz")?.to_string();
    let katman = j.al("katman").and_then(|x| x.sayi()).map(|x| kirp(x, 0.0, 2_147_483_647.0) as u64);
    let deger = j.al("deger").and_then(|x| x.sayi()).map(|x| kirp(x, -1e6, 1e6));
    let mut d = kilit();
    d.komut_no += 1;
    let no = d.komut_no;
    let mut s = format!("{{\"no\":{no},\"tur\":\"{tur}\"");
    if let Some(c) = katman { s.push_str(&format!(",\"katman\":{c}")); }
    if let Some(v) = deger { s.push_str(&format!(",\"deger\":{}", sayi_yaz(v))); }
    s.push('}');
    d.komutlar.push_back((no, s));
    while d.komutlar.len() > MAX_KOMUT { d.komutlar.pop_front(); }
    drop(d);
    ortak().1.notify_all();
    Ok(no)
}

/// Uzun yoklama: yeni proje ya da komut varsa hemen, yoksa en geç `sure` sonunda döner. `komut` yoksa eski komutlar gönderilmez.
pub fn olay_bekle(surum: Option<u64>, komut: Option<u64>, sure: Duration) -> Result<String, u16> {
    let (m, cv) = ortak();
    let mut d = m.lock().unwrap_or_else(|e| e.into_inner());
    if d.bekleyen >= MAX_BEKLEYEN { return Err(503); }
    d.bekleyen += 1;
    let bitis = Instant::now() + sure;
    loop {
        let proje_yeni = surum != Some(d.surum);
        let komut_yeni = komut.map(|c| c < d.komut_no || c > d.komut_no).unwrap_or(true);
        let simdi = Instant::now();
        if proje_yeni || komut_yeni || simdi >= bitis { break; }
        d = cv.wait_timeout(d, bitis - simdi).map(|x| x.0).unwrap_or_else(|e| e.into_inner().0);
    }
    d.bekleyen -= 1;
    let proje = if surum != Some(d.surum) { proje_json(&d.proje) } else { "null".into() };
    let komutlar: Vec<&str> = match komut { Some(c) if c <= d.komut_no => d.komutlar.iter().filter(|x| x.0 > c).map(|x| x.1.as_str()).collect(), _ => vec![] };
    Ok(format!("{{\"surum\":{},\"komutNo\":{},\"proje\":{},\"komutlar\":[{}]}}", d.surum, d.komut_no, proje, komutlar.join(",")))
}

/// Çıkışın anlık durumu (bellekte). Katman başına `etkin` proje içine işlenir (sürüm artmaz), en geç 5 sn'de bir diske yazılır.
pub fn canli_yaz(govde: &str) -> Result<(), String> {
    if govde.len() > MAX_CANLI { return Err("çok büyük".into()); }
    let j = json_coz(govde).ok_or("JSON çözülemedi")?;
    let mut d = kilit();
    if let Some(J::D(v)) = j.al("katmanlar") {
        for c in v {
            let (Some(id), Some(e)) = (c.al("id").and_then(|x| x.sayi()), c.al("etkin").and_then(|x| x.sayi())) else { continue };
            if let Some(kat) = d.proje.katmanlar.iter_mut().find(|x| x.id as f64 == id) {
                let e = if kat.klipler.is_empty() { 0 } else { (kirp(e, 0.0, 1e6) as usize).min(kat.klipler.len() - 1) };
                if kat.etkin != e { kat.etkin = e; d.etkin_kirli = true; }
            }
        }
    }
    if let Some(J::D(v)) = j.al("olaylar") { for o in v.iter().take(10) { if let Some(s) = o.yazi() { gunluk(&format!("çıkış: {s}")); } } }
    d.canli = Some((govde.to_string(), Instant::now()));
    if d.etkin_kirli && d.son_kayit.elapsed() > Duration::from_secs(5) {
        let (yol, p) = (d.yol.clone(), d.proje.clone());
        d.etkin_kirli = false;
        d.son_kayit = Instant::now();
        let _ = kaydet(&yol, &p);
    }
    Ok(())
}
pub fn canli_oku() -> String {
    let d = kilit();
    match &d.canli { Some((s, z)) => format!("{{\"yasMs\":{},\"durum\":{s}}}", z.elapsed().as_millis()), None => "{\"yasMs\":null,\"durum\":null}".into() }
}

// ---------------------------------------------------------------- Sahne pişirme (senaryo → mp4 klip)
#[derive(Default)]
struct Sahne { calisiyor: bool, kare: u32, toplam: u32, dosya: String, mesaj: String, hata: bool }
static SAHNE: Mutex<Option<Sahne>> = Mutex::new(None);
static SAHNE_IPTAL: AtomicBool = AtomicBool::new(false);
fn sahne_json() -> String {
    let g = SAHNE.lock().unwrap_or_else(|e| e.into_inner());
    let d = g.as_ref();
    let s = d.map(|x| (x.calisiyor, x.kare, x.toplam, x.dosya.as_str(), x.mesaj.as_str(), x.hata)).unwrap_or((false, 0, 0, "", "", false));
    format!("{{\"calisiyor\":{},\"kare\":{},\"toplam\":{},\"dosya\":\"{}\",\"mesaj\":\"{}\",\"hata\":{}}}", s.0, s.1, s.2, k(s.3), k(s.4), s.5)
}
fn sahne_baslat(metin: String, kok: PathBuf, uretim: &dyn Fn() -> bool) -> Result<(), (u16, String)> {
    if metin.trim().is_empty() { return Err((400, "senaryo boş".into())); }
    if uretim() { return Err((409, "ana sayfada video üretiliyor; bitince yeniden deneyin".into())); }
    if crate::ffmpeg::bul().is_none() { return Err((400, crate::ffmpeg::YOK_MESAJI.into())); }
    crate::uret::senaryo_oku(&metin).map_err(|e| (400, e))?;
    let ad = format!("oynatici-sahne-{}.mp4", zaman_damgasi());
    {
        let mut g = SAHNE.lock().unwrap_or_else(|e| e.into_inner());
        if g.as_ref().map(|x| x.calisiyor).unwrap_or(false) { return Err((409, "zaten bir sahne pişiriliyor".into())); }
        *g = Some(Sahne { calisiyor: true, dosya: ad.clone(), ..Default::default() });
    }
    SAHNE_IPTAL.store(false, Ordering::Relaxed);
    std::thread::spawn(move || {
        let yol = kok.join(&ad).display().to_string();
        let sonuc = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| crate::uret::video(&metin, &yol, "mp4", &|kare, n| {
            if let Some(s) = SAHNE.lock().unwrap_or_else(|e| e.into_inner()).as_mut() { s.kare = kare; s.toplam = n; }
        }, &SAHNE_IPTAL))).unwrap_or_else(|_| Err("sahne üretimi beklenmedik şekilde durdu".into()));
        let mut g = SAHNE.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(s) = g.as_mut() {
            s.calisiyor = false;
            match sonuc { Ok(_) => { s.mesaj = "hazır".into(); gunluk(&format!("sahne klibi üretildi: {ad}")); } Err(e) => { s.hata = true; s.mesaj = e; } }
        }
    });
    Ok(())
}

// ---------------------------------------------------------------- Çıkış penceresi olayları
fn ac_olayi(p: &Proje, monitor: usize, x: i32, y: i32) -> crate::oynatici_pencere::Olay {
    crate::oynatici_pencere::Olay::Ac { monitor, x, y, gen: p.ekran_gen * p.ekran_sayisi, yuk: p.ekran_yuk }
}
/// Program açılışında çıkış açılsın mı (projede `acilistaCikisAc`). Kayıtlı monitör ve kaydırma kullanılır.
pub fn acilis_olayi() -> Option<crate::oynatici_pencere::Olay> {
    let p = kilit().proje.clone();
    if !p.acilista_cikis_ac { return None; }
    gunluk("çıkış penceresi açılışta açılıyor (acilistaCikisAc)");
    Some(ac_olayi(&p, p.cikis_monitor as usize, p.cikis_x, p.cikis_y))
}

// ---------------------------------------------------------------- HTTP
pub fn bu_yol_mu(yolu: &str) -> bool { yolu == "/cikis" || yolu == "/oynatici" || yolu.starts_with("/oynatici/") || yolu.starts_with("/oynatici.") }

fn yanit<W: Write>(w: &mut W, kod: u16, tur: &str, govde: &[u8], ek: &str) {
    let ad = match kod { 200 => "OK", 400 => "Bad Request", 404 => "Not Found", 405 => "Method Not Allowed", 409 => "Conflict", 501 => "Not Implemented", 503 => "Service Unavailable", _ => "Error" };
    let _ = write!(w, "HTTP/1.1 {kod} {ad}\r\nContent-Type: {tur}\r\nContent-Length: {}\r\nX-Content-Type-Options: nosniff\r\nCache-Control: no-store\r\n{ek}Connection: close\r\n\r\n", govde.len());
    let _ = w.write_all(govde);
}
const JS: &str = "application/json; charset=utf-8";
const MT: &str = "text/plain; charset=utf-8";
fn sorgu(yol: &str, a: &str) -> Option<String> {
    yol.split_once('?')?.1.split('&').find_map(|p| p.split_once('=').filter(|(x, _)| *x == a).map(|(_, v)| medya::yuzde_coz(v)))
}

/// Akışa kendisi yazan asıl giriş (ISTEK-oynatici.md 1). Yetki denetimi çağırandadır.
pub fn isle_akis(s: &mut std::net::TcpStream, yontem: &str, yol: &str, bas: &[(String, String)], govde: &[u8], uretim_calisiyor: &dyn Fn() -> bool) {
    let _ = s.set_write_timeout(Some(Duration::from_secs(30)));
    yanitla(s, yontem, yol, bas, govde, uretim_calisiyor, &crate::arayuz::cikti_klasoru(), Duration::from_secs(20));
}

/// Test edilebilir çekirdek: herhangi bir `Write` hedefine HTTP yanıtı yazar.
pub fn yanitla<W: Write>(w: &mut W, yontem: &str, yol: &str, bas: &[(String, String)], govde: &[u8], uretim: &dyn Fn() -> bool, kok: &Path, bekleme: Duration) {
    let yolu = yol.split('?').next().unwrap_or("");
    let yazi = String::from_utf8_lossy(govde);
    let csp = format!("Content-Security-Policy: {CSP}\r\nReferrer-Policy: no-referrer\r\n");
    let range = bas.iter().find(|(a, _)| a == "range").map(|(_, v)| v.as_str());
    match (yontem, yolu) {
        ("GET", "/oynatici") => yanit(w, 200, "text/html; charset=utf-8", HTML_KONTROL.as_bytes(), &csp),
        ("GET", "/cikis") => yanit(w, 200, "text/html; charset=utf-8", HTML_CIKIS.as_bytes(), &csp),
        ("GET", "/oynatici.js") | ("GET", "/oynatici.css") => { let ad = &yolu[1..]; let (_, t, m) = DOSYALAR.iter().find(|x| x.0 == ad).unwrap(); yanit(w, 200, t, m.as_bytes(), "") }
        ("GET", y) if y.starts_with("/oynatici/dosya/") => match DOSYALAR.iter().find(|x| x.0 == &y[16..]) {
            Some((_, t, m)) => yanit(w, 200, t, m.as_bytes(), ""),
            None => yanit(w, 404, MT, b"yok", ""),
        },
        ("GET", "/oynatici/proje") => yanit(w, 200, JS, proje_oku().as_bytes(), ""),
        ("POST", "/oynatici/proje") => match proje_yaz(&yazi) {
            Ok(s) => yanit(w, 200, JS, format!("{{\"surum\":{s}}}").as_bytes(), ""),
            Err((409, g)) => yanit(w, 409, JS, g.as_bytes(), ""),
            Err((kod, e)) => yanit(w, kod, MT, e.as_bytes(), ""),
        },
        ("POST", "/oynatici/komut") => match komut_ekle(&yazi) {
            Ok(n) => yanit(w, 200, JS, format!("{{\"no\":{n}}}").as_bytes(), ""),
            Err(e) => yanit(w, 400, MT, e.as_bytes(), ""),
        },
        ("GET", "/oynatici/olay") => {
            let surum = sorgu(yol, "surum").and_then(|v| v.parse().ok());
            let komut = sorgu(yol, "komut").and_then(|v| v.parse().ok());
            match olay_bekle(surum, komut, bekleme) { Ok(g) => yanit(w, 200, JS, g.as_bytes(), ""), Err(kod) => yanit(w, kod, MT, b"cok fazla bekleyen istek", "") }
        }
        ("POST", "/oynatici/canli") => match canli_yaz(&yazi) { Ok(()) => yanit(w, 200, MT, b"ok", ""), Err(e) => yanit(w, 400, MT, e.as_bytes(), "") },
        ("GET", "/oynatici/canli") => yanit(w, 200, JS, canli_oku().as_bytes(), ""),
        ("GET", "/oynatici/medyalar") => yanit(w, 200, JS, medya::liste(kok).as_bytes(), ""),
        ("GET", y) | ("HEAD", y) if y.starts_with("/oynatici/medya/") => medya::yaz(w, kok, &y[16..], yontem == "HEAD", range, false),
        ("GET", y) if y.starts_with("/oynatici/kucuk/") => medya::yaz(w, kok, &y[16..], false, range, true),
        ("POST", "/oynatici/sahne") => match sahne_baslat(yazi.into_owned(), kok.to_path_buf(), uretim) {
            Ok(()) => yanit(w, 200, JS, sahne_json().as_bytes(), ""),
            Err((kod, e)) => yanit(w, kod, MT, e.as_bytes(), ""),
        },
        ("GET", "/oynatici/sahne") => yanit(w, 200, JS, sahne_json().as_bytes(), ""),
        ("POST", "/oynatici/sahne-iptal") => { SAHNE_IPTAL.store(true, Ordering::Relaxed); yanit(w, 200, MT, b"ok", "") }
        ("GET", "/oynatici/monitorler") => match crate::oynatici_pencere::monitorler() {
            Some(m) => yanit(w, 200, JS, m.as_bytes(), ""),
            None => yanit(w, 503, MT, "pencere yok".as_bytes(), ""),
        },
        ("POST", "/oynatici/cikis") => {
            let j = json_coz(&yazi).unwrap_or(J::Bos);
            let ac = j.al("ac").and_then(|x| x.mantik()).unwrap_or(false);
            let p = kilit().proje.clone();
            let mon = j.al("monitor").and_then(|x| x.sayi()).map(|x| kirp(x, 0.0, 16.0) as usize).unwrap_or(p.cikis_monitor as usize);
            let x = j.al("x").and_then(|x| x.sayi()).map(|v| kirp(v, -20000.0, 20000.0) as i32).unwrap_or(p.cikis_x);
            let y = j.al("y").and_then(|x| x.sayi()).map(|v| kirp(v, -20000.0, 20000.0) as i32).unwrap_or(p.cikis_y);
            let olay = if ac { ac_olayi(&p, mon, x, y) } else { crate::oynatici_pencere::Olay::Kapat };
            if crate::oynatici_pencere::gonder(olay) { gunluk(if ac { "çıkış penceresi açıldı" } else { "çıkış penceresi kapatıldı" }); yanit(w, 200, MT, b"ok", "") }
            else { yanit(w, 503, MT, "pencere yok".as_bytes(), "") }
        }
        (_, y) if bu_yol_mu(y) => yanit(w, 404, MT, b"yok", ""),
        _ => yanit(w, 404, MT, b"yok", ""),
    }
}

/// Yedek giriş (OYN.md imzası): medya ve akış dışındaki uçlar. `/oynatici/medya/` 501 döner; asıl giriş `isle_akis`.
pub fn isle(yontem: &str, yol: &str, govde: &str, klasor: &Path) -> (u16, &'static str, String) {
    let yolu = yol.split('?').next().unwrap_or("");
    let yolu_tam = if yolu.starts_with("/oynatici") || yolu == "/cikis" { yol.to_string() } else { format!("/oynatici/{}", yol.trim_start_matches('/')) };
    if yolu_tam.starts_with("/oynatici/medya/") || yolu_tam.starts_with("/oynatici/kucuk/") { return (501, MT, "medya için isle_akis kullanılmalı (ISTEK-oynatici.md)".into()); }
    let mut v: Vec<u8> = vec![];
    yanitla(&mut v, yontem, &yolu_tam, &[], govde.as_bytes(), &|| false, klasor, Duration::from_secs(20));
    let metin = String::from_utf8_lossy(&v).into_owned();
    let (bas, g) = metin.split_once("\r\n\r\n").unwrap_or(("", ""));
    let kod = bas.split_whitespace().nth(1).and_then(|x| x.parse().ok()).unwrap_or(500);
    let tur = if bas.contains("text/html") { "text/html; charset=utf-8" } else if bas.contains("application/json") { JS } else if bas.contains("text/javascript") { "text/javascript; charset=utf-8" } else if bas.contains("text/css") { "text/css; charset=utf-8" } else { MT };
    (kod, tur, g.to_string())
}

#[cfg(test)]
mod t {
    use super::*;
    #[test]
    fn json_cozucu() {
        assert_eq!(json_coz("{\"a\":[1,true,null,\"x\\u00e7\\ud83d\\ude00\"]}").unwrap().al("a"), Some(&J::D(vec![J::S(1.0), J::M(true), J::Bos, J::Y("xç😀".into())])));
        assert!(json_coz("{\"a\":1} x").is_none());
        assert!(json_coz(&"[".repeat(40)).is_none());
        assert!(json_coz("{\"a\":1e999}").is_none());
    }
    #[test]
    fn zaman_bicimi() { let z = zaman_damgasi(); assert_eq!(z.len(), 15); assert_eq!(&z[8..9], "-"); assert!(z.starts_with("20")); }
    #[test]
    fn proje_gidis_donus() {
        let p = Proje::default();
        let j = json_coz(&proje_json(&p)).unwrap();
        assert_eq!(dogrula(&j).unwrap(), p);
    }
}
