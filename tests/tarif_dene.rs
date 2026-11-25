// TAA Vidyo Yapıcı: "Tarif et" altın testleri (çevrimdışı kural motoru). Sahip: Yusuf; yazan [Claude#ebd691, 2026-11-16]; ek [Claude#AI Opus 5.5, yenileme-1.0, 2026-11-18]: gerçek mağaza talepleri (niyet şablonları). Silme yok.
// tests/altin.tsv: her satır "tarif<TAB>beklenen özet". Beklenenleri yeniden üretmek için ALTIN_YAZ=1 ile çalıştırın, çıktıyı gözle inceleyip kabul edin.
// Ek [Claude#AI Opus 5.5, yenileme-1.0, 2026-11-25]: /denetle uyarısız koşulu, malzeme sahnesi talepleri, işletme adı ayarı, istem ölçüm araçları (elle: --ignored).
use taa_vidyo_yapici::tarif::*;
/// /denetle (uret::analiz) "uyari" düzeyindeki iletiler.
fn uyarilar(t: &Tarif) -> Vec<String> {
    taa_vidyo_yapici::uret::analiz(&senaryo_metni(t)).unwrap_or_else(|e| vec![("uyari", format!("analiz edilemedi: {e}"))]).into_iter().filter(|x| x.0 == "uyari").map(|x| x.1).collect()
}
fn ozet(t: &Tarif) -> String {
    let s: Vec<String> = t.sahneler.iter().map(|s| format!("[{}|g={}|c={}|e={}|p={}|f={}|{:.1}]", s.metin.join("/"), s.giris, s.cikis, s.efekt.join("+"), s.palet, s.font, s.sure)).collect();
    format!("{} ac={} kap={} gor={}", s.join(" "), t.acilis, t.kapanis, t.gorunum)
}
#[test]
fn altin_ornekler() {
    let kat = Katalog::gercek();
    let ornek = std::fs::read_to_string("tests/ornekler.txt").unwrap();
    let girdiler: Vec<&str> = ornek.lines().filter(|l| !l.trim().is_empty()).collect();
    assert!(girdiler.len() >= 40, "en az 40 altın örnek olmalı");
    let uretilen: Vec<String> = girdiler.iter().map(|l| format!("{l}\t{}", kural_coz(l, &kat, &[]).tarif.as_ref().map(ozet).unwrap_or_else(|| "YOK".into()))).collect();
    if std::env::var("ALTIN_YAZ").is_ok() { std::fs::write("tests/altin.tsv", uretilen.join("\n") + "\n").unwrap(); return; }
    let beklenen = std::fs::read_to_string("tests/altin.tsv").expect("tests/altin.tsv yok (ALTIN_YAZ=1 ile üretin)");
    let b: Vec<&str> = beklenen.lines().collect();
    assert_eq!(b.len(), uretilen.len(), "örnek sayısı değişti");
    for (i, (x, y)) in b.iter().zip(&uretilen).enumerate() { assert_eq!(x, y, "altın örnek #{i} değişti"); }
}
#[test]
fn altin_ornekler_senaryoya_donusur() {
    let kat = Katalog::gercek();
    let ornek = std::fs::read_to_string("tests/ornekler.txt").unwrap();
    for l in ornek.lines().filter(|l| !l.trim().is_empty()) {
        let t = kural_coz(l, &kat, &[]).tarif.unwrap();
        let sn = taa_vidyo_yapici::senaryo::ayristir(&senaryo_metni(&t)).unwrap_or_else(|e| panic!("{l} → {e}"));
        assert_eq!(sn.sahneler.len(), t.sahneler.len());
        assert!(toplam_sure(&t) <= MAX_TOPLAM_SN);
    }
}
#[test]
#[ignore]
fn tani_yazdir() {
    let kat = Katalog::gercek();
    if false { println!("KATALOG giris={:?}\ncikis={:?}\nefekt={:?}\npalet={:?}\nfont={:?}", kat.giris, kat.cikis, kat.efekt, kat.palet, kat.font); }
    let ornek = [
        "takım elbiselerde yüzde 20 indirim var",
        "bayram için kutlama mesajı yaz",
        "Kurban bayramınız mübarek olsun",
        "yeni mağazamız açıldı herkesi bekleriz",
        "kredi kartına 6 taksit",
        "hafta içi 9-19 cumartesi 10-18 açığız",
        "önce hoş geldiniz sonra indirim yazsın",
        "indirim var kırmızı olsun yanıp sönsün",
        "pacman gibi bir animasyon olsun",
        "zıplıyarak gelsin hoşgeldiniz yazsın",
        "indrim yap yüzde otuz",
        "güzel bir şey yap",
        "",
        "   ",
        "yılbaşı kampanyası",
        "anneler günü",
        "damatlık takımlarda büyük indirim, sonra 3 taksit, en son mağazamıza bekleriz",
        "dükkan bugün kapalı",
        "Sevgililer gününe özel %15",
        "okullar açılıyor okul kıyafetlerinde indirim",
        "terzi adem altun hoş geldiniz altın renk",
        "çalışma saatlerimiz 09:00 - 20:00",
        "Kumaş çeşitlerimiz geldi",
        "ütü ve tadilat hizmeti",
        "yazı kayarak geçsin soldan sağa",
        "çok renkli olsun dikkat çeksin",
        "Gömlek 499 TL",
        "mavi renkli yazı ile yeni sezon",
        "Ramazan bayramı kutlu olsun, kapanışta perde",
        "ekranda saat ve tarih gözüksün",
        "bir sürü yazı ".repeat(80).as_str().to_string().leak(),
    ];
    for o in ornek {
        let h = niyet::kural_hazirla(o, &kat, &[], 0);
        println!("  HAZIRLA[{}] {} || vars={:?} anl={:?} not={:?}", h.kaynak, h.tarif.sahneler.iter().map(|s| format!("[{}|g={}|e={}|p={}|{:.1}]", s.metin.join("/"), s.giris, s.efekt.join("+"), s.palet, s.sure)).collect::<Vec<_>>().join(" "), h.varsayimlar, h.anlasilmayan, h.notlar);
        let z = kural_coz(o, &kat, &[]);
        let s = match &z.tarif { Some(t) => t.sahneler.iter().map(|s| format!("[{}|g={}|c={}|e={}|p={}|{:.1}]", s.metin.join("/"), s.giris, s.cikis, s.efekt.join("+"), s.palet, s.sure)).collect::<Vec<_>>().join(" "), None => format!("YOK {:?}", z.notlar) };
        println!("«{}» => {} || anlasilmayan={:?}", o.chars().take(60).collect::<String>(), s, z.anlasilmayan);
    }
}

/// Gerçek mağaza talepleri (teknik olmayan kullanıcı dili): her biri geçerli, okunur ve beklenen metni içeren senaryoya dönüşmeli.
#[test]
fn gercek_magaza_talepleri() {
    let kat = Katalog::gercek();
    let uzun = "bir sürü yazı ".repeat(80);
    let ornek: Vec<(&str, &[&str], &str)> = vec![
        ("takım elbiselerde yüzde 20 indirim var", &["%20", "İNDİRİM", "ELBİSELERDE"], "kural"),
        ("bayram için kutlama mesajı yaz", &["BAYRAMINIZ", "KUTLU"], "kural"),
        ("Kurban bayramınız mübarek olsun", &["KURBAN", "MÜBAREK"], "kural"),
        ("yeni mağazamız açıldı herkesi bekleriz", &["AÇILDI", "BEKLERİZ"], "kural"),
        ("kredi kartına 6 taksit", &["6", "TAKSİT", "KREDİ"], "kural"),
        ("hafta içi 9-19 cumartesi 10-18 açığız", &["09:00-19:00", "CUMARTESİ", "10:00-18:00"], "kural"),
        ("önce hoş geldiniz sonra indirim yazsın", &["HOŞ", "İNDİRİM"], "kural"),
        ("indirim var kırmızı olsun yanıp sönsün", &["İNDİRİM"], "kural"),
        ("pacman gibi bir animasyon olsun", &["HOŞ"], "kural"),
        ("zıplıyarak gelsin hoşgeldiniz yazsın", &["HOŞ"], "kural"),
        ("indrim yap yüzde otuz", &["%30", "İNDİRİM"], "kural"),
        ("güzel bir şey yap", &["HOŞ"], "sablon"),
        ("", &["HOŞ"], "sablon"),
        ("   ", &["HOŞ"], "sablon"),
        ("yılbaşı kampanyası", &["YILBAŞI"], "kural"),
        ("anneler günü", &["ANNELER"], "kural"),
        ("damatlık takımlarda büyük indirim, sonra 3 taksit, en son mağazamıza bekleriz", &["İNDİRİM", "3", "BEKLERİZ"], "kural"),
        ("dükkan bugün kapalı", &["KAPALIYIZ"], "kural"),
        ("Sevgililer gününe özel %15", &["SEVGİLİLER", "%15"], "kural"),
        ("okullar açılıyor okul kıyafetlerinde indirim", &["OKULLAR", "İNDİRİM"], "kural"),
        ("terzi adem altun hoş geldiniz altın renk", &["ADEM ALTUN", "HOŞ"], "kural"),
        ("çalışma saatlerimiz 09:00 - 20:00", &["09:00-20:00"], "kural"),
        ("Kumaş çeşitlerimiz geldi", &["GELDİ"], "kural"),
        ("ütü ve tadilat hizmeti", &["TADİLAT", "ÜTÜ"], "kural"),
        ("Gömlek 499 TL", &["GÖMLEK", "499 TL"], "kural"),
        ("mavi renkli yazı ile yeni sezon", &["YENİ", "SEZON"], "kural"),
        ("Ramazan bayramı kutlu olsun, kapanışta perde", &["RAMAZAN", "KUTLU"], "kural"),
        ("ekranda saat ve tarih gözüksün", &["HOŞ"], "sablon"),
        ("\"HOŞ GELDİNİZ\" zıplasın, sonra \"%30 İNDİRİM\" kırmızı", &["HOŞ", "%30"], "kural"),
        ("ÖĞLE ARASI KAPALIYIZ", &["KAPALIYIZ"], "kural"),
        (uzun.as_str(), &["HOŞ"], "sablon"),
    ];
    assert!(ornek.len() >= 25);
    for (metin, beklenen, kaynak) in &ornek {
        let h = niyet::kural_hazirla(metin, &kat, &[], 0);
        let tum: String = h.tarif.sahneler.iter().map(|s| s.metin.join(" ")).collect::<Vec<_>>().join(" | ");
        for b in beklenen.iter() { assert!(tum.contains(b), "«{metin}» → beklenen {b:?} yok: {tum}"); }
        assert_eq!(&h.kaynak, kaynak, "«{metin}» kaynak: {tum}");
        assert!(!h.tarif.sahneler.is_empty() && h.tarif.sahneler.len() <= MAX_SAHNE);
        assert!(toplam_sure(&h.tarif) <= MAX_TOPLAM_SN);
        for s in &h.tarif.sahneler { assert!(s.metin.iter().all(|m| m.chars().count() <= MAX_KARAKTER)); assert!(s.metin.len() <= 3, "«{metin}» sahne başına en çok 3 satır: {:?}", s.metin); }
        taa_vidyo_yapici::senaryo::ayristir(&senaryo_metni(&h.tarif)).unwrap_or_else(|e| panic!("«{metin}» senaryo okunamadı: {e}"));
        // 2026-11-25 [Claude#AI Opus 5.5]: Fable bulgusu — /denetle okunurluk uyarısı (harf ≥%5, süre ≥5 sn, kontrast, sert hareket) 0 olmalı
        let u = uyarilar(&h.tarif);
        assert!(u.is_empty(), "«{metin}» /denetle uyarısı: {u:?}\n{}", senaryo_metni(&h.tarif));
        // çeşit (Başka öneri) de geçerli olmalı, farklı stil vermeli ve yine uyarısız olmalı
        let h2 = niyet::kural_hazirla(metin, &kat, &[], 1);
        taa_vidyo_yapici::senaryo::ayristir(&senaryo_metni(&h2.tarif)).unwrap();
        let u2 = uyarilar(&h2.tarif);
        assert!(u2.is_empty(), "«{metin}» (çeşit 1) /denetle uyarısı: {u2:?}");
    }
    // stil isteği uygulanır
    let h = niyet::kural_hazirla("indirim var kırmızı olsun yanıp sönsün", &kat, &[], 0);
    // 2026-11-25: istenen kırmızı (hex-ff2b2b) /denetle kontrastını (7:1) geçmediği için en yakın okunur kırmızı palete çekilir
    assert!(h.tarif.sahneler.iter().all(|s| { let p = taa_vidyo_yapici::stil::paletli(&s.palet).unwrap(); s.giris == "nabiz" && p.yuz.0 > 150 && p.yuz.0 > p.yuz.1 && taa_vidyo_yapici::kontrast(p.yuz, p.zemin) >= 7.0 }), "{:?}", h.tarif.sahneler);
    let a = niyet::kural_hazirla("takım elbiselerde yüzde 20 indirim", &kat, &[], 0).tarif;
    let b = niyet::kural_hazirla("takım elbiselerde yüzde 20 indirim", &kat, &[], 1).tarif;
    assert_ne!(a, b, "Başka öneri farklı stil vermeli");
}

/// Görsel istekleri (oyun, havai fişek, kar, geri sayım, kalp, ok, rozet, ekolayzer, çam, yılbaşı) malzeme sahnesi üretir;
/// kimlik katalogdan (Katalog::gercek().malzeme) gelir ve satır Malzeme modülünün kendi doğrulamasından (senaryo_parcasi/satir_coz) geçer.
#[test]
fn malzeme_talepleri() {
    let kat = Katalog::gercek();
    let ornek = ["pacman gibi bir animasyon olsun", "oyun olsun hoş geldiniz", "labirent kovalamaca", "yılbaşı kampanyası", "bayramda havai fişek patlasın",
        "konfeti ile tebrikler", "kar yağsın kış indirimi", "geri sayım yap", "kalp olsun sevgililer günü", "ok işareti ile giriş", "rozet ile %30 indirim",
        "ekolayzer gibi müzikli", "çam ağacı olsun yeni yıl"];
    for m in ornek {
        let h = niyet::kural_hazirla(m, &kat, &[], 0);
        let ms: Vec<&TSahne> = h.tarif.sahneler.iter().filter(|s| !s.malzeme.is_empty()).collect();
        assert!(!ms.is_empty(), "«{m}» malzeme sahnesi yok: {:?}", h.tarif.sahneler);
        for s in &ms {
            assert!(kat.malzeme.contains(&s.malzeme), "«{m}» bilinmeyen malzeme {}", s.malzeme);
            let p = malzeme_parcasi(s).unwrap_or_else(|| panic!("«{m}» malzeme satırı üretilemedi: {}", s.malzeme));
            taa_vidyo_yapici::malzeme::satir_coz(&p["malzeme ".len()..]).unwrap_or_else(|e| panic!("«{m}» {p}: {e}"));
        }
        let sm = senaryo_metni(&h.tarif);
        assert!(sm.contains("\nmalzeme "), "«{m}» senaryoda malzeme satırı yok:\n{sm}");
        let sn = taa_vidyo_yapici::senaryo::ayristir(&sm).unwrap_or_else(|e| panic!("«{m}» {e}\n{sm}"));
        assert!(sn.sahneler.iter().any(|x| x.malzeme.is_some()));
        let u = uyarilar(&h.tarif);
        assert!(u.is_empty(), "«{m}» /denetle uyarısı: {u:?}\n{sm}");
    }
    // görsel istenmeyen talepte malzeme eklenmez
    assert!(niyet::kural_hazirla("takım elbiselerde yüzde 20 indirim", &kat, &[], 0).tarif.sahneler.iter().all(|s| s.malzeme.is_empty()));
    // yapay zekâ yanıtındaki malzeme alanı doğrulanır; katalogda olmayan ad en yakına eşlenir ya da atılır
    let d = dogrula("{\"sahneler\":[{\"metin\":[\"YILBAŞI\"],\"malzeme\":\"havai-fisek-gosteri\",\"sure\":8},{\"metin\":[],\"malzeme\":\"kar-yagisi\",\"sure\":8}]}", &kat).unwrap();
    assert_eq!(d.tarif.sahneler[0].malzeme, "havai-fisek-gosteri");
    assert_eq!(d.tarif.sahneler[1].malzeme, "kar-yagisi");
    let d = dogrula("{\"sahneler\":[{\"metin\":[\"A\"],\"malzeme\":\"uydurma-malzeme-xyz\",\"sure\":6}]}", &kat).unwrap();
    assert!(d.tarif.sahneler[0].malzeme.is_empty() && d.notlar.iter().any(|n| n.contains("malzeme")));
    assert!(dogrula("{\"sahneler\":[{\"metin\":[],\"sure\":6}]}", &kat).is_err(), "malzemesiz boş metin hata olmalı");
}

/// İşletme adı ayardan gelir ('taa-isletme'); boş/kapalı ise imza sahnesi eklenmez, istemde ad geçmez.
#[test]
fn isletme_adi_ayari() {
    let kat = Katalog::gercek();
    let tum = |h: &niyet::Hazirlik| h.tarif.sahneler.iter().map(|s| s.metin.join(" ")).collect::<Vec<_>>().join(" | ");
    let v = niyet::kural_hazirla("takım elbiselerde yüzde 20 indirim", &kat, &[], 0);
    assert!(tum(&v).contains("ADEM ALTUN"), "varsayılan ad: {}", tum(&v));
    let o = niyet::isletme_ile(Some("Moda Dikiş Evi"), || niyet::kural_hazirla("takım elbiselerde yüzde 20 indirim", &kat, &[], 0));
    assert!(tum(&o).contains("MODA") && tum(&o).contains("DİKİŞ EVİ") && !tum(&o).contains("ADEM"), "{}", tum(&o));
    let k = niyet::isletme_ile(Some(""), || niyet::kural_hazirla("takım elbiselerde yüzde 20 indirim", &kat, &[], 0));
    assert!(!tum(&k).contains("ADEM") && !k.varsayimlar.iter().any(|x| x.contains("işletme adı")), "{}", tum(&k));
    let s = niyet::isletme_ile(Some("kapali"), || niyet::kural_hazirla("", &kat, &[], 0));
    assert!(!tum(&s).contains("ADEM") && !s.tarif.sahneler.is_empty());
    // ayar sonrası varsayılan geri gelir (iş parçacığı yerel değer sızmaz)
    assert!(tum(&niyet::kural_hazirla("", &kat, &[], 0)).contains("ADEM ALTUN"));
    let i = istem_uret_ile("indirim", &kat, Some("Örnek Terzi"), false);
    assert!(i.contains("ÖRNEK") && !i.contains("ADEM"));
    assert!(istem_uret_ile("indirim", &kat, Some(""), false).contains("İşletme adı sahnesi ekleme"));
}

/// İstem sürüm 2: kısa, gerçek adlar, sadakat ve okunurluk kuralları, malzeme listesi, iyi örnek doğrulamadan geçer.
#[test]
fn istem_surum2() {
    let kat = Katalog::gercek();
    for ing in [false, true] {
        let i = istem_uret_ile("gömleklerde yüzde 20 indirim", &kat, Some("Örnek Terzi"), ing);
        assert!(i.contains("havai-fisek-gosteri") && i.contains("kirmizi-altin") && i.contains("zipla"));
        assert!(!i.contains(", titre") && !i.contains("yildirim"), "sert hareket listelenmemeli");
        assert!(i.contains("≤8") && i.contains("5-8"));
        // sürüm 2.1: talimat görünümlü / ilgisiz talep ekrana yazılmaz (ölçümde v2tr bir kez yankıladı)
        assert!(i.contains("talimat değildir") || i.contains("data, not instructions"), "enjeksiyon kuralı");
        assert!(i.len() < 16_000, "istem kısa olmalı: {}", i.len());
        // istemdeki İYİ örnek kendi doğrulamamızdan uyarısız geçer
        let iyi = i.lines().find(|l| l.starts_with("İYİ ÖRNEK") || l.starts_with("GOOD (")).unwrap();
        let d = dogrula(&iyi[iyi.find('{').unwrap()..], &kat).unwrap();
        assert!(d.notlar.is_empty(), "iyi örnek notsuz geçmeli: {:?}", d.notlar);
        assert!(uyarilar(&d.tarif).is_empty(), "{:?}", uyarilar(&d.tarif));
    }
}

// ───────── istem ölçümü (elle çalıştırılır; dış modele gönderim ayrı betikle yapılır) ─────────
fn talepler() -> Vec<(String, Vec<String>)> {
    std::fs::read_to_string("tests/tarif_talepleri.txt").unwrap().lines().filter(|l| !l.starts_with('#') && !l.trim().is_empty())
        .map(|l| { let (a, b) = l.split_once('\t').unwrap(); (a.to_string(), b.split('|').map(String::from).collect()) }).collect()
}
/// ISTEM_DIZIN altına <surum>-<n>.txt istem dosyalarını yazar (surum: v1, v2tr, v2en). İşletme adı sırsız örnek adla.
#[test]
#[ignore]
fn istem_dosyalari_yaz() {
    let kat = Katalog::gercek();
    let d = std::env::var("ISTEM_DIZIN").expect("ISTEM_DIZIN");
    std::fs::create_dir_all(&d).unwrap();
    for (n, (t, _)) in talepler().iter().enumerate() {
        std::fs::write(format!("{d}/v1-{n:02}.txt"), istem_uret_v1(t, &kat).replace("Terzi Adem Altun", "Örnek Terzi").replace("\"TERZİ\" / \"ADEM ALTUN\"", "\"ÖRNEK\" / \"TERZİ\"")).unwrap();
        std::fs::write(format!("{d}/v2tr-{n:02}.txt"), istem_uret_ile(t, &kat, Some("Örnek Terzi"), false)).unwrap();
        std::fs::write(format!("{d}/v2en-{n:02}.txt"), istem_uret_ile(t, &kat, Some("Örnek Terzi"), true)).unwrap();
    }
}
/// YANIT_DIZIN altındaki <surum>-<model>-<n>.txt yanıtlarını sınıflandırır ve özet tablo basar.
#[test]
#[ignore]
fn istem_yanitlarini_degerlendir() {
    let kat = Katalog::gercek();
    let d = std::env::var("YANIT_DIZIN").expect("YANIT_DIZIN");
    let tl = talepler();
    let mut gruplar: std::collections::BTreeMap<String, Vec<[bool; 7]>> = Default::default();
    let mut ayrinti = String::new();
    for e in std::fs::read_dir(&d).unwrap().flatten() {
        let ad = e.file_name().to_string_lossy().to_string();
        let Some(kok) = ad.strip_suffix(".txt") else { continue };
        let p: Vec<&str> = kok.rsplitn(2, '-').collect();
        if p.len() != 2 { continue; }
        let (Ok(n), grup) = (p[0].parse::<usize>(), p[1].to_string()) else { continue };
        let Some((talep, bek)) = tl.get(n) else { continue };
        let yanit = std::fs::read_to_string(e.path()).unwrap_or_default();
        if yanit.trim().is_empty() { continue; }
        // [şema geçerli, /denetle uyarısız (ham), /denetle uyarısız (program hattı), alakalı, uydurma yok, satır kısa, ad doğru]
        let mut r = [false; 7];
        match dogrula_ile(&yanit, &kat, false) {
            Err(h) => { ayrinti += &format!("{grup} #{n}: ŞEMA {h:?}\n"); }
            Ok(dh) => {
                r[0] = true;
                let tum: String = dh.tarif.sahneler.iter().map(|s| s.metin.join(" ")).collect::<Vec<_>>().join(" ");
                r[1] = uyarilar(&dh.tarif).is_empty();
                r[2] = dogrula(&yanit, &kat).map(|dy| uyarilar(&dy.tarif).is_empty()).unwrap_or(false);
                // alakalı: beklenen sözcüklerden biri ekranda VE "!" ile işaretli yasak sözcüklerin hiçbiri yok (ör. talimat enjeksiyonu yankısı)
                r[3] = bek.iter().filter(|b| !b.starts_with('!')).any(|b| tum.contains(b.as_str())) && !bek.iter().filter_map(|b| b.strip_prefix('!')).any(|b| tum.contains(b));
                let tal_say: Vec<String> = talep.split(|c: char| !c.is_ascii_digit()).filter(|x| !x.is_empty()).map(|x| x.trim_start_matches('0').to_string()).collect();
                let uyd: Vec<String> = tum.split(|c: char| !c.is_ascii_digit()).filter(|x| !x.is_empty()).filter(|x| !tal_say.contains(&x.trim_start_matches('0').to_string()) && x.trim_start_matches('0') != "").map(String::from).collect();
                r[4] = uyd.is_empty();
                r[5] = dh.tarif.sahneler.iter().all(|s| s.metin.iter().all(|m| m.split_whitespace().all(|w| w.chars().count() <= 10) && m.chars().count() <= 16));
                r[6] = !dh.notlar.iter().any(|x| x.contains("katalogda yok"));
                if !r.iter().all(|x| *x) { ayrinti += &format!("{grup} #{n} «{}» {r:?} uydurma={uyd:?} metin={tum} adnot={:?}\n", talep.chars().take(40).collect::<String>(), dh.notlar.iter().filter(|x| x.contains("katalogda")).collect::<Vec<_>>()); }
            }
        }
        gruplar.entry(grup).or_default().push(r);
    }
    println!("grup | n | şema | /denetle uyarısız (ham) | /denetle uyarısız (program hattı) | alakalı | uydurma yok | satır kısa | ad doğru");
    for (g, v) in &gruplar {
        let y = |i: usize| format!("%{:.0}", 100.0 * v.iter().filter(|r| r[i]).count() as f32 / v.len().max(1) as f32);
        println!("{g} | {} | {} | {} | {} | {} | {} | {} | {}", v.len(), y(0), y(1), y(2), y(3), y(4), y(5), y(6));
    }
    println!("\nAYRINTI\n{ayrinti}");
}
