//! Premium sahneler ve 3B/2B yazı animasyonları (KARAR-ANIM 4. ve 6. bölüm):
//! yazi-3b-don (ekstrüde dönüş), yazi-3b-derin (Z'den gelme + hayalet kopyalar), yazi-yil, harf-kaskad/dalga/oynak/dus
//! (harf harf kinetik tipografi, 3 tonlu piksel harf), altin-toz, isik-huzmesi, izgara-zemin; efektler altin-toz, isik-huzmesi, vinyet.
//! Işıma/neon/bulanıklık yok: derinlik, sert gölge, bantlı (dama dither'li) ton. Kapalı biçimli, deterministik.
//! Sahip: Yusuf. Yazan: [Claude#Malzeme Opus 5.5, yenileme-1.0, 2026-11-25]. Yalnız yazan yapay zekâ ya da Yusuf'un izniyle değişir; silme yok.
use crate::anim::{self, Poz};
use crate::efekt::hash;
use crate::malzeme::MalzemeAyar;
use crate::malzeme_ciz::Tuval;
use crate::malzeme_kis::{kar, kos, kar_katmanli, isik_dizisi};
use crate::senaryo::Hazir;
use crate::Renk;

const PI: f32 = std::f32::consts::PI;
const TAU: f32 = std::f32::consts::TAU;
const KOYU: Renk = Renk(14, 16, 24);
const ALTIN: Renk = Renk(255, 200, 70);

fn fr(x: f32) -> f32 { x - x.floor() }

/// TTF maskeli 3B yazı (Hazir.maskeler[0]); `aci` dikey eksende dönüş (kare3d yan yüzü sinθ ile açılır).
fn yazi(tv: &mut Tuval, z: &Hazir, yuz: Renk, yan: Renk, cx: f32, cy: f32, olcek: f32, aci: f32, derin: f32, parlak: f32) {
    if let Some(m) = z.maskeler.first() {
        let (mx, my) = ((m.bx.0 + m.bx.2) as f32 / 2.0, (m.bx.1 + m.bx.3) as f32 / 2.0);
        anim::kare3d_uzerine(tv.t, m, yuz, yan, &Poz { aci, olcek: olcek.max(0.02), parlak, derinlik: derin, dx: cx - mx, dy: cy - my });
    }
}

/// Dönüş açısı: 8 sn'de bir tam tur, önde 2 sn bekler (yumuşak hızlanma/yavaşlama).
fn don_aci(tt: f32) -> f32 {
    let p = fr(tt / 8.0);
    if p < 0.25 { 0.0 } else { anim::yumusak((p - 0.25) / 0.75) * TAU }
}

/// Bantlı dikey zemin (4 ton, sınırlar dama dither'li): koyu üst → zemin alt.
fn bantli_zemin(tv: &mut Tuval, ust: Renk, alt: Renk, y0: f32, y1: f32) {
    let wf = tv.w as f32;
    let n = 4;
    let bh = (y1 - y0) / n as f32;
    for i in 0..n {
        let r = kar(ust, alt, i as f32 / (n - 1) as f32);
        let a = y0 + i as f32 * bh;
        tv.dikd(0.0, a, wf, a + bh + 1.0, r, 1.0);
        if i + 1 < n { let r2 = kar(ust, alt, (i + 1) as f32 / (n - 1) as f32); let mut x = 0.0; while x < wf { tv.dikd(x, a + bh - 4.0, x + 2.0, a + bh - 2.0, r2, 1.0); tv.dikd(x + 2.0, a + bh - 2.0, x + 4.0, a + bh, r2, 1.0); x += 4.0; } }
    }
}

// ---------------------------------------------------------------- harf animasyonları (piksel harf, 3 ton)
/// Tek harf: koyu ekstrüzyon (sağ alta 4 kademe) + koyu dış çizgi + yüz + üst yarıda ışık tonu şeridi.
fn harf(tv: &mut Tuval, c: char, cx: f32, cy: f32, h: f32, renk: Renk, a: f32) {
    if c == ' ' || a <= 0.0 { return; }
    let s = c.to_string();
    let d = (h * 0.55).max(2.0);
    for k in (1..=4).rev() { let o = d * k as f32 / 4.0; tv.piksel_yazi(&s, cx + o, cy + o, h, kos(renk, 0.3 + 0.08 * (4 - k) as f32), a); }
    let c2 = (h * 0.28).max(1.5);
    for (ox, oy) in [(-c2, 0.0), (c2, 0.0), (0.0, -c2), (0.0, c2)] { tv.piksel_yazi(&s, cx + ox, cy + oy, h, KOYU, a); }
    tv.piksel_yazi(&s, cx, cy, h, renk, a);
}

/// Harf yerleşimi: (harf, merkez x, merkez y, sıra) ve hücre boyu.
fn harf_yerlesim(yazi: &str, wf: f32, cy: f32, u: f32) -> (Vec<(char, f32, f32, usize)>, f32) {
    let satirlar = crate::senaryo::dikey_satirlar(yazi);
    let mut v = vec![];
    if satirlar.is_empty() { return (v, 0.0); }
    let h = satirlar.iter().map(|s| wf * 0.86 / (s.chars().count().max(1) as f32 * 6.0 - 1.0)).fold(34.0 * u, f32::min).max(2.0);
    let ara = h * 12.0;
    let y0 = cy - ara * (satirlar.len() as f32 - 1.0) / 2.0;
    let mut sira = 0;
    for (i, s) in satirlar.iter().enumerate() {
        let n = s.chars().count();
        let x0 = wf / 2.0 - ((n as f32 * 6.0 - 1.0) * h) / 2.0;
        for (j, c) in s.chars().enumerate() { v.push((c, x0 + j as f32 * 6.0 * h + 2.5 * h, y0 + i as f32 * ara, sira)); sira += 1; }
    }
    (v, h)
}

fn harf_sahne(tv: &mut Tuval, tur: &str, yazi: &str, renk: Renk, tt: f32, yerel: f32, sure: f32, u: f32) {
    let (wf, hf) = (tv.w as f32, tv.h as f32);
    let (hl, h) = harf_yerlesim(yazi, wf, hf * 0.5, u);
    let n = hl.len().max(1);
    let cikis = (yerel - (sure - 0.8)).max(0.0) / 0.8; // son 0,8 sn ters sırayla yukarı
    for &(c, x, y, i) in &hl {
        let (mut dx, mut dy, mut olc, mut a) = (0.0f32, 0.0f32, 1.0f32, 1.0f32);
        match tur {
            "harf-kaskad" | "harf-dalga" => {
                let k = anim::geri(((tt - i as f32 * 0.06) / 0.5).clamp(0.0, 1.0));
                if tt - i as f32 * 0.06 <= 0.0 { a = 0.0; }
                dy = (1.0 - k) * 120.0 * u;
                olc = 0.6 + 0.4 * k;
                if tur == "harf-dalga" { let s = (tt - n as f32 * 0.06 - 0.5).max(0.0); let e = (s / 0.6).min(1.0); dy += e * 18.0 * u * (TAU * (tt * 0.8 - i as f32 / 8.0)).sin(); dx = e * 4.0 * u * (TAU * (tt * 0.8 - i as f32 / 8.0)).cos(); }
            }
            "harf-oynak" => {
                let k = anim::geri(((tt - i as f32 * 0.05) / 0.45).clamp(0.0, 1.0));
                if tt - i as f32 * 0.05 <= 0.0 { a = 0.0; }
                olc = 0.5 + 0.5 * k;
                // her harf kendi periyodunda (0,9–1,4 sn) kısa zıplar; pencere periyodun %30'u → aynı anda ~1/3'ü hareketli
                let per = 0.9 + 0.5 * hash(i as u32 * 7 + 1);
                let p = fr(tt / per + hash(i as u32 * 7 + 2));
                if p < 0.3 { let q = p / 0.3; dy = -(q * PI).sin() * 22.0 * u; dx = (q * TAU).sin() * 4.0 * u; }
            }
            _ => { // harf-dus: üstten rastgele sırayla düşer, iki kez seker; oturunca 6 px sönümlü sarsıntı
                let gec = hash(i as u32 * 13 + 5) * 1.2;
                let k = anim::ziplama(((tt - gec) / 0.8).clamp(0.0, 1.0));
                if tt - gec <= 0.0 { a = 0.0; }
                dy = -(1.0 - k) * (y + 200.0 * u);
                let son = tt - gec - 0.8;
                if son > 0.0 { dx = (son * 40.0).sin() * 6.0 * u * (-son * 5.0).exp(); }
            }
        }
        if cikis > 0.0 { let e = anim::yumusak((cikis * 1.6 - (n - 1 - i) as f32 / n as f32 * 0.6).clamp(0.0, 1.0)); dy -= e * hf * 0.6; a *= 1.0 - e; }
        harf(tv, c, x + dx, y + dy, h * olc, renk, a);
    }
}

// ---------------------------------------------------------------- premium efektler
/// Altın toz (A1): 3 derinlikte yükselen sert kenarlı diskler; ön katman en hızlı (paralaks); %10'u 4 köşeli pırıltı.
pub fn altin_toz(tv: &mut Tuval, z: f32, renk: Renk, tohum: u32) {
    let (wf, hf) = (tv.w as f32, tv.h as f32);
    let u = (wf / 950.0).min(hf / 1800.0);
    for (kat, adet, r, al, v) in [(0u32, 70usize, 2.0f32, 0.8f32, 24.0f32), (1, 34, 5.0, 0.5, 46.0), (2, 14, 11.0, 0.28, 80.0)] {
        let yol = hf + 60.0 * u;
        for i in 0..adet {
            let s = |o: u32| hash(tohum.wrapping_mul(613) ^ ((kat * 10_007 + i as u32) * 11 + o));
            let y = hf + 30.0 * u - (s(1) * yol + v * u * z).rem_euclid(yol);
            let x = s(2) * wf + (z * (0.3 + 0.4 * s(3)) + s(4) * TAU).sin() * 20.0 * u;
            let rk = kar(renk, Renk(255, 255, 255), s(5) * 0.4);
            if i % 10 == 0 { tv.yildiz(x, y, r * u * 2.6, 0.28, 4, 0.0, rk, al.max(0.5)); } else if r * u < 1.5 { tv.px(x as i32, y as i32, rk, al); } else { tv.daire(x, y, r * u, rk, al); }
        }
    }
}

/// Işık huzmesi (A3): üst merkezden 8 kama, alfa 0,10–0,18, ±6° ve 9 sn periyotla salınır; üst üste binince doğal bant.
pub fn isik_huzmesi(tv: &mut Tuval, z: f32, renk: Renk) {
    let (wf, hf) = (tv.w as f32, tv.h as f32);
    let (ox, oy) = (wf / 2.0, -hf * 0.05);
    let l = hf * 1.3;
    for i in 0..8 {
        let a0 = PI / 2.0 + (i as f32 - 3.5) * 0.16 + (TAU * z / 9.0 + i as f32 * 0.7).sin() * 6f32.to_radians();
        let gen = 0.035 + 0.02 * hash(i * 31 + 1);
        let al = 0.10 + 0.08 * hash(i * 31 + 2);
        tv.cokgen(&[(ox, oy), (ox + (a0 - gen).cos() * l, oy + (a0 - gen).sin() * l), (ox + (a0 + gen).cos() * l, oy + (a0 + gen).sin() * l)], kar(renk, Renk(255, 255, 255), 0.5), al);
    }
}

/// Vinyet (A2): kenarlardan içe 4 basamaklı koyulaşma (%0/12/24/38), basamak sınırları dama dither'li. Son katman olarak çizilir.
pub fn vinyet(t: &mut [u8], w: u32, h: u32) {
    let (wf, hf) = (w as f32, h as f32);
    let kat = [1.0f32, 0.88, 0.76, 0.62];
    for y in 0..h {
        let dy = (y as f32 / hf - 0.5).abs() * 2.0;
        for x in 0..w {
            let dx = (x as f32 / wf - 0.5).abs() * 2.0;
            let d = (dx.powi(4) + dy.powi(4)).sqrt().sqrt(); // yuvarlatılmış dikdörtgen uzaklığı 0..~1,19
            let q = ((d - 0.62) / 0.12).max(0.0);
            let mut b = q.floor() as usize;
            if fr(q) > 0.75 && (x + y) % 2 == 0 { b += 1; }
            let b = b.min(3);
            if b == 0 { continue; }
            let i = ((y * w + x) * 3) as usize;
            let k = kat[b];
            t[i] = (t[i] as f32 * k) as u8; t[i + 1] = (t[i + 1] as f32 * k) as u8; t[i + 2] = (t[i + 2] as f32 * k) as u8;
        }
    }
}

/// Perspektif ızgara zemin (A4): ufuk %45, yatay çizgiler öne akar, dikeyler kaçış noktasından yelpaze, kalınlık derinlikle 1→5 px.
fn izgara(tv: &mut Tuval, z: f32, renk: Renk, u: f32) {
    let (wf, hf) = (tv.w as f32, tv.h as f32);
    let ufuk = hf * 0.45;
    bantli_zemin(tv, Renk(8, 10, 24), kos(renk, 0.25), 0.0, ufuk);
    tv.dikd(0.0, ufuk, wf, hf, Renk(10, 10, 20), 1.0);
    let n = 14.0;
    let kay = fr(z * 0.5);
    for i in 0..14 {
        let q = i as f32 + kay;
        let y = ufuk + (hf - ufuk) * (q / n).powi(2);
        let kal = 1.0 + 4.0 * (q / n);
        tv.dikd(0.0, y - kal / 2.0, wf, y + kal / 2.0, kar(renk, KOYU, 0.15 + 0.5 * (1.0 - q / n)), 1.0);
    }
    for i in -10..=10 {
        let xb = wf / 2.0 + i as f32 * wf * 0.16;
        tv.cizgi(wf / 2.0, ufuk, xb, hf, (1.5 + 3.5 * (i as f32).abs() / 10.0) * u, kar(renk, KOYU, 0.35), 1.0);
    }
    tv.dikd(0.0, ufuk - 2.0 * u, wf, ufuk + 3.0 * u, kar(renk, Renk(255, 255, 255), 0.3), 1.0);
}

// ---------------------------------------------------------------- yönlendirici
pub fn ciz(m: &MalzemeAyar, yazi_s: &str, z: &Hazir, tv: &mut Tuval, yerel: f32, sure: f32) -> bool {
    let (wf, hf) = (tv.w as f32, tv.h as f32);
    let u = (wf / 950.0).min(hf / 1800.0);
    let (cx, cy) = (wf / 2.0, hf / 2.0);
    let tt = yerel * m.hiz;
    let (yuz, vu, zem) = (z.yuz, z.vurgu, z.zemin);
    let acik = kar(yuz, Renk(255, 255, 255), 0.25);
    match m.id.as_str() {
        "yazi-3b-don" | "yazi-yil" => {
            if m.id == "yazi-yil" { kar_katmanli(tv, tt, sure * m.hiz, 120, 1.0, 0b011, 91); }
            // zemin gölgesi (sert, bulanıksız elips) + taban çizgisi
            let aci = don_aci(tt);
            let gen = 0.25 + 0.75 * aci.cos().abs();
            tv.elips(cx, cy + 300.0 * u, 360.0 * u * gen, 30.0 * u, 0.0, kos(zem, 0.5), 1.0);
            yazi(tv, z, acik, kos(vu, 0.62), cx, cy, 1.0, aci, 24.0 * u, 1.0);
            if aci == 0.0 { let p = fr(tt / 8.0) / 0.25; crate::efekt::isik_supurme(tv.t, tv.w, tv.h, zem, if p < 1.0 { p } else { -1.0 }); }
            if m.id == "yazi-yil" { isik_dizisi(tv, tt, 30.0 * u, 80.0 * u, 3); }
        }
        "yazi-3b-derin" => {
            let k = (tt / 1.4).min(1.0);
            let e = if k >= 1.0 { 1.0 } else { 1.0 - 2f32.powf(-10.0 * k) };
            let olc = 0.25 + 0.75 * e;
            // 3 kademeli hayalet kopya geride kalır (alfa 0,25/0,15/0,08)
            if k < 1.0 { for (j, al) in [(3, 0.08f32), (2, 0.15), (1, 0.25)] { let kk = (k - j as f32 * 0.07).max(0.0); let ee = 1.0 - 2f32.powf(-10.0 * kk); yazi(tv, z, acik, kos(vu, 0.5), cx, cy, 0.25 + 0.75 * ee, 0.0, 4.0 * u, al); } }
            let sal = if k >= 1.0 { ((tt - 1.4) * 0.9).sin() * 0.18 } else { 0.0 };
            // bütçe (KARAR-ANIM 11.1: maliyet bbox × (derinlik+1)): derinlik 28 px ile sınırlı [Claude#Malzeme, 2026-11-25]
            let derin = 28.0 * u + (16.0 * u - 28.0 * u) * e;
            yazi(tv, z, acik, kos(vu, 0.62), cx, cy, olc, sal, derin.max(12.0 * u), 1.0);
        }
        "harf-kaskad" | "harf-dalga" | "harf-oynak" | "harf-dus" => {
            harf_sahne(tv, m.id.as_str(), yazi_s, acik, tt, yerel, sure, u);
        }
        "altin-toz" => {
            bantli_zemin(tv, Renk(10, 8, 6), kos(ALTIN, 0.18), 0.0, hf);
            altin_toz(tv, tt, ALTIN, 7);
            yazi(tv, z, Renk(255, 226, 140), Renk(140, 90, 30), cx, cy, 1.0, (tt * 0.5).sin() * 0.15, 20.0 * u, 1.0);
        }
        "isik-huzmesi" => {
            bantli_zemin(tv, Renk(6, 8, 16), kos(yuz, 0.22), 0.0, hf);
            isik_huzmesi(tv, tt, yuz);
            tv.elips(cx, hf * 0.62, 340.0 * u, 34.0 * u, 0.0, KOYU, 0.6);
            yazi(tv, z, acik, kos(vu, 0.6), cx, cy, 1.0, (tt * 0.5).sin() * 0.12, 22.0 * u, 1.0);
        }
        "sahne-karli-isik" => {
            bantli_zemin(tv, Renk(6, 10, 22), kos(yuz, 0.22), 0.0, hf);
            isik_huzmesi(tv, tt, Renk(200, 225, 255));
            kar_katmanli(tv, tt, sure * m.hiz, 150, 1.0, 0b011, 97);
            yazi(tv, z, acik, kos(vu, 0.6), cx, cy, 1.0, (tt * 0.5).sin() * 0.12, 22.0 * u, 1.0);
            kar_katmanli(tv, tt, sure * m.hiz, 40, 1.0, 0b100, 97);
        }
        "izgara-zemin" => {
            izgara(tv, tt, vu, u);
            yazi(tv, z, acik, kos(vu, 0.62), cx, hf * 0.3, 1.0, (tt * 0.6).sin() * 0.2, 24.0 * u, 1.0);
        }
        _ => return false,
    }
    true
}

/// Premium efekt adları (dekor::ciz'den, içerikten ÖNCE): altin-toz, isik-huzmesi. `vinyet` içerikten SONRA (`son_efekt`).
pub fn efekt(ad: &str, t: &mut [u8], w: u32, h: u32, z: f32, renk: Renk, tohum: u32) -> bool {
    let mut tv = Tuval { t, w, h };
    match ad {
        "altin-toz" => altin_toz(&mut tv, z, kar(renk, ALTIN, 0.6), tohum),
        "isik-huzmesi" => isik_huzmesi(&mut tv, z, renk),
        "vinyet" => {}
        _ => return false,
    }
    true
}

/// İçerik çizildikten sonra uygulanan efektler.
pub fn son_efekt(ad: &str, t: &mut [u8], w: u32, h: u32) { if ad == "vinyet" { vinyet(t, w, h); } }

#[cfg(test)]
mod t {
    use super::*;
    #[test]
    fn vinyet_kenari_koyulastirir_merkezi_korur() {
        let (w, h) = (190u32, 360u32);
        let mut b = vec![200u8; (w * h * 3) as usize];
        vinyet(&mut b, w, h);
        assert_eq!(b[((180 * w + 95) * 3) as usize], 200);
        assert!(b[0] < 140);
    }
    #[test]
    fn don_aci_once_bekler() { assert_eq!(don_aci(1.0), 0.0); assert!(don_aci(5.0) > 0.5); }
}
