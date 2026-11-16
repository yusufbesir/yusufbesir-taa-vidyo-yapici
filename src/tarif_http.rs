//! "Tarif et" HTTP uçları (yalnız 127.0.0.1, jetonlu oturum içinde çalışır; arayuz.rs yönlendirir).
//! Sahip: Yusuf; yazan [Claude#ebd691, 2026-11-16]. Dış sitelere yalnız ADI verilen listeden gidilir; istekten gelen adres açılmaz.
use crate::tarif::*;
use std::path::Path;

pub type Yanit = (u16, &'static str, String);
const JSON: &str = "application/json; charset=utf-8";
const METIN: &str = "text/plain; charset=utf-8";

fn kac(s: &str) -> String {
    let mut o = String::new();
    for c in s.chars() { match c { '"' => o.push_str("\\\""), '\\' => o.push_str("\\\\"), '\n' => o.push_str("\\n"), '\r' | '\t' => o.push(' '), c if (c as u32) < 32 => {}, c => o.push(c) } }
    o
}
fn dizi(v: &[String]) -> String { format!("[{}]", v.iter().map(|x| format!("\"{}\"", kac(x))).collect::<Vec<_>>().join(",")) }

/// LLM sitelerinin varsayılan listesi (ad, adres). Kullanıcı kendi sitelerini ekleyebilir.
pub const VARSAYILAN_SITELER: [(&str, &str); 4] = [("ChatGPT", "https://chatgpt.com/"), ("Gemini", "https://gemini.google.com/app"), ("DeepSeek", "https://chat.deepseek.com/"), ("Claude", "https://claude.ai/new")];

/// Yalnız https, boşluksuz, kullanıcı-parola içermeyen, noktalı ana makineli adresler kabul edilir.
pub fn adres_gecerli(a: &str) -> bool {
    let Some(r) = a.strip_prefix("https://") else { return false };
    if a.len() > 200 || r.is_empty() { return false; }
    if !a.chars().all(|c| c.is_ascii_alphanumeric() || "._~:/?#[]@!$&'()*+,;=%-".contains(c)) { return false; }
    let host = r.split(['/', '?', '#']).next().unwrap_or("");
    let ip = host.split(':').next().unwrap_or("").chars().all(|c| c.is_ascii_digit() || c == '.');
    let son = host.split(':').next().unwrap_or("").rsplit('.').next().unwrap_or("");
    let ip = ip || !son.chars().any(|c| c.is_ascii_alphabetic()) || host.to_lowercase().starts_with("0x");
    !ip && !host.contains('@') && !host.contains('[') && host.contains('.') && !host.starts_with('.') && !host.ends_with('.')
}
fn site_dosyasi(k: &Path) -> std::path::PathBuf { k.join("tarif-siteleri.json") }
fn kural_dosyasi(k: &Path) -> std::path::PathBuf { k.join("tarif-kurallarim.json") }

pub fn siteler(k: &Path) -> Vec<(String, String)> {
    let mut v: Vec<(String, String)> = VARSAYILAN_SITELER.iter().map(|(a, b)| (a.to_string(), b.to_string())).collect();
    if let Ok(j) = std::fs::read_to_string(site_dosyasi(k)) {
        if let Ok(kok) = json_coz(&j) {
            if let Some(Json::Dizi(l)) = kok.al("siteler") {
                for s in l {
                    let (Some(a), Some(u)) = (s.al("ad").and_then(|x| x.yazi()), s.al("adres").and_then(|x| x.yazi())) else { continue };
                    if adres_gecerli(u) && !a.is_empty() && a.chars().count() <= 20 && !v.iter().any(|x| x.0.to_lowercase() == a.to_lowercase()) { v.push((a.to_string(), u.to_string())); }
                }
            }
        }
    }
    v
}
fn site_ekle(k: &Path, ad: &str, adres: &str) -> Result<(), String> {
    let ad = ad.split_whitespace().collect::<Vec<_>>().join(" ");
    if ad.is_empty() || ad.chars().count() > 20 || !ad.chars().all(|c| c.is_alphanumeric() || c == ' ' || c == '-' || c == '.') { return Err("Ad en çok 20 harf, rakam, boşluk ya da tire olmalı.".into()); }
    if !adres_gecerli(adres) { return Err("Adres https:// ile başlamalı, boşluk içermemeli ve geçerli bir site olmalı.".into()); }
    let mut l: Vec<(String, String)> = siteler(k).into_iter().filter(|x| !VARSAYILAN_SITELER.iter().any(|d| d.0 == x.0)).collect();
    if l.iter().any(|x| x.0.to_lowercase() == ad.to_lowercase()) || VARSAYILAN_SITELER.iter().any(|d| d.0.to_lowercase() == ad.to_lowercase()) { return Err("Bu ad zaten var.".into()); }
    if l.len() >= 12 { return Err("En çok 12 özel site eklenebilir.".into()); }
    l.push((ad, adres.to_string()));
    let g = l.iter().map(|(a, u)| format!("{{\"ad\":\"{}\",\"adres\":\"{}\"}}", kac(a), kac(u))).collect::<Vec<_>>().join(",");
    std::fs::create_dir_all(k).map_err(|e| e.to_string())?;
    std::fs::write(site_dosyasi(k), format!("{{\"aciklama\":\"TAA Vidyo Yapıcı 'Tarif et' için eklenen yapay zekâ siteleri. Sahip: kullanıcı. Yalnız https adresleri okunur.\",\"siteler\":[{g}]}}\n")).map_err(|e| e.to_string())
}
#[cfg(windows)]
fn adres_ac(a: &str) { use std::os::windows::process::CommandExt; let _ = std::process::Command::new("explorer").arg(a).creation_flags(0x0800_0000).spawn(); }
#[cfg(not(windows))]
fn adres_ac(a: &str) { let _ = std::process::Command::new("xdg-open").arg(a).spawn(); }

pub fn kullanici_kurallari(k: &Path) -> Vec<Kural> { std::fs::read_to_string(kural_dosyasi(k)).ok().and_then(|j| kurallari_oku(&j).ok()).unwrap_or_default() }

/// Çözüm sonucunu arayüzün okuyacağı JSON'a çevirir.
fn sonuc_json(tarif: Option<&Tarif>, notlar: &[String], anladim: &[String], anlasilmayan: &[String], hatalar: &[String], kat: &Katalog) -> String {
    match tarif {
        Some(t) => {
            let sahneler: Vec<String> = t.sahneler.iter().map(|s| format!("{{\"metin\":\"{}\",\"giris\":\"{}\",\"cikis\":\"{}\",\"efekt\":\"{}\",\"palet\":\"{}\",\"font\":\"{}\",\"sure\":{:.1}}}", kac(&s.metin.join(" / ")), kac(&s.giris), kac(&s.cikis), kac(&s.efekt.join(", ")), kac(&s.palet), kac(&s.font), s.sure)).collect();
            format!("{{\"ok\":true,\"senaryo\":\"{}\",\"gorunum\":\"{}\",\"sure\":{:.1},\"acilis\":\"{}\",\"kapanis\":\"{}\",\"sahneler\":[{}],\"notlar\":{},\"anladim\":{},\"anlasilmayan\":{}}}",
                kac(&senaryo_metni(t)), kac(&t.gorunum), toplam_sure(t), kac(&t.acilis), kac(&t.kapanis), sahneler.join(","), dizi(notlar), dizi(anladim), dizi(anlasilmayan))
        }
        None => format!("{{\"ok\":false,\"hatalar\":{},\"duzelt\":\"{}\",\"anladim\":{},\"anlasilmayan\":{}}}", dizi(hatalar), kac(&duzelt_istemi(hatalar, kat)), dizi(anladim), dizi(anlasilmayan)),
    }
}

/// `ad`: "/tarif/" sonrası yol. `yazi`: istek gövdesi. `klasor`: kullanıcı verisi klasörü (kurallar, siteler).
pub fn isle(yontem: &str, ad: &str, yazi: &str, klasor: &Path) -> Yanit {
    let kat = Katalog::gercek();
    match (yontem, ad) {
        ("POST", "istem") => (200, METIN, istem_uret(yazi, &kat)),
        ("POST", "kural-coz") => {
            let z = kural_coz(yazi, &kat, &kullanici_kurallari(klasor));
            let hatalar = if z.tarif.is_none() { z.notlar.clone() } else { vec![] };
            (200, JSON, sonuc_json(z.tarif.as_ref(), &z.notlar, &z.anladim, &z.anlasilmayan, &hatalar, &kat))
        }
        ("POST", "uygula") => match dogrula(yazi, &kat) {
            Ok(d) => (200, JSON, sonuc_json(Some(&d.tarif), &d.notlar, &[], &[], &[], &kat)),
            Err(h) => (200, JSON, sonuc_json(None, &[], &[], &[], &h, &kat)),
        },
        ("GET", "siteler") => (200, JSON, format!("[{}]", siteler(klasor).iter().map(|(a, _)| format!("\"{}\"", kac(a))).collect::<Vec<_>>().join(","))),
        ("POST", "ac") => match siteler(klasor).into_iter().find(|(a, _)| a == yazi.trim()) {
            Some((_, u)) => { adres_ac(&u); (200, METIN, "ok".into()) }
            None => (404, METIN, "Bu adda bir site listede yok.".into()),
        },
        ("POST", "site-ekle") => {
            let (Ok(j), true) = (json_coz(yazi), yazi.len() < 2000) else { return (400, METIN, "Geçersiz istek.".into()) };
            match site_ekle(klasor, j.al("ad").and_then(|x| x.yazi()).unwrap_or(""), j.al("adres").and_then(|x| x.yazi()).unwrap_or("")) { Ok(()) => (200, METIN, "ok".into()), Err(e) => (400, METIN, e) }
        }
        ("GET", "kurallar") => {
            let mut v: Vec<String> = vec![];
            let onceki: Vec<Kural> = gomulu_kurallar().iter().cloned().chain(kullanici_kurallari(klasor)).collect();
            for (kay, l) in [("gömülü", gomulu_kurallar().clone()), ("benim", kullanici_kurallari(klasor)), ("otomatik", otomatik_kurallar(&kat, &onceki))] {
                for r in l { v.push(format!("{{\"kaynak\":\"{kay}\",\"k\":\"{}\",\"h\":\"{}\",\"i\":{}}}", kac(&r.kategori), kac(&r.hedef), dizi(&r.ifadeler))); }
            }
            (200, JSON, format!("[{}]", v.join(",")))
        }
        ("POST", "kural-ekle") => {
            let (Ok(j), true) = (json_coz(yazi), yazi.len() < 2000) else { return (400, METIN, "Geçersiz istek.".into()) };
            let (k, i, h) = (j.al("k").and_then(|x| x.yazi()).unwrap_or(""), j.al("i").and_then(|x| x.yazi()).unwrap_or(""), j.al("h").and_then(|x| x.yazi()).unwrap_or(""));
            let mevcut = std::fs::read_to_string(kural_dosyasi(klasor)).ok();
            match kural_ekle(mevcut.as_deref(), k, i, h, &kat) {
                Ok(yeni) => match std::fs::create_dir_all(klasor).and_then(|_| std::fs::write(kural_dosyasi(klasor), yeni)) { Ok(()) => (200, METIN, "ok".into()), Err(e) => (500, METIN, format!("Kural yazılamadı: {e}")) },
                Err(e) => (400, METIN, e),
            }
        }
        ("GET", "hazir") => (200, JSON, HAZIR_SENARYOLAR.to_string()),
        ("GET", "llm-durum") => {
            let a = crate::llm::ayar_oku(klasor);
            let sg: Vec<String> = crate::llm::SAGLAYICILAR.iter().map(|(i, ad, m)| format!("{{\"id\":\"{i}\",\"ad\":\"{}\",\"model\":\"{m}\"}}", kac(ad))).collect();
            (200, JSON, format!("{{\"acik\":{},\"saglayici\":\"{}\",\"model\":\"{}\",\"adres\":\"{}\",\"sorma\":{},\"anahtar_var\":{},\"saglayicilar\":[{}]}}", a.acik, kac(&a.saglayici), kac(&a.model), kac(&a.adres), a.sorma, crate::llm::anahtar_var(klasor, &a), sg.join(",")))
        }
        ("POST", "llm-ayar") => {
            let (Ok(j), true) = (json_coz(yazi), yazi.len() < 4000) else { return (400, METIN, "Geçersiz istek.".into()) };
            let mut a = crate::llm::ayar_oku(klasor);
            if let Some(Json::Mantik(b)) = j.al("acik") { a.acik = *b; }
            if let Some(Json::Mantik(b)) = j.al("sorma") { a.sorma = *b; }
            if let Some(x) = j.al("saglayici").and_then(|x| x.yazi()) { a.saglayici = x.to_string(); }
            if let Some(x) = j.al("model").and_then(|x| x.yazi()) { a.model = x.trim().to_string(); }
            if let Some(x) = j.al("adres").and_then(|x| x.yazi()) { a.adres = x.trim().to_string(); }
            if let Some(Json::Mantik(true)) = j.al("anahtar_kaldir") { if let Err(e) = crate::llm::anahtar_kaldir(klasor) { return (500, METIN, e); } a.acik = false; }
            if let Some(k) = j.al("anahtar").and_then(|x| x.yazi()) { if !k.trim().is_empty() { if let Err(e) = crate::llm::anahtar_kaydet(klasor, k, &crate::llm::bag(&a)) { return (400, METIN, e); } } }
            if a.acik && !crate::llm::anahtar_var(klasor, &a) { return (400, METIN, "Açmak için bu hizmet/adres için anahtarı girin (hizmet ya da adres değişince anahtar yeniden girilir).".into()); }
            match crate::llm::ayar_yaz(klasor, &a) { Ok(()) => (200, METIN, "ok".into()), Err(e) => (400, METIN, e) }
        }
        ("POST", "llm-gonder") => {
            let (Ok(j), true) = (json_coz(yazi), yazi.len() < MAX_TARIF * 4 + 200) else { return (400, METIN, "Geçersiz istek.".into()) };
            if !matches!(j.al("onay"), Some(Json::Mantik(true))) { return (400, METIN, "Gönderilecek metin onaylanmadı.".into()); }
            let metin = j.al("metin").and_then(|x| x.yazi()).unwrap_or("");
            if metin.trim().is_empty() { return (400, METIN, "Tarif boş.".into()); }
            match crate::llm::gonder(klasor, metin, &kat) {
                Ok(m) => match dogrula(&m, &kat) {
                    Ok(d) => (200, JSON, sonuc_json(Some(&d.tarif), &d.notlar, &[], &[], &[], &kat)),
                    Err(h) => (200, JSON, sonuc_json(None, &[], &[], &[], &h, &kat)),
                },
                Err(e) => (200, JSON, format!("{{\"ok\":false,\"baglanti\":true,\"hatalar\":{},\"duzelt\":\"\",\"anladim\":[],\"anlasilmayan\":[]}}", dizi(&[e]))),
            }
        }
        ("GET", "katalog") => {
            let ks = |k: &str, v: &[String]| format!("\"{k}\":{}", dizi(v));
            (200, JSON, format!("{{{},{},{},{},{},{}}}", ks("giris", &kat.giris), ks("cikis", &kat.cikis), ks("efekt", &kat.efekt), ks("palet", &kat.palet), ks("font", &kat.font), ks("gecis", &kat.gecis)))
        }
        _ => (404, METIN, "yok".into()),
    }
}

#[cfg(test)]
mod testler {
    use super::*;
    fn gecici(ad: &str) -> std::path::PathBuf { let k = std::env::temp_dir().join(format!("taa-tarif-test-{ad}-{}", std::process::id())); let _ = std::fs::create_dir_all(&k); k }
    #[test]
    fn adres_denetimi() {
        for iyi in ["https://chatgpt.com/", "https://gemini.google.com/app?x=1&y=2", "https://a.b/c#d"] { assert!(adres_gecerli(iyi), "{iyi}"); }
        for kotu in ["http://a.com/", "javascript:alert(1)", "file:///C:/x", "https://a.com/ b", "https://user:pw@a.com/", "https://a.com/\"x", "https://localhost/", "https://", "https://.com/"] { assert!(!adres_gecerli(kotu), "{kotu}"); }
        assert!(!adres_gecerli(&format!("https://a.com/{}", "x".repeat(300))));
    }
    #[test]
    fn site_ekleme_ve_acma_yalniz_listeden() {
        let k = gecici("site");
        assert_eq!(isle("POST", "ac", "https://kotu.example/", &k).0, 404, "adres doğrudan açılamaz, yalnız listedeki ad");
        assert_eq!(isle("POST", "ac", "kotu-ad", &k).0, 404);
        assert_eq!(isle("POST", "site-ekle", "{\"ad\":\"Ornek\",\"adres\":\"http://x.com/\"}", &k).0, 400);
        assert_eq!(isle("POST", "site-ekle", "{\"ad\":\"<b>\",\"adres\":\"https://x.com/\"}", &k).0, 400);
        assert_eq!(isle("POST", "site-ekle", "{\"ad\":\"ChatGPT\",\"adres\":\"https://x.com/\"}", &k).0, 400, "var olan ad tekrar eklenemez");
        assert_eq!(isle("POST", "site-ekle", "{\"ad\":\"Ornek\",\"adres\":\"https://ornek.example/sohbet\"}", &k).0, 200);
        assert!(isle("GET", "siteler", "", &k).2.contains("Ornek"));
    }
    #[test]
    fn uclar_json_dondurur() {
        let k = gecici("uc");
        let r = isle("POST", "kural-coz", "\"HOŞ GELDİNİZ\" zıpla, altın", &k);
        assert_eq!(r.0, 200);
        assert!(r.2.contains("\"ok\":true") && r.2.contains("sahne \\\"HOŞ|GELDİNİZ\\\"") && r.2.contains("zipla"), "{}", r.2);
        assert!(isle("POST", "kural-coz", "", &k).2.contains("\"ok\":false"));
        let u = isle("POST", "uygula", "```json\n{\"sahneler\":[{\"metin\":[\"A\"]}]}\n```", &k);
        assert!(u.2.contains("\"ok\":true"));
        let h = isle("POST", "uygula", "bozuk", &k);
        assert!(h.2.contains("\"ok\":false") && h.2.contains("duzelt"));
        assert!(isle("POST", "istem", "tarif", &k).2.contains("YALNIZ"));
        assert!(isle("GET", "kurallar", "", &k).2.contains("gömülü"));
        assert!(isle("GET", "katalog", "", &k).2.contains("\"giris\""));
        assert_eq!(isle("POST", "kural-ekle", "{\"k\":\"giris\",\"i\":\"hoplayarak\",\"h\":\"yok-boyle-ad\"}", &k).0, 400);
        assert_eq!(isle("POST", "kural-ekle", "{\"k\":\"giris\",\"i\":\"hoplayarak\",\"h\":\"zipla\"}", &k).0, 200);
        assert!(isle("GET", "kurallar", "", &k).2.contains("hoplayarak"));
        assert_eq!(isle("POST", "bilinmeyen", "", &k).0, 404);
    }
}
