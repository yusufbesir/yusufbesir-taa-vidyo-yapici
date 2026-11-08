//! Yazı stili: yerleşik fontlar (hepsi Türkçe karakter destekli, kalın) ve hazır renk paletleri.
use crate::Renk;
use ab_glyph::FontRef;

/// (ad, açıklama, dosya baytları)
pub const FONTLAR: &[(&str, &str, &[u8])] = &[
    ("inter", "Inter Display Bold: dengeli, temiz (varsayılan)", include_bytes!("../fonts/InterDisplay-Bold.otf")),
    ("anton", "Anton: dar ve çok kalın, uzaktan çok okunur", include_bytes!("../fonts/Anton-Regular.ttf")),
    ("archivo", "Archivo Black: geniş ve ağır", include_bytes!("../fonts/ArchivoBlack-Regular.ttf")),
    ("poppins", "Poppins Black: yuvarlak hatlı, modern", include_bytes!("../fonts/Poppins-Black.ttf")),
    ("barlow", "Barlow Condensed Black: dar, afiş tarzı", include_bytes!("../fonts/BarlowCondensed-Black.ttf")),
    ("slab", "Alfa Slab One: kalın tırnaklı, vitrin tarzı", include_bytes!("../fonts/AlfaSlabOne-Regular.ttf")),
    ("russo", "Russo One: teknik, güçlü", include_bytes!("../fonts/RussoOne-Regular.ttf")),
    ("fjalla", "Fjalla One: dar ve net", include_bytes!("../fonts/FjallaOne-Regular.ttf")),
    ("bangers", "Bangers: eğlenceli, çizgi roman", include_bytes!("../fonts/Bangers-Regular.ttf")),
    ("bebas", "Bebas Neue: dar, şık başlık", include_bytes!("../fonts/BebasNeue-Regular.ttf")),
    ("bungee", "Bungee: tabela harfleri, şehir afişi", include_bytes!("../fonts/Bungee-Regular.ttf")),
    ("bungee-ic", "Bungee Inline: içi çizgili tabela", include_bytes!("../fonts/BungeeInline-Regular.ttf")),
    ("black-ops", "Black Ops One: askeri şablon harf, çok güçlü", include_bytes!("../fonts/BlackOpsOne-Regular.ttf")),
    ("audiowide", "Audiowide: yuvarlak köşeli fütüristik", include_bytes!("../fonts/Audiowide-Regular.ttf")),
    ("righteous", "Righteous: sıcak, retro geometrik", include_bytes!("../fonts/Righteous-Regular.ttf")),
    ("kanit", "Kanit Black: modern, geniş ve ağır", include_bytes!("../fonts/Kanit-Black.ttf")),
    ("paytone", "Paytone One: dolgun, dostça", include_bytes!("../fonts/PaytoneOne-Regular.ttf")),
    ("sigmar", "Sigmar One: şişkin, eğlenceli kampanya", include_bytes!("../fonts/SigmarOne-Regular.ttf")),
    ("staatliches", "Staatliches: dar, şık afiş başlığı", include_bytes!("../fonts/Staatliches-Regular.ttf")),
    ("saira-dar", "Saira Extra Condensed Black: çok dar, uzun metin sığar", include_bytes!("../fonts/SairaExtraCondensed-Black.ttf")),
    ("rubik-mono", "Rubik Mono One: kalın tek aralıklı", include_bytes!("../fonts/RubikMonoOne-Regular.ttf")),
    ("yeseva", "Yeseva One: zarif, davetiye ve butik", include_bytes!("../fonts/YesevaOne-Regular.ttf")),
    ("dm-serif", "DM Serif Display: klasik, şık serif", include_bytes!("../fonts/DMSerifDisplay-Regular.ttf")),
    ("abril", "Abril Fatface: moda dergisi başlığı", include_bytes!("../fonts/AbrilFatface-Regular.ttf")),
    ("lobster", "Lobster: el yazısı afiş (süslü, 3 m'den kontrol edin)", include_bytes!("../fonts/Lobster-Regular.ttf")),
    ("pacifico", "Pacifico: akıcı el yazısı (süslü, 3 m'den kontrol edin)", include_bytes!("../fonts/Pacifico-Regular.ttf")),
];

pub fn font(ad: &str) -> Option<FontRef<'static>> {
    let b = FONTLAR.iter().find(|f| f.0 == ad)?.2;
    FontRef::try_from_slice(b).ok()
}
pub fn font_var_mi(ad: &str) -> bool { ad.is_empty() || FONTLAR.iter().any(|f| f.0 == ad) }

/// Palet: yazı yüzü, 3B yan yüz, zemin, vurgu (dekor ve efektler).
#[derive(Clone, Copy, Debug)]
pub struct Paletli { pub yuz: Renk, pub yan: Renk, pub zemin: Renk, pub vurgu: Renk }

fn hex(s: u32) -> Renk { Renk((s >> 16) as u8, (s >> 8) as u8, s as u8) }
fn yan_of(y: Renk) -> Renk { Renk(y.0 / 2, y.1 / 2, y.2 / 3) }

/// Yeni paletler: (ad, açıklama, yüz, zemin, vurgu). Renkler Tailwind CSS tonlarıdır.
pub const PALETLER: &[(&str, &str, u32, u32, u32)] = &[
    ("altin-gece", "Altın yazı, koyu lacivert zemin", 0xffd236, 0x020618, 0xffdf20),
    ("sicak-beyaz", "Kısık sıcak beyaz, nötr siyah zemin", 0xe8e2cd, 0x09090b, 0xffba00),
    ("buz-cyan", "Buz mavisi yazı, gece zemini", 0x53eafd, 0x020618, 0x00bcff),
    ("cyan-vurgu", "Canlı cyan, sade siyah", 0x00d3f3, 0x09090b, 0x00b8db),
    ("magenta-gece", "Fuşya yazı, gece zemini", 0xf4a8ff, 0x020618, 0xed6aff),
    ("pembe-parilti", "Pembe yazı, parıltılı vurgu", 0xfda5d5, 0x09090b, 0xfb64b6),
    ("gok-mavisi", "Gök mavisi yazı", 0x74d4ff, 0x020618, 0x00a6f4),
    ("altin-cyan", "Sarı yazı, cyan vurgu (sıcak-soğuk)", 0xffdf20, 0x020618, 0x53eafd),
    ("pembe-altin", "Altın yazı, pembe vurgu", 0xffd236, 0x09090b, 0xfda5d5),
    ("mor-altin", "Altın yazı, fuşya vurgu", 0xffd236, 0x020618, 0xed6aff),
    ("tek-altin", "Tek renk sarı-altın", 0xffdf20, 0x09090b, 0xffd236),
    ("sari-lacivert", "Sarı yazı, gece laciverti zemin", 0xffdf20, 0x020618, 0x74d4ff),
    ("neon-yesil", "Neon yeşil yazı, siyah zemin", 0x7bf1a8, 0x09090b, 0x05df72),
    ("limon", "Limon sarısı-yeşil, gece zemini", 0xd3f34b, 0x020618, 0xfff085),
    ("turuncu-gece", "Sıcak turuncu, gece zemini", 0xffb86a, 0x020618, 0xff8904),
    ("mercan", "Mercan kırmızısı, siyah zemin", 0xff8a8a, 0x09090b, 0xff6467),
    ("gun-batimi", "Turuncu yazı, pembe vurgu", 0xffb86a, 0x09090b, 0xfb64b6),
    ("okyanus", "Cyan yazı, gök mavisi vurgu", 0x53eafd, 0x020618, 0x00a6f4),
    ("mor-isik", "Lavanta yazı, mor vurgu", 0xdab2ff, 0x020618, 0xc27aff),
    ("lavanta-altin", "Lavanta yazı, altın vurgu", 0xdab2ff, 0x09090b, 0xffd236),
    ("nane", "Nane yeşili, gece zemini", 0x96f7e4, 0x020618, 0x46ecd5),
    ("seftali", "Şeftali pembesi, siyah zemin", 0xffccd3, 0x09090b, 0xff8ba7),
    ("kirmizi-altin", "Altın yazı, kırmızı vurgu", 0xffd236, 0x09090b, 0xff6467),
    ("yesil-altin", "Altın yazı, yeşil vurgu", 0xffd236, 0x020618, 0x05df72),
    ("buz-pembe", "Buz mavisi yazı, pembe vurgu", 0x74d4ff, 0x09090b, 0xfb64b6),
    ("beyaz-cyan", "Kısık beyaz yazı, cyan vurgu", 0xe8e2cd, 0x020618, 0x00d3f3),
    ("beyaz-pembe", "Kısık beyaz yazı, pembe vurgu", 0xe8e2cd, 0x09090b, 0xfb64b6),
    ("bakir", "Bakır tonu, sıcak siyah zemin", 0xffa36c, 0x09090b, 0xffd6a7),
    ("gumus-mavi", "Gümüş mavisi, gece zemini", 0xcad5e2, 0x020618, 0x74d4ff),
    ("gul-kurusu", "Gül kurusu, siyah zemin", 0xffa1ad, 0x09090b, 0xffd6a7),
];

/// Tüm paletlerin (eski renk adları dahil) adı ve açıklaması.
pub fn palet_listesi() -> Vec<(String, String)> {
    let eski = [("altin", "Altın (varsayılan)"), ("beyaz", "Kısık beyaz"), ("sari", "Sarı"), ("turuncu", "Turuncu"), ("cyan", "Cyan"), ("magenta", "Magenta"), ("lime", "Lime")];
    eski.iter().map(|(a, b)| (a.to_string(), b.to_string())).chain(PALETLER.iter().map(|p| (p.0.to_string(), p.1.to_string()))).collect()
}

pub fn paletli(ad: &str) -> Option<Paletli> {
    // Özel renk: "hex-ffcc00" (yazı rengi); zemin koyu, vurgu aynı renk. ('#' senaryoda yorum başlatır, o yüzden hex- öneki.)
    if let Some(h) = ad.strip_prefix("hex-") {
        if h.len() == 6 { if let Ok(v) = u32::from_str_radix(h, 16) { let yuz = hex(v); return Some(Paletli { yuz, yan: yan_of(yuz), zemin: hex(0x09090b), vurgu: yuz }); } }
        return None;
    }
    if let Some(&(_, _, y, z, v)) = PALETLER.iter().find(|p| p.0 == ad) {
        let (yuz, zemin, vurgu) = (hex(y), hex(z), hex(v));
        return Some(Paletli { yuz, yan: yan_of(yuz), zemin, vurgu });
    }
    let (yuz, zemin) = crate::palet(ad)?;
    Some(Paletli { yuz, yan: yan_of(yuz), zemin, vurgu: yuz })
}

#[cfg(test)]
mod t {
    use super::*;
    #[test]
    fn fontlar_turkce_harfleri_cizer() {
        use ab_glyph::Font;
        for (ad, _, _) in FONTLAR {
            let f = font(ad).unwrap_or_else(|| panic!("{ad} açılmadı"));
            for ch in "ŞşĞğİıÇçÖöÜü0123456789".chars() { assert!(f.glyph_id(ch).0 != 0, "{ad}: {ch} yok"); }
        }
    }
    #[test]
    fn paletler_kontrastli() {
        for p in PALETLER { let q = paletli(p.0).unwrap(); assert!(crate::kontrast(q.yuz, q.zemin) >= 7.0, "{} kontrast düşük", p.0); }
        assert!(paletli("altin").is_some() && paletli("yok-boyle").is_none());
    }
}
