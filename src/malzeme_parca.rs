//! Ortak parça / havai fişek motoru (KARAR-ANIM 3. bölüm). Beş tür: küre, söğüt, halka, kalp, çift patlama.
//! Kapalı biçimli: her kıvılcımın konumu yalnız zamanın fonksiyonu (p = c + v0·u·(1−e^(−kτ))/k + ½gτ²); kareler arası durum yok,
//! rastgelelik `efekt::hash` ile deterministik. Tam ekran flaş yok; göz kırpma 2,5 Hz; toplamalı karışım + 255'e kırpma, hale yok.
//! Sahip: Yusuf. Yazan: [Claude#Malzeme Opus 5.5, yenileme-1.0, 2026-11-25]. Yalnız yazan yapay zekâ ya da Yusuf'un izniyle değişir; silme yok.
use crate::efekt::hash;
use crate::malzeme_ciz::Tuval;
use crate::Renk;

const TAU: f32 = std::f32::consts::TAU;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Tur { Kure, Sogut, Halka, Kalp, Cift }

/// (adet, v0 px/sn @950, k, g px/sn², ömür L sn, iz kopya N, yarıçap px)
/// Dalga 4 [Claude#MAL-FIX Opus 5.5, 2026-11-25]: 950x1800 kontak denetiminde patlamalar ekranın ~%30'unda kalıyordu
/// ("hafif kaçan"); kıvılcım sayısı ~1,6×, hız ~1,3×, yarıçap +1,5 px, söğüt ömrü uzatıldı.
fn tablo(t: Tur) -> (usize, f32, f32, f32, f32, usize, f32) {
    match t {
        Tur::Kure => (170, 720.0, 2.2, 170.0, 1.9, 4, 5.5),
        Tur::Sogut => (96, 420.0, 1.1, 260.0, 3.2, 12, 4.8),
        Tur::Halka => (78, 620.0, 2.4, 120.0, 1.7, 3, 6.0),
        Tur::Kalp => (84, 40.0, 2.6, 60.0, 2.0, 3, 6.0),
        Tur::Cift => (64, 620.0, 2.2, 150.0, 1.3, 3, 5.5),
    }
}

/// Patlamanın merkezden en uzak noktası (px, olc=1): v0/k (kalpte v0·17). Çift için ikinci patlama dahil.
/// Dalga 5 [Claude#vidyo-yenileme-bd2f6a, 2026-11-26]: sol kenarda kırpılma — merkez ve yarıçap kadraja sığdırılır.
pub fn erisim(t: Tur) -> f32 {
    let (_, v0, k, ..) = tablo(t);
    let v = if t == Tur::Kalp { v0 * 17.0 } else { v0 };
    (v / k).max(if t == Tur::Cift { 290.0 } else { 0.0 })
}

pub fn tur_ad(t: Tur) -> &'static str { match t { Tur::Kure => "kure", Tur::Sogut => "sogut", Tur::Halka => "halka", Tur::Kalp => "kalp", Tur::Cift => "cift" } }

/// Toplamalı nokta/disk (255'e kırpma). r < 1,5 ise tek piksel.
pub fn ekle(tv: &mut Tuval, x: f32, y: f32, r: f32, c: Renk, a: f32) {
    if a <= 0.004 { return; }
    let a = a.min(1.0);
    let mut koy = |px: i32, py: i32, k: f32| {
        if px < 0 || py < 0 || px >= tv.w as i32 || py >= tv.h as i32 { return; }
        let i = ((py as u32 * tv.w + px as u32) * 3) as usize;
        tv.t[i] = (tv.t[i] as f32 + c.0 as f32 * k).min(255.0) as u8;
        tv.t[i + 1] = (tv.t[i + 1] as f32 + c.1 as f32 * k).min(255.0) as u8;
        tv.t[i + 2] = (tv.t[i + 2] as f32 + c.2 as f32 * k).min(255.0) as u8;
    };
    if r < 1.5 { koy(x as i32, y as i32, a); return; }
    let (x0, x1, y0, y1) = ((x - r - 1.0) as i32, (x + r + 1.0) as i32, (y - r - 1.0) as i32, (y + r + 1.0) as i32);
    for py in y0..=y1 { for px in x0..=x1 {
        let d = ((px as f32 + 0.5 - x).powi(2) + (py as f32 + 0.5 - y).powi(2)).sqrt();
        let k = (r - d + 0.5).clamp(0.0, 1.0);
        if k > 0.0 { koy(px, py, k * a); }
    } }
}

fn kar(a: Renk, b: Renk, k: f32) -> Renk { let k = k.clamp(0.0, 1.0); Renk((a.0 as f32 + (b.0 as f32 - a.0 as f32) * k) as u8, (a.1 as f32 + (b.1 as f32 - a.1 as f32) * k) as u8, (a.2 as f32 + (b.2 as f32 - a.2 as f32) * k) as u8) }
fn kos(r: Renk, k: f32) -> Renk { Renk((r.0 as f32 * k).min(255.0) as u8, (r.1 as f32 * k).min(255.0) as u8, (r.2 as f32 * k).min(255.0) as u8) }

/// Birim hız vektörü (|u| ≤ 1, kalp eğrisinde ~1).
fn birim(t: Tur, i: usize, n: usize, tohum: u32) -> (f32, f32) {
    let h = |o: u32| hash(tohum.wrapping_mul(7919) ^ (i as u32).wrapping_mul(104729) ^ o.wrapping_mul(31));
    match t {
        Tur::Kure | Tur::Sogut | Tur::Cift => {
            // küre yüzeyinden eşit dağılım, ekrana izdüşüm
            let zz = h(1) * 2.0 - 1.0;
            let f = h(2) * TAU;
            let s = (1.0 - zz * zz).sqrt();
            (s * f.cos(), s * f.sin())
        }
        Tur::Halka => {
            let egim = 0.35 + 0.6 * hash(tohum ^ 0xA5A5);
            let ic = i % 3 == 2;
            let a = i as f32 / n as f32 * TAU + hash(tohum) * 0.4;
            let o = if ic { 0.5 } else { 1.0 };
            (a.cos() * o, a.sin() * o * egim)
        }
        Tur::Kalp => {
            let a = i as f32 / n as f32 * TAU;
            let x = 16.0 * a.sin().powi(3);
            let y = -(13.0 * a.cos() - 5.0 * (2.0 * a).cos() - 2.0 * (3.0 * a).cos() - (4.0 * a).cos());
            (x / 17.0, y / 17.0)
        }
    }
}

/// Bir patlamayı τ (patlamadan beri sn) anında çizer. `olc` ekran ölçeği (genişlik/950).
pub fn patlama(tv: &mut Tuval, t: Tur, cx: f32, cy: f32, tau: f32, renk: Renk, ikinci: Renk, tohum: u32, olc: f32) {
    let (n, v0, k, g, l, iz, r0) = tablo(t);
    if tau < 0.0 { return; }
    let v0 = if t == Tur::Kalp { v0 * 17.0 } else { v0 } * olc;
    let g = g * olc;
    // merkezde kısa açık ton (tam ekran flaş YOK)
    if tau < 0.12 { ekle(tv, cx, cy, 22.0 * olc, kar(renk, Renk(255, 255, 255), 0.6), 0.55 * (1.0 - tau / 0.12)); }
    let konum = |u: (f32, f32), tt: f32| -> (f32, f32) {
        let s = v0 * (1.0 - (-k * tt).exp()) / k;
        (cx + u.0 * s, cy + u.1 * s + 0.5 * g * tt * tt)
    };
    if tau <= l {
        for i in 0..n {
            let u = birim(t, i, n, tohum);
            let mut a = 1.0 - (tau / l).powi(2);
            if tau > l * 0.7 && i % 2 == 0 { a *= 0.55 + 0.45 * (TAU * 2.5 * tau + hash(tohum + i as u32) * TAU).sin(); }
            let gec = tau / l;
            let ana = if gec > 0.75 { kos(renk, 0.62) } else { renk };
            for m in (0..=iz).rev() {
                let tt = tau - m as f32 * 0.035;
                if tt < 0.0 { continue; }
                let (x, y) = konum(u, tt);
                let f = 1.0 - m as f32 / (iz as f32 + 1.0);
                let rr = r0 * olc * (0.45 + 0.55 * f);
                let rk = if m == 0 { kar(ana, Renk(255, 255, 255), 0.45) } else { ana };
                ekle(tv, x, y, rr, rk, a * f * if m == 0 { 1.0 } else { 0.7 });
            }
        }
    }
    // çift: 0,55 sn sonra her 5. kıvılcımın konumunda küçük ikinci patlama
    if t == Tur::Cift {
        let t2 = tau - 0.55;
        if t2 >= 0.0 && t2 < 0.9 {
            for i in (0..n).step_by(5) {
                let (px, py) = konum(birim(t, i, n, tohum), 0.55);
                for j in 0..12 {
                    let u = birim(Tur::Kure, j, 12, tohum ^ (i as u32 * 977));
                    let s = 230.0 * olc * (1.0 - (-2.6 * t2).exp()) / 2.6;
                    let a = 1.0 - (t2 / 0.9).powi(2);
                    for m in 0..3 {
                        let tt = (t2 - m as f32 * 0.03).max(0.0);
                        let s2 = s * (1.0 - (-2.6 * tt).exp()) / (1.0 - (-2.6 * t2).exp()).max(1e-3);
                        ekle(tv, px + u.0 * s2, py + u.1 * s2 + 0.5 * 120.0 * olc * tt * tt, 4.5 * olc * (1.0 - m as f32 * 0.25), ikinci, a * (1.0 - m as f32 * 0.3));
                    }
                }
            }
        }
    }
}

/// Toplam patlama ömrü (çift için ikinci patlama dahil).
pub fn omur(t: Tur) -> f32 { tablo(t).4 + if t == Tur::Cift { 0.9 } else { 0.0 } }

/// Gösteri: `adet` roket döngüsel takvimle (periyot `donem`) fırlar; `tur_sec(j)` her roketin türü.
/// Alan: tepe noktaları x %15–85, y `ust..alt` (ekran oranı). `z` = yerel·hiz.
pub fn gosteri(tv: &mut Tuval, z: f32, adet: usize, tur_sec: &dyn Fn(usize) -> Tur, renkler: &[Renk], tohum: u32, ust: f32, alt: f32) {
    let (wf, hf) = (tv.w as f32, tv.h as f32);
    let olc = (wf / 950.0).min(hf / 1800.0);
    let adet = adet.max(1);
    // aynı anda en çok ~3 patlama: aralık ≥ L/3
    let aralik = (2.6 / adet as f32).max(0.6);
    let donem = adet as f32 * aralik + 2.0;
    for tekrar in [-1.0f32, 0.0] {
        let zz = z.rem_euclid(donem) - tekrar * donem;
        let tur_no = ((z / donem).floor() - tekrar) as i64;
        for j in 0..adet {
            let ts = tohum.wrapping_add(j as u32 * 131).wrapping_add((tur_no.rem_euclid(97) as u32) * 7);
            let t = tur_sec(j);
            let tj = j as f32 * aralik + hash(ts) * aralik * 0.5 + 0.1;
            let yuk = 0.9 + 0.4 * hash(ts + 1);
            // dalga 4 [Claude#MAL-FIX]: saf hash ardışık roketleri aynı yarıya topluyordu (kontakta hepsi solda);
            // altın oran tabakalaması ardışık roketleri ekrana yayar, küçük hash sapması düzenliliği kırar
            let baz = hash(tohum ^ (tur_no.rem_euclid(97) as u32).wrapping_mul(7919));
            let xp = (baz + j as f32 * 0.618_034 + 0.2 * (hash(ts + 2) - 0.5)).rem_euclid(1.0);
            // kadraj güvenliği: yarıçap ekranın en kısa kenarının %30'unu aşmaz; merkez, yarıçap + kenar payı içinde kalır
            let kenar = 0.04 * wf.min(hf);
            let r1 = erisim(t) * olc;
            let esk = (0.30 * wf.min(hf) / r1).min(1.0);
            let olc_p = olc * esk;
            let r = r1 * esk;
            let (xl, xh) = (r + kenar, (wf - r - kenar).max(r + kenar));
            let (yl, yh) = (r + kenar, (hf - r - kenar).max(r + kenar));
            let tx = xl + (xh - xl) * xp;
            let ty = (hf * (ust + (alt - ust) * hash(ts + 3))).clamp(yl, yh);
            let rk = renkler[(hash(ts + 4) * renkler.len() as f32) as usize % renkler.len()];
            let rk2 = renkler[(hash(ts + 5) * renkler.len() as f32) as usize % renkler.len()];
            let lt = zz - tj;
            if lt < 0.0 || lt > yuk + omur(t) { continue; }
            if lt < yuk {
                // yükseliş izi: kuyruk + sönük noktalar
                let e = 1.0 - (1.0 - lt / yuk).powi(2);
                let x = tx + (hash(ts + 6) - 0.5) * 60.0 * olc * (1.0 - e);
                let y = hf + (ty - hf) * e;
                tv.cizgi(x, y, x, y + 16.0 * olc, 5.0 * olc, Renk(255, 225, 170), 0.9);
                for q in 1..7 { ekle(tv, x, y + q as f32 * 14.0 * olc, 2.6 * olc, Renk(255, 190, 120), 0.5 * (1.0 - q as f32 / 7.0)); }
            } else {
                patlama(tv, t, tx, ty, lt - yuk, rk, rk2, ts, olc_p);
            }
        }
    }
}

/// Temsil anı: ilk roketin patlamasından ~0,4 sn sonra (küçük resim için).
pub fn ilk_patlama_ani() -> f32 { 0.1 + 1.3 + 0.4 }

#[cfg(test)]
mod t {
    use super::*;
    fn tuval_say(f: &dyn Fn(&mut Tuval)) -> (usize, Vec<u8>) {
        let (w, h) = (380u32, 720u32);
        let mut b = vec![0u8; (w * h * 3) as usize];
        { let mut tv = Tuval { t: &mut b, w, h }; f(&mut tv); }
        (b.chunks(3).filter(|p| p[0] > 30 || p[1] > 30 || p[2] > 30).count(), b)
    }
    #[test]
    fn her_tur_patlar_ve_deterministik() {
        for t in [Tur::Kure, Tur::Sogut, Tur::Halka, Tur::Kalp, Tur::Cift] {
            let (n, a) = tuval_say(&|tv| patlama(tv, t, 190.0, 300.0, 0.4, Renk(255, 200, 40), Renk(0, 200, 255), 3, 0.4));
            assert!(n > 150, "{}: {n}", tur_ad(t));
            let (_, b) = tuval_say(&|tv| patlama(tv, t, 190.0, 300.0, 0.4, Renk(255, 200, 40), Renk(0, 200, 255), 3, 0.4));
            assert!(a == b, "{} deterministik değil", tur_ad(t));
        }
    }
    /// Dalga 5: hiçbir patlama sol/sağ/üst kenara değmez (tüm çözünürlüklerde); söğüt yalnız dikeyde sarkabilir.
    #[test]
    fn patlamalar_kadraj_icinde() {
        let renkler = [Renk(255, 205, 60), Renk(80, 200, 255), Renk(255, 80, 120)];
        for (w, h) in [(380u32, 720u32), (950, 1800), (720, 380), (1900, 950), (600, 1920), (1920, 1080), (1080, 1920)] {
            for t in [Tur::Kure, Tur::Sogut, Tur::Halka, Tur::Kalp, Tur::Cift] {
                let mut kenar_isik = 0usize;
                let mut toplam_isik = 0usize;
                let mut z = 0.0f32;
                while z < 9.0 {
                    let mut b = vec![0u8; (w * h * 3) as usize];
                    { let mut tv = Tuval { t: &mut b, w, h }; gosteri(&mut tv, z, 6, &|_| t, &renkler, 41, 0.2, 0.46); }
                    toplam_isik += b.chunks(3).filter(|p| p[0] > 30 || p[1] > 30 || p[2] > 30).count();
                    for y in 0..h as usize {
                        for x in [0usize, w as usize - 1] { let i = (y * w as usize + x) * 3; if b[i] > 30 || b[i + 1] > 30 || b[i + 2] > 30 { kenar_isik += 1; } }
                    }
                    for x in 0..w as usize { let i = x * 3; if b[i] > 30 || b[i + 1] > 30 || b[i + 2] > 30 { kenar_isik += 1; } }
                    z += 0.25;
                }
                assert!(toplam_isik > 2000, "{}x{} {}: patlama çizilmiyor ({toplam_isik})", w, h, tur_ad(t));
                assert_eq!(kenar_isik, 0, "{}x{} {}: {kenar_isik} kenar pikseli yanıyor (kırpılma)", w, h, tur_ad(t));
            }
        }
    }
    #[test]
    fn kalp_yatay_simetrik() {
        let (_, b) = tuval_say(&|tv| patlama(tv, Tur::Kalp, 190.0, 300.0, 0.5, Renk(255, 80, 120), Renk(255, 80, 120), 9, 0.4));
        let (w, h) = (380usize, 720usize);
        let (mut sol, mut sag) = (0usize, 0usize);
        for y in 0..h { for x in 0..w { if b[(y * w + x) * 3] > 30 { if x < 190 { sol += 1 } else { sag += 1 } } } }
        let fark = (sol as f32 - sag as f32).abs() / (sol + sag).max(1) as f32;
        assert!(fark < 0.08, "kalp simetri farkı {fark}");
    }
}
