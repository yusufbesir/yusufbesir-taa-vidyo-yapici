//! Vektör çizimler (çiçek, yıldız, rozet, çerçeve) → 3B hattına giren maske.
use crate::anim::Maske;

pub const ADLAR: [&str; 6] = ["cicek", "papatya", "yildiz", "rozet", "cerceve", "elmas"];

/// Kenar yumuşatmalı kapsama: d>0 içeride.
fn kap(d: f32) -> f32 { (d + 0.5).clamp(0.0, 1.0) }

fn alan(ad: &str, x: f32, y: f32, r: f32) -> f32 {
    // x,y merkezden uzaklık (px), r ölçek yarıçapı
    let d = (x * x + y * y).sqrt();
    let a = y.atan2(x);
    let pi = std::f32::consts::PI;
    match ad {
        "cicek" => { // 8 yuvarlak taç yaprak + boş merkez
            let f = r * (0.55 + 0.45 * (4.0 * a).cos().abs().powf(0.6));
            kap(f - d).min(kap(d - r * 0.14))
        }
        "papatya" => { // 12 ince sivri yaprak
            let f = r * (0.3 + 0.7 * (6.0 * a).cos().abs().powf(3.0));
            kap(f - d).min(kap(d - r * 0.12))
        }
        "yildiz" => { // 5 köşeli
            let k = (a + pi / 2.0).rem_euclid(2.0 * pi / 5.0) - pi / 5.0;
            let f = r * 0.5 / (pi / 5.0 - k.abs()).cos().max(0.01) * 0.0 + r * (0.45 + 0.55 * (1.0 - k.abs() / (pi / 5.0)).powf(1.3));
            kap(f - d)
        }
        "rozet" => { // halka + 12 nokta
            let halka = kap(r * 0.62 - d).min(kap(d - r * 0.48));
            let ic = kap(r * 0.18 - d);
            let m = (a.rem_euclid(2.0 * pi / 12.0)) - pi / 12.0;
            let (dx, dy) = (d * m.cos() - r * 0.88, d * m.sin());
            let nokta = kap(r * 0.07 - (dx * dx + dy * dy).sqrt());
            halka.max(ic).max(nokta)
        }
        "elmas" => kap(r * 0.9 - (x.abs() * 0.8 + y.abs() * 1.2)),
        _ => 0.0,
    }
}

/// Çerçeve: çift çizgi, köşe girintili (mağaza camı afişi stili).
fn cerceve(x: f32, y: f32, hx: f32, hy: f32, kal: f32) -> f32 {
    let dis = |hx: f32, hy: f32| (x.abs() - hx).max(y.abs() - hy); // <0 içeride
    let c1 = kap(kal / 2.0 - dis(hx, hy).abs());
    let c2 = kap(kal / 3.0 - dis(hx - kal * 3.0, hy - kal * 3.0).abs());
    c1.max(c2)
}

pub fn maske(ad: &str, w: u32, h: u32, boy: f32) -> Option<Maske> {
    if !ADLAR.contains(&ad) { return None; }
    let (cx, cy) = (w as f32 / 2.0, h as f32 / 2.0);
    let mut a = vec![0f32; (w * h) as usize];
    let r = boy / 2.0;
    let kal = (boy * 0.018).max(4.0);
    let (mut x0, mut y0, mut x1, mut y1) = (w, h, 0, 0);
    for y in 0..h {
        for x in 0..w {
            let (dx, dy) = (x as f32 - cx, y as f32 - cy);
            let v = if ad == "cerceve" { cerceve(dx, dy, w as f32 * 0.46, h as f32 * 0.44, kal) } else { alan(ad, dx, dy, r) };
            if v > 0.0 { let i = (y * w + x) as usize; a[i] = v; x0 = x0.min(x); x1 = x1.max(x); y0 = y0.min(y); y1 = y1.max(y); }
        }
    }
    Some(Maske { w, h, a, bx: (x0, y0, x1, y1) })
}

#[cfg(test)]
mod t {
    use super::*;
    #[test]
    fn hepsi_cizilir() {
        for ad in ADLAR { let m = maske(ad, 400, 300, 240.0).unwrap(); assert!(m.bx.2 > m.bx.0, "{ad}"); }
        assert!(maske("yok", 10, 10, 5.0).is_none());
    }
    #[test]
    fn cicek_merkezi_bos() {
        let m = maske("cicek", 400, 300, 240.0).unwrap();
        assert_eq!(m.a[(150 * 400 + 200) as usize], 0.0);
    }
}
