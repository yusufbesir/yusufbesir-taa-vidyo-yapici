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
    for i in 0..n {
        if iptal.load(Ordering::Relaxed) { drop(girdi); let _ = kod.kill(); let _ = kod.wait(); let _ = std::fs::remove_file(&gecici); return Err("iptal".into()); }
        let kare = senaryo::kare_uret(&sen, &hz, i as f32 / sen.fps as f32, i);
        let kare = senaryo::dikey_kucult(&kare, sen.w, sen.sanal_h(), sen.h);
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
