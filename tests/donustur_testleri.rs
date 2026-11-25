// Açıklama: Dönüştürücü bütünleşik testleri: her tür için gerçek ffmpeg ile 950x1800 PNG önizleme (ffmpeg yoksa atlanır),
// kısa gerçek dönüşümler (çizgi film ve piksel art örnekleri tasarim/yenileme/ornek/donusum-*.mp4 + kare PNG), ilerlemeli iş
// (başlat/durum/iptal), HTTP uçları ve web/donustur.js Node testi.
// Çalıştırma: tasarim/yenileme/kilitli-cargo.sh test --release --test donustur_testleri -- --nocapture --test-threads=1
// Sahip Yusuf; yazan [Claude#Dönüştürücü Opus 5.5, yenileme-1.0, 2026-11-25]; yalnız yazan yapay zekâ ya da Yusuf'un izniyle değişir; silme yok.
// Dalga 4 [Claude#SEC-FIX Opus 5.5, yenileme-1.0, 2026-11-25]: m3u8 içerikli .mp4 reddi, önizleme tekliği ve zaman aşımı, eşzamanlı/arka
// plan iş yalıtımı. Kırık testlerin sebebi: donustur_esz arka plan işinin ortak IPTAL/COCUK kaydını paylaşıyordu; paralel koşuda
// ilerlemeli_is testindeki iptal ornek_donusumler'in ffmpeg'ini öldürüyordu. Kod düzeltildi; d::iptal() çağıran testler ayrıca sıralı.
use std::path::{Path, PathBuf};
use std::sync::{Mutex, MutexGuard};
use std::time::{Duration, Instant};
use taa_vidyo_yapici::{donustur as d, donustur_http as h, ffmpeg};

static ARKA: Mutex<()> = Mutex::new(());
/// Ortak arka plan işini (baslat/iptal) kullanan testler aynı anda koşmaz.
fn arka_sirasi() -> MutexGuard<'static, ()> { ARKA.lock().unwrap_or_else(|p| p.into_inner()) }

fn kok() -> PathBuf { PathBuf::from(env!("CARGO_MANIFEST_DIR")) }
fn ornek() -> PathBuf { kok().join("tasarim/yenileme/ornek") }
fn kaynak() -> PathBuf { ornek().join("yutkun.mp4") }
fn al(v: &[(&str, &str)]) -> Vec<(String, String)> { v.iter().map(|(a, b)| (a.to_string(), b.to_string())).collect() }
fn png_boyut(p: &[u8]) -> (u32, u32) {
    assert!(p.starts_with(b"\x89PNG"), "PNG değil");
    (u32::from_be_bytes(p[16..20].try_into().unwrap()), u32::from_be_bytes(p[20..24].try_into().unwrap()))
}
fn hazir() -> bool {
    if ffmpeg::bul().is_none() { eprintln!("ffmpeg yok: gerçek dönüşüm testleri atlandı"); return false; }
    if !kaynak().is_file() { eprintln!("örnek video yok: atlandı"); return false; }
    true
}

#[test]
fn her_tur_gercek_onizleme_950x1800() {
    if !hazir() { return; }
    for t in d::TURLER {
        for (s, kucuk) in [("kirp", false), ("doldur", true)] {
            let a = d::ayar_coz(&al(&[("tur", t.id), ("sigdir", s)])).unwrap();
            let bas = Instant::now();
            let png = d::onizle(&a, &kaynak(), 2.0, kucuk).unwrap_or_else(|e| panic!("{} {s}: {e}\n{}", t.id, d::zincir(&a, d::PaletKaynak::Icinde, kucuk)));
            let beklenen = if kucuk { (d::KUCUK_W, d::KUCUK_H) } else { (d::W, d::H) };
            assert_eq!(png_boyut(&png), beklenen, "{} {s}", t.id);
            if let (Some(k), false) = (std::env::var_os("DONUSTUR_KARELER"), kucuk) { std::fs::write(PathBuf::from(k).join(format!("{}.png", t.id)), &png).unwrap(); }
            eprintln!("önizleme {:<10} {s:<6} {:>5} ms  {} KB", t.id, bas.elapsed().as_millis(), png.len() / 1024);
        }
    }
    // sabit palet ve sığdır
    for p in ["retro16", "gameboy", "sepya"] {
        let a = d::ayar_coz(&al(&[("tur", "piksel"), ("palet", p), ("blok", "25"), ("sigdir", "sigdir"), ("dither", "floyd")])).unwrap();
        assert_eq!(png_boyut(&d::onizle(&a, &kaynak(), 1.0, false).unwrap()), (d::W, d::H), "{p}");
    }
    // resim kaynağı
    let r = ornek().join("yutkun-kare.png");
    if r.is_file() {
        let a = d::ayar_coz(&al(&[("tur", "duotone"), ("palet", "bordo-krem")])).unwrap();
        assert_eq!(png_boyut(&d::onizle(&a, &r, 0.0, false).unwrap()), (d::W, d::H));
    }
}

#[test]
fn ornek_donusumler_uretilir() {
    if !hazir() { return; }
    let ff = ffmpeg::bul().unwrap();
    for (tur, ek) in [("cizgifilm", vec![("yogunluk", "60")]), ("piksel", vec![("blok", "10"), ("renk", "24")]), ("retro8bit", vec![])] {
        let mut v = vec![("tur", tur), ("sure", "3"), ("baslangic", "1")];
        v.extend(ek);
        let a = d::ayar_coz(&al(&v)).unwrap();
        let cikis = ornek().join(format!("donusum-{tur}.mp4"));
        let bas = Instant::now();
        let m = d::donustur_esz(&ff, &a, &kaynak(), false, 3.0, &cikis).unwrap_or_else(|e| panic!("{tur}: {e}"));
        let sn = bas.elapsed().as_secs_f32();
        let boyut = std::fs::metadata(&cikis).unwrap().len();
        assert!(boyut > 10_000, "{tur}: çıktı çok küçük");
        // çıktı 950x1800 ve ~3 sn mi
        let o = std::process::Command::new(&ff).args(["-hide_banner", "-i"]).arg(&cikis).output().unwrap();
        let bilgi = String::from_utf8_lossy(&o.stderr);
        assert!(bilgi.contains("950x1800"), "{tur}: {bilgi}");
        let sure = d::sure_ayikla(&bilgi).unwrap();
        assert!((sure - 3.0).abs() < 0.3, "{tur}: süre {sure}");
        let kare = d::onizle(&a, &kaynak(), 1.5, false).unwrap();
        std::fs::write(ornek().join(format!("donusum-{tur}-kare.png")), &kare).unwrap();
        eprintln!("{m}  {tur}: {sn:.1} sn, {} KB", boyut / 1024);
    }
    // resimden piksel giriş animasyonu (3 sn): ilk kare iri pikselli, son kare net olmalı
    let r = ornek().join("yutkun-kare.png");
    if r.is_file() {
        let a = d::ayar_coz(&al(&[("tur", "piksel-giris"), ("sure", "3")])).unwrap();
        let cikis = ornek().join("donusum-piksel-giris.mp4");
        d::donustur_esz(&ff, &a, &r, true, 3.0, &cikis).unwrap();
        let kare = |t: &str| std::process::Command::new(&ff).args(["-hide_banner", "-loglevel", "error", "-ss", t, "-i"]).arg(&cikis).args(["-frames:v", "1", "-f", "image2pipe", "-c:v", "png", "-"]).output().unwrap().stdout;
        let (ilk, son) = (kare("0"), kare("2.9"));
        assert_eq!(png_boyut(&ilk), (d::W, d::H));
        assert!(son.len() > ilk.len() * 2, "son kare daha ayrıntılı olmalı ({} / {})", son.len(), ilk.len());
        std::fs::write(ornek().join("donusum-piksel-giris-kare.png"), &son).unwrap();
        eprintln!("piksel-giris: ilk kare {} KB, son kare {} KB", ilk.len() / 1024, son.len() / 1024);
    }
}

#[test]
fn ilerlemeli_is_baslat_durum_iptal() {
    if !hazir() { return; }
    let _s = arka_sirasi();
    let k = std::env::temp_dir().join(format!("taa-donustur-is-{}", std::process::id()));
    std::fs::create_dir_all(&k).unwrap();
    let kaynak_kopya = k.join("kaynak.mp4");
    std::fs::copy(kaynak(), &kaynak_kopya).unwrap();
    // 1) tamamlanan iş (HTTP üzerinden)
    let govde = format!("{{\"kaynak\":\"{}\",\"tur\":\"karakalem\",\"sure\":2,\"fps\":12}}", h::jk(&kaynak_kopya.display().to_string()));
    let (kod, _, g) = h::isle("POST", "baslat", &govde, &k);
    assert_eq!(kod, 200, "{g}");
    assert_eq!(h::isle("POST", "baslat", &govde, &k).0, 409, "aynı anda tek iş");
    let bas = Instant::now();
    loop {
        let s = d::durum();
        if !s.calisiyor { assert!(!s.hata, "{}", s.mesaj); assert_eq!(s.kare, s.toplam); assert_eq!(s.toplam, 24); break; }
        assert!(bas.elapsed() < Duration::from_secs(120), "iş bitmedi");
        std::thread::sleep(Duration::from_millis(200));
    }
    let cikti = k.join("kaynak-karakalem.mp4");
    assert!(cikti.is_file());
    assert_eq!(d::son_cikti(&k).map(|p| p.file_name().unwrap().to_owned()), Some(cikti.file_name().unwrap().to_owned()));
    let (_, _, dj) = h::isle("GET", "durum", "", &k);
    assert!(dj.contains("\"yuzde\":100.0") && dj.contains("\"hata\":false"), "{dj}");
    // var olan çıktı ezilmez
    assert!(d::cikti_yolu(&k, &kaynak_kopya, "karakalem").ends_with("kaynak-karakalem-2.mp4"));
    // 2) iptal edilen iş: çıktı klasörüne yarım dosya bırakılmaz
    let govde2 = format!("{{\"kaynak\":\"{}\",\"tur\":\"cizgiroman\",\"sure\":20}}", h::jk(&kaynak_kopya.display().to_string()));
    assert_eq!(h::isle("POST", "baslat", &govde2, &k).0, 200);
    std::thread::sleep(Duration::from_millis(800));
    assert_eq!(h::isle("POST", "iptal", "", &k).0, 200);
    let bas = Instant::now();
    while d::durum().calisiyor { assert!(bas.elapsed() < Duration::from_secs(30), "iptal çalışmadı"); std::thread::sleep(Duration::from_millis(100)); }
    let s = d::durum();
    assert!(s.hata && s.mesaj.contains("iptal"), "{:?}", s);
    assert!(!k.join("kaynak-cizgiroman.mp4").exists(), "iptalde yarım çıktı kalmamalı");
    assert!(d::son_cikti(&k).is_none());
}

#[test]
fn http_kaynak_guvenligi() {
    let k = std::env::temp_dir().join(format!("taa-donustur-http-{}", std::process::id()));
    std::fs::create_dir_all(&k).unwrap();
    std::fs::write(k.join("a.mp4"), b"x").unwrap();
    std::fs::write(k.join("gizli.txt"), b"x").unwrap();
    let (kod, _, j) = h::isle("GET", "kaynaklar", "", &k);
    assert_eq!(kod, 200);
    assert!(j.contains("a.mp4") && !j.contains("gizli.txt"), "{}", &j[..j.len().min(300)]);
    for yol in [k.join("gizli.txt"), k.join("..").join("baska.mp4"), PathBuf::from("a.mp4"), Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml")] {
        let g = format!("{{\"kaynak\":\"{}\",\"tur\":\"piksel\"}}", h::jk(&yol.display().to_string()));
        let (kod, _, j) = h::isle("POST", "onizle", &g, &k);
        assert_eq!(kod, 400, "{yol:?} {j}");
    }
    // sunucu kaydı yoksa oynat 404
    let _ = d::durum();
}

// ---- Dalga 4 [Claude#SEC-FIX Opus 5.5, yenileme-1.0, 2026-11-25]: güvenlik bulguları ----

/// (1) Uzantıdan biçim zorlanır: .mp4 adlı m3u8/HLS listesi (uzak ya da yerel bölüm adresli) açılmaz.
#[test]
fn m3u8_icerikli_mp4_okunmaz() {
    if ffmpeg::bul().is_none() { eprintln!("ffmpeg yok: atlandı"); return; }
    let k = std::env::temp_dir().join(format!("taa-donustur-hls-{}", std::process::id()));
    std::fs::create_dir_all(&k).unwrap();
    let yerel = if kaynak().is_file() { format!("file:{}", kaynak().display()) } else { "file:C:/Windows/win.ini".into() };
    for (ad, bolum) in [("uzak.mp4", "http://127.0.0.1:9/x.ts".to_string()), ("yerel.mp4", yerel), ("yerel.webm", "x.ts".into())] {
        let p = k.join(ad);
        std::fs::write(&p, format!("#EXTM3U\n#EXT-X-TARGETDURATION:10\n#EXTINF:10,\n{bolum}\n#EXT-X-ENDLIST\n")).unwrap();
        let a = d::ayar_coz(&al(&[("tur", "cizgifilm")])).unwrap();
        let r = d::onizle(&a, &p, 0.0, true);
        assert!(r.is_err(), "{ad}: m3u8 içerikli dosya okunmamalı");
        // HTTP yolunda da 500 (okunamadı), içerik sızmaz
        let g = format!("{{\"kaynak\":\"{}\",\"tur\":\"cizgifilm\",\"kucuk\":true}}", h::jk(&p.display().to_string()));
        let (kod, _, j) = h::isle("POST", "onizle", &g, &k);
        assert_eq!(kod, 500, "{ad}: {j}");
        assert!(!j.contains("png"), "{ad}");
    }
}

/// (2) Önizleme aynı anda tek ffmpeg; 25 sn sınırı aşılırsa süreç öldürülür (burada sınır 1 ms ile sınanır).
#[test]
fn onizleme_tek_ve_zaman_asimli() {
    if !hazir() { return; }
    let a = d::ayar_coz(&al(&[("tur", "boyama")])).unwrap();
    let bas = Instant::now();
    let r = d::onizle_sinirli(&a, &kaynak(), 1.0, false, Duration::from_millis(1));
    let e = r.expect_err("1 ms'de önizleme bitmemeli");
    assert!(e.contains("zaman aşımı"), "{e}");
    assert!(bas.elapsed() < Duration::from_secs(10), "zaman aşımında süreç beklenmeden öldürülmeli");
    assert_eq!(d::ONIZLE_SURE, Duration::from_secs(25));
    let ipler: Vec<_> = (0..4).map(|i| std::thread::spawn(move || {
        let a = d::ayar_coz(&al(&[("tur", "cizgifilm")])).unwrap();
        d::onizle(&a, &kaynak(), i as f32 * 0.5, true)
    })).collect();
    for ip in ipler { let png = ip.join().unwrap().expect("sıradaki önizleme de üretilmeli"); assert_eq!(png_boyut(&png), (d::KUCUK_W, d::KUCUK_H)); }
    assert_eq!(d::onizle_en_cok_esz(), 1, "aynı anda birden çok önizleme ffmpeg'i çalıştı");
}

/// Fable: eşzamanlı dönüştürme ile arka plan işi birbirini iptal etmez (paralel koşudaki "cizgifilm: iptal edildi").
#[test]
fn eszamanli_donusum_arka_iptalinden_etkilenmez() {
    if !hazir() { return; }
    let ff = ffmpeg::bul().unwrap();
    let k = std::env::temp_dir().join(format!("taa-donustur-yalitim-{}", std::process::id()));
    std::fs::create_dir_all(&k).unwrap();
    let a = d::ayar_coz(&al(&[("tur", "karakalem"), ("sure", "1"), ("fps", "10")])).unwrap();
    let _s = arka_sirasi(); // d::iptal() ortak arka plan işini de iptal eder: ilerlemeli_is testiyle aynı anda koşmasın
    let k2 = k.clone();
    let ip = std::thread::spawn(move || d::donustur_esz(&ff, &a, &kaynak(), false, 1.0, &k2.join("esz.mp4")));
    // arka plan işi yokken bile iptal çağrıları eşzamanlı dönüşümü durdurmamalı
    for _ in 0..20 { d::iptal(); std::thread::sleep(Duration::from_millis(50)); }
    let m = ip.join().unwrap().expect("eşzamanlı dönüşüm iptal edilmemeli");
    assert!(m.starts_with("Hazır") && k.join("esz.mp4").is_file(), "{m}");
}

#[test]
fn donustur_js_node_testi() {
    let betik = kok().join("tests/donustur_test.js");
    match std::process::Command::new("node").arg(&betik).output() {
        Ok(o) => assert!(o.status.success(), "{}\n{}", String::from_utf8_lossy(&o.stdout), String::from_utf8_lossy(&o.stderr)),
        Err(_) => eprintln!("node bulunamadı, donustur_test.js atlandı"),
    }
}
