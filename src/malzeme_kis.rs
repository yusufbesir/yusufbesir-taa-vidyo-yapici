//! Kış / yılbaşı paketi (KARAR-ANIM 2. bölüm): katmanlı kar, biriken kar, çam ağacı, havai fişek türleri, yılbaşı şenliği,
//! yılbaşı geri sayımı, kardan adam, buz kristali, kış manzarası, hediye yığını; efektler kar-katmanli, kar-ortusu, isik-dizisi.
//! "Premium ağır" görünüm: ışıma/neon yok; 3 tonlu gölgeleme (yüz / ×0,62 gölge / beyaza %35 ışık, ışık sol üstten), sert gölge,
//! katmanlı derinlik. Hepsi kapalı biçimli ve `efekt::hash` ile deterministik. LED güvenliği: tam ekran flaş yok, yanıp sönme ≤ 1 Hz.
//! Sahip: Yusuf. Yazan: [Claude#Malzeme Opus 5.5, yenileme-1.0, 2026-11-25]. Yalnız yazan yapay zekâ ya da Yusuf'un izniyle değişir; silme yok.
use crate::anim::{self, Poz};
use crate::efekt::hash;
use crate::malzeme::MalzemeAyar;
use crate::malzeme_ciz::{piksel_gen, Tuval};
use crate::malzeme_parca::{self as parca, Tur};
use crate::senaryo::Hazir;
use crate::Renk;

const PI: f32 = std::f32::consts::PI;
const TAU: f32 = std::f32::consts::TAU;
const KOYU: Renk = Renk(14, 16, 24);
const KAR: Renk = Renk(236, 242, 252);
const KAR_GOLGE: Renk = Renk(150, 170, 205);
const YESIL: Renk = Renk(40, 150, 80);
const ALTIN: Renk = Renk(255, 200, 60);
const SUS: [Renk; 6] = [Renk(230, 40, 50), Renk(255, 200, 40), Renk(60, 160, 255), Renk(240, 240, 250), Renk(200, 80, 220), Renk(255, 120, 40)];
const FISEK: [Renk; 6] = [Renk(255, 205, 60), Renk(80, 200, 255), Renk(255, 80, 120), Renk(130, 255, 110), Renk(255, 140, 40), Renk(200, 140, 255)];

pub fn kos(r: Renk, k: f32) -> Renk { Renk((r.0 as f32 * k).min(255.0) as u8, (r.1 as f32 * k).min(255.0) as u8, (r.2 as f32 * k).min(255.0) as u8) }
pub fn kar(a: Renk, b: Renk, k: f32) -> Renk { let k = k.clamp(0.0, 1.0); Renk((a.0 as f32 + (b.0 as f32 - a.0 as f32) * k) as u8, (a.1 as f32 + (b.1 as f32 - a.1 as f32) * k) as u8, (a.2 as f32 + (b.2 as f32 - a.2 as f32) * k) as u8) }
/// 3 ton kuralı: (yüz, gölge ×0,62, ışık beyaza %35).
pub fn uc_ton(r: Renk) -> (Renk, Renk, Renk) { (r, kos(r, 0.62), kar(r, Renk(255, 255, 255), 0.35)) }
fn fr(x: f32) -> f32 { x - x.floor() }
fn yumusak(t: f32) -> f32 { anim::yumusak(t) }

/// 1B değer gürültüsü (0..1), yumuşak aradeğer.
pub fn gurultu(x: f32, tohum: u32) -> f32 {
    let i = x.floor();
    let f = x - i;
    let a = hash(tohum.wrapping_mul(977) ^ (i as i32 as u32).wrapping_mul(7919));
    let b = hash(tohum.wrapping_mul(977) ^ ((i as i32 + 1) as u32).wrapping_mul(7919));
    a + (b - a) * f * f * (3.0 - 2.0 * f)
}

/// Koyu dış çizgili, koyu ekstrüzyonlu (3B görünümlü) piksel yazı; dönüş kullanılan hücre.
pub fn piksel_3b(tv: &mut Tuval, s: &str, cx: f32, cy: f32, gen: f32, en_cok: f32, renk: Renk) -> f32 {
    if s.trim().is_empty() { return 0.0; }
    let n = s.chars().count().max(1) as f32;
    let h = (gen / (n * 6.0 - 1.0)).min(en_cok).max(1.0);
    let derin = (h * 0.5).max(2.0);
    for k in (1..=4).rev() { let o = derin * k as f32 / 4.0; tv.piksel_yazi(s, cx + o, cy + o, h, kos(renk, 0.35), 1.0); }
    let c = (h * 0.3).max(2.0);
    for (ox, oy) in [(-c, 0.0), (c, 0.0), (0.0, -c), (0.0, c)] { tv.piksel_yazi(s, cx + ox, cy + oy, h, KOYU, 1.0); }
    tv.piksel_yazi(s, cx, cy, h, renk, 1.0);
    h
}

/// Maske (TTF) varsa 3B yazı, yoksa piksel 3B yazı.
fn yazi_3b(tv: &mut Tuval, z: &Hazir, yazi: &str, cx: f32, cy: f32, gen: f32, olcek: f32, aci: f32, u: f32) {
    if let Some(m) = z.maskeler.first() {
        let my = (m.bx.1 + m.bx.3) as f32 / 2.0;
        let mx = (m.bx.0 + m.bx.2) as f32 / 2.0;
        anim::kare3d_uzerine(tv.t, m, Renk(255, 250, 238), kos(z.vurgu, 0.55), &Poz { aci, olcek: olcek.max(0.02), parlak: 1.0, derinlik: 16.0 * u, dx: cx - mx, dy: cy - my });
    } else if !yazi.is_empty() {
        piksel_3b(tv, yazi, cx, cy, gen * olcek, 30.0 * u * olcek, Renk(255, 250, 238));
    }
}

// ---------------------------------------------------------------- kar
/// Katmanlı kar (K1): 3 derinlik; `n` toplam tane; `ruzgar` ±1; `katman_maske` hangi katmanlar (bit 0 arka, 1 orta, 2 ön).
pub fn kar_katmanli(tv: &mut Tuval, z: f32, sure: f32, n: usize, ruzgar: f32, katman_maske: u8, tohum: u32) {
    let (wf, hf) = (tv.w as f32, tv.h as f32);
    let u = (wf / 950.0).min(hf / 1800.0);
    let sure = sure.max(1.0);
    let r_esinti = ruzgar * (40.0 + 30.0 * (TAU * z / sure).sin() + 15.0 * (TAU * 3.0 * z / sure).sin()) * u;
    for (kat, oran, vnom, r0, r1, alfa) in [(0u8, 0.5f32, 60.0f32, 1.5f32, 3.0f32, 0.45f32), (1, 0.35, 130.0, 4.0, 7.0, 0.75), (2, 0.15, 240.0, 10.0, 18.0, 0.95)] {
        if katman_maske & (1 << kat) == 0 { continue; }
        let adet = ((n as f32 * oran).round() as usize).max(1);
        let pay = 60.0 * u;
        let yol = hf + pay * 2.0;
        // döngü güvenli: v·sure yolun tam katı
        let tur = ((vnom * u * sure / yol).round()).max(1.0);
        let v = tur * yol / sure;
        let derin = [0.35f32, 0.7, 1.0][kat as usize];
        for i in 0..adet {
            let s = |o: u32| hash(tohum.wrapping_mul(131) ^ ((kat as u32 * 100_003 + i as u32) * 13 + o));
            let y = (s(1) * yol + v * z).rem_euclid(yol) - pay;
            let x = (s(2) * (wf + 200.0 * u) - 100.0 * u + (TAU * z / sure * (1.0 + (s(3) * 3.0).floor()) + s(4) * TAU).sin() * (12.0 + 30.0 * s(5)) * u * derin + r_esinti * derin).rem_euclid(wf + 200.0 * u) - 100.0 * u;
            let r = (r0 + (r1 - r0) * s(6)) * u;
            if kat < 2 {
                if r < 1.5 { tv.px(x as i32, y as i32, KAR, alfa); } else { tv.daire(x, y, r, KAR, alfa); }
            } else {
                // ön katman: altı kollu tane, yavaş döner; koyu gölge kopyası sağ altta (sert gölge)
                // dönüş döngü güvenli: altı kollu tanenin dönemi π/3, süre içinde tam sayı dönem
                let don = (PI / 3.0) * ((sure * (0.3 + 0.4 * s(7)) / (PI / 3.0)).round().max(1.0)) / sure;
                let a = z * don * if s(8) > 0.5 { 1.0 } else { -1.0 } + s(9) * TAU;
                for (dx, rk, al) in [(r * 0.18, KAR_GOLGE, 0.6f32), (0.0, KAR, alfa)] {
                    for k in 0..3 {
                        let b = a + k as f32 * PI / 3.0;
                        let (ex, ey) = (b.cos() * r, b.sin() * r);
                        tv.cizgi(x - ex + dx, y - ey + dx, x + ex + dx, y + ey + dx, (r * 0.16).max(2.0), rk, al);
                        for sg in [-1.0f32, 1.0] {
                            let (mx, my) = (x + sg * ex * 0.55 + dx, y + sg * ey * 0.55 + dx);
                            for c in [-0.7f32, 0.7] { let cb = b + c + if sg < 0.0 { PI } else { 0.0 }; tv.cizgi(mx, my, mx + cb.cos() * r * 0.35, my + cb.sin() * r * 0.35, (r * 0.12).max(1.5), rk, al); }
                        }
                    }
                }
            }
        }
    }
}

/// Alt kenarda kar örtüsü; `yuk` 0..1 en çok yüksekliğin oranı (ekran yüksekliği × `en_cok`).
pub fn kar_ortusu(tv: &mut Tuval, yuk: f32, en_cok: f32, tohum: u32) {
    let (wf, hf) = (tv.w as f32, tv.h as f32);
    let u = (wf / 950.0).min(hf / 1800.0);
    let hmax = hf * en_cok * yuk.clamp(0.0, 1.0);
    if hmax < 1.0 { return; }
    let (yuz, golge, isik) = (Renk(222, 230, 245), Renk(170, 186, 214), Renk(250, 252, 255));
    let mut x = 0.0f32;
    while x < wf {
        let g = 0.65 * gurultu(x / (140.0 * u), tohum) + 0.35 * gurultu(x / (47.0 * u), tohum + 5);
        let hh = hmax * (0.7 + 0.3 * g);
        let ust = hf - hh;
        tv.dikd(x, ust, x + 2.0, hf, yuz, 1.0);
        // alt yarı gölge tonu, sınır 2x2 dama ile bantlı
        let sinir = ust + hh * 0.55;
        tv.dikd(x, sinir + 4.0 * u, x + 2.0, hf, golge, 1.0);
        if (x as i32 / 2) % 2 == 0 { tv.dikd(x, sinir, x + 2.0, sinir + 4.0 * u, golge, 1.0); }
        tv.dikd(x, ust, x + 2.0, ust + 6.0 * u, isik, 1.0);
        x += 2.0;
    }
}

/// Üst kenarda sarkan, koşan ampullü ışık dizisi (≤ 1 Hz). Ampuller toplamalı, hale yok.
pub fn isik_dizisi(tv: &mut Tuval, z: f32, y0: f32, sarkma: f32, tohum: u32) {
    let (wf, hf) = (tv.w as f32, tv.h as f32);
    let u = (wf / 950.0).min(hf / 1800.0);
    let kemer = 3;
    let kw = wf / kemer as f32;
    let tel = Renk(40, 50, 40);
    let mut no = 0usize;
    for k in 0..kemer {
        let x0 = k as f32 * kw;
        let yy = |x: f32| { let p = (x - x0) / kw; y0 + sarkma * 4.0 * p * (1.0 - p) };
        let mut x = x0;
        while x < x0 + kw { let x2 = x + 10.0 * u; tv.cizgi(x, yy(x), x2, yy(x2), 4.0 * u, tel, 1.0); x = x2; }
        let adet = 9;
        for i in 0..adet {
            let bx = x0 + kw * (i as f32 + 0.5) / adet as f32;
            let by = yy(bx) + 16.0 * u;
            let rk = SUS[(no + tohum as usize) % 4];
            let p = fr(z * 0.8 - no as f32 / 6.0);
            let ucgen = 1.0 - (p * 2.0 - 1.0).abs();
            let par = 0.4 + 0.6 * yumusak(ucgen);
            tv.dikd(bx - 4.0 * u, by - 18.0 * u, bx + 4.0 * u, by - 8.0 * u, Renk(90, 90, 80), 1.0);
            tv.elips(bx, by, 9.0 * u, 13.0 * u, 0.0, kos(rk, 0.35), 1.0);
            parca::ekle(tv, bx, by, 9.0 * u, kos(rk, 0.8 * par), 1.0);
            tv.daire(bx - 3.0 * u, by - 4.0 * u, 2.5 * u, kar(rk, Renk(255, 255, 255), 0.7), par);
            no += 1;
        }
    }
}

// ---------------------------------------------------------------- çam ağacı
fn cam_agaci(tv: &mut Tuval, cx: f32, taban: f32, boy: f32, z: f32, sus: usize, giris: bool) {
    let u = boy / 1150.0;
    let (yuz, golge, isik) = uc_ton(YESIL);
    let kat = 5;
    let govde_h = boy * 0.09;
    tv.dikd(cx - boy * 0.05, taban - govde_h, cx + boy * 0.05, taban, Renk(110, 70, 40), 1.0);
    tv.dikd(cx + boy * 0.01, taban - govde_h, cx + boy * 0.05, taban, Renk(70, 44, 26), 1.0);
    let ust_y = taban - boy;
    let kat_h = (boy - govde_h) / kat as f32;
    let mut sus_no = 0usize;
    for i in 0..kat {
        // giriş: katlar alttan üste 0,25 sn arayla geri eğrisiyle oturur
        let k = if giris { anim::geri(((z - i as f32 * 0.25) / 0.6).clamp(0.0, 1.0)) } else { 1.0 };
        if k <= 0.0 { continue; }
        let alt = taban - govde_h - i as f32 * kat_h * 0.82 + (1.0 - k) * -120.0 * u;
        let tepe = alt - kat_h * 1.45;
        let gen = boy * 0.46 * (1.0 - i as f32 * 0.16);
        let dis = 7;
        let mut tum = vec![(cx, tepe)];
        let mut sol = vec![(cx, tepe)];
        for d in 0..=dis * 2 {
            let p = d as f32 / (dis * 2) as f32;
            let x = cx + gen - p * gen * 2.0;
            let y = if d % 2 == 0 { alt } else { alt - kat_h * 0.16 };
            tum.push((x, y));
            if x <= cx + 0.5 { sol.push((x, y)); }
        }
        sol.push((cx, alt));
        let gol: Vec<(f32, f32)> = tum.iter().map(|p| (p.0 + 10.0 * u, p.1 + 10.0 * u)).collect();
        tv.cokgen(&gol, KOYU, 0.55);
        tv.cokgen(&tum, golge, 1.0);
        let sol_rev: Vec<(f32, f32)> = sol.clone();
        tv.cokgen(&sol_rev, yuz, 1.0);
        tv.cizgi(cx - gen * 0.15, tepe + kat_h * 0.35, cx - gen * 0.55, alt - kat_h * 0.25, 6.0 * u, isik, 0.8);
        // ışık dizisi: kat boyunca sarkan yay, koşan ampuller (≤ 1 Hz)
        if k >= 1.0 {
            let (a, b) = ((cx - gen * 0.82, alt - kat_h * 0.55), (cx + gen * 0.82, alt - kat_h * 0.3));
            let adet = 8;
            for j in 0..=adet {
                let p = j as f32 / adet as f32;
                let x = a.0 + (b.0 - a.0) * p;
                let y = a.1 + (b.1 - a.1) * p + kat_h * 0.22 * 4.0 * p * (1.0 - p);
                let rk = SUS[(i + j) % 4];
                let q = fr(z * 0.8 - (i * 9 + j) as f32 / 9.0);
                let par = 0.55 + 0.45 * yumusak(1.0 - (q * 2.0 - 1.0).abs());
                tv.daire(x, y, 9.0 * u, kos(rk, 0.3), 1.0);
                parca::ekle(tv, x, y, 8.0 * u, kos(rk, 0.75 * par), 1.0);
            }
            // süsler: kat kenarında toplar, sol üstte ışık noktası
            let bu_kat = (sus + kat - 1 - i) / kat;
            for j in 0..bu_kat {
                let p = (j as f32 + 0.5) / bu_kat as f32;
                let x = cx - gen * 0.8 + p * gen * 1.6;
                let y = alt - kat_h * (0.08 + 0.12 * hash(sus_no as u32 * 7 + 3));
                let rk = SUS[(sus_no * 5 + 1) % SUS.len()];
                let r = (17.0 + 6.0 * hash(sus_no as u32)) * u;
                tv.daire(x + 3.0 * u, y + 3.0 * u, r, KOYU, 0.6);
                tv.daire(x, y, r, kos(rk, 0.7), 1.0);
                tv.daire(x - r * 0.15, y - r * 0.15, r * 0.75, rk, 1.0);
                tv.daire(x - r * 0.38, y - r * 0.38, r * 0.22, kar(rk, Renk(255, 255, 255), 0.7), 1.0);
                sus_no += 1;
            }
        }
    }
    // tepe yıldızı ±8° salınır
    let ac = (z * 1.4).sin() * 0.14;
    let gir = if giris { anim::geri(((z - 1.3) / 0.5).clamp(0.0, 1.0)) } else { 1.0 };
    if gir > 0.0 {
        let r = 80.0 * u * gir;
        tv.yildiz(cx + 6.0 * u, ust_y + 6.0 * u, r * 1.1, 0.45, 5, ac, KOYU, 0.7);
        tv.yildiz(cx, ust_y, r * 1.08, 0.45, 5, ac, kos(ALTIN, 0.6), 1.0);
        tv.yildiz(cx, ust_y, r * 0.88, 0.45, 5, ac, ALTIN, 1.0);
        tv.yildiz(cx - r * 0.12, ust_y - r * 0.12, r * 0.35, 0.45, 5, ac, kar(ALTIN, Renk(255, 255, 255), 0.6), 1.0);
    }
}

/// Sahte 3B hediye kutusu (ön yüz, sağ yan koyu, üst açık) + kurdele + fiyonk.
pub fn hediye(tv: &mut Tuval, cx: f32, alt: f32, gen: f32, boy: f32, renk: Renk, kurdele: Renk) {
    let d = gen * 0.22;
    let (yuz, golge, isik) = uc_ton(renk);
    let (x0, x1, y0) = (cx - gen / 2.0, cx + gen / 2.0, alt - boy);
    tv.cokgen(&[(x1, y0), (x1 + d, y0 - d * 0.6), (x1 + d, alt - d * 0.6), (x1, alt)], golge, 1.0);
    tv.cokgen(&[(x0, y0), (x0 + d, y0 - d * 0.6), (x1 + d, y0 - d * 0.6), (x1, y0)], isik, 1.0);
    tv.dikd(x0, y0, x1, alt, yuz, 1.0);
    let kw = gen * 0.14;
    tv.dikd(cx - kw / 2.0, y0, cx + kw / 2.0, alt, kurdele, 1.0);
    tv.cokgen(&[(cx - kw / 2.0 + d * 0.5, y0 - d * 0.3), (cx + kw / 2.0 + d * 0.5, y0 - d * 0.3), (cx + kw / 2.0, y0), (cx - kw / 2.0, y0)], kar(kurdele, Renk(255, 255, 255), 0.3), 1.0);
    tv.cokgen(&[(x1, y0 + boy * 0.42), (x1 + d, y0 + boy * 0.42 - d * 0.6), (x1 + d, y0 + boy * 0.42 - d * 0.6 + kw), (x1, y0 + boy * 0.42 + kw)], kos(kurdele, 0.62), 1.0);
    tv.dikd(x0, y0 + boy * 0.42, x1, y0 + boy * 0.42 + kw, kurdele, 1.0);
    let (fx, fy) = (cx + d * 0.5, y0 - d * 0.3);
    for s in [-1.0f32, 1.0] { tv.elips(fx + s * gen * 0.13, fy - gen * 0.07, gen * 0.14, gen * 0.07, s * 0.5, kurdele, 1.0); }
    tv.daire(fx, fy - gen * 0.05, gen * 0.06, kos(kurdele, 0.8), 1.0);
}

fn kar_tumsegi(tv: &mut Tuval, cx: f32, y: f32, rx: f32, ry: f32) {
    tv.elips(cx + rx * 0.04, y + ry * 0.1, rx, ry, 0.0, KAR_GOLGE, 1.0);
    tv.elips(cx - rx * 0.03, y, rx * 0.97, ry * 0.9, 0.0, KAR, 1.0);
}

// ---------------------------------------------------------------- kardan adam
fn kardan_adam(tv: &mut Tuval, cx: f32, taban: f32, boy: f32, z: f32, bak: f32) {
    let u = boy / 1100.0;
    let golge_top = |tv: &mut Tuval, x: f32, y: f32, r: f32| {
        tv.daire(x, y, r + 5.0 * u, KOYU, 0.5);
        tv.daire(x, y, r, KAR_GOLGE, 1.0);
        tv.daire(x - r * 0.1, y - r * 0.1, r * 0.9, KAR, 1.0);
    };
    let (r1, r2, r3) = (boy * 0.2, boy * 0.15, boy * 0.11);
    let (y1, y2, y3) = (taban - r1, taban - r1 * 2.0 - r2 * 0.8, taban - r1 * 2.0 - r2 * 1.6 - r3 * 0.85);
    // kollar (sağ kol 1,2 sn'de bir el sallar ±18°)
    let sal = (z * TAU / 1.2).sin() * 0.31;
    for (s, a) in [(-1.0f32, 0.35f32), (1.0, -0.6 + sal)] {
        let (bx, by) = (cx + s * r2 * 0.85, y2 - r2 * 0.1);
        let l = r2 * 1.7;
        let ang = if s < 0.0 { PI + a } else { a };
        let (ex, ey) = (bx + ang.cos() * l, by + ang.sin() * l);
        tv.cizgi(bx, by, ex, ey, 12.0 * u, Renk(100, 64, 36), 1.0);
        let (mx, my) = (bx + ang.cos() * l * 0.7, by + ang.sin() * l * 0.7);
        tv.cizgi(mx, my, mx + (ang - 0.6).cos() * l * 0.3, my + (ang - 0.6).sin() * l * 0.3, 8.0 * u, Renk(100, 64, 36), 1.0);
    }
    golge_top(tv, cx, y1, r1);
    golge_top(tv, cx, y2, r2);
    golge_top(tv, cx, y3, r3);
    for i in 0..3 { tv.daire(cx, y2 - r2 * 0.45 + i as f32 * r2 * 0.42, 11.0 * u, KOYU, 1.0); }
    // atkı
    let atki = Renk(220, 40, 50);
    tv.yuvarlak(cx, y3 + r3 * 0.85, r3 * 1.05, r3 * 0.2, r3 * 0.18, kos(atki, 0.75), 1.0);
    tv.yuvarlak(cx, y3 + r3 * 0.8, r3 * 1.0, r3 * 0.16, r3 * 0.15, atki, 1.0);
    let mut px = (cx + r3 * 0.5, y3 + r3 * 0.9);
    for k in 1..8 { let y = y3 + r3 * 0.9 + k as f32 * r3 * 0.16; let x = cx + r3 * 0.5 + (z * 3.0 + k as f32 * 0.8).sin() * r3 * 0.12 * k as f32 / 7.0; tv.cizgi(px.0, px.1, x, y, r3 * 0.28, if k % 2 == 0 { atki } else { kar(atki, Renk(255, 255, 255), 0.35) }, 1.0); px = (x, y); }
    // yüz
    for s in [-1.0f32, 1.0] { tv.daire(cx + s * r3 * 0.35 + bak * r3 * 0.12, y3 - r3 * 0.2, 9.0 * u, KOYU, 1.0); }
    let nx = cx + bak * r3 * 0.15;
    tv.cokgen(&[(nx, y3 - r3 * 0.02), (nx, y3 + r3 * 0.18), (nx + bak.signum().max(-1.0) * r3 * 0.0 + r3 * 0.75 * if bak < 0.0 { -1.0 } else { 1.0 }, y3 + r3 * 0.1)], Renk(255, 130, 30), 1.0);
    for i in 0..5 { let a = PI * (0.2 + 0.15 * i as f32); tv.daire(cx + a.cos() * r3 * 0.5, y3 + r3 * 0.15 + a.sin() * r3 * 0.35, 5.0 * u, KOYU, 1.0); }
    // silindir şapka
    let sy = y3 - r3 * 0.75;
    tv.yuvarlak(cx, sy, r3 * 0.95, r3 * 0.1, r3 * 0.05, Renk(28, 28, 34), 1.0);
    tv.dikd(cx - r3 * 0.6, sy - r3 * 0.95, cx + r3 * 0.6, sy, Renk(28, 28, 34), 1.0);
    tv.dikd(cx - r3 * 0.6, sy - r3 * 0.3, cx + r3 * 0.6, sy - r3 * 0.12, Renk(200, 40, 50), 1.0);
    tv.dikd(cx - r3 * 0.5, sy - r3 * 0.9, cx - r3 * 0.38, sy - r3 * 0.35, Renk(70, 70, 82), 1.0);
}

// ---------------------------------------------------------------- buz kristali
fn kristal(tv: &mut Tuval, cx: f32, cy: f32, r: f32, buyume: f32, aci: f32, renk: Renk) {
    let (koyu, acik) = (kos(renk, 0.55), kar(renk, Renk(255, 255, 255), 0.55));
    let kal = (r * 0.045).max(3.0);
    for (k, rk) in [(kal * 2.2, koyu), (kal, acik)] {
        for i in 0..6 {
            let a = aci + i as f32 * PI / 3.0;
            let l = r * buyume;
            let (ex, ey) = (cx + a.cos() * l, cy + a.sin() * l);
            tv.cizgi(cx, cy, ex, ey, k, rk, 1.0);
            for (j, (pos, oran)) in [(0.35f32, 0.45f32), (0.58, 0.35), (0.8, 0.22)].iter().enumerate() {
                let bp = pos * r;
                if bp > l { continue; }
                let dl = (oran * r).min((l - bp) * 1.2 + oran * r * 0.2);
                let (bx, by) = (cx + a.cos() * bp, cy + a.sin() * bp);
                for s in [-1.0f32, 1.0] { let b = a + s * PI / 3.0; tv.cizgi(bx, by, bx + b.cos() * dl, by + b.sin() * dl, k * (0.85 - j as f32 * 0.1), rk, 1.0); }
            }
            if buyume >= 0.98 { let p: Vec<(f32, f32)> = (0..6).map(|q| { let b = a + q as f32 * PI / 3.0; (ex + b.cos() * r * 0.07, ey + b.sin() * r * 0.07) }).collect(); tv.cokgen(&p, rk, 1.0); }
        }
    }
    tv.daire(cx, cy, r * 0.09, acik, 1.0);
}

// ---------------------------------------------------------------- kış manzarası
fn manzara(tv: &mut Tuval, z: f32, yon: f32, tohum: u32) {
    let (wf, hf) = (tv.w as f32, tv.h as f32);
    let u = (wf / 950.0).min(hf / 1800.0);
    // gökyüzü: 5 bant, sınırlar 2x2 dama ile
    let bantlar = [Renk(10, 14, 34), Renk(16, 24, 52), Renk(24, 36, 72), Renk(34, 52, 96), Renk(48, 70, 120)];
    let bh = hf * 0.62 / 5.0;
    for (i, r) in bantlar.iter().enumerate() {
        let y0 = i as f32 * bh;
        tv.dikd(0.0, y0, wf, y0 + bh, *r, 1.0);
        if i + 1 < bantlar.len() { let mut x = 0.0; while x < wf { tv.dikd(x, y0 + bh - 4.0, x + 2.0, y0 + bh - 2.0, bantlar[i + 1], 1.0); tv.dikd(x + 2.0, y0 + bh - 2.0, x + 4.0, y0 + bh, bantlar[i + 1], 1.0); x += 4.0; } }
    }
    tv.dikd(0.0, hf * 0.62, wf, hf, bantlar[4], 1.0);
    // ay (kraterli)
    let (mx, my, mr) = (wf * 0.74, hf * 0.12, 70.0 * u);
    tv.daire(mx, my, mr, Renk(225, 225, 210), 1.0);
    tv.daire(mx + mr * 0.12, my + mr * 0.12, mr * 0.92, Renk(240, 238, 222), 1.0);
    for (dx, dy, r) in [(-0.3f32, -0.2f32, 0.18f32), (0.25, 0.3, 0.12), (0.1, -0.45, 0.09)] { tv.daire(mx + dx * mr, my + dy * mr, r * mr, Renk(200, 198, 184), 1.0); }
    let w_ = wf + 1.0;
    let kay = |hiz: f32| (yon * hiz * u * z).rem_euclid(w_);
    // uzak dağlar (kar tepeli)
    let k1 = kay(4.0);
    let mut x = 0.0;
    while x < wf {
        let gx = (x - k1).rem_euclid(w_);
        let g = 0.6 * gurultu(gx / (220.0 * u), tohum) + 0.4 * gurultu(gx / (70.0 * u), tohum + 3);
        let tepe = hf * 0.42 + g * hf * 0.12;
        tv.dikd(x, tepe, x + 2.0, hf * 0.66, Renk(60, 76, 116), 1.0);
        tv.dikd(x, tepe, x + 2.0, tepe + 26.0 * u * (1.0 - g * 0.5), Renk(210, 220, 238), 1.0);
        x += 2.0;
    }
    // çam siluetleri sırası
    let k2 = kay(10.0);
    for i in 0..12 {
        let bx = (i as f32 * wf / 12.0 + hash(tohum + i) * 40.0 * u + k2).rem_euclid(w_ + 160.0 * u) - 80.0 * u;
        let b = (110.0 + 70.0 * hash(tohum + 40 + i)) * u;
        let alt = hf * 0.68;
        for k in 0..3 { let gy = alt - k as f32 * b * 0.28; let gg = b * 0.42 * (1.0 - k as f32 * 0.25); tv.cokgen(&[(bx, gy - b * 0.5), (bx + gg, gy), (bx - gg, gy)], Renk(22, 48, 50), 1.0); }
    }
    // ön tepe + kulübe
    let k3 = kay(22.0);
    let mut x = 0.0;
    while x < wf { let gx = (x - k3).rem_euclid(w_); let tepe = hf * 0.7 + gurultu(gx / (300.0 * u), tohum + 9) * hf * 0.06; tv.dikd(x, tepe, x + 2.0, hf, Renk(205, 215, 235), 1.0); tv.dikd(x, tepe, x + 2.0, tepe + 6.0 * u, Renk(250, 252, 255), 1.0); x += 2.0; }
    let kx = (wf * 0.3 + kay(22.0)).rem_euclid(w_ + 400.0 * u) - 200.0 * u;
    let ky = hf * 0.74;
    let (kw, kh) = (240.0 * u, 160.0 * u);
    tv.dikd(kx - kw / 2.0, ky - kh, kx + kw / 2.0, ky, Renk(110, 66, 40), 1.0);
    for i in 0..5 { tv.dikd(kx - kw / 2.0, ky - kh + i as f32 * kh / 5.0, kx + kw / 2.0, ky - kh + i as f32 * kh / 5.0 + 4.0 * u, Renk(80, 46, 28), 1.0); }
    tv.cokgen(&[(kx - kw * 0.62, ky - kh), (kx, ky - kh - 110.0 * u), (kx + kw * 0.62, ky - kh)], Renk(240, 244, 252), 1.0);
    tv.dikd(kx - 34.0 * u, ky - kh * 0.7, kx + 34.0 * u, ky - kh * 0.25, Renk(255, 200, 80), 1.0);
    tv.dikd(kx - 3.0 * u, ky - kh * 0.7, kx + 3.0 * u, ky - kh * 0.25, Renk(80, 46, 28), 1.0);
    let (bcx, bcy) = (kx + kw * 0.28, ky - kh - 70.0 * u);
    tv.dikd(bcx - 16.0 * u, bcy, bcx + 16.0 * u, bcy + 60.0 * u, Renk(90, 60, 50), 1.0);
    for i in 0..5 { let p = fr(z * 0.25 + i as f32 / 5.0); tv.daire(bcx + p * 60.0 * u + (z + i as f32).sin() * 8.0 * u, bcy - p * 260.0 * u, (14.0 + 30.0 * p) * u, Renk(170, 176, 190), 0.55 * (1.0 - p)); }
}

// ---------------------------------------------------------------- ana yönlendirici
/// Kış kalemlerini çizer; id bu modüle ait değilse false.
pub fn ciz(m: &MalzemeAyar, yazi: &str, z: &Hazir, tv: &mut Tuval, yerel: f32, sure: f32) -> bool {
    let (wf, hf) = (tv.w as f32, tv.h as f32);
    let u = (wf / 950.0).min(hf / 1800.0);
    let (cx, cy) = (wf / 2.0, hf / 2.0);
    let tt = yerel * m.hiz;
    let dv = m.deger;
    let ruzgar = if m.yon == "sol" { -1.0 } else { 1.0 };
    let sure_z = sure * m.hiz;
    match m.id.as_str() {
        "kar-katmanli" => {
            kar_katmanli(tv, tt, sure_z, (dv.round() as usize).clamp(40, 400), ruzgar, 0b111, 21);
            if !yazi.is_empty() { yazi_3b(tv, z, yazi, cx, hf * 0.42, wf * 0.86, 1.0, (tt * 0.6).sin() * 0.12, u); }
        }
        "kar-birikme" => {
            kar_katmanli(tv, tt, sure_z, 160, 1.0, 0b011, 23);
            let yuk = (tt / (0.8 * sure_z).max(0.5)).min(1.0);
            kar_ortusu(tv, yuk, dv.clamp(5.0, 30.0) / 100.0, 5);
            let ust = hf * (1.0 - dv.clamp(5.0, 30.0) / 100.0 * yuk) - 230.0 * u;
            if !yazi.is_empty() { yazi_3b(tv, z, yazi, cx, ust.min(hf * 0.5), wf * 0.86, 1.0, 0.0, u); }
            kar_katmanli(tv, tt, sure_z, 40, 1.0, 0b100, 23);
        }
        "cam-agaci" => {
            kar_katmanli(tv, tt, sure_z, 120, 1.0, 0b011, 31);
            kar_tumsegi(tv, cx, hf * 0.9, wf * 0.62, hf * 0.06);
            cam_agaci(tv, cx, hf * 0.88, hf * 0.66, tt, (dv.round() as usize).clamp(6, 40), true);
            hediye(tv, cx - 260.0 * u, hf * 0.92, 150.0 * u, 120.0 * u, Renk(200, 30, 50), ALTIN);
            hediye(tv, cx + 230.0 * u, hf * 0.93, 120.0 * u, 100.0 * u, Renk(40, 110, 220), Renk(240, 240, 250));
            if !yazi.is_empty() { piksel_3b(tv, yazi, cx, hf * 0.07, wf * 0.9, 24.0 * u, ALTIN); }
        }
        "fisek-kabarcik" | "fisek-sogut" | "fisek-halka" | "fisek-kalp" | "fisek-cift" => {
            let t = match m.id.as_str() { "fisek-sogut" => Tur::Sogut, "fisek-halka" => Tur::Halka, "fisek-kalp" => Tur::Kalp, "fisek-cift" => Tur::Cift, _ => Tur::Kure };
            parca::gosteri(tv, tt, (dv.round() as usize).clamp(1, 10), &|_| t, &FISEK, 41, 0.12, 0.42);
            if !yazi.is_empty() { piksel_3b(tv, yazi, cx, hf * 0.8, wf * 0.9, 30.0 * u, Renk(255, 248, 230)); }
        }
        "yilbasi-senlik" => {
            parca::gosteri(tv, tt, (dv.round() as usize).clamp(3, 14), &|j| [Tur::Kure, Tur::Sogut, Tur::Halka, Tur::Kalp, Tur::Cift][(hash(j as u32 * 17 + 3) * 5.0) as usize % 5], &FISEK, 51, 0.1, 0.36);
            // şehir silueti: binalar, sarı pencereler (%6'sı 4 sn'de bir söner)
            let mut x = -20.0 * u;
            let mut i = 0u32;
            while x < wf {
                let bw = (70.0 + 70.0 * hash(i * 3 + 1)) * u;
                let bh = (220.0 + 330.0 * hash(i * 3 + 2)) * u;
                let ust = hf - bh;
                tv.dikd(x, ust, x + bw - 6.0 * u, hf, Renk(18, 22, 38), 1.0);
                tv.dikd(x + bw - 18.0 * u, ust, x + bw - 6.0 * u, hf, Renk(12, 14, 26), 1.0);
                let mut wy = ust + 24.0 * u;
                let mut no = 0u32;
                while wy < hf - 40.0 * u {
                    let mut wx = x + 14.0 * u;
                    while wx < x + bw - 34.0 * u {
                        let hs = hash(i * 1000 + no);
                        if hs > 0.35 {
                            let yanik = !(hs > 0.94 && fr(tt / 4.0 + hs * 3.0) < 0.5);
                            if yanik { tv.dikd(wx, wy, wx + 16.0 * u, wy + 22.0 * u, Renk(255, 206, 90), 1.0); }
                        }
                        wx += 28.0 * u; no += 1;
                    }
                    wy += 40.0 * u;
                }
                x += bw; i += 1;
            }
            let yazi_y = hf * 0.52;
            yazi_3b(tv, z, yazi, cx, yazi_y, wf * 0.88, 1.0, (tt * 0.7).sin() * 0.1, u);
        }
        "yilbasi-geri-sayim" => {
            kar_katmanli(tv, tt, sure_z, 140, 1.0, 0b001, 61);
            let bas = dv.clamp(3.0, 60.0);
            let kalan = bas - tt;
            let rr = 380.0 * u;
            if kalan > 0.0 {
                let n = kalan.ceil() as i64;
                let f = 1.0 - (kalan - kalan.floor()); // saniyenin geçen kısmı (0 → yeni rakam)
                let f = if kalan == kalan.floor() { 0.0 } else { f };
                tv.halka(cx, cy, rr, 36.0 * u, Renk(30, 40, 70), 1.0);
                tv.yay(cx, cy, rr, 36.0 * u, -PI / 2.0, 1.0 - f, z.vurgu, 1.0);
                // yeni rakam üstten düşüp geri eğrisiyle oturur; eskisi alta kayıp söner
                let k = anim::geri((f / 0.35).min(1.0));
                let dy = (1.0 - k) * -420.0 * u;
                if f < 0.3 && n + 1 <= bas.ceil() as i64 {
                    let eski = (n + 1).to_string();
                    let e = f / 0.3;
                    let h = ((rr * 1.3) / (eski.len() as f32 * 6.0 - 1.0)).min(60.0 * u);
                    tv.piksel_yazi(&eski, cx, cy + e * 380.0 * u, h, kos(Renk(255, 248, 230), 1.0 - e), 1.0 - e);
                }
                let s = n.to_string();
                piksel_3b(tv, &s, cx, cy + dy, rr * 1.3, 60.0 * u, Renk(255, 248, 230));
            } else {
                parca::gosteri(tv, -kalan + 1.2, 4, &|j| if j % 2 == 0 { Tur::Kure } else { Tur::Cift }, &FISEK, 63, 0.1, 0.3);
                yazi_3b(tv, z, yazi, cx, cy, wf * 0.88, anim::geri((-kalan / 0.6).min(1.0)), 0.0, u);
            }
        }
        "kardan-adam" => {
            kar_katmanli(tv, tt, sure_z, 140, 1.0, 0b011, 71);
            kar_tumsegi(tv, cx, hf * 0.92, wf * 0.7, hf * 0.08);
            let bak = if m.yon == "sol" { -1.0 } else { 1.0 };
            kardan_adam(tv, cx, hf * 0.9, hf * 0.66, tt, bak);
            if !yazi.is_empty() { piksel_3b(tv, yazi, cx, hf * 0.08, wf * 0.9, 26.0 * u, Renk(255, 248, 230)); }
        }
        "buz-kristali" => {
            let n = (dv.round() as usize).clamp(1, 9);
            let rk = kar(z.yuz, Renk(160, 220, 255), 0.6);
            for i in 0..n {
                let (x, y, r, faz) = if i == 0 { (cx, cy, 330.0 * u, 0.0) } else {
                    let a = i as f32 / (n - 1).max(1) as f32 * TAU + 0.4;
                    (cx + a.cos() * wf * 0.36, cy + a.sin() * hf * 0.33, (70.0 + 50.0 * hash(i as u32 * 7)) * u, 0.3 + hash(i as u32) * 1.2)
                };
                let b = yumusak(((tt - faz) / 2.0).clamp(0.0, 1.0));
                let aci = (tt - faz - 2.0).max(0.0) * 12f32.to_radians() * if i % 2 == 0 { 1.0 } else { -1.0 };
                if b > 0.0 { kristal(tv, x, y, r, b, aci, rk); }
            }
            if !yazi.is_empty() { piksel_3b(tv, yazi, cx, hf * 0.9, wf * 0.9, 26.0 * u, Renk(235, 248, 255)); }
        }
        "kis-manzarasi" => {
            let yon = if m.yon == "sol" { -1.0 } else { 1.0 };
            manzara(tv, tt, yon, 81);
            kar_katmanli(tv, tt, sure_z, 160, yon, 0b111, 83);
            if !yazi.is_empty() { yazi_3b(tv, z, yazi, cx, hf * 0.3, wf * 0.86, 1.0, 0.0, u); }
        }
        "hediye-yigini" => {
            kar_tumsegi(tv, cx, hf * 0.9, wf * 0.66, hf * 0.05);
            let n = (dv.round() as usize).clamp(3, 7);
            let yer = [(0.0f32, 0.0f32, 300.0f32, 220.0f32), (-250.0, 0.0, 220.0, 180.0), (240.0, 0.0, 230.0, 200.0), (-120.0, -220.0, 220.0, 170.0), (110.0, -200.0, 200.0, 160.0), (0.0, -380.0, 180.0, 150.0), (-330.0, -180.0, 150.0, 130.0)];
            let renk = [Renk(200, 30, 50), Renk(40, 110, 220), Renk(40, 150, 80), Renk(220, 160, 30), Renk(150, 60, 200), Renk(230, 90, 40), Renk(30, 160, 170)];
            let kurd = [ALTIN, Renk(240, 240, 250), Renk(230, 40, 50), Renk(200, 30, 50), ALTIN, Renk(240, 240, 250), ALTIN];
            let taban = hf * 0.88;
            for i in 0..n {
                let (ox, oy, gw, gh) = yer[i];
                let k = anim::ziplama(((tt - i as f32 * 0.45) / 0.9).clamp(0.0, 1.0));
                if tt - i as f32 * 0.45 <= 0.0 { continue; }
                let alt = taban + oy * u - (1.0 - k) * hf * 0.9;
                hediye(tv, cx + ox * u, alt, gw * u, gh * u, renk[i], kurd[i]);
            }
            if !yazi.is_empty() { piksel_3b(tv, yazi, cx, hf * 0.18, wf * 0.9, 28.0 * u, Renk(255, 248, 230)); }
        }
        _ => return false,
    }
    let _ = piksel_gen;
    true
}

/// Kış efektleri (`efekt=` adları): dekor::ciz'den çağrılır. Bilinmeyen ad false.
pub fn efekt(ad: &str, t: &mut [u8], w: u32, h: u32, z: f32, tohum: u32) -> bool {
    let mut tv = Tuval { t, w, h };
    let u = (w as f32 / 950.0).min(h as f32 / 1800.0);
    match ad {
        "kar-katmanli" => kar_katmanli(&mut tv, z, 12.0, 140, 1.0, 0b111, tohum),
        "kar-ortusu" => kar_ortusu(&mut tv, 1.0, 0.1, tohum),
        "isik-dizisi" => isik_dizisi(&mut tv, z, 40.0 * u, 90.0 * u, tohum),
        _ => return false,
    }
    true
}

#[cfg(test)]
mod t {
    use super::*;
    #[test]
    fn gurultu_aralikta_ve_surekli() {
        for i in 0..200 { let x = i as f32 * 0.137; let g = gurultu(x, 3); assert!((0.0..=1.0).contains(&g)); assert!((gurultu(x + 0.001, 3) - g).abs() < 0.01); }
    }
    #[test]
    fn kar_dongu_guvenli() {
        let (w, h) = (190u32, 360u32);
        let ciz = |z: f32| { let mut b = vec![0u8; (w * h * 3) as usize]; { let mut tv = Tuval { t: &mut b, w, h }; kar_katmanli(&mut tv, z, 10.0, 120, 1.0, 0b111, 1); } b };
        // t=0 ile t=sure aynı kare (rüzgâr ve salınım periyotları süreye bölünür)
        let (a, b) = (ciz(0.0), ciz(10.0));
        let fark = a.iter().zip(&b).filter(|(x, y)| (**x as i32 - **y as i32).abs() > 8).count();
        assert!(fark < a.len() / 200, "döngü dikişi: {fark}");
    }
}
