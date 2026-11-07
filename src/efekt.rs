//! Arka plan/üst efektler: ışık süpürme, pırıltı yıldızları, koşan ışıklı çerçeve.
use crate::Renk;

fn hash(n: u32) -> f32 {
    let mut x = n.wrapping_mul(0x9E3779B1) ^ 0x85EBCA6B;
    x ^= x >> 15; x = x.wrapping_mul(0x2C1B3C6D); x ^= x >> 12;
    (x & 0xFFFF) as f32 / 65535.0
}

/// Yazı piksellerinin (zeminden farklı) üzerinden geçen çapraz parlak bant. `konum` 0..1.
pub fn isik_supurme(t: &mut [u8], w: u32, h: u32, zemin: Renk, konum: f32) {
    if !(0.0..=1.0).contains(&konum) { return; }
    let toplam = (w + h / 2) as f32;
    let merkez = konum * (toplam + 200.0) - 100.0;
    let gen = 70.0;
    for y in 0..h {
        for x in 0..w {
            let d = ((x as f32 + y as f32 * 0.5) - merkez).abs();
            if d >= gen { continue; }
            let i = ((y * w + x) * 3) as usize;
            if t[i] == zemin.0 && t[i + 1] == zemin.1 && t[i + 2] == zemin.2 { continue; }
            let k = (1.0 - d / gen).powf(1.5) * 0.85;
            for c in 0..3 { t[i + c] = (t[i + c] as f32 + (255.0 - t[i + c] as f32) * k) as u8; }
        }
    }
}

/// Dört kollu pırıltılar; `n` adet, yerleri sabit, parlama fazı zamana bağlı.
pub fn piriltilar(t: &mut [u8], w: u32, h: u32, zaman: f32, n: u32, renk: Renk, tohum: u32) {
    for k in 0..n {
        let (x, y) = (hash(tohum * 100 + k * 3) * w as f32, hash(tohum * 100 + k * 3 + 1) * h as f32);
        let faz = hash(tohum * 100 + k * 3 + 2) * 6.28;
        let p = ((zaman * 3.0 + faz).sin() * 0.5 + 0.5).powf(3.0);
        let boy = 10.0 + 40.0 * p;
        if p < 0.05 { continue; }
        for d in -(boy as i32)..=(boy as i32) {
            let a = 1.0 - d.abs() as f32 / boy;
            for (px, py) in [(x as i32 + d, y as i32), (x as i32, y as i32 + d)] {
                for g in -1..=1 {
                    let (qx, qy) = if py == y as i32 { (px, py + g) } else { (px + g, py) };
                    if qx < 0 || qy < 0 || qx >= w as i32 || qy >= h as i32 { continue; }
                    let i = ((qy as u32 * w + qx as u32) * 3) as usize;
                    let v = a * p * if g == 0 { 1.0 } else { 0.5 };
                    t[i] = (t[i] as f32 + (renk.0 as f32 - t[i] as f32).max(0.0) * v).max(t[i] as f32) as u8;
                    t[i + 1] = (t[i + 1] as f32 + (renk.1 as f32 - t[i + 1] as f32).max(0.0) * v).max(t[i + 1] as f32) as u8;
                    t[i + 2] = (t[i + 2] as f32 + (renk.2 as f32 - t[i + 2] as f32).max(0.0) * v).max(t[i + 2] as f32) as u8;
                }
            }
        }
    }
}

/// Kenar boyunca koşan ışıklı çerçeve (dikey ekranda dikkat çeker).
pub fn kosan_cerceve(t: &mut [u8], w: u32, h: u32, zaman: f32, renk: Renk, kalinlik: u32, bosluk: u32) {
    let (iw, ih) = (w - 2 * bosluk, h - 2 * bosluk);
    let cevre = (2 * (iw + ih)) as f32;
    for y in bosluk..h - bosluk {
        for x in bosluk..w - bosluk {
            let (lx, ly) = (x - bosluk, y - bosluk);
            let kenar = lx.min(iw - 1 - lx).min(ly.min(ih - 1 - ly));
            if kenar >= kalinlik { continue; }
            let u = if ly < kalinlik { lx as f32 }
                else if lx >= iw - kalinlik { iw as f32 + ly as f32 }
                else if ly >= ih - kalinlik { (iw + ih) as f32 + (iw - lx) as f32 }
                else { (2 * iw + ih) as f32 + (ih - ly) as f32 };
            let ph = (u / cevre * 14.0 - zaman * 2.5).rem_euclid(1.0);
            let par = if ph < 0.5 { 1.0 - ph } else { 0.18 };
            let i = ((y * w + x) * 3) as usize;
            t[i] = (renk.0 as f32 * par) as u8; t[i + 1] = (renk.1 as f32 * par) as u8; t[i + 2] = (renk.2 as f32 * par) as u8;
        }
    }
}

/// Çift ince çizgili çerçeve (afiş stili).
pub fn cift_cerceve(t: &mut [u8], w: u32, h: u32, renk: Renk, kal: u32, bosluk: u32) {
    let ara = kal * 2 + 2;
    for y in 0..h {
        for x in 0..w {
            let d = x.min(w - 1 - x).min(y.min(h - 1 - y));
            if (d >= bosluk && d < bosluk + kal) || (d >= bosluk + ara && d < bosluk + ara + kal) {
                let i = ((y * w + x) * 3) as usize;
                t[i] = renk.0; t[i + 1] = renk.1; t[i + 2] = renk.2;
            }
        }
    }
}

#[cfg(test)]
mod t {
    use super::*;
    #[test]
    fn cerceve_ve_piriltilar_cizer() {
        let z = Renk(0, 0, 0);
        let mut k = vec![0u8; 300 * 500 * 3];
        kosan_cerceve(&mut k, 300, 500, 0.3, Renk(255, 200, 0), 6, 10);
        assert!(k.iter().any(|&b| b > 100));
        let mut k2 = vec![0u8; 300 * 500 * 3];
        piriltilar(&mut k2, 300, 500, 0.5, 20, Renk(255, 255, 255), 1);
        assert!(k2.iter().any(|&b| b > 100));
        let mut k3 = vec![0u8; 300 * 500 * 3];
        k3[0] = 200; // "yazı" pikseli
        isik_supurme(&mut k3, 300, 500, z, 0.0);
    }
}
