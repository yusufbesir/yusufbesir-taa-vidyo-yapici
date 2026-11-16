// TAA Vidyo Yapıcı: "Tarif et" altın testleri (çevrimdışı kural motoru). Sahip: Yusuf; yazan [Claude#ebd691, 2026-11-16].
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
