//! Hazır şablonlar ve rehber senaryoları (programa gömülü metin dosyaları).
//! Şablon: `# baslik:`, `# kategori:`, `# varsayilan:` (yazı satırları `|` ile) başlıkları + `{1}`, `{2}`... yer tutuculu senaryo.
//! Rehber: `# baslik:`, `# aciklama:` başlıkları + normal senaryo (canlı klip olarak oynatılır).

pub const SABLONLAR: &[(&str, &str)] = &[
    ("acilis-davet", include_str!("../sablonlar/acilis-davet.txt")),
    ("anneler-gunu", include_str!("../sablonlar/anneler-gunu.txt")),
    ("bayram", include_str!("../sablonlar/bayram.txt")),
    ("buyuk-indirim", include_str!("../sablonlar/buyuk-indirim.txt")),
    ("calisma-saati", include_str!("../sablonlar/calisma-saati.txt")),
    ("cuma-kampanyasi", include_str!("../sablonlar/cuma-kampanyasi.txt")),
    ("damatlik", include_str!("../sablonlar/damatlik.txt")),
    ("dikim", include_str!("../sablonlar/dikim.txt")),
    ("hos-geldiniz", include_str!("../sablonlar/hos-geldiniz.txt")),
    ("kapali", include_str!("../sablonlar/kapali.txt")),
    ("pesin-taksit", include_str!("../sablonlar/pesin-taksit.txt")),
    ("randevu", include_str!("../sablonlar/randevu.txt")),
    ("sevgililer", include_str!("../sablonlar/sevgililer.txt")),
    ("sezon-sonu", include_str!("../sablonlar/sezon-sonu.txt")),
    ("son-gun", include_str!("../sablonlar/son-gun.txt")),
    ("tadilat", include_str!("../sablonlar/tadilat.txt")),
    ("yeni-sezon", include_str!("../sablonlar/yeni-sezon.txt")),
    ("yilbasi", include_str!("../sablonlar/yilbasi.txt")),
];

pub const REHBER: &[(&str, &str)] = &[
    ("ayrintilar", include_str!("../senaryolar/rehber/ayrintilar.txt")),
    ("ekranda", include_str!("../senaryolar/rehber/ekranda.txt")),
    ("led-aktar", include_str!("../senaryolar/rehber/led-aktar.txt")),
    ("okunabilirlik", include_str!("../senaryolar/rehber/okunabilirlik.txt")),
    ("sablon", include_str!("../senaryolar/rehber/sablon.txt")),
    ("sasirt-beni", include_str!("../senaryolar/rehber/sasirt-beni.txt")),
    ("uc-adim", include_str!("../senaryolar/rehber/uc-adim.txt")),
];

/// `# anahtar: değer` başlığını okur.
pub fn baslik(icerik: &str, anahtar: &str) -> String {
    let on = format!("# {anahtar}:");
    icerik.lines().find_map(|l| l.strip_prefix(on.as_str())).map(|v| v.trim().to_string()).unwrap_or_default()
}

/// Şablondaki yer tutucuları verilen satırlarla doldurur; eksik satır varsayılanla tamamlanır.
pub fn doldur(icerik: &str, satirlar: &[String]) -> String {
    let var: Vec<String> = baslik(icerik, "varsayilan").split('|').map(|x| x.to_string()).collect();
    let mut o = icerik.to_string();
    for (i, v) in var.iter().enumerate() {
        let s = satirlar.get(i).map(|x| x.trim()).filter(|x| !x.is_empty()).unwrap_or(v.as_str());
        o = o.replace(&format!("{{{}}}", i + 1), &s.replace('"', ""));
    }
    o
}

pub fn sablon_bul(ad: &str) -> Option<&'static str> { SABLONLAR.iter().find(|s| s.0 == ad).map(|s| s.1) }
pub fn rehber_bul(ad: &str) -> Option<&'static str> { REHBER.iter().find(|s| s.0 == ad).map(|s| s.1) }

#[cfg(test)]
mod t {
    use super::*;
    #[test]
    fn sablonlar_gecerli() {
        for (ad, m) in SABLONLAR {
            assert!(!baslik(m, "baslik").is_empty() && !baslik(m, "kategori").is_empty(), "{ad}: başlık eksik");
            let dolu = doldur(m, &[]);
            assert!(!dolu.contains('{'), "{ad}: yer tutucu kaldı");
            crate::senaryo::ayristir(&dolu).unwrap_or_else(|e| panic!("{ad}: {e}"));
            crate::uret::onizleme(&dolu, 1.5).unwrap_or_else(|e| panic!("{ad}: önizleme {e}"));
        }
        assert_eq!(doldur("sahne \"{1}|{2}\"", &["A".into(), "".into()]).matches('|').count(), 1);
    }
    #[test]
    fn rehber_gecerli() {
        for (ad, m) in REHBER {
            assert!(!baslik(m, "baslik").is_empty() && !baslik(m, "aciklama").is_empty(), "{ad}: başlık eksik");
            crate::senaryo::ayristir(m).unwrap_or_else(|e| panic!("{ad}: {e}"));
            crate::uret::onizleme(m, 1.5).unwrap_or_else(|e| panic!("{ad}: önizleme {e}"));
        }
    }
}
