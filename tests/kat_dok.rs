// geçici: katalog adlarını yazdırır
use taa_vidyo_yapici::*;
#[test]
fn dok() {
    animasyon::yukle();
    println!("GIRIS: {}", senaryo::giris_adlari().join(" "));
    println!("CIKIS: {}", senaryo::cikis_adlari().join(" "));
    println!("EFEKT: {}", senaryo::efekt_listesi().iter().map(|x| x.0.clone()).collect::<Vec<_>>().join(" "));
    println!("PALET: {}", stil::palet_listesi().iter().map(|x| x.0.clone()).collect::<Vec<_>>().join(" "));
    println!("FONT: {}", stil::FONTLAR.iter().map(|x| x.0).collect::<Vec<_>>().join(" "));
    println!("GECIS: {}", senaryo::GECISLER.iter().map(|x| x.0).collect::<Vec<_>>().join(" "));
    println!("STIL: {}", oto::STILLER.iter().map(|x| x.0).collect::<Vec<_>>().join(" "));
    println!("EGRI: {}", anim::EGRILER.iter().map(|x| x.0).collect::<Vec<_>>().join(" "));
    println!("SABLON: {}", sablon::SABLONLAR.iter().map(|x| x.0).collect::<Vec<_>>().join(" "));
}
