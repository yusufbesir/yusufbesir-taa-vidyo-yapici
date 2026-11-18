//! Açıklama: 1.0 arayüzünün (web/index.html) yapısal sözleşme testleri: 6 tema ve jeton kontrastı, 5 kip ve kısayollar,
//! çekmece anahtarları, modül yuvaları (aiMount, malzemeMount, window.TaaApp), korunan JS id'leri, gömülü uç noktalar.
//! Sahip: Yusuf. Yazan: [Claude#UI Opus 5.5, yenileme-1.0, 2026-11-18]. Yalnız UI ajanı ya da Yusuf'un izniyle değişir; silme yok.
//! Çalıştırma: tasarim/yenileme/kilitli-cargo.sh test --release --test arayuz_ui

const SAYFA: &str = include_str!("../web/index.html");
const SUNUCU: &str = include_str!("../src/arayuz.rs");

const TEMALAR: [&str; 6] = ["vitrin", "vurus", "cephe", "kadraj", "manset", "eslik"];

fn jetonlar(tema: &str) -> std::collections::HashMap<String, String> {
    let bas = format!("html[data-tema=\"{tema}\"]{{--bg:");
    let i = SAYFA.find(&bas).unwrap_or_else(|| panic!("{tema} jeton satırı yok"));
    let govde = &SAYFA[i + bas.len() - 5..];
    let son = govde.find('}').unwrap();
    govde[..son]
        .split(';')
        .filter_map(|p| p.split_once(':'))
        .map(|(k, v)| (k.trim().to_string(), v.trim().to_string()))
        .collect()
}

fn lin(c: u8) -> f64 {
    let c = c as f64 / 255.0;
    if c <= 0.04045 { c / 12.92 } else { ((c + 0.055) / 1.055).powf(2.4) }
}
fn parlaklik(h: &str) -> f64 {
    assert!(h.len() == 7 && h.starts_with('#'), "hex değil: {h}");
    let n = u32::from_str_radix(&h[1..], 16).unwrap();
    let (r, g, b) = ((n >> 16) as u8, (n >> 8) as u8, n as u8);
    0.2126 * lin(r) + 0.7152 * lin(g) + 0.0722 * lin(b)
}
fn kontrast(a: &str, b: &str) -> f64 {
    let (x, y) = (parlaklik(a), parlaklik(b));
    (x.max(y) + 0.05) / (x.min(y) + 0.05)
}

#[test]
fn alti_tema_var_ve_varsayilan_vitrin() {
    for t in TEMALAR {
        assert!(SAYFA.contains(&format!("{{ad: '{t}'")), "{t} TEMALAR listesinde yok");
        let j = jetonlar(t);
        for k in ["--bg", "--panel", "--panel2", "--line", "--tx", "--mut", "--ac", "--onac", "--ok", "--uy"] {
            assert!(j.contains_key(k), "{t}: {k} jetonu yok");
        }
    }
    assert!(SAYFA.contains("T.indexOf(t) >= 0 ? t : 'vitrin'"), "varsayılan tema vitrin olmalı");
    // eski 0.9 değerlerinin göçü
    assert!(SAYFA.contains("if (t === 'a') t = 'vurus'; else if (t === 'b') t = 'vitrin';"));
    assert!(SAYFA.contains("gocRenk('taa-renk-a', 'taa-renk-vurus')"));
}

#[test]
fn jetonlar_karar_dosyasiyla_birebir() {
    // karar-fable-tur2.md'deki hex değerleri (bağlayıcı): örnek olarak her temanın zemin ve vurgusu
    let beklenen = [("vitrin", "#F1F0EC", "#9A1B3C"), ("vurus", "#191826", "#C2DC55"), ("cephe", "#112B34", "#FFA58C"),
                    ("kadraj", "#EAF2FA", "#174BD1"), ("manset", "#F5DC72", "#173C58"), ("eslik", "#351D2A", "#F4C3DA")];
    for (t, bg, ac) in beklenen {
        let j = jetonlar(t);
        assert_eq!(j["--bg"], bg, "{t} --bg"); assert_eq!(j["--ac"], ac, "{t} --ac");
    }
}

#[test]
fn jeton_kontrastlari_esigi_gecer() {
    // karar-fable-tur2 §7: metin çiftleri >= 4,5; çizgi/zemin >= 3
    let metin = [("--tx", "--bg"), ("--tx", "--panel2"), ("--mut", "--bg"), ("--mut", "--panel"), ("--mut", "--panel2"),
                 ("--onac", "--ac"), ("--ac", "--bg"), ("--ac", "--panel"), ("--ac", "--panel2"), ("--ok", "--panel2"), ("--uy", "--panel2")];
    for t in TEMALAR {
        let j = jetonlar(t);
        for (a, b) in metin {
            let k = kontrast(&j[a], &j[b]);
            assert!(k >= 4.5, "{t}: {a}/{b} = {k:.2} (< 4,5)");
        }
        for b in ["--bg", "--panel", "--panel2"] {
            let k = kontrast(&j["--line"], &j[b]);
            assert!(k >= 3.0, "{t}: --line/{b} = {k:.2} (< 3)");
        }
    }
}

#[test]
fn bes_kip_ad_sira_kisayol() {
    let kipler = [("yaz", "Yaz", "Alt+1"), ("hazir", "Hazır", "Alt+2"), ("yz", "Yapay zekâ", "Alt+3"), ("gorunum", "Görünüm", "Alt+4"), ("ayrinti", "Ayrıntılar", "Alt+5")];
    let mut son = 0;
    for (ad, baslik, tus) in kipler {
        let iz = format!("{{ad: '{ad}', baslik: '{baslik}'");
        let i = SAYFA.find(&iz).unwrap_or_else(|| panic!("kip {ad} yok"));
        assert!(i > son, "kip sırası bozuk: {ad}"); son = i;
        assert!(SAYFA[i..i + 160].contains(&format!("tus: '{tus}'")), "{ad} kısayolu {tus} değil");
        assert!(SAYFA.contains(&format!("class=\"kip") ) && SAYFA.contains(&format!("data-kip=\"{ad}\"")), "{ad} bölümü yok");
    }
    assert!(SAYFA.contains("/^Digit([1-5])$/"), "Alt+1..5 işleyicisi yok");
}

#[test]
fn cekmece_kurali_sayfada() {
    for s in ["'taa.cekmece.dar'", "'taa.cekmece'", "aria-controls=\"cekmece\"", "aria-expanded=", "Ctrl+B", "e.code === 'KeyB'",
              "translateX", "180ms cubic-bezier(.2,0,0,1)", "opacity 120ms", "window.innerWidth < 1200", "id=\"cekmeceKapat\"",
              "html[data-cekmece=\"kapali\"] .cekmece{display:none}", "cekBtn.focus()"] {
        assert!(SAYFA.contains(s), "çekmece: {s} yok");
    }
}

#[test]
fn modul_yuvalari_ve_taaapp() {
    for s in ["id=\"aiMount\"", "id=\"malzemeMount\"", "window.TaaApp = Object.freeze(", "senaryoMetni:", "senaryoyaEkle(metin)", "senaryoDegistir(metin)",
              "onizle(metin)", "uret()", "durum: () =>", "tema: () =>", "window.TaaAI.mount(yeni, window.TaaApp)", "window.TaaMalzeme.mount(el, window.TaaApp)",
              "<script src=\"/ai.js\"></script>", "<script src=\"/malzeme.js\"></script>", "Hazır senaryolar", "Malzeme kitaplığı"] {
        assert!(SAYFA.contains(s), "modül yuvası: {s} yok");
    }
    for s in ["(\"GET\", \"/ai.js\")", "(\"GET\", \"/malzeme.js\")", "include_str!(\"../web/ai.js\")", "include_str!(\"../web/malzeme.js\")", "starts_with(\"/yazi/\")"] {
        assert!(SUNUCU.contains(s), "sunucu: {s} yok");
    }
}

#[test]
fn mevcut_islev_idleri_korunur() {
    // 0.9.8 sayfasındaki tüm işlev id'leri (yalnız yerleşim değişti; hiçbir işlev kaybolmamalı)
    let idler = ["tarifKart", "adimlar", "tarifMetin", "tarifCoz", "tarifDogrudan", "tarifIstem", "tarifSiteler", "panoOneri", "panoUygula", "istemBolum", "istemMetin",
        "istemKopya2", "tarifYanit", "tarifUygula", "tarifSonuc", "llmBolum", "llmDurum", "llmSag", "llmModel", "llmAdresEt", "llmAdres", "llmAnahtar", "llmAcik", "llmSorma",
        "llmKaydet", "llmKaldir", "kuralBolum", "kuralSay", "kuralAra", "kuralListe", "hazirKart", "hazirSuz", "hazirListe", "hazirMesaj", "yazi", "sablonEtiket", "sablonAc",
        "sablonKaydet", "ornek", "sablonlar", "sifirla", "stiller", "sasirt", "renkler", "ayr", "animSay", "animGrup", "ara", "animListe", "giris", "cikis", "egriler", "fontlar",
        "paletler", "hex", "hexUygula", "derinlik", "derinlikV", "acilis", "kapanis", "cerceveler", "zeminler", "efektler", "dosya", "kodek", "ekran", "dongu", "koruma",
        "acilisBolum", "acilisSay", "acilisTam", "acilisHepsi", "acilisListe", "sen", "onizlePanel", "hazir", "durum", "durumMetin", "oneriler", "gorunum", "isikSeg",
        "uzaklik", "guvenli", "tamEkran", "tasarimFig", "tasarim", "simFig", "sim", "simBaslik", "binaFig", "bina", "binaBaslik", "binaAyar", "oynat", "zaman", "sure",
        "uret", "oynatSon", "klasor2", "yeniVideo", "iptal", "bar", "mesaj", "kaynak", "rehberKutu", "rehberKapat", "rehberListe", "rehberVideo", "rehberMesaj",
        "surum", "renkVurgu", "renkZemin", "renkSifirla", "rehberAc", "klasor", "yukleEkrani", "acilisCerceve", "ikiz-kabartma"];
    for id in idler {
        let n = SAYFA.matches(&format!("id=\"{id}\"")).count();
        assert_eq!(n, 1, "id=\"{id}\" sayfada {n} kez (tam 1 olmalı)");
    }
}

#[test]
fn ust_cubuk_ve_eylem_cubugu() {
    for s in ["id=\"cekmeceDugme\"", ">Yeni video<", ">Rehber<", ">Ayarlar<", "id=\"ayarKutu\"", "id=\"temaKartlar\"", "İğne, ölçü, ŞİMDİ",
              "Videoyu yap", "Son videoyu oynat", "Klasörü aç", "Düz", "Bina karşıdan", "Gündüz", "Akşam", "3 m", "15 m", "Güvenli alan", "Tam ekran",
              "prefers-reduced-motion", "--pervaz:#15171A"] {
        assert!(SAYFA.contains(s), "{s} yok");
    }
    // tek dolu düğme: dolu vurgu (background:var(--ac)) yalnız #uret'te
    assert!(SAYFA.contains("#uret{min-height:56px;padding:0 28px;font-size:17px;font-weight:700;background:var(--ac)"));
    // yasaklar: ışıma/bulanıklık/gradyan/steps()
    for y in ["backdrop-filter", "linear-gradient", "radial-gradient", "steps(", "blur("] {
        let css_son = SAYFA.find("</style>").unwrap();
        assert!(!SAYFA[..css_son].contains(y), "CSS'te yasak: {y}");
    }
}

#[test]
fn gomulu_yazilar_ve_lisanslar() {
    for (d, a) in [("spacegrotesk", "spacegrotesk"), ("archivo", "archivo"), ("nunito", "nunito"), ("atkinsonhyperlegible-regular", "atkinsonhyperlegible"), ("ibmplexmono-regular", "ibmplexmono")] {
        assert!(SAYFA.contains(&format!("url(/yazi/{d}.woff2)")), "{d} @font-face yok");
        assert!(SUNUCU.contains(&format!("\"{d}.woff2\"")), "{d} sunucuda yok");
        let lisans = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(format!("web/yazi/OFL-{a}.txt"));
        assert!(lisans.exists(), "{a} lisans dosyası yok");
    }
}
