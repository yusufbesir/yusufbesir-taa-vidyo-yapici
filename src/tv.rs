//! Tüplü (CRT) televizyon efekti: açılış çizgisi, bombeli ekran, tarama çizgileri, kapanış.

fn hash(n: u32) -> f32 {
    let mut x = n.wrapping_mul(0x9E3779B1) ^ 0x85EBCA6B;
    x ^= x >> 15; x = x.wrapping_mul(0x2C1B3C6D); x ^= x >> 12;
    (x & 0xFFFF) as f32 / 65535.0
}
fn yumusak(t: f32) -> f32 { let t = t.clamp(0.0, 1.0); t * t * (3.0 - 2.0 * t) }

#[derive(Clone, Copy, Debug)]
pub struct Tup { pub sv: f32, pub sh: f32, pub flas: f32, pub parlak: f32, pub gurultu: f32 }

pub const ACILIS_SN: f32 = 1.5;
pub const KAPANIS_SN: f32 = 1.4;

/// Açılış: yatay çizgi genişler → dikey açılır, beyaz parlama söner, karlama azalır.
pub fn acilis(t: f32) -> Tup {
    let ince = 0.012;
    if t < 0.45 {
        let k = yumusak(t / 0.45);
        Tup { sv: ince, sh: 0.004 + 0.996 * k, flas: 0.6, parlak: 1.0, gurultu: 0.0 }
    } else if t < 1.1 {
        let k = yumusak((t - 0.45) / 0.65);
        Tup { sv: ince + (1.0 - ince) * k, sh: 1.0, flas: 0.6 * (1.0 - k), parlak: 1.0, gurultu: 0.25 * (1.0 - k) }
    } else {
        let r = ((t - 1.1) * 40.0).sin() * 0.04 * (1.0 - ((t - 1.1) / 0.4).min(1.0));
        Tup { sv: 1.0, sh: 1.0, flas: 0.0, parlak: 1.0 + r, gurultu: 0.0 }
    }
}

/// Kapanış: üstten ve alttan kapanıp ince çizgiye iner, çizgi yatayda küçülür, nokta söner.
pub fn kapanis(t: f32) -> Tup {
    let ince = 0.012;
    if t < 0.6 {
        let k = (t / 0.6).powf(2.0);
        Tup { sv: 1.0 - (1.0 - ince) * k, sh: 1.0, flas: 0.6 * k, parlak: 1.0, gurultu: 0.0 }
    } else if t < 1.1 {
        let k = ((t - 0.6) / 0.5).powf(2.0);
        Tup { sv: ince, sh: 1.0 - 0.996 * k, flas: 0.6, parlak: 1.0, gurultu: 0.0 }
    } else {
        let k = ((t - 1.1) / (KAPANIS_SN - 1.1)).clamp(0.0, 1.0);
        Tup { sv: ince, sh: 0.004, flas: 0.6, parlak: 1.0 - k, gurultu: 0.0 }
    }
}

/// Kareyi tüp ekrandan geçirir. `tohum`: karlama için kare numarası.
pub fn tup(kaynak: &[u8], w: u32, h: u32, p: Tup, tohum: u32) -> Vec<u8> {
    let mut cikis = vec![0u8; kaynak.len()];
    if p.parlak <= 0.0 { return cikis; }
    let (cx, cy) = (w as f32 / 2.0, h as f32 / 2.0);
    let k = 0.075_f32; // bombe
    for y in 0..h {
        let v = (y as f32 - cy) / cy;
        if v.abs() > p.sv { continue; }
        let pv = v / p.sv;
        let tarama = 1.0; // tarama çizgisi yok: 10-17 mm piksel aralığında 3 px'lik çizgi moiré/düzensiz parlaklık yapar
        for x in 0..w {
            let u = (x as f32 - cx) / cx;
            if u.abs() > p.sh { continue; }
            let pu = u / p.sh;
            let r2 = pu * pu + pv * pv;
            let (qu, qv) = (pu * (1.0 + k * r2), pv * (1.0 + k * r2));
            if qu.abs() >= 1.0 || qv.abs() >= 1.0 { continue; }
            let (sx, sy) = (((qu * cx + cx) as u32).min(w - 1), ((qv * cy + cy) as u32).min(h - 1));
            let si = ((sy * w + sx) * 3) as usize;
            let vin = 1.0 - 0.15 * r2.min(1.5);
            let n = if p.gurultu > 0.0 { (hash(tohum.wrapping_mul(7919) ^ (y * w + x)) - 0.5) * 255.0 * p.gurultu } else { 0.0 };
            let oi = ((y * w + x) * 3) as usize;
            for c in 0..3 {
                let mut d = kaynak[si + c] as f32 * vin * tarama * p.parlak + n;
                d = d + (255.0 - d) * p.flas;
                cikis[oi + c] = d.clamp(0.0, 255.0) as u8;
            }
        }
    }
    cikis
}

#[cfg(test)]
mod t {
    use super::*;
    #[test]
    fn zaman_cizelgesi_uclari() {
        assert!(acilis(0.0).sh < 0.01 && acilis(0.0).sv < 0.02);
        let son = acilis(ACILIS_SN);
        assert!((son.sv - 1.0).abs() < 1e-3 && son.flas == 0.0);
        assert!(kapanis(0.0).sv > 0.99);
        assert!(kapanis(KAPANIS_SN).parlak < 0.01);
    }
    #[test]
    fn tup_ortayi_boyar_koseyi_keser() {
        let (w, h) = (200u32, 300u32);
        let kaynak = vec![200u8; (w * h * 3) as usize];
        let c = tup(&kaynak, w, h, Tup { sv: 1.0, sh: 1.0, flas: 0.0, parlak: 1.0, gurultu: 0.0 }, 1);
        assert!(c[((150 * w + 100) * 3) as usize] > 100);
        assert_eq!(c[0], 0); // köşe siyah çerçeve
    }
    #[test]
    fn kapali_cizgi_beyaz() {
        let (w, h) = (200u32, 300u32);
        let kaynak = vec![10u8; (w * h * 3) as usize];
        let c = tup(&kaynak, w, h, kapanis(0.9), 1);
        assert!(c.iter().any(|&b| b > 140));
    }
}
