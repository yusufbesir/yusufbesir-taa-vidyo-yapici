//! Dekor efektleri: çiçek, yaprak, böcek, kalp, balon, havai fişek vb. Hepsi yumuşak kenarlı, ani parlama içermez (LED güvenliği).
use crate::efekt::hash;
use crate::Renk;

/// Döndürülmüş yumuşak kenarlı elips; alfa karışımı.
pub fn elips(t: &mut [u8], w: u32, h: u32, cx: f32, cy: f32, rx: f32, ry: f32, aci: f32, r: Renk, alfa: f32) {
    let m = rx.max(ry) + 2.0;
    let (x0, x1, y0, y1) = ((cx - m).floor().max(0.0) as i32, (cx + m).ceil().min(w as f32 - 1.0) as i32, (cy - m).floor().max(0.0) as i32, (cy + m).ceil().min(h as f32 - 1.0) as i32);
    let (c, s) = (aci.cos(), aci.sin());
    for y in y0..=y1 {
        for x in x0..=x1 {
            let (dx, dy) = (x as f32 - cx, y as f32 - cy);
            let (u, v) = ((dx * c + dy * s) / rx.max(0.5), (-dx * s + dy * c) / ry.max(0.5));
            let d = (u * u + v * v).sqrt();
            if d >= 1.0 { continue; }
            let a = ((1.0 - d) * rx.min(ry).max(1.0) * 0.8).clamp(0.0, 1.0) * alfa;
            let i = ((y as u32 * w + x as u32) * 3) as usize;
            t[i] = (t[i] as f32 + (r.0 as f32 - t[i] as f32) * a) as u8;
            t[i + 1] = (t[i + 1] as f32 + (r.1 as f32 - t[i + 1] as f32) * a) as u8;
            t[i + 2] = (t[i + 2] as f32 + (r.2 as f32 - t[i + 2] as f32) * a) as u8;
        }
    }
}

fn kos(r: Renk, k: f32) -> Renk { Renk((r.0 as f32 * k) as u8, (r.1 as f32 * k) as u8, (r.2 as f32 * k) as u8) }

fn cicek(t: &mut [u8], w: u32, h: u32, x: f32, y: f32, boy: f32, aci: f32, yaprak: usize, tac: Renk, orta: Renk) {
    for k in 0..yaprak {
        let a = aci + k as f32 / yaprak as f32 * std::f32::consts::TAU;
        elips(t, w, h, x + a.cos() * boy * 0.55, y + a.sin() * boy * 0.55, boy * 0.5, boy * 0.22, a, tac, 0.95);
    }
    elips(t, w, h, x, y, boy * 0.28, boy * 0.28, 0.0, orta, 1.0);
}

/// Aşağı süzülen öğeler için ortak konum: (x, y, faz). hız: ekran/sn oranı.
fn dusen(w: u32, h: u32, zaman: f32, k: u32, tohum: u32, hiz: f32, boy: f32) -> (f32, f32, f32) {
    let r = |j: u32| hash(tohum * 1000 + k * 7 + j);
    let yol = h as f32 + 2.0 * boy;
    let y = ((zaman * hiz * (0.7 + 0.6 * r(1)) + r(0)) % 1.0) * yol - boy;
    let x = r(2) * w as f32 + (zaman * (0.8 + r(3)) + r(4) * 6.28).sin() * w as f32 * 0.04;
    (x, y, r(5))
}

pub fn cicek_yagmuru(t: &mut [u8], w: u32, h: u32, z: f32, n: u32, renk: Renk, tohum: u32) {
    for k in 0..n { let b = w as f32 * (0.05 + 0.04 * hash(k + tohum)); let (x, y, f) = dusen(w, h, z, k, tohum, 0.09, b * 2.0); cicek(t, w, h, x, y, b, z * (0.4 + f) + f * 6.0, 5, renk, kos(renk, 0.6)); }
}
pub fn papatya(t: &mut [u8], w: u32, h: u32, z: f32, n: u32, _r: Renk, tohum: u32) {
    for k in 0..n { let b = w as f32 * (0.07 + 0.04 * hash(k + tohum)); let (x, y, f) = dusen(w, h, z, k, tohum + 5, 0.07, b * 2.0); cicek(t, w, h, x, y, b, z * 0.5 + f * 6.0, 10, Renk(236, 236, 226), Renk(255, 190, 0)); }
}
pub fn yaprak(t: &mut [u8], w: u32, h: u32, z: f32, n: u32, _r: Renk, tohum: u32) {
    let ton = [Renk(60, 160, 70), Renk(210, 140, 30), Renk(180, 70, 30), Renk(120, 180, 60)];
    for k in 0..n { let b = w as f32 * (0.04 + 0.03 * hash(k + tohum)); let (x, y, f) = dusen(w, h, z, k, tohum + 9, 0.1, b * 2.0); elips(t, w, h, x, y, b * 1.6, b * 0.7, z * (0.8 + f) + f * 6.0, ton[(f * 4.0) as usize % 4], 0.95); }
}
pub fn kelebek(t: &mut [u8], w: u32, h: u32, z: f32, n: u32, renk: Renk, tohum: u32) {
    for k in 0..n {
        let r = |j: u32| hash(tohum * 500 + k * 9 + j);
        let boy = w as f32 * (0.07 + 0.03 * r(0));
        let u = z * (0.12 + 0.05 * r(1)) + r(2);
        let x = w as f32 * (0.15 + 0.7 * (0.5 + 0.5 * (u * 6.28).sin()));
        let y = h as f32 * ((u * 0.9 + r(3)) % 1.0) * 0.9 + h as f32 * 0.05 + (z * 2.0 + r(4) * 6.0).sin() * 20.0;
        let kanat = 0.35 + 0.65 * (z * 9.0 + r(5) * 6.0).sin().abs();
        let ton = [Renk(255, 140, 20), renk, Renk(90, 160, 255)][k as usize % 3];
        for yon in [-1.0f32, 1.0] {
            elips(t, w, h, x + yon * boy * 0.5 * kanat, y - boy * 0.15, boy * 0.55 * kanat, boy * 0.4, yon * 0.5, ton, 0.95);
            elips(t, w, h, x + yon * boy * 0.4 * kanat, y + boy * 0.25, boy * 0.38 * kanat, boy * 0.28, -yon * 0.4, kos(ton, 0.75), 0.95);
        }
        elips(t, w, h, x, y, boy * 0.06, boy * 0.4, 0.0, Renk(30, 20, 20), 1.0);
    }
}
pub fn ugur_bocegi(t: &mut [u8], w: u32, h: u32, z: f32, n: u32, _r: Renk, tohum: u32) {
    for k in 0..n {
        let r = |j: u32| hash(tohum * 300 + k * 5 + j);
        let boy = w as f32 * (0.05 + 0.02 * r(0));
        let u = (z * (0.05 + 0.03 * r(1)) + r(2)) % 1.0;
        let x = if k % 2 == 0 { u * (w as f32 + 2.0 * boy) - boy } else { (1.0 - u) * (w as f32 + 2.0 * boy) - boy };
        let y = h as f32 * (0.12 + 0.76 * r(3)) + (z * 3.0 + r(4) * 6.0).sin() * 10.0;
        elips(t, w, h, x, y, boy, boy * 0.85, 0.0, Renk(215, 30, 30), 1.0);
        elips(t, w, h, x, y - boy * 0.85, boy * 0.45, boy * 0.35, 0.0, Renk(25, 25, 25), 1.0);
        for (dx, dy) in [(-0.45, 0.0), (0.45, 0.0), (-0.3, 0.5), (0.3, 0.5)] { elips(t, w, h, x + dx * boy, y + dy * boy, boy * 0.16, boy * 0.16, 0.0, Renk(25, 25, 25), 1.0); }
    }
}
pub fn ari(t: &mut [u8], w: u32, h: u32, z: f32, n: u32, _r: Renk, tohum: u32) {
    for k in 0..n {
        let r = |j: u32| hash(tohum * 700 + k * 5 + j);
        let boy = w as f32 * (0.045 + 0.02 * r(0));
        let u = z * (0.1 + 0.05 * r(1)) + r(2);
        let x = w as f32 * (0.5 + 0.4 * (u * 6.28).sin() * (u * 2.0).cos());
        let y = h as f32 * (0.5 + 0.38 * (u * 4.0 + r(3) * 6.0).sin()) + (z * 20.0).sin() * 4.0;
        let kn = 0.6 + 0.4 * (z * 40.0 + r(4)).sin().abs();
        for yon in [-1.0f32, 1.0] { elips(t, w, h, x + yon * boy * 0.4, y - boy * 0.7, boy * 0.55 * kn, boy * 0.28, yon * 0.7, Renk(210, 230, 245), 0.7); }
        elips(t, w, h, x, y, boy * 0.9, boy * 0.6, 0.0, Renk(250, 190, 20), 1.0);
        for dx in [-0.35f32, 0.1, 0.5] { elips(t, w, h, x + dx * boy, y, boy * 0.1, boy * 0.55, 0.0, Renk(30, 25, 10), 1.0); }
    }
}
pub fn ates_bocegi(t: &mut [u8], w: u32, h: u32, z: f32, n: u32, _r: Renk, tohum: u32) {
    for k in 0..n {
        let r = |j: u32| hash(tohum * 900 + k * 5 + j);
        let p = 0.5 + 0.5 * (z * (0.9 + 0.5 * r(0)) + r(1) * 6.28).sin(); // yavaş atım, flaş yok
        let x = r(2) * w as f32 + (z * 0.6 + r(3) * 6.0).sin() * 40.0;
        let y = r(4) * h as f32 + (z * 0.5 + r(2) * 6.0).cos() * 40.0;
        elips(t, w, h, x, y, 22.0, 22.0, 0.0, Renk(200, 255, 80), 0.15 + 0.35 * p);
        elips(t, w, h, x, y, 8.0, 8.0, 0.0, Renk(240, 255, 170), 0.5 + 0.5 * p);
    }
}
fn kalp_ciz(t: &mut [u8], w: u32, h: u32, x: f32, y: f32, b: f32, r: Renk) {
    elips(t, w, h, x - b * 0.3, y - b * 0.2, b * 0.4, b * 0.4, 0.0, r, 1.0);
    elips(t, w, h, x + b * 0.3, y - b * 0.2, b * 0.4, b * 0.4, 0.0, r, 1.0);
    elips(t, w, h, x, y + b * 0.15, b * 0.42, b * 0.5, std::f32::consts::FRAC_PI_4 * 0.0, r, 1.0);
    for s in 0..6 { let k = s as f32 / 6.0; elips(t, w, h, x, y + b * (0.1 + 0.4 * k), b * 0.55 * (1.0 - k) + 1.0, b * 0.2, 0.0, r, 1.0); }
}
pub fn kalp(t: &mut [u8], w: u32, h: u32, z: f32, n: u32, renk: Renk, tohum: u32) {
    for k in 0..n { let b = w as f32 * (0.05 + 0.04 * hash(k + tohum)); let (x, y, _) = dusen(w, h, z, k, tohum + 21, 0.08, b * 2.0); kalp_ciz(t, w, h, x, h as f32 - y, b, renk); }
}
pub fn yildiz_yagmuru(t: &mut [u8], w: u32, h: u32, z: f32, n: u32, renk: Renk, tohum: u32) {
    for k in 0..n {
        let b = w as f32 * (0.02 + 0.02 * hash(k + tohum));
        let (x, y, f) = dusen(w, h, z, k, tohum + 31, 0.12, b * 3.0);
        let a = z * (0.5 + f) + f * 6.0;
        for q in 0..4 { let aa = a + q as f32 * std::f32::consts::FRAC_PI_4; elips(t, w, h, x, y, b * 2.2, b * 0.35, aa, renk, 0.95); }
        elips(t, w, h, x, y, b * 0.6, b * 0.6, 0.0, Renk(255, 255, 255), 0.8);
    }
}
pub fn konfeti(t: &mut [u8], w: u32, h: u32, z: f32, n: u32, _r: Renk, tohum: u32) {
    let ton = [Renk(250, 70, 90), Renk(60, 190, 255), Renk(255, 215, 40), Renk(120, 230, 120), Renk(220, 120, 255)];
    for k in 0..n {
        let b = w as f32 * 0.016;
        let (x, y, f) = dusen(w, h, z, k, tohum + 41, 0.2, b * 3.0);
        let dn = (z * (4.0 + 6.0 * f) + f * 6.0).sin().abs();
        elips(t, w, h, x, y, b * 1.4, (b * 0.8 * dn).max(1.5), z * 2.0 + f * 6.0, ton[k as usize % 5], 0.95);
    }
}
pub fn balon(t: &mut [u8], w: u32, h: u32, z: f32, n: u32, renk: Renk, tohum: u32) {
    let ton = [renk, Renk(250, 90, 120), Renk(90, 190, 255), Renk(255, 200, 50), Renk(140, 230, 140)];
    for k in 0..n {
        let b = w as f32 * (0.06 + 0.03 * hash(k + tohum));
        let (x, y0, _) = dusen(w, h, z, k, tohum + 51, 0.07, b * 3.0);
        let y = h as f32 - y0;
        let c = ton[k as usize % 5];
        elips(t, w, h, x, y, b, b * 1.25, 0.0, c, 0.95);
        elips(t, w, h, x - b * 0.3, y - b * 0.45, b * 0.18, b * 0.3, 0.5, Renk(255, 255, 255), 0.35);
        for s in 0..8 { let q = s as f32; elips(t, w, h, x + (q * 0.9 + z * 3.0).sin() * 3.0, y + b * 1.3 + q * b * 0.18, 1.6, b * 0.1, 0.0, Renk(210, 210, 210), 0.7); }
    }
}
pub fn havai_fisek(t: &mut [u8], w: u32, h: u32, z: f32, n: u32, renk: Renk, tohum: u32) {
    for k in 0..n {
        let r = |j: u32| hash(tohum * 1100 + k * 7 + j);
        let period = 2.4 + r(0) * 1.2;
        let u = ((z + r(1) * period) % period) / period; // 0..1
        let (cx, cy) = (w as f32 * (0.2 + 0.6 * r(2)), h as f32 * (0.2 + 0.4 * r(3)));
        let ton = [renk, Renk(255, 120, 150), Renk(120, 200, 255), Renk(255, 220, 90)][k as usize % 4];
        if u < 0.3 { // yükselen iz
            let p = u / 0.3; let y = h as f32 - (h as f32 - cy) * p;
            elips(t, w, h, cx, y, 6.0, 14.0, 0.0, ton, 0.8);
        } else { // patlama: büyüyen ve sönen parçacıklar
            let p = (u - 0.3) / 0.7; let yar = w as f32 * 0.22 * (1.0 - (1.0 - p).powi(2)); let a = (1.0 - p).powf(1.3);
            for q in 0..18 { let ang = q as f32 / 18.0 * std::f32::consts::TAU + r(4); elips(t, w, h, cx + ang.cos() * yar, cy + ang.sin() * yar + p * p * 60.0, 7.0, 7.0, 0.0, ton, a * 0.9); }
        }
    }
}
pub fn pirilti_bulutu(t: &mut [u8], w: u32, h: u32, z: f32, n: u32, renk: Renk, tohum: u32) {
    for k in 0..n {
        let r = |j: u32| hash(tohum * 1300 + k * 7 + j);
        let p = ((z * (0.6 + r(0)) + r(1) * 6.28).sin() * 0.5 + 0.5).powf(2.0);
        let (x, y) = (r(2) * w as f32, r(3) * h as f32);
        let b = 6.0 + 18.0 * p;
        elips(t, w, h, x, y, b, b * 0.2 + 1.0, 0.0, renk, 0.9 * p);
        elips(t, w, h, x, y, b * 0.2 + 1.0, b, 0.0, renk, 0.9 * p);
        elips(t, w, h, x, y, 3.0, 3.0, 0.0, Renk(255, 255, 255), p);
    }
}

pub const ADLAR: &[(&str, &str)] = &[
    ("cicek-yagmuru", "Yağan çiçekler (palet vurgu rengi)"), ("papatya", "Süzülen papatyalar"), ("yaprak", "Dönerek düşen sonbahar yaprakları"),
    ("kelebek", "Kanat çırpan kelebekler"), ("ugur-bocegi", "Yürüyen uğur böcekleri"), ("ari", "Vızıldayan arılar"),
    ("ates-bocegi", "Yavaş yanıp sönen ateş böcekleri"), ("kalp", "Yükselen kalpler"), ("yildiz-yagmuru", "Dönerek düşen yıldızlar"),
    ("konfeti", "Konfeti yağmuru"), ("balon", "Yükselen balonlar"), ("havai-fisek", "Havai fişek (tam ekran flaş yok)"), ("pirilti-bulutu", "Yavaş parlayan ışıltılar"),
];

/// Ad bilinen bir dekor ise çizer ve true döner.
pub fn ciz(ad: &str, t: &mut [u8], w: u32, h: u32, z: f32, renk: Renk, tohum: u32) -> bool {
    match ad {
        "cicek-yagmuru" => cicek_yagmuru(t, w, h, z, 9, renk, tohum),
        "papatya" => papatya(t, w, h, z, 7, renk, tohum),
        "yaprak" => yaprak(t, w, h, z, 14, renk, tohum),
        "kelebek" => kelebek(t, w, h, z, 4, renk, tohum),
        "ugur-bocegi" => ugur_bocegi(t, w, h, z, 5, renk, tohum),
        "ari" => ari(t, w, h, z, 4, renk, tohum),
        "ates-bocegi" => ates_bocegi(t, w, h, z, 16, renk, tohum),
        "kalp" => kalp(t, w, h, z, 8, renk, tohum),
        "yildiz-yagmuru" => yildiz_yagmuru(t, w, h, z, 14, renk, tohum),
        "konfeti" => konfeti(t, w, h, z, 60, renk, tohum),
        "balon" => balon(t, w, h, z, 6, renk, tohum),
        "havai-fisek" => havai_fisek(t, w, h, z, 3, renk, tohum),
        "pirilti-bulutu" => pirilti_bulutu(t, w, h, z, 24, renk, tohum),
        _ => return false,
    }
    true
}
