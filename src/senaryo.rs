//! Senaryo motoru: metin dosyasından (sahne sahne) video üretir.
use crate::anim::{self, Maske, Poz};
use crate::{efekt, tv, *};
use ab_glyph::FontRef;

#[derive(Clone, Debug, PartialEq)]
pub enum Gecis { Tv, Perde, Kapi, Iris, Slayt, Fade, Neon, Sil, Jaluzi, Mozaik, Dalga, Cubuk, Elmas, Kare, Saat, Capraz, Damali, Yanlar, Yok }
pub const GECISLER: [(&str, &str); 19] = [("yok", "Geçiş yok"), ("tv", "Tüplü TV"), ("perde", "Üstten ve alttan perde"), ("kapi", "Ortadan açılan kapı"), ("iris", "Daire iris"), ("slayt", "Üstten slayt kapağı"), ("fade", "Yumuşak karartma"), ("neon", "Neon yanması (tek sefer, yumuşak)"), ("sil", "Soldan sağa silme"), ("jaluzi", "Yatay jaluzi şeritleri"), ("mozaik", "Kare mozaik dağılması"), ("dalga", "Dalgalı kenarla yukarıdan iner"), ("cubuk", "Dikey çubuklar sırayla iner"), ("elmas", "Elmas (eşkenar dörtgen) iris"), ("kare", "Dikdörtgen iris"), ("saat", "Saat yelkovanı gibi dönerek kapanır"), ("capraz", "Çapraz yumuşak silme"), ("damali", "Dama tahtası kapanma"), ("yanlar", "İki yandan ortaya kapanır")];

#[derive(Clone, Debug)]
pub struct Sahne { pub metin: String, pub renk: String, pub giris: String, pub cikis: String, pub sure: f32, pub harf: f32, pub efekt: Vec<String>, pub font: String, pub derinlik: f32, pub egri: String }

#[derive(Clone, Debug)]
pub struct Senaryo { pub w: u32, pub h: u32, pub fiziksel: Option<(f32, f32)>, pub fps: u32, pub acilis: Gecis, pub kapanis: Gecis, pub koruma: bool, pub sahneler: Vec<Sahne> }

pub const GIRISLER: [&str; 12] = ["yok", "donerek", "zipla", "yandan", "zoom", "alttan", "ustten", "belir", "elastik", "daktilo", "sil", "toplan"];
pub const CIKISLAR: [&str; 8] = ["yok", "yukari", "asagi", "donerek", "sondur", "kucul", "sil", "dagil"];
pub const EFEKTLER: [&str; 10] = ["parlama", "pirilti", "cerceve", "cift-cerceve", "parcacik", "kivilcim", "kar", "yagmur", "halka", "kayan-yildiz"];

const ESKI_GIRIS_ACIK: [&str; 12] = ["Giriş animasyonu yok (hemen görünür)", "Dönerek büyüyerek girer", "Yukarıdan zıplayarak gelir", "Yandan dönerek kayar", "Dev boyuttan yerine küçülür", "Alttan yükselir", "Üstten iner", "Yavaşça belirir, satırlar sırayla", "Yaylanarak büyür", "Harf harf yazılır", "Soldan sağa açılır", "Parçacıklardan toplanır"];
const ESKI_CIKIS_ACIK: [&str; 8] = ["Çıkış animasyonu yok", "Yukarı kayarak çıkar", "Aşağı kayarak çıkar", "Dönerek kaybolur", "Yavaşça söner", "Dönerek küçülüp kaybolur", "Soldan sağa silinir", "Parçacıklara dağılır"];
/// (ad, açıklama, özel_mi) listeleri; arayüz açılır listeleri bundan beslenir.
/// Tüm efektler: (ad, açıklama). Önce yazı efektleri, sonra dekorlar.
pub fn efekt_listesi() -> Vec<(String, String)> { EFEKTLER.iter().zip(EFEKT_ACIK).map(|(a, b)| (a.to_string(), b.to_string())).chain(crate::dekor::ADLAR.iter().map(|d| (d.0.to_string(), d.1.to_string()))).collect() }
pub fn giris_listesi() -> Vec<(String, String, bool)> { GIRISLER.iter().zip(ESKI_GIRIS_ACIK).map(|(a, b)| (a.to_string(), b.to_string(), false)).chain(crate::animasyon::liste()).collect() }
pub fn cikis_listesi() -> Vec<(String, String, bool)> { CIKISLAR.iter().zip(ESKI_CIKIS_ACIK).map(|(a, b)| (a.to_string(), b.to_string(), false)).chain(crate::animasyon::liste()).collect() }
pub const EFEKT_ACIK: [&str; 10] = ["Yazıdan geçen ışık", "Pırıltı yıldızları", "Koşan ışıklı çerçeve", "Çift çizgi çerçeve", "Süzülen parçacıklar", "Uçan kıvılcımlar", "Yağan kar", "Eğik yağmur", "Merkezden yayılan halkalar", "Kayan yıldızlar"];

/// Geçerli giriş adları: eski özel durumlular + animasyon kütüphanesi (yerleşik ve özel).
pub fn giris_adlari() -> Vec<String> { GIRISLER.iter().map(|x| x.to_string()).chain(crate::animasyon::liste().into_iter().map(|x| x.0)).collect() }
pub fn cikis_adlari() -> Vec<String> { CIKISLAR.iter().map(|x| x.to_string()).chain(crate::animasyon::liste().into_iter().map(|x| x.0)).collect() }

fn gecis(s: &str) -> Result<Gecis, String> {
    match s { "tv" => Ok(Gecis::Tv), "perde" => Ok(Gecis::Perde), "kapi" => Ok(Gecis::Kapi), "iris" => Ok(Gecis::Iris), "slayt" => Ok(Gecis::Slayt), "fade" => Ok(Gecis::Fade), "neon" => Ok(Gecis::Neon), "sil" => Ok(Gecis::Sil), "jaluzi" => Ok(Gecis::Jaluzi), "mozaik" => Ok(Gecis::Mozaik), "dalga" => Ok(Gecis::Dalga), "cubuk" => Ok(Gecis::Cubuk), "elmas" => Ok(Gecis::Elmas), "kare" => Ok(Gecis::Kare), "saat" => Ok(Gecis::Saat), "capraz" => Ok(Gecis::Capraz), "damali" => Ok(Gecis::Damali), "yanlar" => Ok(Gecis::Yanlar), "yok" => Ok(Gecis::Yok), _ => Err(format!("bilinmeyen geçiş: {s} (tv|perde|kapi|iris|slayt|fade|neon|sil|jaluzi|mozaik|dalga|cubuk|elmas|kare|saat|capraz|damali|yanlar|yok)")) }
}

pub fn ayristir(metin: &str) -> Result<Senaryo, String> {
    let mut s = Senaryo { w: 980, h: 1800, fiziksel: None, fps: 25, acilis: Gecis::Yok, kapanis: Gecis::Yok, koruma: true, sahneler: vec![] };
    for (no, ham) in metin.lines().enumerate() {
        let satir = ham.split('#').next().unwrap().trim();
        if satir.is_empty() { continue; }
        let hata = |m: &str| format!("satır {}: {m}", no + 1);
        if let Some(r) = satir.strip_prefix("sahne") {
            let a = r.find('"').ok_or_else(|| hata("sahne metni tırnak içinde olmalı"))?;
            let b = r[a + 1..].find('"').ok_or_else(|| hata("kapanış tırnağı yok"))? + a + 1;
            let mut sh = Sahne { metin: dinamik_metin(&r[a + 1..b]), renk: "beyaz".into(), giris: "donerek".into(), cikis: "yok".into(), sure: 4.5, harf: 330.0, efekt: vec!["parlama".into()], font: String::new(), derinlik: 1.0, egri: String::new() };
            for kv in r[b + 1..].split_whitespace() {
                let (k, v) = kv.split_once('=').ok_or_else(|| hata(&format!("anahtar=değer bekleniyor: {kv}")))?;
                match k {
                    "renk" | "palet" => { if crate::stil::paletli(v).is_none() { return Err(hata(&format!("bilinmeyen renk/palet: {v}"))); } sh.renk = v.into() }
                    "font" => { if !crate::stil::font_var_mi(v) { return Err(hata(&format!("bilinmeyen font: {v} (olanlar: {})", crate::stil::FONTLAR.iter().map(|f| f.0).collect::<Vec<_>>().join(", ")))); } sh.font = v.into() }
                    "egri" => { if !v.is_empty() && !anim::EGRILER.iter().any(|e| e.0 == v) { return Err(hata(&format!("bilinmeyen eğri: {v} ({:?})", anim::EGRILER.iter().map(|e| e.0).collect::<Vec<_>>()))); } sh.egri = if v == "yok" { String::new() } else { v.into() } }
                    "derinlik" => { sh.derinlik = v.parse().map_err(|_| hata("derinlik sayı olmalı"))?; if !(0.0..=3.0).contains(&sh.derinlik) { return Err(hata("derinlik 0 ile 3 arası")); } }
                    "giris" => { if !giris_adlari().iter().any(|x| x == v) { return Err(hata(&format!("bilinmeyen giriş: {v} (olanlar: {})", giris_adlari().join(", ")))); } sh.giris = v.into() }
                    "cikis" => { if !cikis_adlari().iter().any(|x| x == v) { return Err(hata(&format!("bilinmeyen çıkış: {v} (olanlar: {})", cikis_adlari().join(", ")))); } sh.cikis = v.into() }
                    "sure" => sh.sure = v.parse().map_err(|_| hata("sure sayı olmalı"))?,
                    "harf" => sh.harf = v.parse().map_err(|_| hata("harf sayı olmalı"))?,
                    "efekt" => { sh.efekt = v.split(',').filter(|e| *e != "yok").map(String::from).collect(); for e in &sh.efekt { if !EFEKTLER.contains(&e.as_str()) && crate::dekor::ADLAR.iter().all(|d| d.0 != e) { return Err(hata(&format!("bilinmeyen efekt: {e}"))); } } }
                    _ => return Err(hata(&format!("bilinmeyen anahtar: {k}"))),
                }
            }
            if sh.sure < 1.5 { return Err(hata("sure en az 1.5 sn")); }
            if sh.metin.starts_with('@') && cizim::ADLAR.iter().all(|a| *a != sh.metin[1..].trim().to_lowercase()) { return Err(hata(&format!("bilinmeyen çizim: {} ({:?})", sh.metin, cizim::ADLAR))); }
            s.sahneler.push(sh);
            continue;
        }
        let mut p = satir.split_whitespace();
        let (k, v) = (p.next().unwrap(), p.next().ok_or_else(|| hata("değer eksik"))?);
        match k {
            "ekran" => { let (a, b) = v.split_once('x').ok_or_else(|| hata("ekran GENxYUK"))?; s.w = a.parse().map_err(|_| hata("genişlik"))?; s.h = b.parse().map_err(|_| hata("yükseklik"))?; }
            "fiziksel" => { let (a, b) = v.split_once('x').ok_or_else(|| hata("fiziksel GENxYUK (metre)"))?; let (a, b): (f32, f32) = (a.parse().map_err(|_| hata("fiziksel genişlik"))?, b.parse().map_err(|_| hata("fiziksel yükseklik"))?); if a <= 0.0 || b <= 0.0 { return Err(hata("fiziksel ölçü sıfırdan büyük olmalı")); } s.fiziksel = Some((a, b)); }
            "fps" => s.fps = v.parse().map_err(|_| hata("fps"))?,
            "acilis" => s.acilis = gecis(v).map_err(|e| hata(&e))?,
            "kapanis" => s.kapanis = gecis(v).map_err(|e| hata(&e))?,
            "koruma" => s.koruma = match v { "acik" => true, "kapali" => false, _ => return Err(hata("koruma acik veya kapali olmalı")) },
            _ => return Err(hata(&format!("bilinmeyen komut: {k}"))),
        }
    }
    if s.sahneler.is_empty() { return Err("en az bir sahne gerekli".into()); }
    Ok(s)
}

impl Senaryo {
    /// Çizim yüksekliği. `fiziksel` verilmişse kare pikselli sanal tuval: gerçek ekran oranında çizilir, çıkışta dikeyde piksel yüksekliğine indirilir.
    pub fn sanal_h(&self) -> u32 {
        match self.fiziksel { Some((gen, yuk)) => ((self.w as f32 * yuk / gen / 2.0).round() as u32 * 2).max(2), None => self.h }
    }
    /// Piksel oranı: >1 ise piksel dikeyde fiziksel olarak daha uzun.
    pub fn piksel_orani(&self) -> Option<f32> {
        self.fiziksel.map(|(g, y)| (y / self.h as f32) / (g / self.w as f32))
    }
}

/// RGB kareyi dikeyde `hv` satırdan `h` satıra alan ortalamasıyla indirir.
pub fn dikey_kucult(k: &[u8], w: u32, hv: u32, h: u32) -> Vec<u8> {
    if hv == h { return k.to_vec(); }
    let (w, hv, h) = (w as usize, hv as usize, h as usize);
    let oran = hv as f32 / h as f32;
    let mut c = vec![0u8; w * h * 3];
    for y in 0..h {
        let (a, b) = (y as f32 * oran, (y + 1) as f32 * oran);
        let (r0, r1) = (a as usize, (b.ceil() as usize).min(hv));
        for x in 0..w * 3 {
            let mut top = 0.0;
            for r in r0..r1 {
                let ag = ((r + 1) as f32).min(b) - (r as f32).max(a);
                top += k[r * w * 3 + x] as f32 * ag;
            }
            c[y * w * 3 + x] = (top / (b - a)).round().min(255.0) as u8;
        }
    }
    c
}

pub fn acilis_sn(g: &Gecis) -> f32 { match g { Gecis::Tv => tv::ACILIS_SN, Gecis::Yok => 0.0, _ => 1.2 } }
pub fn kapanis_sn(g: &Gecis) -> f32 { match g { Gecis::Tv => tv::KAPANIS_SN, Gecis::Yok => 0.0, _ => 1.2 } }
pub fn toplam_sn(s: &Senaryo) -> f32 { acilis_sn(&s.acilis) + s.sahneler.iter().map(|x| x.sure).sum::<f32>() + kapanis_sn(&s.kapanis) }

const GIRIS_SN: f32 = 0.9;
const CIKIS_SN: f32 = 0.9;

/// Kütüphane animasyonunu eğriyle çağırır; eğri 0..1 dışına taşarsa (geri/esnek) başlangıç ve son poz arasında doğrusal uzatır.
fn poz_egri(ad: &str, k: f32, c: &crate::animasyon::Ctx) -> Option<Poz> {
    if (0.0..=1.0).contains(&k) { return crate::animasyon::poz(ad, k, c); }
    let (bas, son) = (crate::animasyon::poz(ad, 0.0, c)?, crate::animasyon::poz(ad, 1.0, c)?);
    let (taban, u) = if k > 1.0 { (son, 1.0 - k) } else { (bas, -k) };
    let f = |a: f32, b: f32, c0: f32| c0 + (a - b) * u;
    Some(Poz { dx: f(bas.dx, son.dx, taban.dx), dy: f(bas.dy, son.dy, taban.dy), aci: f(bas.aci, son.aci, taban.aci), olcek: f(bas.olcek, son.olcek, taban.olcek).max(0.01), parlak: f(bas.parlak, son.parlak, taban.parlak).clamp(0.0, 1.0), derinlik: f(bas.derinlik, son.derinlik, taban.derinlik).max(0.0) })
}

/// Bir satırın pozu: `t` sahne içi zaman.
pub fn satir_poz(giris: &str, cikis: &str, egri: &str, t: f32, sure: f32, li: usize, d: f32, ekran: (u32, u32)) -> Poz {
    let (w, h) = (ekran.0 as f32, ekran.1 as f32);
    let pi = std::f32::consts::PI;
    let faz = li as f32 * 0.9;
    let sal = Poz { aci: ((t - GIRIS_SN).max(0.0) * 1.3 + faz).sin() * 0.26, olcek: 1.0 + 0.03 * (t * 3.0 + faz).sin(), parlak: 1.0, derinlik: d, dx: 0.0, dy: (t * 2.0 + faz).sin() * 6.0 };
    let kutuphane_cikis = crate::animasyon::bilgi(cikis);
    let cs = if cikis == "dagil" { 1.2 } else if let Some((sn, _)) = kutuphane_cikis { sn } else { CIKIS_SN };
    if cikis != "yok" && t > sure - cs {
        if kutuphane_cikis.is_some() {
            let k = ((t - (sure - cs)) / cs).clamp(0.0, 1.0);
            let c = crate::animasyon::Ctx { w, h, li, d, sal };
            if let Some(p) = poz_egri(cikis, 1.0 - anim::egri_k(egri, k), &c) { return p; }
        }
        let k = ((t - (sure - cs)) / cs).clamp(0.0, 1.0);
        let e = k * k * (3.0 - 2.0 * k);
        let mut p = sal;
        match cikis {
            "yukari" => { p.dy = -e * h * 1.05 - k * li as f32 * 60.0; p.aci = -e * pi * 0.25; }
            "asagi" => { p.dy = e * h * 1.05 + k * li as f32 * 60.0; p.aci = e * pi * 0.25; }
            "sondur" => { p.parlak = 1.0 - k; }
            "kucul" => { p.olcek = (1.0 - e).max(0.02); p.aci = e * pi * 2.0; p.parlak = 1.0 - k * k; }
            "sil" => {}
            "dagil" => { p.parlak = 0.0; }
            _ => { p.aci = -e * pi * 0.5; p.parlak = 1.0 - k; p.olcek = 1.0 + 0.25 * k; }
        }
        return p;
    }
    if giris == "toplan" {
        let ti = t - li as f32 * 0.15;
        if ti < 1.4 { return Poz { parlak: 0.0, ..sal }; }
        if ti < 1.7 { return Poz { derinlik: d * anim::yumusak((ti - 1.4) / 0.3), ..sal }; }
        return sal;
    }
    if let Some((gsn, ggec)) = crate::animasyon::bilgi(giris) {
        let ti = t - li as f32 * ggec;
        if ti < 0.0 { return Poz { parlak: 0.0, ..sal }; }
        if ti < gsn {
            let c = crate::animasyon::Ctx { w, h, li, d, sal };
            if let Some(p) = poz_egri(giris, anim::egri_k(egri, ti / gsn), &c) { return p; }
        }
        return sal;
    }
    if giris != "yok" {
        let ti = t - li as f32 * if giris == "belir" { 0.45 } else { 0.15 };
        if ti < 0.0 { return Poz { parlak: 0.0, ..sal }; }
        if ti < GIRIS_SN {
            let k = ti / GIRIS_SN;
            let mut p = sal;
            match giris {
                "donerek" => { p.aci = (1.0 - anim::geri(k)) * pi * 0.5 + (1.0 - anim::yumusak(k)) * pi * 0.5; p.olcek = 0.4 + 0.6 * anim::geri(k); }
                "zipla" => { p.dy = -(1.0 - anim::ziplama(k)) * h * 0.7; p.aci = 0.0; }
                "yandan" => { p.aci = (1.0 - anim::yumusak(k)) * pi * 1.5; p.olcek = 0.6 + 0.4 * anim::yumusak(k); p.dx = (1.0 - anim::yumusak(k)) * w * 1.1; }
                "zoom" => { p.olcek = 1.0 + 2.2 * (1.0 - anim::geri(k)); p.aci = 0.0; }
                "alttan" => { p.dy = (1.0 - anim::geri(k)) * h * 1.05; p.aci = (1.0 - k) * pi * 0.5 + p.aci * k; }
                "belir" => { p.aci = p.aci * k; }
                "elastik" => { p.olcek = anim::elastik(k).max(0.01); }
                "sil" | "daktilo" => {}
                _ => { p.dy = -(1.0 - anim::geri(k)) * h * 1.05; p.aci = -(1.0 - k) * pi * 0.5 + p.aci * k; } // ustten
            }
            p.parlak = match giris { "belir" => k, "sil" | "daktilo" | "elastik" => 1.0, _ => (0.4 + 0.6 * k).min(1.0) };
            p.derinlik = match giris { "belir" | "sil" | "daktilo" | "elastik" => d, _ => d * anim::yumusak(k) };
            return p;
        }
    }
    sal
}

/// Silme/daktilo için yazının görünür sol-sağ aralığı.
pub fn satir_kirp(giris: &str, cikis: &str, t: f32, sure: f32, li: usize) -> (f32, f32) {
    let mut k = (0.0, 1.0);
    let ti = t - li as f32 * 0.15;
    if giris == "sil" || giris == "daktilo" {
        let dur = if giris == "daktilo" { 1.4 } else { 0.9 };
        if ti < dur { let p = (ti / dur).clamp(0.0, 1.0); k.1 = if giris == "daktilo" { (p * 12.0).floor() / 12.0 } else { anim::yumusak(p) }; }
    }
    if cikis == "sil" && t > sure - CIKIS_SN { k.0 = anim::yumusak(((t - (sure - CIKIS_SN)) / CIKIS_SN).clamp(0.0, 1.0)); }
    k
}

/// Perde: üstten ve alttan siyah kepenk; kenarda ince parlak çizgi. `kapali`: 0=açık, 1=tam kapalı.
pub fn perde(t: &mut [u8], w: u32, h: u32, kapali: f32, renk: Renk) {
    let kapa = ((h as f32 / 2.0) * kapali.clamp(0.0, 1.0)) as u32;
    if kapa == 0 { return; }
    let cizgi = 8u32;
    for y in 0..h {
        let (ust, alt) = (y < kapa, y >= h - kapa);
        let kenar = (y >= kapa.saturating_sub(cizgi) && y < kapa) || (y >= h - kapa && y < h - kapa + cizgi);
        if !(ust || alt || kenar) { continue; }
        for x in 0..w {
            let i = ((y * w + x) * 3) as usize;
            if kenar && kapa < h / 2 { t[i] = renk.0; t[i + 1] = renk.1; t[i + 2] = renk.2; }
            else { t[i] = 0; t[i + 1] = 0; t[i + 2] = 0; }
        }
    }
}

/// Açılış/kapanış kaplaması. `kapali`: 0 açık, 1 tam kapalı (siyah). Tv ayrıca işlenir.
/// Sahne metnindeki canlı öğeleri video üretildiği ana göre çözer: `{tarih}` (gg.aa.yyyy), `{gun:2026-12-31}` (o tarihe kalan gün sayısı).
pub fn dinamik_metin(m: &str) -> String {
    if !m.contains('{') { return m.to_string(); }
    let bugun = bugun_gun_no();
    let (y, a, g) = tarih_ayir(bugun);
    let mut o = m.replace("{tarih}", &format!("{g:02}.{a:02}.{y}"));
    while let Some(i) = o.find("{gun:") {
        let Some(j) = o[i..].find('}') else { break };
        let ic = o[i + 5..i + j].to_string();
        let kalan = ic.split('-').map(|x| x.parse::<i64>().ok()).collect::<Option<Vec<_>>>().filter(|v| v.len() == 3).map(|v| (gun_no(v[0], v[1], v[2]) - bugun).max(0));
        o.replace_range(i..i + j + 1, &kalan.map(|k| k.to_string()).unwrap_or_else(|| "?".into()));
    }
    o
}
/// 1970-01-01'den beri gün sayısı (sivil takvim).
fn gun_no(y: i64, m: i64, d: i64) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = y.div_euclid(400);
    let yoe = y - era * 400;
    let doy = (153 * (if m > 2 { m - 3 } else { m + 9 }) + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe - 719468
}
fn tarih_ayir(z: i64) -> (i64, i64, i64) {
    let z = z + 719468;
    let era = z.div_euclid(146097);
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    (if m <= 2 { y + 1 } else { y }, m, d)
}
fn bugun_gun_no() -> i64 {
    let sn = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs() as i64).unwrap_or(0);
    (sn + 3 * 3600).div_euclid(86400) // Türkiye saati (UTC+3)
}

pub fn kaplama(g: &Gecis, t: &mut [u8], w: u32, h: u32, kapali: f32, renk: Renk) {
    let kapali = kapali.clamp(0.0, 1.0);
    if kapali <= 0.0 { return; }
    let cizgi = (w / 120).max(6);
    match g {
        Gecis::Perde => perde(t, w, h, kapali, renk),
        Gecis::Fade => for b in t.iter_mut() { *b = (*b as f32 * (1.0 - kapali)) as u8; },
        Gecis::Neon => { // iki kademeli, yumuşak: önce yarım, sonra tam yanar (flaş yok)
            let y = 1.0 - kapali; let f = if y < 0.5 { y * 0.8 } else { 0.4 + (y - 0.5) * 1.2 };
            for b in t.iter_mut() { *b = (*b as f32 * f.clamp(0.0, 1.0)) as u8; }
        }
        Gecis::Slayt => {
            let alt = (h as f32 * kapali) as u32;
            for y in 0..h.min(alt + cizgi) { for x in 0..w { let i = ((y * w + x) * 3) as usize; if y < alt { t[i] = 0; t[i + 1] = 0; t[i + 2] = 0; } else if kapali < 1.0 { t[i] = renk.0; t[i + 1] = renk.1; t[i + 2] = renk.2; } } }
        }
        Gecis::Kapi => {
            let pw = (w as f32 / 2.0 * kapali) as u32;
            for y in 0..h { for x in 0..w {
                let (sol, sag) = (x < pw, x >= w - pw);
                let kenar = kapali < 1.0 && ((x >= pw && x < pw + cizgi) || (x + pw >= w.saturating_sub(cizgi) && x + pw < w));
                let i = ((y * w + x) * 3) as usize;
                if sol || sag { t[i] = 0; t[i + 1] = 0; t[i + 2] = 0; } else if kenar && pw > 0 { t[i] = renk.0; t[i + 1] = renk.1; t[i + 2] = renk.2; }
            } }
        }
        Gecis::Iris => {
            let (cx, cy) = (w as f32 / 2.0, h as f32 / 2.0);
            let maks = (cx * cx + cy * cy).sqrt();
            let r = maks * (1.0 - kapali);
            let yum = (w as f32 * 0.03).max(8.0);
            for y in 0..h { for x in 0..w {
                let d = ((x as f32 - cx).powi(2) + (y as f32 - cy).powi(2)).sqrt();
                if d <= r { continue; }
                let a = ((d - r) / yum).clamp(0.0, 1.0);
                let i = ((y * w + x) * 3) as usize;
                for c in 0..3 { t[i + c] = (t[i + c] as f32 * (1.0 - a)) as u8; }
            } }
        }
        Gecis::Sil => {
            let sinir = (w as f32 * kapali) as u32;
            for y in 0..h { for x in 0..w.min(sinir + cizgi) { let i = ((y * w + x) * 3) as usize; if x < sinir { t[i] = 0; t[i + 1] = 0; t[i + 2] = 0; } else if kapali < 1.0 { t[i] = renk.0; t[i + 1] = renk.1; t[i + 2] = renk.2; } } }
        }
        Gecis::Jaluzi => {
            let serit = (h / 14).max(8);
            for y in 0..h {
                let u = (y % serit) as f32 / serit as f32;
                if u < kapali { for x in 0..w { let i = ((y * w + x) * 3) as usize; t[i] = 0; t[i + 1] = 0; t[i + 2] = 0; } }
            }
        }
        Gecis::Mozaik => {
            let k = (w / 10).max(8);
            for y in 0..h { for x in 0..w {
                let (bx, by) = (x / k, y / k);
                let e = crate::efekt::hash(bx * 131 + by * 977 + 7);
                if e < kapali { let i = ((y * w + x) * 3) as usize; t[i] = 0; t[i + 1] = 0; t[i + 2] = 0; }
            } }
        }
        Gecis::Dalga => {
            let genlik = w as f32 * 0.08;
            for x in 0..w {
                let sinir = (h as f32 + 2.0 * genlik) * kapali - genlik + (x as f32 / w as f32 * 6.2832 * 2.0).sin() * genlik * (1.0 - (2.0 * kapali - 1.0).abs());
                for y in 0..h {
                    let d = sinir - y as f32;
                    let i = ((y * w + x) * 3) as usize;
                    if d >= 0.0 { t[i] = 0; t[i + 1] = 0; t[i + 2] = 0; }
                    else if d > -(cizgi as f32) && kapali < 1.0 { t[i] = renk.0; t[i + 1] = renk.1; t[i + 2] = renk.2; }
                }
            }
        }
        Gecis::Cubuk => {
            let n = 8u32; let bw = (w / n).max(1);
            for x in 0..w {
                let b = (x / bw).min(n - 1) as f32;
                let yerel = ((kapali * (1.0 + 0.6) - b / n as f32 * 0.6) ).clamp(0.0, 1.0);
                let alt = (h as f32 * yerel) as u32;
                for y in 0..alt.min(h) { let i = ((y * w + x) * 3) as usize; t[i] = 0; t[i + 1] = 0; t[i + 2] = 0; }
            }
        }
        Gecis::Elmas => {
            let (cx, cy) = (w as f32 / 2.0, h as f32 / 2.0);
            let r = 1.0 - kapali;
            let yum = 0.05;
            for y in 0..h { for x in 0..w {
                let d = (x as f32 - cx).abs() / cx + (y as f32 - cy).abs() / cy;
                let dd = d * 0.5;
                if dd <= r { continue; }
                let a = ((dd - r) / yum).clamp(0.0, 1.0);
                let i = ((y * w + x) * 3) as usize;
                for c in 0..3 { t[i + c] = (t[i + c] as f32 * (1.0 - a)) as u8; }
            } }
        }
        Gecis::Kare => {
            let (cx, cy) = (w as f32 / 2.0, h as f32 / 2.0);
            let r = 1.0 - kapali;
            let yum = 0.04;
            for y in 0..h { for x in 0..w {
                let d = ((x as f32 - cx).abs() / cx).max((y as f32 - cy).abs() / cy);
                if d <= r { continue; }
                let a = ((d - r) / yum).clamp(0.0, 1.0);
                let i = ((y * w + x) * 3) as usize;
                for c in 0..3 { t[i + c] = (t[i + c] as f32 * (1.0 - a)) as u8; }
            } }
        }
        Gecis::Saat => {
            let (cx, cy) = (w as f32 / 2.0, h as f32 / 2.0);
            let acik = 1.0 - kapali; // açık kalan oran (saat yönünde)
            for y in 0..h { for x in 0..w {
                let mut a = (x as f32 - cx).atan2(-(y as f32 - cy)); if a < 0.0 { a += std::f32::consts::TAU; }
                let f = a / std::f32::consts::TAU;
                if f >= acik { let i = ((y * w + x) * 3) as usize; t[i] = 0; t[i + 1] = 0; t[i + 2] = 0; }
            } }
        }
        Gecis::Capraz => {
            let yum = 0.08;
            for y in 0..h { for x in 0..w {
                let d = (x as f32 / w as f32 + y as f32 / h as f32) * 0.5;
                let e = d - (1.0 - kapali) * (1.0 + yum) + yum;
                if e <= 0.0 { continue; }
                let a = (e / yum).clamp(0.0, 1.0);
                let i = ((y * w + x) * 3) as usize;
                for c in 0..3 { t[i + c] = (t[i + c] as f32 * (1.0 - a)) as u8; }
            } }
        }
        Gecis::Damali => {
            let n = 6u32; let bw = (w / n).max(1); let bh = (h / (n * 2)).max(1);
            for y in 0..h { for x in 0..w {
                let (cx, cy) = (x / bw, y / bh);
                let sira = ((cx + cy) % 2) as f32 * 0.5;
                let yerel = ((kapali * 1.5 - sira)).clamp(0.0, 1.0);
                if yerel >= 0.999 { let i = ((y * w + x) * 3) as usize; t[i] = 0; t[i + 1] = 0; t[i + 2] = 0; }
                else if yerel > 0.0 { let i = ((y * w + x) * 3) as usize; for c in 0..3 { t[i + c] = (t[i + c] as f32 * (1.0 - yerel)) as u8; } }
            } }
        }
        Gecis::Yanlar => {
            let k = (w as f32 / 2.0 * kapali) as u32;
            for y in 0..h { for x in 0..w {
                if x < k || x >= w - k.min(w) { let i = ((y * w + x) * 3) as usize; t[i] = 0; t[i + 1] = 0; t[i + 2] = 0; }
            } }
        }
        _ => {}
    }
}

fn yerlesim(font: &FontRef, w: u32, h: u32, s: &Sahne) -> (Vec<Maske>, f32, Vec<(f32, bool)>) {
    if let Some(ad) = s.metin.strip_prefix('@') {
        return (vec![cizim::maske(ad.trim().to_lowercase().as_str(), w, h, w.min(h) as f32 * 0.9).unwrap()], w as f32 * 0.06, vec![]);
    }
    if s.metin.contains('|') { return afis_yerlesim(font, w, h, s); }
    let satir = dikey_satirlar(&s.metin);
    let mut boylar: Vec<f32> = satir.iter().map(|x| (w as f32 * 0.84 / genislik(font, x, 100.0).max(1.0) * 100.0).min(s.harf)).collect();
    let toplam: f32 = boylar.iter().map(|b| satir_yuksekligi(font, *b)).sum();
    if toplam > h as f32 * 0.8 { let k = h as f32 * 0.8 / toplam; for b in boylar.iter_mut() { *b *= k; } }
    let sh: Vec<f32> = boylar.iter().map(|b| satir_yuksekligi(font, *b)).collect();
    let mut y = (h as f32 - sh.iter().sum::<f32>()) / 2.0;
    let mut v = vec![];
    for (k, s1) in satir.iter().enumerate() {
        v.push(anim::kaydir(&anim::maske(font, w, h, &[s1.clone()], boylar[k]), (y + sh[k] / 2.0 - h as f32 / 2.0) as i32));
        y += sh[k];
    }
    (v, boylar.iter().cloned().fold(0.0, f32::max) * 0.14, boylar.iter().map(|b| (*b, true)).collect())
}

/// Açık satırlı düzen: "PEŞİN|FİYATINA|*3|TAKSİT". `*` ile başlayan satır büyük rakamdır ve iki yanına çift çizgi gelir.
fn afis_yerlesim(font: &FontRef, w: u32, h: u32, s: &Sahne) -> (Vec<Maske>, f32, Vec<(f32, bool)>) {
    // satır türleri: 0 düz, 1 büyük rakam + çift çizgi (*), 2 büyük rakam + tek çizgi (+), 3 ince yatay çizgi (~)
    let satirlar: Vec<(String, u8)> = s.metin.split('|').map(|x| { let x = x.trim(); match x.chars().next() {
        Some('*') => (x[1..].trim().to_string(), 1), Some('+') => (x[1..].trim().to_string(), 2), Some('~') => (String::new(), 3), _ => (x.to_string(), 0) } }).collect();
    let yazili = |k: u8| k == 0;
    let buyuk = |k: u8| k == 1 || k == 2;
    let fit = |t: &str| (w as f32 * 0.88 / genislik(font, t, 100.0).max(1.0) * 100.0).min(s.harf);
    let ref_boy = satirlar.iter().filter(|x| yazili(x.1)).map(|x| fit(&x.0)).fold(f32::MAX, f32::min).min(s.harf);
    let mut boylar: Vec<f32> = satirlar.iter().map(|(t, k)| if buyuk(*k) { (ref_boy * 3.6).min(s.harf * 1.8).min(h as f32 * 0.34) } else if *k == 3 { 0.0 } else { fit(t) }).collect();
    // art arda gelen düz satırlar aynı boyda olsun (en küçüğüne eşitlenir): PEŞİN / FİYATINA gibi
    let mut i = 0;
    while i < satirlar.len() {
        if !yazili(satirlar[i].1) { i += 1; continue; }
        let mut j = i;
        while j < satirlar.len() && yazili(satirlar[j].1) { j += 1; }
        let en_kucuk = boylar[i..j].iter().cloned().fold(f32::MAX, f32::min);
        for b in &mut boylar[i..j] { *b = en_kucuk; }
        i = j;
    }
    let cizgi_h = (w as f32 * 0.008).max(6.0);
    let yuk = |b: &[f32]| -> Vec<f32> { (0..b.len()).map(|k| if satirlar[k].1 == 3 { cizgi_h } else { satir_yuksekligi(font, b[k]) }).collect() };
    let toplam: f32 = yuk(&boylar).iter().sum();
    if toplam > h as f32 * 0.8 { let k = h as f32 * 0.8 / toplam; for b in boylar.iter_mut() { *b *= k; } }
    let sh: Vec<f32> = yuk(&boylar);
    let bosluk = h as f32 * 0.05; // uzun dikey ekranda satırlar nefes alsın
    let mut y = (h as f32 - sh.iter().sum::<f32>() - bosluk * (satirlar.len() - 1) as f32) / 2.0;
    let mut v = vec![];
    for (k, (t, tur)) in satirlar.iter().enumerate() {
        let orta = y + sh[k] / 2.0;
        if *tur == 3 {
            let mut m = anim::kaydir(&anim::maske(font, w, h, &[" ".to_string()], 20.0), 0);
            let (x0, x1) = ((w as f32 * 0.25) as u32, (w as f32 * 0.75) as u32);
            anim::dikdortgen(&mut m, x0, y as u32, x1, (y + cizgi_h) as u32);
            v.push(m);
            y += sh[k] + bosluk;
            continue;
        }
        let mut m = anim::kaydir(&anim::maske(font, w, h, &[t.clone()], boylar[k]), (orta - h as f32 / 2.0) as i32);
        if buyuk(*tur) {
            let gen = genislik(font, t, boylar[k]);
            let (kal, ara) = ((boylar[k] * 0.035).max(6.0) as u32, (boylar[k] * 0.06).max(10.0) as u32);
            let (sol_bit, sag_bas) = ((w as f32 / 2.0 - gen / 2.0 - boylar[k] * 0.18) as u32, (w as f32 / 2.0 + gen / 2.0 + boylar[k] * 0.18) as u32);
            let (bas, son) = ((w as f32 * 0.10) as u32, (w as f32 * 0.90) as u32);
            let cy = orta as u32;
            let ofsetler: Vec<u32> = if *tur == 1 { vec![cy - (ara + kal) / 2 - kal / 2, cy + (ara + kal) / 2 - kal / 2] } else { vec![cy - kal / 2] };
            for off in ofsetler {
                if sol_bit > bas { anim::dikdortgen(&mut m, bas, off, sol_bit, off + kal); }
                if son > sag_bas { anim::dikdortgen(&mut m, sag_bas, off, son, off + kal); }
            }
        }
        v.push(m);
        y += sh[k] + bosluk;
    }
    let bilgi = satirlar.iter().zip(&boylar).filter(|(sa, _)| sa.1 != 3).map(|(sa, b)| (*b, sa.1 == 0 || sa.1 == 1 || sa.1 == 2)).collect();
    (v, boylar.iter().cloned().fold(0.0, f32::max) * 0.08, bilgi)
}

/// Dikey ekran için satırlara böl: her kelime ayrı satır; kısa kelimeler sonrakiyle birleşir.
pub fn dikey_satirlar(s: &str) -> Vec<String> {
    let kel: Vec<&str> = s.split_whitespace().collect();
    let mut out: Vec<String> = vec![];
    let mut i = 0;
    while i < kel.len() {
        let mut satir = kel[i].to_string();
        while i + 1 < kel.len() && (satir.chars().count() <= 2 || (satir.chars().count() + 1 + kel[i + 1].chars().count()) <= 7) {
            i += 1; satir.push(' '); satir.push_str(kel[i]);
        }
        out.push(satir);
        i += 1;
    }
    out
}

pub struct Hazir { pub maskeler: Vec<Maske>, pub d: f32, pub yuz: Renk, pub yan: Renk, pub zemin: Renk, pub vurgu: Renk, pub boylar: Vec<(f32, bool)> }

pub fn hazirla(font: &FontRef, s: &Senaryo) -> Vec<Hazir> {
    let tvli = s.acilis == Gecis::Tv || s.kapanis == Gecis::Tv;
    s.sahneler.iter().map(|sh| {
        let pl = crate::stil::paletli(&sh.renk).unwrap();
        let sf = crate::stil::font(&sh.font);
        let (maskeler, d, boylar) = yerlesim(sf.as_ref().unwrap_or(font), s.w, s.sanal_h(), sh);
        Hazir { maskeler, d: d * sh.derinlik, yuz: pl.yuz, yan: pl.yan, zemin: if tvli { Renk(18, 20, 24) } else { pl.zemin }, vurgu: pl.vurgu, boylar }
    }).collect()
}

/// t anındaki RGB kare.
pub fn kare_uret(s: &Senaryo, hz: &[Hazir], t: f32, no: u32) -> Vec<u8> {
    let (w, h) = (s.w, s.sanal_h());
    let ac = acilis_sn(&s.acilis);
    let toplam = toplam_sn(s);
    let ka = kapanis_sn(&s.kapanis);
    // aktif sahne
    let mut bas = ac;
    let mut idx = 0;
    for (i, sh) in s.sahneler.iter().enumerate() { if t >= bas { idx = i; } if i + 1 < s.sahneler.len() { bas += sh.sure; } }
    let bas_i: f32 = ac + s.sahneler[..idx].iter().map(|x| x.sure).sum::<f32>();
    let yerel = (t - bas_i).max(0.0);
    let (sh, z) = (&s.sahneler[idx], &hz[idx]);
    let mut k = anim::bos_kare(w, h, z.zemin);
    // ilk sahne açılışla birlikte zaten görünür: girişi açılış geçişi yapar
    let giris = if idx == 0 && s.acilis != Gecis::Yok { "yok" } else { sh.giris.as_str() };
    if sh.efekt.iter().any(|e| e == "parcacik") { efekt::parcaciklar(&mut k, w, h, t, 60, z.vurgu, idx as u32 + 7); }
    if sh.efekt.iter().any(|e| e == "kar") { efekt::kar(&mut k, w, h, t, 90, idx as u32 + 11); }
    if sh.efekt.iter().any(|e| e == "yagmur") { efekt::yagmur(&mut k, w, h, t, 70, z.vurgu, idx as u32 + 13); }
    if sh.efekt.iter().any(|e| e == "halka") { efekt::halkalar(&mut k, w, h, t, z.vurgu); }
    if sh.efekt.iter().any(|e| e == "kayan-yildiz") { efekt::kayan_yildizlar(&mut k, w, h, t, z.vurgu, idx as u32 + 17); }
    for e in &sh.efekt { crate::dekor::ciz(e, &mut k, w, h, t, z.vurgu, idx as u32 + 5); }
    if sh.efekt.iter().any(|e| e == "kivilcim") { efekt::kivilcimlar(&mut k, w, h, t, 40, z.vurgu, idx as u32 + 3); }
    for (li, m) in z.maskeler.iter().enumerate() {
        let ti = yerel - li as f32 * 0.15;
        if giris == "toplan" && (0.0..1.4).contains(&ti) { anim::dagilma(&mut k, m, z.yuz, 1.0 - ti / 1.4, (idx * 10 + li) as u32); continue; }
        if sh.cikis == "dagil" && yerel > sh.sure - 1.2 { anim::dagilma(&mut k, m, z.yuz, ((yerel - (sh.sure - 1.2)) / 1.2).clamp(0.0, 1.0), (idx * 10 + li) as u32 + 50); continue; }
        let p = satir_poz(giris, &sh.cikis, &sh.egri, yerel, sh.sure, li, z.d, (w, h));
        anim::kare3d_kirpmali(&mut k, m, z.yuz, z.yan, &p, satir_kirp(giris, &sh.cikis, yerel, sh.sure, li));
    }
    if sh.efekt.iter().any(|e| e == "parlama") {
        let konum = (yerel - 1.0) / 1.3;
        efekt::isik_supurme(&mut k, w, h, z.zemin, if (0.0..=1.0).contains(&konum) { konum } else { -1.0 });
    }
    if sh.efekt.iter().any(|e| e == "cerceve") { efekt::kosan_cerceve(&mut k, w, h, t, z.vurgu, (w / 70).max(6), (w / 25).max(10)); }
    if sh.efekt.iter().any(|e| e == "cift-cerceve") { efekt::cift_cerceve(&mut k, w, h, z.vurgu, (w / 240).max(3), (w / 22).max(12)); }
    if sh.efekt.iter().any(|e| e == "pirilti") { efekt::piriltilar(&mut k, w, h, t, 18, Renk(255, 255, 255), idx as u32 + 1); }
    let (kapat_basla, acik_bitis) = (toplam - ka, ac);
    let normal = tv::Tup { sv: 1.0, sh: 1.0, flas: 0.0, parlak: 0.985 + 0.015 * (no as f32 * 0.9).sin(), gurultu: 0.0 };
    if s.acilis == Gecis::Tv && t < acik_bitis { return tv::tup(&k, w, h, tv::acilis(t), no); }
    if s.kapanis == Gecis::Tv && t > kapat_basla { return tv::tup(&k, w, h, tv::kapanis(t - kapat_basla), no); }
    if t < acik_bitis { kaplama(&s.acilis, &mut k, w, h, 1.0 - anim::yumusak(t / acik_bitis), z.yuz); }
    if t > kapat_basla { kaplama(&s.kapanis, &mut k, w, h, anim::yumusak((t - kapat_basla) / ka), z.yuz); }
    if s.acilis == Gecis::Tv || s.kapanis == Gecis::Tv { return tv::tup(&k, w, h, normal, no); }
    k
}

#[cfg(test)]
mod t {
    use super::*;
    const ORNEK: &str = "ekran 490x900\nfps 10\nacilis tv\nkapanis perde\nsahne \"PEŞİN FİYATINA 3 TAKSİT\" renk=beyaz giris=zipla cikis=yukari sure=4\nsahne \"TERZİ ADEM ALTUN\" renk=altin giris=alttan efekt=parlama,pirilti # not\n";
    #[test]
    fn ayristirir() {
        let s = ayristir(ORNEK).unwrap();
        assert_eq!((s.w, s.h, s.fps), (490, 900, 10));
        assert_eq!(s.sahneler.len(), 2);
        assert_eq!(s.sahneler[1].efekt, vec!["parlama", "pirilti"]);
        assert!((toplam_sn(&s) - (1.5 + 4.0 + 4.5 + 1.2)).abs() < 1e-3);
    }
    #[test]
    fn fiziksel_olcu_ve_kucultme() {
        let s = ayristir("ekran 980x1800\nfiziksel 8x25\nsahne \"A\"").unwrap();
        assert_eq!(s.sanal_h(), 3062);
        assert!((s.piksel_orani().unwrap() - 1.7).abs() < 0.05);
        let k = vec![100u8; 10 * 30 * 3];
        let c = dikey_kucult(&k, 10, 30, 17);
        assert_eq!(c.len(), 10 * 17 * 3);
        assert!(c.iter().all(|&b| b == 100));
        assert_eq!(ayristir("sahne \"A\"").unwrap().sanal_h(), 1800);
    }
    #[test]
    fn hata_mesajlari() {
        assert!(ayristir("sahne \"A\" giris=xyzzy").unwrap_err().contains("bilinmeyen giriş"));
        assert!(ayristir("acilis tvv\nsahne \"A\"").is_err());
        assert!(ayristir("ekran 10x10").unwrap_err().contains("sahne"));
        assert!(ayristir("sahne \"@yok\"").unwrap_err().contains("çizim"));
    }
    #[test]
    fn kare_uretir_tum_gecisler() {
        let f = FontRef::try_from_slice(include_bytes!("../fonts/InterDisplay-Bold.otf")).unwrap();
        let s = ayristir(ORNEK).unwrap();
        let hz = hazirla(&f, &s);
        for t in [0.2, 1.0, 3.5, 6.0, 9.0, 10.4] {
            let k = kare_uret(&s, &hz, t, 3);
            assert_eq!(k.len(), (s.w * s.h * 3) as usize);
        }
        // sahne 1 ortasında yazı var
        assert!(kare_uret(&s, &hz, 3.0, 1).iter().any(|&b| b > 150));
    }
    #[test]
    fn yeni_gecis_ve_efektler_cizilir() {
        let f = FontRef::try_from_slice(include_bytes!("../fonts/InterDisplay-Bold.otf")).unwrap();
        for (g, _) in GECISLER.iter() {
            let m = format!("ekran 120x240\nacilis {g}\nkapanis {g}\nsahne \"TAA\" sure=3 harf=80\n");
            let s = ayristir(&m).unwrap_or_else(|e| panic!("{g}: {e}"));
            let hz = hazirla(&f, &s);
            let toplam = toplam_sn(&s);
            for t in [0.05, toplam * 0.5, toplam - 0.05] { assert_eq!(kare_uret(&s, &hz, t, 2).len(), (s.w * s.h * 3) as usize, "{g}"); }
        }
        for (e, _) in efekt_listesi() {
            let m = format!("ekran 120x240\nsahne \"TAA\" sure=3 harf=80 efekt={e}\n");
            let s = ayristir(&m).unwrap_or_else(|er| panic!("{e}: {er}"));
            let hz = hazirla(&f, &s);
            let k = kare_uret(&s, &hz, 2.0, 1);
            assert!(k.iter().any(|&b| b > 100), "{e}: kare boş");
        }
    }
}
