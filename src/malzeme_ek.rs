//! Dalga 3 ek paketi (KARAR-ANIM Ö1 açık işleri + Ö2/Ö3): maske tabanlı premium yazılar (uzun gölge, kabartma, ağır süpürme,
//! yansıma, metal/ahşap/tarama dokusu, dikiş, bayrak dalgası, karlı yazı, buzlu cam), alan efektleri (aurora, sis, duman,
//! sıvı metal; 1/4 çözünürlük + 4x4 Bayer), gerçek 3B düz gölgeli nesneler (dönen küp, noel yıldızı prizması), kış/yılbaşı
//! ek sahneleri ve premium sahneler. Kurallar: ışıma/neon/bulanıklık yok; 3 ton, sert gölge, bantlı ton; kapalı biçimli ve
//! `efekt::hash` ile deterministik; ortam sahnelerinde fazlar süreye tam bölünür (döngü güvenli), döngüsüzler katalogda yazılı.
//! Sahip: Yusuf. Yazan: [Claude#Malzeme Opus 5.5, yenileme-1.0, 2026-11-25]. Yalnız yazan yapay zekâ ya da Yusuf'un izniyle değişir; silme yok.
use crate::anim::{self, Maske, Poz};
use crate::efekt::hash;
use crate::malzeme::MalzemeAyar;
use crate::malzeme_ciz::Tuval;
use crate::malzeme_kis::{gurultu, hediye, kar, kar_katmanli, kar_ortusu, kos, piksel_3b};
use crate::malzeme_parca::{self as parca, Tur};
use crate::senaryo::Hazir;
use crate::Renk;

const PI: f32 = std::f32::consts::PI;
const TAU: f32 = std::f32::consts::TAU;
const KOYU: Renk = Renk(14, 16, 24);
const BEYAZ: Renk = Renk(255, 255, 255);
const KREM: Renk = Renk(255, 250, 238);
const ALTIN: Renk = Renk(255, 200, 70);
const BAYER: [[f32; 4]; 4] = [[0.0, 8.0, 2.0, 10.0], [12.0, 4.0, 14.0, 6.0], [3.0, 11.0, 1.0, 9.0], [15.0, 7.0, 13.0, 5.0]];

fn fr(x: f32) -> f32 { x - x.floor() }
fn bay(x: u32, y: u32) -> f32 { (BAYER[(y & 3) as usize][(x & 3) as usize] + 0.5) / 16.0 }
/// Döngü güvenli açısal hız: yaklaşık `hedef` rad/sn, ama `sure` içinde tam tur sayısı.
fn dw(sure: f32, hedef: f32) -> f32 { let k = (hedef * sure / TAU).round().max(1.0); TAU * k / sure.max(0.5) }

/// Dönemli 2B değer gürültüsü (x dönemi `px`, y dönemi `py` hücre; 0 = dönemsiz), 0..1.
fn gurultu2(x: f32, y: f32, tohum: u32, px: i32, py: i32) -> f32 {
    let (xi, yi) = (x.floor() as i32, y.floor() as i32);
    let (fx, fy) = (x - xi as f32, y - yi as f32);
    let h = |a: i32, b: i32| {
        let a = if px > 0 { a.rem_euclid(px) } else { a };
        let b = if py > 0 { b.rem_euclid(py) } else { b };
        hash(tohum.wrapping_mul(2_654_435_761) ^ (a as u32).wrapping_mul(73_856_093) ^ (b as u32).wrapping_mul(19_349_663))
    };
    let (sx, sy) = (fx * fx * (3.0 - 2.0 * fx), fy * fy * (3.0 - 2.0 * fy));
    let ust = h(xi, yi) + (h(xi + 1, yi) - h(xi, yi)) * sx;
    let alt = h(xi, yi + 1) + (h(xi + 1, yi + 1) - h(xi, yi + 1)) * sx;
    ust + (alt - ust) * sy
}

/// Alan efekti (KARAR-ANIM 1. bölüm): `f` 1/4 çözünürlükte (değer 0..1, alfa 0..1) döner; tam kareye en yakın komşu ve
/// 4x4 Bayer ile `seviye` basamağa nicemlenerek `rampa` renkleriyle yazılır. Tam karede piksel başına sin/sqrt yok.
fn alan(tv: &mut Tuval, seviye: usize, rampa: &dyn Fn(usize) -> Renk, f: &dyn Fn(f32, f32) -> (f32, f32)) {
    let (w, h) = (tv.w, tv.h);
    let (gw, gh) = (w.div_ceil(4), h.div_ceil(4));
    let mut g = vec![(0f32, 0f32); (gw * gh) as usize];
    for gy in 0..gh { for gx in 0..gw { g[(gy * gw + gx) as usize] = f(gx as f32 * 4.0 + 2.0, gy as f32 * 4.0 + 2.0); } }
    let seviye = seviye.max(2);
    let renk: Vec<Renk> = (0..seviye).map(rampa).collect();
    for y in 0..h {
        let sat = (y / 4) * gw;
        for x in 0..w {
            let (v, a) = g[(sat + x / 4) as usize];
            if a <= 0.004 { continue; }
            let q = ((v.clamp(0.0, 0.9999) * (seviye - 1) as f32) + bay(x, y)).floor() as usize;
            tv.px(x as i32, y as i32, renk[q.min(seviye - 1)], a);
        }
    }
}

/// Bantlı dikey zemin (4 ton, sınırlar dama dither'li).
fn bantli(tv: &mut Tuval, ust: Renk, alt: Renk) {
    let (wf, hf) = (tv.w as f32, tv.h as f32);
    let bh = hf / 4.0;
    for i in 0..4 {
        let r = kar(ust, alt, i as f32 / 3.0);
        let a = i as f32 * bh;
        tv.dikd(0.0, a, wf, a + bh + 1.0, r, 1.0);
        if i < 3 { let r2 = kar(ust, alt, (i + 1) as f32 / 3.0); let mut x = 0.0; while x < wf { tv.dikd(x, a + bh - 4.0, x + 2.0, a + bh - 2.0, r2, 1.0); tv.dikd(x + 2.0, a + bh - 2.0, x + 4.0, a + bh, r2, 1.0); x += 4.0; } }
    }
}

// ---------------------------------------------------------------- maske yardımcıları
#[inline]
fn ma(m: &Maske, x: i32, y: i32) -> f32 { if x < 0 || y < 0 || x >= m.w as i32 || y >= m.h as i32 { 0.0 } else { m.a[(y as u32 * m.w + x as u32) as usize] } }
fn kutu(m: &Maske) -> (i32, i32, i32, i32) { (m.bx.0 as i32, m.bx.1 as i32, m.bx.2 as i32, m.bx.3 as i32) }
/// Maskenin merkezini (cx, cy)'ye taşıyan tam sayı kayma.
fn ofs(m: &Maske, cx: f32, cy: f32) -> (i32, i32) { ((cx - (m.bx.0 + m.bx.2) as f32 / 2.0).round() as i32, (cy - (m.bx.1 + m.bx.3) as f32 / 2.0).round() as i32) }
fn bos(m: &Maske) -> bool { m.bx.2 <= m.bx.0 }

/// Maske yüzünü piksel piksel boyar; `renk(x, y, a)` maske koordinatında.
fn yuz_boya(tv: &mut Tuval, m: &Maske, ox: i32, oy: i32, renk: &dyn Fn(i32, i32, f32) -> Option<Renk>) {
    if bos(m) { return; }
    let (x0, y0, x1, y1) = kutu(m);
    for y in y0..=y1 { for x in x0..=x1 {
        let a = ma(m, x, y);
        if a <= 0.01 { continue; }
        if let Some(r) = renk(x, y, a) { tv.px(x + ox, y + oy, r, a); }
    } }
}

/// Kalın koyu dış çizgi (LED okunurluğu): maskenin dışında, `k` px içinde maske olan pikseller.
fn kontur(tv: &mut Tuval, m: &Maske, ox: i32, oy: i32, k: i32, renk: Renk) {
    if bos(m) { return; }
    let (x0, y0, x1, y1) = kutu(m);
    let d = (k as f32 * 0.7) as i32;
    for y in y0 - k..=y1 + k { for x in x0 - k..=x1 + k {
        if ma(m, x, y) > 0.5 { continue; }
        let v = [(k, 0), (-k, 0), (0, k), (0, -k), (d, d), (-d, d), (d, -d), (-d, -d)].iter().map(|&(a, b)| ma(m, x + a, y + b)).fold(0.0f32, f32::max);
        if v > 0.3 { tv.px(x + ox, y + oy, renk, v.min(1.0)); }
    } }
}

/// Uzun gölge (Y5): maskenin 45° sağ alta `l` px kopyası tek koyu tonla, satır satır çapraz sayaçla (O(alan)). Bulanıksız, kesik uçlu.
fn uzun_golge(tv: &mut Tuval, m: &Maske, ox: i32, oy: i32, l: i32, renk: Renk) {
    if bos(m) || l <= 0 { return; }
    let (x0, y0, x1, y1) = kutu(m);
    let ww = (x1 - x0 + 1 + l) as usize;
    let mut kal = vec![0i32; ww + 1];
    let mut yeni = vec![0i32; ww + 1];
    let uzak = kos(renk, 0.8);
    for y in y0..=y1 + l {
        for xi in 0..ww {
            let x = x0 + xi as i32;
            let once = kal[xi];
            if ma(m, x, y) > 0.5 { yeni[xi + 1] = l; } else {
                yeni[xi + 1] = (once - 1).max(0);
                if once > 0 { tv.px(x + ox, y + oy, if once > l / 2 { renk } else { uzak }, 1.0); }
            }
        }
        std::mem::swap(&mut kal, &mut yeni);
    }
}

/// 3B ekstrüde yazı (anim::kare3d), maske merkezi (cx, cy).
fn y3(tv: &mut Tuval, z: &Hazir, yuz: Renk, yan: Renk, cx: f32, cy: f32, olcek: f32, aci: f32, derin: f32, parlak: f32) {
    if let Some(m) = z.maskeler.first() {
        let (mx, my) = ((m.bx.0 + m.bx.2) as f32 / 2.0, (m.bx.1 + m.bx.3) as f32 / 2.0);
        anim::kare3d_uzerine(tv.t, m, yuz, yan, &Poz { aci, olcek: olcek.max(0.02), parlak, derinlik: derin, dx: cx - mx, dy: cy - my });
    }
}

/// Maske yoksa (boş metin) piksel 3B yazı yedeği.
fn yazi_ya(tv: &mut Tuval, z: &Hazir, s: &str, cx: f32, cy: f32, u: f32) {
    if z.maskeler.is_empty() && !s.is_empty() { piksel_3b(tv, s, cx, cy, tv.w as f32 * 0.86, 30.0 * u, KREM); }
}

// ---------------------------------------------------------------- 3B nesneler (Lambert 4 seviye, ressam sıralaması)
type V3 = (f32, f32, f32);
fn don3(p: V3, ay: f32, ax: f32) -> V3 {
    let (x, y, z) = p;
    let (x1, z1) = (x * ay.cos() + z * ay.sin(), -x * ay.sin() + z * ay.cos());
    (x1, y * ax.cos() - z1 * ax.sin(), y * ax.sin() + z1 * ax.cos())
}
fn izdus(p: V3, cx: f32, cy: f32, f: f32) -> (f32, f32) { let k = f / (f + p.2); (cx + p.0 * k, cy + p.1 * k) }
/// Yüz listesi (köşe indeksleri + renk) çizer; yüz normali ile sol üst-ön ışık, 4 seviyeli düz gölge, arka yüzler atlanır.
fn nesne(tv: &mut Tuval, v: &[V3], yuzler: &[(Vec<usize>, Renk)], ay: f32, ax: f32, cx: f32, cy: f32, f: f32, cizgi: f32) {
    let d: Vec<V3> = v.iter().map(|&p| don3(p, ay, ax)).collect();
    let isik = { let (a, b, c) = (-0.45f32, -0.6f32, -0.66f32); let l = (a * a + b * b + c * c).sqrt(); (a / l, b / l, c / l) };
    let mut sira: Vec<(f32, usize)> = yuzler.iter().enumerate().map(|(i, (ix, _))| (ix.iter().map(|&k| d[k].2).sum::<f32>() / ix.len() as f32, i)).collect();
    sira.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
    for (_, i) in sira {
        let (ix, renk) = &yuzler[i];
        if ix.len() < 3 { continue; }
        let (a, b, c) = (d[ix[0]], d[ix[1]], d[ix[2]]);
        let (u1, u2) = ((b.0 - a.0, b.1 - a.1, b.2 - a.2), (c.0 - a.0, c.1 - a.1, c.2 - a.2));
        let n = (u1.1 * u2.2 - u1.2 * u2.1, u1.2 * u2.0 - u1.0 * u2.2, u1.0 * u2.1 - u1.1 * u2.0);
        let nl = (n.0 * n.0 + n.1 * n.1 + n.2 * n.2).sqrt().max(1e-6);
        let mut n = (n.0 / nl, n.1 / nl, n.2 / nl);
        // sarım yönünden bağımsız: normal kameraya (−z) çevrilir; görünürlük ressam sıralamasıyla
        if n.2 > 0.0 { n = (-n.0, -n.1, -n.2); }
        let lam = (n.0 * isik.0 + n.1 * isik.1 + n.2 * isik.2).max(0.0);
        let sev = ((0.35 + 0.65 * lam) * 4.0).floor().min(3.0) / 3.0;
        let ton = kar(kos(*renk, 0.55), kar(*renk, BEYAZ, 0.3), sev);
        let p: Vec<(f32, f32)> = ix.iter().map(|&k| izdus(d[k], cx, cy, f)).collect();
        tv.cokgen(&p, ton, 1.0);
        if cizgi > 0.0 { for j in 0..p.len() { let q = p[(j + 1) % p.len()]; tv.cizgi(p[j].0, p[j].1, q.0, q.1, cizgi, KOYU, 1.0); } }
    }
}
fn kup(r: f32) -> Vec<V3> { let mut v = vec![]; for &x in &[-r, r] { for &y in &[-r, r] { for &z in &[-r, r] { v.push((x, y, z)); } } } v }
/// Küp yüzleri (dışa dönük, saat yönü tutarlı).
fn kup_yuz(r: [Renk; 6]) -> Vec<(Vec<usize>, Renk)> {
    // indeks: x*4 + y*2 + z  (0 = eksi, 1 = artı)
    vec![(vec![0, 2, 3, 1], r[0]), (vec![4, 5, 7, 6], r[1]), (vec![0, 1, 5, 4], r[2]), (vec![2, 6, 7, 3], r[3]), (vec![0, 4, 6, 2], r[4]), (vec![1, 3, 7, 5], r[5])]
}
/// Yıldız prizması: ön/arka 10 köşeli yıldız + yan dörtgenler.
fn yildiz_prizma(r: f32, ic: f32, d: f32, renk: Renk) -> (Vec<V3>, Vec<(Vec<usize>, Renk)>) {
    let mut v = vec![];
    for zz in [-d, d] { for i in 0..10 { let a = -PI / 2.0 + i as f32 * PI / 5.0; let rr = if i % 2 == 0 { r } else { r * ic }; v.push((a.cos() * rr, a.sin() * rr, zz)); } }
    let mut y = vec![];
    // ön (z=-d, kameraya yakın) ve arka; ön yüz çokgen olarak tek parça (içbükey: cokgen tarama çiftlik kuralıyla çizer)
    y.push(((0..10).collect::<Vec<_>>(), renk));
    y.push(((10..20).rev().collect::<Vec<_>>(), renk));
    for i in 0..10 { let j = (i + 1) % 10; y.push((vec![i, i + 10, j + 10, j], kos(renk, 0.9))); }
    (v, y)
}

// ---------------------------------------------------------------- kış yardımcıları
/// Buz köşeleri (efekt `buz-koseler`): dört köşeden içe dallanan kırağı kolları; büyüme `k` 0..1.
pub fn buz_koseler(tv: &mut Tuval, x0: f32, y0: f32, x1: f32, y1: f32, k: f32, u: f32) {
    let (acik, koyu) = (Renk(225, 240, 255), Renk(120, 160, 210));
    for (kx, ky, sx, sy) in [(x0, y0, 1.0f32, 1.0f32), (x1, y0, -1.0, 1.0), (x0, y1, 1.0, -1.0), (x1, y1, -1.0, -1.0)] {
        for kol in 0..5 {
            let a = (kol as f32 + 0.5) / 5.0 * PI / 2.0;
            let l = (90.0 + 70.0 * hash(kol * 17 + (kx as u32) % 97)) * u * k;
            let (dx, dy) = (a.cos() * sx, a.sin() * sy);
            for (kal, r) in [(7.0f32, koyu), (3.0, acik)] {
                tv.cizgi(kx, ky, kx + dx * l, ky + dy * l, kal * u, r, 1.0);
                for d in 1..4 {
                    let p = d as f32 / 4.0;
                    let (bx, by) = (kx + dx * l * p, ky + dy * l * p);
                    let dl = l * (0.42 - 0.1 * d as f32);
                    for s in [-1.0f32, 1.0] { let b = a + s * PI / 3.0; tv.cizgi(bx, by, bx + b.cos() * sx * dl, by + b.sin() * sy * dl, kal * 0.7 * u, r, 1.0); }
                }
            }
        }
    }
}

fn cam_siluet(tv: &mut Tuval, cx: f32, taban: f32, boy: f32, renk: Renk) {
    for k in 0..4 { let f = k as f32 / 4.0; let gen = boy * (0.42 - f * 0.08); let y = taban - boy * f * 0.75; tv.cokgen(&[(cx - gen / 2.0, y), (cx + gen / 2.0, y), (cx, y - boy * 0.45)], renk, 1.0); }
}

const SUS: [Renk; 5] = [Renk(230, 40, 50), Renk(255, 200, 40), Renk(60, 160, 255), Renk(200, 80, 220), Renk(40, 190, 110)];

// 16x16 piksel kış sprite'ları (P5): K koyu, A ana, B açık, R kırmızı, Y yeşil, S sarı, T turuncu
const SP_KARDAN: [&str; 16] = ["......KKKK......", ".....KKKKKK.....", ".....KKKKKK.....", "....KKKKKKKK....", ".....BBBBBB.....", "....BBKBBKBB....", "....BBBBTTBB....", ".....BBBBBB.....", "....RRRRRRRR....", "...BBBRRBBBBB...", "..BBBBBKBBBBBB..", "..BBBBBBBBBBBB..", "..BBBBBKBBBBBB..", "..BBBBBBBBBBBB..", "...BBBBBBBBBB...", ".....BBBBBB....."];
const SP_CAM: [&str; 16] = [".......SS.......", "......SSSS......", ".......YY.......", "......YYYY......", ".....YYRYYY.....", "....YYYYYYYY....", ".....YYYYSY.....", "....YYSYYYYY....", "...YYYYYYYRYY...", "....YYYYYYYY....", "...YYRYYYYSYY...", "..YYYYYYYYYYYY..", ".YYYYSYYYYRYYYY.", "......KKKK......", "......KKKK......", "....KKKKKKKK...."];
const SP_HEDIYE: [&str; 16] = ["................", "....SS...SS.....", "...S..S.S..S....", "....SS.S.SS.....", ".......S........", "..RRRRRSRRRRRR..", "..RRRRRSRRRRRR..", "..SSSSSSSSSSSS..", "..RRRRRSRRRRRR..", "..RRRRRSRRRRRR..", "..RRRRRSRRRRRR..", "..RRRRRSRRRRRR..", "..RRRRRSRRRRRR..", "..RRRRRSRRRRRR..", "..KKKKKKKKKKKK..", "................"];
const SP_TANE_A: [&str; 16] = ["................", ".......B........", "....B..B..B.....", ".....B.B.B......", "......BBB.......", "..BBBBBBBBBBB...", "......BBB.......", ".....B.B.B......", "....B..B..B.....", ".......B........", "................", "................", "................", "................", "................", "................"];
const SP_TANE_B: [&str; 16] = ["................", "....B.....B.....", ".....B...B......", "......B.B.......", ".......B........", "..B..BBBBB..B...", ".......B........", "......B.B.......", ".....B...B......", "....B.....B.....", "................", "................", "................", "................", "................", "................"];

fn sp_renk(c: char, ana: Renk) -> Option<Renk> {
    match c { 'K' => Some(Renk(40, 40, 50)), 'A' => Some(ana), 'B' => Some(Renk(235, 242, 255)), 'R' => Some(Renk(220, 40, 50)), 'Y' => Some(Renk(40, 160, 80)), 'S' => Some(Renk(255, 205, 50)), 'T' => Some(Renk(255, 130, 30)), _ => None }
}

// ---------------------------------------------------------------- sahne yönlendirici
/// TTF maskeli yazı kullanan kalemler için yazı kutusu (950 genişliğe göre px); malzeme_ciz::yazi_kutusu'ndan çağrılır.
pub fn yazi_kutusu(id: &str) -> Option<(f32, f32)> {
    Some(match id {
        "yazi-uzun-golge" | "yazi-kabartma" | "yazi-supurme" | "yazi-metal" | "yazi-ahsap" | "yazi-tarama" | "yazi-bayrak" => (780.0, 520.0),
        "yazi-yansima" => (800.0, 380.0), "dikis-yazi" => (760.0, 460.0), "kis-suslu-yazi" => (780.0, 400.0), "buzlu-cam-yazi" => (660.0, 380.0),
        "aurora" | "fisek-yazi" | "sis" | "duman" | "spot-isik" | "derin-tunel" | "yildiz-alani" | "sahne-perde" | "parcacik-girdap" | "kumas-dalga" => (780.0, 400.0),
        "sivi-metal" | "cam-panel" => (760.0, 420.0),
        _ => return None,
    })
}

pub fn ciz(m: &MalzemeAyar, yazi_s: &str, z: &Hazir, tv: &mut Tuval, yerel: f32, sure: f32) -> bool {
    let (wf, hf) = (tv.w as f32, tv.h as f32);
    let u = (wf / 950.0).min(hf / 1800.0);
    let (cx, cy) = (wf / 2.0, hf / 2.0);
    let tt = yerel * m.hiz;
    let sz = (sure * m.hiz).max(1.0);
    let (yuz, vu, zem) = (z.yuz, z.vurgu, z.zemin);
    let acik = kar(yuz, BEYAZ, 0.25);
    let mk = z.maskeler.first();
    let dv = m.deger;
    match m.id.as_str() {
        // ------------------------------------------------ maskeli premium yazılar (Y5–Y7, Y10, Y12, Y13)
        "yazi-uzun-golge" => {
            bantli(tv, kos(zem, 0.7), kos(vu, 0.35));
            if let Some(mm) = mk {
                let (ox, oy) = ofs(mm, cx, cy);
                let l = ((40.0 + 50.0 * anim::yumusak(tt / 1.2)) * u) as i32;
                uzun_golge(tv, mm, ox, oy, l, kos(vu, 0.42));
                let k = (5.0 * u).max(1.0) as i32;
                yuz_boya(tv, mm, ox, oy, &|x, y, _| Some(if ma(mm, x - k, y - k) < 0.5 { kar(acik, BEYAZ, 0.4) } else { acik }));
            } else { yazi_ya(tv, z, yazi_s, cx, cy, u); }
        }
        "yazi-kabartma" => {
            bantli(tv, kos(zem, 0.6), kos(yuz, 0.3));
            if let Some(mm) = mk {
                let aci = (tt * 0.5).sin() * 0.06;
                y3(tv, z, yuz, kos(yuz, 0.4), cx, cy, 1.0, aci, 14.0 * u, 1.0);
                let (ox, oy) = ofs(mm, cx, cy);
                let k = (3.0 * u).max(2.0) as i32;
                let (isik, golge) = (kar(yuz, BEYAZ, 0.45), kos(yuz, 0.55));
                if aci.abs() < 0.08 { yuz_boya(tv, mm, ox, oy, &|x, y, _| if ma(mm, x - k, y - k) < 0.5 { Some(isik) } else if ma(mm, x + k, y + k) < 0.5 { Some(golge) } else { None }); }
            } else { yazi_ya(tv, z, yazi_s, cx, cy, u); }
        }
        "yazi-supurme" => {
            bantli(tv, Renk(8, 8, 12), kos(vu, 0.3));
            if let Some(mm) = mk {
                y3(tv, z, yuz, kos(vu, 0.55), cx, cy, 1.0, 0.0, 12.0 * u, 1.0);
                let (ox, oy) = ofs(mm, cx, cy);
                let top = wf + hf * 0.36 + 600.0 * u;
                let c = fr(tt / 3.5) * top - 300.0 * u;
                yuz_boya(tv, mm, ox, oy, &|x, y, _| {
                    let s = (x + ox) as f32 + (y + oy) as f32 * 0.36;
                    if (s - c).abs() < 45.0 * u { Some(kar(yuz, BEYAZ, 0.28)) } else if (s - c - 110.0 * u).abs() < 9.0 * u { Some(kar(yuz, BEYAZ, 0.45)) } else { None }
                });
            } else { yazi_ya(tv, z, yazi_s, cx, cy, u); }
        }
        "yazi-yansima" => {
            bantli(tv, Renk(6, 8, 14), kos(vu, 0.2));
            if let Some(mm) = mk {
                let (_, by0, _, by1) = kutu(mm);
                let (ox, oy) = ofs(mm, cx, hf * 0.4);
                let taban = by1 + oy + (6.0 * u) as i32;
                tv.dikd(0.0, taban as f32 - 2.0 * u, wf, taban as f32 + 2.0 * u, kos(vu, 0.6), 1.0);
                let bh = (by1 - by0).max(1) as f32;
                let rh = (bh * 0.55) as i32;
                let (x0, _, x1, _) = kutu(mm);
                let ac_r = kos(acik, 0.8);
                for r in 0..rh {
                    if r % 3 == 2 { continue; }
                    let al = 0.42 * (1.0 - r as f32 / rh as f32);
                    let sy = by1 - (r as f32 / 0.55) as i32;
                    let kay = (3.0 * u * ((taban + r) as f32 / 9.0 + tt * 2.0).sin()).round() as i32;
                    for x in x0..=x1 { let a = ma(mm, x - kay, sy); if a > 0.01 { tv.px(x + ox, taban + 4 + r, ac_r, a * al); } }
                }
                y3(tv, z, acik, kos(vu, 0.6), cx, hf * 0.4, 1.0, 0.0, 12.0 * u, 1.0);
            } else { yazi_ya(tv, z, yazi_s, cx, cy, u); }
        }
        "yazi-metal" | "yazi-ahsap" | "yazi-tarama" => {
            bantli(tv, Renk(10, 10, 14), kos(zem, 0.6));
            if let Some(mm) = mk {
                let aci = (tt * 0.45).sin() * 0.05;
                let taban = match m.id.as_str() { "yazi-metal" => Renk(196, 200, 210), "yazi-ahsap" => Renk(170, 110, 60), _ => yuz };
                y3(tv, z, taban, kos(taban, 0.38), cx, cy, 1.0, aci, 12.0 * u, 1.0);
                let (ox, oy) = ofs(mm, cx, cy);
                let (_, by0, _, by1) = kutu(mm);
                let id = m.id.clone();
                let isik_y = by0 as f32 + (by1 - by0) as f32 * (0.25 + 0.1 * (tt * 0.7).sin());
                yuz_boya(tv, mm, ox, oy, &|x, y, _| Some(match id.as_str() {
                    "yazi-metal" => {
                        let f = 0.9 + 0.2 * hash(y as u32 * 7 + 3) + if (y as f32 - isik_y).abs() < 10.0 * u { 0.22 } else { 0.0 };
                        kos(taban, f.min(1.3))
                    }
                    "yazi-ahsap" => {
                        let v = ((y as f32 + 18.0 * u * gurultu(x as f32 / (80.0 * u), 3) + 6.0 * u * gurultu(x as f32 / (23.0 * u), 9)) / (9.0 * u)).sin();
                        let q = ((v * 0.5 + 0.5) * 3.0).floor().min(2.0) / 2.0;
                        kar(Renk(120, 70, 32), Renk(214, 150, 88), q)
                    }
                    _ => if (x + y).rem_euclid((12.0 * u).max(4.0) as i32) < (4.0 * u).max(2.0) as i32 { kos(taban, 0.7) } else { taban },
                }));
            } else { yazi_ya(tv, z, yazi_s, cx, cy, u); }
        }
        "dikis-yazi" => {
            // kumaş zemin: çapraz tarama dokusu
            let kumas = kar(vu, KOYU, 0.68);
            tv.dikd(0.0, 0.0, wf, hf, kumas, 1.0);
            let koyu_k = kos(kumas, 0.6);
            let p = (8.0 * u).max(4.0) as i32;
            for y in 0..tv.h as i32 { for x in 0..tv.w as i32 { if (x + y).rem_euclid(p) == 0 || (x - y).rem_euclid(p) == 0 { tv.px(x, y, koyu_k, 1.0); } } }
            if let Some(mm) = mk {
                let (ox, oy) = ofs(mm, cx, cy);
                kontur(tv, mm, ox + (5.0 * u) as i32, oy + (5.0 * u) as i32, (3.0 * u).max(1.0) as i32, kos(zem, 0.5));
                yuz_boya(tv, mm, ox + (5.0 * u) as i32, oy + (5.0 * u) as i32, &|_, _, _| Some(kos(zem, 0.5)));
                yuz_boya(tv, mm, ox, oy, &|_, _, _| Some(yuz));
                let (x0, _, x1, _) = kutu(mm);
                let ac = x0 as f32 + (x1 - x0) as f32 * (tt / (sz * 0.6).max(1.0)).min(1.0);
                let (i8, i12) = ((8.0 * u).max(3.0) as i32, (12.0 * u).max(5.0) as i32);
                let ic = |x: i32, y: i32, r: i32| { let d = (r as f32 * 0.7) as i32; [(r, 0), (-r, 0), (0, r), (0, -r), (d, d), (-d, -d), (d, -d), (-d, d)].iter().all(|&(a, b)| ma(mm, x + a, y + b) > 0.5) };
                // dikiş ipi yüzle zıt tonda (açık yüzde koyu ip): LED'de okunur
                let ip = if yuz.0 as u32 + yuz.1 as u32 + yuz.2 as u32 > 380 { Renk(40, 24, 60) } else { Renk(250, 246, 236) };
                let des = (15.0 * u).max(6.0) as i32;
                let dol = (9.0 * u).max(4.0) as i32;
                yuz_boya(tv, mm, ox, oy, &|x, y, _| if (x as f32) < ac && ic(x, y, i8) && !ic(x, y, i12) && (x + y / 2).rem_euclid(des) < dol { Some(ip) } else { None });
                if ac < x1 as f32 { let ny = (mm.bx.1 + mm.bx.3) as f32 / 2.0 + oy as f32; tv.cizgi(ac + ox as f32, ny - 50.0 * u, ac + ox as f32 + 20.0 * u, ny + 30.0 * u, 4.0 * u, Renk(210, 214, 222), 1.0); tv.daire(ac + ox as f32, ny - 50.0 * u, 4.0 * u, KOYU, 1.0); }
            } else { yazi_ya(tv, z, yazi_s, cx, cy, u); }
        }
        "yazi-bayrak" => {
            bantli(tv, Renk(8, 10, 18), kos(vu, 0.25));
            if let Some(mm) = mk {
                let (ox, oy) = ofs(mm, cx, cy);
                let w_ = dw(sz, 2.2);
                let (x0, y0, x1, y1) = kutu(mm);
                let l = 160.0 * u;
                let amp = 26.0 * u;
                let dyx = |x: i32| amp * ((x as f32) / l - w_ * tt).sin();
                let ton = |x: i32| { let c = ((x as f32) / l - w_ * tt).cos(); 0.72 + 0.28 * ((c * 0.5 + 0.5) * 4.0).floor() / 4.0 };
                let gol = kos(vu, 0.35);
                for x in x0..=x1 {
                    let d = dyx(x).round() as i32;
                    let tk = ton(x);
                    let f = kos(acik, tk);
                    for y in y0 - (amp as i32 + 2)..=y1 + amp as i32 + 2 {
                        let a = ma(mm, x, y - d);
                        if a > 0.01 { tv.px(x + ox + (8.0 * u) as i32, y + oy + (8.0 * u) as i32, gol, a); }
                    }
                    for y in y0 - (amp as i32 + 2)..=y1 + amp as i32 + 2 {
                        let a = ma(mm, x, y - d);
                        if a > 0.01 { tv.px(x + ox, y + oy, f, a); }
                    }
                }
            } else { yazi_ya(tv, z, yazi_s, cx, cy, u); }
        }
        "kinetik-yazi" => {
            let kel: Vec<&str> = yazi_s.split(|c: char| c == '|' || c.is_whitespace()).filter(|s| !s.is_empty()).collect();
            let kel = if kel.is_empty() { vec!["TAA"] } else { kel };
            let n = kel.len();
            let dwk = 0.9f32;
            let i = ((tt / dwk).floor() as usize) % n;
            let yk = fr(tt / dwk);
            // LED: zeminler hep koyu (kare ortalaması ≤ %45), yazı açık
            let tonlar = [(kos(vu, 0.32), KREM), (kos(yuz, 0.2), kar(vu, BEYAZ, 0.35)), (Renk(14, 14, 20), acik), (kos(vu, 0.16), kar(yuz, BEYAZ, 0.3))];
            let (z1, y1c) = tonlar[i % 4];
            let (z0, _) = tonlar[(i + 3) % 4];
            tv.dikd(0.0, 0.0, wf, hf, z0, 1.0);
            // şerit silme geçişi (flaş yok): yeni zemin 0,18 sn'de yukarıdan aşağı açılır
            let sil = (yk / 0.2).min(1.0);
            tv.dikd(0.0, 0.0, wf, hf * anim::yumusak(sil), z1, 1.0);
            let s = kel[i];
            let sab = 70.0 * u;
            match i % 4 {
                0 => { let ol = 1.0 + 0.6 * (1.0 - anim::geri((yk / 0.35).min(1.0))); let sars = if yk < 0.4 { (yk * 60.0).sin() * 8.0 * u * (1.0 - yk / 0.4) } else { 0.0 }; piksel_3b(tv, s, cx + sars, cy, wf * 0.88 * ol, sab * ol, y1c); }
                1 => {
                    // dikey dizilmiş harfler sol kenardan kayar
                    let x = -200.0 * u + (cx + 200.0 * u) * anim::geri((yk / 0.4).min(1.0));
                    let nc = s.chars().count().max(1) as f32;
                    let hh = (hf * 0.7 / (nc * 8.0)).min(sab);
                    for (j, c) in s.chars().enumerate() { piksel_3b(tv, &c.to_string(), x, cy - hf * 0.35 + (j as f32 + 0.5) * hh * 8.0, wf, hh, y1c); }
                }
                2 => {
                    let ch: Vec<char> = s.chars().collect();
                    let yari = ch.len().div_ceil(2);
                    let (a, b): (String, String) = (ch[..yari].iter().collect(), ch[yari..].iter().collect());
                    let k = anim::geri((yk / 0.4).min(1.0));
                    piksel_3b(tv, &a, -wf * 0.5 + (cx + wf * 0.5) * k, cy - 90.0 * u, wf * 0.86, sab, y1c);
                    if !b.is_empty() { piksel_3b(tv, &b, wf * 1.5 - (wf * 1.5 - cx) * k, cy + 90.0 * u, wf * 0.86, sab, y1c); }
                }
                _ => {
                    let nc = s.chars().count().max(1);
                    let hh = (wf * 0.86 / (nc as f32 * 6.0 - 1.0)).min(sab);
                    let x0 = cx - (nc as f32 * 6.0 - 1.0) * hh / 2.0;
                    for (j, c) in s.chars().enumerate() {
                        let k = anim::geri(((yk - j as f32 * 0.05) / 0.3).clamp(0.0, 1.0));
                        if yk - j as f32 * 0.05 <= 0.0 { continue; }
                        piksel_3b(tv, &c.to_string(), x0 + j as f32 * 6.0 * hh + 2.5 * hh, cy + (1.0 - k) * 120.0 * u, wf, hh * (0.6 + 0.4 * k), y1c);
                    }
                }
            }
        }
        // ------------------------------------------------ kış ekleri
        "kis-suslu-yazi" => {
            bantli(tv, Renk(6, 12, 30), kos(yuz, 0.3));
            kar_katmanli(tv, tt, sz, 140, 1.0, 0b011, 41);
            if let Some(mm) = mk {
                y3(tv, z, acik, kos(vu, 0.55), cx, cy, 1.0, 0.0, 12.0 * u, 1.0);
                let (ox, oy) = ofs(mm, cx, cy);
                let (x0, y0, x1, y1) = kutu(mm);
                let gn = (x1 - x0 + 1) as usize;
                let ust: Vec<i32> = (x0..=x1).map(|x| (y0..=y1).find(|&y| ma(mm, x, y) > 0.5).unwrap_or(i32::MAX)).collect();
                let alt: Vec<i32> = (x0..=x1).map(|x| (y0..=y1).rev().find(|&y| ma(mm, x, y) > 0.5).unwrap_or(i32::MIN)).collect();
                let buy = anim::yumusak(tt / 2.0);
                let pen = (4.0 * u).max(2.0) as usize;
                for i in 0..gn {
                    let (a, b) = (i.saturating_sub(pen), (i + pen).min(gn - 1));
                    let v: Vec<i32> = (a..=b).map(|j| ust[j]).filter(|&y| y != i32::MAX).collect();
                    if v.is_empty() || ust[i] == i32::MAX { continue; }
                    let yy = v.iter().sum::<i32>() as f32 / v.len() as f32;
                    let x = x0 + i as i32;
                    let kal = (10.0 + dv.clamp(4.0, 30.0) * 0.5 + 8.0 * gurultu(x as f32 / (26.0 * u), 7)) * u * buy;
                    let (ya, yb) = (yy - kal, yy + 3.0 * u);
                    tv.dikd((x + ox) as f32, ya + oy as f32, (x + ox + 1) as f32, yb + oy as f32, Renk(236, 242, 252), 1.0);
                    tv.dikd((x + ox) as f32, ya + oy as f32, (x + ox + 1) as f32, ya + oy as f32 + 3.0 * u, BEYAZ, 1.0);
                    tv.dikd((x + ox) as f32, yb + oy as f32 - 3.0 * u, (x + ox + 1) as f32, yb + oy as f32, Renk(160, 180, 212), 1.0);
                }
                // 3 buz sarkıtı: harflerin alt kenarından
                for s in 0..3u32 {
                    let i = ((0.15 + 0.7 * hash(s * 29 + 3)) * gn as f32) as usize;
                    if alt[i.min(gn - 1)] == i32::MIN { continue; }
                    let (x, y) = ((x0 + i as i32 + ox) as f32, (alt[i.min(gn - 1)] + oy) as f32);
                    let l = (30.0 + 30.0 * hash(s * 29 + 4)) * u * buy;
                    tv.cokgen(&[(x - 7.0 * u, y), (x + 7.0 * u, y), (x, y + l)], Renk(200, 225, 250), 1.0);
                    tv.cokgen(&[(x - 7.0 * u, y), (x - 1.0 * u, y), (x - 1.0 * u, y + l * 0.85)], BEYAZ, 0.8);
                }
            } else { yazi_ya(tv, z, yazi_s, cx, cy, u); }
            kar_ortusu(tv, 1.0, 0.06, 9);
            kar_katmanli(tv, tt, sz, 30, 1.0, 0b100, 41);
        }
        "buzlu-cam-yazi" => {
            bantli(tv, Renk(10, 22, 44), kos(yuz, 0.35));
            let (px0, py0, px1, py1) = (wf * 0.08, hf * 0.3, wf * 0.92, hf * 0.7);
            // panel çerçevesi (3 ton)
            tv.dikd(px0 - 18.0 * u, py0 - 18.0 * u, px1 + 18.0 * u, py1 + 18.0 * u, Renk(150, 175, 205), 1.0);
            tv.dikd(px0 - 18.0 * u, py0 - 18.0 * u, px1 + 18.0 * u, py0 - 12.0 * u, Renk(225, 238, 252), 1.0);
            tv.dikd(px0 - 18.0 * u, py1 + 12.0 * u, px1 + 18.0 * u, py1 + 18.0 * u, Renk(80, 100, 130), 1.0);
            tv.dikd(px0, py0, px1, py1, kos(yuz, 0.28), 1.0);
            y3(tv, z, acik, kos(vu, 0.55), cx, cy, 1.0, 0.0, 14.0 * u, 1.0);
            yazi_ya(tv, z, yazi_s, cx, cy, u);
            // kırağı: silinen şerit soldan sağa açılır (döngüsüz)
            let sil = px0 + (px1 - px0 + 200.0 * u) * anim::yumusak(((tt - 0.6) / 2.4).clamp(0.0, 1.0)) - 100.0 * u;
            alan(tv, 4, &|i| kar(Renk(170, 200, 230), Renk(240, 248, 255), i as f32 / 3.0), &|x, y| {
                if x < px0 || x > px1 || y < py0 || y > py1 { return (0.0, 0.0); }
                let v = 0.6 * gurultu2(x / (60.0 * u), y / (60.0 * u), 5, 0, 0) + 0.4 * gurultu2(x / (17.0 * u), y / (17.0 * u), 9, 0, 0);
                let ac = ((x - sil) / (40.0 * u)).clamp(0.0, 1.0);
                (v, 0.18 + 0.62 * ac)
            });
            buz_koseler(tv, px0, py0, px1, py1, 1.0, u);
        }
        "aurora" => {
            bantli(tv, Renk(4, 8, 22), Renk(10, 24, 46));
            let w1 = dw(sz, 0.5);
            let w2 = dw(sz, 0.8);
            let pr = kar(vu, Renk(80, 255, 170), 0.5);
            alan(tv, 6, &|i| kar(Renk(10, 40, 60), kar(pr, BEYAZ, 0.35), i as f32 / 5.0), &|x, y| {
                if y > hf * 0.72 { return (0.0, 0.0); }
                let xx = x + 60.0 * u * (y / (170.0 * u) + w1 * tt).sin() + 30.0 * u * (y / (61.0 * u) - w2 * tt).sin();
                let mut b = 0.0f32;
                for (c, g) in [(0.25f32, 0.07f32), (0.52, 0.09), (0.78, 0.06)] { let d = (xx / wf - c) / g; b += (-d * d).exp(); }
                let dus = (1.0 - ((y / hf - 0.32) / 0.38).abs()).clamp(0.0, 1.0);
                let v = (b * dus).min(1.0);
                (v, if v > 0.06 { 0.85 } else { 0.0 })
            });
            for i in 0..14 { let x = wf * (i as f32 + 0.3 * hash(i * 5 + 1)) / 13.0; cam_siluet(tv, x, hf * 0.86, (260.0 + 160.0 * hash(i * 5 + 2)) * u, Renk(6, 14, 18)); }
            tv.dikd(0.0, hf * 0.86, wf, hf, Renk(205, 220, 240), 1.0);
            tv.dikd(0.0, hf * 0.86, wf, hf * 0.86 + 5.0 * u, BEYAZ, 1.0);
            y3(tv, z, acik, kos(vu, 0.5), cx, hf * 0.22, 1.0, (tt * 0.4).sin() * 0.1, 12.0 * u, 1.0);
            yazi_ya(tv, z, yazi_s, cx, hf * 0.22, u);
        }
        "yilbasi-susleri" => {
            bantli(tv, Renk(30, 6, 12), kos(yuz, 0.3));
            kar_katmanli(tv, tt, sz, 80, 1.0, 0b001, 47);
            let n = (dv.round() as usize).clamp(5, 9);
            let w_ = dw(sz, 2.0);
            for i in 0..n {
                let x = wf * (i as f32 + 0.5) / n as f32;
                let l = (260.0 + 380.0 * hash(i as u32 * 11 + 1)) * u;
                let th = 0.16 * (w_ * tt + i as f32 * 1.3).sin();
                let (bx, by) = (x + th.sin() * l, th.cos() * l);
                tv.cizgi(x, 0.0, bx, by, 3.0 * u, Renk(200, 170, 90), 1.0);
                let r = (54.0 + 26.0 * hash(i as u32 * 11 + 2)) * u;
                let rk = SUS[i % SUS.len()];
                tv.dikd(bx - r * 0.25, by, bx + r * 0.25, by + r * 0.3, Renk(200, 170, 90), 1.0);
                let cy2 = by + r * 0.3 + r;
                tv.daire(bx + r * 0.08, cy2 + r * 0.08, r, kos(rk, 0.5), 1.0);
                tv.daire(bx, cy2, r * 0.96, rk, 1.0);
                tv.dikd(bx - r * 0.9, cy2 - r * 0.14, bx + r * 0.9, cy2 + r * 0.14, kar(rk, BEYAZ, 0.45), 1.0);
                for k in 0..5 { let a = k as f32 / 5.0 * TAU + tt * 0.0; let _ = a; tv.daire(bx - r * 0.6 + k as f32 * r * 0.3, cy2, r * 0.07, kos(rk, 0.55), 1.0); }
                tv.daire(bx - r * 0.38, cy2 - r * 0.42, r * 0.18, kar(rk, BEYAZ, 0.7), 1.0);
            }
            piksel_3b(tv, yazi_s, cx, hf * 0.74, wf * 0.86, 34.0 * u, kar(ALTIN, BEYAZ, 0.3));
        }
        "piksel-kis" => {
            tv.dikd(0.0, 0.0, wf, hf, Renk(10, 16, 40), 1.0);
            let hc = 6.0 * u * 1.4;
            let sp: [&[&str]; 4] = [&SP_KARDAN, &SP_CAM, &SP_HEDIYE, &SP_TANE_A];
            let ara = 16.0 * hc + 40.0 * u;
            let bant = 4.0 * ara;
            for (sira, yb, yon) in [(0usize, hf * 0.3, 1.0f32), (1, hf * 0.62, -1.0)] {
                let v = bant / sz * (sz / 4.0).round().max(1.0);
                let kay = (tt * v * yon).rem_euclid(bant);
                let mut k = 0;
                let mut x = -bant + kay;
                while x < wf + ara {
                    let i = (k + sira) % 4;
                    let tane = if i == 3 && ((tt * 2.0) as i32) % 2 == 1 { &SP_TANE_B[..] } else { sp[i] };
                    let zip = if i == 0 { -((tt * 3.0 + k as f32).sin().abs()) * 2.0 * hc } else { 0.0 };
                    tv.sprite(tane, x, yb + zip, hc, hc, yon < 0.0, &|c| sp_renk(c, yuz));
                    x += ara; k += 1;
                }
                tv.dikd(0.0, yb + 16.0 * hc + 4.0 * u, wf, yb + 16.0 * hc + 14.0 * u, Renk(220, 232, 250), 1.0);
            }
            for i in 0..60u32 { let x = (hash(i * 3) * wf / hc).floor() * hc; let y = ((hash(i * 3 + 1) * hf + tt * 90.0 * u).rem_euclid(hf) / hc).floor() * hc; tv.dikd(x, y, x + hc, y + hc, Renk(230, 238, 255), 0.8); }
            piksel_3b(tv, yazi_s, cx, hf * 0.12, wf * 0.86, 30.0 * u, kar(ALTIN, BEYAZ, 0.2));
        }
        "piksel-fisek" => {
            tv.dikd(0.0, 0.0, wf, hf, Renk(8, 8, 24), 1.0);
            let hc = (10.0 * u).max(2.0);
            let per = 2.4f32;
            for j in 0..4u32 {
                let bas = j as f32 * per * 0.5;
                let lt = (tt - bas).rem_euclid(per * 2.0);
                let tur = ((tt - bas) / (per * 2.0)).floor() as i32;
                let ts = j * 37 + tur.rem_euclid(50) as u32 * 5;
                let (mx, my) = ((wf * (0.27 + 0.46 * hash(ts)) / hc).floor() * hc, (hf * (0.15 + 0.35 * hash(ts + 1)) / hc).floor() * hc);
                let rk = parca_renk(ts);
                if lt < 0.7 {
                    let y = ((hf + (my - hf) * (lt / 0.7)) / hc).floor() * hc;
                    tv.dikd(mx, y, mx + hc, y + hc * 2.0, Renk(255, 220, 160), 1.0);
                } else if lt < 2.2 {
                    let k = (lt - 0.7) / 1.5;
                    let r = (k.sqrt() * 18.0).floor();
                    for d in 0..8 {
                        let (dx, dy) = [(1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1), (0, -1), (1, -1)][d];
                        for s in 0..3 {
                            let q = r - s as f32 * 2.0;
                            if q < 1.0 { continue; }
                            let (x, y) = (mx + dx as f32 * q * hc, my + dy as f32 * q * hc + k * k * 6.0 * hc);
                            tv.dikd(x, y, x + hc, y + hc, if s == 0 { kar(rk, BEYAZ, 0.5) } else { kos(rk, 1.0 - 0.3 * s as f32) }, 1.0 - k * 0.6);
                        }
                    }
                }
            }
            piksel_3b(tv, yazi_s, cx, hf * 0.76, wf * 0.86, 34.0 * u, kar(ALTIN, BEYAZ, 0.2));
        }
        "kizak-gecisi" => {
            bantli(tv, Renk(4, 8, 24), Renk(18, 30, 64));
            for i in 0..80u32 { let x = hash(i * 7) * wf; let y = hash(i * 7 + 1) * hf * 0.7; tv.daire(x, y, (1.0 + 2.0 * hash(i * 7 + 2)) * u, Renk(230, 235, 250), 0.7); }
            let (ax, ay, ar) = (wf * 0.62, hf * 0.3, 230.0 * u);
            tv.daire(ax, ay, ar, Renk(236, 232, 210), 1.0);
            for (dx, dy, r) in [(-0.3f32, -0.2f32, 0.18f32), (0.25, 0.1, 0.12), (-0.05, 0.35, 0.1), (0.4, -0.35, 0.08)] { tv.daire(ax + dx * ar, ay + dy * ar, r * ar, Renk(205, 200, 180), 1.0); }
            let p = fr(tt / sz.max(4.0) * (sz / 6.0).round().max(1.0));
            let kx = -300.0 * u + p * (wf + 600.0 * u);
            let ky = ay + 120.0 * u - (p * PI).sin() * 260.0 * u;
            let s = KOYU;
            // kızak + iki geyik silueti
            tv.cokgen(&[(kx - 90.0 * u, ky), (kx + 40.0 * u, ky), (kx + 60.0 * u, ky - 40.0 * u), (kx - 70.0 * u, ky - 40.0 * u)], s, 1.0);
            tv.cizgi(kx - 110.0 * u, ky + 12.0 * u, kx + 70.0 * u, ky + 12.0 * u, 6.0 * u, s, 1.0);
            tv.daire(kx - 20.0 * u, ky - 60.0 * u, 24.0 * u, s, 1.0);
            for g in 0..2 {
                let gx = kx + (140.0 + g as f32 * 120.0) * u;
                let gy = ky - 30.0 * u - g as f32 * 20.0 * u;
                tv.elips(gx, gy, 46.0 * u, 20.0 * u, -0.15, s, 1.0);
                tv.cizgi(gx + 36.0 * u, gy - 8.0 * u, gx + 56.0 * u, gy - 40.0 * u, 12.0 * u, s, 1.0);
                tv.daire(gx + 60.0 * u, gy - 46.0 * u, 13.0 * u, s, 1.0);
                for b in [-1.0f32, 1.0] { tv.cizgi(gx + 58.0 * u, gy - 56.0 * u, gx + (58.0 + b * 16.0) * u, gy - 84.0 * u, 4.0 * u, s, 1.0); }
                let ad = (tt * 10.0 + g as f32).sin() * 12.0 * u;
                for (lx, sg) in [(-30.0f32, 1.0f32), (-18.0, -1.0), (22.0, 1.0), (34.0, -1.0)] { tv.cizgi(gx + lx * u, gy + 12.0 * u, gx + lx * u + sg * ad, gy + 46.0 * u, 5.0 * u, s, 1.0); }
                tv.cizgi(kx + 60.0 * u, ky - 20.0 * u, gx - 40.0 * u, gy, 2.5 * u, s, 1.0);
            }
            for i in 0..9 { let x = wf * (i as f32 + 0.4) / 9.0; cam_siluet(tv, x, hf * 0.82, (220.0 + 140.0 * hash(i * 3 + 9)) * u, Renk(10, 18, 30)); }
            tv.dikd(0.0, hf * 0.82, wf, hf, Renk(210, 222, 240), 1.0);
            kar_katmanli(tv, tt, sz, 90, 1.0, 0b011, 53);
            piksel_3b(tv, yazi_s, cx, hf * 0.9, wf * 0.86, 30.0 * u, Renk(200, 30, 40));
        }
        "kar-kuresi" => {
            bantli(tv, Renk(20, 10, 30), kos(yuz, 0.25));
            let (kx, ky, r) = (cx, hf * 0.42, 340.0 * u);
            // altlık (ahşap, 3 ton) + plaka
            tv.cokgen(&[(kx - r * 0.85, ky + r * 0.82), (kx + r * 0.85, ky + r * 0.82), (kx + r * 1.0, ky + r * 1.32), (kx - r * 1.0, ky + r * 1.32)], Renk(120, 70, 36), 1.0);
            tv.dikd(kx - r * 1.0, ky + r * 1.32, kx + r * 1.0, ky + r * 1.42, Renk(80, 44, 22), 1.0);
            tv.dikd(kx - r * 0.85, ky + r * 0.82, kx + r * 0.85, ky + r * 0.88, Renk(170, 110, 60), 1.0);
            tv.dikd(kx - r * 0.6, ky + r * 0.98, kx + r * 0.6, ky + r * 1.2, Renk(220, 180, 90), 1.0);
            // küre içi
            tv.daire(kx, ky, r, Renk(30, 60, 100), 1.0);
            tv.daire(kx, ky + r * 0.15, r * 0.82, Renk(40, 80, 130), 1.0);
            tv.elips(kx, ky + r * 0.62, r * 0.78, r * 0.2, 0.0, Renk(230, 238, 250), 1.0);
            cam_siluet(tv, kx - r * 0.35, ky + r * 0.62, r * 0.9, Renk(30, 120, 70));
            tv.dikd(kx + r * 0.05, ky + r * 0.25, kx + r * 0.5, ky + r * 0.6, Renk(170, 60, 50), 1.0);
            tv.cokgen(&[(kx, ky + r * 0.27), (kx + r * 0.55, ky + r * 0.27), (kx + r * 0.275, ky + r * 0.02)], Renk(235, 240, 250), 1.0);
            tv.dikd(kx + r * 0.2, ky + r * 0.38, kx + r * 0.32, ky + r * 0.5, Renk(255, 210, 90), 1.0);
            // girdap kar (kürenin içinde kırpılmış)
            let w_ = dw(sz, 0.7);
            for i in 0..120u32 {
                let rr = r * 0.9 * hash(i * 5 + 1).sqrt();
                let a = hash(i * 5 + 2) * TAU + w_ * tt * (0.5 + hash(i * 5 + 3));
                let (x, y) = (kx + a.cos() * rr, ky + a.sin() * rr * 0.9 + ((tt * 0.8 + hash(i * 5 + 4) * 3.0).sin()) * 8.0 * u);
                if (x - kx).powi(2) + (y - ky).powi(2) < (r * 0.95).powi(2) { tv.daire(x, y, (2.0 + 4.0 * hash(i * 5)) * u, BEYAZ, 0.9); }
            }
            // cam: sert yansıma yayı (ışık tonu) + kenar
            tv.halka(kx, ky, r, 8.0 * u, Renk(160, 190, 220), 1.0);
            tv.yay(kx, ky, r * 0.84, 14.0 * u, 0.58, 0.14, Renk(220, 236, 252), 0.8);
            piksel_3b(tv, yazi_s, cx, ky + r * 1.09, r * 1.1, 14.0 * u, Renk(60, 30, 10));
        }
        "mum-isigi" => {
            bantli(tv, Renk(14, 6, 4), Renk(60, 26, 12));
            for (i, (mx, boy)) in [(cx - 220.0 * u, 420.0 * u), (cx, 560.0 * u), (cx + 220.0 * u, 360.0 * u)].iter().enumerate() {
                let (mx, boy) = (*mx, *boy);
                let taban = hf * 0.8;
                let gen = 120.0 * u;
                let r = [Renk(230, 40, 50), Renk(245, 236, 220), Renk(230, 40, 50)][i];
                tv.dikd(mx - gen / 2.0, taban - boy, mx + gen / 2.0, taban, r, 1.0);
                tv.dikd(mx + gen * 0.2, taban - boy, mx + gen / 2.0, taban, kos(r, 0.62), 1.0);
                tv.dikd(mx - gen / 2.0, taban - boy, mx - gen * 0.3, taban, kar(r, BEYAZ, 0.35), 1.0);
                tv.elips(mx, taban - boy, gen / 2.0, 12.0 * u, 0.0, kar(r, BEYAZ, 0.2), 1.0);
                tv.cizgi(mx, taban - boy, mx, taban - boy - 22.0 * u, 4.0 * u, KOYU, 1.0);
                let sal = (tt * 3.1 + i as f32 * 1.7).sin() * 6.0 * u + (tt * 7.3 + i as f32).sin() * 2.0 * u;
                let (fx, fy) = (mx + sal, taban - boy - 22.0 * u);
                let fl = 90.0 * u * (1.0 + 0.06 * (tt * 5.0 + i as f32).sin());
                tv.cokgen(&[(fx - 22.0 * u, fy), (fx + 22.0 * u, fy), (fx + sal * 0.6, fy - fl)], Renk(255, 140, 30), 1.0);
                tv.daire(fx, fy - 4.0 * u, 22.0 * u, Renk(255, 140, 30), 1.0);
                tv.cokgen(&[(fx - 11.0 * u, fy), (fx + 11.0 * u, fy), (fx + sal * 0.4, fy - fl * 0.6)], Renk(255, 220, 110), 1.0);
                tv.daire(fx, fy - 2.0 * u, 11.0 * u, Renk(255, 220, 110), 1.0);
            }
            tv.dikd(0.0, hf * 0.8, wf, hf * 0.83, Renk(110, 60, 30), 1.0);
            for k in 0..9 { let x = wf * (k as f32 + 0.5) / 9.0; tv.elips(x, hf * 0.835, 40.0 * u, 16.0 * u, 0.4, Renk(30, 110, 50), 1.0); if k % 2 == 0 { tv.daire(x + 30.0 * u, hf * 0.83, 10.0 * u, Renk(210, 30, 40), 1.0); } }
            piksel_3b(tv, yazi_s, cx, hf * 0.2, wf * 0.86, 34.0 * u, kar(ALTIN, BEYAZ, 0.3));
        }
        "hediye-acilis" => {
            bantli(tv, Renk(20, 6, 14), kos(yuz, 0.3));
            let (bx, alt, gen) = (cx, hf * 0.78, 420.0 * u);
            hediye(tv, bx, alt, gen, gen * 0.8, Renk(200, 30, 50), Renk(255, 200, 60));
            let k = ((tt - 1.0) / 0.6).clamp(0.0, 1.0);
            if k > 0.0 {
                // kapak uçar (yukarı + dönme), içten kıvılcım patlaması ve 3B yazı yükselir
                let kx = bx + k * 260.0 * u;
                let ky = alt - gen * 0.8 - k * 520.0 * u + k * k * 200.0 * u;
                let a = k * 1.2;
                let (c, s) = (a.cos(), a.sin());
                let pk: Vec<(f32, f32)> = [(-0.56f32, -0.08f32), (0.56, -0.08), (0.56, 0.08), (-0.56, 0.08)].iter().map(|&(x, y)| (kx + (x * c - y * s) * gen, ky + (x * s + y * c) * gen)).collect();
                tv.cokgen(&pk, Renk(230, 50, 70), 1.0);
                if tt > 1.2 { parca::patlama(tv, Tur::Kure, bx, alt - gen * 0.85, tt - 1.2, Renk(255, 205, 60), Renk(80, 200, 255), 77, u); }
                let yk = anim::geri(((tt - 1.3) / 0.9).clamp(0.0, 1.0));
                if tt > 1.3 { piksel_3b(tv, yazi_s, cx, alt - gen * 0.8 - yk * 560.0 * u, wf * 0.86 * yk.max(0.2), 40.0 * u * yk.max(0.2), kar(ALTIN, BEYAZ, 0.3)); }
            } else {
                tv.cokgen(&[(bx - gen * 0.56, alt - gen * 0.88), (bx + gen * 0.56, alt - gen * 0.88), (bx + gen * 0.56, alt - gen * 0.72), (bx - gen * 0.56, alt - gen * 0.72)], Renk(230, 50, 70), 1.0);
                let sal = (tt * 18.0).sin() * 5.0 * u * tt.min(1.0);
                tv.dikd(bx - gen * 0.07 + sal, alt - gen * 0.88, bx + gen * 0.07 + sal, alt - gen * 0.72, Renk(255, 200, 60), 1.0);
            }
        }
        "fisek-yazi" => {
            bantli(tv, Renk(4, 4, 14), Renk(16, 14, 40));
            let n = (dv.round() as usize).clamp(2, 10);
            parca::gosteri(tv, tt, n, &|j| [Tur::Kure, Tur::Halka, Tur::Sogut, Tur::Cift][j % 4], &[Renk(255, 205, 60), Renk(80, 200, 255), Renk(255, 80, 120), Renk(130, 255, 110)], 61, 0.1, 0.4);
            y3(tv, z, acik, kos(vu, 0.55), cx, hf * 0.66, 1.0, (tt * 0.5).sin() * 0.1, 12.0 * u, 1.0);
            yazi_ya(tv, z, yazi_s, cx, hf * 0.66, u);
        }
        "kis-penceresi" => {
            bantli(tv, Renk(8, 14, 40), Renk(24, 40, 80));
            let (x0, y0, x1, y1) = (wf * 0.14, hf * 0.3, wf * 0.86, hf * 0.78);
            tv.daire(x1 - 120.0 * u, y0 + 120.0 * u, 70.0 * u, Renk(236, 232, 210), 1.0);
            for i in 0..6 { cam_siluet(tv, x0 + (x1 - x0) * (i as f32 + 0.5) / 6.0, y1 - 40.0 * u, (200.0 + 100.0 * hash(i * 4 + 1)) * u, Renk(14, 30, 40)); }
            tv.dikd(x0, y1 - 40.0 * u, x1, y1, Renk(210, 222, 240), 1.0);
            kar_katmanli(tv, tt, sz, 160, 1.0, 0b111, 59);
            // duvar (pencerenin çevresi) ve çerçeve
            let duvar = Renk(120, 54, 40);
            tv.dikd(0.0, 0.0, wf, y0 - 30.0 * u, duvar, 1.0);
            tv.dikd(0.0, y1 + 30.0 * u, wf, hf, duvar, 1.0);
            tv.dikd(0.0, y0 - 30.0 * u, x0 - 30.0 * u, y1 + 30.0 * u, duvar, 1.0);
            tv.dikd(x1 + 30.0 * u, y0 - 30.0 * u, wf, y1 + 30.0 * u, duvar, 1.0);
            let mut yy = 0.0; let mut sat = 0; while yy < hf { tv.dikd(0.0, yy, wf, yy + 3.0 * u, kos(duvar, 0.7), 1.0); let of = if sat % 2 == 0 { 0.0 } else { 60.0 * u }; let mut xx = of; while xx < wf { tv.dikd(xx, yy, xx + 3.0 * u, yy + 50.0 * u, kos(duvar, 0.7), 1.0); xx += 120.0 * u; } yy += 50.0 * u; sat += 1; }
            tv.dikd(0.0, y0 - 30.0 * u, wf, y0 - 30.0 * u, duvar, 1.0);
            let ah = Renk(235, 230, 220);
            for (a, b, c, d) in [(x0 - 30.0 * u, y0 - 30.0 * u, x1 + 30.0 * u, y0), (x0 - 30.0 * u, y1, x1 + 30.0 * u, y1 + 30.0 * u), (x0 - 30.0 * u, y0, x0, y1), (x1, y0, x1 + 30.0 * u, y1), (cx - 10.0 * u, y0, cx + 10.0 * u, y1), (x0, (y0 + y1) / 2.0 - 10.0 * u, x1, (y0 + y1) / 2.0 + 10.0 * u)] {
                tv.dikd(a, b, c, d, ah, 1.0);
                tv.dikd(c - 5.0 * u, b, c, d, kos(ah, 0.62), 1.0);
            }
            tv.dikd(x0 - 50.0 * u, y1 + 30.0 * u, x1 + 50.0 * u, y1 + 54.0 * u, Renk(200, 190, 175), 1.0);
            tv.elips(cx, y1 + 30.0 * u, (x1 - x0) / 2.0 + 40.0 * u, 18.0 * u, 0.0, Renk(240, 245, 252), 1.0);
            buz_koseler(tv, x0, y0, x1, y1, anim::yumusak(tt / 3.0), u);
            piksel_3b(tv, yazi_s, cx, y0 * 0.45, wf * 0.86, 32.0 * u, KREM);
        }
        "noel-yildizi" => {
            bantli(tv, Renk(6, 8, 26), kos(yuz, 0.3));
            kar_katmanli(tv, tt, sz, 100, 1.0, 0b011, 67);
            let (v, y) = yildiz_prizma(300.0 * u, 0.45, 50.0 * u, ALTIN);
            // önde 2,4 sn bekler, sonra tam tur (küçük resimde yüz görünür)
            let p = fr(tt / 8.0);
            let ay = if p < 0.3 { 0.0 } else { anim::yumusak((p - 0.3) / 0.7) * TAU };
            nesne(tv, &v, &y, ay, 0.25, cx, hf * 0.38, 1600.0 * u, 3.0 * u);
            piksel_3b(tv, yazi_s, cx, hf * 0.74, wf * 0.86, 34.0 * u, KREM);
            kar_katmanli(tv, tt, sz, 30, 1.0, 0b100, 67);
        }
        // ------------------------------------------------ premium sahneler (A5–A8 + ek)
        "sivi-metal" => {
            let w1 = dw(sz, 1.0); let w2 = dw(sz, 0.7); let w3 = dw(sz, 1.3);
            let p = kar(vu, yuz, 0.3); // palet vurgusu: tek-altin altın, gumus-mavi gümüş, mercan bakır
            let rampa = [kos(p, 0.14), kos(p, 0.28), kos(p, 0.5), kos(p, 0.85), kos(p, 0.5), kos(p, 0.28), kos(p, 0.18)]; // LED: kare ortalaması ≤ %45
            alan(tv, 7, &|i| rampa[i], &|x, y| {
                let f = (x / (90.0 * u) + w1 * tt).sin() + (y / (70.0 * u) - w2 * tt).sin() + ((x + y) / (120.0 * u) + w3 * tt).sin();
                (f / 6.0 + 0.5, 1.0)
            });
            if let Some(mm) = mk {
                y3(tv, z, kar(p, BEYAZ, 0.2), kos(p, 0.3), cx, cy, 1.0, 0.0, 12.0 * u, 1.0);
                let (ox, oy) = ofs(mm, cx, cy);
                kontur(tv, mm, ox, oy, (5.0 * u).max(2.0) as i32, KOYU);
                let k = (3.0 * u).max(2.0) as i32;
                yuz_boya(tv, mm, ox, oy, &|x, y, _| if ma(mm, x - k, y - k) < 0.5 { Some(kar(p, BEYAZ, 0.6)) } else if ma(mm, x + k, y + k) < 0.5 { Some(kos(p, 0.45)) } else { None });
            } else { yazi_ya(tv, z, yazi_s, cx, cy, u); }
        }
        "sis" | "duman" => {
            let duman = m.id == "duman";
            bantli(tv, Renk(8, 10, 16), kos(zem, 0.5));
            y3(tv, z, acik, kos(vu, 0.55), cx, cy, 1.0, (tt * 0.4).sin() * 0.08, 12.0 * u, 1.0);
            yazi_ya(tv, z, yazi_s, cx, cy, u);
            let hc = 150.0 * u;
            let pxp = ((wf / hc).ceil() as i32).max(2);
            let pyp = ((hf / hc).ceil() as i32).max(2);
            let k = (sz / 8.0).round().max(1.0);
            let kay = tt / sz * k;
            let renk = if duman { Renk(150, 150, 158) } else { Renk(190, 205, 225) };
            alan(tv, 4, &|i| kar(kos(renk, 0.7), renk, i as f32 / 3.0), &|x, y| {
                if duman {
                    // alttan yükselen üç sütun; y yönünde dönemli gürültü (döngü güvenli)
                    let yy = y / hc + kay * pyp as f32;
                    let mut s = 0.0f32;
                    for c in [0.22f32, 0.5, 0.8] { let gen = 0.06 + 0.22 * (1.0 - y / hf); let d = (x / wf - c - 0.04 * (yy * 0.7).sin()) / gen; s += (-d * d).exp(); }
                    let n = 0.6 * gurultu2(x / hc, yy, 3, pxp, pyp) + 0.4 * gurultu2(x / hc * 2.0, yy * 2.0, 4, pxp * 2, pyp * 2);
                    let v = (s.min(1.0) * n * 1.4).min(1.0);
                    (v, if v > 0.12 { 0.55 * v } else { 0.0 })
                } else {
                    let a = gurultu2(x / hc + kay * pxp as f32, y / hc, 5, pxp, pyp);
                    let b = gurultu2(x / hc * 1.7 - kay * pxp as f32 * 1.0, y / hc * 1.7, 6, pxp, pyp);
                    let v = (a * 0.6 + b * 0.4) * (0.4 + 0.6 * (y / hf));
                    (v, (v - 0.2).max(0.0) * 0.55)
                }
            });
        }
        "kumas-dalga" => {
            let w_ = dw(sz, 1.6);
            let l = 120.0 * u;
            let amp = 30.0 * u;
            let rk = yuz;
            let (w, h) = (tv.w as i32, tv.h as i32);
            for x in 0..w {
                let fz = x as f32 / l - w_ * tt;
                let d = amp * fz.sin();
                let ton = 0.7 + 0.3 * ((fz.cos() * 0.5 + 0.5) * 5.0).floor() / 5.0;
                let c1 = kos(rk, ton * 0.42); // LED: koyu kumaş, açık yazı
                let c2 = kos(rk, ton * 0.52);
                let mut y = 0;
                while y < h { let bant = (((y as f32 - d) / (90.0 * u)).floor() as i32).rem_euclid(2) == 0; tv.px(x, y, if bant { c1 } else { c2 }, 1.0); y += 1; }
            }
            if let Some(mm) = mk {
                let (ox, oy) = ofs(mm, cx, cy);
                let (x0, y0, x1, y1) = kutu(mm);
                for x in x0..=x1 {
                    let fz = (x + ox) as f32 / l - w_ * tt;
                    let d = (amp * fz.sin()).round() as i32;
                    let ton = 0.72 + 0.28 * ((fz.cos() * 0.5 + 0.5) * 5.0).floor() / 5.0;
                    for y in y0 - amp as i32 - 2..=y1 + amp as i32 + 2 { let a = ma(mm, x, y - d); if a > 0.01 { tv.px(x + ox + (6.0 * u) as i32, y + oy + (6.0 * u) as i32, KOYU, a); } }
                    for y in y0 - amp as i32 - 2..=y1 + amp as i32 + 2 { let a = ma(mm, x, y - d); if a > 0.01 { tv.px(x + ox, y + oy, kos(kar(acik, BEYAZ, 0.5), ton), a); } }
                }
            } else { yazi_ya(tv, z, yazi_s, cx, cy, u); }
        }
        "ucboyut-kutu" => {
            bantli(tv, Renk(8, 8, 18), kos(vu, 0.3));
            tv.elips(cx, hf * 0.62, 280.0 * u, 40.0 * u, 0.0, KOYU, 0.7);
            let ay = dw(sz, 0.8) * tt;
            let ax = 0.45 + 0.1 * (dw(sz, 0.5) * tt).sin();
            let r = 200.0 * u;
            nesne(tv, &kup(r), &kup_yuz([yuz, vu, yuz, vu, kar(yuz, vu, 0.5), kar(yuz, vu, 0.5)]), ay, ax, cx, hf * 0.4, 1500.0 * u, 4.0 * u);
            piksel_3b(tv, yazi_s, cx, hf * 0.76, wf * 0.86, 36.0 * u, KREM);
        }
        "parcacik-girdap" => {
            bantli(tv, Renk(4, 4, 12), kos(vu, 0.2));
            let w_ = dw(sz, 0.6);
            let n = 520u32;
            let parc = |tv: &mut Tuval, on: bool| {
                for i in 0..n {
                    let rr = (120.0 + 340.0 * hash(i * 3 + 1)) * u;
                    let a = hash(i * 3 + 2) * TAU + w_ * tt * (1.0 + (1.0 - rr / (460.0 * u)) * 2.0).round();
                    let d = a.sin();
                    if (d > 0.0) != on { continue; }
                    let (x, y) = (cx + a.cos() * rr, cy + (hash(i * 3) - 0.5) * 120.0 * u + d * rr * 0.32);
                    let s = (2.0 + 3.0 * (d * 0.5 + 0.5)) * u;
                    let rk = kar(vu, BEYAZ, 0.2 + 0.4 * (d * 0.5 + 0.5));
                    if s < 1.5 { tv.px(x as i32, y as i32, rk, 0.9); } else { tv.daire(x, y, s, rk, 0.5 + 0.5 * (d * 0.5 + 0.5)); }
                }
            };
            parc(tv, false);
            y3(tv, z, acik, kos(vu, 0.55), cx, cy, 1.0, (tt * 0.5).sin() * 0.1, 12.0 * u, 1.0);
            yazi_ya(tv, z, yazi_s, cx, cy, u);
            parc(tv, true);
        }
        "cam-panel" => {
            bantli(tv, Renk(6, 10, 18), kos(yuz, 0.3));
            for (i, (px, py, pw, ph)) in [(0.12f32, 0.24f32, 0.5f32, 0.34f32), (0.38, 0.4, 0.5, 0.3), (0.2, 0.56, 0.62, 0.22)].iter().enumerate() {
                let k = anim::yumusak(((tt - i as f32 * 0.3) / 1.0).clamp(0.0, 1.0));
                let kay = (1.0 - k) * wf * if i % 2 == 0 { -1.0 } else { 1.0 } + (tt * 0.6 + i as f32).sin() * 10.0 * u;
                let (x0, y0, x1, y1) = (wf * px + kay, hf * py, wf * (px + pw) + kay, hf * (py + ph));
                tv.dikd(x0 + 16.0 * u, y0 + 16.0 * u, x1 + 16.0 * u, y1 + 16.0 * u, KOYU, 0.45);
                tv.dikd(x0, y0, x1, y1, kar(yuz, BEYAZ, 0.3), 0.22);
                tv.dikd(x0, y0, x1, y0 + 4.0 * u, kar(yuz, BEYAZ, 0.6), 0.9);
                tv.dikd(x0, y0, x0 + 4.0 * u, y1, kar(yuz, BEYAZ, 0.6), 0.9);
                tv.dikd(x0, y1 - 4.0 * u, x1, y1, kos(yuz, 0.4), 0.9);
                // eğik yansıma bandı (ton, parlama değil)
                let bx = x0 + fr(tt / 4.0 + i as f32 * 0.3) * (x1 - x0 + 300.0 * u) - 150.0 * u;
                let (ya, yb) = (y0, y1);
                let e = 0.35 * (yb - ya);
                let pts = [(bx, ya), (bx + 60.0 * u, ya), (bx + 60.0 * u - e, yb), (bx - e, yb)];
                let pk: Vec<(f32, f32)> = pts.iter().map(|&(x, y)| (x.clamp(x0, x1), y)).collect();
                tv.cokgen(&pk, BEYAZ, 0.12);
            }
            y3(tv, z, acik, kos(vu, 0.55), cx, hf * 0.47, 1.0, 0.0, 12.0 * u, 1.0);
            yazi_ya(tv, z, yazi_s, cx, hf * 0.47, u);
        }
        "spot-isik" => {
            tv.dikd(0.0, 0.0, wf, hf, Renk(10, 8, 14), 1.0);
            tv.dikd(0.0, hf * 0.7, wf, hf, Renk(26, 18, 22), 1.0);
            y3(tv, z, acik, kos(vu, 0.55), cx, cy, 1.0, 0.0, 12.0 * u, 1.0);
            yazi_ya(tv, z, yazi_s, cx, cy, u);
            let w_ = dw(sz, 0.7);
            let sp = [(cx + (w_ * tt).sin() * wf * 0.3, cy + (w_ * tt * 2.0).sin() * 80.0 * u), (cx + (w_ * tt + 2.4).sin() * wf * 0.32, cy + (w_ * tt + 1.0).cos() * 90.0 * u)];
            let r = 300.0 * u;
            let (w, h) = (tv.w, tv.h);
            for y in 0..h { for x in 0..w {
                let mut q = 3usize;
                for &(sx, sy) in &sp { let d = ((x as f32 - sx).powi(2) + ((y as f32 - sy) * 1.4).powi(2)) / (r * r); let l = if d < 0.7 { 0 } else if d < 0.85 { 1 } else if d < 1.0 { 2 } else { 3 }; let l = if l > 0 && l < 3 && bay(x, y) > 0.5 { l + 1 } else { l }; q = q.min(l.min(3)); }
                if q == 0 { continue; }
                let k = [1.0f32, 0.72, 0.5, 0.3][q];
                let i = ((y * w + x) * 3) as usize;
                tv.t[i] = (tv.t[i] as f32 * k) as u8; tv.t[i + 1] = (tv.t[i + 1] as f32 * k) as u8; tv.t[i + 2] = (tv.t[i + 2] as f32 * k) as u8;
            } }
        }
        "derin-tunel" => {
            tv.dikd(0.0, 0.0, wf, hf, Renk(6, 6, 12), 1.0);
            let k = (sz / 4.0).round().max(1.0);
            for i in (0..14).rev() {
                let q = fr(i as f32 / 14.0 + tt / sz * k);
                let s = 0.06 / (1.06 - q);
                let (hw, hh) = (wf * 0.5 * s, hf * 0.5 * s);
                if hw > wf * 1.5 { continue; }
                let kal = (2.0 + 16.0 * q * q) * u;
                let rk = if i % 2 == 0 { kar(vu, KOYU, 0.6 - 0.5 * q) } else { kar(yuz, KOYU, 0.65 - 0.5 * q) };
                tv.dikd(cx - hw, cy - hh, cx + hw, cy - hh + kal, rk, 1.0);
                tv.dikd(cx - hw, cy + hh - kal, cx + hw, cy + hh, rk, 1.0);
                tv.dikd(cx - hw, cy - hh, cx - hw + kal, cy + hh, rk, 1.0);
                tv.dikd(cx + hw - kal, cy - hh, cx + hw, cy + hh, rk, 1.0);
            }
            y3(tv, z, acik, kos(vu, 0.55), cx, cy, 1.0, (tt * 0.5).sin() * 0.1, 12.0 * u, 1.0);
            yazi_ya(tv, z, yazi_s, cx, cy, u);
        }
        "yildiz-alani" => {
            tv.dikd(0.0, 0.0, wf, hf, Renk(2, 2, 8), 1.0);
            let k = (sz / 3.0).round().max(1.0);
            for i in 0..420u32 {
                let (x3, y3_) = (hash(i * 4) * 2.0 - 1.0, hash(i * 4 + 1) * 2.0 - 1.0);
                let q = fr(hash(i * 4 + 2) + tt / sz * k);
                let zz = 1.0 - q * 0.97;
                let (x, y) = (cx + x3 * wf * 0.5 / zz * 0.3, cy + y3_ * hf * 0.5 / zz * 0.3);
                let zz2 = (zz + 0.05).min(1.0);
                let (xa, ya) = (cx + x3 * wf * 0.5 / zz2 * 0.3, cy + y3_ * hf * 0.5 / zz2 * 0.3);
                let par = (q * 1.3).min(1.0);
                tv.cizgi(xa, ya, x, y, (1.0 + 3.0 * q) * u, kar(Renk(120, 140, 200), BEYAZ, par), par);
            }
            y3(tv, z, acik, kos(vu, 0.55), cx, cy, 1.0, (tt * 0.5).sin() * 0.1, 12.0 * u, 1.0);
            yazi_ya(tv, z, yazi_s, cx, cy, u);
        }
        "sahne-perde" => {
            bantli(tv, Renk(8, 6, 10), Renk(30, 20, 26));
            crate::malzeme_sahne::isik_huzmesi(tv, tt, yuz);
            y3(tv, z, acik, kos(vu, 0.55), cx, cy, 1.0, (tt * 0.5).sin() * 0.1, 12.0 * u, 1.0);
            yazi_ya(tv, z, yazi_s, cx, cy, u);
            // perde: ortadan açılır (döngüsüz), kadife kıvrımları 5 seviyeli ton
            let ac = anim::yumusak(((tt - 0.3) / 1.8).clamp(0.0, 1.0));
            let kad = Renk(170, 20, 36);
            let yari = wf * 0.5 * (1.0 - ac * 0.82);
            let w = tv.w as i32;
            for x in 0..w {
                let xf = x as f32;
                let (ic, yer) = if xf < cx { (xf < yari, xf) } else { (xf > wf - yari, wf - xf) };
                if !ic { continue; }
                let sik = 70.0 * u * (1.0 - ac * 0.5);
                let c = (yer / sik * TAU).cos();
                let ton = 0.55 + 0.45 * ((c * 0.5 + 0.5) * 5.0).floor() / 5.0;
                let etek = hf - 30.0 * u * (yer / sik * TAU).sin().abs();
                tv.dikd(xf, 0.0, xf + 1.0, etek, kos(kad, ton), 1.0);
            }
            tv.dikd(0.0, 0.0, wf, 70.0 * u, kos(kad, 0.7), 1.0);
            let mut x = 0.0; while x < wf { tv.daire(x + 30.0 * u, 70.0 * u, 30.0 * u, kos(kad, 0.7), 1.0); x += 60.0 * u; }
            tv.dikd(0.0, 60.0 * u, wf, 66.0 * u, ALTIN, 1.0);
        }
        _ => return false,
    }
    true
}

fn parca_renk(ts: u32) -> Renk { [Renk(255, 205, 60), Renk(80, 200, 255), Renk(255, 80, 120), Renk(130, 255, 110), Renk(255, 140, 40)][(hash(ts + 9) * 5.0) as usize % 5] }

/// Ek efekt adları (`efekt=`; dekor::ciz'den, içerikten önce): buz-koseler, sis-alt.
pub fn efekt(ad: &str, t: &mut [u8], w: u32, h: u32, z: f32, _renk: Renk, _tohum: u32) -> bool {
    let mut tv = Tuval { t, w, h };
    let (wf, hf) = (w as f32, h as f32);
    let u = (wf / 950.0).min(hf / 1800.0);
    match ad {
        "buz-koseler" => buz_koseler(&mut tv, 0.0, 0.0, wf, hf, anim::yumusak(z / 3.0), u),
        "sis-alt" => {
            let hc = 150.0 * u;
            let pxp = ((wf / hc).ceil() as i32).max(2);
            alan(&mut tv, 4, &|i| kar(Renk(120, 130, 150), Renk(190, 205, 225), i as f32 / 3.0), &|x, y| {
                if y < hf * 0.55 { return (0.0, 0.0); }
                let v = gurultu2(x / hc + z * 0.15, y / hc, 5, pxp, 0) * ((y / hf - 0.55) / 0.45);
                (v, (v - 0.15).max(0.0) * 0.6)
            });
        }
        _ => return false,
    }
    true
}

#[cfg(test)]
mod t {
    use super::*;
    #[test]
    fn uzun_golge_capraz_kopyalar() {
        let (w, h) = (60u32, 60u32);
        let mut a = vec![0f32; (w * h) as usize];
        for y in 10..20 { for x in 10..20 { a[(y * w + x) as usize] = 1.0; } }
        let m = Maske { w, h, a, bx: (10, 10, 19, 19) };
        let mut b = vec![0u8; (w * h * 3) as usize];
        { let mut tv = Tuval { t: &mut b, w, h }; uzun_golge(&mut tv, &m, 0, 0, 8, Renk(200, 0, 0)); }
        let px = |x: u32, y: u32| b[((y * w + x) * 3) as usize];
        assert!(px(25, 25) > 0, "köşegende gölge olmalı");
        assert_eq!(px(15, 15), 0, "maskenin içi gölgelenmez");
        assert_eq!(px(29, 29), 0, "gölge boyu sınırlı");
        assert_eq!(px(5, 25), 0, "sol alt gölgesiz");
    }
    #[test]
    fn donemli_gurultu_sarar() {
        for i in 0..50 { let y = i as f32 * 0.31; assert!((gurultu2(0.4, y, 3, 5, 0) - gurultu2(5.4, y, 3, 5, 0)).abs() < 1e-5); }
    }
    #[test]
    fn dw_tam_tur() { let s = 7.3; let w = dw(s, 1.0); assert!(((w * s / TAU) - (w * s / TAU).round()).abs() < 1e-4); }
}
