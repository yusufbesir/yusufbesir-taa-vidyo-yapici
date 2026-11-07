//! Çekirdek: LED ekran için okunabilir yazı karesi üretimi ve okunabilirlik kuralları.
pub mod anim;
pub mod cizim;
pub mod efekt;
pub mod tv;
pub mod senaryo;
/// Bir satırın yüksekliği (kare() ile aynı formül).
pub fn satir_yuksekligi(font: &FontRef, harf_px: f32) -> f32 { font.as_scaled(PxScale::from(harf_px)).height() * 1.15 }
use ab_glyph::{point, Font, FontRef, PxScale, ScaleFont};

pub const MIN_HARF_PX: f32 = 48.0;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Renk(pub u8, pub u8, pub u8);

/// Karanlıkta görünür hazır paletler: (ad, yazı, zemin)
pub fn palet(ad: &str) -> Option<(Renk, Renk)> {
    let siyah = Renk(0, 0, 0);
    Some(match ad {
        "beyaz" => (Renk(214, 214, 208), siyah), // saf beyaz LED'de projektör gibi parlar; hafif kısık
        "sari" => (Renk(255, 220, 0), siyah),
        "altin" => (Renk(255, 206, 48), siyah), // gold: turuncuya kaçmayan sıcak sarı
        "turuncu" => (Renk(255, 140, 0), siyah),
        "cyan" => (Renk(0, 230, 255), siyah),
        "magenta" => (Renk(255, 0, 200), siyah),
        "lime" => (Renk(130, 255, 0), siyah),
        _ => return None,
    })
}

fn lin(c: u8) -> f32 {
    let c = c as f32 / 255.0;
    if c <= 0.04045 { c / 12.92 } else { ((c + 0.055) / 1.055).powf(2.4) }
}
pub fn parlaklik(r: Renk) -> f32 {
    0.2126 * lin(r.0) + 0.7152 * lin(r.1) + 0.0722 * lin(r.2)
}
pub fn kontrast(a: Renk, b: Renk) -> f32 {
    let (x, y) = (parlaklik(a), parlaklik(b));
    (x.max(y) + 0.05) / (x.min(y) + 0.05)
}

/// Okunabilirlik uyarıları (sade metin).
pub fn uyarilar(yazi: Renk, zemin: Renk, harf_px: f32, satirlar: &[String], kayma_px_sn: f32, w: u32) -> Vec<String> {
    let mut u = vec![];
    if harf_px < MIN_HARF_PX { u.push(format!("Harf yüksekliği {harf_px:.0} px, en az {MIN_HARF_PX:.0} px olmalı.")); }
    if kontrast(yazi, zemin) < 7.0 { u.push("Yazı ile zemin kontrastı düşük (7:1 altı).".into()); }
    if parlaklik(yazi) < 0.25 { u.push("Yazı rengi karanlıkta soluk görünür; daha parlak renk seçin.".into()); }
    if satirlar.len() > 3 { u.push("En çok 3 satır önerilir.".into()); }
    if satirlar.iter().any(|s| s.split_whitespace().count() > 6) { u.push("Satır başına en çok 6 kelime önerilir.".into()); }
    if kayma_px_sn > 0.2 * w as f32 { u.push("Kayma çok hızlı; ekran genişliğinin saniyede %20'sini aşmasın.".into()); }
    u
}

/// RGB kare (w*h*3). `x_ofset` yatay kaydırma içindir.
pub fn kare(font: &FontRef, w: u32, h: u32, satirlar: &[String], harf_px: f32, yazi: Renk, zemin: Renk, x_ofset: f32) -> Vec<u8> {
    let mut tampon = vec![0u8; (w * h * 3) as usize];
    for p in tampon.chunks_mut(3) { p.copy_from_slice(&[zemin.0, zemin.1, zemin.2]); }
    let scale = PxScale::from(harf_px);
    let sf = font.as_scaled(scale);
    let satir_h = sf.height() * 1.15;
    let toplam = satir_h * satirlar.len() as f32;
    let mut y0 = (h as f32 - toplam) / 2.0 + sf.ascent();
    for s in satirlar {
        let gen = genislik(font, s, harf_px);
        let mut x = (w as f32 - gen) / 2.0 + x_ofset;
        let mut onceki = None;
        for ch in s.chars() {
            let id = font.glyph_id(ch);
            if let Some(o) = onceki { x += sf.kern(o, id); }
            let g = id.with_scale_and_position(scale, point(x, y0));
            x += sf.h_advance(id) + harf_px * 0.04; // açık harf aralığı
            if let Some(o) = font.outline_glyph(g) {
                let b = o.px_bounds();
                o.draw(|gx, gy, c| {
                    let (px, py) = (b.min.x as i32 + gx as i32, b.min.y as i32 + gy as i32);
                    if px < 0 || py < 0 || px >= w as i32 || py >= h as i32 { return; }
                    let i = ((py as u32 * w + px as u32) * 3) as usize;
                    let c = c.clamp(0.0, 1.0);
                    for (k, (f, z)) in [(yazi.0, zemin.0), (yazi.1, zemin.1), (yazi.2, zemin.2)].iter().enumerate() {
                        tampon[i + k] = (*f as f32 * c + *z as f32 * (1.0 - c)) as u8;
                    }
                });
            }
            onceki = Some(id);
        }
        y0 += satir_h;
    }
    tampon
}

pub fn genislik(font: &FontRef, s: &str, harf_px: f32) -> f32 {
    let sf = font.as_scaled(PxScale::from(harf_px));
    s.chars().map(|c| sf.h_advance(font.glyph_id(c)) + harf_px * 0.04).sum()
}

/// Kayan yazı için toplam yol: sağdan girip soldan çıkar.
pub fn kayma_x(t: f32, sure: f32, w: u32, yazi_gen: f32) -> f32 {
    let toplam = w as f32 + yazi_gen;
    toplam / 2.0 - toplam * (t / sure)
}

#[cfg(test)]
mod t {
    use super::*;
    #[test]
    fn kontrast_siyah_beyaz() { assert!(kontrast(Renk(255,255,255), Renk(0,0,0)) > 20.0); }
    #[test]
    fn koyu_renk_uyarir() {
        let u = uyarilar(Renk(20,20,80), Renk(0,0,0), 100.0, &["A".into()], 0.0, 1800);
        assert!(!u.is_empty());
    }
    #[test]
    fn iyi_ayar_uyarisiz() {
        let (y, z) = palet("sari").unwrap();
        assert!(uyarilar(y, z, 120.0, &["TERZİ ADEM ALTUN".into()], 100.0, 1800).is_empty());
    }
    #[test]
    fn kare_boyutu_ve_yazi() {
        let f = FontRef::try_from_slice(include_bytes!("../fonts/InterDisplay-Bold.otf")).unwrap();
        let (y, z) = palet("sari").unwrap();
        let k = kare(&f, 1800, 980, &["TAA".into()], 200.0, y, z, 0.0);
        assert_eq!(k.len(), 1800 * 980 * 3);
        assert!(k.iter().any(|&b| b > 200));
    }
}
