//! Malzeme çizimi: yazılım render (GPU yok), 950x1800 dikey LED için kalın, yüksek kontrastlı şekiller.
//! İçerik: çizim temelleri (daire, halka, çokgen, kapsül çizgi, kalp, yıldız, yay), 5x7 piksel yazı tipi (Türkçe harfler dahil,
//! harici font yok), özgün piksel karakterler ve 53 malzemenin çizicileri. Tüm rastgelelik sabit tohumlu `efekt::hash` ile: deterministik.
//! Sahip: Yusuf. Yazan: [Claude#Malzeme Opus 5.5, yenileme-1.0, 2026-11-18]. Yalnız yazan yapay zekâ ya da Yusuf'un izniyle değişir; silme yok.
use crate::anim::{self, Maske, Poz};
use crate::efekt::hash;
use crate::malzeme::MalzemeAyar;
use crate::senaryo::Hazir;
use crate::Renk;
use ab_glyph::FontRef;

const PI: f32 = std::f32::consts::PI;
const TAU: f32 = std::f32::consts::TAU;
/// Canlı, LED'de iyi görünen sabit renkler (konfeti, balon, havai fişek).
const RENKLER: [Renk; 6] = [Renk(255, 214, 0), Renk(0, 220, 255), Renk(255, 70, 140), Renk(120, 255, 90), Renk(255, 140, 0), Renk(190, 120, 255)];
const SARI: Renk = Renk(255, 214, 0);
const KOYU: Renk = Renk(16, 16, 20);
const BEYAZ: Renk = Renk(235, 235, 228);

fn kos(r: Renk, k: f32) -> Renk { Renk((r.0 as f32 * k).min(255.0) as u8, (r.1 as f32 * k).min(255.0) as u8, (r.2 as f32 * k).min(255.0) as u8) }
fn kar(a: Renk, b: Renk, k: f32) -> Renk { let k = k.clamp(0.0, 1.0); Renk((a.0 as f32 + (b.0 as f32 - a.0 as f32) * k) as u8, (a.1 as f32 + (b.1 as f32 - a.1 as f32) * k) as u8, (a.2 as f32 + (b.2 as f32 - a.2 as f32) * k) as u8) }
fn don(p: (f32, f32), a: f32) -> (f32, f32) { let (c, s) = (a.cos(), a.sin()); (p.0 * c - p.1 * s, p.0 * s + p.1 * c) }
fn fr(x: f32) -> f32 { x - x.floor() }
/// Ters sınırda da güvenli kıstırma (a > b ise a döner).
fn sinir(x: f32, a: f32, b: f32) -> f32 { x.max(a).min(b.max(a)) }

pub struct Tuval<'a> { pub t: &'a mut [u8], pub w: u32, pub h: u32 }

impl<'a> Tuval<'a> {
    #[inline]
    pub fn px(&mut self, x: i32, y: i32, r: Renk, a: f32) {
        if a <= 0.0 || x < 0 || y < 0 || x >= self.w as i32 || y >= self.h as i32 { return; }
        let i = ((y as u32 * self.w + x as u32) * 3) as usize;
        let a = a.min(1.0);
        self.t[i] = (self.t[i] as f32 + (r.0 as f32 - self.t[i] as f32) * a) as u8;
        self.t[i + 1] = (self.t[i + 1] as f32 + (r.1 as f32 - self.t[i + 1] as f32) * a) as u8;
        self.t[i + 2] = (self.t[i + 2] as f32 + (r.2 as f32 - self.t[i + 2] as f32) * a) as u8;
    }
    fn kutu(&self, x0: f32, y0: f32, x1: f32, y1: f32) -> Option<(i32, i32, i32, i32)> {
        let (a, b) = (x0.floor().max(0.0) as i32, y0.floor().max(0.0) as i32);
        let (c, d) = (x1.ceil().min(self.w as f32 - 1.0) as i32, y1.ceil().min(self.h as f32 - 1.0) as i32);
        if c < a || d < b || x1 < 0.0 || y1 < 0.0 { None } else { Some((a, b, c, d)) }
    }
    pub fn daire(&mut self, cx: f32, cy: f32, r: f32, renk: Renk, a: f32) {
        if r <= 0.0 { return; }
        let Some((x0, y0, x1, y1)) = self.kutu(cx - r - 1.0, cy - r - 1.0, cx + r + 1.0, cy + r + 1.0) else { return };
        for y in y0..=y1 { for x in x0..=x1 {
            let d = ((x as f32 + 0.5 - cx).powi(2) + (y as f32 + 0.5 - cy).powi(2)).sqrt();
            let k = (r - d + 0.5).clamp(0.0, 1.0);
            if k > 0.0 { self.px(x, y, renk, k * a); }
        } }
    }
    pub fn halka(&mut self, cx: f32, cy: f32, r: f32, kal: f32, renk: Renk, a: f32) { self.yay(cx, cy, r, kal, 0.0, 1.0, renk, a) }
    /// Yay: `bas` açısından saat yönünde `oran` (0..1) tur.
    pub fn yay(&mut self, cx: f32, cy: f32, r: f32, kal: f32, bas: f32, oran: f32, renk: Renk, a: f32) {
        if oran <= 0.0 { return; }
        let m = r + kal;
        let Some((x0, y0, x1, y1)) = self.kutu(cx - m, cy - m, cx + m, cy + m) else { return };
        let tam = oran >= 1.0;
        for y in y0..=y1 { for x in x0..=x1 {
            let (dx, dy) = (x as f32 + 0.5 - cx, y as f32 + 0.5 - cy);
            let d = (dx * dx + dy * dy).sqrt();
            let k = (kal / 2.0 - (d - r).abs() + 0.5).clamp(0.0, 1.0);
            if k <= 0.0 { continue; }
            if !tam { let aci = (dy.atan2(dx) - bas).rem_euclid(TAU); if aci > oran * TAU { continue; } }
            self.px(x, y, renk, k * a);
        } }
    }
    pub fn dikd(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, renk: Renk, a: f32) {
        let Some((a0, b0, a1, b1)) = self.kutu(x0, y0, x1 - 1.0, y1 - 1.0) else { return };
        for y in b0..=b1 { for x in a0..=a1 { self.px(x, y, renk, a); } }
    }
    /// Yuvarlak köşeli dolu dikdörtgen (merkez, yarı genişlik/yükseklik, köşe yarıçapı).
    pub fn yuvarlak(&mut self, cx: f32, cy: f32, hw: f32, hh: f32, rr: f32, renk: Renk, a: f32) {
        let Some((x0, y0, x1, y1)) = self.kutu(cx - hw - 1.0, cy - hh - 1.0, cx + hw + 1.0, cy + hh + 1.0) else { return };
        let rr = rr.min(hw).min(hh);
        for y in y0..=y1 { for x in x0..=x1 {
            let (qx, qy) = ((x as f32 + 0.5 - cx).abs() - hw + rr, (y as f32 + 0.5 - cy).abs() - hh + rr);
            let d = (qx.max(0.0).powi(2) + qy.max(0.0).powi(2)).sqrt() + qx.max(qy).min(0.0) - rr;
            let k = (0.5 - d).clamp(0.0, 1.0);
            if k > 0.0 { self.px(x, y, renk, k * a); }
        } }
    }
    /// Kalın çizgi (yuvarlak uçlu kapsül).
    pub fn cizgi(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, kal: f32, renk: Renk, a: f32) {
        let r = kal / 2.0;
        let Some((a0, b0, a1, b1)) = self.kutu(x0.min(x1) - r - 1.0, y0.min(y1) - r - 1.0, x0.max(x1) + r + 1.0, y0.max(y1) + r + 1.0) else { return };
        let (vx, vy) = (x1 - x0, y1 - y0);
        let l2 = (vx * vx + vy * vy).max(1e-6);
        for y in b0..=b1 { for x in a0..=a1 {
            let (px, py) = (x as f32 + 0.5 - x0, y as f32 + 0.5 - y0);
            let h = ((px * vx + py * vy) / l2).clamp(0.0, 1.0);
            let d = ((px - vx * h).powi(2) + (py - vy * h).powi(2)).sqrt();
            let k = (r - d + 0.5).clamp(0.0, 1.0);
            if k > 0.0 { self.px(x, y, renk, k * a); }
        } }
    }
    pub fn cokgen(&mut self, p: &[(f32, f32)], renk: Renk, a: f32) {
        if p.len() < 3 { return; }
        let (mut x0, mut y0, mut x1, mut y1) = (f32::MAX, f32::MAX, f32::MIN, f32::MIN);
        for q in p { x0 = x0.min(q.0); y0 = y0.min(q.1); x1 = x1.max(q.0); y1 = y1.max(q.1); }
        let Some((a0, b0, a1, b1)) = self.kutu(x0, y0, x1, y1) else { return };
        for y in b0..=b1 {
            let yy = y as f32 + 0.5;
            // tarama çizgisi: kesişimler
            let mut kes: Vec<f32> = vec![];
            for i in 0..p.len() {
                let (u, v) = (p[i], p[(i + 1) % p.len()]);
                if (u.1 <= yy && v.1 > yy) || (v.1 <= yy && u.1 > yy) { kes.push(u.0 + (yy - u.1) / (v.1 - u.1) * (v.0 - u.0)); }
            }
            kes.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            for c in kes.chunks(2) {
                if c.len() < 2 { break; }
                let (s, e) = (c[0], c[1]);
                for x in (s.floor() as i32).max(a0)..=(e.ceil() as i32).min(a1) {
                    let k = ((x as f32 + 1.0).min(e) - (x as f32).max(s)).clamp(0.0, 1.0);
                    if k > 0.0 { self.px(x, y, renk, k * a); }
                }
            }
        }
    }
    pub fn yildiz(&mut self, cx: f32, cy: f32, r: f32, ic: f32, n: usize, aci: f32, renk: Renk, a: f32) {
        let p: Vec<(f32, f32)> = (0..n * 2).map(|i| { let rr = if i % 2 == 0 { r } else { r * ic }; let t = aci - PI / 2.0 + i as f32 * PI / n as f32; (cx + rr * t.cos(), cy + rr * t.sin()) }).collect();
        self.cokgen(&p, renk, a);
    }
    pub fn kalp(&mut self, cx: f32, cy: f32, r: f32, renk: Renk, a: f32) {
        if r <= 1.0 { return; }
        let Some((x0, y0, x1, y1)) = self.kutu(cx - r * 1.25, cy - r * 1.1, cx + r * 1.25, cy + r * 1.25) else { return };
        let e = 1.6 / r;
        for y in y0..=y1 { for x in x0..=x1 {
            let (u, v) = ((x as f32 + 0.5 - cx) / r * 1.12, -(y as f32 + 0.5 - cy) / r * 1.12 + 0.12);
            let f = (u * u + v * v - 1.0).powi(3) - u * u * v * v * v;
            let k = (0.5 - f / e).clamp(0.0, 1.0);
            if k > 0.0 { self.px(x, y, renk, k * a); }
        } }
    }
    pub fn elips(&mut self, cx: f32, cy: f32, rx: f32, ry: f32, aci: f32, renk: Renk, a: f32) { crate::dekor::elips(self.t, self.w, self.h, cx, cy, rx, ry, aci, renk, a) }
    /// Piksel (bitmap) karakter: her hücre LED pikseli gibi aralıklı kare. `ayna`: yatay çevir.
    pub fn sprite(&mut self, satirlar: &[&str], x: f32, y: f32, hx: f32, hy: f32, ayna: bool, renk: &dyn Fn(char) -> Option<Renk>) {
        let gen = satirlar.iter().map(|s| s.chars().count()).max().unwrap_or(0);
        for (r, s) in satirlar.iter().enumerate() {
            for (c, ch) in s.chars().enumerate() {
                let Some(rk) = renk(ch) else { continue };
                let cc = if ayna { gen - 1 - c } else { c };
                let (px, py) = (x + cc as f32 * hx, y + r as f32 * hy);
                self.dikd(px + hx * 0.06, py + hy * 0.06, px + hx * 0.94, py + hy * 0.94, rk, 1.0);
            }
        }
    }
}

// ---------------------------------------------------------------- 5x7 piksel yazı tipi
fn glif(c: char) -> ([u8; 7], u8) {
    // işaret: 0 yok, 1 nokta (İ), 2 iki nokta (Ö Ü), 3 kısa çizgi (Ğ), 4 çengel (Ç Ş)
    let g = |s: [u8; 7]| s;
    let taban = |c: char| -> [u8; 7] { match c {
        'A' => g([14, 17, 17, 31, 17, 17, 17]), 'B' => g([30, 17, 17, 30, 17, 17, 30]), 'C' => g([14, 17, 16, 16, 16, 17, 14]), 'D' => g([30, 17, 17, 17, 17, 17, 30]),
        'E' => g([31, 16, 16, 30, 16, 16, 31]), 'F' => g([31, 16, 16, 30, 16, 16, 16]), 'G' => g([14, 17, 16, 23, 17, 17, 15]), 'H' => g([17, 17, 17, 31, 17, 17, 17]),
        'I' => g([14, 4, 4, 4, 4, 4, 14]), 'J' => g([7, 2, 2, 2, 2, 18, 12]), 'K' => g([17, 18, 20, 24, 20, 18, 17]), 'L' => g([16, 16, 16, 16, 16, 16, 31]),
        'M' => g([17, 27, 21, 21, 17, 17, 17]), 'N' => g([17, 17, 25, 21, 19, 17, 17]), 'O' => g([14, 17, 17, 17, 17, 17, 14]), 'P' => g([30, 17, 17, 30, 16, 16, 16]),
        'Q' => g([14, 17, 17, 17, 21, 18, 13]), 'R' => g([30, 17, 17, 30, 20, 18, 17]), 'S' => g([15, 16, 16, 14, 1, 1, 30]), 'T' => g([31, 4, 4, 4, 4, 4, 4]),
        'U' => g([17, 17, 17, 17, 17, 17, 14]), 'V' => g([17, 17, 17, 17, 17, 10, 4]), 'W' => g([17, 17, 17, 21, 21, 21, 10]), 'X' => g([17, 17, 10, 4, 10, 17, 17]),
        'Y' => g([17, 17, 10, 4, 4, 4, 4]), 'Z' => g([31, 1, 2, 4, 8, 16, 31]),
        '0' => g([14, 17, 19, 21, 25, 17, 14]), '1' => g([4, 12, 4, 4, 4, 4, 14]), '2' => g([14, 17, 1, 2, 4, 8, 31]), '3' => g([31, 2, 4, 2, 1, 17, 14]),
        '4' => g([2, 6, 10, 18, 31, 2, 2]), '5' => g([31, 16, 30, 1, 1, 17, 14]), '6' => g([6, 8, 16, 30, 17, 17, 14]), '7' => g([31, 1, 2, 4, 8, 8, 8]),
        '8' => g([14, 17, 17, 14, 17, 17, 14]), '9' => g([14, 17, 17, 15, 1, 2, 12]),
        '%' => g([24, 25, 2, 4, 8, 19, 3]), ':' => g([0, 12, 12, 0, 12, 12, 0]), '.' => g([0, 0, 0, 0, 0, 12, 12]), ',' => g([0, 0, 0, 0, 12, 4, 8]),
        '!' => g([4, 4, 4, 4, 4, 0, 4]), '?' => g([14, 17, 1, 2, 4, 0, 4]), '-' => g([0, 0, 0, 31, 0, 0, 0]), '+' => g([0, 4, 4, 31, 4, 4, 0]),
        '/' => g([1, 1, 2, 4, 8, 16, 16]), '\'' => g([4, 4, 8, 0, 0, 0, 0]), '*' => g([0, 21, 14, 31, 14, 21, 0]), '>' => g([8, 4, 2, 1, 2, 4, 8]),
        '<' => g([2, 4, 8, 16, 8, 4, 2]), '=' => g([0, 0, 31, 0, 31, 0, 0]), '(' => g([2, 4, 8, 8, 8, 4, 2]), ')' => g([8, 4, 2, 2, 2, 4, 8]),
        '&' => g([12, 18, 20, 8, 21, 18, 13]), '₺' => g([12, 4, 6, 12, 6, 5, 6]), '♥' => g([10, 31, 31, 31, 14, 4, 0]), '$' => g([4, 15, 20, 14, 5, 30, 4]),
        _ => [0; 7],
    } };
    match c { 'Ç' => (taban('C'), 4), 'Ş' => (taban('S'), 4), 'Ğ' => (taban('G'), 3), 'İ' => (taban('I'), 1), 'Ö' => (taban('O'), 2), 'Ü' => (taban('U'), 2), c => (taban(c), 0) }
}

/// Türkçe büyük harf (i→İ, ı→I).
pub fn buyuk(s: &str) -> String { s.chars().flat_map(|c| match c { 'i' => vec!['İ'], 'ı' => vec!['I'], c => c.to_uppercase().collect() }).collect() }
pub fn piksel_gen(s: &str, hucre: f32) -> f32 { let n = s.chars().count() as f32; if n == 0.0 { 0.0 } else { (n * 6.0 - 1.0) * hucre } }
/// Genişliğe sığan hücre boyu.
fn sigdir(s: &str, gen: f32, en_cok: f32) -> f32 { let n = s.chars().count().max(1) as f32; (gen / (n * 6.0 - 1.0)).min(en_cok).max(1.0) }

/// Bir glifin yanan noktaları (sütun, satır); satır -1 üst işaret, 7 alt çengel.
fn glif_noktalari(c: char) -> Vec<(i32, i32)> {
    let (b, isaret) = glif(c);
    let mut v = vec![];
    for (r, satir) in b.iter().enumerate() { for col in 0..5 { if satir & (16 >> col) != 0 { v.push((col, r as i32)); } } }
    match isaret { 1 => v.push((2, -1)), 2 => { v.push((1, -1)); v.push((3, -1)); } 3 => { v.push((1, -1)); v.push((2, -1)); v.push((3, -1)); } 4 => v.push((2, 7)), _ => {} }
    v
}

impl<'a> Tuval<'a> {
    /// Piksel yazı; `x0` sol kenar, `cy` dikey merkez. Noktalar LED gibi yuvarlak. `kolon(col)` her sütuna dikey kaydırma/renk verebilir.
    pub fn piksel_yazi_x(&mut self, s: &str, x0: f32, cy: f32, hucre: f32, renk: Renk, a: f32) { self.piksel_yazi_f(s, x0, cy, hucre, &|_| (0.0, renk), a) }
    pub fn piksel_yazi_f(&mut self, s: &str, x0: f32, cy: f32, hucre: f32, kolon: &dyn Fn(i32) -> (f32, Renk), a: f32) {
        let y0 = cy - 3.5 * hucre;
        let r = if hucre >= 5.0 { hucre * 0.44 } else { 0.0 };
        for (i, c) in s.chars().enumerate() {
            let gx = x0 + i as f32 * 6.0 * hucre;
            if gx > self.w as f32 || gx + 5.0 * hucre < 0.0 || c == ' ' { continue; }
            for (col, row) in glif_noktalari(c) {
                let kc = i as i32 * 6 + col;
                let (dy, rk) = kolon(kc);
                let (px, py) = (gx + col as f32 * hucre, y0 + row as f32 * hucre + dy);
                if r > 0.0 { self.daire(px + hucre / 2.0, py + hucre / 2.0, r, rk, a); } else { self.dikd(px, py, px + hucre.max(1.0), py + hucre.max(1.0), rk, a); }
            }
        }
    }
    pub fn piksel_yazi(&mut self, s: &str, cx: f32, cy: f32, hucre: f32, renk: Renk, a: f32) { self.piksel_yazi_x(s, cx - piksel_gen(s, hucre) / 2.0, cy, hucre, renk, a) }
    /// Genişliğe sığdırılmış ortalı piksel yazı; dönüş kullanılan hücre.
    pub fn piksel_sigdir(&mut self, s: &str, cx: f32, cy: f32, gen: f32, en_cok: f32, renk: Renk, a: f32) -> f32 { let h = sigdir(s, gen, en_cok); self.piksel_yazi(s, cx, cy, h, renk, a); h }
    /// Çok satırlı (kelime kelime) ortalı piksel yazı.
    pub fn piksel_satirlar(&mut self, s: &str, cx: f32, cy: f32, gen: f32, en_cok: f32, renk: Renk, a: f32) {
        let satirlar = crate::senaryo::dikey_satirlar(s);
        if satirlar.is_empty() { return; }
        let h = satirlar.iter().map(|x| sigdir(x, gen, en_cok)).fold(en_cok, f32::min);
        let ara = h * 10.0;
        let y0 = cy - ara * (satirlar.len() as f32 - 1.0) / 2.0;
        for (i, x) in satirlar.iter().enumerate() { self.piksel_yazi(x, cx, y0 + i as f32 * ara, h, renk, a); }
    }
}

// ---------------------------------------------------------------- özgün karakterler
/// Yutkun (dalga 3 özgünleştirme): yuvarlatılmış KARE gövdeli mercan-turuncu yaratık; ağız kama değil, öne açılan
/// DİKDÖRTGEN çene yarığı (üst/alt çene), başında yeşil filiz, kalın koyu dış çizgi, 3 tonlu gölge. Top/kama silüeti yok.
pub fn yutkun(tv: &mut Tuval, cx: f32, cy: f32, r: f32, yon: f32, agiz: f32) {
    const GOVDE: Renk = Renk(255, 122, 64);
    const GOLGE: Renk = Renk(170, 66, 30);
    const ISIK: Renk = Renk(255, 196, 150);
    let sag = yon.cos() >= 0.0;
    let s = if sag { 1.0 } else { -1.0 };
    let dikey = yon.sin().abs() > 0.7;
    // dış çizgi + gölge + gövde (sert gölge sağ alt)
    tv.yuvarlak(cx, cy, r * 1.06, r * 1.0, r * 0.42, KOYU, 1.0);
    tv.yuvarlak(cx + r * 0.08, cy + r * 0.08, r * 0.94, r * 0.88, r * 0.36, GOLGE, 1.0);
    tv.yuvarlak(cx, cy, r * 0.94, r * 0.88, r * 0.36, GOVDE, 1.0);
    tv.yuvarlak(cx - r * 0.3, cy - r * 0.5, r * 0.36, r * 0.12, r * 0.1, ISIK, 1.0);
    // ağız: öne doğru dikdörtgen yarık; açıklık 0..1
    let ac = agiz.clamp(0.0, 1.0);
    let ay = r * (0.06 + 0.34 * ac);
    if dikey {
        let sy = if yon.sin() > 0.0 { 1.0 } else { -1.0 };
        tv.dikd(cx - ay, cy + sy * r * 0.15 - if sy < 0.0 { r * 0.95 } else { 0.0 }, cx + ay, cy + sy * r * 0.15 + if sy > 0.0 { r * 0.95 } else { 0.0 }, KOYU, 1.0);
    } else {
        let (x0, x1) = if sag { (cx + r * 0.05, cx + r * 1.1) } else { (cx - r * 1.1, cx - r * 0.05) };
        tv.dikd(x0, cy + r * 0.12 - ay, x1, cy + r * 0.12 + ay, KOYU, 1.0);
        // dişler (beyaz küçük kareler, üst çenede)
        for i in 0..2 { let x = cx + s * r * (0.45 + i as f32 * 0.3); tv.dikd(x - r * 0.07, cy + r * 0.12 - ay, x + r * 0.07, cy + r * 0.12 - ay + r * 0.12, BEYAZ, 1.0); }
    }
    // tek büyük göz (beyaz + koyu bebek), yöne bakar
    let (gx, gy) = (cx + s * r * 0.18, cy - r * 0.36);
    tv.daire(gx, gy, r * 0.24, BEYAZ, 1.0);
    tv.daire(gx + s * r * 0.08, gy, r * 0.12, KOYU, 1.0);
    // filiz
    tv.cizgi(cx - s * r * 0.2, cy - r * 0.86, cx - s * r * 0.3, cy - r * 1.22, r * 0.1, Renk(60, 170, 60), 1.0);
    tv.elips(cx - s * r * 0.08, cy - r * 1.22, r * 0.26, r * 0.11, -0.5 * s, Renk(110, 230, 90), 1.0);
}

/// Kovalayıcı (dalga 3 özgünleştirme): KARE kafalı robot-jöle; kubbe başlık ve dalgalı etek YOK. Yuvarlatılmış kare gövde,
/// yatay vizör (göz şeridi), tek kısa anten topu, iki YUVARLAK ayak (yürürken sırayla kalkar), kalın koyu dış çizgi.
pub fn kovalayici(tv: &mut Tuval, cx: f32, cy: f32, r: f32, renk: Renk, bak: (f32, f32), tt: f32) {
    let golge = kos(renk, 0.6);
    // anten
    tv.cizgi(cx, cy - r * 0.9, cx, cy - r * 1.25, r * 0.1, KOYU, 1.0);
    tv.daire(cx, cy - r * 1.3, r * 0.15, BEYAZ, 1.0);
    // ayaklar
    for (i, sx) in [-0.48f32, 0.48].iter().enumerate() {
        let kalk = ((tt * 8.0 + i as f32 * PI).sin().max(0.0)) * r * 0.16;
        tv.daire(cx + sx * r, cy + r * 0.9 - kalk, r * 0.3, KOYU, 1.0);
        tv.daire(cx + sx * r, cy + r * 0.9 - kalk, r * 0.22, golge, 1.0);
    }
    // gövde: dış çizgi, gölge, yüz
    tv.yuvarlak(cx, cy, r * 1.0, r * 0.95, r * 0.18, KOYU, 1.0);
    tv.yuvarlak(cx + r * 0.06, cy + r * 0.06, r * 0.88, r * 0.83, r * 0.14, golge, 1.0);
    tv.yuvarlak(cx, cy, r * 0.88, r * 0.83, r * 0.14, renk, 1.0);
    // vizör
    tv.yuvarlak(cx, cy - r * 0.2, r * 0.7, r * 0.24, r * 0.08, KOYU, 1.0);
    for s in [-1.0f32, 1.0] { tv.dikd(cx + s * r * 0.32 + bak.0 * r * 0.16 - r * 0.12, cy - r * 0.32 + bak.1 * r * 0.06, cx + s * r * 0.32 + bak.0 * r * 0.16 + r * 0.12, cy - r * 0.08 + bak.1 * r * 0.06, BEYAZ, 1.0); }
    // ağız ızgarası
    for i in 0..3 { let x = cx + (i as f32 - 1.0) * r * 0.3; tv.dikd(x - r * 0.08, cy + r * 0.3, x + r * 0.08, cy + r * 0.5, KOYU, 1.0); }
}

const KOVALAYICI_RENK: [Renk; 4] = [Renk(170, 90, 255), Renk(60, 220, 120), Renk(255, 70, 140), Renk(40, 150, 255)];

/// Yutkun koridoru (dalga 3): mavi çizgili tüp DEĞİL; dolu koyu zemin şeridi + kenarda kalın açık bordür + enine şeritler.
fn koridor(tv: &mut Tuval, yol: &Yol, yari: f32, renk: Renk, u: f32) {
    let (zemin, serit, bordur) = (kos(renk, 0.26), kos(renk, 0.36), kar(renk, BEYAZ, 0.25));
    for i in 1..yol.p.len() { tv.cizgi(yol.p[i - 1].0, yol.p[i - 1].1, yol.p[i].0, yol.p[i].1, yari * 2.0, bordur, 1.0); }
    for i in 1..yol.p.len() { tv.cizgi(yol.p[i - 1].0, yol.p[i - 1].1, yol.p[i].0, yol.p[i].1, yari * 2.0 - 16.0 * u, zemin, 1.0); }
    let mut d = 0.0;
    while d < yol.uzun() {
        let (x, y, a) = yol.nokta(d);
        let (nx, ny) = (-a.sin(), a.cos());
        tv.cizgi(x + nx * (yari - 14.0 * u), y + ny * (yari - 14.0 * u), x - nx * (yari - 14.0 * u), y - ny * (yari - 14.0 * u), 10.0 * u, serit, 1.0);
        d += 46.0 * u;
    }
}

/// Yem: nokta değil küçük elmas (döndürülmüş kare).
fn yem(tv: &mut Tuval, x: f32, y: f32, r: f32, renk: Renk) { tv.cokgen(&[(x, y - r), (x + r, y), (x, y + r), (x - r, y)], renk, 1.0); }

const ROBI_A: [&str; 14] = [".....AA.....", ".....AA.....", "..KKKKKKKK..", "..KGGKKGGK..", "..KGSKKGSK..", "..KKKKKKKK..", "..KKSSSSKK..", "..KKKKKKKK..", ".KKKKKKKKKK.", "K.KKKKKKKK.K", "K.KKAAAAKK.K", "..KKKKKKKK..", "..KK....KK..", ".KK......KK."];
const ROBI_B: [&str; 14] = [".....AA.....", ".....AA.....", "..KKKKKKKK..", "..KGGKKGGK..", "..KGSKKGSK..", "..KKKKKKKK..", "..KKSSSSKK..", "..KKKKKKKK..", "KKKKKKKKKKKK", "..KKKKKKKK..", "..KKAAAAKK..", "..KKKKKKKK..", "...KK..KK...", "...KK..KK..."];
const ZIPZIP: [&str; 9] = ["..KKKKKK..", ".KKKKKKKK.", "KKGGKKGGKK", "KKGSKKGSKK", "KKKKKKKKKK", "KKKSSSSKKK", "KKKKKKKKKK", ".KKKKKKKK.", ".AA....AA."];
const UFOCUK_A: [&str; 7] = ["....KKK....", "..KKKKKKK..", ".KKGKKKGKK.", "KKKSKKKSKKK", "KKKKKKKKKKK", "A.A.A.A.A.A", ".A...A...A."];
const UFOCUK_B: [&str; 7] = ["....KKK....", "..KKKKKKK..", ".KKGKKKGKK.", "KKKSKKKSKKK", "KKKKKKKKKKK", ".A.A.A.A.A.", "A...A...A.A"];
const GEMI: [&str; 6] = [".....K.....", "....KKK....", "....KKK....", ".KKKKKKKKK.", "KKKKKKKKKKK", "KKKKKKKKKKK"];

fn renk_esle(k: Renk, a: Renk) -> impl Fn(char) -> Option<Renk> { move |c| match c { 'K' => Some(k), 'A' => Some(a), 'G' => Some(BEYAZ), 'S' => Some(KOYU), _ => None } }

// ---------------------------------------------------------------- hazırlık (yazı maskesi)
/// Rozet/logo yazısının sığacağı kutu (950 genişliğe göre px). None: gömülü yazı tipi kullanılmaz.
fn yazi_kutusu(id: &str) -> Option<(f32, f32)> {
    Some(match id {
        "rozet-yuzde" => (500.0, 300.0), "rozet-taksit" => (700.0, 400.0), "rozet-yeni" => (470.0, 240.0), "rozet-son-gunler" => (740.0, 250.0),
        "rozet-indirim" => (440.0, 300.0), "rozet-firsat" => (700.0, 280.0), "logo-perde" | "logo-iris" | "logo-parlama" => (800.0, 560.0),
        // dalga 3: 3B (TTF maske) yazılı malzemeler
        "yazi-3b-don" | "yazi-3b-derin" => (820.0, 560.0), "yazi-yil" => (820.0, 420.0), "altin-toz" | "isik-huzmesi" | "sahne-karli-isik" => (800.0, 480.0), "izgara-zemin" => (820.0, 380.0),
        "kar-katmanli" | "kar-birikme" | "kis-manzarasi" => (800.0, 380.0), "yilbasi-senlik" | "yilbasi-geri-sayim" => (820.0, 400.0),
        _ => return crate::malzeme_ek::yazi_kutusu(id),
    })
}

/// Sahne için yazı maskeleri (rozet/logo); diğer malzemelerde boş.
pub fn maskeler(font: &FontRef, w: u32, h: u32, id: &str, metin: &str) -> Vec<Maske> {
    let Some((bw, bh)) = yazi_kutusu(id) else { return vec![] };
    let metin = metin.trim();
    if metin.is_empty() { return vec![]; }
    let u = (w as f32 / 950.0).min(h as f32 / 1800.0);
    let (bw, bh) = (bw * u, bh * u);
    let satirlar: Vec<String> = if metin.contains('|') { metin.split('|').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect() } else if metin.chars().count() > 8 { crate::senaryo::dikey_satirlar(metin) } else { vec![metin.to_string()] };
    if satirlar.is_empty() { return vec![]; }
    let mut harf = satirlar.iter().map(|s| bw / crate::genislik(font, s, 100.0).max(1.0) * 100.0).fold(f32::MAX, f32::min);
    let sat100 = crate::satir_yuksekligi(font, 100.0) / 100.0;
    harf = harf.min(bh / satirlar.len() as f32 / sat100).max(4.0);
    vec![anim::maske(font, w, h, &satirlar, harf)]
}

fn yazi_bas(tv: &mut Tuval, z: &Hazir, yuz: Renk, yan: Renk, olcek: f32, aci: f32, dx: f32, dy: f32, derin: f32) {
    if let Some(m) = z.maskeler.first() {
        anim::kare3d_uzerine(tv.t, m, yuz, yan, &Poz { aci, olcek: olcek.max(0.02), parlak: 1.0, derinlik: derin, dx, dy });
    }
}

/// Rozet yazısı (dalga 3, Fable bulgusu "koyu yazı LED'de okunmuyor"): AÇIK yüz + kalın koyu dış çizgi (8 yönlü kopya) + koyu ekstrüzyon.
fn yazi_rozet(tv: &mut Tuval, z: &Hazir, olcek: f32, aci: f32, dx: f32, dy: f32, u: f32) {
    let kal = 9.0 * u * olcek.max(0.3);
    for (ox, oy) in [(-1.0f32, 0.0f32), (1.0, 0.0), (0.0, -1.0), (0.0, 1.0), (-0.7, -0.7), (0.7, -0.7), (-0.7, 0.7), (0.7, 0.7)] {
        yazi_bas(tv, z, KOYU, KOYU, olcek, aci, dx + ox * kal, dy + oy * kal, 1.0);
    }
    yazi_bas(tv, z, Renk(255, 252, 240), KOYU, olcek, aci, dx, dy, 7.0 * u);
}

/// Koyu dış çizgili açık piksel yazı (LED okunurluğu).
fn piksel_cizgili(tv: &mut Tuval, s: &str, cx: f32, cy: f32, gen: f32, en_cok: f32, renk: Renk) {
    let h = sigdir(s, gen, en_cok);
    let k = (h * 0.35).max(2.0);
    for (ox, oy) in [(-k, 0.0), (k, 0.0), (0.0, -k), (0.0, k), (-k, -k), (k, -k), (-k, k), (k, k)] { tv.piksel_yazi(s, cx + ox, cy + oy, h, KOYU, 1.0); }
    tv.piksel_yazi(s, cx, cy, h, renk, 1.0);
}

// ---------------------------------------------------------------- yol (labirent)
struct Yol { p: Vec<(f32, f32)>, l: Vec<f32> }
impl Yol {
    fn yeni(p: Vec<(f32, f32)>) -> Yol {
        let mut l = vec![0.0];
        for i in 1..p.len() { let d = ((p[i].0 - p[i - 1].0).powi(2) + (p[i].1 - p[i - 1].1).powi(2)).sqrt(); l.push(l[i - 1] + d); }
        Yol { p, l }
    }
    fn uzun(&self) -> f32 { *self.l.last().unwrap_or(&0.0) }
    /// s konumundaki nokta ve yön açısı.
    fn nokta(&self, s: f32) -> (f32, f32, f32) {
        let s = s.clamp(0.0, self.uzun());
        for i in 1..self.p.len() {
            if s <= self.l[i] || i == self.p.len() - 1 {
                let (a, b) = (self.p[i - 1], self.p[i]);
                let seg = (self.l[i] - self.l[i - 1]).max(1e-6);
                let k = ((s - self.l[i - 1]) / seg).clamp(0.0, 1.0);
                return (a.0 + (b.0 - a.0) * k, a.1 + (b.1 - a.1) * k, (b.1 - a.1).atan2(b.0 - a.0));
            }
        }
        (self.p[0].0, self.p[0].1, 0.0)
    }
    #[allow(dead_code)]
    fn tup(&self, tv: &mut Tuval, dis: f32, ic: f32, duvar: Renk, zemin: Renk) {
        for i in 1..self.p.len() { tv.cizgi(self.p[i - 1].0, self.p[i - 1].1, self.p[i].0, self.p[i].1, dis * 2.0, duvar, 1.0); }
        for i in 1..self.p.len() { tv.cizgi(self.p[i - 1].0, self.p[i - 1].1, self.p[i].0, self.p[i].1, ic * 2.0, zemin, 1.0); }
    }
}

/// Yumuşak, deterministik ses seviyesi (0..1).
fn seviye(i: usize, t: f32) -> f32 {
    let f = i as f32;
    let v = 0.5 + 0.28 * (t * (2.1 + hash(i as u32 * 7) * 2.0) + f * 0.9).sin() + 0.16 * (t * (5.3 + hash(i as u32 * 13) * 3.0) + f * f * 0.37).sin() + 0.1 * (t * 11.0 + f * 2.3).sin();
    v.clamp(0.04, 1.0)
}

// ---------------------------------------------------------------- ana çizici
/// Malzeme sahnesini kareye çizer. `yerel`: sahne içi zaman, `sure`: sahne süresi.
pub fn ciz(m: &MalzemeAyar, metin: &str, z: &Hazir, k: &mut [u8], w: u32, h: u32, yerel: f32, sure: f32) {
    let mut tv = Tuval { t: k, w, h };
    let (wf, hf) = (w as f32, h as f32);
    let u = (wf / 950.0).min(hf / 1800.0);
    let (cx, cy) = (wf / 2.0, hf / 2.0);
    let tt = yerel * m.hiz;
    let (yuz, vu, zem) = (z.yuz, z.vurgu, z.zemin);
    let yazi = buyuk(metin);
    // dalga 3 paketleri: kış/yılbaşı ve premium sahne/3B yazı (ayrı dosyalarda)
    if crate::malzeme_kis::ciz(m, &yazi, z, &mut tv, yerel, sure) || crate::malzeme_sahne::ciz(m, &yazi, z, &mut tv, yerel, sure) || crate::malzeme_ek::ciz(m, &yazi, z, &mut tv, yerel, sure) { return; }
    let dv = m.deger;
    let yon_aci = match m.yon.as_str() { "sol" => PI, "yukari" => -PI / 2.0, "asagi" => PI / 2.0, _ => 0.0 };
    match m.id.as_str() {
        // ------------------------------------------------ oyun
        "yutkun-labirent" => {
            let n = (dv.round() as usize).clamp(2, 6);
            let (ust, alt) = (hf * 0.24, hf * 0.94);
            let lh = (alt - ust) / n as f32;
            let (xl, xr) = (110.0 * u, wf - 110.0 * u);
            let mut p = vec![];
            for i in 0..n { let y = ust + lh * (i as f32 + 0.5); if i % 2 == 0 { p.push((xl, y)); p.push((xr, y)); } else { p.push((xr, y)); p.push((xl, y)); } }
            let yol = Yol::yeni(p);
            let r_dis = (lh * 0.42).min(118.0 * u);
            koridor(&mut tv, &yol, r_dis, yuz, u);
            let _ = zem;
            let v = 300.0 * u;
            let ara = r_dis * 1.9;
            let dongu = yol.uzun() + ara * 4.0 + 200.0 * u;
            let s = fr(tt * v / dongu) * dongu;
            let adim = 58.0 * u;
            let mut yenen = 0;
            let mut d = adim;
            while d < yol.uzun() {
                let (x, y, _) = yol.nokta(d);
                if d > s { yem(&mut tv, x, y, 13.0 * u, Renk(255, 230, 120)); } else { yenen += 1; }
                d += adim;
            }
            for i in 0..n {
                let (x, y, _) = yol.nokta(yol.l[i * 2 + 1].min(yol.uzun()));
                if yol.l[i * 2 + 1] > s { tv.yildiz(x, y, (30.0 + 4.0 * (tt * 3.0).sin()) * u, 0.5, 4, 0.0, Renk(120, 240, 255), 1.0); }
            }
            // karakter boyu ekran genişliğinin ≥%15'i (Fable bulgusu)
            let r = (r_dis * 0.78).max(wf * 0.075);
            for c in (0..3).rev() {
                let sc = s - ara * (c as f32 + 1.0);
                if sc < 0.0 || sc > yol.uzun() { continue; }
                let (x, y, a) = yol.nokta(sc);
                kovalayici(&mut tv, x, y, r * 0.95, KOVALAYICI_RENK[c], (a.cos(), a.sin()), tt + c as f32);
            }
            if s <= yol.uzun() { let (x, y, a) = yol.nokta(s); yutkun(&mut tv, x, y, r, a, (tt * 7.0).sin().abs()); }
            tv.piksel_yazi(&format!("PUAN {:04}", yenen * 10), cx, hf * 0.05, 12.0 * u, vu, 1.0);
            tv.piksel_sigdir(&yazi, cx, hf * 0.14, wf * 0.9, 22.0 * u, BEYAZ, 1.0);
        }
        "yutkun-kovalamaca" => {
            let adet = (dv.round() as usize).clamp(1, 4);
            for (sira, yc) in [(0usize, hf * 0.40), (1, hf * 0.62), (2, hf * 0.84)] {
                let r_dis = 140.0 * u;
                let duz = Yol::yeni(vec![(-300.0, yc), (wf + 300.0, yc)]);
                koridor(&mut tv, &duz, r_dis, yuz, u);
                let r = 92.0 * u;
                let ara = r * 2.7;
                let yol_l = wf + 2.0 * r + ara * adet as f32 + 120.0 * u;
                let v = 340.0 * u;
                let gecis = yol_l / v;
                let zt = tt + sira as f32 * gecis * 0.5;
                let tur = (zt / gecis).floor() as i64;
                let s = fr(zt / gecis) * yol_l;
                let sag = (tur + sira as i64) % 2 == 0;
                let x_of = |s: f32| if sag { -r + s } else { wf + r - s };
                let yon = if sag { 0.0 } else { PI };
                let mut x = 40.0 * u;
                while x < wf {
                    let gecildi = if sag { x < x_of(s) } else { x > x_of(s) };
                    if !gecildi { yem(&mut tv, x, yc, 13.0 * u, Renk(255, 230, 120)); }
                    x += 70.0 * u;
                }
                for c in 0..adet { let xc = x_of(s - ara * (c as f32 + 1.0)); kovalayici(&mut tv, xc, yc, r * 0.85, KOVALAYICI_RENK[(c + sira) % 4], (yon.cos(), 0.0), tt + c as f32); }
                yutkun(&mut tv, x_of(s), yc, r, yon, (tt * 7.0).sin().abs());
            }
            tv.piksel_satirlar(&yazi, cx, hf * 0.15, wf * 0.9, 26.0 * u, BEYAZ, 1.0);
        }
        "robi-yuruyus" => {
            let sol = m.yon == "sol";
            let hc = 24.0 * u;
            let gen = 12.0 * hc;
            let zemin_y = hf * 0.74;
            for i in 0..((wf / (40.0 * u)) as i32 + 2) { let x = fr(i as f32 * 40.0 * u / wf + if sol { tt * 0.05 } else { -tt * 0.05 }) * wf; tv.dikd(x, zemin_y, x + 22.0 * u, zemin_y + 10.0 * u, vu, 0.9); }
            let yol = wf + gen;
            let p = fr(tt * 170.0 * u / yol) * yol - gen;
            let x = if sol { wf - p - gen } else { p };
            let kare = (tt * 4.0) as i32 % 2 == 0;
            let zip = (tt * 8.0 * PI / 2.0).sin().abs() * 10.0 * u;
            let es = renk_esle(yuz, vu);
            tv.sprite(if kare { &ROBI_A } else { &ROBI_B }, x, zemin_y - 14.0 * hc - zip, hc, hc, sol, &es);
            tv.piksel_satirlar(&yazi, cx, hf * 0.3, wf * 0.9, 24.0 * u, yuz, 1.0);
        }
        "zipzip" => {
            let hc = 26.0 * u;
            let (gw, gh) = (10.0 * hc, 9.0 * hc);
            let zemin_y = hf * 0.82;
            let p = fr(tt * 0.75);
            let yuk = (p * PI).sin() * 620.0 * u;
            let ezik = if p < 0.07 || p > 0.93 { 1.0 } else { 0.0 };
            let (hx, hy) = (hc * (1.0 + 0.18 * ezik), hc * (1.0 - 0.22 * ezik));
            let yatay = (tt * 0.75 * 0.5).rem_euclid(2.0);
            let kx = if yatay < 1.0 { yatay } else { 2.0 - yatay };
            let x = 60.0 * u + kx * (wf - 120.0 * u - gw);
            let golge = 1.0 - yuk / (700.0 * u);
            tv.elips(x + gw / 2.0, zemin_y + 8.0 * u, gw * 0.45 * golge.max(0.3), 16.0 * u, 0.0, kos(yuz, 0.35), 1.0);
            let es = renk_esle(yuz, vu);
            tv.sprite(&ZIPZIP, x + (gw - 10.0 * hx) / 2.0, zemin_y - 9.0 * hy - yuk, hx, hy, false, &es);
            let _ = gh;
            tv.piksel_satirlar(&yazi, cx, hf * 0.2, wf * 0.9, 24.0 * u, yuz, 1.0);
        }
        "ufocuk-istila" => {
            let satir = (dv.round() as usize).clamp(1, 5);
            let hc = 13.0 * u;
            let (aw, ah) = (190.0 * u, 160.0 * u);
            let sut = 4;
            let ox = cx - aw * (sut as f32 - 1.0) / 2.0 - 5.5 * hc + (tt * 0.7).sin() * 120.0 * u;
            let oy = hf * 0.27;
            let kare = (tt * 2.0) as i32 % 2 == 0;
            for r in 0..satir { for c in 0..sut {
                let es = renk_esle(RENKLER[(r + 1) % 6], BEYAZ);
                tv.sprite(if kare { &UFOCUK_A } else { &UFOCUK_B }, ox + c as f32 * aw, oy + r as f32 * ah, hc, hc, false, &es);
            } }
            let gx = cx + (tt * 0.9).sin() * wf * 0.35;
            let gy = hf * 0.86;
            let es = renk_esle(yuz, vu);
            tv.sprite(&GEMI, gx - 5.5 * hc * 1.3, gy, hc * 1.3, hc * 1.3, false, &es);
            for i in 0..3 { let p = fr(tt * 0.9 + i as f32 / 3.0); let y = gy - p * hf * 0.55; tv.cizgi(gx, y, gx, y + 40.0 * u, 10.0 * u, BEYAZ, 1.0 - p * 0.5); }
            tv.piksel_satirlar(&yazi, cx, hf * 0.1, wf * 0.92, 22.0 * u, yuz, 1.0);
        }
        "yilancik" => {
            let hc = 64.0 * u;
            let (sut, sat) = (((wf - 60.0 * u) / hc) as i32, ((hf * 0.8) / hc) as i32);
            let (x0, y0) = ((wf - sut as f32 * hc) / 2.0, hf * 0.12);
            for y in 0..=sat { tv.dikd(x0, y0 + y as f32 * hc - 1.0, x0 + sut as f32 * hc, y0 + y as f32 * hc + 1.0, kos(yuz, 0.18), 1.0); }
            for x in 0..=sut { tv.dikd(x0 + x as f32 * hc - 1.0, y0, x0 + x as f32 * hc + 1.0, y0 + sat as f32 * hc, kos(yuz, 0.18), 1.0); }
            let toplam = (sut * sat).max(1);
            let hucre = |n: i32| { let n = n.rem_euclid(toplam); let r = n / sut; let c = if r % 2 == 0 { n % sut } else { sut - 1 - n % sut }; (x0 + c as f32 * hc, y0 + r as f32 * hc) };
            let adim = (tt * 7.0) as i32;
            let tur_bas = adim - adim.rem_euclid(toplam);
            let yenen = ((adim - tur_bas) / 23) as i32;
            let boy = 5 + yenen * 2;
            for e in 1..toplam / 23 + 1 { let n = tur_bas + e * 23; if n > adim { let (x, y) = hucre(n); tv.daire(x + hc / 2.0, y + hc / 2.0, hc * 0.36, Renk(255, 60, 60), 1.0); tv.cizgi(x + hc / 2.0, y + hc * 0.18, x + hc * 0.62, y + hc * 0.05, hc * 0.1, Renk(80, 200, 80), 1.0); } }
            for b in (0..boy).rev() {
                let n = adim - b;
                if n < tur_bas { continue; }
                let (x, y) = hucre(n);
                let rk = kar(yuz, kos(yuz, 0.55), b as f32 / boy as f32);
                tv.yuvarlak(x + hc / 2.0, y + hc / 2.0, hc * 0.46, hc * 0.46, hc * 0.18, rk, 1.0);
                if b == 0 { tv.daire(x + hc * 0.33, y + hc * 0.35, hc * 0.1, KOYU, 1.0); tv.daire(x + hc * 0.67, y + hc * 0.35, hc * 0.1, KOYU, 1.0); }
            }
            tv.piksel_yazi(&format!("UZUNLUK {boy}"), cx, hf * 0.06, 11.0 * u, vu, 1.0);
        }
        // ------------------------------------------------ kayan yazı
        "kayan-led" | "kayan-cift" => {
            let bantlar: Vec<(f32, String, Renk, bool)> = if m.id == "kayan-led" {
                vec![(cy, yazi.clone(), yuz, m.yon != "sag")]
            } else {
                let mut p = yazi.split('|');
                let a = p.next().unwrap_or("").trim().to_string();
                let b = p.next().map(|x| x.trim().to_string()).unwrap_or_else(|| a.clone());
                vec![(hf * 0.34, a, yuz, true), (hf * 0.66, b, vu, false)]
            };
            let hc = if m.id == "kayan-led" { 36.0 * u } else { 30.0 * u };
            for (by, s, rk, sola) in bantlar {
                let (ust, alt) = (by - 5.5 * hc, by + 5.5 * hc);
                tv.dikd(0.0, ust - 8.0 * u, wf, ust, vu, 1.0);
                tv.dikd(0.0, alt, wf, alt + 8.0 * u, vu, 1.0);
                let kol = (wf / hc) as i32 + 1;
                for c in 0..kol { for r in 0..7 { tv.daire(c as f32 * hc + hc / 2.0, by - 3.5 * hc + r as f32 * hc + hc / 2.0, hc * 0.36, kos(rk, 0.13), 1.0); } }
                let tw = piksel_gen(&s, hc) + 6.0 * hc;
                let ilerle = fr(tt * 260.0 * u / (wf + tw)) * (wf + tw);
                let x = if sola { wf - ilerle } else { ilerle - tw };
                let x = (x / hc).floor() * hc;
                tv.piksel_yazi_x(&s, x, by, hc, rk, 1.0);
            }
        }
        "dikey-akis" => {
            let kel: Vec<&str> = yazi.split_whitespace().collect();
            if !kel.is_empty() {
                let hc = kel.iter().map(|k| sigdir(k, wf * 0.86, 30.0 * u)).fold(30.0 * u, f32::min);
                let ara = hc * 12.0;
                let top = ara * kel.len() as f32;
                let kay = fr(tt * 150.0 * u / top.max(1.0)) * top;
                for tekrar in 0..((hf / top).ceil() as i32 + 2) { for (i, kk) in kel.iter().enumerate() {
                    let y = hf + ara * 0.5 + i as f32 * ara + tekrar as f32 * top - kay - top;
                    if y < -ara || y > hf + ara { continue; }
                    let kenar = (y.min(hf - y) / (hf * 0.18)).clamp(0.0, 1.0);
                    tv.piksel_yazi(kk, cx, y, hc, if i % 2 == 0 { yuz } else { vu }, kenar);
                } }
            }
        }
        "harf-harf" => {
            let hc = yazi.split_whitespace().map(|k| sigdir(k, wf * 0.9, 24.0 * u)).fold(24.0 * u, f32::min);
            let kol = ((wf * 0.9 / (6.0 * hc)) as usize).max(1);
            let mut satirlar: Vec<String> = vec![];
            for kk in yazi.split_whitespace() {
                if let Some(s) = satirlar.last_mut() { if s.chars().count() + 1 + kk.chars().count() <= kol { s.push(' '); s.push_str(kk); continue; } }
                satirlar.push(kk.chars().take(kol).collect());
            }
            let toplam: usize = satirlar.iter().map(|s| s.chars().count()).sum();
            let yaz_sn = toplam as f32 / 9.0;
            let dongu = yaz_sn + 2.5;
            let n = ((fr(tt / dongu) * dongu) * 9.0) as usize;
            let ara = hc * 11.0;
            let y0 = cy - ara * (satirlar.len() as f32 - 1.0) / 2.0;
            let mut kalan = n;
            let mut imlec = (cx, y0);
            for (i, s) in satirlar.iter().enumerate() {
                let goster: String = s.chars().take(kalan).collect();
                let x0 = cx - piksel_gen(s, hc) / 2.0;
                tv.piksel_yazi_x(&goster, x0, y0 + i as f32 * ara, hc, yuz, 1.0);
                let g = goster.chars().count();
                if kalan > 0 || i == 0 { imlec = (x0 + g as f32 * 6.0 * hc, y0 + i as f32 * ara); }
                kalan = kalan.saturating_sub(s.chars().count());
            }
            let yanik = 0.25 + 0.75 * (0.5 + 0.5 * (tt * TAU * 0.9).cos());
            tv.dikd(imlec.0, imlec.1 - 3.5 * hc, imlec.0 + 4.0 * hc, imlec.1 + 3.5 * hc, vu, yanik);
        }
        "dalgali-yazi" => {
            let hc = sigdir(&yazi, wf * 0.9, 34.0 * u);
            let x0 = cx - piksel_gen(&yazi, hc) / 2.0;
            let f = move |c: i32| -> (f32, Renk) { ((tt * 3.0 + c as f32 * 0.35).sin() * 90.0 * u, kar(yuz, vu, 0.5 + 0.5 * (c as f32 * 0.25 - tt * 2.0).sin())) };
            tv.piksel_yazi_f(&yazi, x0, cy, hc, &f, 1.0);
            for i in 0..2 { let yy = cy + (i as f32 * 2.0 - 1.0) * 360.0 * u; let mut x = 30.0 * u; while x < wf - 30.0 * u { let y = yy + (tt * 3.0 + x / (60.0 * u)).sin() * 30.0 * u; tv.daire(x, y, 9.0 * u, vu, 0.8); x += 34.0 * u; } }
        }
        "nabiz-yazi" => {
            let olcek = 1.0 + 0.08 * (tt * 2.4).sin();
            tv.piksel_satirlar(&yazi, cx, cy, wf * 0.88 * olcek, 40.0 * u * olcek, yuz, 1.0);
            tv.halka(cx, cy, 440.0 * u * olcek, 10.0 * u, vu, 0.8);
        }
        // ------------------------------------------------ zaman
        "geri-sayim" => {
            let kalan = (dv - yerel * m.hiz).max(0.0);
            let rr = 400.0 * u;
            tv.halka(cx, cy, rr, 40.0 * u, kos(yuz, 0.18), 1.0);
            if kalan > 0.0 {
                let n = kalan.ceil() as i64;
                let fk = kalan - kalan.floor();
                tv.yay(cx, cy, rr, 40.0 * u, -PI / 2.0, if fk == 0.0 { 1.0 } else { fk }, vu, 1.0);
                let s = n.to_string();
                let pop = 1.0 + 0.15 * fk.powi(8);
                let hc = sigdir(&s, rr * 1.35, 62.0 * u) * pop;
                tv.piksel_yazi(&s, cx, cy, hc, yuz, 1.0);
            } else {
                tv.halka(cx, cy, rr, 40.0 * u, vu, 1.0);
                let olcek = 1.0 + 0.06 * (tt * 3.0).sin();
                tv.piksel_satirlar(&yazi, cx, cy, rr * 1.5 * olcek, 40.0 * u, yuz, 1.0);
            }
        }
        "saat-dijital" => {
            let bas = if dv < 0.0 { let sn = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs() as i64).unwrap_or(0) + 3 * 3600; (sn.rem_euclid(86400)) as f32 } else { dv * 60.0 };
            let sn = (bas + yerel) as i64;
            let (s_, d_, n_) = ((sn / 3600) % 24, (sn / 60) % 60, sn % 60);
            let hc = sigdir("00:00", wf * 0.9, 30.0 * u);
            let iki_nokta = 0.35 + 0.65 * (0.5 + 0.5 * (yerel * TAU).cos());
            let x0 = cx - piksel_gen("00:00", hc) / 2.0;
            tv.piksel_yazi_x(&format!("{s_:02}"), x0, cy - 120.0 * u, hc, yuz, 1.0);
            tv.piksel_yazi_x(":", x0 + 12.0 * hc, cy - 120.0 * u, hc, yuz, iki_nokta);
            tv.piksel_yazi_x(&format!("{d_:02}"), x0 + 18.0 * hc, cy - 120.0 * u, hc, yuz, 1.0);
            let hs = 16.0 * u;
            tv.piksel_yazi(&format!("{n_:02}"), cx, cy + 150.0 * u, hs, vu, 1.0);
            tv.yay(cx, cy + 150.0 * u, 140.0 * u, 16.0 * u, -PI / 2.0, (n_ as f32 + fr(bas + yerel)) / 60.0, vu, 1.0);
            let (y, a, g) = crate::senaryo::tarih_ayir(crate::senaryo::bugun_gun_no());
            tv.piksel_yazi(&format!("{g:02}.{a:02}.{y}"), cx, cy + 420.0 * u, 13.0 * u, yuz, 0.85);
        }
        "tarih-takvim" => {
            let gun = crate::senaryo::bugun_gun_no() + dv.round() as i64;
            let (_, ay, g) = crate::senaryo::tarih_ayir(gun);
            const AYLAR: [&str; 12] = ["OCAK", "ŞUBAT", "MART", "NİSAN", "MAYIS", "HAZİRAN", "TEMMUZ", "AĞUSTOS", "EYLÜL", "EKİM", "KASIM", "ARALIK"];
            const GUNLER: [&str; 7] = ["PAZARTESİ", "SALI", "ÇARŞAMBA", "PERŞEMBE", "CUMA", "CUMARTESİ", "PAZAR"];
            let ac = anim::geri((yerel / 0.7).min(1.0));
            let (hw, hh) = (360.0 * u, 470.0 * u * ac.max(0.02));
            tv.yuvarlak(cx, cy, hw, hh, 40.0 * u, BEYAZ, 1.0);
            tv.yuvarlak(cx, cy - hh + 110.0 * u * ac, hw, 110.0 * u * ac.max(0.02), 40.0 * u, vu, 1.0);
            tv.dikd(cx - hw, cy - hh + 110.0 * u * ac, cx + hw, cy - hh + 220.0 * u * ac, vu, 1.0);
            if ac > 0.6 {
                for s in [-1.0f32, 1.0] { tv.daire(cx + s * 200.0 * u, cy - hh, 26.0 * u, kos(zem, 1.0), 1.0); tv.cizgi(cx + s * 200.0 * u, cy - hh - 50.0 * u, cx + s * 200.0 * u, cy - hh + 10.0 * u, 18.0 * u, Renk(150, 150, 150), 1.0); }
                tv.piksel_sigdir(AYLAR[(ay - 1).clamp(0, 11) as usize], cx, cy - hh + 140.0 * u, hw * 1.6, 17.0 * u, BEYAZ, 1.0);
                let gs = g.to_string();
                tv.piksel_sigdir(&gs, cx, cy + 60.0 * u, hw * 1.5, 50.0 * u, KOYU, 1.0);
                tv.piksel_sigdir(GUNLER[(gun + 3).rem_euclid(7) as usize], cx, cy + hh - 90.0 * u, hw * 1.7, 13.0 * u, vu, 1.0);
            }
        }
        "ilerleme-cubugu" => {
            let p = anim::yumusak((yerel * m.hiz / (sure * 0.8).max(0.5)).min(1.0)) * dv / 100.0;
            let (hw, hh) = (wf * 0.42, 70.0 * u);
            tv.yuvarlak(cx, cy, hw + 14.0 * u, hh + 14.0 * u, hh + 14.0 * u, yuz, 1.0);
            tv.yuvarlak(cx, cy, hw, hh, hh, zem, 1.0);
            let fw = (hw - 10.0 * u) * 2.0 * p;
            if fw > 1.0 {
                let (x0, x1) = (cx - hw + 10.0 * u, cx - hw + 10.0 * u + fw);
                tv.yuvarlak((x0 + x1) / 2.0, cy, fw / 2.0, hh - 10.0 * u, hh - 10.0 * u, vu, 1.0);
                let seritler = ((x1 - x0) / (60.0 * u)) as i32 + 3;
                for i in -1..seritler {
                    let sx = x0 + i as f32 * 60.0 * u + fr(tt * 0.8) * 60.0 * u;
                    let pts = [(sx, cy + hh - 10.0 * u), (sx + 26.0 * u, cy + hh - 10.0 * u), (sx + 26.0 * u + 60.0 * u, cy - hh + 10.0 * u), (sx + 60.0 * u, cy - hh + 10.0 * u)];
                    let pts: Vec<(f32, f32)> = pts.iter().map(|q| (sinir(q.0, x0 + 6.0 * u, x1 - 6.0 * u), q.1)).collect();
                    tv.cokgen(&pts, kos(vu, 0.75), 1.0);
                }
            }
            tv.piksel_yazi(&format!("%{}", (p * 100.0).round() as i32), cx, cy - 230.0 * u, 26.0 * u, yuz, 1.0);
            tv.piksel_sigdir(&yazi, cx, cy + 240.0 * u, wf * 0.86, 16.0 * u, vu, 1.0);
        }
        "yuzde-sayaci" => {
            let p = anim::yumusak((yerel * m.hiz / (sure * 0.7).max(0.5)).min(1.0)) * dv / 100.0;
            let rr = 360.0 * u;
            tv.halka(cx, cy - 80.0 * u, rr, 60.0 * u, kos(yuz, 0.16), 1.0);
            tv.yay(cx, cy - 80.0 * u, rr, 60.0 * u, -PI / 2.0, p, vu, 1.0);
            let s = format!("%{}", (p * 100.0).round() as i32);
            tv.piksel_sigdir(&s, cx, cy - 80.0 * u, rr * 1.3, 40.0 * u, yuz, 1.0);
            tv.piksel_sigdir(&yazi, cx, cy + 420.0 * u, wf * 0.88, 18.0 * u, yuz, 1.0);
        }
        "kum-saati" => {
            let p = (yerel * m.hiz / sure).clamp(0.0, 1.0);
            let (hw, hh) = (300.0 * u, 560.0 * u);
            let cam = kos(yuz, 0.9);
            tv.dikd(cx - hw - 40.0 * u, cy - hh - 40.0 * u, cx + hw + 40.0 * u, cy - hh, vu, 1.0);
            tv.dikd(cx - hw - 40.0 * u, cy + hh, cx + hw + 40.0 * u, cy + hh + 40.0 * u, vu, 1.0);
            let kum = Renk(255, 196, 90);
            // üst hazne: tepe tarafı boşalır
            let ust_y = cy - hh * (1.0 - p).sqrt() * 0.95;
            let gu = hw * (cy - ust_y) / hh;
            if p < 1.0 { tv.cokgen(&[(cx - gu, ust_y), (cx + gu, ust_y), (cx, cy)], kum, 1.0); }
            // alt hazne: tepecik
            let yuk = hh * 0.92 * p.sqrt();
            tv.cokgen(&[(cx - hw * 0.95, cy + hh), (cx + hw * 0.95, cy + hh), (cx + hw * 0.95 * (1.0 - p * 0.5), cy + hh - yuk * 0.4), (cx, cy + hh - yuk), (cx - hw * 0.95 * (1.0 - p * 0.5), cy + hh - yuk * 0.4)], kum, 1.0);
            if p < 1.0 { for i in 0..10 { let y = cy + fr(tt * 1.6 + i as f32 * 0.1) * (hh - yuk); tv.daire(cx, y, 7.0 * u, kum, 1.0); } }
            for s in [-1.0f32, 1.0] { tv.cizgi(cx + s * hw, cy - hh, cx + s * 18.0 * u, cy, 16.0 * u, cam, 1.0); tv.cizgi(cx + s * 18.0 * u, cy, cx + s * hw, cy + hh, 16.0 * u, cam, 1.0); }
        }
        // ------------------------------------------------ kutlama
        "havai-fisek-gosteri" => {
            let n = (dv.round() as usize).clamp(1, 10);
            let donem = 3.2;
            for gecikme in [0.0f32, donem / 2.0] {
                let zt = tt + gecikme;
                let tur = (zt / donem).floor() as u32;
                let yt = fr(zt / donem) * donem;
                for i in 0..n {
                    let tohum = tur * 131 + i as u32 * 17 + (gecikme * 10.0) as u32;
                    let bas = i as f32 / n as f32 * 1.2;
                    let lt = yt - bas;
                    if lt < 0.0 { continue; }
                    let x = wf * (0.15 + 0.7 * hash(tohum));
                    let tepe = hf * (0.15 + 0.3 * hash(tohum + 1));
                    let rk = RENKLER[(hash(tohum + 2) * 6.0) as usize % 6];
                    if lt < 0.8 {
                        let y = hf + (tepe - hf) * anim::yumusak(lt / 0.8);
                        tv.cizgi(x, y, x, y + 60.0 * u, 8.0 * u, Renk(255, 230, 170), 1.0);
                        tv.daire(x, y, 9.0 * u, BEYAZ, 1.0);
                    } else if lt < 2.4 {
                        let e = (lt - 0.8) / 1.6;
                        let rr = 330.0 * u * (1.0 - (1.0 - e).powi(3));
                        let sol = (1.0 - e).powf(0.7);
                        for p in 0..30 {
                            let a = p as f32 / 30.0 * TAU + hash(tohum + 9) * 0.5;
                            let (px, py) = (x + a.cos() * rr, tepe + a.sin() * rr + e * e * 160.0 * u);
                            let (qx, qy) = (x + a.cos() * rr * 0.82, tepe + a.sin() * rr * 0.82 + e * e * 140.0 * u);
                            tv.cizgi(qx, qy, px, py, 9.0 * u, rk, sol);
                            tv.daire(px, py, 9.0 * u, kar(rk, BEYAZ, 0.4), sol);
                        }
                    }
                }
            }
            tv.piksel_satirlar(&yazi, cx, hf * 0.82, wf * 0.9, 28.0 * u, yuz, 1.0);
        }
        "konfeti-patlama" => {
            let n = (dv.round() as usize).clamp(20, 200);
            tv.piksel_satirlar(&yazi, cx, cy, wf * 0.9, 34.0 * u, yuz, 1.0);
            let donem = 3.0;
            let tur = (tt / donem).floor() as u32;
            let lt = fr(tt / donem) * donem;
            for taraf in [0u32, 1] { for i in 0..n / 2 {
                let tohum = tur * 977 + taraf * 7919 + i as u32 * 31;
                let (vx, vy) = ((250.0 + 550.0 * hash(tohum)) * u, -(1450.0 + 1000.0 * hash(tohum + 1)) * u);
                let x0 = if taraf == 0 { -20.0 } else { wf + 20.0 };
                // balistik yükseliş, tepeden sonra hava direnciyle yavaş süzülme
                let g = 1500.0 * u;
                let tepe_t = -vy / g;
                let py = if lt < tepe_t { hf + vy * lt + g * lt * lt * 0.5 } else { hf - vy * vy / (2.0 * g) + (lt - tepe_t) * 260.0 * u };
                let px = x0 + (if taraf == 0 { 1.0 } else { -1.0 }) * vx * (1.0 - (-lt / 0.8).exp()) * 0.8 + (lt * 3.0 + hash(tohum + 6) * TAU).sin() * 40.0 * u * (lt / 1.5).min(1.0);
                if py > hf + 40.0 { continue; }
                let a = lt * (4.0 + hash(tohum + 3) * 8.0) + hash(tohum + 4) * TAU;
                let (bw, bh) = (22.0 * u, 11.0 * u * a.cos().abs().max(0.2));
                let pts: Vec<(f32, f32)> = [(-bw, -bh), (bw, -bh), (bw, bh), (-bw, bh)].iter().map(|q| { let r = don(*q, a * 0.6); (px + r.0, py + r.1) }).collect();
                tv.cokgen(&pts, RENKLER[(hash(tohum + 5) * 6.0) as usize % 6], 1.0);
            } }
        }
        "balon-kutlama" => {
            let n = (dv.round() as usize).clamp(3, 15);
            for i in 0..n {
                let tohum = i as u32 * 53 + 5;
                let hiz = (110.0 + 90.0 * hash(tohum)) * u;
                let r = (70.0 + 40.0 * hash(tohum + 1)) * u;
                let yol = hf + r * 5.0;
                let y = hf + r * 1.4 - fr((tt * hiz) / yol + hash(tohum + 2)) * yol;
                let x = wf * (0.08 + 0.84 * hash(tohum + 3)) + (tt * 1.1 + i as f32).sin() * 30.0 * u;
                let rk = RENKLER[i % 6];
                for s in 0..14 { let sy = y + r * 1.25 + s as f32 * 14.0 * u; tv.daire(x + (sy * 0.02 + tt * 2.0).sin() * 10.0 * u, sy, 3.0 * u, Renk(200, 200, 200), 1.0); }
                tv.cokgen(&[(x, y + r * 1.15), (x - 14.0 * u, y + r * 1.32), (x + 14.0 * u, y + r * 1.32)], kos(rk, 0.8), 1.0);
                tv.elips(x, y, r, r * 1.2, 0.0, rk, 1.0);
                tv.elips(x - r * 0.35, y - r * 0.45, r * 0.18, r * 0.3, 0.5, kar(rk, BEYAZ, 0.6), 1.0);
            }
            tv.piksel_satirlar(&yazi, cx, cy, wf * 0.9, 32.0 * u, BEYAZ, 1.0);
        }
        "yildiz-patlama" => {
            let n = 14;
            for i in 0..n {
                let a0 = tt * 0.25 + i as f32 / n as f32 * TAU;
                let a1 = a0 + TAU / n as f32 * 0.5;
                let r = hf;
                tv.cokgen(&[(cx, cy), (cx + a0.cos() * r, cy + a0.sin() * r), (cx + a1.cos() * r, cy + a1.sin() * r)], kos(vu, 0.32), 1.0);
            }
            let olcek = 1.0 + 0.05 * (tt * 2.5).sin();
            tv.yildiz(cx, cy, 380.0 * u * olcek, 0.48, 5, tt * 0.2, yuz, 1.0);
            piksel_cizgili(&mut tv, &yazi, cx, cy + 40.0 * u, 380.0 * u, 26.0 * u, BEYAZ);
        }
        // ------------------------------------------------ hava
        "kar-yagisi" => {
            let n = (dv.round() as usize).clamp(10, 120);
            for i in 0..n {
                let tohum = i as u32 * 71 + 3;
                let r = (16.0 + 34.0 * hash(tohum)) * u;
                let hiz = (60.0 + r / u * 3.0) * u;
                let yol = hf + r * 4.0;
                let y = fr((tt * hiz) / yol + hash(tohum + 1)) * yol - r * 2.0;
                let x = hash(tohum + 2) * wf + (tt * 0.8 + i as f32).sin() * 40.0 * u;
                let a = tt * (0.3 + hash(tohum + 3)) + i as f32;
                let rk = kar(BEYAZ, yuz, 0.3);
                for k in 0..6 {
                    let b = a + k as f32 * PI / 3.0;
                    let (ex, ey) = (x + b.cos() * r, y + b.sin() * r);
                    tv.cizgi(x, y, ex, ey, (r * 0.16).max(2.0), rk, 1.0);
                    let (mx, my) = (x + b.cos() * r * 0.55, y + b.sin() * r * 0.55);
                    for s in [-1.0f32, 1.0] { let c = b + s * 0.7; tv.cizgi(mx, my, mx + c.cos() * r * 0.32, my + c.sin() * r * 0.32, (r * 0.12).max(2.0), rk, 1.0); }
                }
            }
        }
        "yagmur" => {
            let n = (dv.round() as usize).clamp(30, 300);
            let zemin_y = hf * 0.93;
            for i in 0..n {
                let tohum = i as u32 * 37 + 11;
                let hiz = (1300.0 + 600.0 * hash(tohum)) * u;
                let yol = zemin_y - hf * 0.12;
                let p = fr(tt * hiz / yol + hash(tohum + 1));
                let y = hf * 0.12 + p * yol;
                let x = hash(tohum + 2) * (wf + 200.0 * u) - 100.0 * u + p * 120.0 * u;
                tv.cizgi(x, y, x - 14.0 * u, y - 70.0 * u, 6.0 * u, kar(yuz, BEYAZ, 0.2), 0.9);
                let sp = fr(tt * hiz / yol + hash(tohum + 1) + 0.0);
                if sp < 0.12 { let e = sp / 0.12; let sx = hash(tohum + 2) * (wf + 200.0 * u) - 100.0 * u + 120.0 * u; tv.elips(sx, zemin_y, 50.0 * u * e + 4.0, 10.0 * u * e + 2.0, 0.0, yuz, (1.0 - e) * 0.8); }
            }
            tv.dikd(0.0, zemin_y, wf, hf, kos(yuz, 0.15), 1.0);
            for (bx, by, br) in [(0.25, 0.08, 150.0), (0.5, 0.06, 190.0), (0.78, 0.09, 150.0), (0.1, 0.1, 120.0), (0.92, 0.07, 120.0)] {
                tv.daire(wf * bx + (tt * 0.2).sin() * 20.0 * u, hf * by, br * u, Renk(120, 128, 140), 1.0);
            }
        }
        "parcacik-galaksi" => {
            let n = (dv.round() as usize).clamp(50, 400);
            for i in 0..n {
                let tohum = i as u32 * 29 + 1;
                let a = hash(tohum) * TAU;
                let z0 = fr(tt * 0.35 + hash(tohum + 1));
                let d = z0 * z0 * hf * 0.75;
                let d2 = (z0 - 0.04).max(0.0).powi(2) * hf * 0.75;
                let (x, y) = (cx + a.cos() * d, cy + a.sin() * d);
                let (x2, y2) = (cx + a.cos() * d2, cy + a.sin() * d2);
                let rk = if i % 5 == 0 { yuz } else { BEYAZ };
                tv.cizgi(x2, y2, x, y, (2.0 + 9.0 * z0) * u, rk, z0.min(1.0));
            }
        }
        "kabarcik" => {
            let n = (dv.round() as usize).clamp(5, 80);
            for i in 0..n {
                let tohum = i as u32 * 43 + 9;
                let r = (20.0 + 60.0 * hash(tohum)) * u;
                let hiz = (90.0 + 120.0 * hash(tohum + 1)) * u;
                let yol = hf + r * 3.0;
                let y = hf + r - fr(tt * hiz / yol + hash(tohum + 2)) * yol;
                let x = hash(tohum + 3) * wf + (tt * 1.4 + i as f32).sin() * 25.0 * u;
                tv.halka(x, y, r, (r * 0.12).max(3.0), yuz, 0.9);
                tv.daire(x - r * 0.35, y - r * 0.35, r * 0.18, BEYAZ, 0.8);
            }
        }
        // ------------------------------------------------ ikon
        "kalp-atisi" => {
            let ph = fr(tt * 1.1);
            let atis = 1.0 + 0.14 * (-((ph) / 0.06).powi(2)).exp() + 0.1 * (-((ph - 0.24) / 0.06).powi(2)).exp();
            tv.kalp(cx, cy - 160.0 * u, 300.0 * u * atis, yuz, 1.0);
            tv.elips(cx - 120.0 * u * atis, cy - 260.0 * u * atis, 50.0 * u, 30.0 * u, -0.6, kar(yuz, BEYAZ, 0.5), 0.8);
            let ey = cy + 330.0 * u;
            let kay = tt * 300.0 * u;
            let ekg = |x: f32| -> f32 { let p = (x + kay).rem_euclid(600.0 * u) / (600.0 * u); if (0.40..0.44).contains(&p) { -(p - 0.40) / 0.04 * 140.0 * u } else if (0.44..0.50).contains(&p) { -140.0 * u + (p - 0.44) / 0.06 * 220.0 * u } else if (0.50..0.53).contains(&p) { 80.0 * u - (p - 0.50) / 0.03 * 80.0 * u } else { 0.0 } };
            let mut x = 40.0 * u;
            while x < wf - 40.0 * u { let x2 = x + 8.0 * u; tv.cizgi(x, ey + ekg(x), x2, ey + ekg(x2), 10.0 * u, vu, 1.0); x = x2; }
            tv.piksel_sigdir(&yazi, cx, hf * 0.86, wf * 0.86, 22.0 * u, yuz, 1.0);
        }
        "kalp-yagmuru" => {
            let n = (dv.round() as usize).clamp(4, 40);
            let renkler = [yuz, vu, Renk(255, 60, 100), Renk(255, 140, 180)];
            for i in 0..n {
                let tohum = i as u32 * 61 + 7;
                let r = (40.0 + 80.0 * hash(tohum)) * u;
                let hiz = (120.0 + 120.0 * hash(tohum + 1)) * u;
                let yol = hf + r * 3.0;
                let y = hf + r * 1.5 - fr(tt * hiz / yol + hash(tohum + 2)) * yol;
                let x = hash(tohum + 3) * wf + (tt * 1.2 + i as f32).sin() * 50.0 * u;
                tv.kalp(x, y, r, renkler[i % 4], 1.0);
            }
        }
        "gulen-yuz" | "ask-yuz" => {
            let by = cy + (tt * 1.6).sin() * 18.0 * u;
            let r = 340.0 * u;
            tv.daire(cx, by, r + 12.0 * u, kos(SARI, 0.6), 1.0);
            tv.daire(cx, by, r, SARI, 1.0);
            if m.id == "gulen-yuz" {
                let kirp = fr(tt / 3.2);
                let acik = if kirp > 0.94 { ((kirp - 0.97).abs() / 0.03).clamp(0.08, 1.0) } else { 1.0 };
                for s in [-1.0f32, 1.0] { tv.elips(cx + s * 120.0 * u, by - 90.0 * u, 42.0 * u, 70.0 * u * acik, 0.0, KOYU, 1.0); }
                for s in [-1.0f32, 1.0] { tv.elips(cx + s * 215.0 * u, by + 40.0 * u, 50.0 * u, 30.0 * u, 0.0, Renk(255, 120, 110), 0.7); }
            } else {
                let nb = 1.0 + 0.12 * (tt * 4.0).sin().abs();
                for s in [-1.0f32, 1.0] { tv.kalp(cx + s * 125.0 * u, by - 85.0 * u, 75.0 * u * nb, Renk(235, 30, 60), 1.0); }
                for i in 0..6 { let p = fr(tt * 0.4 + i as f32 / 6.0); let x = cx + (i as f32 - 2.5) * 140.0 * u + (tt + i as f32).sin() * 20.0 * u; tv.kalp(x, by - r - p * 500.0 * u, 34.0 * u, Renk(255, 70, 110), 1.0 - p); }
            }
            tv.yay(cx, by + 20.0 * u, 200.0 * u, 34.0 * u, PI * 0.15, 0.35, KOYU, 1.0);
        }
        "onay-isareti" => {
            let rr = 330.0 * u;
            let (oy, p1, p2) = (cy - 100.0 * u, (yerel / 0.8).min(1.0), ((yerel - 0.8) / 0.6).clamp(0.0, 1.0));
            tv.daire(cx, oy, rr, kos(yuz, 0.16), 1.0);
            tv.yay(cx, oy, rr, 44.0 * u, -PI / 2.0, anim::yumusak(p1), yuz, 1.0);
            let nb = if p2 >= 1.0 { 1.0 + 0.04 * (tt * 3.0).sin() } else { 1.0 };
            let (a, b, c) = ((cx - 150.0 * u * nb, oy + 10.0 * u), (cx - 40.0 * u * nb, oy + 120.0 * u * nb), (cx + 170.0 * u * nb, oy - 120.0 * u * nb));
            let k1 = (p2 / 0.4).min(1.0);
            if p2 > 0.0 { tv.cizgi(a.0, a.1, a.0 + (b.0 - a.0) * k1, a.1 + (b.1 - a.1) * k1, 60.0 * u, yuz, 1.0); }
            if p2 > 0.4 { let k2 = ((p2 - 0.4) / 0.6).min(1.0); tv.cizgi(b.0, b.1, b.0 + (c.0 - b.0) * k2, b.1 + (c.1 - b.1) * k2, 60.0 * u, yuz, 1.0); }
            tv.piksel_sigdir(&yazi, cx, cy + 400.0 * u, wf * 0.86, 26.0 * u, yuz, p2);
        }
        "hediye-kutusu" => {
            let (bw, bh) = (300.0 * u, 260.0 * u);
            let by = cy + 120.0 * u;
            let ph = fr(tt * 0.6);
            let zip = if ph < 0.3 { (ph / 0.3 * PI).sin() * 160.0 * u } else { 0.0 };
            tv.dikd(cx - bw, by - bh, cx + bw, by + bh, vu, 1.0);
            tv.dikd(cx - 40.0 * u, by - bh, cx + 40.0 * u, by + bh, yuz, 1.0);
            let ly = by - bh - zip;
            if zip > 0.0 { for i in 0..7 { let a = -PI / 2.0 + (i as f32 - 3.0) * 0.32; let d = zip * 2.4; tv.yildiz(cx + a.cos() * d, by - bh + a.sin() * d, 34.0 * u, 0.45, 5, tt, RENKLER[i % 6], 1.0); } }
            tv.dikd(cx - bw - 30.0 * u, ly - 90.0 * u, cx + bw + 30.0 * u, ly, kos(vu, 0.85), 1.0);
            tv.dikd(cx - 40.0 * u, ly - 90.0 * u, cx + 40.0 * u, ly, yuz, 1.0);
            for s in [-1.0f32, 1.0] { tv.elips(cx + s * 85.0 * u, ly - 150.0 * u, 95.0 * u, 50.0 * u, s * 0.5, yuz, 1.0); }
            tv.daire(cx, ly - 120.0 * u, 34.0 * u, kos(yuz, 0.8), 1.0);
            tv.piksel_sigdir(&yazi, cx, hf * 0.15, wf * 0.86, 28.0 * u, yuz, 1.0);
        }
        "yildiz-parlak" => {
            for i in 0..14 {
                let tohum = i as u32 * 19 + 2;
                let p = 0.5 + 0.5 * (tt * 2.0 + hash(tohum) * TAU).sin();
                let (x, y) = (hash(tohum + 1) * wf, hash(tohum + 2) * hf);
                tv.yildiz(x, y, (20.0 + 40.0 * p) * u, 0.25, 4, 0.0, BEYAZ, p);
            }
            tv.yildiz(cx, cy, 400.0 * u, 0.45, 5, tt * 0.35, kos(yuz, 0.6), 1.0);
            tv.yildiz(cx, cy, 360.0 * u, 0.45, 5, tt * 0.35, yuz, 1.0);
        }
        // ------------------------------------------------ kampanya
        "rozet-yuzde" => {
            let ol = anim::geri((yerel / 0.6).min(1.0)) * (1.0 + 0.04 * (tt * 3.0).sin());
            tv.yildiz(cx, cy, 420.0 * u * ol, 0.84, 18, tt * 0.3, vu, 1.0);
            tv.daire(cx, cy, 330.0 * u * ol, yuz, 1.0);
            tv.halka(cx, cy, 300.0 * u * ol, 8.0 * u, vu, 1.0);
            yazi_rozet(&mut tv, z, ol, 0.0, 0.0, 0.0, u);
        }
        "rozet-taksit" => {
            let ol = anim::geri((yerel / 0.6).min(1.0));
            let (hw, hh) = (420.0 * u * ol, 270.0 * u * ol);
            tv.yuvarlak(cx, cy, hw + 24.0 * u, hh + 24.0 * u, 70.0 * u, vu, 1.0);
            tv.yuvarlak(cx, cy, hw, hh, 50.0 * u, yuz, 1.0);
            let ad = 90.0 * u;
            for i in -2..((hw * 2.0 / ad) as i32 + 3) {
                let sx = cx - hw + i as f32 * ad + fr(tt * 0.6) * ad;
                let pts: Vec<(f32, f32)> = [(sx, cy + hh - 20.0 * u), (sx + 30.0 * u, cy + hh - 20.0 * u), (sx + 30.0 * u + 80.0 * u, cy - hh + 20.0 * u), (sx + 80.0 * u, cy - hh + 20.0 * u)].iter().map(|q| (sinir(q.0, cx - hw + 30.0 * u, cx + hw - 30.0 * u), q.1)).collect();
                tv.cokgen(&pts, kos(yuz, 0.9), 1.0);
            }
            yazi_rozet(&mut tv, z, ol, 0.0, 0.0, 0.0, u);
        }
        "rozet-yeni" => {
            let ol = anim::elastik((yerel / 1.0).min(1.0)).max(0.02);
            tv.daire(cx, cy, 410.0 * u * ol, vu, 1.0);
            tv.daire(cx, cy, 330.0 * u * ol, yuz, 1.0);
            for i in 0..28 { let a = tt * 0.5 + i as f32 / 28.0 * TAU; tv.daire(cx + a.cos() * 370.0 * u * ol, cy + a.sin() * 370.0 * u * ol, 13.0 * u * ol, yuz, 1.0); }
            yazi_rozet(&mut tv, z, ol, 0.0, 0.0, 0.0, u);
        }
        "rozet-son-gunler" => {
            let ol = anim::geri((yerel / 0.6).min(1.0));
            let (hw, hh) = (410.0 * u * ol, 160.0 * u * ol);
            for s in [-1.0f32, 1.0] {
                let x0 = cx + s * (hw - 40.0 * u);
                let x1 = cx + s * (hw + 140.0 * u);
                tv.cokgen(&[(x0, cy - hh + 60.0 * u), (x1, cy - hh + 60.0 * u), (x1 - s * 60.0 * u, cy + 40.0 * u), (x1, cy + hh + 60.0 * u), (x0, cy + hh + 60.0 * u)], kos(vu, 0.7), 1.0);
            }
            tv.dikd(cx - hw, cy - hh, cx + hw, cy + hh, vu, 1.0);
            tv.dikd(cx - hw + 16.0 * u, cy - hh + 16.0 * u, cx + hw - 16.0 * u, cy + hh - 16.0 * u, yuz, 1.0);
            for (s, yy) in [(1.0f32, cy - hh - 200.0 * u), (-1.0, cy + hh + 200.0 * u)] {
                for i in 0..3 {
                    let a = 0.3 + 0.7 * (0.5 + 0.5 * (tt * 3.0 - i as f32 * 0.9).sin());
                    let x = cx + (i as f32 - 1.0) * 150.0 * u;
                    tv.cizgi(x - 50.0 * u, yy + s * 40.0 * u, x, yy - s * 20.0 * u, 26.0 * u, vu, a);
                    tv.cizgi(x, yy - s * 20.0 * u, x + 50.0 * u, yy + s * 40.0 * u, 26.0 * u, vu, a);
                }
            }
            yazi_rozet(&mut tv, z, ol, 0.0, 0.0, 0.0, u);
        }
        "rozet-indirim" => {
            let aci = (tt * 1.4).sin() * 0.35 * (1.0 - (-yerel).exp() * 0.0);
            let sx = aci.cos();
            let (hw, ust, alt) = (300.0 * u, cy - 430.0 * u, cy + 380.0 * u);
            let p = |x: f32, y: f32| (cx + (x - cx) * sx, y);
            tv.cizgi(cx, hf * 0.04, cx, ust + 70.0 * u, 8.0 * u, Renk(170, 170, 170), 1.0);
            let govde = [p(cx, ust - 60.0 * u), p(cx + hw, ust + 140.0 * u), p(cx + hw, alt), p(cx - hw, alt), p(cx - hw, ust + 140.0 * u)];
            tv.cokgen(&govde, vu, 1.0);
            let ic: Vec<(f32, f32)> = [(cx, ust - 30.0 * u), (cx + hw - 22.0 * u, ust + 150.0 * u), (cx + hw - 22.0 * u, alt - 22.0 * u), (cx - hw + 22.0 * u, alt - 22.0 * u), (cx - hw + 22.0 * u, ust + 150.0 * u)].iter().map(|q| p(q.0, q.1)).collect();
            tv.cokgen(&ic, yuz, 1.0);
            tv.elips(cx, ust + 70.0 * u, 34.0 * u * sx.max(0.1), 34.0 * u, 0.0, zem, 1.0);
            yazi_rozet(&mut tv, z, 1.0, aci, 0.0, 120.0 * u, u);
        }
        "rozet-firsat" => {
            let ol = anim::geri((yerel / 0.6).min(1.0));
            let (hw, hh) = (420.0 * u * ol, 220.0 * u * ol);
            tv.dikd(cx - hw - 22.0 * u, cy - hh - 22.0 * u, cx + hw + 22.0 * u, cy + hh + 22.0 * u, vu, 1.0);
            tv.dikd(cx - hw, cy - hh, cx + hw, cy + hh, yuz, 1.0);
            for s in [-1.0f32, 1.0] { for t2 in [-1.0f32, 1.0] { tv.daire(cx + s * (hw - 30.0 * u), cy + t2 * (hh - 30.0 * u), 12.0 * u, vu, 1.0); } }
            yazi_rozet(&mut tv, z, ol, 0.0, 0.0, 0.0, u);
            let konum = fr(tt / 2.4) * 1.6 - 0.3;
            let bx = cx - hw + konum * hw * 2.0;
            if let Some((x0, y0, x1, y1)) = tv.kutu(cx - hw, cy - hh, cx + hw, cy + hh) {
                for y in y0..=y1 { for x in x0..=x1 {
                    let d = ((x as f32 - bx) + (y as f32 - cy) * 0.45).abs();
                    if d < 60.0 * u { tv.px(x, y, BEYAZ, (1.0 - d / (60.0 * u)) * 0.55); }
                } }
            }
        }
        // ------------------------------------------------ ok ve yön
        "ok-akisi" => {
            let yatay = matches!(m.yon.as_str(), "sag" | "sol");
            let boy = 120.0 * u;
            let ara = boy * 1.5;
            for sira in 0..3 {
                let sabit = if yatay { hf * (0.35 + 0.15 * sira as f32) } else { wf * (0.25 + 0.25 * sira as f32) };
                let uzun = if yatay { wf } else { hf };
                let n = (uzun / ara) as i32 + 2;
                let kay = fr(tt * 0.9 + sira as f32 * 0.33) * ara;
                for i in -1..n {
                    let s = i as f32 * ara + kay;
                    let a = ((s / uzun) * PI).sin().clamp(0.0, 1.0);
                    let (x, y) = if yatay { (if m.yon == "sag" { s } else { uzun - s }, sabit) } else { (sabit, if m.yon == "asagi" { s } else { uzun - s }) };
                    let (p1, p2, p3) = (don((-boy * 0.4, -boy * 0.7), yon_aci), don((boy * 0.4, 0.0), yon_aci), don((-boy * 0.4, boy * 0.7), yon_aci));
                    let rk = if sira == 1 { yuz } else { vu };
                    tv.cizgi(x + p1.0, y + p1.1, x + p2.0, y + p2.1, 34.0 * u, rk, a);
                    tv.cizgi(x + p2.0, y + p2.1, x + p3.0, y + p3.1, 34.0 * u, rk, a);
                }
            }
        }
        "ok-nabiz" => {
            let it = (tt * 3.0).sin().max(0.0).powi(2) * 90.0 * u;
            let (l, t_, hl, hw) = (620.0 * u, 160.0 * u, 260.0 * u, 420.0 * u);
            let govde = [(-l / 2.0, -t_ / 2.0), (l / 2.0 - hl, -t_ / 2.0), (l / 2.0 - hl, -hw / 2.0), (l / 2.0, 0.0), (l / 2.0 - hl, hw / 2.0), (l / 2.0 - hl, t_ / 2.0), (-l / 2.0, t_ / 2.0)];
            let yazi_y = if matches!(m.yon.as_str(), "asagi") { hf * 0.18 } else { hf * 0.82 };
            let (ox, oy) = (cx, if matches!(m.yon.as_str(), "asagi" | "yukari") { cy + 60.0 * u * (if m.yon == "asagi" { 1.0 } else { -1.0 }) } else { cy });
            for iz in (0..3).rev() {
                let geri = iz as f32 * 110.0 * u;
                let (dx, dy) = (yon_aci.cos() * (it - geri), yon_aci.sin() * (it - geri));
                let pts: Vec<(f32, f32)> = govde.iter().map(|q| { let r = don(*q, yon_aci); (ox + r.0 + dx, oy + r.1 + dy) }).collect();
                tv.cokgen(&pts, if iz == 0 { yuz } else { kos(yuz, 0.25 / iz as f32) }, 1.0);
            }
            tv.piksel_sigdir(&yazi, cx, yazi_y, wf * 0.86, 30.0 * u, vu, 1.0);
        }
        "konum-pini" => {
            let zip = (tt * 2.4).sin().abs() * 180.0 * u;
            let zemin_y = cy + 330.0 * u;
            tv.elips(cx, zemin_y, 160.0 * u * (1.0 - zip / (400.0 * u)), 30.0 * u, 0.0, kos(yuz, 0.3), 1.0);
            let (px, py) = (cx, zemin_y - 420.0 * u - zip);
            tv.cokgen(&[(px - 190.0 * u, py + 60.0 * u), (px + 190.0 * u, py + 60.0 * u), (px, zemin_y - zip)], yuz, 1.0);
            tv.daire(px, py, 220.0 * u, yuz, 1.0);
            tv.daire(px, py, 90.0 * u, zem, 1.0);
            tv.piksel_sigdir(&yazi, cx, hf * 0.84, wf * 0.86, 26.0 * u, vu, 1.0);
        }
        "ok-donen" => {
            let rr = 320.0 * u;
            for i in 0..2 {
                let bas = tt * 1.2 + i as f32 * PI;
                tv.yay(cx, cy, rr, 70.0 * u, bas, 0.36, if i == 0 { yuz } else { vu }, 1.0);
                let son = bas + 0.36 * TAU;
                let (hx, hy) = (cx + son.cos() * rr, cy + son.sin() * rr);
                let (tx, ty) = (-son.sin(), son.cos());
                let (nx, ny) = (son.cos(), son.sin());
                tv.cokgen(&[(hx + nx * 90.0 * u, hy + ny * 90.0 * u), (hx - nx * 90.0 * u, hy - ny * 90.0 * u), (hx + tx * 110.0 * u, hy + ty * 110.0 * u)], if i == 0 { yuz } else { vu }, 1.0);
            }
        }
        // ------------------------------------------------ logo ve geçiş
        "logo-perde" => {
            yazi_bas(&mut tv, z, yuz, z.yan, 1.0, 0.0, 0.0, 0.0, z.d.max(10.0 * u));
            let p = anim::yumusak(((yerel - 0.3) / 1.4).clamp(0.0, 1.0));
            let kapa = cx * (1.0 - p);
            if kapa > 0.5 {
                let perde = kos(vu, 0.75);
                if let Some((x0, y0, x1, y1)) = tv.kutu(0.0, 0.0, wf, hf) {
                    for y in y0..=y1 { for x in x0..=x1 {
                        let xf = x as f32;
                        let (ic, uz) = if xf < kapa { (true, kapa - xf) } else if xf > wf - kapa { (true, xf - (wf - kapa)) } else { (false, 0.0) };
                        if !ic { continue; }
                        let kat = 0.65 + 0.35 * ((uz / (70.0 * u)) * PI).sin().abs();
                        tv.px(x, y, kos(perde, kat), 1.0);
                    } }
                }
                for s in [kapa, wf - kapa] { tv.dikd(s - 6.0 * u, 0.0, s + 6.0 * u, hf, yuz, 1.0); }
            } else {
                let konum = (yerel - 1.8) / 1.3;
                crate::efekt::isik_supurme(tv.t, w, h, zem, if (0.0..=1.0).contains(&konum) { konum } else { -1.0 });
            }
            tv.dikd(0.0, 0.0, wf, 50.0 * u, kos(vu, 0.6), 1.0);
        }
        "logo-iris" => {
            yazi_bas(&mut tv, z, yuz, z.yan, 1.0, 0.0, 0.0, 0.0, z.d.max(10.0 * u));
            let p = anim::yumusak((yerel / 1.3).min(1.0));
            let maks = (cx * cx + cy * cy).sqrt();
            let r = maks * p;
            if p < 1.0 {
                if let Some((x0, y0, x1, y1)) = tv.kutu(0.0, 0.0, wf, hf) {
                    for y in y0..=y1 { for x in x0..=x1 {
                        let d = ((x as f32 - cx).powi(2) + (y as f32 - cy).powi(2)).sqrt();
                        if d > r { tv.px(x, y, zem, ((d - r) / 6.0).clamp(0.0, 1.0)); }
                    } }
                }
            }
            tv.halka(cx, cy, r.max(1.0), 16.0 * u, vu, if p < 1.0 { 1.0 } else { 0.0 });
            if p >= 1.0 { tv.halka(cx, cy, 440.0 * u + (tt * 2.0).sin() * 10.0 * u, 12.0 * u, vu, 1.0); }
        }
        "logo-parlama" => {
            let ol = anim::geri((yerel / 0.9).min(1.0));
            yazi_bas(&mut tv, z, yuz, z.yan, ol, 0.0, 0.0, 0.0, z.d.max(10.0 * u));
            if yerel > 1.0 { let konum = fr((yerel - 1.0) / 2.5) * 1.4 - 0.2; crate::efekt::isik_supurme(tv.t, w, h, zem, if (0.0..=1.0).contains(&konum) { konum } else { -1.0 }); }
            tv.dikd(cx - 300.0 * u * ol, cy + 330.0 * u, cx + 300.0 * u * ol, cy + 346.0 * u, vu, 1.0);
        }
        "gecis-daire" => {
            let renk = [kos(yuz, 0.9), zem, kos(vu, 0.9), zem];
            let bant = 110.0 * u;
            for y in 0..h as i32 { for x in 0..w as i32 {
                let d = ((x as f32 - cx).powi(2) + (y as f32 - cy).powi(2)).sqrt();
                let q = (d - tt * 260.0 * u) / bant;
                let i = (q.floor() as i64).rem_euclid(4) as usize;
                tv.px(x, y, renk[i], 1.0);
            } }
        }
        "gecis-serit" => {
            let bant = 120.0 * u;
            for y in 0..h as i32 { for x in 0..w as i32 {
                let q = (x as f32 + y as f32 * 0.6 - tt * 380.0 * u) / bant;
                let i = (q.floor() as i64).rem_euclid(3);
                let rk = match i { 0 => yuz, 1 => zem, _ => vu };
                tv.px(x, y, rk, 1.0);
            } }
        }
        "geometri-kare" => {
            for i in 0..9 {
                let r = (430.0 - i as f32 * 46.0) * u;
                let a = tt * 0.45 * if i % 2 == 0 { 1.0 } else { -1.0 } + i as f32 * 0.15;
                let p: Vec<(f32, f32)> = (0..4).map(|k| { let q = don((r, 0.0), a + k as f32 * PI / 2.0); (cx + q.0, cy + q.1) }).collect();
                let rk = if i % 2 == 0 { yuz } else { vu };
                for k in 0..4 { tv.cizgi(p[k].0, p[k].1, p[(k + 1) % 4].0, p[(k + 1) % 4].1, 14.0 * u, rk, 1.0); }
            }
        }
        // ------------------------------------------------ ses
        "ekolayzer" => {
            let n = (dv.round() as usize).clamp(6, 32);
            let (x0, x1) = (wf * 0.06, wf * 0.94);
            let bw = (x1 - x0) / n as f32;
            let (alt, ust) = (hf * 0.86, hf * 0.16);
            let seg = 24usize;
            let sh = (alt - ust) / seg as f32;
            for i in 0..n {
                let v = seviye(i, tt * 1.4);
                let yan = (v * seg as f32).round() as usize;
                for s in 0..seg {
                    let k = s as f32 / seg as f32;
                    let rk = if k > 0.83 { Renk(255, 60, 50) } else if k > 0.6 { Renk(255, 210, 0) } else { kar(Renk(40, 220, 90), yuz, 0.3) };
                    let y = alt - (s + 1) as f32 * sh;
                    tv.dikd(x0 + i as f32 * bw + bw * 0.12, y + sh * 0.18, x0 + (i + 1) as f32 * bw - bw * 0.12, y + sh * 0.92, if s < yan { rk } else { kos(rk, 0.12) }, 1.0);
                }
            }
            tv.dikd(x0, alt + 14.0 * u, x1, alt + 22.0 * u, vu, 1.0);
        }
        "ses-dalgasi" => {
            for (j, rk, gen) in [(0usize, yuz, 1.0f32), (1, vu, 0.7), (2, kos(BEYAZ, 0.6), 0.45)] {
                let mut x = 0.0;
                let f = |x: f32| -> f32 { let e = (x / wf * PI).sin(); cy + (x / (90.0 * u) * (1.0 + j as f32 * 0.3) - tt * (3.0 + j as f32)).sin() * 300.0 * u * gen * e * (0.6 + 0.4 * (tt * 1.3 + j as f32).sin()) };
                while x < wf { let x2 = x + 10.0 * u; tv.cizgi(x, f(x), x2, f(x2), (16.0 - j as f32 * 4.0) * u, rk, 1.0); x = x2; }
            }
        }
        "dairesel-ekolayzer" => {
            let n = (dv.round() as usize).clamp(16, 96);
            let r0 = 240.0 * u;
            tv.daire(cx, cy, r0 - 20.0 * u, kos(vu, 0.25), 1.0);
            for i in 0..n {
                let a = tt * 0.2 + i as f32 / n as f32 * TAU;
                let v = seviye(i % 24, tt * 1.4);
                let l = 40.0 * u + v * 260.0 * u;
                tv.cizgi(cx + a.cos() * r0, cy + a.sin() * r0, cx + a.cos() * (r0 + l), cy + a.sin() * (r0 + l), (TAU * r0 / n as f32 * 0.55).max(3.0), kar(yuz, vu, v), 1.0);
            }
            tv.piksel_sigdir(&yazi, cx, cy, r0 * 1.5, 18.0 * u, yuz, 1.0);
        }
        "vu-metre" => {
            for (j, oy, ad) in [(0usize, hf * 0.3, "L"), (1, hf * 0.7, "R")] {
                let r = 340.0 * u;
                let py = oy + 170.0 * u;
                tv.yuvarlak(cx, oy, 420.0 * u, 250.0 * u, 40.0 * u, kos(yuz, 0.22), 1.0);
                for i in 0..=20 {
                    let k = i as f32 / 20.0;
                    let a = -PI * 0.82 + k * PI * 0.64;
                    let rk = if k > 0.75 { Renk(255, 60, 50) } else { yuz };
                    let l = if i % 5 == 0 { 50.0 } else { 26.0 } * u;
                    tv.cizgi(cx + a.cos() * r, py + a.sin() * r, cx + a.cos() * (r - l), py + a.sin() * (r - l), 8.0 * u, rk, 1.0);
                }
                let v = seviye(j * 5 + 1, tt * 1.2);
                let a = -PI * 0.82 + v * PI * 0.64;
                tv.cizgi(cx, py, cx + a.cos() * (r - 20.0 * u), py + a.sin() * (r - 20.0 * u), 12.0 * u, vu, 1.0);
                tv.daire(cx, py, 30.0 * u, vu, 1.0);
                tv.piksel_yazi(ad, cx - 330.0 * u, oy - 170.0 * u, 10.0 * u, yuz, 1.0);
                tv.piksel_yazi("VU", cx, oy + 60.0 * u, 10.0 * u, yuz, 1.0);
            }
        }
        _ => {}
    }
}
