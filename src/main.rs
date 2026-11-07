use ab_glyph::{Font, FontRef, PxScale, };
use taa_vidyocu::*;
use std::io::Write;

fn arg(a: &[String], k: &str, d: &str) -> String {
    a.iter().position(|x| x == k).and_then(|i| a.get(i + 1)).cloned().unwrap_or(d.into())
}

fn baslat(cikis: &str, kodek: &str, w: u32, h: u32, fps: u32) -> std::process::Child {
    ffmpeg::baslat(cikis, kodek, w, h, fps).unwrap_or_else(|e| { eprintln!("HATA: {e}"); std::process::exit(3) })
}

#[cfg(windows)]
fn konsolu_gizle() { extern "system" { fn FreeConsole() -> i32; } unsafe { FreeConsole(); } }
#[cfg(not(windows))]
fn konsolu_gizle() {}

/// Dikey ekran için satırlara böl: her kelime ayrı satır; ≤2 harfli/rakam kelimeler sonrakiyle birleşir.
fn dikey_satirlar(s: &str) -> Vec<String> {
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

/// Bir mesajı dikey ekran için satır maskelerine böler (her satır genişliğe ayrı sığar). Dönüş: maskeler, derinlik.
fn yerlesim(font: &FontRef, w: u32, h: u32, s: &str, en_cok_px: f32) -> (Vec<anim::Maske>, f32) {
    let satir = dikey_satirlar(s);
    let mut boylar: Vec<f32> = satir.iter().map(|x| (w as f32 * 0.84 / genislik(font, x, 100.0).max(1.0) * 100.0).min(en_cok_px)).collect();
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

fn main() {
    let a: Vec<String> = std::env::args().collect();
    if a.len() == 1 || a.iter().any(|x| x == "--arayuz") {
        // Çift tıklayınca arayüz açılır (konsol gizlenir); --port N ve --tarayici-yok test içindir.
        let port: u16 = arg(&a, "--port", "0").parse().unwrap_or(0);
        if !a.iter().any(|x| x == "--tarayici-yok") { konsolu_gizle(); }
        arayuz::calistir(!a.iter().any(|x| x == "--tarayici-yok"), port);
    }
    if a.iter().any(|x| x == "--surum") { println!("taa-vidyocu {}", arayuz::SURUM); return; }
    if a.iter().any(|x| x == "--yardim") {
        println!("taa-vidyocu --yazi \"SATIR1|SATIR2\" [--palet sari|beyaz|turuncu|cyan|magenta|lime] [--mod sabit|kayan]\n  [--genislik 980 --yukseklik 1800 (dikey 9:16) --fps 25 --sure 8 --harf 120 --font yol.otf --cikis cikti.mp4 --kodek mp4|hap]\n  --onizleme dosya.ppm : yalnız tek kare yazar\n  --senaryo dosya.txt [--cikis v.mp4] : senaryodan video\n  --arayuz : tarayıcı arayüzü (argümansız çalıştırınca da açılır)\nffmpeg: LED_FFMPEG, programın yanı veya PATH");
        return;
    }
    let w: u32 = arg(&a, "--genislik", "980").parse().unwrap();
    let h: u32 = arg(&a, "--yukseklik", "1800").parse().unwrap();
    let fps: u32 = arg(&a, "--fps", "25").parse().unwrap();
    let sure: f32 = arg(&a, "--sure", "8").parse().unwrap();
    let harf: f32 = arg(&a, "--harf", "120").parse().unwrap();
    let mod_ = arg(&a, "--mod", "sabit");
    let satirlar: Vec<String> = arg(&a, "--yazi", "TERZİ ADEM ALTUN").split('|').map(|s| s.trim().to_uppercase()).collect();
    let (yazi, zemin) = palet(arg(&a, "--palet", "sari").split(',').next().unwrap_or("sari")).expect("bilinmeyen palet");
    let font_yolu = arg(&a, "--font", "");
    let veri = if font_yolu.is_empty() { include_bytes!("../fonts/InterDisplay-Bold.otf").to_vec() } else { std::fs::read(&font_yolu).expect("font okunamadı") };
    let font = FontRef::try_from_slice(&veri).expect("font geçersiz");

    let en_uzun = satirlar.iter().map(|s| genislik(&font, s, harf)).fold(0.0, f32::max);
    let hiz = if mod_ == "kayan" { (w as f32 + en_uzun) / sure } else { 0.0 };
    for u in uyarilar(yazi, zemin, harf, &satirlar, hiz, w) { eprintln!("UYARI: {u}"); }
    if mod_ == "sabit" && !a.iter().any(|x| x == "--senaryo") && en_uzun > w as f32 * 0.95 { eprintln!("UYARI: Yazı ekrana sığmıyor; harf boyunu küçültün veya satır bölün."); }
    let _ = font.as_scaled(PxScale::from(harf));

    if let Some(i) = a.iter().position(|x| x == "--senaryo") {
        let yol = a.get(i + 1).expect("--senaryo dosya.txt");
        let metin = std::fs::read_to_string(yol).expect("senaryo dosyası okunamadı");
        let mut sen = match senaryo::ayristir(&metin) { Ok(s) => s, Err(e) => { eprintln!("HATA: {e}"); std::process::exit(2); } };
        if a.iter().any(|x| x == "--genislik") { sen.w = w; }
        if a.iter().any(|x| x == "--yukseklik") { sen.h = h; }
        if a.iter().any(|x| x == "--duz") { sen.fiziksel = None; }
        if a.iter().any(|x| x == "--fps") { sen.fps = fps; }
        if let Err(e) = uret::dogrula(&sen) { eprintln!("HATA: {e}"); std::process::exit(2); }
        let hz = senaryo::hazirla(&font, &sen);
        let mut kod = baslat(&arg(&a, "--cikis", "cikti.mp4"), &arg(&a, "--kodek", "mp4"), sen.w, sen.h, sen.fps);
        let mut girdi = kod.stdin.take().unwrap();
        let toplam = senaryo::toplam_sn(&sen);
        let n = (toplam * sen.fps as f32) as u32;
        if let Some(o) = sen.piksel_orani() { eprintln!("Fiziksel düzeltme: piksel oranı {o:.2} (dikey/yatay); sanal tuval {}x{} -> {}x{}", sen.w, sen.sanal_h(), sen.w, sen.h); }
        for k in 0..n {
            let kare = senaryo::kare_uret(&sen, &hz, k as f32 / sen.fps as f32, k);
            girdi.write_all(&senaryo::dikey_kucult(&kare, sen.w, sen.sanal_h(), sen.h)).unwrap();
        }
        drop(girdi);
        println!("Senaryo videosu tamam: {:.1} sn, {} kare", toplam, n);
        let _ = kod.wait();
        return;
    }

    if mod_ == "tv" {
        // TV sıralaması: açılış → mesaj A (yukarı çıkış) → mesaj B (alttan giriş) → kapanış. `--yazi "A|B"`
        let en_cok_px: f32 = arg(&a, "--harf", "330").parse().unwrap();
        let sure_a: f32 = arg(&a, "--sure", "4.5").parse().unwrap();
        let mut kodlayici = baslat(&arg(&a, "--cikis", "cikti.mp4"), &arg(&a, "--kodek", "mp4"), w, h, fps);
        let mut girdi = kodlayici.stdin.take().unwrap();
        let palet_adlari = arg(&a, "--palet", "beyaz,altin");
        let paletler: Vec<(Renk, Renk)> = palet_adlari.split(',').filter_map(palet).collect();
        let sahneler: Vec<(Vec<anim::Maske>, f32, Renk, Renk, Renk)> = satirlar.iter().take(2).enumerate().map(|(n, s)| {
            let (yuz, z) = paletler[n % paletler.len()];
            let yan = Renk(yuz.0 / 2 + 20, yuz.1 / 3, yuz.2 / 3);
            let (mk, d) = yerlesim(&font, w, h, s, en_cok_px);
            let _ = z;
            (mk, d, yuz, yan, Renk(18, 20, 24)) // tüp ekran çok koyu gri: şekli görünsün, çerçeve saf siyah kalsın
        }).collect();
        let sure_b = sure_a;
        let toplam = tv::ACILIS_SN + sure_a + sure_b + tv::KAPANIS_SN;
        let pi = std::f32::consts::PI;
        for i in 0..(toplam * fps as f32) as u32 {
            let t = i as f32 / fps as f32;
            let (sn, yerel) = if t < tv::ACILIS_SN { (0usize, t) } else if t < tv::ACILIS_SN + sure_a { (0, t - tv::ACILIS_SN) } else { (1, t - tv::ACILIS_SN - sure_a) };
            let (mk, d, yuz, yan, z) = &sahneler[sn.min(sahneler.len() - 1)];
            let mut k = anim::bos_kare(w, h, *z);
            let cik = 0.9_f32; // A: yukarı çıkış süresi
            let gir = 0.9_f32; // B: alttan giriş süresi
            for (li, m) in mk.iter().enumerate() {
                let faz = li as f32 * 0.9;
                let sal = |s: f32| anim::Poz { aci: (s * 1.3 + faz).sin() * 0.26, olcek: 1.0 + 0.03 * (s * 3.0 + faz).sin(), parlak: 1.0, derinlik: *d, dx: 0.0, dy: (s * 2.0 + faz).sin() * 6.0 };
                let mut p = sal(t);
                if sn == 0 && yerel > sure_a - cik {
                    let kk = ((yerel - (sure_a - cik)) / cik).clamp(0.0, 1.0);
                    let e = kk * kk * (3.0 - 2.0 * kk);
                    p.dy = -e * h as f32 * 1.05 - kk * li as f32 * 60.0; // satırlar hafif farklı hızla yukarı çıkar
                    p.aci = -e * pi * 0.25;
                } else if sn == 1 && yerel < gir + 0.4 {
                    let kk = ((yerel - li as f32 * 0.15) / gir).clamp(0.0, 1.0);
                    p.dy = (1.0 - anim::geri(kk)) * h as f32 * 1.05;
                    p.aci = (1.0 - kk) * pi * 0.5 + p.aci * kk;
                    if kk <= 0.0 { p.parlak = 0.0; }
                }
                anim::kare3d_uzerine(&mut k, m, *yuz, *yan, &p);
            }
            if sn == 0 || sn == 1 {
                let t0 = if sn == 0 { 1.0 } else { gir + 0.6 };
                let konum = (yerel - t0) / 1.3;
                efekt::isik_supurme(&mut k, w, h, *z, if (0.0..=1.0).contains(&konum) { konum } else { -1.0 });
            }
            let efektli = if t < tv::ACILIS_SN { tv::tup(&k, w, h, tv::acilis(t), i) }
                else if t > toplam - tv::KAPANIS_SN { tv::tup(&k, w, h, tv::kapanis(t - (toplam - tv::KAPANIS_SN)), i) }
                else { tv::tup(&k, w, h, tv::Tup { sv: 1.0, sh: 1.0, flas: 0.0, parlak: 0.985 + 0.015 * (i as f32 * 0.9).sin(), gurultu: 0.0 }, i) };
            girdi.write_all(&efektli).unwrap();
        }
        drop(girdi);
        println!("TV videosu tamam: {:.1} sn", toplam);
        let _ = kodlayici.wait();
        return;
    }

    if mod_ == "3d" {
        let en_cok_px: f32 = arg(&a, "--harf", "330").parse().unwrap();
        let palet_adlari = arg(&a, "--palet", "sari,cyan,turuncu,lime,magenta");
        let paletler: Vec<(Renk, Renk)> = palet_adlari.split(',').filter_map(palet).collect();
        let sure_m: f32 = arg(&a, "--sure", "6").parse().unwrap();
        let mut kodlayici = baslat(&arg(&a, "--cikis", "cikti.mp4"), &arg(&a, "--kodek", "mp4"), w, h, fps);
        let mut girdi = kodlayici.stdin.take().unwrap();
        for (n, s) in satirlar.iter().enumerate() {
            let (yuz, zemin3) = paletler[n % paletler.len()];
            let yan = Renk(yuz.0 / 2 + 20, yuz.1 / 3, yuz.2 / 3);
            let cizim_ad = s.strip_prefix('@').map(|x| x.trim().to_lowercase());
            // satırlar: kelimeleri alt alta diz (dikey ekran); kısa kelimeler yanındakiyle birleşir
            let satir: Vec<String> = if cizim_ad.is_some() { vec![] } else { dikey_satirlar(s) };
            // her satır genişliğe ayrı sığdırılır (dikey ekranda dolgun, uzaktan okunur); toplam yükseklik %80'i aşmaz
            let mut boylar: Vec<f32> = satir.iter().map(|x| (w as f32 * 0.84 / genislik(&font, x, 100.0).max(1.0) * 100.0).min(en_cok_px)).collect();
            let toplam: f32 = boylar.iter().map(|b| satir_yuksekligi(&font, *b)).sum();
            if toplam > h as f32 * 0.8 { let k = h as f32 * 0.8 / toplam; for b in boylar.iter_mut() { *b *= k; } }
            let (maskeler, d): (Vec<anim::Maske>, f32) = match cizim_ad {
                Some(ad) => (vec![cizim::maske(&ad, w, h, w.min(h) as f32 * 0.9).unwrap_or_else(|| panic!("bilinmeyen çizim: {ad} (olanlar: {:?})", cizim::ADLAR))], w as f32 * 0.06),
                None => {
                    let sh: Vec<f32> = boylar.iter().map(|b| satir_yuksekligi(&font, *b)).collect();
                    let mut y = (h as f32 - sh.iter().sum::<f32>()) / 2.0;
                    let mut v = vec![];
                    for (k, s1) in satir.iter().enumerate() {
                        let orta = y + sh[k] / 2.0;
                        v.push(anim::kaydir(&anim::maske(&font, w, h, &[s1.clone()], boylar[k]), (orta - h as f32 / 2.0) as i32));
                        y += sh[k];
                    }
                    let ort = boylar.iter().cloned().fold(0.0, f32::max);
                    (v, ort * 0.14)
                }
            };
            let sure_s = sure_m.max(1.4 + 0.45 * maskeler.len() as f32 + 1.0);
            for i in 0..(sure_s * fps as f32) as u32 {
                let t = i as f32 / fps as f32;
                let mut k = anim::bos_kare(w, h, zemin3);
                for (li, mk) in maskeler.iter().enumerate() {
                    let p = anim::poz(t, sure_s, d, n + li, li as f32 * 0.45, (w, h));
                    anim::kare3d_uzerine(&mut k, mk, yuz, yan, &p);
                }
                efekt::kosan_cerceve(&mut k, w, h, t, yuz, (w / 70).max(6), (w / 25).max(10));
                let t0 = 1.2 + 0.45 * maskeler.len() as f32;
                let konum = [(t - t0) / 1.3, (t - (t0 + sure_s) / 2.0) / 1.3].into_iter().find(|k| (0.0..=1.0).contains(k)).unwrap_or(-1.0);
                efekt::isik_supurme(&mut k, w, h, zemin3, konum);
                efekt::piriltilar(&mut k, w, h, t, 18, Renk(255, 255, 255), n as u32 + 1);
                girdi.write_all(&k).unwrap();
            }
        }
        drop(girdi);
        println!("3B video tamam: {} mesaj", satirlar.len());
        let _ = kodlayici.wait();
        return;
    }

    let onizleme = arg(&a, "--onizleme", "");
    if !onizleme.is_empty() {
        let k = kare(&font, w, h, &satirlar, harf, yazi, zemin, 0.0);
        let mut f = std::fs::File::create(&onizleme).unwrap();
        write!(f, "P6\n{w} {h}\n255\n").unwrap();
        f.write_all(&k).unwrap();
        return;
    }

    let cikis = arg(&a, "--cikis", "cikti.mp4");
    let kodek = arg(&a, "--kodek", "mp4");
    let mut p = baslat(&cikis, &kodek, w, h, fps);
    let mut girdi = p.stdin.take().unwrap();
    let n = (sure * fps as f32) as u32;
    let sabit = kare(&font, w, h, &satirlar, harf, yazi, zemin, 0.0);
    for i in 0..n {
        let t = i as f32 / fps as f32;
        let k = if mod_ == "kayan" { kare(&font, w, h, &satirlar, harf, yazi, zemin, kayma_x(t, sure, w, en_uzun)) } else { sabit.clone() };
        girdi.write_all(&k).unwrap();
    }
    drop(girdi);
    let d = p.wait().unwrap();
    println!("{} ({} kare, {})", cikis, n, if d.success() { "tamam" } else { "ffmpeg hata" });
}
