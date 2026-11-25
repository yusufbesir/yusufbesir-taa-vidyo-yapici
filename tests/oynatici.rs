//! Oynatıcı uç testleri: ad çözme ve yol geçişi, Range, akışlı medya, proje doğrulama, kalıcılık, sürüm/komut/uzun yoklama, sayfa sözleşmesi, node motor testi.
//! Sahip Yusuf; yazan [Claude#Oynatici Opus 5.5, yenileme-1.0, 2026-11-25]; yalnız yazan yapay zekâ ya da Yusuf'un izniyle değişir; silme yok.
//! Geçici dosyalar sistem geçici klasörüne yazılır; testler dosya silmez (temizlik Yusuf'un kararı: Geri Dönüşüm Kutusu).
use std::path::PathBuf;
use std::sync::Once;
use std::time::{Duration, Instant};
use taa_vidyo_yapici::oynatici_http as oh;
use taa_vidyo_yapici::oynatici_medya as om;

fn gecici(ad: &str) -> PathBuf {
    let k = std::env::temp_dir().join("taa-oynatici-test").join(format!("{ad}-{}", std::process::id()));
    std::fs::create_dir_all(&k).unwrap();
    k
}
static HAZIR: Once = Once::new();
fn hazirla() { HAZIR.call_once(|| std::env::set_var("TAA_OYNATICI_DOSYASI", gecici("ortak").join("oynatici.json"))); }

fn istek(yontem: &str, yol: &str, bas: &[(&str, &str)], govde: &str, kok: &std::path::Path) -> (u16, String, Vec<u8>) {
    let bas: Vec<(String, String)> = bas.iter().map(|(a, b)| (a.to_string(), b.to_string())).collect();
    let mut v = vec![];
    oh::yanitla(&mut v, yontem, yol, &bas, govde.as_bytes(), &|| false, kok, Duration::from_millis(300));
    let i = v.windows(4).position(|w| w == b"\r\n\r\n").unwrap();
    let b = String::from_utf8_lossy(&v[..i]).into_owned();
    let kod = b.split_whitespace().nth(1).unwrap().parse().unwrap();
    (kod, b, v[i + 4..].to_vec())
}

#[test]
fn medya_ad_cozme_ve_yol_gecisi() {
    let kok = gecici("medya");
    std::fs::write(kok.join("Kış Kampanyası.mp4"), b"0123456789").unwrap();
    std::fs::write(kok.join("x.MP4"), b"ab").unwrap();
    let dis = gecici("dis");
    std::fs::write(dis.join("gizli.mp4"), b"GIZLI").unwrap();
    assert!(om::coz(&kok, "Kış Kampanyası.mp4").is_some());
    assert!(om::coz(&kok, "x.MP4").is_some());
    for kotu in ["..\\dis\\gizli.mp4", "../gizli.mp4", "a/b.mp4", "C:x.mp4", "x.mp4.", "x.mp4 ", ".gizli.mp4", "x.exe", "yok.mp4"] {
        assert!(om::coz(&kok, kotu).is_none(), "{kotu}");
    }
    for yol in ["/oynatici/medya/%2e%2e%2fgizli.mp4", "/oynatici/medya/..%5Cgizli.mp4", "/oynatici/medya/%252e%252e%252fgizli.mp4", "/oynatici/medya/x.exe", &format!("/oynatici/medya/{}.mp4", "a".repeat(197))] {
        let (kod, _, g) = istek("GET", yol, &[], "", &kok);
        assert_eq!(kod, 404, "{yol}");
        assert!(!g.windows(5).any(|w| w == b"GIZLI"));
    }
    let (kod, b, g) = istek("GET", "/oynatici/medya/K%C4%B1%C5%9F%20Kampanyas%C4%B1.mp4?v=1", &[], "", &kok);
    assert_eq!(kod, 200); assert_eq!(g, b"0123456789");
    assert!(b.contains("Accept-Ranges: bytes") && b.contains("video/mp4") && b.contains("nosniff") && b.contains("Content-Length: 10"));
    // Kök dışına bağlantı (oluşturulabiliyorsa; Windows'ta yetki gerekebilir)
    #[cfg(windows)]
    let bag = std::os::windows::fs::symlink_file(dis.join("gizli.mp4"), kok.join("bag.mp4"));
    #[cfg(not(windows))]
    let bag = std::os::unix::fs::symlink(dis.join("gizli.mp4"), kok.join("bag.mp4"));
    if bag.is_ok() { assert!(om::coz(&kok, "bag.mp4").is_none(), "bağlantı kök dışına çıkmamalı"); } else { eprintln!("bağlantı oluşturulamadı, bağlantı denetimi atlandı"); }
}

#[test]
fn range_yanitlari() {
    let kok = gecici("range");
    let veri: Vec<u8> = (0..5_000_000u32).map(|i| (i % 251) as u8).collect();
    std::fs::write(kok.join("b.mp4"), &veri).unwrap();
    let y = "/oynatici/medya/b.mp4";
    let (kod, b, g) = istek("GET", y, &[("range", "bytes=0-")], "", &kok);
    assert_eq!(kod, 206); assert!(b.contains("Content-Range: bytes 0-4999999/5000000")); assert_eq!(g, veri, "parça parça okuma tam dosyayla aynı olmalı");
    let (kod, b, g) = istek("GET", y, &[("range", "bytes=0-0")], "", &kok);
    assert_eq!(kod, 206); assert!(b.contains("Content-Length: 1")); assert_eq!(g, &veri[..1]);
    let (_, b, g) = istek("GET", y, &[("range", "bytes=-500")], "", &kok);
    assert!(b.contains("bytes 4999500-4999999/5000000")); assert_eq!(g, &veri[4_999_500..]);
    let (_, _, g) = istek("GET", y, &[("range", "bytes=4999000-9999999")], "", &kok);
    assert_eq!(g, &veri[4_999_000..]);
    let (kod, b, g) = istek("GET", y, &[("range", "bytes=5000000-")], "", &kok);
    assert_eq!(kod, 416); assert!(b.contains("Content-Range: bytes */5000000")); assert!(g.len() < 100);
    assert_eq!(istek("GET", y, &[("range", "bytes=5-2")], "", &kok).0, 416);
    assert_eq!(istek("GET", y, &[("range", "bytes=0-1,4-5")], "", &kok).0, 200);
    assert_eq!(istek("GET", y, &[("range", "bozuk")], "", &kok).0, 200);
    let (kod, b, g) = istek("HEAD", y, &[], "", &kok);
    assert_eq!(kod, 200); assert!(b.contains("Content-Length: 5000000")); assert!(g.is_empty(), "HEAD gövdesiz");
    // küçük resim ucu yalnız resim verir
    std::fs::write(kok.join("r.png"), b"PNG").unwrap();
    assert_eq!(istek("GET", "/oynatici/kucuk/r.png", &[], "", &kok).0, 200);
    assert_eq!(istek("GET", "/oynatici/kucuk/b.mp4", &[], "", &kok).0, 404);
}

#[test]
fn medya_listesi() {
    let kok = gecici("liste");
    for ad in ["a.mp4", "b.png", "c.mov", "d.txt", "e.webm"] { std::fs::write(kok.join(ad), b"x").unwrap(); }
    let j = oh::json_coz(&om::liste(&kok)).unwrap();
    let oh::J::D(v) = j else { panic!() };
    assert_eq!(v.len(), 4, "txt listelenmez");
    let mov = v.iter().find(|x| x.al("ad") == Some(&oh::J::Y("c.mov".into()))).unwrap();
    assert_eq!(mov.al("oynatilabilir"), Some(&oh::J::M(false)));
}

fn proje_metni(katman: usize, klip: usize) -> String {
    let klipler: Vec<String> = (0..klip).map(|i| format!("{{\"tur\":\"video\",\"ad\":\"k{i}.mp4\"}}")).collect();
    let k: Vec<String> = (0..katman).map(|i| format!("{{\"id\":{},\"klipler\":[{}]}}", i + 1, klipler.join(","))).collect();
    format!("{{\"katmanlar\":[{}]}}", k.join(","))
}

#[test]
fn proje_dogrulama() {
    let d = |s: &str| oh::dogrula(&oh::json_coz(s).unwrap());
    assert!(d(&proje_metni(8, 200)).is_ok());
    assert!(d(&proje_metni(9, 0)).unwrap_err().contains("8 katman"));
    assert!(d(&proje_metni(1, 201)).unwrap_err().contains("200 klip"));
    let p = d("{\"ekranGen\":99999,\"anaSes\":5,\"x\":1,\"katmanlar\":[{\"opaklik\":-1,\"hiz\":9,\"gecisMs\":9000,\"etkin\":99,\"bilinmeyen\":true,\"klipler\":[{\"tur\":\"resim\",\"ad\":\"a.png\",\"sure\":0}]}]}").unwrap();
    assert_eq!(p.ekran_gen, 4096); assert_eq!(p.ana_ses, 1.0);
    let k = &p.katmanlar[0];
    assert_eq!((k.opaklik, k.hiz, k.gecis_ms, k.etkin, k.klipler[0].sure), (0.0, 2.0, 2000, 0, 1.0));
    assert!(!oh::proje_json(&p).contains("bilinmeyen"));
    assert!(d("{\"katmanlar\":[{\"kip\":\"ters\"}]}").is_err());
    assert!(d("{\"katmanlar\":[{\"karisim\":\"isik\"}]}").is_err());
    assert!(d("{\"katmanlar\":[{\"klipler\":[{\"tur\":\"video\",\"ad\":\"..\\\\x.mp4\"}]}]}").is_err());
    assert!(d("{\"katmanlar\":[{\"klipler\":[{\"tur\":\"video\",\"ad\":\"x.png\"}]}]}").is_err());
    let uzun = format!("{{\"katmanlar\":[{{\"ad\":\"{}\"}}]}}", "ş".repeat(500));
    assert_eq!(d(&uzun).unwrap().katmanlar[0].ad.chars().count(), 200);
    let cift = d("{\"katmanlar\":[{\"id\":5},{\"id\":5},{}]}").unwrap();
    let mut idler: Vec<u32> = cift.katmanlar.iter().map(|k| k.id).collect(); idler.sort(); idler.dedup();
    assert_eq!(idler.len(), 3);
}

#[test]
fn kalicilik_ve_bozuk_dosya() {
    let k = gecici("kalici");
    let yol = k.join("oynatici.json");
    let mut p = oh::Proje::default();
    p.katmanlar[0].klipler.push(oh::Klip { tur: "video".into(), ad: "Kış.mp4".into(), sure: 0.0 });
    p.karart = true;
    oh::kaydet(&yol, &p).unwrap();
    assert_eq!(oh::yukle(&yol), p);
    let bozuk = k.join("bozuk.json");
    std::fs::write(&bozuk, "{ bu json değil").unwrap();
    assert_eq!(oh::yukle(&bozuk), oh::Proje::default());
    assert_eq!(std::fs::read_to_string(&bozuk).unwrap(), "{ bu json değil", "bozuk dosyaya dokunulmaz");
    assert!(std::fs::read_dir(&k).unwrap().flatten().any(|e| e.file_name().to_string_lossy().starts_with("bozuk.json.bozuk-")), ".bozuk- kopyası olmalı");
    let buyuk = k.join("buyuk.json");
    std::fs::write(&buyuk, " ".repeat(300 * 1024)).unwrap();
    assert_eq!(oh::yukle(&buyuk), oh::Proje::default());
    assert_eq!(oh::yukle(&k.join("yok.json")), oh::Proje::default());
}

#[test]
fn surum_komut_ve_uzun_yoklama() {
    hazirla();
    let j = oh::json_coz(&oh::proje_oku()).unwrap();
    let surum = match j.al("surum") { Some(oh::J::S(x)) => *x as u64, _ => panic!() };
    // yanlış temel sürüm → 409 + güncel proje
    let govde = format!("{{\"temelSurum\":{},\"proje\":{}}}", surum + 100, proje_metni(1, 0));
    let (kod, g) = oh::proje_yaz(&govde).unwrap_err();
    assert_eq!(kod, 409); assert!(g.contains("\"proje\""));
    assert_eq!(oh::proje_yaz("{bozuk").unwrap_err().0, 400);
    // kötü komut
    assert!(oh::komut_ekle("{\"tur\":\"sil\"}").is_err());
    // komut no artar; kuyruk 64'te döner
    let a = oh::komut_ekle("{\"tur\":\"sonraki\",\"katman\":1}").unwrap();
    let b = oh::komut_ekle("{\"tur\":\"tetikle\",\"katman\":1,\"deger\":3}").unwrap();
    assert!(b > a);
    for _ in 0..70 { oh::komut_ekle("{\"tur\":\"basa\"}").unwrap(); }
    let r = oh::olay_bekle(None, Some(b), Duration::from_millis(50)).unwrap();
    let oh::J::D(k) = oh::json_coz(&r).unwrap().al("komutlar").unwrap().clone() else { panic!() };
    assert!(k.len() <= 64);
    // yeni komut gelince bekleyen yoklama hemen döner
    let son = match oh::json_coz(&r).unwrap().al("komutNo") { Some(oh::J::S(x)) => *x as u64, _ => panic!() };
    let s2 = match oh::json_coz(&r).unwrap().al("surum") { Some(oh::J::S(x)) => *x as u64, _ => panic!() };
    let t = std::thread::spawn(move || { let b = Instant::now(); let r = oh::olay_bekle(Some(s2), Some(son), Duration::from_secs(5)).unwrap(); (b.elapsed(), r) });
    std::thread::sleep(Duration::from_millis(150));
    oh::komut_ekle("{\"tur\":\"sonraki\"}").unwrap();
    let (gecen, r) = t.join().unwrap();
    assert!(gecen < Duration::from_secs(2), "yeni komutta hemen dönmeli: {gecen:?}");
    assert!(r.contains("\"sonraki\""));
    // boşta süre dolunca döner (proje null)
    let r = oh::olay_bekle(None, None, Duration::from_millis(10)).unwrap();
    let s3 = match oh::json_coz(&r).unwrap().al("surum") { Some(oh::J::S(x)) => *x as u64, _ => panic!() };
    let n3 = match oh::json_coz(&r).unwrap().al("komutNo") { Some(oh::J::S(x)) => *x as u64, _ => panic!() };
    let b0 = Instant::now();
    let r = oh::olay_bekle(Some(s3), Some(n3), Duration::from_millis(200)).unwrap();
    if r.contains("\"proje\":null") { assert!(b0.elapsed() >= Duration::from_millis(180)); }
    // doğru sürümle yazma başarılı ve sürüm artar
    let yeni = oh::proje_yaz(&format!("{{\"temelSurum\":{s3},\"proje\":{}}}", proje_metni(2, 1))).unwrap();
    assert!(yeni > s3);
    assert!(std::fs::read_to_string(oh::varsayilan_yol()).unwrap().contains("k0.mp4"));
    // canlı durum: etkin işlenir, sürüm artmaz
    oh::canli_yaz("{\"katmanlar\":[{\"id\":1,\"etkin\":0}],\"olaylar\":[]}").unwrap();
    assert!(oh::canli_oku().contains("\"yasMs\":"));
    assert!(oh::canli_yaz("bozuk").is_err());
}

#[test]
fn uclar_ve_sayfa_sozlesmesi() {
    hazirla();
    let kok = gecici("uclar");
    let (kod, b, g) = istek("GET", "/oynatici", &[], "", &kok);
    assert_eq!(kod, 200);
    assert!(b.contains("Content-Security-Policy: default-src 'none'; script-src 'self'"));
    let html = String::from_utf8(g).unwrap();
    let (_, b2, g2) = istek("GET", "/cikis", &[], "", &kok);
    assert!(b2.contains("Content-Security-Policy"));
    let cikis = String::from_utf8(g2).unwrap();
    for (ad, h) in [("oynatici.html", &html), ("cikis.html", &cikis)] {
        assert!(!h.contains("<style"), "{ad}: satır içi stil");
        assert!(!h.contains(" style="), "{ad}: satır içi stil özniteliği");
        assert!(!h.contains("<script>") && !h.contains("onclick") && !h.contains("onload="), "{ad}: satır içi betik");
        assert!(!h.contains("innerHTML"));
    }
    for d in ["oynatici.js", "oynatici.css", "oynatici-motor.js", "oynatici-cikis.js"] {
        let (kod, _, g) = istek("GET", &format!("/oynatici/dosya/{d}"), &[], "", &kok);
        assert_eq!(kod, 200, "{d}");
        let m = String::from_utf8(g).unwrap();
        assert!(!m.contains("innerHTML") && !m.contains("eval("), "{d}: innerHTML/eval yasak");
        if d.ends_with(".css") {
            for yasak in ["gradient", "blur(", "steps(", "box-shadow", "text-shadow", "cubic-bezier(.3,1.5", "drop-shadow"] { assert!(!m.contains(yasak), "{d}: {yasak}"); }
            assert!(m.contains("prefers-reduced-motion"));
            for t in ["vitrin", "vurus", "cephe", "kadraj", "manset", "eslik"] { assert!(m.contains(&format!("data-tema=\"{t}\"")), "{t} teması"); }
        }
    }
    assert_eq!(istek("GET", "/oynatici/dosya/../Cargo.toml", &[], "", &kok).0, 404);
    assert_eq!(istek("GET", "/oynatici/yok", &[], "", &kok).0, 404);
    // karar Ek 1 E1.2: /oynatici.<bilinmeyen> sabit beyaz liste dışında 404
    for y in ["/oynatici.exe", "/oynatici.html", "/oynatici.json", "/oynatici.js.map"] { assert_eq!(istek("GET", y, &[], "", &kok).0, 404, "{y}"); }
    assert_eq!(istek("GET", "/oynatici.js", &[], "", &kok).0, 200);
    let (kod, _, g) = istek("GET", "/oynatici/proje", &[], "", &kok);
    assert_eq!(kod, 200); assert!(String::from_utf8(g).unwrap().contains("\"katmanlar\""));
    assert_eq!(istek("POST", "/oynatici/komut", &[], "{\"tur\":\"yok\"}", &kok).0, 400);
    assert_eq!(istek("POST", "/oynatici/proje", &[], "{\"temelSurum\":0,\"proje\":{}}", &kok).0, 409);
    // pencere köprüsü test sürecinde yok → 503
    assert_eq!(istek("POST", "/oynatici/cikis", &[], "{\"ac\":true}", &kok).0, 503);
    assert_eq!(istek("GET", "/oynatici/monitorler", &[], "", &kok).0, 503);
    // sahne: boş senaryo 400
    assert_eq!(istek("POST", "/oynatici/sahne", &[], "  ", &kok).0, 400);
    // yedek giriş medya için 501
    assert_eq!(oh::isle("GET", "/oynatici/medya/a.mp4", "", &kok).0, 501);
    assert_eq!(oh::isle("GET", "/oynatici/proje", "", &kok).0, 200);
    assert!(oh::bu_yol_mu("/cikis") && oh::bu_yol_mu("/oynatici") && oh::bu_yol_mu("/oynatici/medya/a.mp4") && oh::bu_yol_mu("/oynatici.js"));
    assert!(!oh::bu_yol_mu("/cikisx") && !oh::bu_yol_mu("/"));
}

#[test]
fn motor_node_testi() {
    let betik = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/oynatici_test.js");
    match std::process::Command::new("node").arg(betik).output() {
        Ok(o) => assert!(o.status.success(), "oynatici_test.js başarısız: {}{}", String::from_utf8_lossy(&o.stdout), String::from_utf8_lossy(&o.stderr)),
        Err(_) => eprintln!("node bulunamadı, motor testi atlandı"),
    }
}

/// Elle tarayıcı denemesi için yerel sunucu (yetkisiz; yalnız 127.0.0.1). Çalıştırma:
/// TAA_OYNATICI_SUNUCU=47963 LED_CIKTI=<klasör> TAA_OYNATICI_DOSYASI=<geçici> cargo test --release --test oynatici -- --ignored --nocapture elle_sunucu
#[test]
#[ignore]
fn elle_sunucu() {
    let Ok(port) = std::env::var("TAA_OYNATICI_SUNUCU") else { return };
    let d = std::net::TcpListener::bind(("127.0.0.1", port.parse::<u16>().unwrap())).unwrap();
    let bitis = Instant::now() + Duration::from_secs(std::env::var("TAA_OYNATICI_SURE").ok().and_then(|s| s.parse().ok()).unwrap_or(900));
    d.set_nonblocking(true).unwrap();
    while Instant::now() < bitis {
        match d.accept() {
            Ok((mut s, _)) => { s.set_nonblocking(false).unwrap(); std::thread::spawn(move || {
                use std::io::Read;
                let _ = s.set_read_timeout(Some(Duration::from_secs(10)));
                let mut t = vec![]; let mut p = [0u8; 8192];
                let son = loop { let Ok(n) = s.read(&mut p) else { return }; if n == 0 { return } t.extend(&p[..n]); if let Some(i) = t.windows(4).position(|w| w == b"\r\n\r\n") { break i } };
                let bas = String::from_utf8_lossy(&t[..son]).into_owned();
                let mut l = bas.lines(); let ilk = l.next().unwrap_or(""); let mut q = ilk.split_whitespace();
                let (y, yol) = (q.next().unwrap_or("").to_string(), q.next().unwrap_or("").to_string());
                let bs: Vec<(String, String)> = l.filter_map(|x| x.split_once(':').map(|(a, b)| (a.trim().to_lowercase(), b.trim().to_string()))).collect();
                let uz: usize = bs.iter().find(|(a, _)| a == "content-length").and_then(|(_, v)| v.parse().ok()).unwrap_or(0);
                let mut g = t[son + 4..].to_vec();
                while g.len() < uz { let Ok(n) = s.read(&mut p) else { return }; if n == 0 { return } g.extend(&p[..n]); }
                g.truncate(uz);
                let yolu = yol.split('?').next().unwrap_or("").to_string();
                if yolu == "/" { let _ = std::io::Write::write_all(&mut s, b"HTTP/1.1 302 Found\r\nLocation: /oynatici\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"); return; }
                if yolu == "/ayar" { let _ = std::io::Write::write_all(&mut s, b"HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: 2\r\nConnection: close\r\n\r\n{}"); return; }
                if y == "POST" && !bs.iter().any(|(a, v)| a == "x-led" && v == "1") { let _ = std::io::Write::write_all(&mut s, b"HTTP/1.1 403 Forbidden\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"); return; }
                oh::isle_akis(&mut s, &y, &yol, &bs, &g, &|| false);
            }); }
            Err(_) => std::thread::sleep(Duration::from_millis(20)),
        }
    }
}
