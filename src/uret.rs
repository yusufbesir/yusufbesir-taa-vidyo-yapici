//! Senaryodan önizleme karesi ve video üretimi (arayüz ve komut satırı ortak kullanır).
use crate::{png, senaryo};
use ab_glyph::FontRef;
use std::io::Write;
use std::sync::atomic::{AtomicBool, Ordering};

pub const FONT: &[u8] = include_bytes!("../fonts/InterDisplay-Bold.otf");

pub fn font() -> FontRef<'static> { FontRef::try_from_slice(FONT).expect("gömülü font geçersiz") }


/// Yanıp sönme koruması: ardışık iki karede ortalama parlaklık artışı sınırı aşarsa kare kısılır
/// (saniyede 3 parlamayı aşan ani tam ekran ışıkları engellenir). Senaryoda `koruma kapali` ile kapatılır.
pub struct Koruma { onceki: f32, acik: bool, lut: [f32; 256] }
const KORUMA_ADIM: f32 = 0.08;
impl Koruma {
    pub fn yeni(acik: bool) -> Self {
        let mut lut = [0.0f32; 256];
        for (i, v) in lut.iter_mut().enumerate() { *v = (i as f32 / 255.0).powf(2.2); }
        Koruma { onceki: 0.0, acik, lut }
    }
    fn ortalama(&self, k: &[u8]) -> f32 {
        let (mut top, mut n) = (0.0f32, 0u32);
        for p in k.chunks_exact(3).step_by(17) { top += 0.2126 * self.lut[p[0] as usize] + 0.7152 * self.lut[p[1] as usize] + 0.0722 * self.lut[p[2] as usize]; n += 1; }
        if n == 0 { 0.0 } else { top / n as f32 }
    }
    pub fn uygula(&mut self, k: &mut [u8]) {
        if !self.acik { return; }
        let m = self.ortalama(k);
        if m > self.onceki + KORUMA_ADIM {
            let g = ((self.onceki + KORUMA_ADIM) / m).powf(1.0 / 2.2);
            for b in k.iter_mut() { *b = (*b as f32 * g) as u8; }
            self.onceki += KORUMA_ADIM;
        } else { self.onceki = m; }
    }
}

/// Ekran ölçüsü ve kare hızı denetimi (ffmpeg için genişlik çift olmalı).
pub fn dogrula(s: &senaryo::Senaryo) -> Result<(), String> {
    if s.w < 64 || s.h < 64 || s.w > 4096 || s.h > 4096 { return Err("ekran ölçüsü 64 ile 4096 piksel arasında olmalı".into()); }
    if s.w % 2 != 0 || s.h % 2 != 0 { return Err("ekran genişliği ve yüksekliği çift sayı olmalı".into()); }
    if s.fps == 0 || s.fps > 60 { return Err("fps 1 ile 60 arasında olmalı".into()); }
    if senaryo::toplam_sn(s) > 600.0 { return Err("video 10 dakikayı aşamaz".into()); }
    Ok(())
}

pub fn senaryo_oku(metin: &str) -> Result<senaryo::Senaryo, String> {
    crate::animasyon::yukle(); // klasöre eklenen animasyonlar anında geçerli olur
    let s = senaryo::ayristir(metin)?;
    dogrula(&s)?;
    Ok(s)
}

/// t anındaki karenin küçültülmüş RGB hâli (en çok ~500 px genişlik). Dönüş: (rgb, w, h, toplam_sn).
pub fn onizleme(metin: &str, t: f32) -> Result<(Vec<u8>, u32, u32, f32), String> {
    let sen = senaryo_oku(metin)?;
    let f = font();
    let hz = senaryo::hazirla(&f, &sen);
    let toplam = senaryo::toplam_sn(&sen);
    let t = t.clamp(0.0, (toplam - 0.04).max(0.0));
    let kare = senaryo::kare_uret(&sen, &hz, t, (t * sen.fps as f32) as u32);
    let kare = senaryo::dikey_kucult(&kare, sen.w, sen.sanal_h(), sen.h);
    let k = (sen.w.div_ceil(500)..=sen.w.div_ceil(500) + 6).find(|k| sen.w % k == 0 && sen.h % k == 0).unwrap_or(1);
    let (rgb, w, h) = png::kucult(&kare, sen.w, sen.h, k);
    Ok((rgb, w, h, toplam))
}

/// Videoyu üretir. `ilerleme(kare, toplam)` her karede çağrılır; `iptal` true olunca durur.
pub fn video(metin: &str, cikis: &str, kodek: &str, ilerleme: &dyn Fn(u32, u32), iptal: &AtomicBool) -> Result<String, String> {
    let sen = senaryo_oku(metin)?;
    if kodek == "hap" && (sen.w % 4 != 0 || sen.h % 4 != 0) { return Err("HAP için ekran genişliği ve yüksekliği 4'e tam bölünmeli (örn. 980x1800)".into()); }
    let f = font();
    let hz = senaryo::hazirla(&f, &sen);
    let n = (senaryo::toplam_sn(&sen) * sen.fps as f32) as u32;
    let mut kod = crate::ffmpeg::baslat(cikis, kodek, sen.w, sen.h, sen.fps)?;
    let mut girdi = kod.stdin.take().ok_or("ffmpeg girişi açılamadı")?;
    let mut kor = Koruma::yeni(sen.koruma);
    for k in 0..n {
        if iptal.load(Ordering::Relaxed) { drop(girdi); let _ = kod.kill(); let _ = kod.wait(); return Err("iptal edildi".into()); }
        let kare = senaryo::kare_uret(&sen, &hz, k as f32 / sen.fps as f32, k);
        let mut kare = senaryo::dikey_kucult(&kare, sen.w, sen.sanal_h(), sen.h);
        kor.uygula(&mut kare);
        if girdi.write_all(&kare).is_err() { let _ = kod.wait(); return Err("ffmpeg beklenmedik şekilde kapandı (kodek veya çıkış yolu hatalı olabilir)".into()); }
        ilerleme(k + 1, n);
    }
    drop(girdi);
    let d = kod.wait().map_err(|e| e.to_string())?;
    if d.success() { Ok(format!("{:.1} sn, {} kare", n as f32 / sen.fps as f32, n)) } else { Err("ffmpeg hata verdi".into()) }
}

/// Canlı önizleme klibi: senaryoyu küçültüp (≤480 px genişlik, 12 fps) hızlı kodlanmış bir MP4 olarak döndürür.
pub fn klip(metin: &str, iptal: &AtomicBool) -> Result<Vec<u8>, String> {
    let mut sen = senaryo_oku(metin)?;
    let k = (1..=50u32).find(|k| sen.w % k == 0 && sen.h % k == 0 && sen.w / k <= 480 && (sen.w / k) % 2 == 0 && (sen.h / k) % 2 == 0).unwrap_or(1);
    sen.w /= k; sen.h /= k; sen.fps = 12;
    for sh in sen.sahneler.iter_mut() { sh.harf /= k as f32; }
    let f = font();
    let hz = senaryo::hazirla(&f, &sen);
    let n = (senaryo::toplam_sn(&sen) * sen.fps as f32) as u32;
    let gecici = std::env::temp_dir().join(format!("led-klip-{}-{}.mp4", std::process::id(), std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_nanos()).unwrap_or(0)));
    let yol = crate::ffmpeg::bul().ok_or_else(|| crate::ffmpeg::YOK_MESAJI.to_string())?;
    let mut c = std::process::Command::new(yol);
    c.args(["-y", "-f", "rawvideo", "-pix_fmt", "rgb24", "-s", &format!("{}x{}", sen.w, sen.h), "-r", &sen.fps.to_string(), "-i", "-", "-c:v", "libx264", "-preset", "ultrafast", "-crf", "24", "-pix_fmt", "yuv420p", "-movflags", "+faststart"]);
    c.arg(&gecici).stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::null()).stderr(std::process::Stdio::null());
    #[cfg(windows)]
    { use std::os::windows::process::CommandExt; c.creation_flags(0x0800_0000); }
    let mut kod = c.spawn().map_err(|e| format!("ffmpeg başlatılamadı: {e}"))?;
    let mut girdi = kod.stdin.take().ok_or("ffmpeg girişi açılamadı")?;
    let mut kor = Koruma::yeni(sen.koruma);
    for i in 0..n {
        if iptal.load(Ordering::Relaxed) { drop(girdi); let _ = kod.kill(); let _ = kod.wait(); let _ = std::fs::remove_file(&gecici); return Err("iptal".into()); }
        let kare = senaryo::kare_uret(&sen, &hz, i as f32 / sen.fps as f32, i);
        let mut kare = senaryo::dikey_kucult(&kare, sen.w, sen.sanal_h(), sen.h);
        kor.uygula(&mut kare);
        if girdi.write_all(&kare).is_err() { let _ = kod.wait(); let _ = std::fs::remove_file(&gecici); return Err("ffmpeg kapandı".into()); }
    }
    drop(girdi);
    let d = kod.wait().map_err(|e| e.to_string())?;
    let b = std::fs::read(&gecici).map_err(|e| e.to_string());
    let _ = std::fs::remove_file(&gecici);
    if d.success() { b } else { Err("ffmpeg hata verdi".into()) }
}

/// Denetim ve öneri listesi.
pub fn analiz(metin: &str) -> Result<Vec<(&'static str, String)>, String> {
    let sen = senaryo_oku(metin)?;
    let f = font();
    let hz = senaryo::hazirla(&f, &sen);
    Ok(crate::oto::denetle(&sen, &hz))
}

#[cfg(test)]
mod t {
    use super::*;
    #[test]
    fn koruma_flas_siniri() {
        let mut k = Koruma::yeni(true);
        let mut onceki = 0.0;
        for i in 0..20 {
            let mut kare = vec![if i % 2 == 0 { 255u8 } else { 0 }; 300];
            k.uygula(&mut kare);
            let m = k.ortalama(&kare);
            assert!(m - onceki <= KORUMA_ADIM + 0.01, "kare {i}: artış {}", m - onceki);
            onceki = m;
        }
        let mut kapali = Koruma::yeni(false);
        let mut kare = vec![255u8; 300];
        kapali.uygula(&mut kare);
        assert_eq!(kare[0], 255);
    }
    #[test]
    fn dinamik_metin_cozulur() {
        assert!(!crate::senaryo::dinamik_metin("{tarih}").contains('{'));
        assert_eq!(crate::senaryo::dinamik_metin("SON {gun:2000-01-01} GÜN"), "SON 0 GÜN");
        let s = crate::senaryo::dinamik_metin("{gun:2999-01-01}");
        assert!(s.parse::<i64>().unwrap() > 1000);
    }
    #[test]
    fn egri_ve_koruma_senaryoda() {
        let s = senaryo::ayristir("koruma kapali\nsahne \"TAA\" egri=esnek giris=zoom-in\n").unwrap();
        assert!(!s.koruma && s.sahneler[0].egri == "esnek");
        assert!(senaryo::ayristir("sahne \"A\" egri=yok-boyle\n").is_err());
        for e in crate::anim::EGRILER { assert!((crate::anim::egri_k(e.0, 1.0) - 1.0).abs() < 1e-3, "{}", e.0); assert!(crate::anim::egri_k(e.0, 0.0).abs() < 1e-3, "{}", e.0); }
    }
}
