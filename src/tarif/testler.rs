//! Tarif et birim testleri: JSON, doğrulama, enjeksiyon, sınırlar, istem, kural dosyası. Sahip: Yusuf; yazan [Claude#ebd691, 2026-11-16].
use super::*;
fn kat() -> Katalog { Katalog::gercek() }
fn tek(js: &str) -> Result<Dogrulama, Vec<String>> { dogrula(js, &kat()) }

#[test]
fn json_temel() {
    assert_eq!(json_coz("{\"a\":[1,2,],}").unwrap().al("a").unwrap().dizi().unwrap().len(), 2); // sondaki virgül toleransı
    assert!(json_coz("{\"a\":").is_err());
    assert!(json_coz("{\"a\":1} fazla").is_err());
    assert_eq!(json_coz("\"a\\u00e7\\n\"").unwrap(), Json::Yazi("aç\n".into()));
    let derin = "[".repeat(40) + &"]".repeat(40);
    assert!(json_coz(&derin).is_err(), "çok derin yapı reddedilmeli");
}
#[test]
fn json_bulma_kod_citi_ve_aciklama() {
    let y = "Tabii! İşte sahne listesi:\n```json\n{\"sahneler\":[{\"metin\":[\"A } B\"]}]}\n```\nBaşka bir şey lazım mı?";
    let j = json_bul(y).unwrap();
    assert!(j.starts_with('{') && j.ends_with('}') && j.contains("A } B"));
    assert!(json_bul("hiç JSON yok").is_none());
    assert!(json_bul("{\"yarım\":").is_none());
}
#[test]
fn ornek_yanit_temiz_gecer() {
    let d = tek(ORNEK_YANIT).unwrap();
    assert!(d.notlar.is_empty(), "örnek yanıt hiç not üretmemeli: {:?}", d.notlar);
    assert_eq!(d.tarif.sahneler.len(), 2);
    assert_eq!(d.tarif.acilis, "perde");
}
#[test]
fn bozuk_json_ve_sema() {
    assert!(tek("merhaba").is_err());
    assert!(tek("{\"sahneler\":").is_err());
    assert!(tek("{\"surum\":1}").is_err());
    assert!(tek("{\"sahneler\":[]}").is_err());
    assert!(tek("{\"sahneler\":[{\"giris\":\"zipla\"}]}").is_err(), "metinsiz sahne reddedilir");
    assert!(tek("[1,2,3]").is_err());
    assert!(tek(&"x".repeat(MAX_YANIT + 1)).is_err());
}
#[test]
fn sinirlar() {
    let sahne = "{\"metin\":[\"A\"],\"sure\":3}";
    let on_uc = format!("{{\"sahneler\":[{}]}}", vec![sahne; 13].join(","));
    assert!(tek(&on_uc).is_err(), "13 sahne reddedilir");
    let uzun = format!("{{\"sahneler\":[{}]}}", vec!["{\"metin\":[\"A\"],\"sure\":15}"; 9].join(","));
    assert!(tek(&uzun).is_err(), "135 sn reddedilir");
    let d = tek("{\"sahneler\":[{\"metin\":[\"1\",\"2\",\"3\",\"4\",\"5\",\"6\",\"7\",\"8\"],\"sure\":99}]}").unwrap();
    assert_eq!(d.tarif.sahneler[0].metin.len(), MAX_SATIR);
    assert_eq!(d.tarif.sahneler[0].sure, MAX_SURE);
    let d = tek(&format!("{{\"sahneler\":[{{\"metin\":[\"{}\"],\"sure\":0.2}}]}}", "KELİME ".repeat(20))).unwrap();
    assert!(d.tarif.sahneler[0].metin[0].chars().count() <= MAX_KARAKTER);
    assert_eq!(d.tarif.sahneler[0].sure, MIN_SURE);
    assert!(d.notlar.len() >= 2);
}
#[test]
fn bilinmeyen_ad_en_yakina_eslenir() {
    let d = tek("{\"sahneler\":[{\"metin\":[\"A\"],\"giris\":\"Zıplayarak\",\"cikis\":\"sonduur\",\"efekt\":[\"konfetti\",\"xyzxyz\"],\"palet\":\"#ff8800\",\"font\":\"Anton\"}],\"acilis\":\"PERDE\",\"kapanis\":\"fadee\"}").unwrap();
    let s = &d.tarif.sahneler[0];
    assert_eq!((s.giris.as_str(), s.cikis.as_str()), ("zipla", "sondur"));
    assert_eq!(s.efekt, vec!["konfeti".to_string()]);
    assert_eq!(s.palet, "hex-ff8800");
    assert_eq!(s.font, "anton");
    assert_eq!((d.tarif.acilis.as_str(), d.tarif.kapanis.as_str()), ("perde", "fade"));
    assert!(d.notlar.iter().any(|n| n.contains("xyzxyz")), "eşlenemeyen ad kullanıcıya bildirilir");
}
#[test]
fn enjeksiyon_denemeleri_temizlenir() {
    let kotu = [
        "<script>alert(1)</script>SATIR", "<b>KALIN</b> METİN", "https://kotu.example/x?y=1 ADRES", "www.kotu.example ADRES",
        "C:\\Windows\\System32\\cmd.exe ÇALIŞTIR", "/etc/passwd OKU", "..\\..\\gizli.txt AÇ", "$(calc) HESAP", "`whoami` KİM",
        "powershell -c Remove-Item", "rm -rf / SİL", "cmd.exe /c del x", "ŞU \"#\" {tarih} <> ; & | ^ ~ = * KARAKTERLER",
    ];
    for k in kotu {
        let js = format!("{{\"sahneler\":[{{\"metin\":[\"{}\",\"TAMAM\"]}}]}}", k.replace('\\', "\\\\").replace('"', "\\\""));
        let d = tek(&js).unwrap();
        for satir in &d.tarif.sahneler[0].metin {
            for yasak in ["<", ">", "://", "www.", "\\", "$(", "`", ";", "&", "|", "{", "}", "#", "\"", "powershell", "cmd.exe", "rm -rf"] {
                assert!(!satir.to_lowercase().contains(yasak), "{k:?} → {satir:?} içinde {yasak:?} kaldı");
            }
            assert!(!satir.starts_with('/'), "{k:?} → {satir:?} yol gibi kaldı");
        }
    }
    let d = tek("{\"sahneler\":[{\"metin\":[\"<b>X</b>\"]}]}").unwrap();
    assert!(d.notlar.iter().any(|n| n.contains("temizlendi")), "temizlenen metin kullanıcıya bildirilir");
}
#[test]
fn senaryo_metni_enjeksiyon_yapmaz() {
    // tırnak, satır sonu ve anahtar=değer sızdırma denemesi senaryo satırını bozamaz
    let d = tek("{\"sahneler\":[{\"metin\":[\"A\\\" sure=1 giris=x\\nsahne \\\"B\"]}]}").unwrap();
    let sn = crate::senaryo::ayristir(&senaryo_metni(&d.tarif)).unwrap();
    assert!(sn.sahneler.len() <= 2);
    assert!(sn.sahneler.iter().all(|s| s.giris == "belir"));
}
#[test]
fn bes_tarif_yanit_cifti() {
    let yanitlar = [
        "```json\n{\"surum\":1,\"ekran\":\"dikey\",\"gorunum\":\"duz\",\"sahneler\":[{\"metin\":[\"HOŞ GELDİNİZ\"],\"giris\":\"belir\",\"cikis\":\"yok\",\"efekt\":[\"parlama\"],\"palet\":\"altin\",\"font\":\"\",\"sure\":4.5}],\"acilis\":\"yok\",\"kapanis\":\"yok\"}\n```",
        "Elbette! Buyur:\n{\"sahneler\":[{\"metin\":\"%30|İNDİRİM\",\"giris\":\"zipla\",\"palet\":\"kirmizi-altin\",\"sure\":\"3,5\"}],\"kapanis\":\"fade\"}\nUmarım işine yarar.",
        "{\"sahneler\":[{\"metin\":[\"O “harika” gün\"],\"giris\":\"belir\",\"efekt\":\"konfeti, kar\",\"palet\":\"beyaz\",\"sure\":5}]}",
        "{\"surum\":1,\"gorunum\":\"bina\",\"sahneler\":[{\"metin\":[\"BİNA\",\"TESTİ\"],\"giris\":\"alttan\",\"cikis\":\"yukari\",\"sure\":4},{\"metin\":[\"İKİNCİ\"],\"giris\":\"soldan\",\"sure\":4}],\"acilis\":\"tv\"}",
        "{\"sahneler\":[{\"metin\":[\"SADECE METİN\"]}]}",
    ];
    let beklenen_sahne = [1, 1, 1, 2, 1];
    for (y, n) in yanitlar.iter().zip(beklenen_sahne) {
        let d = tek(y).unwrap_or_else(|e| panic!("{y} → {e:?}"));
        assert_eq!(d.tarif.sahneler.len(), n);
        crate::senaryo::ayristir(&senaryo_metni(&d.tarif)).unwrap_or_else(|e| panic!("senaryo hatası: {e}"));
    }
    assert_eq!(tek(yanitlar[1]).unwrap().tarif.sahneler[0].sure, 3.5);
    assert_eq!(tek(yanitlar[3]).unwrap().tarif.gorunum, "bina");
    assert_eq!(tek(yanitlar[2]).unwrap().tarif.sahneler[0].efekt, vec!["konfeti".to_string(), "kar".to_string()]);
}
#[test]
fn gomulu_kurallar_gecerli_ve_yeterli() {
    let k = gomulu_kurallar();
    assert!(!k.is_empty(), "gömülü kural dosyası okunamadı");
    let kat = kat();
    for r in k { kural_gecerli(r, &kat).unwrap_or_else(|e| panic!("{e}")); }
    assert!(kural_ifade_sayisi(k) >= 300, "en az 300 ifade olmalı, var: {}", kural_ifade_sayisi(k));
    let renk: usize = k.iter().filter(|r| r.kategori == "palet").map(|r| r.ifadeler.len()).sum();
    assert!(renk >= 40, "en az 40 renk ifadesi olmalı, var: {renk}");
    for c in KATEGORILER { assert!(k.iter().any(|r| r.kategori == c), "kategori boş: {c}"); }
}
#[test]
fn kural_ekle_dogrular() {
    let kat = kat();
    let j = kural_ekle(None, "giris", "Zıplatarak Gel", "zipla", &kat).unwrap();
    let j2 = kural_ekle(Some(&j), "giris", "hoplayarak", "zipla", &kat).unwrap();
    let k = kurallari_oku(&j2).unwrap();
    assert_eq!(k.len(), 1);
    assert_eq!(k[0].ifadeler.len(), 2);
    assert!(kural_ekle(None, "giris", "x y", "olmayan-ad", &kat).is_err(), "katalogda olmayan hedef reddedilir");
    assert!(kural_ekle(None, "giris", "<b>", "zipla", &kat).is_err(), "uygunsuz ifade reddedilir");
    assert!(kural_ekle(None, "palet", "turuncumsu", "hex-ff8800", &kat).is_ok());
    assert!(kural_ekle(None, "kotu", "x", "zipla", &kat).is_err());
    let z = kural_coz("\"MERHABA\" hoplayarak gelsin", &kat, &kurallari_oku(&j2).unwrap());
    assert_eq!(z.tarif.unwrap().sahneler[0].giris, "zipla");
}
#[test]
fn istem_gercek_katalogdan() {
    let k = kat();
    let i = istem_uret("Hoş geldiniz yaz, altın olsun", &k);
    for ad in ["zipla", "konfeti", "altin", "anton", "perde", "YALNIZ", "<<<", "Hoş geldiniz yaz"] { assert!(i.contains(ad), "istemde yok: {ad}"); }
    assert!(i.len() < 20_000, "istem fazla uzun: {}", i.len());
    let d = duzelt_istemi(&["Sahne 1: metin boş.".into()], &k);
    assert!(d.contains("Sahne 1: metin boş.") && d.contains("JSON"));
    assert!(dogrula(ORNEK_YANIT, &k).unwrap().notlar.is_empty(), "istemdeki örnek katalogla kopmamalı");
    assert!(istem_uret(&"a".repeat(50_000), &k).len() < 30_000, "aşırı uzun tarif kırpılır");
}
#[test]
fn asiri_uzun_tarif_cokmez() {
    let k = kat();
    let z = kural_coz(&"\"METİN\" zıpla, ".repeat(2000), &k, &[]);
    assert!(z.tarif.map(|t| t.sahneler.len() <= MAX_SAHNE).unwrap_or(true));
    let z = kural_coz("", &k, &[]);
    assert!(z.tarif.is_none() && !z.notlar.is_empty());
    let z = kural_coz("<script>alert(1)</script> \"A B C\"", &k, &[]);
    assert!(z.tarif.unwrap().sahneler[0].metin.iter().all(|s| !s.contains('<')));
}
#[test]
fn harf_hatasi_toleransi() {
    let k = kat();
    for tarif in ["\"A\" ZIPLA", "\"A\" zipla", "\"A\" ziplayarak gelsin", "\"A\" zıplatarak", "\"A\" Zıplasın"] {
        assert_eq!(kural_coz(tarif, &k, &[]).tarif.unwrap().sahneler[0].giris, "zipla", "{tarif}");
    }
}
#[test]
fn mesafe_ve_en_yakin() {
    assert_eq!(mesafe("kitten", "sitting"), 3);
    let l = vec!["zipla".to_string(), "konfeti".to_string()];
    assert_eq!(en_yakin("Zıpla", &l).as_deref(), Some("zipla"));
    assert_eq!(en_yakin("konfetti", &l).as_deref(), Some("konfeti"));
    assert_eq!(en_yakin("tamamen-alakasiz", &l), None);
}
#[test]
fn otomatik_kurallar_katalogu_kapsar() {
    let k = kat();
    let oto = otomatik_kurallar(&k, gomulu_kurallar());
    for r in &oto { kural_gecerli(r, &k).unwrap_or_else(|e| panic!("{e}")); }
    assert!(kural_ifade_sayisi(&oto) > 400, "otomatik ifade sayisi: {}", kural_ifade_sayisi(&oto));
    let z = kural_coz("\"A\" tek tek sol gelsin", &k, &[]);
    assert_eq!(z.tarif.unwrap().sahneler[0].giris, "tek-tek-sol");
    let z = kural_coz("\"B\" nabiz cikis", &k, &[]);
    assert_eq!(z.tarif.unwrap().sahneler[0].cikis, "nabiz");
}

fn json_yaz(j: &Json) -> String {
    match j {
        Json::Bos => "null".into(), Json::Mantik(b) => b.to_string(), Json::Sayi(n) => n.to_string(),
        Json::Yazi(s) => format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\"")),
        Json::Dizi(v) => format!("[{}]", v.iter().map(json_yaz).collect::<Vec<_>>().join(",")),
        Json::Nesne(v) => format!("{{{}}}", v.iter().map(|(k, x)| format!("\"{k}\":{}", json_yaz(x))).collect::<Vec<_>>().join(",")),
    }
}
#[test]
fn hazir_senaryolar_gecerli_ve_okunur() {
    let k = kat();
    let j = json_coz(HAZIR_SENARYOLAR).expect("hazir-senaryolar.json okunamadi");
    let Some(Json::Dizi(l)) = j.al("senaryolar") else { panic!("senaryolar yok") };
    assert!(l.len() >= 12, "en az 12 hazir senaryo olmali: {}", l.len());
    let mut idler: Vec<&str> = vec![];
    for e in l {
        let id = e.al("id").and_then(|x| x.yazi()).expect("id");
        assert!(!idler.contains(&id), "tekrar eden id: {id}");
        idler.push(id);
        let ham = json_yaz(e.al("tarif").expect("tarif"));
        let d = dogrula(&ham, &k).unwrap_or_else(|h| panic!("{id}: {h:?}"));
        assert!(d.notlar.is_empty(), "{id}: dogrulayici degisiklik yapti: {:?}", d.notlar);
        assert!((3..=6).contains(&d.tarif.sahneler.len()), "{id}: sahne sayisi");
        for s in &d.tarif.sahneler {
            assert!(s.metin.len() <= 3, "{id}: sahne basina en cok 3 satir");
            for m in &s.metin { assert!(m.chars().count() <= 7, "{id}: satir 7 harften uzun: {m}"); }
        }
        let sn = senaryo_metni(&d.tarif);
        crate::senaryo::ayristir(&sn).unwrap_or_else(|e| panic!("{id}: senaryo hatasi {e}"));
        for satir in sn.lines().filter(|x| x.starts_with("sahne")) {
            let harf: f32 = satir.split("harf=").nth(1).and_then(|x| x.split_whitespace().next()).and_then(|x| x.parse().ok()).expect("harf");
            assert!(harf >= 148.0, "{id}: 3 m okunurluk (harf >= 148 px) saglanmadi: {harf}");
        }
        assert!(toplam_sure(&d.tarif) <= 40.0, "{id}: kisa kalmali");
    }
}
