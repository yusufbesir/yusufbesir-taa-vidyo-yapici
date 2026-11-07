//! Animasyon kütüphanesi: yerleşik (kodla yazılmış) animasyonlar ve metin dosyasından yüklenen
//! anahtar kareli ÖZEL animasyonlar. Her animasyon hem giriş hem çıkış olarak kullanılır
//! (çıkışta ters oynatılır). Yeni animasyon eklemek için `animasyonlar/*.txt` dosyasına anahtar kareler yazmak yeter.
use crate::anim::{self, Poz};
use crate::efekt::hash;
use std::path::PathBuf;
use std::sync::RwLock;

/// Animasyon fonksiyonuna verilen bağlam: ekran ölçüsü, satır sırası, yazı derinliği ve salınımlı dinlenme pozu.
pub struct Ctx { pub w: f32, pub h: f32, pub li: usize, pub d: f32, pub sal: Poz }

pub struct Tanim { pub ad: &'static str, pub acik: &'static str, pub sure: f32, pub gecikme: f32, pub f: fn(f32, &Ctx) -> Poz }

const PI: f32 = std::f32::consts::PI;

fn ys(k: f32) -> f32 { anim::yumusak(k) }

pub const YERLESIK: &[Tanim] = &[
    Tanim { ad: "dusus", acik: "Yukarıdan düşer ve zıplayarak oturur", sure: 1.1, gecikme: 0.18, f: |k, c| Poz { dy: -(1.0 - anim::ziplama(k)) * c.h * 0.8, parlak: (0.3 + k).min(1.0), ..c.sal } },
    Tanim { ad: "sagdan", acik: "Sağdan kayarak girer, hafif aşar", sure: 0.9, gecikme: 0.15, f: |k, c| Poz { dx: (1.0 - anim::geri(k)) * c.w * 1.1, ..c.sal } },
    Tanim { ad: "soldan", acik: "Soldan kayarak girer, hafif aşar", sure: 0.9, gecikme: 0.15, f: |k, c| Poz { dx: -(1.0 - anim::geri(k)) * c.w * 1.1, ..c.sal } },
    Tanim { ad: "merdiven", acik: "Satırlar sırayla bir soldan bir sağdan girer", sure: 0.9, gecikme: 0.25, f: |k, c| Poz { dx: (1.0 - anim::geri(k)) * c.w * 1.1 * if c.li % 2 == 0 { -1.0 } else { 1.0 }, ..c.sal } },
    Tanim { ad: "cevir", acik: "Dikey eksende tam tur dönerek yerine gelir", sure: 1.1, gecikme: 0.15, f: |k, c| Poz { aci: (1.0 - ys(k)) * 2.0 * PI + c.sal.aci * k, olcek: 0.6 + 0.4 * ys(k), parlak: (0.4 + k).min(1.0), ..c.sal } },
    Tanim { ad: "salincak", acik: "Sallanarak yatışır", sure: 1.4, gecikme: 0.15, f: |k, c| Poz { aci: (1.0 - k) * (k * 14.0).sin() * 1.2 + c.sal.aci * k, dy: -(1.0 - k) * c.h * 0.08, parlak: (0.3 + k * 2.0).min(1.0), ..c.sal } },
    Tanim { ad: "buyu", acik: "Küçükten büyür, hafif aşıp yerine oturur", sure: 0.8, gecikme: 0.15, f: |k, c| Poz { olcek: anim::geri(k).max(0.01), ..c.sal } },
    Tanim { ad: "patla", acik: "Dev boyutta başlar, hızla yerine küçülür", sure: 0.7, gecikme: 0.12, f: |k, c| Poz { olcek: 1.0 + 2.5 * (1.0 - ys(k)).powi(2), parlak: (k * 1.5).min(1.0), ..c.sal } },
    Tanim { ad: "sarsil", acik: "Yatay sarsılarak belirir", sure: 0.9, gecikme: 0.12, f: |k, c| Poz { dx: (k * 45.0).sin() * (1.0 - k) * c.w * 0.06, parlak: (k * 3.0).min(1.0), ..c.sal } },
    Tanim { ad: "dalga", acik: "Dalgalanarak durulur", sure: 1.3, gecikme: 0.2, f: |k, c| Poz { dy: (k * 2.0 * PI * 1.5 + c.li as f32).sin() * (1.0 - k) * c.h * 0.1, dx: (k * 2.0 * PI).sin() * (1.0 - k) * c.w * 0.08, parlak: (k * 2.5).min(1.0), ..c.sal } },
    Tanim { ad: "yay-sag", acik: "Sağdan yay gibi esneyerek gelir", sure: 1.2, gecikme: 0.15, f: |k, c| Poz { dx: (1.0 - anim::elastik(k)) * c.w, ..c.sal } },
    Tanim { ad: "yay-sol", acik: "Soldan yay gibi esneyerek gelir", sure: 1.2, gecikme: 0.15, f: |k, c| Poz { dx: -(1.0 - anim::elastik(k)) * c.w, ..c.sal } },
    Tanim { ad: "ucarak", acik: "Sağ üstten dönerek uçup gelir", sure: 1.1, gecikme: 0.15, f: |k, c| Poz { dx: (1.0 - ys(k)) * c.w * 1.2, dy: -(1.0 - ys(k)) * c.h * 0.4, olcek: 0.5 + 0.5 * ys(k), aci: (1.0 - ys(k)) * PI + c.sal.aci * k, ..c.sal } },
    Tanim { ad: "spiral", acik: "Daralan sarmal çizerek gelir", sure: 1.5, gecikme: 0.15, f: |k, c| { let r = 1.0 - ys(k); Poz { dx: (k * 9.0).cos() * r * c.w * 0.5, dy: (k * 9.0).sin() * r * c.h * 0.15, olcek: 0.2 + 0.8 * ys(k), aci: r * 2.0 * PI + c.sal.aci * k, parlak: (k * 3.0).min(1.0), ..c.sal } } },
    Tanim { ad: "derinles", acik: "Yazı önce çok kalın (derin) başlar, incelerek yerine oturur", sure: 1.2, gecikme: 0.15, f: |k, c| Poz { derinlik: c.d * (1.0 + 5.0 * (1.0 - ys(k)).powi(2)), parlak: ys(k), ..c.sal } },
    Tanim { ad: "uzaktan", acik: "Çok uzaktan küçük gelir, yakınlaşır", sure: 1.2, gecikme: 0.15, f: |k, c| Poz { olcek: 0.08 + 0.92 * ys(k), parlak: k, ..c.sal } },
    Tanim { ad: "sinema", acik: "Yavaş, sinematik geri çekilerek belirir", sure: 1.8, gecikme: 0.25, f: |k, c| Poz { olcek: 1.5 - 0.5 * ys(k), parlak: ys(k), ..c.sal } },
    Tanim { ad: "capraz", acik: "Sol üstten çapraz kayar", sure: 1.0, gecikme: 0.15, f: |k, c| Poz { dx: -(1.0 - ys(k)) * c.w, dy: -(1.0 - ys(k)) * c.h * 0.5, ..c.sal } },
    Tanim { ad: "asilma", acik: "Tavandan sarkaç gibi sallanarak iner", sure: 1.5, gecikme: 0.18, f: |k, c| Poz { dy: -(1.0 - anim::geri(k)) * c.h * 0.9, aci: (1.0 - k) * (k * 10.0).sin() * 0.9 + c.sal.aci * k, ..c.sal } },
    Tanim { ad: "nabiz", acik: "Kalp atışı gibi büyüyüp küçülerek belirir", sure: 1.2, gecikme: 0.15, f: |k, c| Poz { olcek: 1.0 + 0.35 * (k * 3.0 * PI).sin().abs() * (1.0 - k), parlak: (k * 4.0).min(1.0), ..c.sal } },
    Tanim { ad: "neon", acik: "Neon tabela gibi yanıp sönerek yanar (hızlı strobe değil, kısa titreme)", sure: 1.2, gecikme: 0.2, f: |k, c| Poz { parlak: if k >= 0.85 { 1.0 } else if hash((k * 14.0) as u32 + c.li as u32 * 31) > 0.5 { 0.9 } else { 0.15 }, ..c.sal } },
    Tanim { ad: "yavas-belir", acik: "Sakin ve yavaş belirir", sure: 1.6, gecikme: 0.3, f: |k, c| Poz { parlak: ys(k), olcek: 0.95 + 0.05 * ys(k), ..c.sal } },
    Tanim { ad: "yuksel", acik: "Aşağıdan süzülerek yükselir", sure: 1.3, gecikme: 0.2, f: |k, c| Poz { dy: (1.0 - ys(k)) * c.h * 0.15, parlak: ys(k), ..c.sal } },
    Tanim { ad: "donerek-buyu", acik: "Çift tur dönerek büyür", sure: 1.2, gecikme: 0.15, f: |k, c| Poz { aci: (1.0 - ys(k)) * 4.0 * PI + c.sal.aci * k, olcek: 0.02 + 0.98 * ys(k), parlak: (k * 3.0).min(1.0), ..c.sal } },
    Tanim { ad: "yildiz", acik: "Esneyerek büyür ve dönerek durur", sure: 1.2, gecikme: 0.15, f: |k, c| Poz { olcek: anim::elastik(k).max(0.01), aci: (1.0 - k) * 2.0 * PI + c.sal.aci * k, ..c.sal } },
];

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Egri { Yumusak, Dogrusal }

/// Anahtar karlı özel animasyon. Alanlar: dx, dy (ekran oranı), olcek, aci (derece), parlak, derinlik (kat sayı).
#[derive(Clone, Debug)]
pub struct Ozel { pub ad: String, pub acik: String, pub sure: f32, pub gecikme: f32, pub egri: Egri, pub anahtar: Vec<(f32, [f32; 6])> }

const VARSAYILAN: [f32; 6] = [0.0, 0.0, 1.0, 0.0, 1.0, 1.0]; // dx dy olcek aci parlak derinlik
const ALANLAR: [&str; 6] = ["dx", "dy", "olcek", "aci", "parlak", "derinlik"];

static OZEL: RwLock<Vec<Ozel>> = RwLock::new(Vec::new());

/// Yerleşik özel animasyon dosyaları (programa gömülü).
const GOMULU: &[(&str, &str)] = &[("temel.txt", include_str!("../animasyonlar/temel.txt")), ("vurucu.txt", include_str!("../animasyonlar/vurucu.txt"))];

/// Metin dosyasındaki animasyonları çözer. Biçim:
/// ```text
/// animasyon ad
/// aciklama kısa açıklama
/// sure 1.2          # saniye (isteğe bağlı)
/// gecikme 0.2       # satırlar arası gecikme (isteğe bağlı)
/// egri yumusak      # yumusak | dogrusal
/// anahtar 0.0 dx=-1 olcek=0.5 parlak=0
/// anahtar 0.7 dx=0.05 olcek=1.1
/// anahtar 1.0
/// ```
/// Belirtilmeyen alan bir önceki anahtardan devam eder; ilk anahtarda ve k=1.0 anahtarında varsayılan (yerine oturmuş) değerlerle başlar:
/// dx=0 dy=0 olcek=1 aci=0 parlak=1 derinlik=1. Son anahtar 1.0 değilse yerine oturmuş hâl eklenir.
pub fn coz(metin: &str) -> Result<Vec<Ozel>, String> {
    let mut v: Vec<Ozel> = vec![];
    for (no, ham) in metin.lines().enumerate() {
        let s = ham.split('#').next().unwrap().trim();
        if s.is_empty() { continue; }
        let hata = |m: &str| format!("satır {}: {m}", no + 1);
        let (komut, rest) = s.split_once(char::is_whitespace).map(|(a, b)| (a, b.trim())).unwrap_or((s, ""));
        if komut == "animasyon" {
            if rest.is_empty() || rest.contains(char::is_whitespace) { return Err(hata("animasyon adı tek kelime olmalı")); }
            v.push(Ozel { ad: rest.to_lowercase(), acik: String::new(), sure: 1.0, gecikme: 0.15, egri: Egri::Yumusak, anahtar: vec![] });
            continue;
        }
        let o = v.last_mut().ok_or_else(|| hata("önce 'animasyon ad' satırı gerekli"))?;
        match komut {
            "aciklama" => o.acik = rest.to_string(),
            "sure" => { o.sure = rest.parse().map_err(|_| hata("sure sayı olmalı"))?; if !(0.2..=5.0).contains(&o.sure) { return Err(hata("sure 0.2 ile 5 sn arasında olmalı")); } }
            "gecikme" => { o.gecikme = rest.parse().map_err(|_| hata("gecikme sayı olmalı"))?; if !(0.0..=2.0).contains(&o.gecikme) { return Err(hata("gecikme 0 ile 2 sn arasında olmalı")); } }
            "egri" => o.egri = match rest { "yumusak" => Egri::Yumusak, "dogrusal" => Egri::Dogrusal, _ => return Err(hata("egri yumusak veya dogrusal olmalı")) },
            "anahtar" => {
                let mut p = rest.split_whitespace();
                let k: f32 = p.next().ok_or_else(|| hata("anahtar zamanı eksik"))?.parse().map_err(|_| hata("anahtar zamanı sayı olmalı (0..1)"))?;
                if !(0.0..=1.0).contains(&k) { return Err(hata("anahtar zamanı 0 ile 1 arasında olmalı")); }
                if let Some(&(son, _)) = o.anahtar.last() { if k <= son { return Err(hata("anahtar zamanları artan sırada olmalı")); } }
                let mut d = if k >= 1.0 { VARSAYILAN } else { o.anahtar.last().map(|x| x.1).unwrap_or(VARSAYILAN) };
                for kv in p {
                    let (a, b) = kv.split_once('=').ok_or_else(|| hata(&format!("alan=değer bekleniyor: {kv}")))?;
                    let i = ALANLAR.iter().position(|x| *x == a).ok_or_else(|| hata(&format!("bilinmeyen alan: {a} ({ALANLAR:?})")))?;
                    d[i] = b.parse().map_err(|_| hata(&format!("{a} sayı olmalı")))?;
                }
                o.anahtar.push((k, d));
            }
            _ => return Err(hata(&format!("bilinmeyen komut: {komut}"))),
        }
    }
    for o in &mut v {
        if o.anahtar.is_empty() { return Err(format!("animasyon {}: en az bir anahtar gerekli", o.ad)); }
        if o.anahtar.last().unwrap().0 < 1.0 { o.anahtar.push((1.0, VARSAYILAN)); }
        if YERLESIK.iter().any(|y| y.ad == o.ad) || RESERVED.contains(&o.ad.as_str()) { return Err(format!("animasyon adı ayrılmış: {}", o.ad)); }
    }
    Ok(v)
}

const RESERVED: [&str; 18] = ["yok", "donerek", "zipla", "yandan", "zoom", "alttan", "ustten", "belir", "elastik", "daktilo", "sil", "toplan", "yukari", "asagi", "sondur", "kucul", "dagil", "perde"];

fn ozel_poz(o: &Ozel, k: f32, c: &Ctx) -> Poz {
    let k = k.clamp(0.0, 1.0);
    let a = &o.anahtar;
    let d = if k <= a[0].0 { a[0].1 } else {
        let i = a.iter().position(|x| x.0 >= k).unwrap_or(a.len() - 1);
        if i == 0 { a[0].1 } else {
            let (k0, d0) = a[i - 1]; let (k1, d1) = a[i];
            let mut u = if k1 > k0 { (k - k0) / (k1 - k0) } else { 1.0 };
            if o.egri == Egri::Yumusak { u = anim::yumusak(u); }
            let mut r = [0.0; 6];
            for j in 0..6 { r[j] = d0[j] + (d1[j] - d0[j]) * u; }
            r
        }
    };
    Poz { dx: d[0] * c.w, dy: d[1] * c.h, olcek: d[2].max(0.01), aci: d[3].to_radians() + c.sal.aci * k, parlak: d[4].clamp(0.0, 1.0), derinlik: c.d * d[5].max(0.0), ..c.sal }
}

/// Gömülü ve klasördeki özel animasyonları (yeniden) yükler. Aynı adlıda sonra yüklenen kazanır.
pub fn yukle() {
    let mut hepsi: Vec<Ozel> = vec![];
    for (ad, m) in GOMULU { match coz(m) { Ok(v) => hepsi.extend(v), Err(e) => eprintln!("UYARI: gömülü {ad}: {e}") } }
    for k in klasorler() {
        let Ok(r) = std::fs::read_dir(&k) else { continue };
        let mut dosyalar: Vec<PathBuf> = r.filter_map(|e| e.ok()).map(|e| e.path()).filter(|p| p.extension().map(|x| x == "txt").unwrap_or(false)).collect();
        dosyalar.sort();
        for p in dosyalar {
            match std::fs::read_to_string(&p).map_err(|e| e.to_string()).and_then(|m| coz(&m)) {
                Ok(v) => for o in v { hepsi.retain(|x| x.ad != o.ad); hepsi.push(o); },
                Err(e) => eprintln!("UYARI: {}: {e}", p.display()),
            }
        }
    }
    *OZEL.write().unwrap() = hepsi;
}

/// Kullanıcının animasyon dosyası koyabileceği klasörler: LED_ANIMASYON, programın yanındaki `animasyonlar`, çıktı klasörü altında `animasyonlar`.
pub fn klasorler() -> Vec<PathBuf> {
    let mut k = vec![];
    if let Ok(p) = std::env::var("LED_ANIMASYON") { k.push(PathBuf::from(p)); }
    if let Ok(e) = std::env::current_exe() { if let Some(d) = e.parent() { k.push(d.join("animasyonlar")); } }
    k.push(crate::arayuz::cikti_klasoru().join("animasyonlar"));
    k
}

fn once() { static O: std::sync::Once = std::sync::Once::new(); O.call_once(yukle); }

/// (süre, satır gecikmesi) — yalnız yerleşik tablo ve özel animasyonlar için; eski özel durumlu adlar için None.
pub fn bilgi(ad: &str) -> Option<(f32, f32)> {
    once();
    if let Some(t) = YERLESIK.iter().find(|t| t.ad == ad) { return Some((t.sure, t.gecikme)); }
    OZEL.read().unwrap().iter().find(|o| o.ad == ad).map(|o| (o.sure, o.gecikme))
}

/// k: 0 = görünmez başlangıç, 1 = yerine oturmuş. Çıkış için çağıran 1-k verir.
pub fn poz(ad: &str, k: f32, c: &Ctx) -> Option<Poz> {
    once();
    if let Some(t) = YERLESIK.iter().find(|t| t.ad == ad) { return Some((t.f)(k.clamp(0.0, 1.0), c)); }
    OZEL.read().unwrap().iter().find(|o| o.ad == ad).map(|o| ozel_poz(o, k, c))
}

/// Tüm tablo/özel animasyonlar: (ad, açıklama, özel_mi).
pub fn liste() -> Vec<(String, String, bool)> {
    once();
    let mut v: Vec<(String, String, bool)> = YERLESIK.iter().map(|t| (t.ad.to_string(), t.acik.to_string(), false)).collect();
    v.extend(OZEL.read().unwrap().iter().map(|o| (o.ad.clone(), o.acik.clone(), true)));
    v
}

#[cfg(test)]
mod t {
    use super::*;
    fn ctx() -> Ctx { Ctx { w: 500.0, h: 900.0, li: 1, d: 20.0, sal: Poz { aci: 0.0, olcek: 1.0, parlak: 1.0, derinlik: 20.0, dx: 0.0, dy: 0.0 } } }
    #[test]
    fn yerlesikler_sonda_yerine_oturur() {
        let c = ctx();
        for t in YERLESIK {
            let p = (t.f)(1.0, &c);
            assert!(p.dx.abs() < 1.0 && p.dy.abs() < 1.0, "{} dx/dy", t.ad);
            assert!((p.olcek - 1.0).abs() < 0.02, "{} ölçek {}", t.ad, p.olcek);
            assert!((p.parlak - 1.0).abs() < 0.01, "{} parlak", t.ad);
            assert!((p.derinlik - c.d).abs() < 0.5, "{} derinlik", t.ad);
            let b = (t.f)(0.0, &c);
            assert!(b.parlak >= 0.0 && b.olcek > 0.0);
        }
        let mut adlar: Vec<&str> = YERLESIK.iter().map(|t| t.ad).collect(); adlar.sort(); adlar.dedup();
        assert_eq!(adlar.len(), YERLESIK.len());
        assert!(YERLESIK.iter().all(|t| !RESERVED.contains(&t.ad)));
    }
    #[test]
    fn gomulu_dosyalar_gecerli_ve_oturur() {
        let c = ctx();
        for (ad, m) in GOMULU {
            for o in coz(m).unwrap_or_else(|e| panic!("{ad}: {e}")) {
                let p = ozel_poz(&o, 1.0, &c);
                assert!(p.dx.abs() < 1.0 && (p.olcek - 1.0).abs() < 0.02 && (p.parlak - 1.0).abs() < 0.01, "{}", o.ad);
            }
        }
    }
    #[test]
    fn anahtar_kare_ara_deger() {
        let v = coz("animasyon deneme\negri dogrusal\nanahtar 0 dx=-1 parlak=0\nanahtar 1\n").unwrap();
        let p = ozel_poz(&v[0], 0.5, &ctx());
        assert!((p.dx + 250.0).abs() < 1.0 && (p.parlak - 0.5).abs() < 0.01);
    }
    #[test]
    fn hata_mesajlari() {
        assert!(coz("anahtar 0").unwrap_err().contains("önce"));
        assert!(coz("animasyon a\nanahtar 0 xx=1").unwrap_err().contains("bilinmeyen alan"));
        assert!(coz("animasyon a\nanahtar 0.5\nanahtar 0.2").unwrap_err().contains("artan"));
        assert!(coz("animasyon dusus\nanahtar 0").unwrap_err().contains("ayrılmış"));
        assert!(coz("animasyon a\nsure 99\nanahtar 0").unwrap_err().contains("sure"));
        assert!(coz("animasyon a").unwrap_err().contains("anahtar"));
    }
}
