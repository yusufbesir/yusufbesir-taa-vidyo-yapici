//! Senaryo motoru: metin dosyasından (sahne sahne) video üretir.
use crate::anim::{self, Maske, Poz};
use crate::{efekt, tv, *};
use ab_glyph::FontRef;

#[derive(Clone, Debug, PartialEq)]
pub enum Gecis { Tv, Perde, Yok }

#[derive(Clone, Debug)]
pub struct Sahne { pub metin: String, pub renk: String, pub giris: String, pub cikis: String, pub sure: f32, pub harf: f32, pub efekt: Vec<String> }

#[derive(Clone, Debug)]
pub struct Senaryo { pub w: u32, pub h: u32, pub fiziksel: Option<(f32, f32)>, pub fps: u32, pub acilis: Gecis, pub kapanis: Gecis, pub sahneler: Vec<Sahne> }

pub const GIRISLER: [&str; 12] = ["yok", "donerek", "zipla", "yandan", "zoom", "alttan", "ustten", "belir", "elastik", "daktilo", "sil", "toplan"];
pub const CIKISLAR: [&str; 8] = ["yok", "yukari", "asagi", "donerek", "sondur", "kucul", "sil", "dagil"];
pub const EFEKTLER: [&str; 6] = ["parlama", "pirilti", "cerceve", "cift-cerceve", "parcacik", "kivilcim"];

fn gecis(s: &str) -> Result<Gecis, String> {
    match s { "tv" => Ok(Gecis::Tv), "perde" => Ok(Gecis::Perde), "yok" => Ok(Gecis::Yok), _ => Err(format!("bilinmeyen geçiş: {s} (tv|perde|yok)")) }
}

pub fn ayristir(metin: &str) -> Result<Senaryo, String> {
    let mut s = Senaryo { w: 980, h: 1800, fiziksel: None, fps: 25, acilis: Gecis::Yok, kapanis: Gecis::Yok, sahneler: vec![] };
    for (no, ham) in metin.lines().enumerate() {
        let satir = ham.split('#').next().unwrap().trim();
        if satir.is_empty() { continue; }
        let hata = |m: &str| format!("satır {}: {m}", no + 1);
        if let Some(r) = satir.strip_prefix("sahne") {
            let a = r.find('"').ok_or_else(|| hata("sahne metni tırnak içinde olmalı"))?;
            let b = r[a + 1..].find('"').ok_or_else(|| hata("kapanış tırnağı yok"))? + a + 1;
            let mut sh = Sahne { metin: r[a + 1..b].to_string(), renk: "beyaz".into(), giris: "donerek".into(), cikis: "yok".into(), sure: 4.5, harf: 330.0, efekt: vec!["parlama".into()] };
            for kv in r[b + 1..].split_whitespace() {
                let (k, v) = kv.split_once('=').ok_or_else(|| hata(&format!("anahtar=değer bekleniyor: {kv}")))?;
                match k {
                    "renk" => { if palet(v).is_none() { return Err(hata(&format!("bilinmeyen renk: {v}"))); } sh.renk = v.into() }
                    "giris" => { if !GIRISLER.contains(&v) { return Err(hata(&format!("bilinmeyen giriş: {v} ({GIRISLER:?})"))); } sh.giris = v.into() }
                    "cikis" => { if !CIKISLAR.contains(&v) { return Err(hata(&format!("bilinmeyen çıkış: {v} ({CIKISLAR:?})"))); } sh.cikis = v.into() }
                    "sure" => sh.sure = v.parse().map_err(|_| hata("sure sayı olmalı"))?,
                    "harf" => sh.harf = v.parse().map_err(|_| hata("harf sayı olmalı"))?,
                    "efekt" => { sh.efekt = v.split(',').filter(|e| *e != "yok").map(String::from).collect(); for e in &sh.efekt { if !EFEKTLER.contains(&e.as_str()) { return Err(hata(&format!("bilinmeyen efekt: {e}"))); } } }
                    _ => return Err(hata(&format!("bilinmeyen anahtar: {k}"))),
                }
            }
            if sh.sure < 1.5 { return Err(hata("sure en az 1.5 sn")); }
            if sh.metin.starts_with('@') && cizim::ADLAR.iter().all(|a| *a != sh.metin[1..].trim().to_lowercase()) { return Err(hata(&format!("bilinmeyen çizim: {} ({:?})", sh.metin, cizim::ADLAR))); }
            s.sahneler.push(sh);
            continue;
        }
        let mut p = satir.split_whitespace();
        let (k, v) = (p.next().unwrap(), p.next().ok_or_else(|| hata("değer eksik"))?);
        match k {
            "ekran" => { let (a, b) = v.split_once('x').ok_or_else(|| hata("ekran GENxYUK"))?; s.w = a.parse().map_err(|_| hata("genişlik"))?; s.h = b.parse().map_err(|_| hata("yükseklik"))?; }
            "fiziksel" => { let (a, b) = v.split_once('x').ok_or_else(|| hata("fiziksel GENxYUK (metre)"))?; let (a, b): (f32, f32) = (a.parse().map_err(|_| hata("fiziksel genişlik"))?, b.parse().map_err(|_| hata("fiziksel yükseklik"))?); if a <= 0.0 || b <= 0.0 { return Err(hata("fiziksel ölçü sıfırdan büyük olmalı")); } s.fiziksel = Some((a, b)); }
            "fps" => s.fps = v.parse().map_err(|_| hata("fps"))?,
            "acilis" => s.acilis = gecis(v).map_err(|e| hata(&e))?,
            "kapanis" => s.kapanis = gecis(v).map_err(|e| hata(&e))?,
            _ => return Err(hata(&format!("bilinmeyen komut: {k}"))),
        }
    }
    if s.sahneler.is_empty() { return Err("en az bir sahne gerekli".into()); }
    Ok(s)
}

impl Senaryo {
    /// Çizim yüksekliği. `fiziksel` verilmişse kare pikselli sanal tuval: gerçek ekran oranında çizilir, çıkışta dikeyde piksel yüksekliğine indirilir.
    pub fn sanal_h(&self) -> u32 {
        match self.fiziksel { Some((gen, yuk)) => ((self.w as f32 * yuk / gen / 2.0).round() as u32 * 2).max(2), None => self.h }
    }
    /// Piksel oranı: >1 ise piksel dikeyde fiziksel olarak daha uzun.
    pub fn piksel_orani(&self) -> Option<f32> {
        self.fiziksel.map(|(g, y)| (y / self.h as f32) / (g / self.w as f32))
    }
}

/// RGB kareyi dikeyde `hv` satırdan `h` satıra alan ortalamasıyla indirir.
pub fn dikey_kucult(k: &[u8], w: u32, hv: u32, h: u32) -> Vec<u8> {
    if hv == h { return k.to_vec(); }
    let (w, hv, h) = (w as usize, hv as usize, h as usize);
    let oran = hv as f32 / h as f32;
    let mut c = vec![0u8; w * h * 3];
    for y in 0..h {
        let (a, b) = (y as f32 * oran, (y + 1) as f32 * oran);
        let (r0, r1) = (a as usize, (b.ceil() as usize).min(hv));
        for x in 0..w * 3 {
            let mut top = 0.0;
            for r in r0..r1 {
                let ag = ((r + 1) as f32).min(b) - (r as f32).max(a);
                top += k[r * w * 3 + x] as f32 * ag;
            }
            c[y * w * 3 + x] = (top / (b - a)).round().min(255.0) as u8;
        }
    }
    c
}

pub fn acilis_sn(g: &Gecis) -> f32 { match g { Gecis::Tv => tv::ACILIS_SN, Gecis::Perde => 1.2, Gecis::Yok => 0.0 } }
pub fn kapanis_sn(g: &Gecis) -> f32 { match g { Gecis::Tv => tv::KAPANIS_SN, Gecis::Perde => 1.2, Gecis::Yok => 0.0 } }
pub fn toplam_sn(s: &Senaryo) -> f32 { acilis_sn(&s.acilis) + s.sahneler.iter().map(|x| x.sure).sum::<f32>() + kapanis_sn(&s.kapanis) }

const GIRIS_SN: f32 = 0.9;
const CIKIS_SN: f32 = 0.9;

/// Bir satırın pozu: `t` sahne içi zaman.
pub fn satir_poz(giris: &str, cikis: &str, t: f32, sure: f32, li: usize, d: f32, ekran: (u32, u32)) -> Poz {
    let (w, h) = (ekran.0 as f32, ekran.1 as f32);
    let pi = std::f32::consts::PI;
    let faz = li as f32 * 0.9;
    let sal = Poz { aci: ((t - GIRIS_SN).max(0.0) * 1.3 + faz).sin() * 0.26, olcek: 1.0 + 0.03 * (t * 3.0 + faz).sin(), parlak: 1.0, derinlik: d, dx: 0.0, dy: (t * 2.0 + faz).sin() * 6.0 };
    let cs = if cikis == "dagil" { 1.2 } else { CIKIS_SN };
    if cikis != "yok" && t > sure - cs {
        let k = ((t - (sure - cs)) / cs).clamp(0.0, 1.0);
        let e = k * k * (3.0 - 2.0 * k);
        let mut p = sal;
        match cikis {
            "yukari" => { p.dy = -e * h * 1.05 - k * li as f32 * 60.0; p.aci = -e * pi * 0.25; }
            "asagi" => { p.dy = e * h * 1.05 + k * li as f32 * 60.0; p.aci = e * pi * 0.25; }
            "sondur" => { p.parlak = 1.0 - k; }
            "kucul" => { p.olcek = (1.0 - e).max(0.02); p.aci = e * pi * 2.0; p.parlak = 1.0 - k * k; }
            "sil" => {}
            "dagil" => { p.parlak = 0.0; }
            _ => { p.aci = -e * pi * 0.5; p.parlak = 1.0 - k; p.olcek = 1.0 + 0.25 * k; }
        }
        return p;
    }
    if giris == "toplan" {
        let ti = t - li as f32 * 0.15;
        if ti < 1.4 { return Poz { parlak: 0.0, ..sal }; }
        if ti < 1.7 { return Poz { derinlik: d * anim::yumusak((ti - 1.4) / 0.3), ..sal }; }
        return sal;
    }
    if giris != "yok" {
        let ti = t - li as f32 * if giris == "belir" { 0.45 } else { 0.15 };
        if ti < 0.0 { return Poz { parlak: 0.0, ..sal }; }
        if ti < GIRIS_SN {
            let k = ti / GIRIS_SN;
            let mut p = sal;
            match giris {
                "donerek" => { p.aci = (1.0 - anim::geri(k)) * pi * 0.5 + (1.0 - anim::yumusak(k)) * pi * 0.5; p.olcek = 0.4 + 0.6 * anim::geri(k); }
                "zipla" => { p.dy = -(1.0 - anim::ziplama(k)) * h * 0.7; p.aci = 0.0; }
                "yandan" => { p.aci = (1.0 - anim::yumusak(k)) * pi * 1.5; p.olcek = 0.6 + 0.4 * anim::yumusak(k); p.dx = (1.0 - anim::yumusak(k)) * w * 1.1; }
                "zoom" => { p.olcek = 1.0 + 2.2 * (1.0 - anim::geri(k)); p.aci = 0.0; }
                "alttan" => { p.dy = (1.0 - anim::geri(k)) * h * 1.05; p.aci = (1.0 - k) * pi * 0.5 + p.aci * k; }
                "belir" => { p.aci = p.aci * k; }
                "elastik" => { p.olcek = anim::elastik(k).max(0.01); }
                "sil" | "daktilo" => {}
                _ => { p.dy = -(1.0 - anim::geri(k)) * h * 1.05; p.aci = -(1.0 - k) * pi * 0.5 + p.aci * k; } // ustten
            }
            p.parlak = match giris { "belir" => k, "sil" | "daktilo" | "elastik" => 1.0, _ => (0.4 + 0.6 * k).min(1.0) };
            p.derinlik = match giris { "belir" | "sil" | "daktilo" | "elastik" => d, _ => d * anim::yumusak(k) };
            return p;
        }
    }
    sal
}

/// Silme/daktilo için yazının görünür sol-sağ aralığı.
pub fn satir_kirp(giris: &str, cikis: &str, t: f32, sure: f32, li: usize) -> (f32, f32) {
    let mut k = (0.0, 1.0);
    let ti = t - li as f32 * 0.15;
    if giris == "sil" || giris == "daktilo" {
        let dur = if giris == "daktilo" { 1.4 } else { 0.9 };
        if ti < dur { let p = (ti / dur).clamp(0.0, 1.0); k.1 = if giris == "daktilo" { (p * 12.0).floor() / 12.0 } else { anim::yumusak(p) }; }
    }
    if cikis == "sil" && t > sure - CIKIS_SN { k.0 = anim::yumusak(((t - (sure - CIKIS_SN)) / CIKIS_SN).clamp(0.0, 1.0)); }
    k
}

/// Perde: üstten ve alttan siyah kepenk; kenarda ince parlak çizgi. `kapali`: 0=açık, 1=tam kapalı.
pub fn perde(t: &mut [u8], w: u32, h: u32, kapali: f32, renk: Renk) {
    let kapa = ((h as f32 / 2.0) * kapali.clamp(0.0, 1.0)) as u32;
    if kapa == 0 { return; }
    let cizgi = 8u32;
    for y in 0..h {
        let (ust, alt) = (y < kapa, y >= h - kapa);
        let kenar = (y >= kapa.saturating_sub(cizgi) && y < kapa) || (y >= h - kapa && y < h - kapa + cizgi);
        if !(ust || alt || kenar) { continue; }
        for x in 0..w {
            let i = ((y * w + x) * 3) as usize;
            if kenar && kapa < h / 2 { t[i] = renk.0; t[i + 1] = renk.1; t[i + 2] = renk.2; }
            else { t[i] = 0; t[i + 1] = 0; t[i + 2] = 0; }
        }
    }
}

fn yerlesim(font: &FontRef, w: u32, h: u32, s: &Sahne) -> (Vec<Maske>, f32) {
    if let Some(ad) = s.metin.strip_prefix('@') {
        return (vec![cizim::maske(ad.trim().to_lowercase().as_str(), w, h, w.min(h) as f32 * 0.9).unwrap()], w as f32 * 0.06);
    }
    if s.metin.contains('|') { return afis_yerlesim(font, w, h, s); }
    let satir = dikey_satirlar(&s.metin);
    let mut boylar: Vec<f32> = satir.iter().map(|x| (w as f32 * 0.84 / genislik(font, x, 100.0).max(1.0) * 100.0).min(s.harf)).collect();
    let toplam: f32 = boylar.iter().map(|b| satir_yuksekligi(font, *b)).sum();
    if toplam > h as f32 * 0.8 { let k = h as f32 * 0.8 / toplam; for b in boylar.iter_mut() { *b *= k; } }
    let sh: Vec<f32> = boylar.iter().map(|b| satir_yuksekligi(font, *b)).collect();
    let mut y = (h as f32 - sh.iter().sum::<f32>()) / 2.0;
    let mut v = vec![];
    for (k, s1) in satir.iter().enumerate() {
        v.push(anim::kaydir(&anim::maske(font, w, h, &[s1.clone()], boylar[k]), (y + sh[k] / 2.0 - h as f32 / 2.0) as i32));
        y += sh[k];
    }
    (v, boylar.iter().cloned().fold(0.0, f32::max) * 0.14)
}

/// Açık satırlı düzen: "PEŞİN FİYATINA|*3|TAKSİT". `*` ile başlayan satır büyük rakamdır ve iki yanına çift çizgi gelir.
fn afis_yerlesim(font: &FontRef, w: u32, h: u32, s: &Sahne) -> (Vec<Maske>, f32) {
    let satirlar: Vec<(String, bool)> = s.metin.split('|').map(|x| { let x = x.trim(); match x.strip_prefix('*') { Some(r) => (r.trim().to_string(), true), None => (x.to_string(), false) } }).collect();
    let fit = |t: &str| (w as f32 * 0.80 / genislik(font, t, 100.0).max(1.0) * 100.0).min(s.harf);
    let ref_boy = satirlar.iter().filter(|x| !x.1).map(|x| fit(&x.0)).fold(f32::MAX, f32::min).min(s.harf);
    let mut boylar: Vec<f32> = satirlar.iter().map(|(t, buyuk)| if *buyuk { (ref_boy * 3.6).min(s.harf * 1.8).min(h as f32 * 0.34) } else { fit(t) }).collect();
    let toplam: f32 = boylar.iter().map(|b| satir_yuksekligi(font, *b)).sum();
    if toplam > h as f32 * 0.8 { let k = h as f32 * 0.8 / toplam; for b in boylar.iter_mut() { *b *= k; } }
    let sh: Vec<f32> = boylar.iter().map(|b| satir_yuksekligi(font, *b)).collect();
    let bosluk = h as f32 * 0.05; // uzun dikey ekranda satırlar nefes alsın
    let mut y = (h as f32 - sh.iter().sum::<f32>() - bosluk * (satirlar.len() - 1) as f32) / 2.0;
    let mut v = vec![];
    for (k, (t, buyuk)) in satirlar.iter().enumerate() {
        let orta = y + sh[k] / 2.0;
        let mut m = anim::kaydir(&anim::maske(font, w, h, &[t.clone()], boylar[k]), (orta - h as f32 / 2.0) as i32);
        if *buyuk {
            let gen = genislik(font, t, boylar[k]);
            let (kal, ara) = ((boylar[k] * 0.035).max(6.0) as u32, (boylar[k] * 0.06).max(10.0) as u32);
            let (sol_bit, sag_bas) = ((w as f32 / 2.0 - gen / 2.0 - boylar[k] * 0.18) as u32, (w as f32 / 2.0 + gen / 2.0 + boylar[k] * 0.18) as u32);
            let (bas, son) = ((w as f32 * 0.10) as u32, (w as f32 * 0.90) as u32);
            let cy = orta as u32;
            for off in [cy - (ara + kal) / 2 - kal / 2, cy + (ara + kal) / 2 - kal / 2] {
                if sol_bit > bas { anim::dikdortgen(&mut m, bas, off, sol_bit, off + kal); }
                if son > sag_bas { anim::dikdortgen(&mut m, sag_bas, off, son, off + kal); }
            }
        }
        v.push(m);
        y += sh[k] + bosluk;
    }
    (v, boylar.iter().cloned().fold(0.0, f32::max) * 0.08)
}

/// Dikey ekran için satırlara böl: her kelime ayrı satır; kısa kelimeler sonrakiyle birleşir.
pub fn dikey_satirlar(s: &str) -> Vec<String> {
    let kel: Vec<&str> = s.split_whitespace().collect();
    let mut out: Vec<String> = vec![];
    let mut i = 0;
    while i < kel.len() {
        let mut satir = kel[i].to_string();
        while i + 1 < kel.len() && (satir.chars().count() <= 2 || (satir.chars().count() + 1 + kel[i + 1].chars().count()) <= 7) {
            i += 1; satir.push(' '); satir.push_str(kel[i]);
        }
        out.push(satir);
        i += 1;
    }
    out
}

pub struct Hazir { pub maskeler: Vec<Maske>, pub d: f32, pub yuz: Renk, pub yan: Renk, pub zemin: Renk }

pub fn hazirla(font: &FontRef, s: &Senaryo) -> Vec<Hazir> {
    let tvli = s.acilis == Gecis::Tv || s.kapanis == Gecis::Tv;
    s.sahneler.iter().map(|sh| {
        let (yuz, z) = palet(&sh.renk).unwrap();
        let yan = Renk(yuz.0 / 2, yuz.1 / 2, yuz.2 / 3); // aynı tonun koyusu: altın yan yüz altın kalır
        let (maskeler, d) = yerlesim(font, s.w, s.sanal_h(), sh);
        Hazir { maskeler, d, yuz, yan, zemin: if tvli { Renk(18, 20, 24) } else { z } }
    }).collect()
}

/// t anındaki RGB kare.
pub fn kare_uret(s: &Senaryo, hz: &[Hazir], t: f32, no: u32) -> Vec<u8> {
    let (w, h) = (s.w, s.sanal_h());
    let ac = acilis_sn(&s.acilis);
    let toplam = toplam_sn(s);
    let ka = kapanis_sn(&s.kapanis);
    // aktif sahne
    let mut bas = ac;
    let mut idx = 0;
    for (i, sh) in s.sahneler.iter().enumerate() { if t >= bas { idx = i; } if i + 1 < s.sahneler.len() { bas += sh.sure; } }
    let bas_i: f32 = ac + s.sahneler[..idx].iter().map(|x| x.sure).sum::<f32>();
    let yerel = (t - bas_i).max(0.0);
    let (sh, z) = (&s.sahneler[idx], &hz[idx]);
    let mut k = anim::bos_kare(w, h, z.zemin);
    // ilk sahne açılışla birlikte zaten görünür: girişi açılış geçişi yapar
    let giris = if idx == 0 && s.acilis != Gecis::Yok { "yok" } else { sh.giris.as_str() };
    if sh.efekt.iter().any(|e| e == "parcacik") { efekt::parcaciklar(&mut k, w, h, t, 60, z.yuz, idx as u32 + 7); }
    if sh.efekt.iter().any(|e| e == "kivilcim") { efekt::kivilcimlar(&mut k, w, h, t, 40, z.yuz, idx as u32 + 3); }
    for (li, m) in z.maskeler.iter().enumerate() {
        let ti = yerel - li as f32 * 0.15;
        if giris == "toplan" && (0.0..1.4).contains(&ti) { anim::dagilma(&mut k, m, z.yuz, 1.0 - ti / 1.4, (idx * 10 + li) as u32); continue; }
        if sh.cikis == "dagil" && yerel > sh.sure - 1.2 { anim::dagilma(&mut k, m, z.yuz, ((yerel - (sh.sure - 1.2)) / 1.2).clamp(0.0, 1.0), (idx * 10 + li) as u32 + 50); continue; }
        let p = satir_poz(giris, &sh.cikis, yerel, sh.sure, li, z.d, (w, h));
        anim::kare3d_kirpmali(&mut k, m, z.yuz, z.yan, &p, satir_kirp(giris, &sh.cikis, yerel, sh.sure, li));
    }
    if sh.efekt.iter().any(|e| e == "parlama") {
        let konum = (yerel - 1.0) / 1.3;
        efekt::isik_supurme(&mut k, w, h, z.zemin, if (0.0..=1.0).contains(&konum) { konum } else { -1.0 });
    }
    if sh.efekt.iter().any(|e| e == "cerceve") { efekt::kosan_cerceve(&mut k, w, h, t, z.yuz, (w / 70).max(6), (w / 25).max(10)); }
    if sh.efekt.iter().any(|e| e == "cift-cerceve") { efekt::cift_cerceve(&mut k, w, h, z.yuz, (w / 240).max(3), (w / 22).max(12)); }
    if sh.efekt.iter().any(|e| e == "pirilti") { efekt::piriltilar(&mut k, w, h, t, 18, Renk(255, 255, 255), idx as u32 + 1); }
    let (kapat_basla, acik_bitis) = (toplam - ka, ac);
    let normal = tv::Tup { sv: 1.0, sh: 1.0, flas: 0.0, parlak: 0.985 + 0.015 * (no as f32 * 0.9).sin(), gurultu: 0.0 };
    if s.acilis == Gecis::Tv && t < acik_bitis { return tv::tup(&k, w, h, tv::acilis(t), no); }
    if s.kapanis == Gecis::Tv && t > kapat_basla { return tv::tup(&k, w, h, tv::kapanis(t - kapat_basla), no); }
    if s.acilis == Gecis::Perde && t < acik_bitis { perde(&mut k, w, h, 1.0 - anim::yumusak(t / acik_bitis), z.yuz); }
    if s.kapanis == Gecis::Perde && t > kapat_basla { perde(&mut k, w, h, anim::yumusak((t - kapat_basla) / ka), z.yuz); }
    if s.acilis == Gecis::Tv || s.kapanis == Gecis::Tv { return tv::tup(&k, w, h, normal, no); }
    k
}

#[cfg(test)]
mod t {
    use super::*;
    const ORNEK: &str = "ekran 490x900\nfps 10\nacilis tv\nkapanis perde\nsahne \"PEŞİN FİYATINA 3 TAKSİT\" renk=beyaz giris=zipla cikis=yukari sure=4\nsahne \"TERZİ ADEM ALTUN\" renk=altin giris=alttan efekt=parlama,pirilti # not\n";
    #[test]
    fn ayristirir() {
        let s = ayristir(ORNEK).unwrap();
        assert_eq!((s.w, s.h, s.fps), (490, 900, 10));
        assert_eq!(s.sahneler.len(), 2);
        assert_eq!(s.sahneler[1].efekt, vec!["parlama", "pirilti"]);
        assert!((toplam_sn(&s) - (1.5 + 4.0 + 4.5 + 1.2)).abs() < 1e-3);
    }
    #[test]
    fn fiziksel_olcu_ve_kucultme() {
        let s = ayristir("ekran 980x1800\nfiziksel 8x25\nsahne \"A\"").unwrap();
        assert_eq!(s.sanal_h(), 3062);
        assert!((s.piksel_orani().unwrap() - 1.7).abs() < 0.05);
        let k = vec![100u8; 10 * 30 * 3];
        let c = dikey_kucult(&k, 10, 30, 17);
        assert_eq!(c.len(), 10 * 17 * 3);
        assert!(c.iter().all(|&b| b == 100));
        assert_eq!(ayristir("sahne \"A\"").unwrap().sanal_h(), 1800);
    }
    #[test]
    fn hata_mesajlari() {
        assert!(ayristir("sahne \"A\" giris=uzay").unwrap_err().contains("bilinmeyen giriş"));
        assert!(ayristir("acilis tvv\nsahne \"A\"").is_err());
        assert!(ayristir("ekran 10x10").unwrap_err().contains("sahne"));
        assert!(ayristir("sahne \"@yok\"").unwrap_err().contains("çizim"));
    }
    #[test]
    fn kare_uretir_tum_gecisler() {
        let f = FontRef::try_from_slice(include_bytes!("../fonts/InterDisplay-Bold.otf")).unwrap();
        let s = ayristir(ORNEK).unwrap();
        let hz = hazirla(&f, &s);
        for t in [0.2, 1.0, 3.5, 6.0, 9.0, 10.4] {
            let k = kare_uret(&s, &hz, t, 3);
            assert_eq!(k.len(), (s.w * s.h * 3) as usize);
        }
        // sahne 1 ortasında yazı var
        assert!(kare_uret(&s, &hz, 3.0, 1).iter().any(|&b| b > 150));
    }
}
