//! ffmpeg bulma ve başlatma. ffmpeg uygulamayla gelmez; sırayla şurada aranır:
//! LED_FFMPEG ortam değişkeni, programın yanı (ffmpeg.exe veya ffmpeg\bin), Windows'ta
//! D:\Programlar\Araçlar Ve İndirilenler altı, en sonda PATH.
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};

fn ad() -> &'static str { if cfg!(windows) { "ffmpeg.exe" } else { "ffmpeg" } }

fn klasorde_ara(k: &Path, derinlik: u32) -> Option<PathBuf> {
    let d = k.join(ad());
    if d.is_file() { return Some(d); }
    let b = k.join("bin").join(ad());
    if b.is_file() { return Some(b); }
    if derinlik == 0 { return None; }
    let mut alt: Vec<PathBuf> = std::fs::read_dir(k).ok()?.filter_map(|e| e.ok()).map(|e| e.path()).filter(|p| p.is_dir()).collect();
    alt.sort();
    alt.into_iter().find_map(|p| klasorde_ara(&p, derinlik - 1))
}

/// ffmpeg yolunu bulur; bulunamazsa None.
pub fn bul() -> Option<PathBuf> {
    if let Ok(p) = std::env::var("LED_FFMPEG") { let p = PathBuf::from(p); if p.is_file() { return Some(p); } }
    if let Ok(exe) = std::env::current_exe() {
        if let Some(k) = exe.parent() { if let Some(p) = klasorde_ara(k, 0).or_else(|| klasorde_ara(&k.join("ffmpeg"), 1)) { return Some(p); } }
    }
    if cfg!(windows) {
        for kok in [r"D:\Programlar\Araçlar Ve İndirilenler", r"C:\Programlar\Araçlar Ve İndirilenler"] {
            if let Some(p) = klasorde_ara(Path::new(kok), 6) { return Some(p); }
        }
    }
    let yol = std::env::var_os("PATH")?;
    std::env::split_paths(&yol).map(|k| k.join(ad())).find(|p| p.is_file())
}

pub const YOK_MESAJI: &str = "ffmpeg bulunamadı. ffmpeg.exe dosyasını programın klasörüne koyun, D:\\Programlar\\Araçlar Ve İndirilenler altına kurun veya LED_FFMPEG ortam değişkeniyle yolunu verin.";

/// Ham RGB24 kareleri stdin'den alıp MP4 (H.264) veya HAP MOV yazan ffmpeg sürecini başlatır.
pub fn baslat(cikis: &str, kodek: &str, w: u32, h: u32, fps: u32) -> Result<Child, String> {
    let yol = bul().ok_or_else(|| YOK_MESAJI.to_string())?;
    let mut c = Command::new(yol);
    c.args(["-y", "-f", "rawvideo", "-pix_fmt", "rgb24", "-s", &format!("{w}x{h}"), "-r", &fps.to_string(), "-i", "-"]);
    match kodek {
        "hap" => c.args(["-c:v", "hap", "-format", "hap"]),
        _ => c.args(["-c:v", "libx264", "-pix_fmt", "yuv420p", "-crf", "12", "-preset", "slow", "-g", "1", "-movflags", "+faststart"]),
    };
    c.arg(cikis).stdin(Stdio::piped()).stdout(Stdio::null()).stderr(Stdio::null());
    #[cfg(windows)]
    { use std::os::windows::process::CommandExt; c.creation_flags(0x0800_0000); } // konsol penceresi açma
    c.spawn().map_err(|e| format!("ffmpeg başlatılamadı: {e}"))
}
