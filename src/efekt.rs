//! Arka plan/üst efektler: ışık süpürme, pırıltı yıldızları, koşan ışıklı çerçeve.
use crate::Renk;

pub fn hash(n: u32) -> f32 {
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

fn nokta(t: &mut [u8], w: u32, h: u32, x: f32, y: f32, r: f32, renk: Renk, par: f32) {
    let (x0, x1, y0, y1) = ((x - r - 1.0).max(0.0) as u32, ((x + r + 1.0) as u32).min(w.saturating_sub(1)), (y - r - 1.0).max(0.0) as u32, ((y + r + 1.0) as u32).min(h.saturating_sub(1)));
    if x + r < 0.0 || y + r < 0.0 { return; }
    for py in y0..=y1 { for px in x0..=x1 {
        let d = ((px as f32 - x).powi(2) + (py as f32 - y).powi(2)).sqrt();
        let a = ((1.0 - d / r).clamp(0.0, 1.0)).powf(0.7) * par;
        if a <= 0.0 { continue; }
        let i = ((py * w + px) * 3) as usize;
        for (c, v) in [renk.0, renk.1, renk.2].iter().enumerate() { t[i + c] = t[i + c].max((*v as f32 * a) as u8); }
    } }
}

/// Yukarı süzülen, sallanan, parlayıp sönen yumuşak parçacıklar (arka planda).
pub fn parcaciklar(t: &mut [u8], w: u32, h: u32, zaman: f32, n: u32, renk: Renk, tohum: u32) {
    for k in 0..n {
        let s = |o: u32| hash(tohum * 1000 + k * 7 + o);
        let hiz = 50.0 + s(0) * 160.0;
        let dongu = h as f32 + 80.0;
        let y = h as f32 + 40.0 - (zaman * hiz + s(1) * dongu).rem_euclid(dongu);
        let x = s(2) * w as f32 + (zaman * (0.6 + s(3) * 1.5) + s(4) * 6.28).sin() * (20.0 + s(5) * 50.0);
        let r = 4.0 + s(6) * 12.0 * (w as f32 / 980.0);
        let par = 0.35 + 0.65 * ((zaman * (1.0 + s(7) * 2.5) + s(4) * 6.28).sin() * 0.5 + 0.5);
        nokta(t, w, h, x, y, r, renk, par);
    }
}

/// Hızlı çapraz uçan kıvılcımlar, kısa kuyruklu.
pub fn kivilcimlar(t: &mut [u8], w: u32, h: u32, zaman: f32, n: u32, renk: Renk, tohum: u32) {
    for k in 0..n {
        let s = |o: u32| hash(tohum * 1000 + k * 11 + o);
        let hiz = 0.25 + s(0) * 0.45; // ekran/sn
        let p = (zaman * hiz + s(1)).rem_euclid(1.0);
        let (bx, by) = (s(2) * w as f32 * 1.2 - w as f32 * 0.1, h as f32 * 1.05);
        let (vx, vy) = (w as f32 * (0.15 + s(3) * 0.5) * if s(4) > 0.5 { 1.0 } else { -1.0 }, -(h as f32) * 1.1);
        for kuyruk in 0..7 {
            let q = (p - kuyruk as f32 * 0.006).max(0.0);
            nokta(t, w, h, bx + vx * q, by + vy * q, (3.5 - kuyruk as f32 * 0.4) * (w as f32 / 980.0), renk, 1.0 - kuyruk as f32 / 7.0);
        }
    }
}

/// Yavaşça düşen, sallanan beyaz kar taneleri.
pub fn kar(t: &mut [u8], w: u32, h: u32, zaman: f32, n: u32, tohum: u32) {
    for k in 0..n {
        let s = |o: u32| hash(tohum * 1000 + k * 13 + o);
        let hiz = 40.0 + s(0) * 90.0;
        let dongu = h as f32 + 60.0;
        let y = (zaman * hiz + s(1) * dongu).rem_euclid(dongu) - 30.0;
        let x = s(2) * w as f32 + (zaman * (0.5 + s(3)) + s(4) * 6.28).sin() * (15.0 + s(5) * 35.0);
        nokta(t, w, h, x, y, (3.0 + s(6) * 8.0) * (w as f32 / 980.0), Renk(210, 210, 205), 0.5 + 0.5 * s(7));
    }
}

/// Hızlı eğik ince yağmur çizgileri.
pub fn yagmur(t: &mut [u8], w: u32, h: u32, zaman: f32, n: u32, renk: Renk, tohum: u32) {
    for k in 0..n {
        let s = |o: u32| hash(tohum * 1000 + k * 17 + o);
        let hiz = 500.0 + s(0) * 500.0;
        let dongu = h as f32 + 200.0;
        let y0 = (zaman * hiz + s(1) * dongu).rem_euclid(dongu) - 100.0;
        let x0 = s(2) * w as f32 - (y0 * 0.12);
        for q in 0..9 {
            let yy = y0 - q as f32 * 9.0;
            nokta(t, w, h, x0 + (y0 - yy) * 0.12, yy, (2.2 - q as f32 * 0.2).max(0.8) * (w as f32 / 980.0), renk, 0.7 - q as f32 * 0.07);
        }
    }
}

/// Merkezden dışa yayılan ince, yumuşak halkalar (sakin, yavaş).
pub fn halkalar(t: &mut [u8], w: u32, h: u32, zaman: f32, renk: Renk) {
    let (cx, cy) = (w as f32 / 2.0, h as f32 / 2.0);
    let maks = ((w * w + h * h) as f32).sqrt() / 2.0;
    for i in 0..3 {
        let p = (zaman * 0.22 + i as f32 / 3.0).rem_euclid(1.0);
        let r = p * maks;
        let a = (1.0 - p) * 0.55;
        let adim = 90;
        for j in 0..adim {
            let aci = j as f32 / adim as f32 * 6.2832;
            nokta(t, w, h, cx + aci.cos() * r, cy + aci.sin() * r, 3.0 * (w as f32 / 980.0), renk, a);
        }
    }
}

/// Ara sıra ekranı çaprazlayan kayan yıldızlar (kuyruklu).
pub fn kayan_yildizlar(t: &mut [u8], w: u32, h: u32, zaman: f32, renk: Renk, tohum: u32) {
    for k in 0..4u32 {
        let s = |o: u32| hash(tohum * 100 + k * 5 + o);
        let periyot = 2.5 + s(0) * 2.0;
        let p = ((zaman + s(1) * periyot) / periyot).fract() * periyot / 1.0; // 0..periyot
        if p > 1.0 { continue; }
        let (bx, by) = (s(2) * w as f32 * 0.8 + w as f32 * 0.2, s(3) * h as f32 * 0.4);
        let (vx, vy) = (-(w as f32) * 0.9, h as f32 * 0.5);
        for kuyruk in 0..14 {
            let q = (p - kuyruk as f32 * 0.012).max(0.0);
            nokta(t, w, h, bx + vx * q, by + vy * q, (4.5 - kuyruk as f32 * 0.3).max(1.0) * (w as f32 / 980.0), renk, (1.0 - kuyruk as f32 / 14.0) * (1.0 - p * 0.5));
        }
    }
}

#[cfg(test)]
mod t {
    use super::*;
    #[test]
    fn parcacik_ve_kivilcim_cizer() {
        let (w, h) = (300u32, 500u32);
        let mut a = vec![0u8; (w * h * 3) as usize];
        parcaciklar(&mut a, w, h, 1.0, 40, Renk(255, 206, 48), 1);
        assert!(a.iter().any(|&b| b > 100));
        let mut b = vec![0u8; (w * h * 3) as usize];
        kivilcimlar(&mut b, w, h, 0.5, 40, Renk(255, 206, 48), 1);
        assert!(b.iter().any(|&x| x > 100));
    }
    #[test]
    fn yeni_efektler_cizer() {
        let (w, h) = (300u32, 500u32);
        for (ad, f) in [("kar", 0), ("yagmur", 1), ("halka", 2), ("kayan", 3)] {
            let mut a = vec![0u8; (w * h * 3) as usize];
            for zaman in [0.2f32, 0.7, 1.3] {
                match f { 0 => kar(&mut a, w, h, zaman, 60, 1), 1 => yagmur(&mut a, w, h, zaman, 60, Renk(255, 206, 48), 1), 2 => halkalar(&mut a, w, h, zaman, Renk(255, 206, 48)), _ => kayan_yildizlar(&mut a, w, h, zaman, Renk(255, 206, 48), 1) }
            }
            assert!(a.iter().any(|&b| b > 100), "{ad}");
        }
    }
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
