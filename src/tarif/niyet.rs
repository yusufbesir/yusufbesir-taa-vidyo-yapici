//! Yazı → video planı: niyet (amaç) tanıma, reklam metni şablonları, varsayım ve yedek zinciri yerel adımları.
//! Sahip: Yusuf. Yazan: [Claude#AI Opus 5.5, yenileme-1.0, 2026-11-18]. Yalnız yazan yapay zekâ ya da Yusuf'un izniyle değişir; silme yok.
//! Kural: kullanıcıya teknik soru sorulmaz; belirsizlikte makul varsayım yapılır ve "varsayimlar" listesinde gösterilir.
//! Güvenlik: yalnız METİN işler; her sonuç tarif::dogrula hattından geçer.
use super::*;

/// Yerel hazırlık sonucu. `kaynak`: "kural" ya da "sablon".
#[derive(Debug, Clone)]
pub struct Hazirlik { pub tarif: Tarif, pub kaynak: &'static str, pub notlar: Vec<String>, pub anladim: Vec<String>, pub anlasilmayan: Vec<String>, pub varsayimlar: Vec<String> }

pub const MARKA: [&str; 2] = ["TERZİ", "ADEM ALTUN"];

// ───────── küçük yardımcılar ─────────
/// Belirteç `t` hedef sözcüğe uyar mı? Ön ek (4+ harf), tam eşleşme (kısa) ve tek harf yazım hatası toleransı (5+ harf).
pub fn benzer(t: &str, h: &str) -> bool {
    let (tn, hn) = (t.chars().count(), h.chars().count());
    if hn < 4 { return t == h; }
    if t.starts_with(h) { return true; }
    if hn >= 5 && tn >= hn && t.chars().next() == h.chars().next() { let p: String = t.chars().take(hn).collect(); return unlu_hatasi(&p, h); }
    if hn >= 6 && tn + 1 == hn && t.chars().next() == h.chars().next() { return mesafe(t, h) <= 1; }
    false
}
fn sayi_sozcuk(s: &str) -> Option<u32> {
    Some(match s { "bir" => 1, "iki" => 2, "uc" => 3, "dort" => 4, "bes" => 5, "alti" => 6, "yedi" => 7, "sekiz" => 8, "dokuz" => 9, "on" => 10, "yirmi" => 20, "otuz" => 30, "kirk" => 40, "elli" => 50, "altmis" => 60, "yetmis" => 70, "seksen" => 80, "doksan" => 90, _ => return None })
}
/// tok[i..] başındaki sayıyı okur (rakam ya da "yirmi beş" gibi sözcük). (değer, kullanılan belirteç sayısı)
fn sayi_oku_tok(tok: &[Bel], i: usize) -> Option<(u32, usize)> {
    let s = &tok.get(i)?.s;
    if let Ok(n) = s.trim_start_matches('%').parse::<u32>() { return Some((n, 1)); }
    let a = sayi_sozcuk(s)?;
    if a >= 10 && a % 10 == 0 { if let Some(b) = tok.get(i + 1).and_then(|x| sayi_sozcuk(&x.s)) { if b < 10 { return Some((a + b, 2)); } } }
    Some((a, 1))
}
fn fiil_mi(s: &str) -> bool { ["iyor", "acak", "ecek", "mis", "dik", "dim", "sin", "sun", "lim", "yor"].iter().any(|e| s.ends_with(e)) || (s.chars().count() > 4 && ["di", "du", "ti", "tu"].iter().any(|e| s.ends_with(e))) }
const BOS: &[&str] = &["var", "yok", "tum", "butun", "her", "cok", "en", "olan", "bu", "su", "o", "ve", "ile", "icin", "da", "de", "ki", "bir", "biz", "siz", "sizin", "bizim", "magazamizda", "dukkanimizda", "buyuk", "super", "harika", "ozel", "yuzde", "olsun", "yaz", "yazsin", "ekrana", "ekranda", "lutfen", "simdi", "hemen", "gibi", "kadar", "olarak", "sonra", "once", "ardindan", "son", "hos", "gelsin", "daha"];
fn bos_mu(s: &str) -> bool { BOS.contains(&s) || DURAK.iter().any(|d| s == *d) }

/// Bir LED sahnesi taslağı: satırlar + rol (stil seçimi için).
#[derive(Clone, Debug)]
struct Taslak { satir: Vec<String>, rol: Rol, tema: Option<(&'static str, &'static str, &'static str)> }
#[derive(Clone, Copy, Debug, PartialEq)]
enum Rol { Giris, Vurgu, Bilgi, Imza }
fn t(satir: &[&str], rol: Rol) -> Taslak { Taslak { satir: satir.iter().map(|x| x.to_string()).collect(), rol, tema: None } }
fn ts(satir: Vec<String>, rol: Rol) -> Taslak { Taslak { satir, rol, tema: None } }
impl Taslak { fn tema(mut self, giris: &'static str, efekt: &'static str, palet: &'static str) -> Self { self.tema = Some((giris, efekt, palet)); self } }

/// Rol başına stil seçenekleri (giriş, efekt, palet, yazı tipi, süre). `cesit` ile döner ("Başka öneri").
fn stil(rol: Rol, cesit: usize) -> (&'static str, &'static str, &'static str, &'static str, f32) {
    let v: &[(&str, &str, &str, &str, f32)] = match rol {
        Rol::Giris => &[("belir", "", "sicak-beyaz", "bebas", 3.5), ("alttan", "", "beyaz", "anton", 3.5), ("yandan", "", "gok-mavisi", "bebas", 3.5), ("toplan", "", "seftali", "anton", 3.5)],
        Rol::Vurgu => &[("zipla", "konfeti", "kirmizi-altin", "anton", 4.0), ("zoom", "havai-fisek", "altin-gece", "anton", 4.0), ("patla", "pirilti", "gun-batimi", "bebas", 4.0), ("elastik", "parlama", "sari-lacivert", "anton", 4.0)],
        Rol::Bilgi => &[("alttan", "", "sicak-beyaz", "bebas", 4.0), ("belir", "cerceve", "buz-cyan", "anton", 4.0), ("soldan", "", "altin", "bebas", 4.0), ("yandan", "", "nane", "anton", 4.0)],
        Rol::Imza => &[("belir", "cerceve-altin", "altin-gece", "anton", 3.5), ("toplan", "parlama", "altin", "anton", 3.5), ("belir", "pirilti", "bakir", "bebas", 3.5), ("alttan", "cerceve", "tek-altin", "anton", 3.5)],
    };
    v[cesit % v.len()]
}

// ───────── niyet tanıma ─────────
#[derive(Default, Debug)]
struct Bulgu { taslak: Vec<(usize, Taslak)>, kullanilan: Vec<usize>, varsayim: Vec<String>, notlar: Vec<String>, marka_var: bool, kampanya: bool }
impl Bulgu {
    fn ekle(&mut self, kon: usize, x: Taslak) { self.taslak.push((kon, x)); }
    fn kul(&mut self, i: usize) { if !self.kullanilan.contains(&i) { self.kullanilan.push(i); } }
}
fn bul(tok: &[Bel], h: &[&str]) -> Option<usize> { tok.iter().position(|b| h.iter().any(|x| benzer(&b.s, x))) }
fn buyuk_satirlar(sozcukler: &[String]) -> Vec<String> {
    // satır başına en çok ~10 harf: kısa sözcükler aynı satırda birleşir
    let mut o: Vec<String> = vec![];
    for s in sozcukler {
        let s = buyuk(s);
        match o.last_mut() { Some(l) if l.chars().count() + 1 + s.chars().count() <= 10 => { l.push(' '); l.push_str(&s); } _ => o.push(s) }
    }
    o
}
/// Tetik belirtecinden önceki ürün/konu sözcükleri (en çok 3). Bulunduğu (-de/-da) sözcük ve öncesi tercih edilir; fiil ve boş sözcükte durur.
fn urun_once(tok: &[Bel], tetik: usize, b: &mut Bulgu, stil_sozcuk: &dyn Fn(&str) -> bool) -> Option<Vec<String>> {
    let mut v = vec![];
    let mut i = tetik;
    while i > 0 && v.len() < 3 {
        i -= 1;
        let s = &tok[i].s;
        if b.kullanilan.contains(&i) { break; }
        if bos_mu(s) || fiil_mi(s) || stil_sozcuk(s) || s.starts_with('%') || s.chars().all(|c| c.is_ascii_digit()) || sayi_sozcuk(s).is_some() || s.contains('§') { if v.is_empty() && (s == "yuzde" || s.starts_with('%') || s.chars().all(|c| c.is_ascii_digit()) || sayi_sozcuk(s).is_some() || s == "buyuk" || s == "ozel") { continue; } break; }
        v.insert(0, i);
    }
    if v.is_empty() { return None; }
    for &k in &v { b.kul(k); }
    Some(v.iter().map(|&k| tok[k].o.clone()).collect())
}
fn yuzde_bul(tok: &[Bel], b: &mut Bulgu) -> Option<u32> {
    for i in 0..tok.len() {
        let s = &tok[i].s;
        if let Some(n) = s.strip_prefix('%').and_then(|x| x.replace(',', ".").parse::<f32>().ok()) { b.kul(i); return Some(n as u32); }
        if let Some(n) = s.strip_suffix('%').and_then(|x| x.parse::<u32>().ok()) { b.kul(i); return Some(n); }
        if s == "yuzde" || benzer(s, "yuzde") { if let Some((n, k)) = sayi_oku_tok(tok, i + 1) { for j in i..=i + k { b.kul(j); } return Some(n); } }
    }
    None
}
fn saat_bicim(h: u32, m: u32) -> String { format!("{h:02}:{m:02}") }
/// Ham metinde "09:00 - 20:00", "9-19", "10.30-18" gibi saat aralıklarını ve önündeki gün etiketini bulur.
fn saatler(ham: &str) -> Vec<(Option<String>, String)> {
    let c: Vec<char> = ham.chars().collect();
    let mut o = vec![];
    let mut i = 0;
    let sayi = |i: &mut usize| -> Option<(u32, u32)> {
        let b = *i; let mut h = 0u32;
        while *i < c.len() && c[*i].is_ascii_digit() && *i - b < 2 { h = h * 10 + c[*i].to_digit(10).unwrap(); *i += 1; }
        if *i == b || (*i < c.len() && c[*i].is_ascii_digit()) { return None; }
        let mut m = 0;
        if *i + 2 < c.len() && matches!(c[*i], ':' | '.') && c[*i + 1].is_ascii_digit() && c[*i + 2].is_ascii_digit() { m = c[*i + 1].to_digit(10).unwrap() * 10 + c[*i + 2].to_digit(10).unwrap(); *i += 3; }
        if h > 24 || m > 59 { return None; }
        Some((h, m))
    };
    while i < c.len() {
        if c[i].is_ascii_digit() && (i == 0 || !c[i - 1].is_ascii_digit()) {
            let bas = i;
            let mut j = i;
            if let Some((h1, m1)) = sayi(&mut j) {
                let mut k = j; while k < c.len() && c[k] == ' ' { k += 1; }
                if k < c.len() && matches!(c[k], '-' | '–' | '—') {
                    k += 1; while k < c.len() && c[k] == ' ' { k += 1; }
                    let mut l = k;
                    if let Some((h2, m2)) = sayi(&mut l) {
                        if h2 > h1 || (h2 == h1 && m2 > m1) || h2 < h1 && h2 <= 6 {
                            let once: String = c[..bas].iter().collect();
                            let son = sadelestir(&once);
                            let w: Vec<&str> = son.split(|x: char| !x.is_alphanumeric()).filter(|x| !x.is_empty()).collect();
                            let etiket = w.iter().rev().take(3).rev().copied().collect::<Vec<_>>();
                            let gun = |e: &[&str]| -> Option<String> {
                                let j = e.join(" ");
                                for (a, b) in [("hafta ici", "HAFTA İÇİ"), ("haftaici", "HAFTA İÇİ"), ("hafta sonu", "HAFTA SONU"), ("haftasonu", "HAFTA SONU"), ("cumartesi", "CUMARTESİ"), ("pazar", "PAZAR"), ("her gun", "HER GÜN"), ("pazartesi", "PAZARTESİ"), ("cuma", "CUMA")] { if j.contains(a) { return Some(b.into()); } }
                                None
                            };
                            o.push((gun(&etiket), format!("{}-{}", saat_bicim(h1, m1), saat_bicim(h2, m2))));
                            i = l; continue;
                        }
                    }
                }
            }
            i = j.max(i + 1); continue;
        }
        i += 1;
    }
    o
}

fn niyetler(ham: &str, tok: &[Bel], stil_sozcuk: &dyn Fn(&str) -> bool) -> Bulgu {
    let mut b = Bulgu::default();
    let ss = |h: &[&str]| bul(tok, h);
    // marka
    if let Some(i) = tok.windows(2).position(|w| w[0].s == "adem" && benzer(&w[1].s, "altun")) {
        b.marka_var = true; b.kul(i); b.kul(i + 1); if i > 0 && tok[i - 1].s == "terzi" { b.kul(i - 1); }
        b.ekle(if i > 0 && tok[i - 1].s == "terzi" { i - 1 } else { i }, t(&MARKA, Rol::Imza));
    }
    // bayram ve özel günler
    if let Some(i) = ss(&["bayram"]) {
        b.kul(i);
        let ad = if i > 0 { match tok[i - 1].s.as_str() { "ramazan" => Some("RAMAZAN"), "kurban" => Some("KURBAN"), "seker" => Some("ŞEKER"), "cumhuriyet" => Some("CUMHURİYET"), _ => None } } else { None };
        if ad.is_some() { b.kul(i - 1); }
        let mubarek = ss(&["mubarek"]).map(|k| { b.kul(k); true }).unwrap_or(false);
        let mut s: Vec<String> = vec![];
        if let Some(a) = ad { s.push(a.into()); }
        s.push("BAYRAMINIZ".into());
        b.ekle(i, ts(s, Rol::Giris).tema("belir", "yildiz-yagmuru", "altin-gece"));
        b.ekle(i, t(&[if mubarek { "MÜBAREK" } else { "KUTLU" }, "OLSUN"], Rol::Vurgu).tema("zoom", "havai-fisek", "yesil-altin"));
        for k in [ss(&["kutlu"]), ss(&["kutlama"]), ss(&["mesaj"]), ss(&["olsun"])].into_iter().flatten() { b.kul(k); }
    }
    for (anahtar, satir, efekt, palet) in [("anneler", ["ANNELER", "GÜNÜNÜZ"], "kalp", "gul-kurusu"), ("babalar", ["BABALAR", "GÜNÜNÜZ"], "yildiz-yagmuru", "okyanus"), ("sevgililer", ["SEVGİLİLER", "GÜNÜ"], "kalp", "pembe-parilti"), ("ogretmenler", ["ÖĞRETMENLER", "GÜNÜNÜZ"], "pirilti", "sicak-beyaz")] {
        if let Some(i) = ss(&[anahtar]) {
            b.kul(i); if tok.get(i + 1).map(|x| x.s.starts_with("gun")).unwrap_or(false) { b.kul(i + 1); }
            b.ekle(i, t(&satir, Rol::Giris).tema("belir", efekt, palet));
            if ss(&["indirim", "%"]).is_none() && yuzde_bul(tok, &mut Bulgu::default()).is_none() { b.ekle(i, t(&["KUTLU", "OLSUN"], Rol::Vurgu).tema("zoom", efekt, palet)); } else { b.kampanya = true; }
        }
    }
    if let Some(i) = ss(&["yilbasi"]).or_else(|| tok.windows(2).position(|w| w[0].s == "yeni" && (benzer(&w[1].s, "yil") || w[1].s.starts_with("yil")))) {
        b.kul(i); if tok.get(i + 1).map(|x| x.s.starts_with("yil")).unwrap_or(false) { b.kul(i + 1); }
        let kampanya = ss(&["kampanya", "indirim"]).is_some() || yuzde_bul(tok, &mut Bulgu::default()).is_some();
        if kampanya { b.ekle(i, t(&["YILBAŞI", "FIRSATI"], Rol::Giris).tema("belir", "kar", "buz-cyan")); b.kampanya = true; if let Some(k) = ss(&["kampanya"]) { b.kul(k); } }
        else { b.ekle(i, t(&["MUTLU", "YILLAR"], Rol::Vurgu).tema("zoom", "kar", "buz-cyan")); }
    }
    if let Some(i) = ss(&["okul"]) {
        b.kul(i); let acil = ss(&["aciliyor", "acilis"]); if let Some(k) = acil { b.kul(k); }
        b.ekle(i, t(if acil.is_some() { &["OKULLAR", "AÇILIYOR"] } else { &["OKULA", "DÖNÜŞ"] }, Rol::Giris).tema("alttan", "balon", "sari-lacivert"));
    }
    // açılış
    let acilis = tok.iter().position(|x| ["acildi", "acildik", "acilisimiz", "acilisina", "acilisa", "acilisimiza"].iter().any(|h| x.s.starts_with(h)))
        .or_else(|| tok.windows(2).position(|w| w[0].s == "yeni" && (benzer(&w[1].s, "magaza") || benzer(&w[1].s, "subemiz") || w[1].s.starts_with("sube"))));
    if let Some(i) = acilis {
        b.kul(i); for k in i.saturating_sub(2)..(i + 2).min(tok.len()) { let s = &tok[k].s; if s == "yeni" || benzer(s, "magaza") || s.starts_with("sube") || s.starts_with("acil") { b.kul(k); } }
        b.ekle(i, t(&["YENİ", "MAĞAZAMIZ"], Rol::Giris));
        b.ekle(i, t(&["AÇILDI"], Rol::Vurgu).tema("patla", "havai-fisek", "altin-gece"));
        b.kampanya = true;
    }
    // indirim
    let yuzde = yuzde_bul(tok, &mut b);
    let ind = ss(&["indirim", "indirm", "ucuzluk", "firsat", "kampanya"]);
    if ind.is_some() || yuzde.is_some() {
        let i = ind.or_else(|| tok.iter().position(|x| x.s.starts_with('%') || x.s == "yuzde")).unwrap_or(0);
        if let Some(k) = ind { b.kul(k); }
        let tetik = tok.iter().position(|x| x.s.starts_with('%') || x.s == "yuzde" || benzer(&x.s, "yuzde")).map(|p| p.min(i)).unwrap_or(i);
        if let Some(u) = urun_once(tok, tetik, &mut b, stil_sozcuk) { b.ekle(i, ts(buyuk_satirlar(&u), Rol::Giris)); }
        let buyuk_ind = ss(&["buyuk"]).map(|k| { b.kul(k); true }).unwrap_or(false);
        let vurgu = match yuzde { Some(n) if n > 0 && n < 100 => vec![format!("%{n}"), "İNDİRİM".into()], _ => { if !buyuk_ind { b.varsayim.push("İndirim oranı yazılmadığı için \"BÜYÜK İNDİRİM\" yazdım.".into()); } vec!["BÜYÜK".into(), "İNDİRİM".into()] } };
        b.ekle(i, ts(vurgu, Rol::Vurgu));
        if let Some(k) = tok.windows(2).position(|w| w[0].s == "son" && benzer(&w[1].s, "gun")) { b.kul(k); b.kul(k + 1); b.ekle(k, t(&["SON", "GÜN"], Rol::Vurgu).tema("nabiz", "", "turuncu")); }
        b.kampanya = true;
    }
    // taksit
    if let Some(i) = ss(&["taksit"]) {
        b.kul(i);
        let n = (i > 0).then(|| sayi_oku_tok(tok, i - 1)).flatten().filter(|(_, k)| *k == 1).or_else(|| if i > 1 { sayi_oku_tok(tok, i - 2).filter(|(_, k)| *k == 2) } else { None });
        if let Some((_, k)) = n { for j in i - k..i { b.kul(j); } }
        if let Some(k) = ss(&["kredi"]) { b.kul(k); if let Some(m) = ss(&["kart"]) { b.kul(m); } b.ekle(i, t(&["KREDİ", "KARTINA"], Rol::Giris)); }
        else if ss(&["pesin"]).is_some() { if let Some(k) = ss(&["pesin"]) { b.kul(k); } if let Some(k) = ss(&["fiyat"]) { b.kul(k); } b.ekle(i, t(&["PEŞİN", "FİYATINA"], Rol::Giris)); }
        match n { Some((n, _)) if (2..=24).contains(&n) => b.ekle(i, ts(vec![n.to_string(), "TAKSİT".into()], Rol::Vurgu)), _ => { b.varsayim.push("Taksit sayısı yazılmadığı için \"TAKSİTLİ SATIŞ\" yazdım.".into()); b.ekle(i, t(&["TAKSİTLİ", "SATIŞ"], Rol::Vurgu)); } }
        b.kampanya = true;
    }
    // fiyat (taksit/yüzde değilse): "499 TL", "₺499", "1.500 lira"
    for i in 0..tok.len() {
        let s = tok[i].s.as_str();
        let rakam = s.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false);
        let tl_sonra = tok.get(i + 1).map(|x| x.s == "tl" || x.s.starts_with("lira")).unwrap_or(false);
        let tl_ek = rakam && (s.ends_with("tl") || s.ends_with("lira"));
        if (rakam && tl_sonra) || s.starts_with('₺') || tl_ek {
            let sayi: String = tok[i].o.trim_start_matches('₺').trim_end_matches(|c: char| c.is_alphabetic()).to_string();
            if sayi.is_empty() { continue; }
            b.kul(i); if tl_sonra { b.kul(i + 1); }
            if let Some(u) = urun_once(tok, i, &mut b, stil_sozcuk) { b.ekle(i, ts(buyuk_satirlar(&u), Rol::Giris)); }
            b.ekle(i, ts(vec![format!("{sayi} TL")], Rol::Vurgu).tema("zoom", "parlama", "altin-gece"));
            b.kampanya = true;
            break;
        }
    }
    // çalışma saatleri
    let s = saatler(ham);
    let saat_soz = ss(&["saat", "calisma", "acigiz", "acik"]);
    if !s.is_empty() {
        let i = saat_soz.unwrap_or(0);
        for k in 0..tok.len() { let x = &tok[k].s; if x.chars().all(|c| c.is_ascii_digit()) || benzer(x, "saatleri") || x.starts_with("saat") || benzer(x, "calisma") || x.starts_with("acig") || x == "acik" || ["hafta", "ici", "sonu", "cumartesi", "pazar", "pazartesi", "cuma", "her", "gun", "arasi"].contains(&x.as_str()) { b.kul(k); } }
        if s.len() == 1 && s[0].0.is_none() { b.ekle(i, t(&["HER GÜN", "AÇIĞIZ"], Rol::Giris)); b.ekle(i, ts(vec![s[0].1.clone()], Rol::Bilgi)); }
        else { b.ekle(i, t(&["ÇALIŞMA", "SAATLERİ"], Rol::Giris)); for (g, r) in s.iter().take(4) { b.ekle(i, ts(vec![g.clone().unwrap_or_else(|| "AÇIK".into()), r.clone()], Rol::Bilgi)); } }
        if let Some(k) = tok.windows(2).position(|w| w[0].s == "pazar" && benzer(&w[1].s, "kapali")) { b.kul(k + 1); b.ekle(i, t(&["PAZAR", "KAPALI"], Rol::Bilgi)); }
    } else if saat_soz.is_some() && ss(&["tarih"]).is_some() {
        b.notlar.push("Canlı saat/tarih gösterimi bu sürümde yok; ekrana yazı olarak yazabilirsiniz (ör. \"AÇILIŞ 09:00\").".into());
    }
    // kapalıyız
    if let Some(i) = ss(&["kapali", "kapaliyiz", "kapatiyoruz", "tatildeyiz"]).filter(|_| s.is_empty()) {
        b.kul(i); for k in [ss(&["bugun"]), ss(&["dukkan"]), ss(&["magaza"]), ss(&["tatil"])].into_iter().flatten() { b.kul(k); }
        let neden = ss(&["tadilat"]).map(|k| { b.kul(k); "TADİLAT" }).or_else(|| ss(&["tatil"]).map(|_| "TATİL"));
        let gun = if ss(&["yarin"]).is_some() { "YARIN" } else { "BUGÜN" };
        if let Some(k) = ss(&["yarin"]) { b.kul(k); }
        match neden { Some(n) => { b.ekle(i, t(&[n, "NEDENİYLE"], Rol::Giris)); b.ekle(i, t(&[gun, "KAPALIYIZ"], Rol::Vurgu).tema("belir", "", "turuncu")); } None => b.ekle(i, t(&[gun, "KAPALIYIZ"], Rol::Vurgu).tema("belir", "", "turuncu")) }
    }
    // yeni sezon / yeni ürün geldi
    if let Some(i) = tok.windows(2).position(|w| w[0].s == "yeni" && benzer(&w[1].s, "sezon")) { b.kul(i); b.kul(i + 1); b.ekle(i, t(&["YENİ", "SEZON"], Rol::Vurgu).tema("zoom", "pirilti", "altin-cyan")); b.kampanya = true; }
    else if let Some(i) = tok.iter().position(|x| x.s == "geldi" || x.s == "geldik" || x.s.starts_with("gelenler")) {
        b.kul(i);
        let urun = urun_once(tok, i, &mut b, stil_sozcuk).map(|u| u.iter().map(|w| { let s = sadelestir(w); let o = w.clone(); if s.ends_with("lerimiz") || s.ends_with("larimiz") { o.chars().take(o.chars().count() - 3).collect() } else { o } }).collect::<Vec<String>>());
        if let Some(u) = urun { b.ekle(i, ts(buyuk_satirlar(&[vec!["YENİ".to_string()], u].concat()), Rol::Giris)); b.ekle(i, t(&["GELDİ"], Rol::Vurgu).tema("zipla", "pirilti", "altin-cyan")); }
        else { b.ekle(i, t(&["YENİLER", "GELDİ"], Rol::Vurgu).tema("zipla", "pirilti", "altin-cyan")); }
        b.kampanya = true;
    }
    // hizmetler
    let hizmet: Vec<(usize, &str)> = [("tadilat", "TADİLAT"), ("dikim", "DİKİM"), ("utu", "ÜTÜ"), ("paca", "PAÇA"), ("kisaltma", "KISALTMA"), ("daraltma", "DARALTMA"), ("kuru", "KURU TEMİZLEME"), ("olcu", "ÖLÇÜYE"), ("ozel dikim", "ÖZEL DİKİM")]
        .iter().filter_map(|(h, a)| tok.iter().position(|x| x.s == *h || (h.len() >= 4 && x.s.starts_with(h))).filter(|&k| !b.kullanilan.contains(&k)).map(|k| (k, *a))).collect();
    if !hizmet.is_empty() {
        let i = hizmet[0].0;
        for (k, _) in &hizmet { b.kul(*k); }
        for k in [ss(&["hizmet"]), ss(&["temizleme"]), ss(&["yapilir"]), ss(&["yapilir"])].into_iter().flatten() { b.kul(k); }
        let adlar: Vec<String> = hizmet.iter().map(|x| x.1.to_string()).collect();
        for parca in adlar.chunks(2) { b.ekle(i, ts(parca.to_vec(), Rol::Bilgi)); }
        b.ekle(i, t(&["YAPILIR"], Rol::Vurgu).tema("zipla", "", "sicak-beyaz"));
        b.kampanya = true;
    }
    // karşılama, teşekkür, davet
    if let Some(i) = tok.windows(2).position(|w| w[0].s == "hos" && benzer(&w[1].s, "geldiniz")).or_else(|| ss(&["hosgeldin"])) {
        b.kul(i); if tok.get(i + 1).map(|x| x.s.starts_with("geld")).unwrap_or(false) { b.kul(i + 1); }
        b.ekle(i, t(&["HOŞ", "GELDİNİZ"], Rol::Vurgu).tema("belir", "pirilti", "altin"));
    }
    if let Some(i) = ss(&["tesekkur", "sagolun"]) { b.kul(i); if let Some(k) = ss(&["ederiz"]) { b.kul(k); } if let Some(k) = ss(&["musteri"]) { b.kul(k); } b.ekle(i, t(&["TEŞEKKÜR", "EDERİZ"], Rol::Vurgu).tema("belir", "kalp", "sicak-beyaz")); }
    if let Some(i) = ss(&["bekleriz", "bekliyoruz", "davetlisiniz", "buyurun", "ugrayin"]) { b.kul(i); for k in [ss(&["herkes"]), ss(&["sizi"]), ss(&["magazamiza"])].into_iter().flatten() { b.kul(k); } b.ekle(i, t(&["SİZİ", "BEKLERİZ"], Rol::Bilgi)); }
    if let Some(i) = ss(&["merhaba"]).filter(|_| b.taslak.is_empty()) { b.kul(i); b.ekle(i, t(&["MERHABA"], Rol::Vurgu)); }
    b
}

/// Kullanıcının yazdığı stil (renk, animasyon, efekt, süre, geçiş) kural motoruyla okunur.
#[derive(Default, Debug, Clone)]
struct Istek { giris: Option<String>, cikis: Option<String>, palet: Option<String>, font: Option<String>, efekt: Vec<String>, sure: Option<f32>, acilis: String, kapanis: String, gorunum: String, anladim: Vec<String>, anlasilmayan: Vec<String> }
fn istek_oku(metin: &str, kat: &Katalog, kullanici: &[Kural]) -> Istek {
    // metinsiz cümleler bir yer tutucu sahneye bağlanır; yalnız stil bilgisi alınır
    let z = kural_coz(&format!("\"X\" {metin}"), kat, kullanici);
    let mut i = Istek { acilis: "yok".into(), kapanis: "yok".into(), gorunum: "duz".into(), anlasilmayan: z.anlasilmayan.clone(), ..Default::default() };
    let a = z.anladim.join(" ; ");
    if let Some(tf) = z.tarif {
        i.acilis = tf.acilis; i.kapanis = tf.kapanis; i.gorunum = tf.gorunum;
        let mut sahneler = tf.sahneler.into_iter();
        if let Some(s) = sahneler.next() {
            if a.contains("→ giriş:") { i.giris = Some(s.giris); }
            if a.contains("→ çıkış:") { i.cikis = Some(s.cikis); }
            if a.contains("→ renk:") { i.palet = Some(s.palet); }
            if a.contains("→ yazı tipi:") { i.font = Some(s.font); }
            if a.contains("süre:") { i.sure = Some(s.sure); }
            i.efekt = s.efekt;
        }
    }
    for x in z.anladim { let x = x.replace("Sahne 1: \"X\"", "").replace("Sahne 1 için ek: ", ""); let x = x.trim_start_matches(" — ").trim().to_string(); if !x.is_empty() { i.anladim.push(x); } }
    i
}
fn stil_sozcuk_mu(s: &str, kurallar: &[Kural]) -> bool {
    if s.chars().count() < 3 { return false; }
    kurallar.iter().filter(|k| k.kategori != "gorunum").any(|k| k.ifadeler.iter().any(|f| { let w: Vec<String> = sadelestir(f).split_whitespace().map(String::from).collect(); w.len() == 1 && kelime_uyar(s, &w[0]) }))
}

/// Taslakları stillendirip doğrulanmış tarife çevirir.
fn kur(taslak: Vec<Taslak>, ist: &Istek, cesit: usize, kat: &Katalog) -> Result<Dogrulama, Vec<String>> {
    let mut sahneler = vec![];
    for x in taslak.into_iter().take(MAX_SAHNE) {
        let (mut g, mut e, mut p, f, s) = stil(x.rol, cesit);
        if let (Some((tg, te, tp)), 0) = (x.tema, cesit) { g = tg; e = te; p = tp; }
        let mut efekt: Vec<String> = if e.is_empty() { vec![] } else { vec![e.to_string()] };
        if x.rol == Rol::Vurgu || x.rol == Rol::Giris { for k in &ist.efekt { if !efekt.contains(k) { efekt.push(k.clone()); } } }
        if !ist.efekt.is_empty() && x.rol != Rol::Imza { efekt.retain(|k| ist.efekt.contains(k) || !matches!(k.as_str(), "konfeti" | "havai-fisek" | "pirilti" | "parlama")); }
        sahneler.push(TSahne {
            metin: x.satir, giris: ist.giris.clone().unwrap_or_else(|| g.to_string()), cikis: ist.cikis.clone().unwrap_or_else(|| "sondur".to_string()),
            efekt, palet: ist.palet.clone().unwrap_or_else(|| p.to_string()), font: ist.font.clone().unwrap_or_else(|| f.to_string()), sure: ist.sure.unwrap_or(s),
        });
    }
    let acilis = if ist.acilis != "yok" { ist.acilis.clone() } else { ["fade", "perde", "kapi", "iris"][cesit % 4].to_string() };
    let kapanis = if ist.kapanis != "yok" { ist.kapanis.clone() } else { "fade".to_string() };
    dogrula(&tarif_json(&Tarif { gorunum: ist.gorunum.clone(), sahneler, acilis, kapanis }), kat)
}

/// Her koşulda geçerli olan güvenli karşılama şablonu.
pub fn guvenli_sablon(kat: &Katalog, cesit: usize) -> Tarif {
    let ist = Istek { acilis: "yok".into(), kapanis: "yok".into(), gorunum: "duz".into(), ..Default::default() };
    let x = vec![t(&["HOŞ", "GELDİNİZ"], Rol::Vurgu).tema("belir", "pirilti", "altin"), t(&MARKA, Rol::Imza)];
    match kur(x, &ist, cesit, kat) {
        Ok(d) => d.tarif,
        // katalog boş olsa bile çalışan en sade biçim
        Err(_) => Tarif { gorunum: "duz".into(), acilis: "yok".into(), kapanis: "yok".into(), sahneler: vec![TSahne { metin: vec!["HOŞ GELDİNİZ".into()], giris: "belir".into(), cikis: "yok".into(), efekt: vec![], palet: "altin".into(), font: String::new(), sure: 4.5 }] },
    }
}

/// Uzun sahneleri (6+ sözcük) okunur parçalara böler: sahne başına en çok 3 sözcük.
fn uzunlari_bol(t: &mut Tarif) -> bool {
    let mut degisti = false;
    let mut yeni = vec![];
    for s in t.sahneler.drain(..) {
        let w: Vec<String> = s.metin.iter().flat_map(|m| m.split_whitespace().map(String::from).collect::<Vec<_>>()).collect();
        if w.len() <= 5 { yeni.push(s); continue; }
        degisti = true;
        for (k, p) in w.chunks(3).take(4).enumerate() { let mut x = s.clone(); x.metin = buyuk_satirlar(p); x.sure = 3.5; if k > 0 && x.giris == "belir" { x.giris = "alttan".into(); } yeni.push(x); }
    }
    yeni.truncate(MAX_SAHNE);
    t.sahneler = yeni;
    degisti
}
/// Aynı sözcük öbeği defalarca yinelenen metni sadeleştirir ("bir sürü yazı bir sürü yazı ...").
fn yinelemeyi_azalt(m: &str) -> (String, bool) {
    let w: Vec<&str> = m.split_whitespace().collect();
    if w.len() < 12 { return (m.to_string(), false); }
    let mut tekil: Vec<&str> = vec![];
    for x in &w { if !tekil.iter().any(|y| sadelestir(y) == sadelestir(x)) { tekil.push(x); } }
    if tekil.len() * 3 < w.len() { (tekil.join(" "), true) } else { (m.to_string(), false) }
}
/// Ekrana yazılacak içerik taşımayan sözcük kökleri (yalnız bunlar kaldıysa metin uydurulmaz, şablon önerilir).
const ICERIKSIZ: &[&str] = &["guzel", "sey", "saga", "sola", "yukari", "asagi", "dikkat", "cek", "renkli", "rengarenk", "animasyon", "efekt", "gozuk", "goruns", "saat", "tarih", "suru", "yazi", "hareket", "canli", "havali", "sik", "profesyonel", "kayarak", "gecsin", "cok", "biraz", "hizli", "yavas"];
const BIRIM: &[&str] = &["tl", "ytl", "kdv", "tv", "led", "avm", "xl", "xxl", "usb", "sms"];
fn acik_metin_var(m: &str) -> bool {
    if m.chars().any(|c| matches!(c, '"' | '“' | '”' | '«' | '»')) { return true; }
    let kucuk = m.chars().any(|c| c.is_lowercase());
    kucuk && m.split(|c: char| !c.is_alphanumeric()).any(|w| { let h: Vec<char> = w.chars().filter(|c| c.is_alphabetic()).collect(); h.len() >= 2 && h.iter().all(|c| c.is_uppercase()) && !BIRIM.contains(&sadelestir(w).as_str()) })
}

/// Yerel zincir: niyet şablonu → kural motoru (metin aynen) → güvenli şablon. Her zaman geçerli bir tarif döner.
pub fn kural_hazirla(metin: &str, kat: &Katalog, kullanici: &[Kural], cesit: usize) -> Hazirlik {
    let ham: String = metin.chars().filter(|c| !c.is_control() || *c == '\n').take(MAX_TARIF).collect();
    let (ham, yinelendi) = yinelemeyi_azalt(ham.trim());
    let mut varsayimlar = vec![];
    if yinelendi { varsayimlar.push("Yinelenen sözcükleri bir kez aldım.".into()); }
    let sablon = |varsayimlar: Vec<String>, not: &str| Hazirlik { tarif: guvenli_sablon(kat, cesit), kaynak: "sablon", notlar: vec![], anladim: vec![], anlasilmayan: vec![], varsayimlar: [varsayimlar, vec![not.to_string()]].concat() };
    if ham.trim().is_empty() { return sablon(varsayimlar, "Tarif boş olduğu için hazır karşılama videosu önerdim. Videoyu bir cümleyle anlatırsanız ona göre hazırlarım."); }
    let mut kurallar: Vec<Kural> = gomulu_kurallar().clone();
    for k in kullanici { if kural_gecerli(k, kat).is_ok() { kurallar.push(k.clone()); } }
    let oto = otomatik_kurallar(kat, &kurallar); kurallar.extend(oto);
    let ist = istek_oku(&ham, kat, kullanici);
    // 1) Kullanıcı yazılacak metni açıkça verdiyse (tırnak ya da BÜYÜK HARF) metne dokunulmaz
    if acik_metin_var(&ham) {
        let z = kural_coz(&ham, kat, kullanici);
        if let Some(mut tf) = z.tarif {
            let mut notlar = z.notlar;
            if uzunlari_bol(&mut tf) { varsayimlar.push("Uzun metni uzaktan okunsun diye kısa sahnelere böldüm.".into()); }
            let varsayilan = tf.sahneler.iter().all(|s| s.giris == "belir" && s.efekt.is_empty()) && ist.giris.is_none() && ist.efekt.is_empty();
            if varsayilan {
                let n = tf.sahneler.len();
                for (k, s) in tf.sahneler.iter_mut().enumerate() { let rol = if k + 1 == n && n > 1 { Rol::Vurgu } else if k == 0 { Rol::Giris } else { Rol::Bilgi }; let (g, e, p, f, _) = stil(rol, cesit); s.giris = g.into(); if !e.is_empty() { s.efekt = vec![e.into()]; } if ist.palet.is_none() { s.palet = p.into(); } if s.font.is_empty() { s.font = f.into(); } if s.cikis == "yok" { s.cikis = "sondur".into(); } }
                if tf.acilis == "yok" { tf.acilis = "fade".into(); }
                varsayimlar.push("Animasyon ve renk yazılmadığı için okunaklı bir düzen seçtim.".into());
            }
            if let Ok(d) = dogrula(&tarif_json(&tf), kat) { notlar.extend(d.notlar); return Hazirlik { tarif: d.tarif, kaynak: "kural", notlar, anladim: z.anladim, anlasilmayan: z.anlasilmayan, varsayimlar }; }
        }
    }
    // 2) Niyet şablonları (indirim, taksit, bayram, açılış, saat, kapalı, yeni sezon, hizmet, karşılama ...)
    let tok = belirtecle(&ham);
    let ss = |s: &str| stil_sozcuk_mu(s, &kurallar);
    let mut b = niyetler(&ham, &tok, &ss);
    if !b.taslak.is_empty() {
        b.taslak.sort_by_key(|x| x.0);
        let mut taslak: Vec<Taslak> = vec![];
        for (_, x) in b.taslak.drain(..) { if !taslak.iter().any(|y| y.satir == x.satir) { taslak.push(x); } }
        if b.kampanya && !b.marka_var { taslak.push(t(&MARKA, Rol::Imza)); varsayimlar.push("Sona işletme adını (TERZİ ADEM ALTUN) ekledim.".into()); }
        if taslak.len() > MAX_SAHNE { taslak.truncate(MAX_SAHNE); }
        varsayimlar.extend(b.varsayim.drain(..));
        if let Ok(d) = kur(taslak, &ist, cesit, kat) {
            let kul: Vec<String> = b.kullanilan.iter().map(|&i| tok[i].o.to_lowercase()).collect();
            let anlasilmayan: Vec<String> = ist.anlasilmayan.iter().filter(|w| !kul.contains(w) && !ss(&sadelestir(w)) && !bos_mu(&sadelestir(w)) && !fiil_mi(&sadelestir(w))).cloned().collect();
            let mut anladim = vec![format!("Konu: {}", d.tarif.sahneler.iter().map(|s| s.metin.join(" ")).collect::<Vec<_>>().join(" → "))];
            anladim.extend(ist.anladim.iter().cloned());
            let mut notlar = b.notlar; notlar.extend(d.notlar);
            return Hazirlik { tarif: d.tarif, kaynak: "kural", notlar, anladim, anlasilmayan, varsayimlar };
        }
    }
    // 3) Metni olduğu gibi yaz (kural motoru). Yalnız "güzel bir şey", "sağa", "animasyon" gibi içerik taşımayan sözcükler kaldıysa şablona geçilir.
    let z = kural_coz(&ham, kat, kullanici);
    let icerik_yok = z.tarif.as_ref().map(|tf| tf.sahneler.iter().flat_map(|s| s.metin.iter()).flat_map(|m| m.split_whitespace().map(|w| sadelestir(w)).collect::<Vec<_>>()).all(|w| ICERIKSIZ.iter().any(|j| w.starts_with(j)) || bos_mu(&w))).unwrap_or(true);
    let anim_istegi = tok.iter().any(|x| x.s.starts_with("animasyon") || x.s.starts_with("efekt"));
    if icerik_yok || anim_istegi {
        let bilinmeyen: Vec<String> = tok.iter().filter(|x| !ICERIKSIZ.iter().any(|j| x.s.starts_with(j)) && !bos_mu(&x.s) && !ss(&x.s) && !fiil_mi(&x.s) && !x.s.starts_with("animasyon") && !x.s.starts_with("efekt")).map(|x| x.o.to_lowercase()).collect();
        let mut h = sablon(varsayimlar.clone(), "Ekrana yazılacak metin yazılmadığı için \"HOŞ GELDİNİZ\" yazdım; isterseniz metni değiştirin.");
        h.notlar.extend(b.notlar.iter().cloned());
        if !bilinmeyen.is_empty() && anim_istegi { h.notlar.push(format!("\"{}\" adında bir animasyon ya da efekt katalogda bulunamadı; benzer bir hareket seçtim.", bilinmeyen.join(" "))); }
        let mut x = vec![t(&["HOŞ", "GELDİNİZ"], Rol::Vurgu).tema("belir", "pirilti", "altin"), t(&MARKA, Rol::Imza)];
        if ist.giris.is_none() && ist.efekt.is_empty() && anim_istegi { x[0] = x[0].clone().tema("zipla", "konfeti", "gun-batimi"); }
        if let Ok(d) = kur(x, &ist, cesit, kat) { h.tarif = d.tarif; }
        h.anladim = ist.anladim.clone();
        return h;
    }
    if let Some(mut tf) = z.tarif {
        if uzunlari_bol(&mut tf) { varsayimlar.push("Uzun metni uzaktan okunsun diye kısa sahnelere böldüm.".into()); }
        if tf.sahneler.iter().all(|s| s.giris == "belir" && s.efekt.is_empty()) && ist.giris.is_none() {
            for (k, s) in tf.sahneler.iter_mut().enumerate() { let (g, e, p, f, _) = stil(if k == 0 { Rol::Vurgu } else { Rol::Bilgi }, cesit); s.giris = g.into(); if !e.is_empty() { s.efekt = vec![e.into()]; } if ist.palet.is_none() { s.palet = p.into(); } if s.font.is_empty() { s.font = f.into(); } }
        }
        varsayimlar.push("Yazdığınız sözcükleri ekrana olduğu gibi yazdım.".into());
        let mut notlar = b.notlar; notlar.extend(z.notlar);
        if let Ok(d) = dogrula(&tarif_json(&tf), kat) { notlar.extend(d.notlar); return Hazirlik { tarif: d.tarif, kaynak: "kural", notlar, anladim: z.anladim, anlasilmayan: z.anlasilmayan, varsayimlar }; }
    }
    sablon(varsayimlar, "Tariften ekrana yazılacak metin çıkaramadım; hazır karşılama videosu önerdim.")
}
