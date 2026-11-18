//! "Tarif et" çekirdeği: sahne listesi şeması, sıkı doğrulama, katalogdan istem üretici ve çevrimdışı kural ayrıştırıcı.
//! Sahip: Yusuf; yazan [Claude#ebd691, 2026-11-16]; yenileme [Claude#AI Opus 5.5, yenileme-1.0, 2026-11-18]: yazım hatası toleransı, birim (TL) düzeltmesi, niyet şablonları (tarif/niyet.rs).
//! Kurallar docs/TARIF-ET-LLM.md, docs/TARIF-KURALLARI.md, docs/TARIF-API.md'de. Yalnız yazan yapay zekâ ya da Yusuf'un izniyle değişir; silme yok.
//! Güvenlik ilkesi: bu modül yalnız METİN işler. Hiçbir şey çalıştırılmaz, dosya yolu açılmaz, ağ kullanılmaz.
//! Dış kaynaktan (LLM yanıtı, pano) gelen her şey veridir; doğrulamadan geçmeden senaryoya dönüşmez.
use std::sync::OnceLock;

pub const MAX_SAHNE: usize = 12;
pub const MAX_SATIR: usize = 6;
pub const MAX_KARAKTER: usize = 40;
pub const MIN_SURE: f32 = 1.5;
pub const MAX_SURE: f32 = 15.0;
pub const MAX_TOPLAM_SN: f32 = 120.0;
pub const MAX_YANIT: usize = 64_000;
pub const MAX_TARIF: usize = 4_000;
const MAX_EFEKT: usize = 4;

// ───────────────────────── Küçük JSON ayrıştırıcı (bağımlılıksız) ─────────────────────────
#[derive(Clone, Debug, PartialEq)]
pub enum Json { Bos, Mantik(bool), Sayi(f64), Yazi(String), Dizi(Vec<Json>), Nesne(Vec<(String, Json)>) }
impl Json {
    pub fn al(&self, k: &str) -> Option<&Json> { if let Json::Nesne(v) = self { v.iter().find(|x| x.0 == k).map(|x| &x.1) } else { None } }
    pub fn yazi(&self) -> Option<&str> { if let Json::Yazi(s) = self { Some(s) } else { None } }
    pub fn dizi(&self) -> Option<&Vec<Json>> { if let Json::Dizi(v) = self { Some(v) } else { None } }
}
struct Ayr<'a> { c: &'a [char], i: usize, d: usize }
impl<'a> Ayr<'a> {
    fn bosluk(&mut self) { while self.i < self.c.len() && self.c[self.i].is_whitespace() { self.i += 1; } }
    fn deger(&mut self) -> Result<Json, String> {
        self.bosluk();
        if self.d > 24 { return Err("iç içe yapı çok derin".into()); }
        match self.c.get(self.i).copied() {
            None => Err("JSON yarım kalmış".into()),
            Some('{') => {
                self.i += 1; self.d += 1;
                let mut v = vec![];
                loop {
                    self.bosluk();
                    match self.c.get(self.i).copied() {
                        Some('}') => { self.i += 1; break; }
                        Some(',') => { self.i += 1; continue; }
                        Some('"') => {
                            let k = self.yazi()?; self.bosluk();
                            if self.c.get(self.i) != Some(&':') { return Err(format!("':' bekleniyordu (konum {})", self.i)); }
                            self.i += 1;
                            let d = self.deger()?; v.push((k, d));
                        }
                        _ => return Err(format!("nesne anahtarı bekleniyordu (konum {})", self.i)),
                    }
                }
                self.d -= 1; Ok(Json::Nesne(v))
            }
            Some('[') => {
                self.i += 1; self.d += 1;
                let mut v = vec![];
                loop {
                    self.bosluk();
                    match self.c.get(self.i).copied() {
                        Some(']') => { self.i += 1; break; }
                        Some(',') => { self.i += 1; continue; }
                        None => return Err("dizi kapanmamış".into()),
                        _ => v.push(self.deger()?),
                    }
                }
                self.d -= 1; Ok(Json::Dizi(v))
            }
            Some('"') => Ok(Json::Yazi(self.yazi()?)),
            Some(c) if c.is_ascii_digit() || c == '-' => {
                let b = self.i; self.i += 1;
                while self.i < self.c.len() && (self.c[self.i].is_ascii_digit() || matches!(self.c[self.i], '.' | 'e' | 'E' | '+' | '-')) { self.i += 1; }
                let s: String = self.c[b..self.i].iter().collect();
                s.parse::<f64>().map(Json::Sayi).map_err(|_| format!("sayı okunamadı: {s}"))
            }
            Some(c) if c.is_ascii_alphabetic() => {
                let b = self.i;
                while self.i < self.c.len() && self.c[self.i].is_ascii_alphabetic() { self.i += 1; }
                match self.c[b..self.i].iter().collect::<String>().as_str() { "true" => Ok(Json::Mantik(true)), "false" => Ok(Json::Mantik(false)), "null" => Ok(Json::Bos), k => Err(format!("bilinmeyen sözcük: {k}")) }
            }
            Some(c) => Err(format!("beklenmeyen karakter '{c}' (konum {})", self.i)),
        }
    }
    fn yazi(&mut self) -> Result<String, String> {
        self.i += 1; // açılış tırnağı
        let mut o = String::new();
        loop {
            let Some(&c) = self.c.get(self.i) else { return Err("metin tırnağı kapanmamış".into()) };
            self.i += 1;
            match c {
                '"' => return Ok(o),
                '\\' => {
                    let Some(&e) = self.c.get(self.i) else { return Err("kaçış dizisi yarım".into()) };
                    self.i += 1;
                    match e {
                        'n' => o.push('\n'), 't' => o.push(' '), 'r' => {}, 'b' | 'f' => {},
                        '"' => o.push('"'), '\\' => o.push('\\'), '/' => o.push('/'),
                        'u' => {
                            let h: String = self.c.iter().skip(self.i).take(4).collect();
                            if h.chars().count() < 4 { return Err("\\u kaçışı yarım".into()); }
                            let n = u32::from_str_radix(&h, 16).map_err(|_| "\\u kaçışı hatalı".to_string())?;
                            self.i += 4;
                            if (0xD800..0xDC00).contains(&n) && self.c.get(self.i) == Some(&'\\') && self.c.get(self.i + 1) == Some(&'u') {
                                let h2: String = self.c.iter().skip(self.i + 2).take(4).collect();
                                if let Ok(n2) = u32::from_str_radix(&h2, 16) { if (0xDC00..0xE000).contains(&n2) { self.i += 6; if let Some(ch) = char::from_u32(0x10000 + ((n - 0xD800) << 10) + (n2 - 0xDC00)) { o.push(ch); } continue; } }
                            }
                            o.push(char::from_u32(n).unwrap_or(' '));
                        }
                        d => o.push(d),
                    }
                }
                c => o.push(c),
            }
        }
    }
}
/// JSON'u çözer. Sondaki virgüle ve LLM'lerin sık yaptığı küçük hatalara toleranslıdır; boyut ve derinlik sınırlıdır.
pub fn json_coz(s: &str) -> Result<Json, String> {
    if s.len() > MAX_YANIT { return Err("metin çok uzun".into()); }
    let c: Vec<char> = s.chars().collect();
    let mut a = Ayr { c: &c, i: 0, d: 0 };
    let d = a.deger()?; a.bosluk();
    if a.i < c.len() { return Err(format!("JSON'dan sonra fazladan metin var (konum {})", a.i)); }
    Ok(d)
}
/// Metnin başındaki dengeli `[ … ]` parçasını bulur (tırnak içindeki köşeli ayraçlar sayılmaz).
pub fn dizi_bul(ham: &str) -> Option<String> {
    let c: Vec<char> = ham.chars().collect();
    let b = c.iter().position(|&x| x == '[')?;
    let (mut derin, mut ic, mut kac) = (0i32, false, false);
    for (k, &ch) in c.iter().enumerate().skip(b) {
        if ic { if kac { kac = false; } else if ch == '\\' { kac = true; } else if ch == '"' { ic = false; } continue; }
        match ch { '"' => ic = true, '[' => derin += 1, ']' => { derin -= 1; if derin == 0 { return Some(c[b..=k].iter().collect()); } } _ => {} }
    }
    None
}
/// Metindeki ilk dengeli `{ … }` parçasını bulur (kod çiti, öncesi ve sonrası açıklama yazısı olsa da).
pub fn json_bul(ham: &str) -> Option<String> {
    let c: Vec<char> = ham.chars().collect();
    let b = c.iter().position(|&x| x == '{')?;
    let (mut derin, mut ic, mut kac) = (0i32, false, false);
    for (k, &ch) in c.iter().enumerate().skip(b) {
        if ic { if kac { kac = false; } else if ch == '\\' { kac = true; } else if ch == '"' { ic = false; } continue; }
        match ch { '"' => ic = true, '{' => derin += 1, '}' => { derin -= 1; if derin == 0 { return Some(c[b..=k].iter().collect()); } } _ => {} }
    }
    None
}

// ───────────────────────── Metin yardımcıları ─────────────────────────
/// ı/İ, ş, ğ, ü, ö, ç → ASCII, küçük harf. "ZIPLA", "zıpla", "ziplA" aynı olur.
pub fn sadelestir(s: &str) -> String {
    s.chars().flat_map(|c| match c {
        'İ' | 'I' | 'ı' | 'i' => vec!['i'], 'Ş' | 'ş' => vec!['s'], 'Ğ' | 'ğ' => vec!['g'], 'Ü' | 'ü' => vec!['u'], 'Ö' | 'ö' => vec!['o'], 'Ç' | 'ç' => vec!['c'],
        c => c.to_lowercase().collect(),
    }).collect()
}
pub fn buyuk(s: &str) -> String { s.chars().flat_map(|c| match c { 'i' => vec!['İ'], 'ı' => vec!['I'], c => c.to_uppercase().collect::<Vec<_>>() }).collect() }
fn harf_buyuk_mu(c: char) -> bool { c.is_uppercase() || c == 'İ' || c == 'I' }

const TEHLIKELI: [&str; 14] = ["powershell", "cmd.exe", "rm -rf", "del /", "curl ", "wget ", "invoke-", "eval(", "<script", "javascript:", "format c", "chmod ", "sudo ", "reg add"];
/// Sahne metnini temizler: HTML, bağlantı, dosya yolu, komut benzeri ve senaryoyu bozan karakterler çıkar. (temiz, değişti_mi)
pub fn metin_temizle(s: &str) -> (String, bool) {
    let ilk = s.split_whitespace().collect::<Vec<_>>().join(" ");
    if TEHLIKELI.iter().any(|t| sadelestir(&ilk).contains(t)) { return (String::new(), true); }
    // HTML etiketleri
    let mut a = String::new(); let mut etiket = false;
    for c in ilk.chars() { match c { '<' => etiket = true, '>' => { if etiket { etiket = false; a.push(' '); } } c if !etiket => a.push(c), _ => {} } }
    let mut parcalar: Vec<String> = vec![];
    for t in a.split_whitespace() {
        let k = sadelestir(t);
        let yol = t.contains('\\') || (t.starts_with('/') && t.len() > 1 && t.chars().skip(1).any(|c| c.is_alphabetic())) || t.contains("../") || (t.len() >= 3 && t.as_bytes()[1] == b':' && t.as_bytes()[0].is_ascii_alphabetic() && matches!(t.as_bytes()[2], b'\\' | b'/'));
        if k.starts_with("http") && t.contains("://") || t.contains("://") || k.starts_with("www.") || k.starts_with("file:") || k.starts_with("ftp:") || yol || t.contains("$(") || t.contains('`') { continue; }
        parcalar.push(t.to_string());
    }
    let temiz: String = parcalar.join(" ").chars().filter(|c| !matches!(c, '`' | '"' | '#' | '{' | '}' | '<' | '>' | '\\' | ';' | '&' | '|' | '^' | '~' | '$' | '=' | '*') && !c.is_control()).collect();
    let temiz = temiz.split_whitespace().collect::<Vec<_>>().join(" ");
    let temiz = temiz.trim_start_matches('@').trim().to_string();
    let degisti = temiz != ilk;
    (temiz, degisti)
}
fn satir_kisalt(s: &str, en_cok: usize) -> (String, bool) {
    if s.chars().count() <= en_cok { return (s.to_string(), false); }
    let mut o = String::new();
    for k in s.split(' ') {
        let yeni = if o.is_empty() { k.to_string() } else { format!("{o} {k}") };
        if yeni.chars().count() > en_cok { break; }
        o = yeni;
    }
    if o.is_empty() { o = s.chars().take(en_cok).collect(); }
    (o, true)
}
fn mesafe(a: &str, b: &str) -> usize {
    let (a, b): (Vec<char>, Vec<char>) = (a.chars().collect(), b.chars().collect());
    let mut onceki: Vec<usize> = (0..=b.len()).collect();
    for i in 1..=a.len() {
        let mut yeni = vec![i];
        for j in 1..=b.len() { yeni.push((onceki[j] + 1).min(yeni[j - 1] + 1).min(onceki[j - 1] + usize::from(a[i - 1] != b[j - 1]))); }
        onceki = yeni;
    }
    onceki[b.len()]
}

// ───────────────────────── Katalog ─────────────────────────
#[derive(Clone, Debug, Default)]
pub struct Katalog { pub giris: Vec<String>, pub cikis: Vec<String>, pub efekt: Vec<String>, pub palet: Vec<String>, pub font: Vec<String>, pub gecis: Vec<String> }
impl Katalog {
    /// Programın GERÇEK listelerinden (animasyonlar, efektler, paletler, fontlar, geçişler) okunur; katalog büyüdükçe kendiliğinden güncellenir.
    pub fn gercek() -> Katalog {
        crate::animasyon::yukle();
        Katalog {
            giris: crate::senaryo::giris_adlari(), cikis: crate::senaryo::cikis_adlari(),
            efekt: crate::senaryo::efekt_listesi().into_iter().map(|x| x.0).collect(),
            palet: crate::stil::palet_listesi().into_iter().map(|x| x.0).collect(),
            font: crate::stil::FONTLAR.iter().map(|f| f.0.to_string()).collect(),
            gecis: crate::senaryo::GECISLER.iter().map(|g| g.0.to_string()).collect(),
        }
    }
    pub fn palet_gecerli(&self, a: &str) -> bool { hex_palet(a) || self.palet.iter().any(|x| x == a) }
}
fn hex_palet(a: &str) -> bool { a.strip_prefix("hex-").map(|h| h.len() == 6 && h.chars().all(|c| c.is_ascii_hexdigit())).unwrap_or(false) }

/// Verilen adı katalogdaki en yakın ada eşler. Tam eşleşme (büyük/küçük harf ve Türkçe karakter farkı yok sayılır), sonra ön ek, sonra yazım uzaklığı.
pub fn en_yakin(ad: &str, liste: &[String]) -> Option<String> {
    let n = |s: &str| sadelestir(s).replace(['_', ' '], "-");
    let a = n(ad.trim());
    if a.is_empty() { return None; }
    if let Some(x) = liste.iter().find(|x| n(x) == a) { return Some(x.clone()); }
    if a.chars().count() >= 4 {
        if let Some(x) = liste.iter().filter(|x| { let y = n(x); y.chars().count() >= 4 && (a.starts_with(&y) || y.starts_with(&a)) }).min_by_key(|x| x.len()) { return Some(x.clone()); }
    }
    let esik = if a.chars().count() >= 5 { 2 } else { 1 };
    liste.iter().map(|x| (mesafe(&a, &n(x)), x)).filter(|(d, _)| *d <= esik && *d > 0).min_by_key(|(d, x)| (*d, x.len())).map(|(_, x)| x.clone())
}

// ───────────────────────── Şema ─────────────────────────
#[derive(Clone, Debug, PartialEq)]
pub struct TSahne { pub metin: Vec<String>, pub giris: String, pub cikis: String, pub efekt: Vec<String>, pub palet: String, pub font: String, pub sure: f32 }
#[derive(Clone, Debug, PartialEq)]
pub struct Tarif { pub gorunum: String, pub sahneler: Vec<TSahne>, pub acilis: String, pub kapanis: String }
pub struct Dogrulama { pub tarif: Tarif, pub notlar: Vec<String> }

fn eslestir(alan: &str, ad: &str, liste: &[String], varsayilan: &str, bos_olur: bool, ek: &str, notlar: &mut Vec<String>) -> String {
    let ad = ad.trim();
    let ad: String = ad.chars().take(64).collect();
    let ad = ad.as_str();
    if ad.is_empty() { return varsayilan.to_string(); }
    if let Some(x) = liste.iter().find(|x| x.as_str() == ad) { return x.clone(); }
    match en_yakin(ad, liste) {
        Some(x) => { notlar.push(format!("{ek}{alan} \"{}\" katalogda yok; en yakın ad \"{x}\" kullanıldı.", ad.chars().take(40).collect::<String>())); x }
        None => { if !(bos_olur && ad.is_empty()) { notlar.push(format!("{ek}{alan} \"{}\" katalogda yok; varsayılan \"{varsayilan}\" kullanıldı.", ad.chars().take(40).collect::<String>())); } varsayilan.to_string() }
    }
}
fn palet_coz(ad: &str, kat: &Katalog, ek: &str, notlar: &mut Vec<String>) -> String {
    let ad = ad.trim();
    if ad.is_empty() { return "altin".into(); }
    if let Some(h) = ad.strip_prefix('#') { let k = format!("hex-{}", h.to_lowercase()); if hex_palet(&k) { return k; } }
    if hex_palet(&ad.to_lowercase()) { return ad.to_lowercase(); }
    eslestir("renk/palet", ad, &kat.palet, "altin", false, ek, notlar)
}
fn metin_satirlari(j: Option<&Json>) -> Vec<String> {
    match j {
        Some(Json::Dizi(v)) => v.iter().filter_map(|x| match x { Json::Yazi(s) => Some(s.clone()), Json::Sayi(n) => Some(n.to_string()), _ => None }).flat_map(|s| s.split(['\n', '|']).map(String::from).collect::<Vec<_>>()).collect(),
        Some(Json::Yazi(s)) => s.split(['\n', '|']).map(String::from).collect(),
        _ => vec![],
    }
}
fn sayi_oku(j: Option<&Json>) -> Option<f32> {
    match j { Some(Json::Sayi(n)) => Some(*n as f32), Some(Json::Yazi(s)) => s.trim().replace(',', ".").trim_end_matches(|c: char| c.is_alphabetic() || c == ' ').parse().ok(), _ => None }
}

/// LLM yanıtını (ya da panodaki metni) doğrular. Başarıda temizlenmiş sahne listesi ve "şu değişti" notları; hatada okunur Türkçe hata listesi döner.
pub fn dogrula(ham: &str, kat: &Katalog) -> Result<Dogrulama, Vec<String>> {
    if ham.len() > MAX_YANIT { return Err(vec![format!("Yanıt çok uzun (en çok {} KB).", MAX_YANIT / 1000)]); }
    let adaylar = [ham.to_string(), ham.replace(['“', '”', '„', '«', '»'], "\"").replace(['‘', '’'], "'")];
    let mut son_hata = String::new();
    let mut kok = None;
    for a in &adaylar {
        let Some(js) = json_bul(a) else { continue };
        match json_coz(&js) { Ok(k) => { kok = Some(k); break; } Err(e) => son_hata = e }
    }
    // Yanıt doğrudan sahne dizisi ("[{...},{...}]") ise sarılır.
    if kok.is_none() || kok.as_ref().map(|k| k.al("sahneler").is_none()).unwrap_or(false) {
        for a in &adaylar {
            let (ib, ia) = (a.find('{'), a.find('['));
            if let (Some(ib), Some(ia)) = (ib, ia) { if ia < ib { if let Some(d) = dizi_bul(&a[ia..]) { if let Ok(j @ Json::Dizi(_)) = json_coz(&d) { kok = Some(Json::Nesne(vec![("sahneler".into(), j)])); break; } } } }
        }
    }
    // Tek sahne nesnesi ({"metin":[...]}) ise sahne listesine sarılır.
    if let Some(k) = &kok { if k.al("sahneler").is_none() && k.al("metin").is_some() { kok = Some(Json::Nesne(vec![("sahneler".into(), Json::Dizi(vec![k.clone()]))])); } }
    let Some(kok) = kok else {
        return Err(vec![if son_hata.is_empty() { "Yanıtta JSON bulunamadı. Yanıt { ile başlayan sahne listesi olmalı.".into() } else { format!("JSON okunamadı: {son_hata}") }]);
    };
    if !matches!(kok, Json::Nesne(_)) { return Err(vec!["Yanıt bir JSON nesnesi olmalı.".into()]); }
    let (mut hatalar, mut notlar) = (vec![], vec![]);
    if let Some(Json::Sayi(n)) = kok.al("surum") { if *n as i64 != 1 { notlar.push(format!("Şema sürümü {n}; sürüm 1 gibi okundu.")); } }
    let gorunum = match kok.al("gorunum").and_then(|x| x.yazi()).map(sadelestir).as_deref() { Some("bina") => "bina", Some("duz") | None | Some("") => "duz", Some(d) => { notlar.push(format!("Görünüm \"{d}\" bilinmiyor; düz kullanıldı.")); "duz" } }.to_string();
    let Some(Json::Dizi(ham_sahneler)) = kok.al("sahneler") else { return Err(vec!["\"sahneler\" listesi yok.".into()]) };
    if ham_sahneler.is_empty() { return Err(vec!["\"sahneler\" listesi boş.".into()]); }
    if ham_sahneler.len() > MAX_SAHNE { return Err(vec![format!("Çok fazla sahne: {} (en çok {MAX_SAHNE}). Daha az sahneyle yeniden isteyin.", ham_sahneler.len())]); }
    let mut sahneler = vec![];
    for (i, hs) in ham_sahneler.iter().enumerate() {
        let ek = format!("Sahne {}: ", i + 1);
        if !matches!(hs, Json::Nesne(_)) { hatalar.push(format!("{ek}nesne olmalı.")); continue; }
        let mut satirlar = vec![];
        for s in metin_satirlari(hs.al("metin")) {
            let (t, d) = metin_temizle(&s);
            if d { let n = format!("{ek}metin temizlendi (HTML, bağlantı, yol veya komut benzeri parça çıkarıldı)."); if !notlar.contains(&n) { notlar.push(n); } }
            let t = buyuk(&t);
            if t.is_empty() { continue; }
            let (k, kirpildi) = satir_kisalt(&t, MAX_KARAKTER);
            if kirpildi { notlar.push(format!("{ek}\"{}…\" satırı {MAX_KARAKTER} karaktere kısaltıldı.", k.chars().take(20).collect::<String>())); }
            satirlar.push(k);
        }
        if satirlar.is_empty() { hatalar.push(format!("{ek}metin boş.")); continue; }
        if satirlar.len() > MAX_SATIR { notlar.push(format!("{ek}{} satır vardı, ilk {MAX_SATIR} satır alındı.", satirlar.len())); satirlar.truncate(MAX_SATIR); }
        let giris = eslestir("giriş", hs.al("giris").and_then(|x| x.yazi()).unwrap_or(""), &kat.giris, "belir", true, &ek, &mut notlar);
        let cikis = eslestir("çıkış", hs.al("cikis").and_then(|x| x.yazi()).unwrap_or(""), &kat.cikis, "yok", true, &ek, &mut notlar);
        let mut efekt: Vec<String> = vec![];
        let ham_efekt: Vec<String> = match hs.al("efekt") { Some(Json::Dizi(v)) => v.iter().filter_map(|x| x.yazi().map(String::from)).collect(), Some(Json::Yazi(s)) => s.split(',').map(String::from).collect(), _ => vec![] };
        for e in ham_efekt { let e = e.trim().to_string(); if e.is_empty() || sadelestir(&e) == "yok" { continue; } let x = eslestir("efekt", &e, &kat.efekt, "", false, &ek, &mut notlar); if !x.is_empty() && !efekt.contains(&x) { efekt.push(x); } }
        if efekt.len() > MAX_EFEKT { notlar.push(format!("{ek}en çok {MAX_EFEKT} efekt kullanılır.")); efekt.truncate(MAX_EFEKT); }
        let palet = palet_coz(hs.al("palet").and_then(|x| x.yazi()).unwrap_or(""), kat, &ek, &mut notlar);
        let font_ad = hs.al("font").and_then(|x| x.yazi()).unwrap_or("").trim().to_string();
        let font = if font_ad.is_empty() || sadelestir(&font_ad) == "yok" { String::new() } else { eslestir("font", &font_ad, &kat.font, "", false, &ek, &mut notlar) };
        let mut sure = sayi_oku(hs.al("sure")).unwrap_or(4.5);
        if !sure.is_finite() { sure = 4.5; }
        if !(MIN_SURE..=MAX_SURE).contains(&sure) { let y = sure.clamp(MIN_SURE, MAX_SURE); notlar.push(format!("{ek}süre {sure:.1} sn; {MIN_SURE}-{MAX_SURE} sn aralığına çekildi ({y:.1} sn).")); sure = y; }
        sahneler.push(TSahne { metin: satirlar, giris, cikis, efekt, palet, font, sure });
    }
    if !hatalar.is_empty() { return Err(hatalar); }
    let toplam: f32 = sahneler.iter().map(|s| s.sure).sum();
    if toplam > MAX_TOPLAM_SN { return Err(vec![format!("Toplam süre {toplam:.0} sn; en çok {MAX_TOPLAM_SN:.0} sn olabilir. Sahneleri kısaltın.")]); }
    let mut gecis = |alan: &str| -> String { let a = kok.al(alan).and_then(|x| x.yazi()).unwrap_or(""); eslestir(&format!("{alan} geçişi"), a, &kat.gecis, "yok", true, "", &mut notlar) };
    let (acilis, kapanis) = (gecis("acilis"), gecis("kapanis"));
    Ok(Dogrulama { tarif: Tarif { gorunum, sahneler, acilis, kapanis }, notlar })
}

fn json_kac(s: &str) -> String { s.chars().filter(|c| !c.is_control()).flat_map(|c| match c { '"' => vec!['\\', '"'], '\\' => vec!['\\', '\\'], c => vec![c] }).collect() }
/// Sahne listesini şemaya uygun JSON olarak yazar (önizleme ve "doğrulamadan geçen" kayıt için).
pub fn tarif_json(t: &Tarif) -> String {
    let sahne = |s: &TSahne| format!("{{\"metin\":[{}],\"giris\":\"{}\",\"cikis\":\"{}\",\"efekt\":[{}],\"palet\":\"{}\",\"font\":\"{}\",\"sure\":{:.1}}}",
        s.metin.iter().map(|m| format!("\"{}\"", json_kac(m))).collect::<Vec<_>>().join(","), json_kac(&s.giris), json_kac(&s.cikis),
        s.efekt.iter().map(|m| format!("\"{}\"", json_kac(m))).collect::<Vec<_>>().join(","), json_kac(&s.palet), json_kac(&s.font), s.sure);
    format!("{{\"surum\":1,\"ekran\":\"dikey\",\"gorunum\":\"{}\",\"sahneler\":[{}],\"acilis\":\"{}\",\"kapanis\":\"{}\"}}", json_kac(&t.gorunum), t.sahneler.iter().map(sahne).collect::<Vec<_>>().join(","), json_kac(&t.acilis), json_kac(&t.kapanis))
}
/// Doğrulanmış sahne listesini programın senaryo metnine çevirir (senaryo::ayristir okur).
pub fn senaryo_metni(t: &Tarif) -> String {
    let mut o = String::from("# Tarif et ile üretildi\nekran 950x1800\nfiziksel 8x25\nfps 25\n");
    if t.acilis != "yok" { o.push_str(&format!("acilis {}\n", t.acilis)); }
    if t.kapanis != "yok" { o.push_str(&format!("kapanis {}\n", t.kapanis)); }
    for s in &t.sahneler {
        // dikey LED ekran: uzun satırlar kısa satırlara bölünür (programın kendi kuralı), harf hem genişliğe hem yüksekliğe göre seçilir
        let satirlar: Vec<String> = s.metin.iter().flat_map(|m| crate::senaryo::dikey_satirlar(m)).collect();
        let en_uzun = satirlar.iter().map(|m| m.chars().count()).max().unwrap_or(1).max(1) as f32;
        let harf = (950.0 * 0.92 / (0.78 * en_uzun)).min(2969.0 * 0.9 / (1.15 * satirlar.len().max(1) as f32)).clamp(60.0, 330.0).round();
        let efekt = if s.efekt.is_empty() { "yok".to_string() } else { s.efekt.join(",") };
        let font = if s.font.is_empty() { String::new() } else { format!(" font={}", s.font) };
        o.push_str(&format!("sahne \"{}\" palet={} giris={} cikis={} sure={:.1} harf={harf} efekt={efekt}{font}\n", satirlar.join("|"), s.palet, s.giris, s.cikis, s.sure));
    }
    o
}
pub fn toplam_sure(t: &Tarif) -> f32 { t.sahneler.iter().map(|s| s.sure).sum() }

// ───────────────────────── İstem üretici ─────────────────────────
pub const ORNEK_YANIT: &str = "{\"surum\":1,\"ekran\":\"dikey\",\"gorunum\":\"duz\",\"sahneler\":[{\"metin\":[\"TERZİ ADEM ALTUN\",\"HOŞ GELDİNİZ\"],\"giris\":\"belir\",\"cikis\":\"yok\",\"efekt\":[\"parlama\"],\"palet\":\"altin\",\"font\":\"anton\",\"sure\":4.5},{\"metin\":[\"%30\",\"İNDİRİM\"],\"giris\":\"zipla\",\"cikis\":\"kucul\",\"efekt\":[\"konfeti\"],\"palet\":\"kirmizi-altin\",\"font\":\"\",\"sure\":4}],\"acilis\":\"perde\",\"kapanis\":\"fade\"}";

/// Kullanıcının tarifinden, programın gerçek katalog adlarını içeren, "yalnız JSON döndür" diyen istem üretir. İstemde sır ve müşteri verisi yoktur; yalnız kullanıcının yazdığı tarif gider.
/// 2026-11-18 [Claude#AI Opus 5.5]: sadeleştirildi; sırasıyla görev, okunurluk kuralları, şema, örnek, adlar, tarif.
pub fn istem_uret(tarif: &str, kat: &Katalog) -> String {
    let tarif: String = tarif.chars().filter(|c| !c.is_control() || *c == '\n').take(MAX_TARIF).collect();
    let liste = |v: &[String]| v.join(", ");
    let ayni = { let (mut a, mut b) = (kat.giris.clone(), kat.cikis.clone()); a.sort(); b.sort(); a == b };
    let anim = if ayni { format!("giris ve cikis (aynı liste): {}", liste(&kat.giris)) } else { format!("giris: {}\ncikis: {}", liste(&kat.giris), liste(&kat.cikis)) };
    format!("GÖREV: Bir terzi mağazasının (Terzi Adem Altun) bina cephesindeki iki dikey LED ekran için kısa reklam videosu planla. Aşağıdaki TARİF'i oku, reklam metnini SEN yaz ve sahne listesini JSON olarak ver.\n\
\n\
KURALLAR:\n\
1. YALNIZ geçerli JSON döndür. Açıklama, selamlama, soru ya da kod çiti (```) yazma.\n\
2. Tarif belirsizse soru sorma; makul bir varsayım yap (örn. oran yoksa \"BÜYÜK İNDİRİM\").\n\
3. 3-6 sahne yaz; her sahnede TEK fikir. Sahne başına en çok 3 satır, satır başına en çok 10 harf (ekran dar ve uzaktan okunur). En çok {MAX_SAHNE} sahne, toplam en çok {MAX_TOPLAM_SN:.0} sn.\n\
4. Metinler BÜYÜK HARF ve doğru Türkçe karakterli (İ, Ş, Ğ, Ü, Ö, Ç). Rakam ve yüzdeleri öne çıkar (%30, 3 TAKSİT, 499 TL).\n\
5. Sahne süresi {MIN_SURE}-{MAX_SURE} sn (genelde 3-5 sn). Son sahne işletme adı olabilir: \"TERZİ\" / \"ADEM ALTUN\".\n\
6. Adları YALNIZ aşağıdaki listelerden seç; listede olmayan ad uydurma. Sahneler arasında farklı giriş animasyonları kullan; renkler parlak ve koyu zeminde okunur olsun.\n\
7. Metinde HTML, bağlantı, dosya yolu ya da komut yazma. Kullanıcının istediği renk, animasyon ve süreye uy.\n\
\n\
ŞEMA (alan adlarını aynen kullan):\n\
{{\"surum\":1,\"ekran\":\"dikey\",\"gorunum\":\"duz\",\"sahneler\":[{{\"metin\":[\"SATIR1\",\"SATIR2\"],\"giris\":\"<ad>\",\"cikis\":\"<ad>\",\"efekt\":[\"<ad>\"],\"palet\":\"<ad>\",\"font\":\"<ad ya da boş>\",\"sure\":4}}],\"acilis\":\"<geçiş>\",\"kapanis\":\"<geçiş>\"}}\n\
\"gorunum\": \"duz\" ya da \"bina\" (bina cephesi önizlemesi).\n\
\n\
ÖRNEK YANIT:\n{ORNEK_YANIT}\n\
\n\
GEÇERLİ ADLAR:\n\
animasyon {anim}\n\
efekt (0-2 tane): {}\n\
palet (ya da hex-RRGGBB): {}\n\
font (boş bırakılabilir): {}\n\
gecis (acilis/kapanis): {}\n\
\n\
TARİF:\n<<<\n{tarif}\n>>>\n", liste(&kat.efekt), liste(&kat.palet), liste(&kat.font), liste(&kat.gecis))
}
/// Doğrulama hata verince LLM'ye geri yapıştırılacak "düzelt" istemi.
pub fn duzelt_istemi(hatalar: &[String], kat: &Katalog) -> String {
    let h = hatalar.iter().take(8).map(|x| format!("- {x}")).collect::<Vec<_>>().join("\n");
    format!("Verdiğin yanıt programda şu nedenlerle kullanılamadı:\n{h}\n\nLütfen aynı tarife göre YALNIZ düzeltilmiş JSON döndür (kod çiti ve açıklama yok). Şema:\n{ORNEK_YANIT}\nAdlar yalnız şu listelerden olsun. efekt: {}\npalet: {}\nanimasyon: {}\ngecis: {}\n", kat.efekt.join(", "), kat.palet.join(", "), kat.giris.join(", "), kat.gecis.join(", "))
}

// ───────────────────────── Çevrimdışı kural motoru ─────────────────────────
#[derive(Clone, Debug)]
pub struct Kural { pub kategori: String, pub hedef: String, pub ifadeler: Vec<String> }
const GOMULU_KURALLAR: &str = include_str!("../kurallar/tarif-kurallari.json");
/// Hazır senaryo galerisi (her giriş geçerli sahne listesi; testler doğrular).
pub const HAZIR_SENARYOLAR: &str = include_str!("../kurallar/hazir-senaryolar.json");
pub const KATEGORILER: [&str; 7] = ["giris", "cikis", "efekt", "palet", "font", "gecis", "gorunum"];

pub fn kurallari_oku(js: &str) -> Result<Vec<Kural>, String> {
    let k = json_coz(js)?;
    let Some(Json::Dizi(v)) = k.al("kurallar") else { return Err("\"kurallar\" listesi yok".into()) };
    let mut o = vec![];
    for r in v {
        let kategori = r.al("k").and_then(|x| x.yazi()).ok_or("kuralda \"k\" (kategori) yok")?.to_string();
        let hedef = r.al("h").and_then(|x| x.yazi()).ok_or("kuralda \"h\" (hedef) yok")?.to_string();
        let ifadeler = r.al("i").and_then(|x| x.dizi()).ok_or("kuralda \"i\" (ifadeler) yok")?.iter().filter_map(|x| x.yazi().map(String::from)).collect();
        o.push(Kural { kategori, hedef, ifadeler });
    }
    Ok(o)
}
pub fn gomulu_kurallar() -> &'static Vec<Kural> {
    static K: OnceLock<Vec<Kural>> = OnceLock::new();
    K.get_or_init(|| kurallari_oku(GOMULU_KURALLAR).unwrap_or_default())
}
/// Bir kuralın hedefi gerçekten katalogda var mı ve ifadeleri okunabilir mi?
pub fn kural_gecerli(k: &Kural, kat: &Katalog) -> Result<(), String> {
    let var = match k.kategori.as_str() {
        "giris" => kat.giris.contains(&k.hedef), "cikis" => kat.cikis.contains(&k.hedef), "efekt" => kat.efekt.contains(&k.hedef),
        "palet" => kat.palet_gecerli(&k.hedef), "font" => kat.font.contains(&k.hedef), "gecis" => kat.gecis.contains(&k.hedef),
        "gorunum" => k.hedef == "bina" || k.hedef == "duz",
        d => return Err(format!("bilinmeyen kategori: {d}")),
    };
    if !var { return Err(format!("{} kuralının hedefi katalogda yok: {}", k.kategori, k.hedef)); }
    if k.ifadeler.is_empty() { return Err(format!("{} kuralında ifade yok", k.hedef)); }
    for i in &k.ifadeler {
        let n = sadelestir(i);
        if n.chars().count() < 2 || n.chars().count() > 40 || n.split_whitespace().count() > 4 || !n.chars().all(|c| c.is_alphanumeric() || c == ' ') { return Err(format!("ifade uygun değil: \"{i}\"")); }
    }
    Ok(())
}
/// Katalogdaki HER ad için ad-ifade kuralı üretir (kebab-case → boşluklu: "tek-tek-sol" → "tek tek sol"); elle yazılmış kurallarla aynı hedefi tekrar etmez. Katalog büyüdükçe kurallar kendiliğinden büyür.
pub fn otomatik_kurallar(kat: &Katalog, mevcut: &[Kural]) -> Vec<Kural> {
    let var = |k: &str, h: &str| mevcut.iter().any(|r| r.kategori == k && r.hedef == h);
    let mut v = vec![];
    let mut ekle = |k: &str, h: &str, ifadeler: Vec<String>| {
        let temiz: Vec<String> = ifadeler.into_iter().filter(|i| { let n = i.split_whitespace().count(); n >= 1 && n <= 4 && i.chars().all(|c| c.is_ascii_alphanumeric() || c == ' ') && i.chars().count() >= 3 }).collect();
        if !temiz.is_empty() { v.push(Kural { kategori: k.into(), hedef: h.into(), ifadeler: temiz }); }
    };
    for a in &kat.giris { if a != "yok" && !var("giris", a) { ekle("giris", a, vec![a.replace('-', " ")]); } }
    for a in &kat.cikis { if a != "yok" && !var("cikis", a) { let ad = a.replace('-', " "); ekle("cikis", a, vec![format!("{ad} cikis"), format!("{ad} cik"), format!("cikista {ad}")]); } }
    for a in &kat.efekt { if !var("efekt", a) { ekle("efekt", a, vec![a.replace('-', " ")]); } }
    for a in &kat.font { if !var("font", a) { ekle("font", a, vec![a.replace('-', " ")]); } }
    for a in &kat.gecis { if a != "yok" && !var("gecis", a) { ekle("gecis", a, vec![a.replace('-', " ")]); } }
    for a in &kat.palet { if !var("palet", a) && !a.starts_with("hex-") { ekle("palet", a, vec![a.replace('-', " ")]); } }
    v
}
pub fn kural_ifade_sayisi(k: &[Kural]) -> usize { k.iter().map(|x| x.ifadeler.len()).sum() }

#[derive(Clone, Debug)]
struct Bel { o: String, s: String }
fn belirtecle(metin: &str) -> Vec<Bel> {
    let c: Vec<char> = metin.chars().collect();
    let mut o = vec![]; let mut t = String::new();
    for (i, &ch) in c.iter().enumerate() {
        let sayi_ici = matches!(ch, ',' | '.') && i > 0 && c[i - 1].is_ascii_digit() && c.get(i + 1).map(|x| x.is_ascii_digit()).unwrap_or(false);
        if ch.is_alphanumeric() || matches!(ch, '%' | '₺' | '§' | '/') || sayi_ici { t.push(ch); } else if !t.is_empty() { o.push(Bel { s: sadelestir(&t), o: std::mem::take(&mut t) }); }
    }
    if !t.is_empty() { o.push(Bel { s: sadelestir(&t), o: t }); }
    o
}
/// Ön ek eşleşmesi (4+ harf) ya da tam eşleşme; 5+ harfli sözcükte TEK harflik yazım hatası da kabul edilir ("zıplıyarak" → zıpla, "indrim" → indirim).
fn kelime_uyar(belirtec: &str, kelime: &str) -> bool {
    let kn = kelime.chars().count();
    if kn < 4 { return belirtec == kelime; }
    if belirtec.starts_with(kelime) { return true; }
    let bn = belirtec.chars().count();
    if kn >= 5 && bn >= kn && belirtec.chars().next() == kelime.chars().next() && belirtec.chars().nth(1) == kelime.chars().nth(1) { return unlu_hatasi(&belirtec.chars().take(kn).collect::<String>(), kelime); }
    if kn >= 6 && bn + 1 == kn && belirtec.chars().next() == kelime.chars().next() { return mesafe(belirtec, kelime) <= 1; }
    false
}
/// Aynı uzunlukta iki sözcük yalnız TEK ünlü harfte mi ayrışıyor? ("zipli"~"zipla" evet; "kapal"~"kapak" hayır: ünsüz farkı başka sözcüktür.)
pub(crate) fn unlu_hatasi(a: &str, b: &str) -> bool {
    let unlu = |c: char| "aeiou".contains(c);
    let f: Vec<(char, char)> = a.chars().zip(b.chars()).filter(|(x, y)| x != y).collect();
    a.chars().count() == b.chars().count() && f.len() == 1 && unlu(f[0].0) && unlu(f[0].1)
}
fn say_sozcuk(s: &str) -> Option<f32> {
    Some(match s { "bir" => 1.0, "iki" => 2.0, "uc" => 3.0, "dort" => 4.0, "bes" => 5.0, "alti" => 6.0, "yedi" => 7.0, "sekiz" => 8.0, "dokuz" => 9.0, "on" => 10.0, "onbes" => 15.0, _ => return None })
}
fn sayi_belirtec(s: &str) -> Option<f32> { if s.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false) { s.replace(',', ".").parse().ok() } else { say_sozcuk(s) } }
const DURAK: &[&str] = &["yaz", "yazsin", "yazin", "yazi", "yazar", "goster", "gosterin", "olsun", "olacak", "ekle", "ekran", "ekrana", "ekranda", "sahne", "video", "mesaj", "metin", "tarif", "istiyorum", "lutfen", "bir", "ve", "ile", "icin", "gibi", "kadar", "bu", "su", "da", "de", "ki", "acilis", "kapanis", "gecis", "ile", "olarak", "uzerinde", "yapsin", "yap", "olsunda", "gelsin", "gelir", "gel", "sursun", "sur", "renk", "reng", "efek", "cikis", "giris", "kalsin", "kal", "yansin", "ucsun", "tipi", "zemin", "satir", "kapan", "olur", "imkan", "yazilsin", "yazdir", "cikar"];

#[derive(Default, Clone, Debug)]
struct Ozellik { giris: Option<String>, cikis: Option<String>, palet: Option<String>, font: Option<String>, efekt: Vec<String>, sure: Option<f32> }
#[derive(Default, Debug)]
pub struct Cozum { pub tarif: Option<Tarif>, pub notlar: Vec<String>, pub anladim: Vec<String>, pub anlasilmayan: Vec<String> }

fn ifade_listesi(k: &[Kural]) -> Vec<(Vec<String>, usize)> {
    let mut v = vec![];
    for (ki, r) in k.iter().enumerate() { for i in &r.ifadeler { let w: Vec<String> = sadelestir(i).split_whitespace().map(String::from).collect(); if !w.is_empty() { v.push((w, ki)); } } }
    v
}
/// Belirtilen kategorilerdeki ifadeleri bulur; uzun ifade önce, çakışan belirteç iki kez kullanılmaz.
fn eslesmeler(tok: &[Bel], kurallar: &[Kural], ifadeler: &[(Vec<String>, usize)], kategoriler: &[&str], ayrik: &mut [bool]) -> Vec<(usize, usize, usize)> {
    let mut aday = vec![];
    for (w, ki) in ifadeler {
        if !kategoriler.contains(&kurallar[*ki].kategori.as_str()) { continue; }
        for i in 0..tok.len() {
            if i + w.len() <= tok.len() && w.iter().enumerate().all(|(j, x)| kelime_uyar(&tok[i + j].s, x)) { aday.push((i, w.len(), *ki, w.iter().map(|x| x.chars().count()).sum::<usize>())); }
        }
    }
    aday.sort_by(|a, b| b.1.cmp(&a.1).then(b.3.cmp(&a.3)).then(a.0.cmp(&b.0)));
    let mut secilen = vec![];
    for (i, n, ki, _) in aday {
        if (i..i + n).any(|x| ayrik[x]) { continue; }
        for x in i..i + n { ayrik[x] = true; }
        secilen.push((i, n, ki));
    }
    secilen.sort();
    secilen
}
fn kaynak(tok: &[Bel], i: usize, n: usize) -> String { tok[i..i + n].iter().map(|b| b.o.to_lowercase()).collect::<Vec<_>>().join(" ") }

/// (belirteçler, virgülle_ayrılmış_mı): virgülden sonraki cümlecikte "kalan kelimelerden metin tahmini" yapılmaz.
fn klozlara_bol(metin: &str, alintilar: &mut Vec<String>) -> Vec<(Vec<Bel>, bool)> {
    // 1) tırnak içindekileri yer tutucuya çevir
    let c: Vec<char> = metin.chars().collect();
    let mut m = String::new(); let mut i = 0;
    while i < c.len() {
        let (acan, kapan) = match c[i] { '"' => ('"', '"'), '“' => ('“', '”'), '«' => ('«', '»'), _ => ('\0', '\0') };
        if acan != '\0' { if let Some(j) = c[i + 1..].iter().position(|&x| x == kapan || (acan == '“' && x == '"')) { let ic: String = c[i + 1..i + 1 + j].iter().collect(); alintilar.push(ic); m.push_str(&format!(" §{}§ ", alintilar.len() - 1)); i += j + 2; continue; } }
        m.push(c[i]); i += 1;
    }
    // 2) cümle ayırıcılarda böl (sayı içindeki nokta ve virgül hariç)
    let mc: Vec<char> = m.chars().collect();
    let mut parcalar: Vec<(String, bool)> = vec![(String::new(), false)];
    for (k, &ch) in mc.iter().enumerate() {
        let sayi_ici = matches!(ch, '.' | ',') && k > 0 && mc[k - 1].is_ascii_digit() && mc.get(k + 1).map(|x| x.is_ascii_digit()).unwrap_or(false);
        if matches!(ch, '\n' | ';' | '!' | '?') || (matches!(ch, '.' | ',') && !sayi_ici) { parcalar.push((String::new(), ch == ',')); } else { parcalar.last_mut().unwrap().0.push(ch); }
    }
    // 3) sıra bağlaçlarında böl
    let mut klozlar: Vec<(Vec<Bel>, bool)> = vec![];
    for (p, virgul) in parcalar {
        let mut virgul = virgul;
        let tok = belirtecle(&p);
        let mut guncel: Vec<Bel> = vec![];
        let mut i = 0;
        while i < tok.len() {
            let t = &tok[i].s; let sonraki = tok.get(i + 1).map(|x| x.s.as_str()).unwrap_or("");
            let ikili = (t == "en" && sonraki == "son") || (t == "daha" && sonraki == "sonra") || (t == "son" && sonraki == "olarak");
            let tekli = t.starts_with("sonra") || t.starts_with("ardindan") || t == "derken" || t == "once";
            if ikili || tekli { if !guncel.is_empty() { klozlar.push((std::mem::take(&mut guncel), virgul)); } virgul = false; i += if ikili { 2 } else { 1 }; continue; }
            guncel.push(tok[i].clone()); i += 1;
        }
        if !guncel.is_empty() { klozlar.push((guncel, virgul)); }
    }
    klozlar
}

fn sure_bul(tok: &[Bel], ayrik: &mut [bool]) -> Option<f32> {
    let n = tok.len();
    for i in 0..n {
        let s = tok[i].s.as_str();
        // "yarım saniye", "bir buçuk saniye", "iki buçuk saniye"
        if s == "yarim" && tok.get(i + 1).map(|x| x.s.starts_with("sani") || x.s.starts_with("sn")).unwrap_or(false) { ayrik[i] = true; ayrik[i + 1] = true; return Some(0.5); }
        if let Some(d) = sayi_belirtec(s) {
            if tok.get(i + 1).map(|x| x.s == "bucuk").unwrap_or(false) && tok.get(i + 2).map(|x| x.s.starts_with("sani") || x.s.starts_with("sn")).unwrap_or(false) { for k in i..i + 3 { ayrik[k] = true; } return Some(d + 0.5); }
            if tok.get(i + 1).map(|x| x.s.starts_with("sani") || x.s == "sn" || x.s == "s").unwrap_or(false) { ayrik[i] = true; ayrik[i + 1] = true; return Some(d); }
        }
        // "2sn", "3sn"
        if let Some(sayi) = s.strip_suffix("sn").and_then(|x| x.replace(',', ".").parse::<f32>().ok()) { ayrik[i] = true; return Some(sayi); }
    }
    for i in 0..n {
        let s = tok[i].s.as_str();
        let onceki_cok = i > 0 && tok[i - 1].s == "cok";
        let d = match s { "kisa" | "kisaca" => Some(if onceki_cok { 2.0 } else { 3.0 }), "uzun" | "uzunca" => Some(if onceki_cok { 10.0 } else { 7.0 }), _ => None };
        if let Some(d) = d { ayrik[i] = true; if onceki_cok { ayrik[i - 1] = true; } return Some(d); }
    }
    None
}

fn alinti_satirlari(a: &str) -> Vec<String> { a.split(['|', '\n']).map(|x| buyuk(x.trim())).filter(|x| !x.is_empty()).collect() }

/// Türkçe tarifi sahne listesine çevirir. Belirsiz kelimede SORMAZ; "anladım" ve "anlaşılmayan" listelerini döndürür.
pub fn kural_coz(metin: &str, kat: &Katalog, kullanici: &[Kural]) -> Cozum {
    let mut z = Cozum::default();
    let metin: String = metin.chars().filter(|c| !c.is_control() || *c == '\n').take(MAX_TARIF).collect();
    let mut kurallar: Vec<Kural> = gomulu_kurallar().clone();
    for k in kullanici { if kural_gecerli(k, kat).is_ok() { kurallar.push(k.clone()); } }
    let oto = otomatik_kurallar(kat, &kurallar);
    kurallar.extend(oto);
    let ifadeler = ifade_listesi(&kurallar);
    let mut alintilar = vec![];
    let klozlar = klozlara_bol(&metin, &mut alintilar);
    let (mut acilis, mut kapanis, mut gorunum) = ("yok".to_string(), "yok".to_string(), "duz".to_string());
    let mut sahneler: Vec<TSahne> = vec![];
    let mut bekleyen = Ozellik::default();
    let kucuk_harf_var = |tok: &[Bel]| tok.iter().any(|b| b.o.chars().any(|c| c.is_lowercase()));
    for (tok, virgulla) in klozlar {
        let n = tok.len();
        let mut ayrik = vec![false; n]; // metin, süre ya da kural tarafından alınmış belirteçler
        let yer: Vec<Option<usize>> = tok.iter().map(|b| b.o.strip_prefix('§').and_then(|x| x.strip_suffix('§')).and_then(|x| x.parse::<usize>().ok())).collect();
        for (i, y) in yer.iter().enumerate() { if y.is_some() { ayrik[i] = true; } }
        // BÜYÜK HARFLİ metin (cümlenin tamamı büyük harfle yazılmadıysa)
        let mut buyuk_kosular: Vec<(usize, usize)> = vec![];
        if kucuk_harf_var(&tok) && yer.iter().all(|x| x.is_none()) {
            let mut i = 0;
            while i < n {
                let harfler: Vec<char> = tok[i].o.chars().filter(|c| c.is_alphabetic()).collect();
                let buyuk_mu = harfler.len() >= 2 && harfler.iter().all(|c| harf_buyuk_mu(*c));
                // "499 TL" gibi birimler BÜYÜK HARFLİ metin sayılmaz (önceki sayıyla birlikte desen olarak okunur)
                let birim = matches!(tok[i].s.as_str(), "tl" | "ytl" | "kdv" | "xl" | "xxl");
                if buyuk_mu && !birim {
                    let b = i; i += 1;
                    while i < n {
                        let h: Vec<char> = tok[i].o.chars().filter(|c| c.is_alphabetic()).collect();
                        let devam = (h.len() >= 2 && h.iter().all(|c| harf_buyuk_mu(*c))) || (h.is_empty() && tok[i].o.chars().any(|c| c.is_ascii_digit() || c == '%' || c == '₺'));
                        if !devam { break; }
                        i += 1;
                    }
                    buyuk_kosular.push((b, i)); for k in b..i { ayrik[k] = true; }
                } else { i += 1; }
            }
        }
        let sure = sure_bul(&tok, &mut ayrik).map(|x| if x.is_finite() { x.clamp(0.1, 60.0) } else { 4.5 });
        // geçiş kuralları yalnız "açılış/kapanış/geçiş" bağlamında geçerli
        let (ac, kap, gec) = (tok.iter().any(|b| b.s.starts_with("acilis") || b.s.starts_with("basla")), tok.iter().any(|b| b.s.starts_with("kapanis") || b.s.starts_with("bitis") || b.s.starts_with("bitir")), tok.iter().any(|b| b.s.starts_with("gecis") || b.s.starts_with("geciş")));
        let mut kategoriler = vec!["giris", "cikis", "efekt", "palet", "font", "gorunum"];
        if ac || kap || gec { kategoriler.insert(0, "gecis"); }
        let es = eslesmeler(&tok, &kurallar, &ifadeler, &kategoriler, &mut ayrik);
        let mut oz = Ozellik { sure, ..Default::default() };
        let mut aciklama: Vec<String> = vec![];
        for (i, nn, ki) in &es {
            let r = &kurallar[*ki]; let kay = kaynak(&tok, *i, *nn);
            match r.kategori.as_str() {
                "giris" => if oz.giris.is_none() { oz.giris = Some(r.hedef.clone()); aciklama.push(format!("\"{kay}\" → giriş: {}", r.hedef)); },
                "cikis" => if oz.cikis.is_none() { oz.cikis = Some(r.hedef.clone()); aciklama.push(format!("\"{kay}\" → çıkış: {}", r.hedef)); },
                "efekt" => if !oz.efekt.contains(&r.hedef) { oz.efekt.push(r.hedef.clone()); aciklama.push(format!("\"{kay}\" → efekt: {}", r.hedef)); },
                "palet" => if oz.palet.is_none() { oz.palet = Some(r.hedef.clone()); aciklama.push(format!("\"{kay}\" → renk: {}", r.hedef)); },
                "font" => if oz.font.is_none() { oz.font = Some(r.hedef.clone()); aciklama.push(format!("\"{kay}\" → yazı tipi: {}", r.hedef)); },
                "gorunum" => { gorunum = r.hedef.clone(); aciklama.push(format!("\"{kay}\" → görünüm: {}", r.hedef)); }
                "gecis" => {
                    if kap && !ac { kapanis = r.hedef.clone(); aciklama.push(format!("\"{kay}\" → kapanış geçişi: {}", r.hedef)); }
                    else if ac && !kap { acilis = r.hedef.clone(); aciklama.push(format!("\"{kay}\" → açılış geçişi: {}", r.hedef)); }
                    else { if acilis == "yok" { acilis = r.hedef.clone(); } if kapanis == "yok" { kapanis = r.hedef.clone(); } aciklama.push(format!("\"{kay}\" → açılış ve kapanış geçişi: {}", r.hedef)); }
                }
                _ => {}
            }
        }
        if let Some(s) = oz.sure { aciklama.push(format!("süre: {s:.1} sn")); }
        // Metin: 1) tırnak içi, 2) BÜYÜK HARF, 3) desenler (%30, 3 taksit, tutar), 4) kalan kelimeler (tahmin)
        let mut satirlar: Vec<String> = vec![]; let mut tahmin = false;
        for y in yer.iter().flatten() { if let Some(a) = alintilar.get(*y) { satirlar.extend(alinti_satirlari(a)); } }
        if satirlar.is_empty() { for (b, e) in &buyuk_kosular { let s = tok[*b..*e].iter().map(|x| x.o.clone()).collect::<Vec<_>>().join(" "); satirlar.push(buyuk(&s)); } }
        if satirlar.is_empty() {
            let kalan: Vec<usize> = (0..n).filter(|i| !ayrik[*i]).collect();
            for &i in &kalan {
                let s = tok[i].s.as_str();
                let sonraki = kalan.iter().find(|&&j| j > i).map(|&j| tok[j].s.as_str()).unwrap_or("");
                if s.starts_with('%') && s.len() > 1 { satirlar.push(if kalan.iter().any(|&j| tok[j].s.starts_with("indirim")) { format!("{}", buyuk(&tok[i].o)) } else { buyuk(&tok[i].o) }); if kalan.iter().any(|&j| tok[j].s.starts_with("indirim")) { satirlar.push("İNDİRİM".into()); } }
                else if s == "yuzde" && sayi_belirtec(sonraki).is_some() { let d = kalan.iter().find(|&&j| j > i).copied().unwrap(); satirlar.push(format!("%{}", tok[d].o)); if kalan.iter().any(|&j| tok[j].s.starts_with("indirim")) { satirlar.push("İNDİRİM".into()); } }
                else if s.chars().all(|c| c.is_ascii_digit()) && sonraki.starts_with("taksit") { satirlar.push(format!("{} TAKSİT", tok[i].o)); }
                else if (s.chars().all(|c| c.is_ascii_digit()) && (sonraki == "tl" || sonraki == "lira")) || s.starts_with('₺') { satirlar.push(if s.starts_with('₺') { format!("{} TL", tok[i].o.trim_start_matches('₺')) } else { format!("{} TL", tok[i].o) }); }
            }
            if !satirlar.is_empty() { tahmin = true; }
        }
        if satirlar.is_empty() && !virgulla {
            let mut kalan: Vec<String> = vec![];
            for (i, b) in tok.iter().enumerate() {
                if ayrik[i] { continue; }
                let son = i + 1 == n;
                if DURAK.iter().any(|d| kelime_uyar(&b.s, d) && !(*d == "yaz" && !son)) { continue; }
                kalan.push(b.o.clone());
            }
            if !kalan.is_empty() {
                let s = buyuk(&kalan.join(" "));
                // en çok iki satıra böl (ortadan, kelime sınırında)
                if s.chars().count() > 22 { let k: Vec<&str> = s.split(' ').collect(); let yarim = k.len() / 2; satirlar.push(k[..yarim.max(1)].join(" ")); if yarim.max(1) < k.len() { satirlar.push(k[yarim.max(1)..].join(" ")); } } else { satirlar.push(s); }
                tahmin = true;
            }
        } else {
            for (i, b) in tok.iter().enumerate() {
                if ayrik[i] { continue; }
                if DURAK.iter().any(|d| kelime_uyar(&b.s, d) && !(*d == "yaz" && i + 1 != n)) { continue; }
                if satirlar.iter().any(|s| sadelestir(s).contains(&b.s)) { continue; }
                if !b.s.chars().all(|c| c.is_ascii_digit()) && say_sozcuk(&b.s).is_none() { z.anlasilmayan.push(b.o.to_lowercase()); }
            }
        }
        let var_ozellik = oz.giris.is_some() || oz.cikis.is_some() || oz.palet.is_some() || oz.font.is_some() || !oz.efekt.is_empty() || oz.sure.is_some();
        if satirlar.is_empty() {
            if var_ozellik {
                // metinsiz cümle: önceki sahneye ek özellik; yoksa sonraki sahneye bekletilir
                if let Some(son) = sahneler.last_mut() {
                    if let Some(x) = &oz.giris { son.giris = x.clone(); } if let Some(x) = &oz.cikis { son.cikis = x.clone(); }
                    if let Some(x) = &oz.palet { son.palet = x.clone(); } if let Some(x) = &oz.font { son.font = x.clone(); }
                    for e in &oz.efekt { if !son.efekt.contains(e) { son.efekt.push(e.clone()); } }
                    if let Some(x) = oz.sure { son.sure = x; }
                    z.anladim.push(format!("Sahne {} için ek: {}", sahneler.len(), aciklama.join("; ")));
                } else { bekleyen = oz.clone(); z.anladim.push(format!("Sonraki sahne için: {}", aciklama.join("; "))); }
            } else if !aciklama.is_empty() { z.anladim.push(aciklama.join("; ")); }
            continue;
        }
        if satirlar.len() > MAX_SATIR { satirlar.truncate(MAX_SATIR); }
        let al = |a: &Option<String>, b: &Option<String>, d: &str| a.clone().or_else(|| b.clone()).unwrap_or_else(|| d.to_string());
        let mut efekt = oz.efekt.clone(); for e in &bekleyen.efekt { if !efekt.contains(e) { efekt.push(e.clone()); } }
        let sh = TSahne { metin: satirlar.clone(), giris: al(&oz.giris, &bekleyen.giris, "belir"), cikis: al(&oz.cikis, &bekleyen.cikis, "yok"), efekt, palet: al(&oz.palet, &bekleyen.palet, "altin"), font: al(&oz.font, &bekleyen.font, ""), sure: oz.sure.or(bekleyen.sure).unwrap_or(4.5) };
        bekleyen = Ozellik::default();
        let mut satir = format!("Sahne {}: \"{}\"{}", sahneler.len() + 1, satirlar.join(" / "), if tahmin { " (metni tahmin ettim)" } else { "" });
        if !aciklama.is_empty() { satir.push_str(&format!(" — {}", aciklama.join("; "))); }
        z.anladim.push(satir);
        sahneler.push(sh);
    }
    z.anlasilmayan.sort(); z.anlasilmayan.dedup();
    if sahneler.is_empty() { z.notlar.push("Tarifte sahne metni bulunamadı. Yazılacak metni tırnak içinde (\"...\") ya da BÜYÜK HARFLE yazın.".into()); return z; }
    if sahneler.len() > MAX_SAHNE { z.notlar.push(format!("{} sahne bulundu; en çok {MAX_SAHNE} sahne alınır, fazlası atıldı.", sahneler.len())); sahneler.truncate(MAX_SAHNE); }
    // aynı doğrulama hattından geçir (temizlik, sınırlar, katalog denetimi)
    let ham = tarif_json(&Tarif { gorunum, sahneler, acilis, kapanis });
    match dogrula(&ham, kat) { Ok(d) => { z.notlar.extend(d.notlar); z.tarif = Some(d.tarif); } Err(h) => z.notlar.extend(h) }
    z
}

/// Kullanıcı kural dosyasına yeni eşleme ekler ("Bunu kural olarak öğret"). Hedef katalogda yoksa reddedilir. Dosya biçimi gömülü kurallarla aynıdır.
pub fn kural_ekle(mevcut_json: Option<&str>, kategori: &str, ifade: &str, hedef: &str, kat: &Katalog) -> Result<String, String> {
    let mut v = match mevcut_json { Some(j) if !j.trim().is_empty() => kurallari_oku(j).map_err(|e| format!("kural dosyası okunamadı: {e}"))?, _ => vec![] };
    let ifade = sadelestir(&ifade.split_whitespace().collect::<Vec<_>>().join(" "));
    let yeni = Kural { kategori: kategori.to_string(), hedef: hedef.to_string(), ifadeler: vec![ifade.clone()] };
    kural_gecerli(&yeni, kat)?;
    match v.iter_mut().find(|k| k.kategori == kategori && k.hedef == hedef) { Some(k) => { if !k.ifadeler.iter().any(|x| sadelestir(x) == sadelestir(&ifade)) { k.ifadeler.push(ifade); } } None => v.push(yeni) }
    if v.len() > 500 || kural_ifade_sayisi(&v) > 2000 { return Err("Kullanıcı kuralı sınırına ulaşıldı.".into()); }
    let govde = v.iter().map(|k| format!("{{\"k\":\"{}\",\"h\":\"{}\",\"i\":[{}]}}", json_kac(&k.kategori), json_kac(&k.hedef), k.ifadeler.iter().map(|i| format!("\"{}\"", json_kac(i))).collect::<Vec<_>>().join(","))).collect::<Vec<_>>().join(",\n");
    Ok(format!("{{\"aciklama\":\"TAA Vidyo Yapıcı kullanıcı tarif kuralları. Sahip: kullanıcı. Program 'Bunu kural olarak öğret' ile ekler; elle düzenlenebilir, silme için Geri Dönüşüm Kutusu'na taşıyın.\",\"surum\":1,\"kurallar\":[\n{govde}\n]}}\n"))
}

pub mod niyet;

#[cfg(test)]
mod testler;
