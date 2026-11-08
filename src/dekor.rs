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


// ---- Çerçeveler: kenar payı LED güvenli alanında; yumuşak, ani parlama yok ----

fn karistir(t: &mut [u8], i: usize, r: Renk, a: f32) {
    let a = a.clamp(0.0, 1.0);
    t[i] = (t[i] as f32 + (r.0 as f32 - t[i] as f32) * a) as u8;
    t[i + 1] = (t[i + 1] as f32 + (r.1 as f32 - t[i + 1] as f32) * a) as u8;
    t[i + 2] = (t[i + 2] as f32 + (r.2 as f32 - t[i + 2] as f32) * a) as u8;
}

/// Dolu dikdörtgen (kenarları dahil), alfa karışımı.
fn dik(t: &mut [u8], w: u32, h: u32, x0: f32, y0: f32, x1: f32, y1: f32, r: Renk, a: f32) {
    let (xa, xb, ya, yb) = (x0.max(0.0) as u32, (x1.min(w as f32)) as u32, y0.max(0.0) as u32, (y1.min(h as f32)) as u32);
    for y in ya..yb { for x in xa..xb { karistir(t, ((y * w + x) * 3) as usize, r, a); } }
}

/// Dikdörtgen çizgisi (içe doğru kalınlık).
fn halka_dik(t: &mut [u8], w: u32, h: u32, pay: f32, kalin: f32, r: Renk, a: f32) {
    let (x0, y0, x1, y1) = (pay, pay, w as f32 - pay, h as f32 - pay);
    dik(t, w, h, x0, y0, x1, y0 + kalin, r, a);
    dik(t, w, h, x0, y1 - kalin, x1, y1, r, a);
    dik(t, w, h, x0, y0 + kalin, x0 + kalin, y1 - kalin, r, a);
    dik(t, w, h, x1 - kalin, y0 + kalin, x1, y1 - kalin, r, a);
}

/// Dikdörtgen çevresinde 0..1 konum.
fn cevre(w: u32, h: u32, pay: f32, u: f32) -> (f32, f32) {
    let (x0, y0, x1, y1) = (pay, pay, w as f32 - pay, h as f32 - pay);
    let (gw, gh) = (x1 - x0, y1 - y0);
    let toplam = 2.0 * (gw + gh);
    let d = u.rem_euclid(1.0) * toplam;
    if d < gw { (x0 + d, y0) } else if d < gw + gh { (x1, y0 + d - gw) } else if d < 2.0 * gw + gh { (x1 - (d - gw - gh), y1) } else { (x0, y1 - (d - 2.0 * gw - gh)) }
}

fn nefes(z: f32, hiz: f32, alt: f32) -> f32 { alt + (1.0 - alt) * (0.5 + 0.5 * (z * hiz).sin()) }

fn cerceve_ince(t: &mut [u8], w: u32, h: u32, z: f32, r: Renk) { halka_dik(t, w, h, w as f32 * 0.035, (w as f32 / 170.0).max(3.0), r, nefes(z, 1.2, 0.7)); }

fn cerceve_cift(t: &mut [u8], w: u32, h: u32, z: f32, r: Renk) {
    let k = (w as f32 / 120.0).max(4.0);
    halka_dik(t, w, h, w as f32 * 0.03, k, r, nefes(z, 1.0, 0.8));
    halka_dik(t, w, h, w as f32 * 0.03 + k * 2.6, k * 0.45, r, nefes(z, 1.0, 0.6));
}

fn cerceve_kose(t: &mut [u8], w: u32, h: u32, z: f32, r: Renk) {
    let (pay, k, uz) = (w as f32 * 0.04, (w as f32 / 100.0).max(5.0), w as f32 * 0.16);
    let a = nefes(z, 1.4, 0.75);
    let (x1, y1) = (w as f32 - pay, h as f32 - pay);
    for (cx, cy, sx, sy) in [(pay, pay, 1.0f32, 1.0f32), (x1, pay, -1.0, 1.0), (pay, y1, 1.0, -1.0), (x1, y1, -1.0, -1.0)] {
        let (xa, xb) = if sx > 0.0 { (cx, cx + uz) } else { (cx - uz, cx) };
        let (ya, yb) = if sy > 0.0 { (cy, cy + k) } else { (cy - k, cy) };
        dik(t, w, h, xa, ya, xb, yb, r, a);
        let (xa, xb) = if sx > 0.0 { (cx, cx + k) } else { (cx - k, cx) };
        let (ya, yb) = if sy > 0.0 { (cy, cy + uz) } else { (cy - uz, cy) };
        dik(t, w, h, xa, ya, xb, yb, r, a);
    }
}

fn cerceve_nokta(t: &mut [u8], w: u32, h: u32, z: f32, r: Renk) {
    let pay = w as f32 * 0.045;
    let n = 56;
    let boy = (w as f32 / 70.0).max(4.0);
    for k in 0..n {
        let u = k as f32 / n as f32;
        let (x, y) = cevre(w, h, pay, u);
        let faz = (u - z * 0.18).rem_euclid(1.0);
        let a = 0.25 + 0.75 * (0.5 + 0.5 * (faz * 6.2832 * 3.0).cos());
        elips(t, w, h, x, y, boy, boy, 0.0, r, a);
    }
}

fn cerceve_neon(t: &mut [u8], w: u32, h: u32, z: f32, r: Renk) {
    let pay = w as f32 * 0.04;
    let k = (w as f32 / 150.0).max(3.0);
    let n = nefes(z, 0.9, 0.75);
    for (kat, alfa) in [(5.0f32, 0.10f32), (3.2, 0.18), (1.8, 0.35)] { halka_dik(t, w, h, pay - k * (kat - 1.0) / 2.0, k * kat, r, alfa * n); }
    halka_dik(t, w, h, pay, k, Renk(255.min(r.0 as u32 + 90) as u8, 255.min(r.1 as u32 + 90) as u8, 255.min(r.2 as u32 + 90) as u8), 0.95 * n);
}

fn cerceve_yildiz(t: &mut [u8], w: u32, h: u32, z: f32, r: Renk) {
    let pay = w as f32 * 0.07;
    let (x1, y1) = (w as f32 - pay, h as f32 - pay);
    let boy = w as f32 * 0.07;
    for (i, (cx, cy)) in [(pay, pay), (x1, pay), (pay, y1), (x1, y1)].iter().enumerate() {
        let a = nefes(z + i as f32 * 1.3, 1.1, 0.55);
        elips(t, w, h, *cx, *cy, boy, boy * 0.16, 0.0, r, a);
        elips(t, w, h, *cx, *cy, boy * 0.16, boy, 0.0, r, a);
        elips(t, w, h, *cx, *cy, boy * 0.55, boy * 0.09, 0.785, r, a * 0.7);
        elips(t, w, h, *cx, *cy, boy * 0.55, boy * 0.09, -0.785, r, a * 0.7);
    }
    halka_dik(t, w, h, pay, (w as f32 / 260.0).max(2.0), r, 0.5);
    for k in 0..14u32 {
        let (x, y) = cevre(w, h, pay, hash(k * 17 + 3));
        let a = nefes(z * 0.8 + hash(k + 40) * 6.28, 1.0, 0.1) * 0.85;
        elips(t, w, h, x, y, boy * 0.12, boy * 0.12, 0.0, r, a);
    }
}

fn cerceve_dalga(t: &mut [u8], w: u32, h: u32, z: f32, r: Renk) {
    let kal = (w as f32 / 130.0).max(3.0);
    let genlik = w as f32 * 0.02;
    for x in (0..w).step_by(2) {
        let fx = x as f32 / w as f32 * 6.2832 * 2.0;
        for (taban, isaret) in [(w as f32 * 0.05, 1.0f32), (h as f32 - w as f32 * 0.05, -1.0)] {
            let y = taban + (fx + z * 1.2 * isaret).sin() * genlik;
            dik(t, w, h, x as f32, y - kal / 2.0, x as f32 + 2.0, y + kal / 2.0, r, 0.9);
        }
    }
}

fn cerceve_isik(t: &mut [u8], w: u32, h: u32, z: f32, r: Renk) {
    let pay = w as f32 * 0.04;
    halka_dik(t, w, h, pay, (w as f32 / 220.0).max(2.0), r, 0.3);
    let boy = (w as f32 / 60.0).max(5.0);
    for k in 0..40 {
        let iz = k as f32 / 40.0;
        let u = z * 0.12 - iz * 0.18;
        let (x, y) = cevre(w, h, pay, u);
        elips(t, w, h, x, y, boy * (1.0 - iz * 0.5), boy * (1.0 - iz * 0.5), 0.0, r, 1.0 - iz);
    }
}

fn cerceve_altin(t: &mut [u8], w: u32, h: u32, z: f32, r: Renk) {
    let k = (w as f32 / 70.0).max(6.0);
    let pay = w as f32 * 0.025;
    halka_dik(t, w, h, pay, k, r, 0.95);
    halka_dik(t, w, h, pay + k * 1.8, k * 0.35, r, nefes(z, 1.0, 0.7));
    let b = k * 2.6;
    let (x1, y1) = (w as f32 - pay, h as f32 - pay);
    for (cx, cy) in [(pay, pay), (x1, pay), (pay, y1), (x1, y1)] { dik(t, w, h, cx - b / 2.0, cy - b / 2.0, cx + b / 2.0, cy + b / 2.0, r, 1.0); }
}

fn cerceve_serit(t: &mut [u8], w: u32, h: u32, z: f32, r: Renk) {
    let k = (h as f32 / 110.0).max(6.0);
    let a = nefes(z, 1.0, 0.8);
    for (y, kal) in [(h as f32 * 0.035, k), (h as f32 * 0.035 + k * 1.8, k * 0.4), (h as f32 * 0.965 - k, k), (h as f32 * 0.965 - k * 2.8, k * 0.4)] { dik(t, w, h, w as f32 * 0.05, y, w as f32 * 0.95, y + kal, r, a); }
}

// ---- Hareketli zeminler: yazının arkasında, düşük opaklıkta (kontrast korunur) ----

fn zemin_gradyan(t: &mut [u8], w: u32, h: u32, z: f32, r: Renk) {
    for y in 0..h {
        let a = 0.05 + 0.13 * (0.5 + 0.5 * (z * 0.45 + y as f32 / h as f32 * 3.1416).sin());
        for x in 0..w { karistir(t, ((y * w + x) * 3) as usize, r, a); }
    }
}

fn zemin_dalga(t: &mut [u8], w: u32, h: u32, z: f32, r: Renk) {
    let ks = 6.2832 / (h as f32 / 5.0);
    for y in 0..h {
        for x in (0..w).step_by(2) {
            let a = 0.14 * (0.5 + 0.5 * (y as f32 * ks + (x as f32 / w as f32 * 3.0) + z * 0.9).sin());
            let i = ((y * w + x) * 3) as usize; karistir(t, i, r, a); karistir(t, i + 3, r, a);
        }
    }
}

fn zemin_izgara(t: &mut [u8], w: u32, h: u32, z: f32, r: Renk) {
    let adim = (w / 14).max(12);
    let boy = (adim as f32 * 0.10).max(2.0);
    let (cx, cy) = (w as f32 / 2.0, h as f32 / 2.0);
    let mut y = adim / 2;
    while y < h {
        let mut x = adim / 2;
        while x < w {
            let d = (((x as f32 - cx).powi(2) + (y as f32 - cy).powi(2)).sqrt()) / w as f32;
            let a = 0.10 + 0.22 * (0.5 + 0.5 * (d * 9.0 - z * 1.2).sin());
            elips(t, w, h, x as f32, y as f32, boy, boy, 0.0, r, a);
            x += adim;
        }
        y += adim;
    }
}

fn zemin_plazma(t: &mut [u8], w: u32, h: u32, z: f32, r: Renk) {
    let blok = 8u32;
    let (fw, fh) = (w as f32, h as f32);
    let mut y = 0;
    while y < h {
        let mut x = 0;
        while x < w {
            let (u, v) = (x as f32 / fw * 4.0, y as f32 / fh * 7.0);
            let n = ((u * 2.1 + z * 0.6).sin() + (v * 1.3 - z * 0.5).sin() + ((u + v) * 1.1 + z * 0.4).sin() + ((u * u + v * v).sqrt() * 1.7 - z * 0.7).sin()) / 4.0;
            let a = 0.03 + 0.14 * (0.5 + 0.5 * n);
            for yy in y..(y + blok).min(h) { for xx in x..(x + blok).min(w) { karistir(t, ((yy * w + xx) * 3) as usize, r, a); } }
            x += blok;
        }
        y += blok;
    }
}


fn hsh(a: u32) -> f32 { let mut x = a.wrapping_mul(2654435761) ^ 0x9e3779b9; x ^= x >> 15; x = x.wrapping_mul(2246822519); x ^= x >> 13; (x & 0xffff) as f32 / 65535.0 }
fn zemin_serit(t: &mut [u8], w: u32, h: u32, z: f32, r: Renk) {
    let n = 9; let ad = (w + h) as f32 / n as f32;
    for y in 0..h { for x in 0..w {
        let u = ((x + y) as f32 - z * 40.0) / ad;
        let f = u - u.floor();
        if f < 0.22 {
            let a = 0.10 * (1.0 - (f - 0.11).abs() / 0.11);
            let i = ((y * w + x) * 3) as usize;
            let c = [r.0, r.1, r.2];
            for k in 0..3 { t[i + k] = (t[i + k] as f32 * (1.0 - a) + c[k] as f32 * a) as u8; }
        }
    } }
}
fn zemin_yildizli(t: &mut [u8], w: u32, h: u32, z: f32, r: Renk) {
    for k in 0..60u32 {
        let (x, y) = (hsh(k * 3 + 1) * w as f32, hsh(k * 3 + 2) * h as f32);
        let faz = hsh(k * 3 + 3) * 6.28;
        let a = 0.10 + 0.35 * (0.5 + 0.5 * (z * 1.3 + faz).sin());
        let b = (w as f32 / 220.0).max(2.0) * (0.7 + hsh(k + 99));
        elips(t, w, h, x, y, b, b, 0.0, r, a);
    }
}
fn zemin_nabiz(t: &mut [u8], w: u32, h: u32, z: f32, r: Renk) {
    let (cx, cy) = (w as f32 / 2.0, h as f32 / 2.0);
    let en = (cx * cx + cy * cy).sqrt();
    for y in 0..h { for x in 0..w {
        let d = (((x as f32 - cx).powi(2) + (y as f32 - cy).powi(2)).sqrt()) / en;
        let f = ((d * 3.0 - z * 0.25) % 1.0 + 1.0) % 1.0;
        let a = 0.14 * (1.0 - ((f - 0.5).abs() * 2.0)).powi(3);
        if a < 0.004 { continue; }
        let i = ((y * w + x) * 3) as usize;
        let c = [r.0, r.1, r.2];
        for k in 0..3 { t[i + k] = (t[i + k] as f32 * (1.0 - a) + c[k] as f32 * a) as u8; }
    } }
}
fn zemin_sutun(t: &mut [u8], w: u32, h: u32, z: f32, r: Renk) {
    let n = 14u32; let bw = w / n;
    for k in 0..n {
        let boy = (0.10 + 0.14 * (0.5 + 0.5 * (z * (0.9 + hsh(k) * 0.8) + hsh(k + 50) * 6.28).sin())) * h as f32;
        for y in (h as f32 - boy) as u32..h { for x in k * bw + bw / 6..(k + 1) * bw - bw / 6 {
            let i = ((y * w + x) * 3) as usize;
            let c = [r.0, r.1, r.2];
            for q in 0..3 { t[i + q] = (t[i + q] as f32 * 0.88 + c[q] as f32 * 0.12) as u8; }
        } }
    }
}

pub const ADLAR: &[(&str, &str)] = &[
    ("cicek-yagmuru", "Yağan çiçekler (palet vurgu rengi)"), ("papatya", "Süzülen papatyalar"), ("yaprak", "Dönerek düşen sonbahar yaprakları"),
    ("kelebek", "Kanat çırpan kelebekler"), ("ugur-bocegi", "Yürüyen uğur böcekleri"), ("ari", "Vızıldayan arılar"),
    ("ates-bocegi", "Yavaş yanıp sönen ateş böcekleri"), ("kalp", "Yükselen kalpler"), ("yildiz-yagmuru", "Dönerek düşen yıldızlar"),
    ("konfeti", "Konfeti yağmuru"), ("balon", "Yükselen balonlar"), ("havai-fisek", "Havai fişek (tam ekran flaş yok)"), ("pirilti-bulutu", "Yavaş parlayan ışıltılar"),
    ("cerceve-ince", "Çerçeve: ince çizgi (yavaş nefes)"), ("zemin-serit", "Zemin: kayan çapraz şeritler"), ("zemin-yildizli", "Zemin: yanıp sönen yıldızlar"), ("zemin-nabiz", "Zemin: merkezden yayılan nabız"), ("zemin-sutun", "Zemin: dalgalanan alt sütunlar"), ("cerceve-cift", "Çerçeve: kalın ve ince çift çizgi"), ("cerceve-kose", "Çerçeve: köşe işaretleri"),
    ("cerceve-nokta", "Çerçeve: yürüyen LED noktaları"), ("cerceve-neon", "Çerçeve: neon ışıltılı çizgi"), ("cerceve-yildiz", "Çerçeve: köşe yıldızları ve ışıltı"),
    ("cerceve-dalga", "Çerçeve: üstte ve altta dalga"), ("cerceve-isik", "Çerçeve: çevrede dönen ışık kuyruğu"), ("cerceve-altin", "Çerçeve: kalın altın kenar, köşe kareleri"), ("cerceve-serit", "Çerçeve: üst ve alt şerit bant"),
    ("zemin-gradyan", "Zemin: yumuşak nefes alan renk geçişi"), ("zemin-dalga", "Zemin: akan yatay ışık dalgaları"), ("zemin-izgara", "Zemin: LED noktası ızgarası, merkezden dalga"), ("zemin-plazma", "Zemin: yavaş plazma bulutları"),
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
        "cerceve-ince" => cerceve_ince(t, w, h, z, renk),
        "cerceve-cift" => cerceve_cift(t, w, h, z, renk),
        "cerceve-kose" => cerceve_kose(t, w, h, z, renk),
        "cerceve-nokta" => cerceve_nokta(t, w, h, z, renk),
        "cerceve-neon" => cerceve_neon(t, w, h, z, renk),
        "cerceve-yildiz" => cerceve_yildiz(t, w, h, z, renk),
        "cerceve-dalga" => cerceve_dalga(t, w, h, z, renk),
        "cerceve-isik" => cerceve_isik(t, w, h, z, renk),
        "cerceve-altin" => cerceve_altin(t, w, h, z, renk),
        "cerceve-serit" => cerceve_serit(t, w, h, z, renk),
        "zemin-serit" => zemin_serit(t, w, h, z, renk),
        "zemin-yildizli" => zemin_yildizli(t, w, h, z, renk),
        "zemin-nabiz" => zemin_nabiz(t, w, h, z, renk),
        "zemin-sutun" => zemin_sutun(t, w, h, z, renk),
        "zemin-gradyan" => zemin_gradyan(t, w, h, z, renk),
        "zemin-dalga" => zemin_dalga(t, w, h, z, renk),
        "zemin-izgara" => zemin_izgara(t, w, h, z, renk),
        "zemin-plazma" => zemin_plazma(t, w, h, z, renk),
        _ => return false,
    }
    true
}
