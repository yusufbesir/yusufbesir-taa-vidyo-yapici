//! Otomatik mod: yalnız yazıdan senaryo kurar; öneri/denetim motoru (kural tabanlı, yapay zekâ gerekmez).
use crate::efekt::hash;
use crate::senaryo::{Hazir, Senaryo};

pub const STILLER: [(&str, &str); 5] = [
    ("yumusak", "Yumuşak: sakin girişler, yavaş geçişler"),
    ("enerjik", "Enerjik: hızlı, sıçrayan, konfetili"),
    ("minimal", "Minimal: sade, tek renk, efektsiz"),
    ("gosterisli", "Gösterişli: 3B dönüşler, parıltı, havai fişek"),
    ("senlik", "Şenlik: çiçek, kelebek, balon, kalp"),
];

struct Havuz { giris: &'static [&'static str], efekt: &'static [&'static str], gecis: &'static [&'static str], palet: &'static [&'static str], font: &'static [&'static str], efekt_say: usize }

fn havuz(stil: &str) -> Havuz {
    match stil {
        "enerjik" => Havuz { giris: &["zoom-patla", "sicra-asagi", "hizli-sol", "tek-tek-zoom", "tur", "isik-hizi", "buyuk-kucuk"], efekt: &["parlama", "kivilcim", "konfeti", "kayan-yildiz", "cerceve-kose", "cerceve-isik"], gecis: &["tv", "kapi", "slayt", "jaluzi", "cubuk"], palet: &["altin-cyan", "pembe-altin", "sari-lacivert", "altin-gece", "kirmizi-altin", "turuncu-gece"], font: &["anton", "bebas", "russo", "archivo", "bungee", "staatliches"], efekt_say: 2 },
        "minimal" => Havuz { giris: &["belir", "kayar-alt", "odak", "asansor"], efekt: &[], gecis: &["fade", "slayt", "sil", "kare"], palet: &["sicak-beyaz", "tek-altin", "gok-mavisi", "gumus-mavi"], font: &["inter", "barlow", "fjalla", "saira-dar"], efekt_say: 0 },
        "gosterisli" => Havuz { giris: &["derin-donus", "flip-cift", "tur", "murekkep", "kutu-sal", "derin"], efekt: &["parlama", "pirilti", "cift-cerceve", "havai-fisek", "yildiz-yagmuru", "cerceve-altin", "cerceve-neon", "zemin-plazma"], gecis: &["tv", "neon", "kapi", "iris", "elmas", "dalga"], palet: &["altin-gece", "mor-altin", "magenta-gece", "altin-cyan", "bakir", "lavanta-altin"], font: &["slab", "archivo", "anton", "poppins", "yeseva", "dm-serif"], efekt_say: 2 },
        "senlik" => Havuz { giris: &["sicra-asagi", "yaylan-alt", "tek-tek-sicra", "zoom-patla", "kutu-sal"], efekt: &["cicek-yagmuru", "kelebek", "konfeti", "balon", "kalp", "papatya", "ugur-bocegi", "pirilti-bulutu", "yaprak", "cerceve-yildiz", "cerceve-nokta"], gecis: &["iris", "kapi", "fade", "mozaik", "kare"], palet: &["pembe-altin", "altin-cyan", "mor-altin", "pembe-parilti", "gun-batimi", "neon-yesil"], font: &["bangers", "poppins", "archivo", "sigmar", "paytone"], efekt_say: 2 },
        _ => Havuz { giris: &["belir", "odak", "asansor", "yavas-alt", "kayar-alt", "murekkep", "yavas-sol"], efekt: &["pirilti", "parlama", "cerceve-ince", "zemin-gradyan"], gecis: &["fade", "iris", "perde", "slayt", "sil", "dalga"], palet: &["sicak-beyaz", "altin-gece", "gok-mavisi", "buz-cyan", "nane", "okyanus"], font: &["inter", "poppins", "fjalla", "righteous"], efekt_say: 1 },
    }
}

fn sec<'a>(p: &'a [&'a str], tohum: u32, k: u32) -> &'a str { p[(hash(tohum.wrapping_mul(31).wrapping_add(k)) * p.len() as f32) as usize % p.len()] }

/// Yazıyı afiş satırlarına böler: sayı satırı büyük rakam (+N), uzun satırlar en çok 10 harflik parçalara.
fn satirlar(blok: &str) -> Vec<String> {
    let mut v = vec![];
    for l in blok.lines().map(str::trim).filter(|l| !l.is_empty()) {
        if l.chars().all(|c| c.is_ascii_digit()) && l.len() <= 2 { v.push(format!("+{l}")); continue; }
        let mut satir = String::new();
        for k in l.split_whitespace() {
            if !satir.is_empty() && satir.chars().count() + 1 + k.chars().count() > 10 { v.push(std::mem::take(&mut satir)); }
            if !satir.is_empty() { satir.push(' '); }
            satir.push_str(k);
        }
        if !satir.is_empty() { v.push(satir); }
    }
    v
}

/// Yazıdan tam senaryo metni üretir. Boş satırla ayrılan her blok bir sahnedir.
pub fn uret(metin: &str, stil: &str, tohum: u32, ekran: (u32, u32), fiziksel: Option<(f32, f32)>) -> String {
    let hv = havuz(stil);
    let mut bloklar: Vec<Vec<String>> = vec![];
    for b in metin.replace("\r\n", "\n").split("\n\n") { let s = satirlar(b); for parca in s.chunks(5) { if !parca.is_empty() { bloklar.push(parca.to_vec()); } } }
    if bloklar.is_empty() { bloklar.push(vec!["MERHABA".into()]); }
    let uzun = bloklar.iter().any(|b| b.iter().map(|x| x.chars().count()).sum::<usize>() > 24);
    // uzun metinde enerjik havuz yerine sakin havuza düş; tek hareket ailesi
    let giris_havuz = if uzun && stil == "enerjik" { havuz("yumusak").giris } else { hv.giris };
    let giris = sec(giris_havuz, tohum, 1);
    let ikinci = sec(giris_havuz, tohum, 2);
    let (acilis, kapanis) = (sec(hv.gecis, tohum, 3), sec(hv.gecis, tohum, 4));
    let palet = sec(hv.palet, tohum, 5);
    let font = sec(hv.font, tohum, 6);
    let mut o = format!("# Otomatik ({stil}, tohum {tohum})\nekran {}x{}\n", ekran.0, ekran.1);
    if let Some((g, y)) = fiziksel { o += &format!("fiziksel {g}x{y}\n"); }
    o += &format!("fps 25\nacilis {acilis}\nkapanis {kapanis}\n");
    let n = bloklar.len();
    for (i, b) in bloklar.iter().enumerate() {
        let mut parcalar: Vec<String> = vec![];
        if b.len() >= 3 && !b[0].starts_with('+') { parcalar.push("~".into()); }
        parcalar.extend(b.iter().cloned());
        if b.len() >= 3 { parcalar.push("~".into()); }
        let sure = (3.0 + 1.2 * b.len() as f32).clamp(6.0, 10.0);
        // yoğunluk eğrisi: açılış ve kapanış güçlü, orta sakin
        let guclu = i == 0 || i + 1 == n;
        let mut ef: Vec<&str> = vec![];
        if !hv.efekt.is_empty() { for k in 0..(if guclu { hv.efekt_say } else { hv.efekt_say.min(1) }) { let e = sec(hv.efekt, tohum, 10 + i as u32 * 3 + k as u32); if !ef.contains(&e) { ef.push(e); } } }
        let g = if i % 2 == 1 && n > 2 { ikinci } else { giris };
        let efekt = if ef.is_empty() { "yok".to_string() } else { ef.join(",") };
        o += &format!("sahne \"{}\" palet={palet} font={font} giris={g} cikis={} sure={sure} efekt={efekt}\n", parcalar.join("|"), if crate::senaryo::cikis_adlari().iter().any(|c| c == g) { g } else { "sondur" });
    }
    o
}

/// Denetim ve öneriler: (seviye, mesaj). seviye: "uyari" | "oneri".
pub fn denetle(s: &Senaryo, hz: &[Hazir]) -> Vec<(&'static str, String)> {
    let mut v: Vec<(&'static str, String)> = vec![];
    let sh_h = s.sanal_h() as f32;
    let mut giris_adlari: Vec<&str> = vec![];
    for (i, (sh, z)) in s.sahneler.iter().zip(hz).enumerate() {
        let no = i + 1;
        if !giris_adlari.contains(&sh.giris.as_str()) && sh.giris != "yok" { giris_adlari.push(&sh.giris); }
        let kucuk: Vec<f32> = z.boylar.iter().filter(|b| b.1).map(|b| b.0 * 0.72 / sh_h).collect();
        if let Some(m) = kucuk.iter().cloned().reduce(f32::min) {
            if m < 0.05 { v.push(("uyari", format!("Sahne {no}: en küçük harf ekran yüksekliğinin %{:.1}'i; en az %5 (~{} px) olmalı. Metni kısaltın veya satırlara bölün.", m * 100.0, (sh_h * 0.05) as u32))); }
            else if kucuk.iter().cloned().fold(0.0, f32::max) < 0.07 { v.push(("oneri", format!("Sahne {no}: ana mesaj satırları %7-9 aralığında olmalı (şu an en büyüğü %{:.1}). Daha az kelime daha okunur.", kucuk.iter().cloned().fold(0.0, f32::max) * 100.0))); }
        }
        if sh.metin.split('|').any(|l| l.trim().chars().count() > 10 && !l.starts_with('@')) { v.push(("oneri", format!("Sahne {no}: satır başına en çok 8-10 harf okunur; uzun satırı bölün."))); }
        if sh.sure < 5.0 { v.push(("uyari", format!("Sahne {no}: {} sn kısa; her mesaj en az 5-8 sn ekranda kalmalı.", sh.sure))); }
        if crate::kontrast(z.yuz, z.zemin) < 7.0 { v.push(("uyari", format!("Sahne {no}: yazı/zemin kontrastı 7:1'in altında."))); }
        for a in [&sh.giris, &sh.cikis] { if a.starts_with("yildirim") || a.starts_with("hizli-") || a == "titre" || a.contains("flas") { v.push(("uyari", format!("Sahne {no}: '{a}' sert/hızlı hareket; LED'de göz yorabilir, seyrek kullanın."))); break; } }
        let dekor = sh.efekt.iter().filter(|e| crate::dekor::ADLAR.iter().any(|d| d.0 == e.as_str())).count();
        if dekor > 2 { v.push(("oneri", format!("Sahne {no}: {dekor} dekor üst üste kalabalık; en çok 2 önerilir."))); }
        if sh.cikis == "yok" && i + 1 < s.sahneler.len() { v.push(("oneri", format!("Sahne {no}: çıkış animasyonu yok; girişin tersi (cikis=aynı ad) akışı yumuşatır."))); }
    }
    if giris_adlari.len() > 2 { v.push(("oneri", format!("Videoda {} farklı giriş var; en çok 2 hareket ailesi tutarlı görünür.", giris_adlari.len()))); }
    if s.acilis == crate::senaryo::Gecis::Yok && s.kapanis == crate::senaryo::Gecis::Yok { v.push(("oneri", "Açılış ve kapanış animasyonu eklenirse video bütünlük kazanır (örn. acilis fade, kapanis iris).".into())); }
    if s.sahneler.len() >= 2 {
        let (a, b) = (s.sahneler[0].sure, s.sahneler.iter().map(|x| x.sure).fold(f32::MAX, f32::min));
        if a > 0.0 && b > 0.0 && s.sahneler.iter().any(|x| (x.sure - a).abs() / a > 0.4) { v.push(("oneri", "Sahne süreleri birbirinden çok farklı; tek ritim için ±%20 içinde tutun.".into())); }
    }
    v
}

#[cfg(test)]
mod t {
    use super::*;
    #[test]
    fn otomatik_senaryo_gecerli() {
        crate::animasyon::yukle();
        for (st, _) in STILLER {
            for tohum in 0..8 {
                let m = uret("PEŞİN FİYATINA\n3\nTAKSİT\n\nTERZİ ADEM ALTUN\nHER GÜN AÇIK", st, tohum, (950, 1800), Some((8.0, 25.0)));
                let s = crate::senaryo::ayristir(&m).unwrap_or_else(|e| panic!("{st}/{tohum}: {e}\n{m}"));
                assert_eq!(s.sahneler.len(), 2);
            }
        }
    }
    #[test]
    fn denetim_kucuk_yaziyi_yakalar() {
        let f = crate::uret::font();
        let s = crate::senaryo::ayristir("ekran 950x1800\nsahne \"BU ÇOK UZUN BİR CÜMLE OLDUĞU İÇİN KÜÇÜK ÇIKAR GERÇEKTEN\" sure=3\n").unwrap();
        let hz = crate::senaryo::hazirla(&f, &s);
        let n = denetle(&s, &hz);
        assert!(n.iter().any(|x| x.1.contains("%5")), "{n:?}");
        assert!(n.iter().any(|x| x.1.contains("kısa")));
    }
}
