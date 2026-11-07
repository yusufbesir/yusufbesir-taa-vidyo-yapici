//! 3B (ekstrüzyonlu) yazı ve geçiş animasyonları. Yazılım render, GPU gerekmez.
use crate::*;
use ab_glyph::FontRef;

pub struct Maske { pub w: u32, pub h: u32, pub a: Vec<f32>, pub bx: (u32, u32, u32, u32) } // bbox x0,y0,x1,y1

pub fn maske(font: &FontRef, w: u32, h: u32, satirlar: &[String], harf: f32) -> Maske {
    let k = kare(font, w, h, satirlar, harf, Renk(255, 255, 255), Renk(0, 0, 0), 0.0);
    let a: Vec<f32> = k.chunks(3).map(|p| p[0] as f32 / 255.0).collect();
    let (mut x0, mut y0, mut x1, mut y1) = (w, h, 0, 0);
    for y in 0..h { for x in 0..w { if a[(y * w + x) as usize] > 0.02 { x0 = x0.min(x); x1 = x1.max(x); y0 = y0.min(y); y1 = y1.max(y); } } }
    Maske { w, h, a, bx: (x0, y0, x1, y1) }
}

fn ornek(m: &Maske, x: f32, y: f32) -> f32 {
    if x < 0.0 || y < 0.0 || x >= (m.w - 1) as f32 || y >= (m.h - 1) as f32 { return 0.0; }
    let (xi, yi) = (x as u32, y as u32);
    let (fx, fy) = (x - xi as f32, y - yi as f32);
    let g = |xx: u32, yy: u32| m.a[(yy * m.w + xx) as usize];
    let t = g(xi, yi) * (1.0 - fx) + g(xi + 1, yi) * fx;
    let b = g(xi, yi + 1) * (1.0 - fx) + g(xi + 1, yi + 1) * fx;
    t * (1.0 - fy) + b * fy
}

pub fn yumusak(t: f32) -> f32 { let t = t.clamp(0.0, 1.0); t * t * (3.0 - 2.0 * t) }
/// Hafif aşan giriş (geri tepme).
pub fn geri(t: f32) -> f32 { let t = t.clamp(0.0, 1.0) - 1.0; let c = 1.70158; t * t * ((c + 1.0) * t + c) + 1.0 }

#[derive(Clone, Copy)]
pub struct Poz { pub aci: f32, pub olcek: f32, pub parlak: f32, pub derinlik: f32, pub dx: f32, pub dy: f32 }

pub fn ziplama(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    if t < 1.0 / 2.75 { 7.5625 * t * t }
    else if t < 2.0 / 2.75 { let t = t - 1.5 / 2.75; 7.5625 * t * t + 0.75 }
    else if t < 2.5 / 2.75 { let t = t - 2.25 / 2.75; 7.5625 * t * t + 0.9375 }
    else { let t = t - 2.625 / 2.75; 7.5625 * t * t + 0.984375 }
}

pub const STILLER: usize = 4;

/// Bir satırın pozu. `stil`: 0 dönerek giriş, 1 yukarıdan zıplama, 2 yandan dönerek kayma, 3 dev zoom.
/// `gecikme`: satırın giriş gecikmesi (sn). `ekran`: (w,h) kayma uzaklıkları için.
pub fn poz(t: f32, sure: f32, derinlik: f32, stil: usize, gecikme: f32, ekran: (u32, u32)) -> Poz {
    let (w, h) = (ekran.0 as f32, ekran.1 as f32);
    let giris = 0.9_f32.min(sure * 0.25);
    let cikis = 0.7_f32.min(sure * 0.2);
    let pi = std::f32::consts::PI;
    let ti = t - gecikme;
    let bos = Poz { aci: 0.0, olcek: 1.0, parlak: 0.0, derinlik, dx: 0.0, dy: 0.0 };
    if ti < 0.0 { return bos; }
    if t > sure - cikis {
        let k = (t - (sure - cikis)) / cikis;
        return Poz { aci: -yumusak(k) * pi * 0.5, olcek: 1.0 + 0.25 * k, parlak: 1.0 - k, derinlik, dx: 0.0, dy: -k * k * h * 0.25 };
    }
    if ti < giris {
        let k = ti / giris;
        let (aci, olcek, dx, dy) = match stil % STILLER {
            0 => ((1.0 - geri(k)) * pi * 0.5 + (1.0 - yumusak(k)) * pi * 0.5, 0.4 + 0.6 * geri(k), 0.0, 0.0),
            1 => (0.0, 1.0, 0.0, -(1.0 - ziplama(k)) * h * 0.7),
            2 => ((1.0 - yumusak(k)) * pi * 1.5, 0.6 + 0.4 * yumusak(k), (1.0 - yumusak(k)) * w * 1.1, 0.0),
            _ => (0.0, 1.0 + 2.2 * (1.0 - geri(k)), 0.0, 0.0),
        };
        return Poz { aci, olcek, parlak: 0.4 + 0.6 * k, derinlik: derinlik * yumusak(k), dx, dy };
    }
    let s = ti - giris;
    // yavaş sallanma ve nabız; satırlar farklı fazda: köşeden bakanda da yan yüzeyler okunur
    let faz = gecikme * 2.0;
    Poz { aci: (s * 1.3 + faz).sin() * 0.28, olcek: 1.0 + 0.03 * (s * 3.0 + faz).sin(), parlak: 0.93 + 0.07 * (s * 3.0).sin(), derinlik, dx: 0.0, dy: (s * 2.0 + faz).sin() * 6.0 }
}

/// 3B kare: arkadan öne ekstrüzyon katmanları, sonra ön yüz.
pub fn bos_kare(w: u32, h: u32, zemin: Renk) -> Vec<u8> {
    let mut t = vec![0u8; (w * h * 3) as usize];
    for px in t.chunks_mut(3) { px.copy_from_slice(&[zemin.0, zemin.1, zemin.2]); }
    t
}

pub fn kare3d(m: &Maske, yuz: Renk, yan: Renk, zemin: Renk, p: &Poz) -> Vec<u8> {
    let mut t = bos_kare(m.w, m.h, zemin);
    kare3d_uzerine(&mut t, m, yuz, yan, p);
    t
}

/// Maskenin yalnız [y0,y1) satır bandını içeren kopya (satır satır animasyon için).
pub fn bant(m: &Maske, y0: u32, y1: u32) -> Maske {
    let (y0, y1) = (y0.min(m.h), y1.min(m.h));
    let mut a = vec![0f32; m.a.len()];
    let (mut bx0, mut by0, mut bx1, mut by1) = (m.w, m.h, 0, 0);
    for y in y0..y1 { for x in 0..m.w {
        let i = (y * m.w + x) as usize;
        if m.a[i] > 0.0 { a[i] = m.a[i]; if m.a[i] > 0.02 { bx0 = bx0.min(x); bx1 = bx1.max(x); by0 = by0.min(y); by1 = by1.max(y); } }
    } }
    Maske { w: m.w, h: m.h, a, bx: (bx0, by0, bx1, by1) }
}

pub fn kare3d_uzerine(t: &mut [u8], m: &Maske, yuz: Renk, yan: Renk, p: &Poz) {
    let (w, h) = (m.w, m.h);
    if p.parlak <= 0.0 { return; }
    let (bx0, by0, bx1, by1) = m.bx;
    if bx1 <= bx0 { return; }
    let (cx, cy) = ((bx0 + bx1) as f32 / 2.0, (by0 + by1) as f32 / 2.0);
    let sx = (p.olcek * p.aci.cos()).abs().max(0.03);
    let sy = p.olcek;
    let kat = p.derinlik.max(1.0) as i32;
    // ekran sınırı (taşmayı kes)
    let (hw, hh) = ((bx1 - bx0) as f32 / 2.0 * sx.max(p.olcek) + p.derinlik * 1.5 + 4.0, (by1 - by0) as f32 / 2.0 * sy + p.derinlik * 1.5 + 4.0);
    let (ox, oy) = (cx + p.dx, cy + p.dy);
    if ox + hw < 0.0 || ox - hw > w as f32 || oy + hh < 0.0 || oy - hh > h as f32 { return; }
    let (rx0, rx1) = ((ox - hw).max(0.0) as u32, ((ox + hw) as u32).min(w - 1));
    let (ry0, ry1) = ((oy - hh).max(0.0) as u32, ((oy + hh) as u32).min(h - 1));
    let kayma = p.aci.sin() * 0.9 + 0.55; // yatay derinlik izdüşümü (hep biraz yan yüz görünür)
    let ust = 0.45; // yukarıdan bakış
    let mut katman = |dx: f32, dy: f32, renk: [f32; 3]| {
        for y in ry0..=ry1 {
            for x in rx0..=rx1 {
                let mx = cx + (x as f32 - dx - p.dx - cx) / sx;
                let my = cy + (y as f32 - dy - p.dy - cy) / sy;
                let a = ornek(m, mx, my);
                if a <= 0.0 { continue; }
                let i = ((y * w + x) * 3) as usize;
                for k in 0..3 { t[i + k] = (renk[k] * a + t[i + k] as f32 * (1.0 - a)) as u8; }
            }
        }
    };
    for k in (1..=kat).rev() {
        let f = k as f32 / kat as f32; // 1=arka (koyu) → 0=ön
        let par = (0.35 + 0.35 * (1.0 - f)) * p.parlak;
        katman(k as f32 * kayma * 0.55, k as f32 * ust * 0.55, [yan.0 as f32 * par, yan.1 as f32 * par, yan.2 as f32 * par]);
    }
    let pr = p.parlak;
    katman(0.0, 0.0, [yuz.0 as f32 * pr, yuz.1 as f32 * pr, yuz.2 as f32 * pr]);
}

#[cfg(test)]
mod t {
    use super::*;
    #[test]
    fn poz_gecis_uclari() {
        assert!(poz(0.0, 5.0, 20.0, 0, 0.0, (600, 300)).parlak < 0.6);
        assert!((poz(2.5, 5.0, 20.0, 0, 0.0, (600, 300)).derinlik - 20.0).abs() < 0.1);
        assert!(poz(4.99, 5.0, 20.0, 0, 0.0, (600, 300)).parlak < 0.05);
    }
    #[test]
    fn kare3d_yazi_ve_yan_yuz() {
        let f = FontRef::try_from_slice(include_bytes!("../fonts/InterDisplay-Bold.otf")).unwrap();
        let m = maske(&f, 600, 300, &["TAA".into()], 150.0);
        let k = kare3d(&m, Renk(255, 220, 0), Renk(200, 90, 0), Renk(0, 0, 0), &poz(2.5, 5.0, 20.0, 0, 0.0, (600, 300)));
        assert!(k.iter().any(|&b| b > 200));
        // yan yüz rengi (koyu turuncu) görünmeli: R yüksek, G orta
        assert!(k.chunks(3).any(|p| p[0] > 40 && p[0] < 140 && p[1] < p[0]));
    }
}

/// Maskeyi dikeyde `dy` piksel kaydırır (satır başına ayrı boyutlu yerleşim için).
pub fn kaydir(m: &Maske, dy: i32) -> Maske {
    let mut a = vec![0f32; m.a.len()];
    let w = m.w as usize;
    for y in 0..m.h as i32 {
        let ny = y + dy;
        if ny < 0 || ny >= m.h as i32 { continue; }
        a[ny as usize * w..(ny as usize + 1) * w].copy_from_slice(&m.a[y as usize * w..(y as usize + 1) * w]);
    }
    let (x0, y0, x1, y1) = m.bx;
    Maske { w: m.w, h: m.h, a, bx: (x0, (y0 as i32 + dy).max(0) as u32, x1, (y1 as i32 + dy).clamp(0, m.h as i32 - 1) as u32) }
}

/// Maskeye dolu dikdörtgen ekler (çizgi/ayraç süsleri için).
pub fn dikdortgen(m: &mut Maske, x0: u32, y0: u32, x1: u32, y1: u32) {
    let (x1, y1) = (x1.min(m.w - 1), y1.min(m.h - 1));
    for y in y0..=y1 { for x in x0..=x1 { m.a[(y * m.w + x) as usize] = 1.0; } }
    let (bx0, by0, bx1, by1) = m.bx;
    m.bx = if bx1 <= bx0 { (x0, y0, x1, y1) } else { (bx0.min(x0), by0.min(y0), bx1.max(x1), by1.max(y1)) };
}
