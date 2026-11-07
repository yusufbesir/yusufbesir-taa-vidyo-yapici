//! Senaryodan önizleme karesi ve video üretimi (arayüz ve komut satırı ortak kullanır).
use crate::{png, senaryo};
use ab_glyph::FontRef;
use std::io::Write;
use std::sync::atomic::{AtomicBool, Ordering};

pub const FONT: &[u8] = include_bytes!("../fonts/InterDisplay-Bold.otf");

pub fn font() -> FontRef<'static> { FontRef::try_from_slice(FONT).expect("gömülü font geçersiz") }

/// Ekran ölçüsü ve kare hızı denetimi (ffmpeg için genişlik çift olmalı).
pub fn dogrula(s: &senaryo::Senaryo) -> Result<(), String> {
    if s.w < 64 || s.h < 64 || s.w > 4096 || s.h > 4096 { return Err("ekran ölçüsü 64 ile 4096 piksel arasında olmalı".into()); }
    if s.w % 2 != 0 || s.h % 2 != 0 { return Err("ekran genişliği ve yüksekliği çift sayı olmalı".into()); }
    if s.fps == 0 || s.fps > 60 { return Err("fps 1 ile 60 arasında olmalı".into()); }
    if senaryo::toplam_sn(s) > 600.0 { return Err("video 10 dakikayı aşamaz".into()); }
    Ok(())
}

pub fn senaryo_oku(metin: &str) -> Result<senaryo::Senaryo, String> {
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
    for k in 0..n {
        if iptal.load(Ordering::Relaxed) { drop(girdi); let _ = kod.kill(); let _ = kod.wait(); return Err("iptal edildi".into()); }
        let kare = senaryo::kare_uret(&sen, &hz, k as f32 / sen.fps as f32, k);
        let kare = senaryo::dikey_kucult(&kare, sen.w, sen.sanal_h(), sen.h);
        if girdi.write_all(&kare).is_err() { let _ = kod.wait(); return Err("ffmpeg beklenmedik şekilde kapandı (kodek veya çıkış yolu hatalı olabilir)".into()); }
        ilerleme(k + 1, n);
    }
    drop(girdi);
    let d = kod.wait().map_err(|e| e.to_string())?;
    if d.success() { Ok(format!("{:.1} sn, {} kare", n as f32 / sen.fps as f32, n)) } else { Err("ffmpeg hata verdi".into()) }
}
