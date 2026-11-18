//! Malzeme kitaplığı: LED ekran için hazır içerik kataloğu (oyun karakterleri, kayan yazı, sayaç, kutlama,
//! hava, ikon, kampanya rozeti, ok, logo açılışı, ekolayzer), senaryo satırı ayrıştırma ve senaryo üretici.
//! Senaryo dili eki (geriye uyumlu): `malzeme <id> [sure=S] [renk=PALET] [hiz=H] [deger=D] [yon=Y] [metin="..."] [efekt=a,b]`.
//! Çizim `malzeme_ciz.rs` içindedir; önizleme ve MP4 aynı `senaryo::kare_uret` yolundan geçer.
//! Sahip: Yusuf. Yazan: [Claude#Malzeme Opus 5.5, yenileme-1.0, 2026-11-18]. Yalnız yazan yapay zekâ ya da Yusuf'un izniyle değişir; silme yok.
use crate::senaryo::Sahne;

/// Sahneye eklenen malzeme ayarı (metin, renk, süre `Sahne` alanlarındadır).
#[derive(Clone, Debug, PartialEq)]
pub struct MalzemeAyar { pub id: String, pub hiz: f32, pub deger: f32, pub yon: String }

/// Katalog kalemi. `metin`/`deger`/`yon` None ise o parametre bu malzemede kullanılmaz.
pub struct Kalem {
    pub id: &'static str, pub ad: &'static str, pub kategori: &'static str, pub etiketler: &'static str, pub aciklama: &'static str,
    pub sure: f32, pub renk: &'static str, pub hiz: f32,
    pub metin: Option<&'static str>,
    /// (varsayılan, en az, en çok, etiket)
    pub deger: Option<(f32, f32, f32, &'static str)>,
    pub yon: Option<&'static str>,
    /// Canlı (saat/tarih): çıktı üretildiği ana bağlıdır, deterministik değildir.
    pub canli: bool,
}

pub const KATEGORILER: &[(&str, &str)] = &[
    ("oyun", "Oyun ve Karakter"), ("yazi", "Kayan Yazı"), ("zaman", "Sayaç ve Zaman"), ("kutlama", "Kutlama"), ("hava", "Doğa ve Hava"),
    ("ikon", "Kalp ve İkon"), ("kampanya", "Kampanya Rozeti"), ("yon", "Ok ve Yön"), ("acilis", "Logo ve Geçiş"), ("ses", "Ses ve Ekolayzer"),
];
pub const YONLER: &[(&str, &str)] = &[("sag", "Sağa"), ("sol", "Sola"), ("yukari", "Yukarı"), ("asagi", "Aşağı")];
pub const SURE_MIN: f32 = 2.0;
pub const SURE_MAX: f32 = 120.0;
pub const HIZ_MIN: f32 = 0.25;
pub const HIZ_MAX: f32 = 4.0;
pub const METIN_MAX: usize = 120;

const N: Option<&str> = None;
const ND: Option<(f32, f32, f32, &str)> = None;
const fn m(id: &'static str, ad: &'static str, kategori: &'static str, etiketler: &'static str, aciklama: &'static str, sure: f32, renk: &'static str, metin: Option<&'static str>, deger: Option<(f32, f32, f32, &'static str)>, yon: Option<&'static str>) -> Kalem {
    Kalem { id, ad, kategori, etiketler, aciklama, sure, renk, hiz: 1.0, metin, deger, yon, canli: false }
}

pub const KATALOG: &[Kalem] = &[
    // Oyun ve karakter (özgün karakterler: Yutkun, Kovalayıcılar, Robi, Zıpzıp, Ufocuk, Yılancık)
    m("yutkun-labirent", "Yutkun Labirentte", "oyun", "yutkun,labirent,oyun,retro,nokta", "Sarı Yutkun yılankavi labirent koridorunda noktaları yutar, renkli Kovalayıcılar peşinde; üstte skor.", 12.0, "gok-mavisi", Some("YUTKUN"), Some((4.0, 2.0, 6.0, "Koridor sayısı")), N),
    m("yutkun-kovalamaca", "Yutkun Kovalamaca", "oyun", "yutkun,kovalayici,oyun,retro,kovalamaca", "İki büyük koridorda Yutkun ile Kovalayıcılar ileri geri kovalamaca döngüsü.", 10.0, "gok-mavisi", Some("KOVALAMACA"), Some((3.0, 1.0, 4.0, "Kovalayıcı sayısı")), N),
    m("robi-yuruyus", "Robi Yürüyüşü", "oyun", "robot,piksel,yuruyen,karakter", "Piksel robot Robi zemin çizgisinde yürür, üstte yazı.", 8.0, "buz-cyan", Some("MERHABA"), ND, Some("sag")),
    m("zipzip", "Zıpzıp", "oyun", "piksel,ziplayan,karakter,sevimli", "Sevimli piksel Zıpzıp ekranda zıplayarak gezer, gölgesi büyüyüp küçülür.", 8.0, "limon", Some("HOŞ GELDİNİZ"), ND, N),
    m("ufocuk-istila", "Ufocuk İstilası", "oyun", "uzay,ufo,piksel,retro,oyun", "Sıra sıra piksel Ufocuklar sağa sola yürür, alttaki gemi atış yapar.", 10.0, "neon-yesil", Some("UZAYDAN FIRSAT"), Some((4.0, 1.0, 5.0, "Ufocuk satırı")), N),
    m("yilancik", "Yılancık", "oyun", "yilan,oyun,retro,izgara", "Izgarada kıvrılan Yılancık elmaları yiyerek uzar.", 10.0, "neon-yesil", N, ND, N),
    // Kayan yazı
    m("kayan-led", "LED Kayan Yazı", "yazi", "ticker,kayan,led,nokta,yazi", "Nokta matrisli büyük LED bandında kayan yazı; sönük noktalar gerçek tabela görünümü verir.", 10.0, "altin", Some("BUGÜNE ÖZEL KAMPANYA"), ND, Some("sol")),
    m("kayan-cift", "Çift Bant Kayan Yazı", "yazi", "ticker,kayan,cift,bant", "İki bant ters yönde kayar. İki farklı mesaj için metni | ile ayırın.", 10.0, "altin-cyan", Some("YENİ SEZON|ŞİMDİ MAĞAZADA"), ND, N),
    m("dikey-akis", "Dikey Akan Yazı", "yazi", "dikey,akan,jenerik,yazi", "Kelimeler alt alta yukarı doğru akar; dikey ekrana uygun jenerik.", 10.0, "sicak-beyaz", Some("KALİTE GÜVEN HIZ UYGUN FİYAT"), ND, N),
    m("harf-harf", "Harf Harf Yazı", "yazi", "daktilo,yazi,imlec", "Yazı harf harf belirir, sonunda yavaş yanıp sönen imleç.", 8.0, "neon-yesil", Some("SİPARİŞLERİNİZİ ALIYORUZ"), ND, N),
    m("dalgali-yazi", "Dalgalı Yazı", "yazi", "dalga,renkli,yazi", "Piksel yazının sütunları dalga gibi süzülür, renkler akar.", 8.0, "gun-batimi", Some("İNDİRİM"), ND, N),
    m("nabiz-yazi", "Nabız Yazı", "yazi", "nabiz,buyuk,yazi", "Büyük piksel yazı yavaşça büyüyüp küçülür (flaş yok).", 8.0, "mercan", Some("AÇILDIK"), ND, N),
    // Sayaç ve zaman
    m("geri-sayim", "Geri Sayım", "zaman", "geri sayim,sayac,zaman", "Büyük rakamlarla geri sayım ve çevresinde saniye halkası; sonunda yazı.", 12.0, "altin-gece", Some("BAŞLADI"), Some((10.0, 1.0, 999.0, "Başlangıç saniyesi")), N),
    m("saat-dijital", "Dijital Saat", "zaman", "saat,dijital,zaman,canli", "Saat:dakika büyük, saniye ve tarih altta. Değer -1: videonun üretildiği an.", 10.0, "buz-cyan", N, Some((-1.0, -1.0, 1439.0, "Başlangıç dakikası (-1 şimdi)")), N),
    m("tarih-takvim", "Takvim Yaprağı", "zaman", "tarih,takvim,gun,canli", "Takvim yaprağı açılır: ay, gün ve haftanın günü.", 8.0, "kirmizi-altin", N, Some((0.0, -365.0, 365.0, "Gün kaydırma (0 bugün)")), N),
    m("ilerleme-cubugu", "İlerleme Çubuğu", "zaman", "ilerleme,yukleniyor,cubuk,yuzde", "Çizgili çubuk dolar, üstte yüzde sayacı.", 8.0, "neon-yesil", Some("YÜKLENİYOR"), Some((100.0, 1.0, 100.0, "Hedef yüzde")), N),
    m("yuzde-sayaci", "Yüzde Sayacı", "zaman", "yuzde,halka,sayac", "Halka dolarken ortadaki yüzde sayısı artar.", 8.0, "okyanus", Some("MEMNUNİYET"), Some((75.0, 0.0, 100.0, "Hedef yüzde")), N),
    m("kum-saati", "Kum Saati", "zaman", "kum saati,zaman,bekleme", "Kum üst hazneden alta akar.", 10.0, "turuncu-gece", N, ND, N),
    // Kutlama
    m("havai-fisek-gosteri", "Havai Fişek Gösterisi", "kutlama", "havai fisek,kutlama,yilbasi,bayram", "Roketler yükselip renkli patlar (tam ekran flaş yok).", 10.0, "altin-gece", Some("İYİ BAYRAMLAR"), Some((5.0, 1.0, 10.0, "Roket sayısı")), N),
    m("konfeti-patlama", "Konfeti Patlaması", "kutlama", "konfeti,kutlama,tebrik", "Alt köşelerden konfeti fışkırır, dönerek düşer.", 8.0, "pembe-altin", Some("TEBRİKLER"), Some((80.0, 20.0, 200.0, "Konfeti adedi")), N),
    m("balon-kutlama", "Balon Kutlaması", "kutlama", "balon,kutlama,dogum gunu", "Renkli balonlar ipleriyle yükselir.", 8.0, "seftali", Some("İYİ Kİ DOĞDUN"), Some((7.0, 3.0, 15.0, "Balon sayısı")), N),
    m("yildiz-patlama", "Işın Patlaması", "kutlama", "isin,yildiz,patlama,vurgu", "Dönen ışınlar ve ortada büyük yıldız.", 8.0, "kirmizi-altin", Some("SÜPER"), ND, N),
    // Doğa ve hava
    m("kar-yagisi", "Kar Yağışı", "hava", "kar,kis,yilbasi,hava", "Altı kollu büyük kar taneleri süzülerek yağar.", 10.0, "gumus-mavi", N, Some((45.0, 10.0, 120.0, "Tane sayısı")), N),
    m("yagmur", "Yağmur", "hava", "yagmur,bulut,hava", "Bulutlardan eğik yağmur; yerde sıçrayan damlalar.", 10.0, "gok-mavisi", N, Some((120.0, 30.0, 300.0, "Damla sayısı")), N),
    m("parcacik-galaksi", "Yıldız Yolculuğu", "hava", "parcacik,uzay,yildiz,hiz", "Merkezden dışa akan yıldızlar (uzay yolculuğu).", 10.0, "sicak-beyaz", N, Some((220.0, 50.0, 400.0, "Yıldız sayısı")), N),
    m("kabarcik", "Kabarcıklar", "hava", "kabarcik,su,deniz", "Yükselen saydam kabarcıklar.", 10.0, "nane", N, Some((30.0, 5.0, 80.0, "Kabarcık sayısı")), N),
    // Kalp ve ikon
    m("kalp-atisi", "Kalp Atışı", "ikon", "kalp,sevgi,ekg,nabiz", "Büyük kalp çift vuruşla atar, altta kayan EKG çizgisi.", 8.0, "mercan", Some("SEVGİYLE"), ND, N),
    m("kalp-yagmuru", "Kalp Yağmuru", "ikon", "kalp,sevgililer,yukselen", "Büyüklü küçüklü kalpler sallanarak yükselir.", 8.0, "pembe-parilti", N, Some((14.0, 4.0, 40.0, "Kalp sayısı")), N),
    m("gulen-yuz", "Gülen Yüz", "ikon", "emoji,gulen,yuz,mutlu", "Göz kırpan büyük gülen yüz.", 8.0, "altin", N, ND, N),
    m("ask-yuz", "Kalp Gözlü Yüz", "ikon", "emoji,kalp,ask,yuz", "Kalp gözlü yüz ve yükselen kalpçikler.", 8.0, "altin", N, ND, N),
    m("onay-isareti", "Onay İşareti", "ikon", "onay,tamam,tik,basari", "Halka çizilir, ardından onay işareti belirir.", 6.0, "neon-yesil", Some("TAMAM"), ND, N),
    m("hediye-kutusu", "Hediye Kutusu", "ikon", "hediye,kutu,surpriz,yildiz", "Kapağı zıplayan hediye kutusundan yıldızlar çıkar.", 8.0, "kirmizi-altin", Some("HEDİYE"), ND, N),
    m("yildiz-parlak", "Parlayan Yıldız", "ikon", "yildiz,parilti,vurgu", "Yavaş dönen büyük yıldız ve çevresinde pırıltılar.", 8.0, "altin-gece", N, ND, N),
    // Kampanya rozetleri (yazı gömülü yazı tipiyle, 3B)
    m("rozet-yuzde", "Yüzde Rozeti", "kampanya", "indirim,yuzde,rozet,kampanya", "Dönen yıldız patlaması rozet içinde indirim oranı.", 6.0, "kirmizi-altin", Some("%30"), ND, N),
    m("rozet-taksit", "Taksit Rozeti", "kampanya", "taksit,rozet,kampanya,odeme", "Çizgileri akan büyük rozet: taksit mesajı.", 6.0, "kirmizi-altin", Some("3 TAKSİT"), ND, N),
    m("rozet-yeni", "Yeni Mührü", "kampanya", "yeni,muhur,rozet", "Esneyerek gelen yuvarlak mühür, dönen noktalı halka.", 6.0, "mor-altin", Some("YENİ"), ND, N),
    m("rozet-son-gunler", "Son Günler Kurdelesi", "kampanya", "son gunler,kurdele,acil,kampanya", "Kurdele bandı ve yavaş nabızlı oklar.", 6.0, "kirmizi-altin", Some("SON GÜNLER"), ND, N),
    m("rozet-indirim", "İndirim Etiketi", "kampanya", "indirim,etiket,fiyat", "Salınan fiyat etiketi.", 6.0, "turuncu-gece", Some("İNDİRİM"), ND, N),
    m("rozet-firsat", "Fırsat Levhası", "kampanya", "firsat,levha,isik,kampanya", "Üzerinden ışık geçen fırsat levhası.", 6.0, "yesil-altin", Some("FIRSAT"), ND, N),
    // Ok ve yön
    m("ok-akisi", "Ok Akışı", "yon", "ok,yon,akis,yonlendirme", "Sıra sıra oklar seçilen yöne akar.", 6.0, "altin", N, ND, Some("sag")),
    m("ok-nabiz", "Büyük Ok", "yon", "ok,yon,buyuk,isaret", "Büyük ok seçilen yöne itilir, izi kalır.", 6.0, "neon-yesil", Some("GİRİŞ"), ND, Some("asagi")),
    m("konum-pini", "Konum İğnesi", "yon", "konum,harita,burada,pin", "Zıplayan konum iğnesi ve gölgesi.", 6.0, "mercan", Some("BURADAYIZ"), ND, N),
    m("ok-donen", "Dönen Oklar", "yon", "ok,donen,yenile,dongu", "Daire çevresinde dönen iki ok.", 6.0, "okyanus", N, ND, N),
    // Logo ve geçiş
    m("logo-perde", "Perde Açılışı", "acilis", "logo,acilis,perde,isim", "Kadife perde iki yana açılır, isim/logo yazısı görünür.", 6.0, "altin-gece", Some("TAA"), ND, N),
    m("logo-iris", "İris Açılışı", "acilis", "logo,acilis,iris,daire", "Daire iris açılarak yazıyı gösterir.", 6.0, "buz-cyan", Some("TAA"), ND, N),
    m("logo-parlama", "Parlayan Logo", "acilis", "logo,acilis,isik,parlama", "Yazı yerine oturur, üzerinden ışık geçer.", 6.0, "tek-altin", Some("TAA"), ND, N),
    m("gecis-daire", "Halka Geçişi", "acilis", "gecis,halka,daire,sekil", "Merkezden yayılan renk halkaları.", 4.0, "mor-isik", N, ND, N),
    m("gecis-serit", "Şerit Geçişi", "acilis", "gecis,serit,capraz,sekil", "Akan çapraz şeritler.", 4.0, "altin-cyan", N, ND, N),
    m("geometri-kare", "Dönen Kareler", "acilis", "geometri,kare,donen,sekil", "İç içe zıt yönlerde dönen kareler.", 6.0, "mor-altin", N, ND, N),
    // Ses ve ekolayzer
    m("ekolayzer", "Ekolayzer", "ses", "ekolayzer,muzik,ses,cubuk", "LED bloklu ekolayzer çubukları (yeşil-sarı-kırmızı).", 8.0, "neon-yesil", N, Some((16.0, 6.0, 32.0, "Çubuk sayısı")), N),
    m("ses-dalgasi", "Ses Dalgası", "ses", "ses,dalga,muzik,sinyal", "Üst üste akan ses dalgaları.", 8.0, "okyanus", N, ND, N),
    m("dairesel-ekolayzer", "Dairesel Ekolayzer", "ses", "ekolayzer,daire,muzik", "Daire çevresinde ışınsal çubuklar; ortada yazı.", 8.0, "mor-isik", Some("MÜZİK"), Some((48.0, 16.0, 96.0, "Çubuk sayısı")), N),
    m("vu-metre", "VU Metre", "ses", "vu,metre,ibre,ses,analog", "İki analog VU metre ibresi müzikle salınır.", 8.0, "sicak-beyaz", N, ND, N),
];

pub fn bul(id: &str) -> Option<&'static Kalem> { KATALOG.iter().find(|k| k.id == id) }
pub fn canli_mi(id: &str) -> bool { matches!(id, "saat-dijital" | "tarih-takvim") || bul(id).map(|k| k.canli).unwrap_or(false) }

/// Metni senaryo satırına güvenle koyulacak hâle getirir: tırnak, `#`, satır sonu ve denetim karakterleri atılır, uzunluk sınırlanır.
pub fn metin_temizle(s: &str) -> String {
    let t: String = s.chars().filter(|c| *c != '"' && *c != '#').map(|c| if c.is_control() { ' ' } else { c }).collect();
    t.split_whitespace().collect::<Vec<_>>().join(" ").chars().take(METIN_MAX).collect()
}

/// `malzeme ` sonrasını (id ve anahtar=değer) ayrıştırır, doğrular ve sahne üretir.
pub fn satir_coz(r: &str) -> Result<Sahne, String> {
    let mut parcalar: Vec<(String, String)> = vec![];
    let b: Vec<char> = r.chars().collect();
    let mut i = 0;
    let mut id = String::new();
    while i < b.len() {
        while i < b.len() && b[i].is_whitespace() { i += 1; }
        if i >= b.len() { break; }
        let bas = i;
        while i < b.len() && !b[i].is_whitespace() && b[i] != '=' { i += 1; }
        let anahtar: String = b[bas..i].iter().collect();
        if i >= b.len() || b[i] != '=' {
            if id.is_empty() && parcalar.is_empty() { id = anahtar; continue; }
            return Err(format!("anahtar=değer bekleniyor: {anahtar}"));
        }
        i += 1;
        let deger: String = if i < b.len() && b[i] == '"' {
            i += 1;
            let s = i;
            while i < b.len() && b[i] != '"' { i += 1; }
            if i >= b.len() { return Err("metin kapanış tırnağı yok".into()); }
            let v = b[s..i].iter().collect();
            i += 1;
            v
        } else {
            let s = i;
            while i < b.len() && !b[i].is_whitespace() { i += 1; }
            b[s..i].iter().collect()
        };
        parcalar.push((anahtar, deger));
    }
    if id.is_empty() { return Err("malzeme adı eksik (örn. malzeme yutkun-labirent)".into()); }
    let k = bul(&id).ok_or_else(|| format!("bilinmeyen malzeme: {id}"))?;
    let mut sh = Sahne { metin: k.metin.unwrap_or("").to_string(), renk: k.renk.into(), giris: "yok".into(), cikis: "yok".into(), sure: k.sure, harf: 330.0, efekt: vec![], font: String::new(), derinlik: 1.0, egri: String::new(), malzeme: None };
    let mut ayar = MalzemeAyar { id: id.clone(), hiz: k.hiz, deger: k.deger.map(|d| d.0).unwrap_or(0.0), yon: k.yon.unwrap_or("sag").to_string() };
    let sayi = |v: &str, ad: &str| -> Result<f32, String> { v.parse::<f32>().ok().filter(|x| x.is_finite()).ok_or_else(|| format!("{ad} sayı olmalı")) };
    for (a, v) in parcalar {
        match a.as_str() {
            "sure" => { sh.sure = sayi(&v, "sure")?; if !(SURE_MIN..=SURE_MAX).contains(&sh.sure) { return Err(format!("sure {SURE_MIN} ile {SURE_MAX} sn arası olmalı")); } }
            "hiz" => { ayar.hiz = sayi(&v, "hiz")?; if !(HIZ_MIN..=HIZ_MAX).contains(&ayar.hiz) { return Err(format!("hiz {HIZ_MIN} ile {HIZ_MAX} arası olmalı")); } }
            "renk" | "palet" => { if crate::stil::paletli(&v).is_none() { return Err(format!("bilinmeyen renk/palet: {v}")); } sh.renk = v; }
            "deger" => {
                let (_, en_az, en_cok, etiket) = k.deger.ok_or_else(|| format!("{id} deger kullanmaz"))?;
                ayar.deger = sayi(&v, "deger")?;
                if !(en_az..=en_cok).contains(&ayar.deger) { return Err(format!("deger ({etiket}) {en_az} ile {en_cok} arası olmalı")); }
            }
            "yon" => { if k.yon.is_none() { return Err(format!("{id} yon kullanmaz")); } if !YONLER.iter().any(|y| y.0 == v) { return Err(format!("yon sag|sol|yukari|asagi olmalı: {v}")); } ayar.yon = v; }
            "metin" => { if k.metin.is_none() { return Err(format!("{id} metin kullanmaz")); } if v.chars().count() > METIN_MAX { return Err(format!("metin en çok {METIN_MAX} karakter")); } sh.metin = crate::senaryo::dinamik_metin(&v); }
            "efekt" => { sh.efekt = v.split(',').filter(|e| !e.is_empty() && *e != "yok").map(String::from).collect(); for e in &sh.efekt { if !crate::senaryo::EFEKTLER.contains(&e.as_str()) && crate::dekor::ADLAR.iter().all(|d| d.0 != e) { return Err(format!("bilinmeyen efekt: {e}")); } } }
            _ => return Err(format!("bilinmeyen malzeme anahtarı: {a} (sure, renk, hiz, deger, yon, metin, efekt)")),
        }
    }
    sh.malzeme = Some(ayar);
    Ok(sh)
}

/// Parametre isteği (HTTP ya da arayüzden). Boş alan = katalog varsayılanı.
#[derive(Default, Debug, Clone)]
pub struct Istek { pub id: String, pub sure: Option<f32>, pub renk: Option<String>, pub hiz: Option<f32>, pub metin: Option<String>, pub deger: Option<f32>, pub yon: Option<String> }

fn sayi_yaz(x: f32) -> String { let s = format!("{:.2}", x); let s = s.trim_end_matches('0').trim_end_matches('.'); if s.is_empty() || s == "-0" { "0".into() } else { s.to_string() } }

/// Senaryo üretici: parametrelerden tek satırlık senaryo parçası (`malzeme ...`). Çıktı yeniden ayrıştırılarak doğrulanır.
pub fn senaryo_parcasi(q: &Istek) -> Result<String, String> {
    let k = bul(q.id.trim()).ok_or_else(|| format!("bilinmeyen malzeme: {}", q.id))?;
    if q.metin.is_some() && k.metin.is_none() { return Err(format!("{} metin kullanmaz", k.id)); }
    if q.deger.is_some() && k.deger.is_none() { return Err(format!("{} deger kullanmaz", k.id)); }
    if q.yon.is_some() && k.yon.is_none() { return Err(format!("{} yon kullanmaz", k.id)); }
    let mut s = format!("malzeme {} sure={} renk={} hiz={}", k.id, sayi_yaz(q.sure.unwrap_or(k.sure)), q.renk.as_deref().unwrap_or(k.renk).trim(), sayi_yaz(q.hiz.unwrap_or(k.hiz)));
    if let Some(d) = k.deger { s += &format!(" deger={}", sayi_yaz(q.deger.unwrap_or(d.0))); }
    if let Some(y) = k.yon { s += &format!(" yon={}", q.yon.as_deref().unwrap_or(y).trim()); }
    if let Some(mv) = k.metin {
        let t = metin_temizle(q.metin.as_deref().unwrap_or(mv));
        s += &format!(" metin=\"{t}\"");
    }
    satir_coz(&s["malzeme ".len()..])?;
    Ok(s)
}

/// Önizleme için tam senaryo (950x1800 dikey).
pub fn onizleme_senaryosu(parca: &str) -> String { format!("ekran 950x1800\nfps 25\n{parca}\n") }

pub fn json_kac(s: &str) -> String {
    let mut o = String::with_capacity(s.len() + 2);
    for c in s.chars() { match c { '"' => o += "\\\"", '\\' => o += "\\\\", '\n' => o += "\\n", '\r' => o += "\\r", '\t' => o += "\\t", c if (c as u32) < 0x20 => o += &format!("\\u{:04x}", c as u32), c => o.push(c) } }
    o
}

/// Katalog JSON'u (GET /malzeme/liste).
pub fn liste_json() -> String {
    let kat: Vec<String> = KATEGORILER.iter().map(|(a, b)| format!("{{\"id\":\"{a}\",\"ad\":\"{}\",\"adet\":{}}}", json_kac(b), KATALOG.iter().filter(|k| k.kategori == *a).count())).collect();
    let pal: Vec<String> = crate::stil::palet_listesi().iter().map(|(a, b)| { let p = crate::stil::paletli(a).unwrap(); format!("{{\"ad\":\"{}\",\"acik\":\"{}\",\"yuz\":\"#{:02x}{:02x}{:02x}\",\"zemin\":\"#{:02x}{:02x}{:02x}\",\"vurgu\":\"#{:02x}{:02x}{:02x}\"}}", json_kac(a), json_kac(b), p.yuz.0, p.yuz.1, p.yuz.2, p.zemin.0, p.zemin.1, p.zemin.2, p.vurgu.0, p.vurgu.1, p.vurgu.2) }).collect();
    let yon: Vec<String> = YONLER.iter().map(|(a, b)| format!("{{\"id\":\"{a}\",\"ad\":\"{}\"}}", json_kac(b))).collect();
    let mz: Vec<String> = KATALOG.iter().map(|k| {
        let mut sema = vec![
            format!("{{\"ad\":\"sure\",\"etiket\":\"Süre (sn)\",\"tur\":\"sayi\",\"min\":{},\"max\":{},\"adim\":0.5,\"varsayilan\":{}}}", sayi_yaz(SURE_MIN), sayi_yaz(SURE_MAX), sayi_yaz(k.sure)),
            format!("{{\"ad\":\"renk\",\"etiket\":\"Renk paleti\",\"tur\":\"palet\",\"varsayilan\":\"{}\"}}", k.renk),
            format!("{{\"ad\":\"hiz\",\"etiket\":\"Hız\",\"tur\":\"sayi\",\"min\":{},\"max\":{},\"adim\":0.25,\"varsayilan\":{}}}", sayi_yaz(HIZ_MIN), sayi_yaz(HIZ_MAX), sayi_yaz(k.hiz)),
        ];
        if let Some(t) = k.metin { sema.push(format!("{{\"ad\":\"metin\",\"etiket\":\"Yazı\",\"tur\":\"metin\",\"max\":{METIN_MAX},\"varsayilan\":\"{}\"}}", json_kac(t))); }
        if let Some((v, a, b, e)) = k.deger { sema.push(format!("{{\"ad\":\"deger\",\"etiket\":\"{}\",\"tur\":\"sayi\",\"min\":{},\"max\":{},\"adim\":1,\"varsayilan\":{}}}", json_kac(e), sayi_yaz(a), sayi_yaz(b), sayi_yaz(v))); }
        if let Some(y) = k.yon { sema.push(format!("{{\"ad\":\"yon\",\"etiket\":\"Yön\",\"tur\":\"yon\",\"varsayilan\":\"{y}\"}}")); }
        let etiket: Vec<String> = k.etiketler.split(',').map(|e| format!("\"{}\"", json_kac(e.trim()))).collect();
        let parca = senaryo_parcasi(&Istek { id: k.id.into(), ..Default::default() }).unwrap_or_default();
        format!("{{\"id\":\"{}\",\"ad\":\"{}\",\"kategori\":\"{}\",\"etiketler\":[{}],\"aciklama\":\"{}\",\"canli\":{},\"varsayilan\":{{\"sure\":{},\"renk\":\"{}\",\"hiz\":{},\"metin\":{},\"deger\":{},\"yon\":{}}},\"sema\":[{}],\"senaryo\":\"{}\"}}",
            k.id, json_kac(k.ad), k.kategori, etiket.join(","), json_kac(k.aciklama), canli_mi(k.id), sayi_yaz(k.sure), k.renk, sayi_yaz(k.hiz),
            k.metin.map(|t| format!("\"{}\"", json_kac(t))).unwrap_or("null".into()), k.deger.map(|d| sayi_yaz(d.0)).unwrap_or("null".into()), k.yon.map(|y| format!("\"{y}\"")).unwrap_or("null".into()),
            sema.join(","), json_kac(&parca))
    }).collect();
    format!("{{\"surum\":1,\"adet\":{},\"kategoriler\":[{}],\"paletler\":[{}],\"yonler\":[{}],\"malzemeler\":[{}]}}", KATALOG.len(), kat.join(","), pal.join(","), yon.join(","), mz.join(","))
}

/// Düz (iç içe olmayan) JSON nesnesi ayrıştırıcı: metin, sayı, true/false/null değerleri. Hatalıysa None.
pub fn json_nesne(s: &str) -> Option<Vec<(String, String)>> {
    let b = s.as_bytes();
    let mut i = 0;
    let bos = |i: &mut usize| { while *i < b.len() && matches!(b[*i], b' ' | b'\t' | b'\r' | b'\n') { *i += 1; } };
    let metin = |i: &mut usize| -> Option<String> {
        if b.get(*i) != Some(&b'"') { return None; }
        *i += 1;
        let mut o: Vec<u8> = vec![];
        loop {
            let c = *b.get(*i)?;
            *i += 1;
            match c {
                b'"' => break,
                b'\\' => {
                    let e = *b.get(*i)?; *i += 1;
                    match e {
                        b'"' => o.push(b'"'), b'\\' => o.push(b'\\'), b'/' => o.push(b'/'), b'n' => o.push(b'\n'), b't' => o.push(b'\t'), b'r' => o.push(b'\r'), b'b' | b'f' => o.push(b' '),
                        b'u' => {
                            let h = std::str::from_utf8(b.get(*i..*i + 4)?).ok()?; *i += 4;
                            let mut k = u32::from_str_radix(h, 16).ok()?;
                            if (0xD800..0xDC00).contains(&k) && b.get(*i) == Some(&b'\\') && b.get(*i + 1) == Some(&b'u') {
                                let h2 = std::str::from_utf8(b.get(*i + 2..*i + 6)?).ok()?; let k2 = u32::from_str_radix(h2, 16).ok()?;
                                if (0xDC00..0xE000).contains(&k2) { k = 0x10000 + ((k - 0xD800) << 10) + (k2 - 0xDC00); *i += 6; }
                            }
                            let mut t = [0u8; 4];
                            o.extend(char::from_u32(k).unwrap_or('?').encode_utf8(&mut t).as_bytes());
                        }
                        _ => return None,
                    }
                }
                c if c < 0x20 => return None,
                c => o.push(c),
            }
        }
        String::from_utf8(o).ok()
    };
    bos(&mut i);
    if b.get(i) != Some(&b'{') { return None; }
    i += 1;
    let mut v = vec![];
    bos(&mut i);
    if b.get(i) == Some(&b'}') { i += 1; bos(&mut i); return if i == b.len() { Some(v) } else { None }; }
    loop {
        bos(&mut i);
        let k = metin(&mut i)?;
        bos(&mut i);
        if b.get(i) != Some(&b':') { return None; }
        i += 1;
        bos(&mut i);
        let deger = if b.get(i) == Some(&b'"') { metin(&mut i)? } else {
            let s0 = i;
            while i < b.len() && !matches!(b[i], b',' | b'}' | b' ' | b'\t' | b'\r' | b'\n') { i += 1; }
            let t = std::str::from_utf8(&b[s0..i]).ok()?;
            if t == "true" || t == "false" || t == "null" || t.parse::<f64>().is_ok() { t.to_string() } else { return None; }
        };
        if v.len() >= 32 { return None; }
        v.push((k, deger));
        bos(&mut i);
        match b.get(i) { Some(b',') => { i += 1; } Some(b'}') => { i += 1; break; } _ => return None }
    }
    bos(&mut i);
    if i != b.len() { return None; }
    Some(v)
}

/// JSON gövdesinden istek. Bilinmeyen anahtar ya da hatalı tip hata döner.
pub fn istek_coz(govde: &str) -> Result<Istek, String> {
    let alanlar = json_nesne(govde).ok_or("bozuk JSON")?;
    let mut q = Istek::default();
    let sayi = |v: &str, ad: &str| -> Result<Option<f32>, String> { if v == "null" || v.is_empty() { return Ok(None); } v.parse::<f32>().ok().filter(|x| x.is_finite()).map(Some).ok_or_else(|| format!("{ad} sayı olmalı")) };
    let yazi = |v: String| if v == "null" { None } else { Some(v) };
    for (k, v) in alanlar {
        match k.as_str() {
            "id" => q.id = v,
            "sure" => q.sure = sayi(&v, "sure")?,
            "hiz" => q.hiz = sayi(&v, "hiz")?,
            "deger" => q.deger = sayi(&v, "deger")?,
            "renk" => q.renk = yazi(v).filter(|x| !x.is_empty()),
            "yon" => q.yon = yazi(v).filter(|x| !x.is_empty()),
            "metin" => q.metin = yazi(v),
            "t" => {}
            _ => return Err(format!("bilinmeyen alan: {k}")),
        }
    }
    if q.id.trim().is_empty() { return Err("id gerekli".into()); }
    if let Some(r) = &q.renk { if r.chars().any(|c| c.is_whitespace() || c == '"' || c == '#') { return Err("renk geçersiz".into()); } }
    if let Some(r) = &q.yon { if r.chars().any(|c| c.is_whitespace()) { return Err("yon geçersiz".into()); } }
    Ok(q)
}

#[cfg(test)]
mod t {
    use super::*;
    #[test]
    fn katalog_kurallari() {
        assert!(KATALOG.len() >= 40, "en az 40 malzeme");
        let mut ids: Vec<&str> = KATALOG.iter().map(|k| k.id).collect();
        ids.sort(); ids.dedup();
        assert_eq!(ids.len(), KATALOG.len(), "kimlikler tekil olmalı");
        for (kat, _) in KATEGORILER { assert!(KATALOG.iter().filter(|k| k.kategori == *kat).count() >= 4, "{kat} en az 4 malzeme"); }
        assert_eq!(KATEGORILER.len(), 10);
        for k in KATALOG { assert!(KATEGORILER.iter().any(|c| c.0 == k.kategori), "{}", k.id); assert!(crate::stil::paletli(k.renk).is_some(), "{} palet", k.id); }
    }
    #[test]
    fn satir_ve_hatalar() {
        let s = satir_coz("yutkun-labirent sure=9 deger=3 metin=\"MERHABA DÜNYA\" hiz=2").unwrap();
        assert_eq!(s.sure, 9.0); assert_eq!(s.metin, "MERHABA DÜNYA");
        let m = s.malzeme.unwrap(); assert_eq!((m.deger, m.hiz), (3.0, 2.0));
        assert!(satir_coz("yok-boyle").is_err());
        assert!(satir_coz("").is_err());
        assert!(satir_coz("yutkun-labirent deger=99").is_err());
        assert!(satir_coz("yutkun-labirent hiz=0").is_err());
        assert!(satir_coz("yutkun-labirent sure=1").is_err());
        assert!(satir_coz("yutkun-labirent metin=\"acik").is_err());
        assert!(satir_coz("yutkun-labirent yon=sag").is_err());
        assert!(satir_coz("kum-saati metin=\"A\"").is_err());
        assert!(satir_coz("ok-akisi yon=capraz").is_err());
        assert!(satir_coz("ok-akisi bilinmez=1").is_err());
        assert!(satir_coz("ok-akisi renk=yok-renk").is_err());
    }
    #[test]
    fn uretici_temizler_ve_dogrular() {
        let p = senaryo_parcasi(&Istek { id: "kayan-led".into(), metin: Some("A\"B#C\nD".into()), ..Default::default() }).unwrap();
        assert!(p.contains("metin=\"ABC D\""), "{p}");
        assert!(senaryo_parcasi(&Istek { id: "kayan-led".into(), hiz: Some(9.0), ..Default::default() }).is_err());
        let uzun = "X".repeat(500);
        let p = senaryo_parcasi(&Istek { id: "kayan-led".into(), metin: Some(uzun), ..Default::default() }).unwrap();
        assert!(p.len() < 300);
    }
    #[test]
    fn json_ayristirma() {
        let q = istek_coz(r#"{"id":"geri-sayim","sure":12.5,"deger":5,"metin":"BAŞLA","renk":null}"#).unwrap();
        assert_eq!((q.id.as_str(), q.sure, q.deger, q.metin.as_deref(), q.renk), ("geri-sayim", Some(12.5), Some(5.0), Some("BAŞLA"), None));
        for bozuk in ["", "{", "[]", "{\"id\":}", "{\"id\":\"a\",}", "{\"id\":\"a\"} x", "{\"id\":tru}", "{\"x\":1}", "{\"id\":\"a\",\"sure\":\"abc\"}"] { assert!(istek_coz(bozuk).is_err(), "{bozuk}"); }
        assert!(liste_json().starts_with("{\"surum\":1"));
    }
}
