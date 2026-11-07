//! TAA Vidyo Yapıcı kurulum ve kaldırma programı (tek exe; adı "kaldir" ile başlıyorsa kaldırır).
//! Yalnız kendi dosyalarını yazar ve siler; kullanıcının videolarına ve senaryolarına dokunmaz.
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

include!(concat!(env!("OUT_DIR"), "/dosyalar.rs"));

const AD: &str = "TAA Vidyo Yapıcı";
const KLASOR_AD: &str = "TAA Vidyo Yapıcı";
#[cfg(windows)]
const KAYIT: &str = r"HKCU\Software\Microsoft\Windows\CurrentVersion\Uninstall\TAA-Vidyo-Yapici";
const MANIFEST: &str = "kurulum.txt";

#[cfg(windows)]
fn konsol_utf8() { extern "system" { fn SetConsoleOutputCP(k: u32) -> i32; } unsafe { SetConsoleOutputCP(65001); } }
#[cfg(not(windows))]
fn konsol_utf8() {}

fn varsayilan_hedef() -> PathBuf {
    if cfg!(windows) {
        if Path::new(r"D:\").exists() { return PathBuf::from(format!(r"D:\Programlar\{KLASOR_AD}")); }
        if let Ok(p) = std::env::var("LOCALAPPDATA") { return PathBuf::from(p).join("Programs").join(KLASOR_AD); }
    }
    PathBuf::from(KLASOR_AD)
}

fn sor(soru: &str, sessiz: bool) -> bool {
    if sessiz { return true; }
    print!("{soru} (E/H, Enter = Evet): ");
    let _ = io::stdout().flush();
    let mut s = String::new();
    let _ = io::stdin().read_line(&mut s);
    !matches!(s.trim().to_lowercase().as_str(), "h" | "hayir" | "hayır" | "n" | "no")
}

fn bekle(sessiz: bool) {
    if sessiz { return; }
    print!("Kapatmak için Enter'a basın…");
    let _ = io::stdout().flush();
    let mut s = String::new();
    let _ = io::stdin().read_line(&mut s);
}

fn arg(a: &[String], k: &str) -> Option<String> { a.iter().position(|x| x == k).and_then(|i| a.get(i + 1)).cloned() }

#[cfg(windows)]
fn powershell(komut: &str) -> bool {
    use std::os::windows::process::CommandExt;
    Command::new("powershell").args(["-NoProfile", "-NonInteractive", "-ExecutionPolicy", "Bypass", "-Command", komut]).creation_flags(0x0800_0000).status().map(|s| s.success()).unwrap_or(false)
}

#[cfg(windows)]
fn reg(args: &[&str]) -> bool {
    use std::os::windows::process::CommandExt;
    Command::new("reg").args(args).creation_flags(0x0800_0000).status().map(|s| s.success()).unwrap_or(false)
}

/// Masaüstü ve Başlat menüsü kısayolları; oluşturulanların yolunu döndürür.
#[cfg(windows)]
fn kisayol_olustur(hedef: &Path) -> Vec<String> {
    let exe = hedef.join("taa-vidyo-yapici.exe");
    let ico = hedef.join("simge.ico");
    let mut yollar = vec![];
    for (klasor, ad) in [("Desktop", format!("{AD}.lnk")), ("Programs", format!("{AD}.lnk"))] {
        let ps = format!(
            "$d=[Environment]::GetFolderPath('{klasor}'); $y=Join-Path $d '{ad}'; $s=(New-Object -ComObject WScript.Shell).CreateShortcut($y); $s.TargetPath='{exe}'; $s.Arguments='--arayuz'; $s.WorkingDirectory='{dir}'; $s.IconLocation='{ico}'; $s.Description='{AD}'; $s.Save(); Write-Output $y",
            exe = exe.display(), dir = hedef.display(), ico = ico.display());
        use std::os::windows::process::CommandExt;
        if let Ok(o) = Command::new("powershell").args(["-NoProfile", "-NonInteractive", "-ExecutionPolicy", "Bypass", "-Command", &ps]).creation_flags(0x0800_0000).output() {
            if o.status.success() { let y = String::from_utf8_lossy(&o.stdout).trim().to_string(); if !y.is_empty() { yollar.push(y); } }
        }
    }
    yollar
}
#[cfg(not(windows))]
fn kisayol_olustur(_: &Path) -> Vec<String> { vec![] }

#[cfg(windows)]
fn kayit_yaz(hedef: &Path) {
    let kal = hedef.join("kaldir.exe").display().to_string();
    let ico = hedef.join("simge.ico").display().to_string();
    let uz = hedef.display().to_string();
    for (ad, tur, v) in [("DisplayName", "REG_SZ", AD.to_string()), ("DisplayVersion", "REG_SZ", SURUM.to_string()), ("Publisher", "REG_SZ", "TAA".to_string()),
        ("InstallLocation", "REG_SZ", uz), ("UninstallString", "REG_SZ", format!("\"{kal}\"")), ("DisplayIcon", "REG_SZ", ico), ("NoModify", "REG_DWORD", "1".into()), ("NoRepair", "REG_DWORD", "1".into())] {
        reg(&["add", KAYIT, "/v", ad, "/t", tur, "/d", &v, "/f"]);
    }
}
#[cfg(not(windows))]
fn kayit_yaz(_: &Path) {}

fn ffmpeg_var_mi(hedef: &Path) -> bool {
    let ad = if cfg!(windows) { "ffmpeg.exe" } else { "ffmpeg" };
    if std::env::var("LED_FFMPEG").map(|p| Path::new(&p).is_file()).unwrap_or(false) { return true; }
    if hedef.join(ad).is_file() || hedef.join("ffmpeg").join("bin").join(ad).is_file() { return true; }
    if cfg!(windows) {
        fn ara(k: &Path, d: u32, ad: &str) -> bool {
            if k.join(ad).is_file() || k.join("bin").join(ad).is_file() { return true; }
            if d == 0 { return false; }
            fs::read_dir(k).map(|r| r.filter_map(|e| e.ok()).map(|e| e.path()).filter(|p| p.is_dir()).any(|p| ara(&p, d - 1, ad))).unwrap_or(false)
        }
        if ara(Path::new(r"D:\Programlar\Araçlar Ve İndirilenler"), 3, ad) { return true; }
    }
    std::env::var_os("PATH").map(|p| std::env::split_paths(&p).any(|k| k.join(ad).is_file())).unwrap_or(false)
}

fn yaz(yol: &Path, veri: &[u8]) -> Result<(), String> {
    if let Some(k) = yol.parent() { fs::create_dir_all(k).map_err(|e| format!("{}: {e}", k.display()))?; }
    fs::write(yol, veri).map_err(|e| format!("{} yazılamadı ({e}). Program açıksa kapatıp yeniden deneyin.", yol.display()))
}

fn kur(a: &[String]) -> i32 {
    let sessiz = a.iter().any(|x| x == "--sessiz");
    let hedef = arg(a, "--hedef").map(PathBuf::from).unwrap_or_else(varsayilan_hedef);
    println!("{AD} {SURUM} kurulumu");
    println!("Kurulum klasörü: {}", hedef.display());
    if !sor("Kurulsun mu?", sessiz) { println!("Vazgeçildi."); return 0; }
    let mut liste: Vec<String> = vec![];
    for (ad, veri) in DOSYALAR {
        if let Err(e) = yaz(&hedef.join(ad), veri) { eprintln!("HATA: {e}"); bekle(sessiz); return 1; }
        liste.push(ad.to_string());
    }
    // kaldırma programı: bu exenin kopyası, "kaldir" adıyla
    match std::env::current_exe().and_then(|e| fs::read(e)) {
        Ok(v) => { if let Err(e) = yaz(&hedef.join("kaldir.exe"), &v) { eprintln!("HATA: {e}"); bekle(sessiz); return 1; } liste.push("kaldir.exe".into()); }
        Err(e) => { eprintln!("HATA: kaldırma programı yazılamadı: {e}"); bekle(sessiz); return 1; }
    }
    let kisayollar = if a.iter().any(|x| x == "--kisayolsuz") { vec![] } else { kisayol_olustur(&hedef) };
    if !a.iter().any(|x| x == "--kisayolsuz") { kayit_yaz(&hedef); }
    let mut m = liste.join("\n");
    for k in &kisayollar { m.push_str(&format!("\n@{k}")); }
    m.push('\n');
    if let Err(e) = yaz(&hedef.join(MANIFEST), m.as_bytes()) { eprintln!("HATA: {e}"); bekle(sessiz); return 1; }
    println!("Kurulum tamam: {} dosya yazıldı.", liste.len());
    if !kisayollar.is_empty() { println!("Masaüstü ve Başlat menüsü kısayolları oluşturuldu."); }
    if !ffmpeg_var_mi(&hedef) {
        println!("\nUYARI: ffmpeg bulunamadı. Video üretmek için ffmpeg.exe gerekir.\n  ffmpeg.exe dosyasını {} klasörüne koyun\n  veya D:\\Programlar\\Araçlar Ve İndirilenler altına kurun.", hedef.display());
    }
    println!("Videolar varsayılan olarak Videolar\\TAA Vidyo Yapıcı klasörüne yazılır.");
    if cfg!(windows) && !sessiz && sor("Şimdi başlatılsın mı?", false) {
        let _ = Command::new(hedef.join("taa-vidyo-yapici.exe")).arg("--arayuz").spawn();
    }
    0
}

fn kaldir(a: &[String]) -> i32 {
    let sessiz = a.iter().any(|x| x == "--sessiz");
    let hedef = arg(a, "--hedef").map(PathBuf::from).unwrap_or_else(|| std::env::current_exe().ok().and_then(|e| e.parent().map(|p| p.to_path_buf())).unwrap_or_else(varsayilan_hedef));
    let man = match fs::read_to_string(hedef.join(MANIFEST)) { Ok(m) => m, Err(_) => { eprintln!("HATA: {} içinde {MANIFEST} yok; kurulu program bulunamadı.", hedef.display()); bekle(sessiz); return 1; } };
    println!("{AD} kaldırılacak: {}", hedef.display());
    println!("Videolarınıza ve kendi eklediğiniz senaryolara dokunulmaz.");
    if !sor("Kaldırılsın mı?", sessiz) { println!("Vazgeçildi."); return 0; }
    let kendi = std::env::current_exe().ok().and_then(|e| e.file_name().map(|n| n.to_string_lossy().to_lowercase()));
    let mut hata = false;
    let mut kisayollar = vec![];
    let mut dosyalar = vec![];
    for s in man.lines().map(str::trim).filter(|s| !s.is_empty()) { if let Some(k) = s.strip_prefix('@') { kisayollar.push(k.to_string()) } else { dosyalar.push(s.to_string()) } }
    for k in &kisayollar { let _ = fs::remove_file(k); }
    let mut kaldir_sonra = false;
    for d in &dosyalar {
        let yol = d.split('/').fold(hedef.clone(), |p, x| p.join(x));
        if kendi.as_deref() == yol.file_name().map(|n| n.to_string_lossy().to_lowercase()).as_deref() && yol.parent() == Some(hedef.as_path()) { kaldir_sonra = true; continue; }
        if yol.exists() { if let Err(e) = fs::remove_file(&yol) { eprintln!("Silinemedi: {} ({e}). Program açıksa kapatın.", yol.display()); hata = true; } }
    }
    let _ = fs::remove_file(hedef.join(MANIFEST));
    // boş alt klasörleri kaldır (içinde kullanıcı dosyası varsa kalır)
    let mut klasorler: Vec<PathBuf> = dosyalar.iter().filter_map(|d| d.rsplit_once('/').map(|(k, _)| k.split('/').fold(hedef.clone(), |p, x| p.join(x)))).collect();
    klasorler.sort(); klasorler.dedup(); klasorler.reverse();
    for k in klasorler { let _ = fs::remove_dir(k); }
    kayit_sil();
    if hata { eprintln!("Bazı dosyalar silinemedi; programı kapatıp kaldırmayı yeniden çalıştırın."); bekle(sessiz); return 1; }
    println!("Kaldırma tamam.");
    // çalışan kaldırma exesi kendini silemez: kısa gecikmeyle cmd siler
    #[cfg(windows)]
    if kaldir_sonra {
        use std::os::windows::process::CommandExt;
        let _ = Command::new("cmd").args(["/c", &format!("ping -n 3 127.0.0.1 >nul & del /f /q \"{}\" & rmdir \"{}\"", hedef.join("kaldir.exe").display(), hedef.display())]).creation_flags(0x0800_0000).spawn();
    }
    #[cfg(not(windows))]
    if kaldir_sonra { let _ = fs::remove_file(hedef.join("kaldir.exe")); let _ = fs::remove_dir(&hedef); }
    bekle(sessiz);
    0
}

#[cfg(windows)]
fn kayit_sil() { reg(&["delete", KAYIT, "/f"]); }
#[cfg(not(windows))]
fn kayit_sil() {}

fn main() {
    konsol_utf8();
    let a: Vec<String> = std::env::args().collect();
    let ad = a.first().map(|p| Path::new(p).file_stem().map(|s| s.to_string_lossy().to_lowercase()).unwrap_or_default()).unwrap_or_default();
    let kod = if a.iter().any(|x| x == "--kaldir") || ad.starts_with("kaldir") { kaldir(&a) } else { kur(&a) };
    std::process::exit(kod);
}
