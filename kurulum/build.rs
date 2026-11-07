// Kurulum paketine gömülecek dosyalar LED_PAKET klasöründen alınır (derle-windows.sh hazırlar).
use std::{env, fs, path::{Path, PathBuf}};

fn topla(kok: &Path, k: &Path, out: &mut Vec<String>) {
    let mut e: Vec<PathBuf> = fs::read_dir(k).unwrap().map(|x| x.unwrap().path()).collect();
    e.sort();
    for p in e {
        if p.is_dir() { topla(kok, &p, out); }
        else { out.push(p.strip_prefix(kok).unwrap().to_string_lossy().replace('\\', "/")); }
    }
}

fn main() {
    println!("cargo:rerun-if-env-changed=LED_PAKET");
    let paket = env::var("LED_PAKET").unwrap_or_else(|_| panic!("LED_PAKET ortam değişkeni gerekli (derle-windows.sh kullanın)"));
    let kok = PathBuf::from(&paket).canonicalize().expect("LED_PAKET klasörü yok");
    println!("cargo:rerun-if-changed={}", kok.display());
    let mut liste = vec![];
    topla(&kok, &kok, &mut liste);
    let mut kod = String::from("pub const DOSYALAR: &[(&str, &[u8])] = &[\n");
    for r in &liste { kod.push_str(&format!("    ({:?}, include_bytes!({:?})),\n", r, kok.join(r).to_string_lossy())); }
    kod.push_str("];\n");
    let surum = fs::read_to_string("../Cargo.toml").ok().and_then(|t| t.lines().find_map(|l| l.strip_prefix("version = ").map(|v| v.trim().trim_matches('"').to_string()))).unwrap_or_else(|| "0".into());
    kod.push_str(&format!("pub const SURUM: &str = {surum:?};\n"));
    fs::write(PathBuf::from(env::var("OUT_DIR").unwrap()).join("dosyalar.rs"), kod).unwrap();
}
