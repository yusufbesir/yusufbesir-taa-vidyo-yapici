// TAA Vidyo Yapıcı: "Tarif et" altın testleri (çevrimdışı kural motoru). Sahip: Yusuf; yazan [Claude#ebd691, 2026-11-16]; ek [Claude#AI Opus 5.5, yenileme-1.0, 2026-11-18]: gerçek mağaza talepleri (niyet şablonları). Silme yok.
// tests/altin.tsv: her satır "tarif<TAB>beklenen özet". Beklenenleri yeniden üretmek için ALTIN_YAZ=1 ile çalıştırın, çıktıyı gözle inceleyip kabul edin.
use taa_vidyo_yapici::tarif::*;
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
        ("pacman gibi bir animasyon olsun", &["HOŞ"], "sablon"),
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
        // çeşit (Başka öneri) de geçerli olmalı ve farklı stil vermeli
        let h2 = niyet::kural_hazirla(metin, &kat, &[], 1);
        taa_vidyo_yapici::senaryo::ayristir(&senaryo_metni(&h2.tarif)).unwrap();
    }
    // stil isteği uygulanır
    let h = niyet::kural_hazirla("indirim var kırmızı olsun yanıp sönsün", &kat, &[], 0);
    assert!(h.tarif.sahneler.iter().all(|s| s.palet == "hex-ff2b2b" && s.giris == "nabiz"), "{:?}", h.tarif.sahneler);
    let a = niyet::kural_hazirla("takım elbiselerde yüzde 20 indirim", &kat, &[], 0).tarif;
    let b = niyet::kural_hazirla("takım elbiselerde yüzde 20 indirim", &kat, &[], 1).tarif;
    assert_ne!(a, b, "Başka öneri farklı stil vermeli");
}
