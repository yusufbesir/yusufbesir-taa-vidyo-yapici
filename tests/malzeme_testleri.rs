// Açıklama: Malzeme kitaplığı bütünleşik testleri: her malzeme geçerli senaryo üretir/ayrıştırılır, 950x1800'de render edilir
// (boyut doğru, boş/tek renk değil, deterministik), parametre sınırları, eski senaryolar bozulmadı, küçük PNG, web/malzeme.js Node testi, süre ölçümü.
// Çalıştırma: tasarim/yenileme/kilitli-cargo.sh test --release --test malzeme_testleri -- --nocapture
// Sahip: Yusuf. Yazan: [Claude#Malzeme Opus 5.5, yenileme-1.0, 2026-11-18]. Yalnız yazan yapay zekâ ya da Yusuf'un izniyle değişir; silme yok.
use taa_vidyo_yapici::{malzeme, malzeme_http, senaryo, uret};
use std::time::Instant;

fn kare(metin: &str, t: f32) -> (Vec<u8>, u32, u32) {
    let s = uret::senaryo_oku(metin).unwrap_or_else(|e| panic!("{e}\n{metin}"));
    let f = uret::font();
    let hz = senaryo::hazirla(&f, &s);
    (senaryo::kare_uret(&s, &hz, t, (t * 25.0) as u32), s.w, s.sanal_h())
}

#[test]
fn her_malzeme_gecerli_senaryo_uretir() {
    for k in malzeme::KATALOG {
        let p = malzeme::senaryo_parcasi(&malzeme::Istek { id: k.id.into(), ..Default::default() }).unwrap_or_else(|e| panic!("{}: {e}", k.id));
        assert!(p.starts_with(&format!("malzeme {} ", k.id)));
        let s = senaryo::ayristir(&malzeme::onizleme_senaryosu(&p)).unwrap_or_else(|e| panic!("{}: {e}", k.id));
        assert_eq!((s.w, s.h), (950, 1800));
        assert_eq!(s.sahneler[0].malzeme.as_ref().unwrap().id, k.id);
        uret::dogrula(&s).unwrap();
        // eski sahnelerle karışık senaryo
        let karma = format!("ekran 950x1800\nacilis fade\nsahne \"TAA\" sure=3\n{p}\nsahne \"SON\" sure=3 giris=belir\n");
        assert_eq!(senaryo::ayristir(&karma).unwrap().sahneler.len(), 3, "{}", k.id);
    }
}

#[test]
fn her_malzeme_render_edilir_ve_deterministik() {
    let mut olcum: Vec<(f32, &str)> = vec![];
    for k in malzeme::KATALOG {
        let p = malzeme::senaryo_parcasi(&malzeme::Istek { id: k.id.into(), ..Default::default() }).unwrap();
        let metin = malzeme::onizleme_senaryosu(&p);
        let mut en_uzun = 0f32;
        for t in [malzeme_http::temsil_t(k.sure), k.sure * 0.8] {
            let bas = Instant::now();
            let (a, w, h) = kare(&metin, t);
            en_uzun = en_uzun.max(bas.elapsed().as_secs_f32() * 1000.0);
            assert_eq!(a.len(), (w * h * 3) as usize, "{}", k.id);
            assert_eq!((w, h), (950, 1800));
            // boş/tek renk değil: zeminden farklı piksel oranı ve renk çeşitliliği
            let z = (a[0], a[1], a[2]);
            let farkli = a.chunks(3).filter(|p| (p[0], p[1], p[2]) != z).count();
            assert!(farkli as f32 > (w * h) as f32 * 0.004, "{} t={t}: neredeyse boş kare ({farkli} piksel)", k.id);
            let mut renkler: Vec<u32> = a.chunks(3).step_by(7).map(|p| (p[0] as u32) << 16 | (p[1] as u32) << 8 | p[2] as u32).collect();
            renkler.sort_unstable(); renkler.dedup();
            assert!(renkler.len() >= 3, "{} t={t}: tek renk", k.id);
            if !malzeme::canli_mi(k.id) {
                let (b, _, _) = kare(&metin, t);
                assert!(a == b, "{} t={t}: deterministik değil", k.id);
            }
        }
        olcum.push((en_uzun, k.id));
    }
    olcum.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    let ort = olcum.iter().map(|x| x.0).sum::<f32>() / olcum.len() as f32;
    eprintln!("MALZEME RENDER 950x1800 (hazırlık+kare, ms): ortalama {ort:.1}, en yavaş 5: {:?}", &olcum[..5]);
    assert!(olcum[0].0 < 3000.0, "bir kare çok yavaş: {:?}", olcum[0]);
}

#[test]
fn kare_suresi_olcumu() {
    // hazırlık hariç yalnız kare çizimi: yoğun malzemelerde 25 fps üretim için kare başı süre
    for id in ["yutkun-labirent", "yutkun-kovalamaca", "havai-fisek-gosteri", "konfeti-patlama", "kar-yagisi", "gecis-daire", "kayan-led", "rozet-firsat"] {
        let p = malzeme::senaryo_parcasi(&malzeme::Istek { id: id.into(), ..Default::default() }).unwrap();
        let s = uret::senaryo_oku(&malzeme::onizleme_senaryosu(&p)).unwrap();
        let f = uret::font();
        let hz = senaryo::hazirla(&f, &s);
        let bas = Instant::now();
        let n = 10;
        for i in 0..n { let _ = senaryo::kare_uret(&s, &hz, 1.0 + i as f32 * 0.37, i); }
        eprintln!("KARE {id}: {:.1} ms/kare", bas.elapsed().as_secs_f32() * 1000.0 / n as f32);
    }
}

#[test]
fn parametre_sinirlari() {
    for k in malzeme::KATALOG {
        let q = |f: &dyn Fn(&mut malzeme::Istek)| { let mut i = malzeme::Istek { id: k.id.into(), ..Default::default() }; f(&mut i); malzeme::senaryo_parcasi(&i) };
        assert!(q(&|i| i.sure = Some(malzeme::SURE_MIN)).is_ok(), "{}", k.id);
        assert!(q(&|i| i.sure = Some(malzeme::SURE_MAX)).is_ok(), "{}", k.id);
        assert!(q(&|i| i.sure = Some(malzeme::SURE_MIN - 0.5)).is_err(), "{}", k.id);
        assert!(q(&|i| i.sure = Some(malzeme::SURE_MAX + 1.0)).is_err(), "{}", k.id);
        assert!(q(&|i| i.hiz = Some(malzeme::HIZ_MIN)).is_ok() && q(&|i| i.hiz = Some(malzeme::HIZ_MAX)).is_ok(), "{}", k.id);
        assert!(q(&|i| i.hiz = Some(0.1)).is_err() && q(&|i| i.hiz = Some(9.0)).is_err(), "{}", k.id);
        assert!(q(&|i| i.renk = Some("yok-renk".into())).is_err(), "{}", k.id);
        if let Some((_, a, b, _)) = k.deger {
            assert!(q(&|i| i.deger = Some(a)).is_ok() && q(&|i| i.deger = Some(b)).is_ok(), "{}", k.id);
            assert!(q(&|i| i.deger = Some(a - 1.0)).is_err() && q(&|i| i.deger = Some(b + 1.0)).is_err(), "{}", k.id);
            // sınır değerlerde de çizilir
            for d in [a, b] { let p = q(&|i| { i.deger = Some(d); i.sure = Some(4.0); }).unwrap(); let (f, _, _) = kare(&malzeme::onizleme_senaryosu(&p).replace("950x1800", "190x360"), 2.0); assert_eq!(f.len(), 190 * 360 * 3); }
        }
        if k.yon.is_some() { for (y, _) in malzeme::YONLER { assert!(q(&|i| i.yon = Some(y.to_string())).is_ok()); } assert!(q(&|i| i.yon = Some("capraz".into())).is_err()); }
        if k.metin.is_some() {
            let p = q(&|i| i.metin = Some("çok \"tırnak\" # yorum\nsatır ığüşöçİ".into())).unwrap();
            assert!(!p.contains('#') && p.matches('"').count() == 2 && !p.contains('\n'), "{p}");
            let (f, _, _) = kare(&malzeme::onizleme_senaryosu(&p).replace("950x1800", "190x360"), 2.0);
            assert_eq!(f.len(), 190 * 360 * 3);
            assert!(q(&|i| i.metin = Some(String::new())).is_ok());
        }
    }
}

#[test]
fn bozuk_istekler_400() {
    for g in ["", "{", "null", "[1]", r#"{"id":1}"#, r#"{"id":"kayan-led","hiz":"hızlı"}"#, r#"{"id":"kayan-led","fazla":1}"#, r#"{"id":"kayan-led","renk":"a b"}"#, r#"{"id":"kum-saati","metin":"x"}"#, r#"{"id":"geri-sayim","deger":100000}"#] {
        let (k, tur, govde) = malzeme_http::isle("POST", "uret", g);
        assert_eq!(k, 400, "{g} → {govde}");
        assert!(tur.starts_with("application/json") && govde.contains("\"tamam\":false"), "{govde}");
    }
    let buyuk = format!("{{\"id\":\"kayan-led\",\"metin\":\"{}\"}}", "A".repeat(9000));
    assert_eq!(malzeme_http::isle("POST", "uret", &buyuk).0, 400);
    assert_eq!(malzeme_http::isle("GET", "kucuk/yok-boyle", "").0, 404);
}

#[test]
fn kucuk_png_gecerli() {
    let (k, tur, g) = malzeme_http::isle("GET", "kucuk/yutkun-labirent", "");
    assert_eq!(k, 200, "{g}");
    assert!(tur.starts_with("text/plain") && g.starts_with("data:image/png;base64,iVBORw0KGgo"));
    assert!(g.len() < 160_000, "küçük resim çok büyük: {}", g.len());
    let (k, _, g) = malzeme_http::isle("POST", "kucuk", r#"{"id":"rozet-yuzde","metin":"%50","t":1.5}"#);
    assert_eq!(k, 200, "{g}");
}

#[test]
fn eski_senaryolar_bozulmadi() {
    let kl = concat!(env!("CARGO_MANIFEST_DIR"), "/senaryolar");
    let mut n = 0;
    for e in std::fs::read_dir(kl).unwrap() {
        let p = e.unwrap().path();
        if p.extension().and_then(|x| x.to_str()) != Some("txt") { continue; }
        let m = std::fs::read_to_string(&p).unwrap();
        let m = if m.contains("# varsayilan:") { taa_vidyo_yapici::sablon::doldur(&m, &[]) } else { m };
        let s = senaryo::ayristir(&m).unwrap_or_else(|e| panic!("{}: {e}", p.display()));
        assert!(s.sahneler.iter().all(|x| x.malzeme.is_none() || p.file_name().unwrap().to_string_lossy().starts_with("malzeme")));
        n += 1;
    }
    assert!(n >= 10, "senaryo sayısı {n}");
}

#[test]
fn malzeme_js_node_testi() {
    let betik = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/malzeme_test.js");
    match std::process::Command::new("node").arg(betik).output() {
        Ok(o) => assert!(o.status.success(), "malzeme_test.js başarısız: {}{}", String::from_utf8_lossy(&o.stdout), String::from_utf8_lossy(&o.stderr)),
        Err(_) => eprintln!("node bulunamadı, malzeme_test.js atlandı"),
    }
}

/// Görsel denetim: tüm malzemelerin 190x360 karelerini tek bir ızgara PNG'ye yazar. Çalıştırma:
/// MALZEME_CIKTI=klasör tasarim/yenileme/kilitli-cargo.sh test --release --test malzeme_testleri -- --ignored kontak
#[test]
#[ignore]
fn kontak_sayfasi() {
    let Ok(kl) = std::env::var("MALZEME_CIKTI") else { return };
    let (kw, kh, sut) = (190u32, 360u32, 10u32);
    let n = malzeme::KATALOG.len() as u32;
    let (w, h) = (kw * sut, kh * n.div_ceil(sut));
    let mut tuval = vec![40u8; (w * h * 3) as usize];
    for (i, k) in malzeme::KATALOG.iter().enumerate() {
        let p = malzeme::senaryo_parcasi(&malzeme::Istek { id: k.id.into(), ..Default::default() }).unwrap();
        let (f, _, _) = kare(&malzeme::onizleme_senaryosu(&p).replace("950x1800", "190x360"), malzeme_http::temsil_t(k.sure));
        let (ox, oy) = ((i as u32 % sut) * kw, (i as u32 / sut) * kh);
        for y in 0..kh { let s = (y * kw * 3) as usize; let d = (((oy + y) * w + ox) * 3) as usize; tuval[d..d + (kw * 3) as usize].copy_from_slice(&f[s..s + (kw * 3) as usize]); }
    }
    std::fs::write(format!("{kl}/kontak.png"), malzeme_http::png_sikistir(&tuval, w, h)).unwrap();
    for (id, t) in [("yutkun-labirent", 4.0f32), ("yutkun-kovalamaca", 3.0), ("rozet-taksit", 2.0), ("kayan-led", 3.0), ("geri-sayim", 3.3), ("havai-fisek-gosteri", 2.2)] {
        let p = malzeme::senaryo_parcasi(&malzeme::Istek { id: id.into(), ..Default::default() }).unwrap();
        let (f, fw, fh) = kare(&malzeme::onizleme_senaryosu(&p), t);
        std::fs::write(format!("{kl}/{id}.png"), malzeme_http::png_sikistir(&f, fw, fh)).unwrap();
    }
}
