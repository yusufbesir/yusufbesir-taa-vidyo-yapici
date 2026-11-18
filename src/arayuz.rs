//! Yerel arayüz: 127.0.0.1 üzerinde küçük bir HTTP sunucusu; sayfa tarayıcıda açılır.
//! Dış bağımlılık yok. Yalnız bu bilgisayardan erişilir (Host ve X-LED denetimi).
use crate::{png, uret};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

const SAYFA: &str = include_str!("../web/index.html");
const LOGO: &str = include_str!("../logo/logo.svg");
/// Açılış animasyonları (web/acilis): ana sayfa her açılışta rastgele birini iframe'de oynatır. Logo tek yerde: logo.js (İkiz Kabartma).
const ACILIS: &[(&str, &str)] = &[
    ("zaman", include_str!("../web/acilis/zaman.html")),
    ("gece", include_str!("../web/acilis/gece.html")),
    ("kalibrasyon", include_str!("../web/acilis/kalibrasyon.html")),
    ("dikis", include_str!("../web/acilis/dikis.html")),
    ("katlama", include_str!("../web/acilis/katlama.html")),
    ("led", include_str!("../web/acilis/led.html")),
    ("perde", include_str!("../web/acilis/perde.html")),
    ("kurgu", include_str!("../web/acilis/kurgu.html")),
    ("iki-ekran", include_str!("../web/acilis/iki-ekran.html")),
    ("isik", include_str!("../web/acilis/isik.html")),
    ("odak", include_str!("../web/acilis/odak.html")),
    ("zaman-sol", include_str!("../web/acilis/zaman-sol.html")),
    ("gece-terra", include_str!("../web/acilis/gece-terra.html")),
    ("kalibrasyon-luna", include_str!("../web/acilis/kalibrasyon-luna.html")),
];
const ACILIS_LOGO_JS: &str = include_str!("../web/acilis/logo.js");
const YZ_DAR: &[u8] = include_bytes!("../fonts/BarlowCondensed-Black.ttf");
const YZ_RAF: &[u8] = include_bytes!("../fonts/Audiowide-Regular.ttf");
pub const SURUM: &str = env!("CARGO_PKG_VERSION");

pub const ORNEKLER: [(&str, &str); 11] = [
    ("animasyon-galerisi", include_str!("../senaryolar/animasyon-galerisi.txt")),
    ("tv-referans", include_str!("../senaryolar/tv-referans.txt")),
    ("tv-afis", include_str!("../senaryolar/tv-afis.txt")),
    ("ornek-gibi", include_str!("../senaryolar/ornek-gibi.txt")),
    ("toplan-dagil", include_str!("../senaryolar/toplan-dagil.txt")),
    ("daktilo-kivilcim", include_str!("../senaryolar/daktilo-kivilcim.txt")),
    ("elastik-parcacik", include_str!("../senaryolar/elastik-parcacik.txt")),
    ("tv-klasik", include_str!("../senaryolar/tv-klasik.txt")),
    ("tv-iki-mesaj", include_str!("../senaryolar/tv-iki-mesaj.txt")),
    ("perde-hizli", include_str!("../senaryolar/perde-hizli.txt")),
    ("ornek-cicekli", include_str!("../senaryolar/ornek-cicekli.txt")),
];

#[derive(Default)]
struct Durum { calisiyor: bool, kare: u32, toplam: u32, dosya: String, mesaj: String, hata: bool }

struct Ortak {
    durum: Mutex<Durum>,
    iptal: AtomicBool,
    onizleme: Mutex<()>,
    klip: Mutex<()>,
    son_nabiz: Mutex<Instant>,
    port: u16,
    jeton: String,
}

fn json_kacis(s: &str) -> String {
    let mut o = String::new();
    for c in s.chars() { match c { '"' => o.push_str("\\\""), '\\' => o.push_str("\\\\"), '\n' => o.push_str("\\n"), '\r' => {}, c if (c as u32) < 32 => {}, c => o.push(c) } }
    o
}

/// Videoların yazılacağı klasör (LED_CIKTI ile değiştirilebilir).
fn sablon_klasoru() -> PathBuf { let k = cikti_klasoru().join("Şablonlarım"); let _ = std::fs::create_dir_all(&k); k }
fn guvenli_ad(a: &str) -> String { let t: String = a.trim().chars().map(|c| if c.is_alphanumeric() || c == '-' || c == '_' { c } else { '-' }).collect(); t.trim_matches('-').chars().take(40).collect() }
/// Kullanıcının kaydettiği şablonlar: (ad "u-…", içerik).
fn kullanici_sablonlari() -> Vec<(String, String)> {
    let mut v: Vec<(String, String)> = std::fs::read_dir(sablon_klasoru()).into_iter().flatten().flatten()
        .filter(|e| e.path().extension().map(|x| x == "txt").unwrap_or(false))
        .filter_map(|e| { let ad = e.path().file_stem()?.to_string_lossy().to_string(); let m = std::fs::read_to_string(e.path()).ok()?; Some((format!("u-{ad}"), m)) }).collect();
    v.sort(); v
}
fn kullanici_sablon_bul(ad: &str) -> Option<String> { kullanici_sablonlari().into_iter().find(|x| x.0 == ad).map(|x| x.1) }

pub fn cikti_klasoru() -> PathBuf {
    let k = if let Ok(p) = std::env::var("LED_CIKTI") { PathBuf::from(p) }
        else if let Ok(p) = std::env::var("USERPROFILE") { PathBuf::from(p).join("Videos").join("TAA Vidyo Yapıcı") }
        else if let Ok(p) = std::env::var("HOME") { PathBuf::from(p).join("taa-vidyo-yapici-videolar") }
        else { PathBuf::from("taa-vidyo-yapici-videolar") };
    let _ = std::fs::create_dir_all(&k);
    k
}

/// Kullanıcı girdisini güvenli dosya adına çevirir (klasör ayırıcı yok, uzantı kodeğe göre).
pub fn dosya_adi(ham: &str, kodek: &str) -> String {
    let uzanti = if kodek == "hap" { "mov" } else { "mp4" };
    let govde: String = ham.trim().chars().map(|c| if c.is_alphanumeric() || c == '-' || c == '_' || c == ' ' { c } else if c == '.' { '_' } else { '_' }).collect();
    let govde = govde.trim_matches(|c: char| c == '_' || c == ' ').to_string();
    let govde = if govde.is_empty() { "video".to_string() } else { govde.chars().take(80).collect() };
    format!("{govde}.{uzanti}")
}

fn sorgu(yol: &str, k: &str) -> Option<String> {
    let q = yol.split_once('?')?.1;
    q.split('&').find_map(|p| p.split_once('=').filter(|(a, _)| *a == k).map(|(_, v)| url_coz(v)))
}
fn url_coz(s: &str) -> String {
    let b = s.as_bytes();
    let (mut o, mut i) = (vec![], 0);
    while i < b.len() {
        match b[i] {
            b'%' if i + 2 < b.len() => {
                if let Some(v) = std::str::from_utf8(&b[i + 1..i + 3]).ok().and_then(|h| u8::from_str_radix(h, 16).ok()) { o.push(v); i += 3; continue; }
                o.push(b'%');
            }
            b'+' => o.push(b' '),
            c => o.push(c),
        }
        i += 1;
    }
    String::from_utf8_lossy(&o).into_owned()
}

/// Oturum jetonu: her açılışta rastgele (işletim sistemi rastgeleliği, getrandom); yalnız bu süreçte yaşar.
fn rastgele_jeton() -> String {
    let mut b = [0u8; 32];
    getrandom::fill(&mut b).expect("işletim sistemi rastgeleliği alınamadı");
    b.iter().map(|x| format!("{x:02x}")).collect()
}
/// Cookie başlığında `taa-oturum=<jeton>` var mı (sabit sürede karşılaştırır).
fn cerez_jeton_var(cerez: &str, jeton: &str) -> bool {
    cerez.split(';').filter_map(|p| p.trim().strip_prefix("taa-oturum=")).any(|v| v.len() == jeton.len() && v.bytes().zip(jeton.bytes()).fold(0u8, |a, (x, y)| a | (x ^ y)) == 0)
}
fn yanit_cerezli(s: &mut TcpStream, tur: &str, govde: &[u8], jeton: &str) {
    let b = format!("HTTP/1.1 200 OK\r\nContent-Type: {tur}\r\nContent-Length: {}\r\nCache-Control: no-store\r\nSet-Cookie: taa-oturum={jeton}; HttpOnly; SameSite=Strict; Path=/\r\nConnection: close\r\n\r\n", govde.len());
    let _ = s.write_all(b.as_bytes());
    let _ = s.write_all(govde);
}

fn yanit(s: &mut TcpStream, kod: u16, tur: &str, govde: &[u8]) {
    let ad = match kod { 200 => "OK", 400 => "Bad Request", 401 => "Unauthorized", 403 => "Forbidden", 404 => "Not Found", 409 => "Conflict", _ => "Error" };
    let b = format!("HTTP/1.1 {kod} {ad}\r\nContent-Type: {tur}\r\nContent-Length: {}\r\nCache-Control: no-store\r\nConnection: close\r\n\r\n", govde.len());
    let _ = s.write_all(b.as_bytes());
    let _ = s.write_all(govde);
}

fn oku(s: &mut TcpStream) -> Option<(String, String, Vec<(String, String)>, Vec<u8>)> {
    let _ = s.set_read_timeout(Some(Duration::from_secs(10)));
    let mut tampon = vec![];
    let mut parca = [0u8; 8192];
    let son = loop {
        let n = s.read(&mut parca).ok()?;
        if n == 0 { return None; }
        tampon.extend(&parca[..n]);
        if let Some(i) = tampon.windows(4).position(|w| w == b"\r\n\r\n") { break i; }
        if tampon.len() > 65536 { return None; }
    };
    let baslik = String::from_utf8_lossy(&tampon[..son]).into_owned();
    let mut satirlar = baslik.lines();
    let ilk = satirlar.next()?;
    let mut p = ilk.split_whitespace();
    let (yontem, yol) = (p.next()?.to_string(), p.next()?.to_string());
    let basliklar: Vec<(String, String)> = satirlar.filter_map(|l| l.split_once(':').map(|(a, b)| (a.trim().to_lowercase(), b.trim().to_string()))).collect();
    let uzun: usize = basliklar.iter().find(|(a, _)| a == "content-length").and_then(|(_, v)| v.parse().ok()).unwrap_or(0);
    if uzun > 1_000_000 { return None; }
    let mut govde = tampon[son + 4..].to_vec();
    while govde.len() < uzun {
        let n = s.read(&mut parca).ok()?;
        if n == 0 { return None; }
        govde.extend(&parca[..n]);
    }
    govde.truncate(uzun);
    Some((yontem, yol, basliklar, govde))
}

/// Kayıtlı dosya adı çıktı klasöründeyse ve biçimi videoysa tam yolunu döndürür (yoksa None).
fn son_video_yolu(dosya: &str) -> Option<PathBuf> {
    if dosya.is_empty() { return None; }
    let p = PathBuf::from(dosya);
    let ad = p.file_name()?.to_string_lossy().to_lowercase();
    if !(ad.ends_with(".mp4") || ad.ends_with(".mov")) { return None; }
    let tam = if p.is_absolute() { p } else { cikti_klasoru().join(p) };
    let kok = cikti_klasoru();
    if tam.parent().map(|x| x == kok.as_path()).unwrap_or(false) && tam.is_file() { Some(tam) } else { None }
}
#[cfg(windows)]
fn son_video_ac(p: &std::path::Path) {
    use std::os::windows::process::CommandExt;
    let _ = std::process::Command::new("cmd").args(["/c", "start", "", &p.display().to_string()]).creation_flags(0x0800_0000).spawn();
}
#[cfg(not(windows))]
fn son_video_ac(p: &std::path::Path) { let _ = std::process::Command::new("xdg-open").arg(p).spawn(); }

fn klasor_ac(k: &std::path::Path) {
    #[cfg(windows)]
    { use std::os::windows::process::CommandExt; let _ = std::process::Command::new("explorer").arg(k).creation_flags(0x0800_0000).spawn(); }
    #[cfg(not(windows))]
    { let _ = std::process::Command::new("xdg-open").arg(k).spawn(); }
}

fn isle(mut s: TcpStream, o: Arc<Ortak>) {
    let Some((yontem, yol, bas, govde)) = oku(&mut s) else { return };
    let ana = bas.iter().find(|(a, _)| a == "host").map(|(_, v)| v.as_str()).unwrap_or("");
    if ana != format!("127.0.0.1:{}", o.port) && ana != format!("localhost:{}", o.port) { return yanit(&mut s, 403, "text/plain; charset=utf-8", b"yasak"); }
    let cerez = bas.iter().find(|(a, _)| a == "cookie").map(|(_, v)| v.as_str()).unwrap_or("");
    let yetkili = cerez_jeton_var(cerez, &o.jeton);
    if yontem == "GET" && yol.split('?').next() == Some("/") && sorgu(&yol, "t").as_deref() == Some(o.jeton.as_str()) {
        return yanit_cerezli(&mut s, "text/html; charset=utf-8", SAYFA.as_bytes(), &o.jeton);
    }
    if !yetkili { return yanit(&mut s, 401, "text/plain; charset=utf-8", b"oturum gerekli"); }
    let yazi = String::from_utf8_lossy(&govde).into_owned();
    let yolu = yol.split('?').next().unwrap_or("/").to_string();
    if yontem == "POST" && !bas.iter().any(|(a, v)| a == "x-led" && v == "1") { return yanit(&mut s, 403, "text/plain; charset=utf-8", b"yasak"); }
    let metin = "text/plain; charset=utf-8";
    match (yontem.as_str(), yolu.as_str()) {
        ("GET", "/") => yanit(&mut s, 200, "text/html; charset=utf-8", SAYFA.as_bytes()),
        ("GET", "/ayar") => yanit(&mut s, 200, "application/json; charset=utf-8", crate::ayar::json_yaz(&crate::ayar::oku(&crate::ayar::varsayilan_yol())).as_bytes()),
        ("POST", "/ayar") => match crate::ayar::istek_coz(&yazi) {
            Some((k, v)) => match crate::ayar::degistir(&crate::ayar::varsayilan_yol(), &k, v.as_deref()) { Ok(()) => yanit(&mut s, 200, metin, b"ok"), Err(e) => yanit(&mut s, 400, metin, e.as_bytes()) },
            None => yanit(&mut s, 400, metin, b"gecersiz istek"),
        },
        ("GET", "/acilis/logo.js") => yanit(&mut s, 200, "text/javascript; charset=utf-8", ACILIS_LOGO_JS.as_bytes()),
        ("GET", y) if y.starts_with("/acilis/") && y.ends_with(".html") => match ACILIS.iter().find(|(a, _)| *a == &y[8..y.len() - 5]) {
            Some((_, m)) => yanit(&mut s, 200, "text/html; charset=utf-8", m.as_bytes()),
            None => yanit(&mut s, 404, "text/plain; charset=utf-8", b"yok"),
        },
        ("GET", "/logo.svg") => yanit(&mut s, 200, "image/svg+xml", LOGO.as_bytes()),
        ("GET", "/yz/dar.ttf") => yanit(&mut s, 200, "font/ttf", YZ_DAR),
        ("GET", "/yz/raf.ttf") => yanit(&mut s, 200, "font/ttf", YZ_RAF),
        ("GET", "/bilgi") => {
            crate::animasyon::yukle();
            let ff = crate::ffmpeg::bul().map(|p| p.display().to_string()).unwrap_or_default();
            let adlar: Vec<String> = ORNEKLER.iter().map(|(a, _)| format!("\"{a}\"")).collect();
            let liste = |l: Vec<(String, String, bool)>| l.iter().map(|(a, b, o)| format!("{{\"ad\":\"{}\",\"acik\":\"{}\",\"ozel\":{o}}}", json_kacis(a), json_kacis(b))).collect::<Vec<_>>().join(",");
            let efekt: Vec<String> = crate::senaryo::efekt_listesi().iter().map(|(a, b)| format!("{{\"ad\":\"{a}\",\"acik\":\"{b}\",\"ozel\":false}}")).collect();
            let kl: Vec<String> = crate::animasyon::klasorler().iter().map(|k| format!("\"{}\"", json_kacis(&k.display().to_string()))).collect();
            let paletler: Vec<String> = crate::stil::PALETLER.iter().map(|p| { let q = crate::stil::paletli(p.0).unwrap(); format!("{{\"ad\":\"{}\",\"acik\":\"{}\",\"yuz\":[{},{},{}],\"zemin\":[{},{},{}],\"vurgu\":[{},{},{}]}}", p.0, json_kacis(p.1), q.yuz.0, q.yuz.1, q.yuz.2, q.zemin.0, q.zemin.1, q.zemin.2, q.vurgu.0, q.vurgu.1, q.vurgu.2) }).collect();
            let fontlar: Vec<String> = crate::stil::FONTLAR.iter().map(|f| format!("{{\"ad\":\"{}\",\"acik\":\"{}\"}}", f.0, json_kacis(f.1))).collect();
            let gecisler: Vec<String> = crate::senaryo::GECISLER.iter().map(|g| format!("{{\"ad\":\"{}\",\"acik\":\"{}\"}}", g.0, json_kacis(g.1))).collect();
            let stiller: Vec<String> = crate::oto::STILLER.iter().map(|g| format!("{{\"ad\":\"{}\",\"acik\":\"{}\"}}", g.0, json_kacis(g.1))).collect();
            let sablonlar: Vec<String> = crate::sablon::SABLONLAR.iter().map(|(a, m)| format!("{{\"ad\":\"{}\",\"baslik\":\"{}\",\"kategori\":\"{}\",\"varsayilan\":\"{}\"}}", a, json_kacis(&crate::sablon::baslik(m, "baslik")), json_kacis(&crate::sablon::baslik(m, "kategori")), json_kacis(&crate::sablon::baslik(m, "varsayilan")))).collect();
            let mut sablonlar = sablonlar;
            for (a, m) in kullanici_sablonlari() { sablonlar.push(format!("{{\"ad\":\"{}\",\"baslik\":\"{}\",\"kategori\":\"Benim\",\"varsayilan\":\"\"}}", json_kacis(&a), json_kacis(&crate::sablon::baslik(&m, "baslik")))); }
            let rehber: Vec<String> = crate::sablon::REHBER.iter().map(|(a, m)| format!("{{\"ad\":\"{}\",\"baslik\":\"{}\",\"aciklama\":\"{}\"}}", a, json_kacis(&crate::sablon::baslik(m, "baslik")), json_kacis(&crate::sablon::baslik(m, "aciklama")))).collect();
            let egriler: Vec<String> = crate::anim::EGRILER.iter().map(|g| format!("{{\"ad\":\"{}\",\"acik\":\"{}\"}}", g.0, json_kacis(g.1))).collect();
            let j = format!("{{\"egriler\":[{}],\"paletler\":[{}],\"fontlar\":[{}],\"gecisler\":[{}],\"stiller\":[{}],\"surum\":\"{SURUM}\",\"ffmpeg\":\"{}\",\"klasor\":\"{}\",\"ornekler\":[{}],\"girisler\":[{}],\"cikislar\":[{}],\"efektler\":[{}],\"animasyon_klasorleri\":[{}],\"sablonlar\":[{}],\"rehber\":[{}]}}", egriler.join(","), paletler.join(","), fontlar.join(","), gecisler.join(","), stiller.join(","), json_kacis(&ff), json_kacis(&cikti_klasoru().display().to_string()), adlar.join(","), liste(crate::senaryo::giris_listesi()), liste(crate::senaryo::cikis_listesi()), efekt.join(","), kl.join(","), sablonlar.join(","), rehber.join(","));
            yanit(&mut s, 200, "application/json; charset=utf-8", j.as_bytes())
        }
        ("GET", y) if y.starts_with("/ornek/") => match ORNEKLER.iter().find(|(a, _)| *a == &y[7..]) {
            Some((_, m)) => yanit(&mut s, 200, metin, m.as_bytes()),
            None => yanit(&mut s, 404, metin, b"yok"),
        },
        ("GET", y) if y.starts_with("/sablon/") => match crate::sablon::sablon_bul(&y[8..]).map(|m| m.to_string()).or_else(|| kullanici_sablon_bul(&url_coz(&y[8..]))) {
            Some(m) => yanit(&mut s, 200, metin, m.as_bytes()),
            None => yanit(&mut s, 404, metin, b"yok"),
        },
        ("POST", "/sablon-kaydet") => {
            let ad = guvenli_ad(&sorgu(&yol, "ad").unwrap_or_default());
            if ad.is_empty() || yazi.trim().is_empty() { yanit(&mut s, 400, metin, "Şablon adı ve içerik gerekli".as_bytes()); }
            else {
                let icerik = format!("# baslik: {ad}\n# kategori: Benim\n{}\n", yazi.lines().filter(|l| !l.trim_start().starts_with("# baslik:") && !l.trim_start().starts_with("# kategori:")).collect::<Vec<_>>().join("\n"));
                match std::fs::write(sablon_klasoru().join(format!("{ad}.txt")), icerik) { Ok(_) => yanit(&mut s, 200, metin, format!("u-{ad}").as_bytes()), Err(e) => yanit(&mut s, 500, metin, e.to_string().as_bytes()) }
            }
        }
        ("GET", y) if y.starts_with("/rehber/") => match crate::sablon::rehber_bul(&y[8..]) {
            Some(m) => yanit(&mut s, 200, metin, m.as_bytes()),
            None => yanit(&mut s, 404, metin, b"yok"),
        },
        ("GET", "/durum") => {
            let d = o.durum.lock().unwrap();
            let j = format!("{{\"calisiyor\":{},\"kare\":{},\"toplam\":{},\"dosya\":\"{}\",\"mesaj\":\"{}\",\"hata\":{}}}", d.calisiyor, d.kare, d.toplam, json_kacis(&d.dosya), json_kacis(&d.mesaj), d.hata);
            yanit(&mut s, 200, "application/json; charset=utf-8", j.as_bytes())
        }
        ("POST", "/nabiz") => { *o.son_nabiz.lock().unwrap() = Instant::now(); yanit(&mut s, 200, metin, b"ok") }
        ("POST", "/onizle") => {
            let t: f32 = sorgu(&yol, "t").and_then(|v| v.parse().ok()).unwrap_or(0.0);
            let _k = o.onizleme.lock().unwrap();
            let sonuc = std::panic::catch_unwind(|| uret::onizleme(&yazi, t)).unwrap_or_else(|_| Err("önizleme üretilemedi (senaryoyu kontrol edin)".into()));
            match sonuc {
                Ok((rgb, w, h, toplam)) => {
                    let p = png::rgb_png(&rgb, w, h);
                    let b = format!("HTTP/1.1 200 OK\r\nContent-Type: image/png\r\nContent-Length: {}\r\nX-Toplam: {:.2}\r\nCache-Control: no-store\r\nConnection: close\r\n\r\n", p.len(), toplam);
                    let _ = s.write_all(b.as_bytes()); let _ = s.write_all(&p);
                }
                Err(e) => yanit(&mut s, 400, metin, e.as_bytes()),
            }
        }
        ("POST", "/uret") => {
            let kodek = sorgu(&yol, "kodek").unwrap_or_else(|| "mp4".into());
            if kodek != "mp4" && kodek != "hap" { return yanit(&mut s, 400, metin, b"kodek mp4 veya hap olmali"); }
            if crate::ffmpeg::bul().is_none() { return yanit(&mut s, 400, metin, crate::ffmpeg::YOK_MESAJI.as_bytes()); }
            if let Err(e) = uret::senaryo_oku(&yazi) { return yanit(&mut s, 400, metin, e.as_bytes()); }
            let dosya = cikti_klasoru().join(dosya_adi(&sorgu(&yol, "dosya").unwrap_or_default(), &kodek));
            {
                let mut d = o.durum.lock().unwrap();
                if d.calisiyor { return yanit(&mut s, 409, metin, "zaten bir video üretiliyor".as_bytes()); }
                *d = Durum { calisiyor: true, kare: 0, toplam: 0, dosya: dosya.display().to_string(), mesaj: String::new(), hata: false };
            }
            o.iptal.store(false, Ordering::Relaxed);
            let o2 = o.clone();
            let yol_s = dosya.display().to_string();
            std::thread::spawn(move || {
                let o3 = o2.clone();
                let sonuc = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| uret::video(&yazi, &yol_s, &kodek, &move |k, n| { let mut d = o3.durum.lock().unwrap(); d.kare = k; d.toplam = n; }, &o2.iptal)))
                    .unwrap_or_else(|_| Err("video üretimi beklenmedik şekilde durdu".into()));
                let mut d = o2.durum.lock().unwrap();
                d.calisiyor = false;
                match sonuc { Ok(m) => d.mesaj = m, Err(e) => { d.hata = true; d.mesaj = e; } }
            });
            yanit(&mut s, 200, metin, "basladi".as_bytes())
        }
        ("POST", "/oto") => {
            let stil = sorgu(&yol, "stil").unwrap_or_else(|| "yumusak".into());
            let tohum: u32 = sorgu(&yol, "tohum").and_then(|v| v.parse().ok()).unwrap_or(1);
            let (w, h) = (sorgu(&yol, "w").and_then(|v| v.parse().ok()).unwrap_or(950u32), sorgu(&yol, "h").and_then(|v| v.parse().ok()).unwrap_or(1800u32));
            let fiz = if sorgu(&yol, "fiziksel").as_deref() == Some("0") { None } else { Some((8.0, 25.0)) };
            let m = crate::oto::uret(&yazi, &stil, tohum, (w, h), fiz);
            crate::animasyon::yukle();
            yanit(&mut s, 200, metin, m.as_bytes())
        }
        ("POST", "/denetle") => match std::panic::catch_unwind(|| uret::analiz(&yazi)).unwrap_or_else(|_| Err("denetlenemedi".into())) {
            Ok(v) => { let j = v.iter().map(|(a, b)| format!("{{\"seviye\":\"{a}\",\"mesaj\":\"{}\"}}", json_kacis(b))).collect::<Vec<_>>().join(","); yanit(&mut s, 200, "application/json; charset=utf-8", format!("[{j}]").as_bytes()) }
            Err(e) => yanit(&mut s, 400, metin, e.as_bytes()),
        },
        ("POST", "/klip") => {
            let _k = o.klip.lock().unwrap();
            let iptal = AtomicBool::new(false);
            match std::panic::catch_unwind(|| uret::klip(&yazi, &iptal)).unwrap_or_else(|_| Err("klip üretilemedi".into())) {
                Ok(b) => yanit(&mut s, 200, "video/mp4", &b),
                Err(e) => yanit(&mut s, 400, metin, e.as_bytes()),
            }
        }
        ("POST", "/iptal") => { o.iptal.store(true, Ordering::Relaxed); yanit(&mut s, 200, metin, b"ok") }
        // Son üretilen videoyu varsayılan video oynatıcıda açar (tarayıcıda değil). Yol yalnız sunucunun kendi kaydından ve çıktı klasöründen gelir.
        ("POST", "/oynat-son") => {
            let dosya = o.durum.lock().unwrap().dosya.clone();
            match son_video_yolu(&dosya) {
                Some(p) => { son_video_ac(&p); yanit(&mut s, 200, metin, b"ok") }
                None => yanit(&mut s, 404, metin, b"video yok"),
            }
        }
        ("POST", "/klasor") => { klasor_ac(&cikti_klasoru()); yanit(&mut s, 200, metin, b"ok") }
        ("POST", "/kapat") => { yanit(&mut s, 200, metin, b"ok"); std::process::exit(0) }
        // "Tarif et": istem üretme, kural motoru, yanıt doğrulama (hepsi yalnız metin işler; dış adres yalnız listedeki adla açılır)
        (m, y) if y.starts_with("/tarif/") => { let (kod, tur, g) = crate::tarif_http::isle(m, &y[7..], &yazi, &cikti_klasoru()); yanit(&mut s, kod, tur, g.as_bytes()) }
        _ => yanit(&mut s, 404, metin, b"yok"),
    }
}

/// Sunucuyu başlatır ve (isteğe bağlı) tarayıcıyı açar. `port` 0 ise boş bir port seçilir. Dönmez.
/// pencere_ac=false: yalnız hizmet (test/geliştirme); true: gömülü WebView2 penceresi. Varsayılan tarayıcı HİÇ açılmaz.
pub fn calistir(pencere_ac: bool, port: u16) -> ! {
    // Tek örnek: yalnız 127.0.0.1'deki sabit kilit portu; doluysa başka örnek çalışıyordur.
    let _kilit = if pencere_ac { match TcpListener::bind(("127.0.0.1", 47893)) { Ok(k) => Some(k), Err(_) => { hata_goster("TAA Vidyo Yapıcı zaten açık."); std::process::exit(0) } } } else { None };
    // Port dolu/çakışıyorsa en çok 5 kez yeniden dener; sonra sistemin verdiği boş portla (0) açar. Yalnız 127.0.0.1.
    let dinle = { let mut son = None; if port == 0 { for pt in 47894u16..=47913 { if let Ok(d) = TcpListener::bind(("127.0.0.1", pt)) { son = Some(d); break } } } /* sabit tercih: localStorage ve çerezler çoğu zaman korunur */ let mut p = port; for _ in 0..(if son.is_some() { 0 } else { 6 }) { match TcpListener::bind(("127.0.0.1", p)) { Ok(d) => { son = Some(d); break } Err(e) => { eprintln!("uyarı: port {p} açılamadı ({e}), yeniden deneniyor"); p = 0; std::thread::sleep(Duration::from_millis(150)); } } } match son { Some(d) => d, None => { eprintln!("HATA: arayüz portu açılamadı"); std::process::exit(1) } } };
    let port = dinle.local_addr().unwrap().port();
    let jeton = rastgele_jeton();
    let adres = format!("http://127.0.0.1:{port}/?t={jeton}");
    println!("TAA Vidyo Yapıcı arayüzü: {adres}");
    let o = Arc::new(Ortak { durum: Mutex::new(Durum::default()), iptal: AtomicBool::new(false), onizleme: Mutex::new(()), klip: Mutex::new(()), son_nabiz: Mutex::new(Instant::now()), port, jeton });
    let o_hizmet = o.clone();
    std::thread::spawn(move || {
        for ak in dinle.incoming().flatten() { let o = o_hizmet.clone(); std::thread::spawn(move || isle(ak, o)); }
    });
    if pencere_ac {
        #[cfg(windows)]
        { pencere(&adres); }
        #[cfg(not(windows))]
        { eprintln!("Gömülü pencere yalnız Windows'ta; --tarayici-yok ile hizmet çalışıyor: {adres}"); }
    } else {
        // Pencere yoksa (test): sekme/istemci kapanıp 90 sn nabız gelmezse ve iş yoksa kapan
        let o2 = o.clone();
        std::thread::spawn(move || loop {
            std::thread::sleep(Duration::from_secs(5));
            let bos = !o2.durum.lock().unwrap().calisiyor;
            if bos && o2.son_nabiz.lock().unwrap().elapsed() > Duration::from_secs(90) { std::process::exit(0); }
        });
    }
    loop { std::thread::sleep(Duration::from_secs(3600)); }
}

/// Windows'ta kullanıcıya hata göstermek için (konsol gizli olduğundan): küçük PowerShell iletişim kutusu.
#[cfg(windows)]
fn hata_goster(metin: &str) {
    use std::os::windows::process::CommandExt;
    // WScript.Shell.Popup: 8 sn sonra kendiliğinden kapanır (gözetimsiz çalışmada takılmaz)
    let kod = format!("(New-Object -ComObject WScript.Shell).Popup('{}', 8, 'TAA Vidyo Yapıcı', 48) | Out-Null", metin.replace('\'', ""));
    let _ = std::process::Command::new("powershell").args(["-NoProfile", "-WindowStyle", "Hidden", "-Command", &kod]).creation_flags(0x0800_0000).status();
}
#[cfg(not(windows))]
fn hata_goster(metin: &str) { eprintln!("{metin}"); }

/// Gömülü pencere (tao + wry/WebView2). Pencere kapanınca süreç ve hizmet ANINDA sonlanır. WebView2 yoksa açık bir hata iletişim kutusu gösterilir.
#[cfg(windows)]
fn pencere(adres: &str) {
    use tao::event::{Event, WindowEvent};
    use tao::event_loop::{ControlFlow, EventLoopBuilder};
    use tao::window::WindowBuilder;
    use wry::WebViewBuilder;
    let el = EventLoopBuilder::new().build();
    let pencere = match WindowBuilder::new().with_title("TAA Vidyo Yapıcı").with_inner_size(tao::dpi::LogicalSize::new(1280.0, 860.0)).with_min_inner_size(tao::dpi::LogicalSize::new(900.0, 600.0)).build(&el) {
        Ok(p) => p,
        Err(e) => { hata_goster(&format!("Pencere açılamadı: {e}")); std::process::exit(2) }
    };
    // WebView2 profil/çerez/önbellek verisi kurulum klasörüne yazılmasın: kullanıcının yerel uygulama verisi klasörüne gider
    let veri = std::env::var("LOCALAPPDATA").map(|p| std::path::PathBuf::from(p).join("TAA Vidyo Yapici").join("WebView2")).unwrap_or_else(|_| std::env::temp_dir().join("TAA-Vidyo-Yapici-WebView2"));
    let mut baglam = wry::WebContext::new(Some(veri));
    let _gorunum = match WebViewBuilder::new_with_web_context(&mut baglam).with_url(adres).build(&pencere) {
        Ok(g) => g,
        Err(e) => { hata_goster(&format!("WebView2 çalışma zamanı bulunamadı veya başlatılamadı. Microsoft Edge WebView2 Runtime kurun (aka.ms/webview2). Ayrıntı: {e}")); std::process::exit(3) }
    };
    el.run(move |olay, _, akis| {
        *akis = ControlFlow::Wait;
        if let Event::WindowEvent { event: WindowEvent::CloseRequested, .. } = olay { std::process::exit(0); }
    });
}

#[cfg(test)]
mod t {
    use super::*;
    #[test]
    fn jeton_rastgele_ve_dogrulanir() {
        let a = rastgele_jeton(); let b = rastgele_jeton();
        assert_eq!(a.len(), 64); assert!(a.chars().all(|c| c.is_ascii_hexdigit())); assert_ne!(a, b);
        assert!(cerez_jeton_var(&format!("x=1; taa-oturum={a}"), &a));
        assert!(!cerez_jeton_var(&format!("taa-oturum={b}"), &a));
        assert!(!cerez_jeton_var("", &a)); assert!(!cerez_jeton_var("taa-oturum=", &a));
    }
    #[test]
    fn bina_onizleme_varsayilanlari_kilitli() {
        // Yusuf'un bildirdiği bina ölçüleri (11-15/16): değişiklik bilinçli olmalı, bu test onu yakalar.
        assert!(SAYFA.contains("const BINA_VARSAYILAN = {gen: 31, yuk: 35, ew: 10, eh: 30, ealt: 3, logo: 6, logoy: 18, orta: 8, kat: 5.5, yuz: 'alt'};"));
        assert!(SAYFA.contains("taa-bina4") && SAYFA.contains("taa-gorunum"));
        assert!(SAYFA.contains("data-g=\"karsi\"") && SAYFA.contains("id=\"isikSeg\"") && !SAYFA.contains("data-g=\"ust\"") && !SAYFA.contains("bina-foto") && SAYFA.contains("id=\"binaFig\""));
        assert!(SAYFA.contains("UYARI: ekran üstü"));
    }
    #[test]
    fn acilislar_gomulu_ve_sayfa_listesiyle_uyumlu() {
        // index.html'deki ACILISLAR listesi ile sunucudaki tablo aynı adları içermeli; her açılış logo.js'i kullanmalı, uygulama modunda Tekrar oynat gizlenmeli.
        for (ad, m) in ACILIS {
            assert!(SAYFA.contains(&format!("'{ad}'")), "{ad} sayfa listesinde yok");
            assert!(m.contains("src=\"logo.js\"") && m.contains("data-ikiz"), "{ad}: logo.js/İkiz Kabartma yok");
            assert!(m.contains("uygulama=1") && m.contains("taa-acilis-gec"), "{ad}: uygulama modu yok");
            assert!(m.contains("Yaz, önizle, tek tıkla videoyu yap."), "{ad}: slogan yok");
        }
        assert!(ACILIS.len() >= 14);
        assert!(ACILIS_LOGO_JS.contains("#C9566F") && ACILIS_LOGO_JS.contains("#560D20"));
        assert!(SAYFA.contains("id=\"ikiz-kabartma\"") && SAYFA.contains("taa-acilis-torba") && SAYFA.contains("taa-acilis-ayar") && !SAYFA.contains("Tekrar oynat"));
    }
    #[test]
    fn acilis_torbasi_node_testi() {
        // Karıştırılmış torba mantığı (web/index.html TORBA bloğu) tests/torba_test.js ile denenir; node yoksa atlanır.
        assert!(SAYFA.contains("// TORBA-BASLA") && SAYFA.contains("// TORBA-BITIS") && SAYFA.contains("KALICI-AYAR-BASLA") && SAYFA.contains("'/ayar'") && SAYFA.contains("id=\"acilisTam\"") && SAYFA.contains("id=\"acilisListe\""));
        let betik = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/torba_test.js");
        match std::process::Command::new("node").arg(betik).output() {
            Ok(o) => assert!(o.status.success(), "torba_test.js başarısız: {}{}", String::from_utf8_lossy(&o.stdout), String::from_utf8_lossy(&o.stderr)),
            Err(_) => eprintln!("node bulunamadı, torba testi atlandı"),
        }
    }
    #[test]
    fn son_video_yolu_yalniz_cikti_klasorunden() {
        assert!(son_video_yolu("").is_none());
        assert!(son_video_yolu("C:\\Windows\\system32\\cmd.exe").is_none());
        assert!(son_video_yolu("olmayan.mp4").is_none());
        assert!(son_video_yolu("..\\..\\kötü.mp4").is_none());
    }
    #[test]
    fn dosya_adi_guvenli() {
        assert_eq!(dosya_adi("..\\..\\kötü/ad", "mp4"), "kötü_ad.mp4");
        assert_eq!(dosya_adi("", "hap"), "video.mov");
        assert_eq!(dosya_adi("Kampanya 1", "mp4"), "Kampanya 1.mp4");
    }
    #[test]
    fn url_cozme() { assert_eq!(url_coz("a%20b+c%C3%BC"), "a b cü"); assert_eq!(url_coz("%"), "%"); assert_eq!(url_coz("%4"), "%4"); }
    #[test]
    fn ornekler_ayristirilir() { for (a, m) in ORNEKLER { crate::senaryo::ayristir(m).unwrap_or_else(|e| panic!("{a}: {e}")); } }
    #[test]
    fn sorgu_okur() { assert_eq!(sorgu("/x?t=2.5&dosya=a%20b", "dosya").as_deref(), Some("a b")); }
}
