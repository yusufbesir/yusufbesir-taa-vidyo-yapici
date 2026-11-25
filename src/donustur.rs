//! Dönüştürücü: videoyu ya da resmi çizgi film, çizgi roman (halftone), boyama, piksel art, duotone,
//! karakalem, kontur ve retro 8-bit görünüme çevirir; LED için 950x1800 dikey çıkış (kırp/sığdır/doldur).
//! ffmpeg HER ZAMAN argüman dizisiyle çağrılır (kabuk yok); filtre zincirine yalnız doğrulanmış sayılar ve
//! sabit listeden adlar girer. Kaynak yalnız izinli köklerden (çıktı klasörü ve kullanıcı klasörleri),
//! yalnız video/resim uzantısıyla ve boyut sınırıyla kabul edilir. Ayrıntı: docs/DONUSTURUCU.md.
//! Sahip Yusuf; yazan [Claude#Dönüştürücü Opus 5.5, yenileme-1.0, 2026-11-25]; yalnız yazan yapay zekâ ya da Yusuf'un izniyle değişir; silme yok.
//! Dalga 4 [Claude#SEC-FIX Opus 5.5, yenileme-1.0, 2026-11-25]: girişte `-protocol_whitelist file,pipe` ve uzantıdan zorlanan
//! demuxer (`-f mov|matroska|avi|gif|image2`; içerik koklama yok, m3u8 içerikli .mp4 açılmaz); önizleme aynı anda tek ve 25 sn
//! zaman aşımlı (süreç öldürülür); eşzamanlı dönüştürme arka plan işinin iptal/süreç/durum kaydına dokunmaz (test yalıtımı);
//! kaynak listesinde uzak masaüstü alt klasörleri dışarıda, çıktı klasörü en üstte.
use std::io::{BufRead, BufReader, Read};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::{Duration, Instant};

pub const W: u32 = 950;
pub const H: u32 = 1800;
pub const KUCUK_W: u32 = 190;
pub const KUCUK_H: u32 = 360;
pub const VIDEO_MAX: u64 = 4 * 1024 * 1024 * 1024;
pub const RESIM_MAX: u64 = 50 * 1024 * 1024;
pub const SURE_MAX: f32 = 600.0;
pub const VIDEO_UZANTI: &[&str] = &["mp4", "mov", "mkv", "webm", "avi", "m4v", "gif"];
pub const RESIM_UZANTI: &[&str] = &["png", "jpg", "jpeg", "bmp", "webp"];
/// Piksel blok boyutları: 950 ve 1800'ü tam böler (kare pikseller eşit kalır).
pub const BLOKLAR: &[u32] = &[2, 5, 10, 25, 50];

pub struct Tur { pub id: &'static str, pub ad: &'static str, pub aciklama: &'static str }
pub const TURLER: &[Tur] = &[
    Tur { id: "cizgifilm", ad: "Çizgi film", aciklama: "Kalın kontur, az renk, yumuşak düz yüzeyler" },
    Tur { id: "cizgiroman", ad: "Çizgi roman", aciklama: "Mürekkep çizgisi ve halftone nokta gölgesi" },
    Tur { id: "boyama", ad: "Boyama", aciklama: "Fırça darbesi gibi yağlı boya görünümü" },
    Tur { id: "piksel", ad: "Piksel art", aciklama: "Küçült, paleti kısıtla, keskin büyüt" },
    Tur { id: "retro8bit", ad: "Retro 8-bit", aciklama: "İri piksel, 16 renk, ızgara ve düşük kare hızı" },
    Tur { id: "duotone", ad: "Duotone", aciklama: "İki premium tonla afiş görünümü" },
    Tur { id: "karakalem", ad: "Karakalem", aciklama: "Kurşun kalem eskiz" },
    Tur { id: "kontur", ad: "Kontur", aciklama: "Koyu zeminde ince ton çizgileri" },
    Tur { id: "piksel-giris", ad: "Piksel giriş", aciklama: "İri pikselden netleşerek açılan giriş animasyonu (resim ya da video)" },
];

/// Piksel/retro sabit paletleri (neon yok; yumuşak, LED'de göz yormayan tonlar).
pub const PALETLER: &[(&str, &str, &[u32])] = &[
    ("otomatik", "Otomatik (kaynaktan)", &[]),
    ("retro16", "Retro 16", &[0x000000, 0x1d2b53, 0x7e2553, 0x008751, 0xab5236, 0x5f574f, 0xc2c3c7, 0xfff1e8, 0xb5403a, 0xd98a2b, 0xe8d28a, 0x4f8a5b, 0x4a7fb0, 0x83769c, 0xc98a9b, 0xf1c9a5]),
    ("gameboy", "Game Boy yeşili", &[0x0f380f, 0x306230, 0x8bac0f, 0x9bbc0f]),
    ("sepya", "Sepya", &[0x2b1d0e, 0x5a3d1e, 0x8c6239, 0xb98b5a, 0xdcc09a, 0xf3e3c3]),
    ("kehribar", "Kehribar", &[0x1a0f00, 0x4d2e00, 0x8a5300, 0xc27c0e, 0xe8a838, 0xf7d58b]),
    ("lacivert-altin", "Lacivert ve altın", &[0x0b1426, 0x1b2a41, 0x3a4a63, 0x8a7a4a, 0xc9a74a, 0xe9d79b]),
    ("gri", "Gri tonlar", &[0x000000, 0x333333, 0x666666, 0x999999, 0xcccccc, 0xffffff]),
];

/// Duotone ve kontur tonları: (ad, etiket, koyu, açık).
pub const TONLAR: &[(&str, &str, u32, u32)] = &[
    ("lacivert-altin", "Lacivert ve altın", 0x111c33, 0xd4af37),
    ("bordo-krem", "Bordo ve krem", 0x3d0c1a, 0xf3e5c8),
    ("petrol-seftali", "Petrol ve şeftali", 0x0b3c49, 0xf2c6a0),
    ("grafit-kehribar", "Grafit ve kehribar", 0x1c1c1c, 0xe0a84a),
    ("orman-krem", "Orman ve krem", 0x16302a, 0xe9e4d4),
];

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Sigdir { Kirp, Sigdir, Doldur }

#[derive(Clone, Debug, PartialEq)]
pub struct Ayar {
    pub tur: String,
    pub sigdir: Sigdir,
    /// 0 = kaynağın tamamı (resimde 5 sn).
    pub sure: f32,
    pub baslangic: f32,
    pub fps: u32,
    /// 0..100
    pub yogunluk: u32,
    pub palet: String,
    /// Piksel paleti renk sayısı (otomatik palette), 4..64.
    pub renk: u32,
    /// "yok" | "bayer" | "floyd"
    pub dither: String,
    pub blok: u32,
}

impl Default for Ayar {
    fn default() -> Self {
        Ayar { tur: "cizgifilm".into(), sigdir: Sigdir::Kirp, sure: 0.0, baslangic: 0.0, fps: 25, yogunluk: 50, palet: String::new(), renk: 24, dither: "yok".into(), blok: 10 }
    }
}

fn sayi(v: &str, ad: &str, min: f32, max: f32) -> Result<f32, String> {
    let x: f32 = v.trim().parse().map_err(|_| format!("{ad} sayı olmalı"))?;
    if !x.is_finite() || x < min || x > max { return Err(format!("{ad} {min} ile {max} arasında olmalı")); }
    Ok(x)
}

/// JSON alanlarından ayar çözer ve doğrular (bilinmeyen alanlar yok sayılır).
pub fn ayar_coz(alanlar: &[(String, String)]) -> Result<Ayar, String> {
    let mut a = Ayar::default();
    let al = |k: &str| alanlar.iter().find(|(a, _)| a == k).map(|(_, v)| v.as_str());
    match al("tur") { Some(t) if TURLER.iter().any(|x| x.id == t) => a.tur = t.into(), Some(_) => return Err("bilinmeyen dönüştürme türü".into()), None => return Err("tür seçilmedi".into()) }
    if let Some(v) = al("sigdir") { a.sigdir = match v { "kirp" => Sigdir::Kirp, "sigdir" => Sigdir::Sigdir, "doldur" => Sigdir::Doldur, _ => return Err("sığdırma kirp, sigdir ya da doldur olmalı".into()) }; }
    if let Some(v) = al("sure") { a.sure = sayi(v, "süre", 0.0, SURE_MAX)?; }
    if let Some(v) = al("baslangic") { a.baslangic = sayi(v, "başlangıç", 0.0, 36000.0)?; }
    if let Some(v) = al("fps") { a.fps = sayi(v, "kare hızı", 1.0, 60.0)?.round() as u32; }
    if let Some(v) = al("yogunluk") { a.yogunluk = sayi(v, "yoğunluk", 0.0, 100.0)?.round() as u32; }
    if let Some(v) = al("renk") { a.renk = sayi(v, "renk sayısı", 4.0, 64.0)?.round() as u32; }
    if let Some(v) = al("dither") { if !["yok", "bayer", "floyd"].contains(&v) { return Err("dithering yok, bayer ya da floyd olmalı".into()); } a.dither = v.into(); }
    if let Some(v) = al("blok") { let b = sayi(v, "piksel boyutu", 2.0, 50.0)? as u32; if !BLOKLAR.contains(&b) { return Err("piksel boyutu 2, 5, 10, 25 ya da 50 olmalı".into()); } a.blok = b; }
    let palet = al("palet").unwrap_or("");
    match a.tur.as_str() {
        "duotone" | "kontur" => {
            a.palet = if palet.is_empty() { "lacivert-altin".into() } else { palet.into() };
            if !TONLAR.iter().any(|t| t.0 == a.palet) { return Err("bilinmeyen ton".into()); }
        }
        "piksel" | "retro8bit" => {
            a.palet = if palet.is_empty() { if a.tur == "retro8bit" { "retro16".into() } else { "otomatik".into() } } else { palet.into() };
            if !PALETLER.iter().any(|p| p.0 == a.palet) { return Err("bilinmeyen palet".into()); }
            if a.tur == "retro8bit" && al("blok").is_none() { a.blok = 10; }
        }
        _ => a.palet.clear(),
    }
    Ok(a)
}

pub fn palet_renkleri(ad: &str) -> Option<&'static [u32]> { PALETLER.iter().find(|p| p.0 == ad).map(|p| p.2).filter(|r| !r.is_empty()) }

/// paletteuse için 16x16 (256 girdi) palet resmi; renkler döngüyle tekrarlanır.
pub fn palet_png(renkler: &[u32]) -> Vec<u8> {
    let mut rgb = Vec::with_capacity(256 * 3);
    for i in 0..256 { let c = renkler[i % renkler.len()]; rgb.extend([(c >> 16) as u8, (c >> 8) as u8, c as u8]); }
    crate::png::rgb_png(&rgb, 16, 16)
}

/// Palet nereden gelir: ffmpeg'in 2. girdisi ([1:v]) ya da zincir içinde üretilir.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PaletKaynak { Girdi, Icinde }

fn sigdir_zinciri(s: Sigdir) -> String {
    match s {
        Sigdir::Kirp => format!("scale={W}:{H}:force_original_aspect_ratio=increase:flags=lanczos,crop={W}:{H},setsar=1"),
        Sigdir::Sigdir => format!("scale={W}:{H}:force_original_aspect_ratio=decrease:force_divisible_by=2:flags=lanczos,pad={W}:{H}:(ow-iw)/2:(oh-ih)/2:color=black,setsar=1"),
        Sigdir::Doldur => format!("split=2[za][zb];[za]scale={W}:{H}:force_original_aspect_ratio=increase,crop={W}:{H},gblur=sigma=28,eq=brightness=-0.18:saturation=0.8[zbg];[zb]scale={W}:{H}:force_original_aspect_ratio=decrease:force_divisible_by=2:flags=lanczos[zfg];[zbg][zfg]overlay=(W-w)/2:(H-h)/2,setsar=1"),
    }
}

fn posterize(seviye: u32) -> String {
    let q = 256.0 / seviye as f32;
    let e = format!("trunc(val/{q:.4})*{q:.4}+{:.4}", q / 2.0);
    format!("lutrgb=r='{e}':g='{e}':b='{e}'")
}

fn hex(c: u32) -> (u32, u32, u32) { ((c >> 16) & 255, (c >> 8) & 255, c & 255) }

fn dither_ad(d: &str) -> &'static str { match d { "yok" => "none", "floyd" => "floyd_steinberg", _ => "bayer:bayer_scale=3" } }

/// Tam filtre grafiği: giriş [0:v], çıkış [v]. `kucuk` true ise sonunda 190x360'a küçültür (kart önizlemesi).
pub fn zincir(a: &Ayar, pk: PaletKaynak, kucuk: bool) -> String {
    let y = a.yogunluk as f32 / 100.0;
    let fps = if a.tur == "retro8bit" { a.fps.min(15) } else { a.fps };
    let mut g = format!("[0:v]fps={fps},{}[f];", sigdir_zinciri(a.sigdir));
    let keskin = matches!(a.tur.as_str(), "piksel" | "retro8bit" | "piksel-giris");
    let stil = match a.tur.as_str() {
        "cizgifilm" => {
            let seviye = (10.0 - 6.0 * y).round() as u32; // 10..4 renk seviyesi
            let kalin = if y >= 0.5 { ",erosion" } else { "" };
            format!("[f]bilateral=sigmaS={:.1}:sigmaR=0.12,split=2[c0][e0];[c0]{},eq=saturation={:.2},format=gbrp[c1];[e0]edgedetect=low=0.08:high={:.3},negate{kalin},format=gbrp[e1];[c1][e1]blend=all_mode=multiply,format=yuv420p[v]",
                4.0 + 12.0 * y, posterize(seviye), 1.1 + 0.3 * y, 0.30 - 0.15 * y)
        }
        "cizgiroman" => {
            let s = (8.0 + 8.0 * y).round(); // halftone hücresi (px)
            let ht = format!("255*gt(hypot(mod(X\\,{s})-{h}\\,mod(Y\\,{s})-{h})\\,{r:.2}*sqrt(1-lum(X\\,Y)/255))", h = s / 2.0, r = s * 0.72);
            format!("[f]split=3[c0][h0][e0];[c0]{},eq=saturation=1.35:contrast=1.1,format=gbrp[c1];[h0]format=gray,geq=lum='{ht}',format=gbrp[h1];[e0]edgedetect=low=0.1:high=0.25,negate,erosion,format=gbrp[e1];[c1][h1]blend=all_mode=multiply[ch];[ch][e1]blend=all_mode=multiply,format=yuv420p[v]",
                posterize(4))
        }
        "boyama" => format!("[f]median=radius={},smartblur=lr=2:ls=0.6,unsharp=5:5:0.9,eq=saturation={:.2}:contrast=1.06,format=yuv420p[v]", 2 + (5.0 * y).round() as u32, 1.15 + 0.25 * y),
        "piksel" | "retro8bit" => {
            let (pw, ph) = (W / a.blok, H / a.blok);
            let kucult = format!("[f]scale={pw}:{ph}:flags=area");
            let kullan = format!("paletteuse=dither={}", dither_ad(&a.dither));
            let izgara = if a.tur == "retro8bit" && a.blok >= 5 { format!(",drawgrid=w={b}:h={b}:t=1:color=black@0.35", b = a.blok) } else { String::new() };
            let buyut = format!("scale={W}:{H}:flags=neighbor{izgara},format=yuv420p[v]");
            match pk {
                PaletKaynak::Girdi => format!("{kucult}[k];[k][1:v]{kullan},{buyut}"),
                PaletKaynak::Icinde => format!("{kucult},split=2[k0][k1];[k0]palettegen=max_colors={}:reserve_transparent=0:stats_mode=full[p];[k1][p]{kullan},{buyut}", a.renk),
            }
        }
        "duotone" => {
            let t = TONLAR.iter().find(|t| t.0 == a.palet).unwrap_or(&TONLAR[0]);
            let ((kr, kg, kb), (ar, ag, ab)) = (hex(t.2), hex(t.3));
            let e = |k: u32, a: u32| format!("{k}+({}*val/255)", a as i32 - k as i32);
            format!("[f]eq=contrast={:.2},format=gray,format=gbrp,lutrgb=r='{}':g='{}':b='{}',format=yuv420p[v]", 1.0 + 0.5 * y, e(kr, ar), e(kg, ag), e(kb, ab))
        }
        // Kâğıt beyazı kısık sıcak tona çekilir (LED'de tam beyaz ekran göz kamaştırır).
        "karakalem" => format!("[f]format=gray,split=2[a0][b0];[b0]negate,gblur=sigma={:.1}[b1];[a0][b1]blend=all_mode=dodge,eq=contrast=1.15,format=gbrp,lutrgb=r='val*232/255':g='val*224/255':b='val*208/255',format=yuv420p[v]", 3.0 + 10.0 * y),
        // Blok boyutu 50 px'ten 1 px'e iner; süre `sure` alanından (video işinde etkin süreyle doldurulur), animasyon sürenin %80'inde biter.
        "piksel-giris" => {
            let d = (if a.sure > 0.0 { a.sure } else { 5.0 } * 0.8).max(0.5);
            let b = format!("max(1,{:.0}*pow(1-min(1,t/{d:.3}),2))", 50.0 - 30.0 * (1.0 - y));
            format!("[f]scale=w='max(1,trunc({W}/{b}))':h='max(1,trunc({H}/{b}))':eval=frame:flags=area,scale={W}:{H}:flags=neighbor,setsar=1,format=yuv420p[v]")
        }
        _ /* kontur */ => {
            let t = TONLAR.iter().find(|t| t.0 == a.palet).unwrap_or(&TONLAR[0]);
            let (r, gg, b) = hex(t.3);
            let kalin = if y >= 0.5 { ",dilation" } else { "" };
            format!("[f]edgedetect=low=0.05:high={:.3}{kalin},format=gbrp,lutrgb=r='val*{r}/255':g='val*{gg}/255':b='val*{b}/255',format=yuv420p[v]", 0.25 - 0.15 * y)
        }
    };
    g.push_str(&stil);
    if kucuk {
        g.truncate(g.len() - 3); // sondaki "[v]"
        g.push_str(&format!(",scale={KUCUK_W}:{KUCUK_H}:flags={}[v]", if keskin { "neighbor" } else { "area" }));
    }
    g
}

/// Windows canonicalize çıktısındaki \\?\ önekini kaldırır (ffmpeg ve gezgin için).
fn sade(p: PathBuf) -> PathBuf {
    let s = p.to_string_lossy().to_string();
    match s.strip_prefix(r"\\?\") { Some(r) if r.as_bytes().get(1) == Some(&b':') => PathBuf::from(r), _ => p }
}

pub fn uzanti(p: &Path) -> String { p.extension().map(|e| e.to_string_lossy().to_lowercase()).unwrap_or_default() }
pub fn resim_mi(p: &Path) -> bool { RESIM_UZANTI.contains(&uzanti(p).as_str()) }

/// İzinli kaynak kökleri: program çıktı klasörü + kullanıcının Videolar, Resimler, Masaüstü, İndirilenler klasörleri.
pub fn kokler(cikti: &Path) -> Vec<PathBuf> {
    let mut v = vec![cikti.to_path_buf()];
    if let Some(ev) = std::env::var_os("USERPROFILE").or_else(|| std::env::var_os("HOME")) {
        let ev = PathBuf::from(ev);
        for k in ["Videos", "Pictures", "Desktop", "Downloads"] { v.push(ev.join(k)); }
    }
    v.into_iter().filter_map(|k| k.canonicalize().ok().map(sade)).collect()
}

/// Kaynak yolunu doğrular: mutlak, izinli kök altında, video/resim uzantılı, sınır altında dosya.
pub fn kaynak_dogrula(yol: &str, kokler: &[PathBuf]) -> Result<PathBuf, String> {
    if yol.is_empty() { return Err("kaynak seçilmedi".into()); }
    if yol.contains('\0') { return Err("geçersiz yol".into()); }
    let p = PathBuf::from(yol);
    if !p.is_absolute() { return Err("kaynak tam yol olmalı".into()); }
    let ext = uzanti(&p);
    if !VIDEO_UZANTI.contains(&ext.as_str()) && !RESIM_UZANTI.contains(&ext.as_str()) { return Err("yalnız video ya da resim dosyası seçilebilir".into()); }
    let t = p.canonicalize().map(sade).map_err(|_| "kaynak dosya bulunamadı".to_string())?;
    if !kokler.iter().any(|k| t.starts_with(k)) { return Err("bu klasörden dosya seçilemez (yalnız çıktı klasörü, Videolar, Resimler, Masaüstü, İndirilenler)".into()); }
    let m = std::fs::metadata(&t).map_err(|_| "kaynak okunamadı".to_string())?;
    if !m.is_file() { return Err("kaynak bir dosya olmalı".into()); }
    let sinir = if resim_mi(&t) { RESIM_MAX } else { VIDEO_MAX };
    if m.len() > sinir { return Err(format!("dosya çok büyük (en çok {} MB)", sinir / 1024 / 1024)); }
    Ok(t)
}

/// Varsayılan olarak taranmayan alt klasörler (uzak masaüstü ve ekran kaydı araçlarının kendi kayıtları; küçük harf).
pub const DISLANAN_ALT: &[&str] = &["rustdesk", "anydesk", "teamviewer", "parsec", "chrome remote desktop", "remote desktop", "uzak masaüstü", "splashtop", "nomachine"];

/// Alt klasör adı dışlanacak mı (yalnız kök altındaki klasörlerde uygulanır; kökün kendisi hep taranır).
pub fn alt_dislanir(ad: &str) -> bool {
    let a = ad.trim().to_lowercase();
    DISLANAN_ALT.iter().any(|d| a == *d || a.starts_with(&format!("{d} ")) || a.starts_with(&format!("{d}-")) || a.starts_with(&format!("{d}_")))
}

/// Köklerdeki (en çok bir alt klasör derinliği) video/resim dosyaları; en çok 300. İlk kök (program çıktı klasörü)
/// dosyaları en üstte, sonra yeniden eskiye. Uzak masaüstü alt klasörleri (RustDesk vb.) taranmaz.
pub fn kaynak_listesi(kokler: &[PathBuf]) -> Vec<(PathBuf, u64)> {
    let mut v: Vec<(PathBuf, u64, std::time::SystemTime)> = vec![];
    fn tara(k: &Path, derinlik: u32, v: &mut Vec<(PathBuf, u64, std::time::SystemTime)>) {
        let Ok(rd) = std::fs::read_dir(k) else { return };
        for e in rd.flatten().take(2000) {
            let p = e.path();
            let Ok(m) = e.metadata() else { continue };
            if m.is_dir() {
                if derinlik > 0 && !alt_dislanir(&e.file_name().to_string_lossy()) { tara(&p, derinlik - 1, v); }
                continue;
            }
            let ext = uzanti(&p);
            let sinir = if RESIM_UZANTI.contains(&ext.as_str()) { RESIM_MAX } else if VIDEO_UZANTI.contains(&ext.as_str()) { VIDEO_MAX } else { continue };
            if m.len() == 0 || m.len() > sinir { continue; }
            v.push((p, m.len(), m.modified().unwrap_or(std::time::UNIX_EPOCH)));
        }
    }
    for k in kokler { tara(k, 1, &mut v); }
    let oncelik = kokler.first().cloned();
    let ust = |p: &Path| oncelik.as_ref().map(|o| p.parent() == Some(o.as_path())).unwrap_or(false);
    v.sort_by(|a, b| ust(&b.0).cmp(&ust(&a.0)).then(b.2.cmp(&a.2)));
    let mut gorulen = std::collections::HashSet::new();
    v.retain(|x| gorulen.insert(x.0.clone()));
    v.into_iter().take(300).map(|(p, n, _)| (p, n)).collect()
}

fn komut(ff: &Path) -> Command {
    let mut c = Command::new(ff);
    #[cfg(windows)]
    { use std::os::windows::process::CommandExt; c.creation_flags(0x0800_0000); }
    c
}

/// "Duration: 00:00:22.40" satırından süre (sn).
pub fn sure_ayikla(cikti: &str) -> Option<f32> {
    let i = cikti.find("Duration: ")? + 10;
    let s = cikti.get(i..i + 11)?;
    let p: Vec<f32> = s.split(':').map(|x| x.trim().parse::<f32>()).collect::<Result<_, _>>().ok()?;
    if p.len() != 3 { return None; }
    Some(p[0] * 3600.0 + p[1] * 60.0 + p[2])
}

pub fn sure_oku(ff: &Path, kaynak: &Path) -> Option<f32> {
    let mut a: Vec<String> = vec!["-hide_banner".into(), "-nostdin".into()];
    a.extend(giris_guvenlik(kaynak));
    a.push("-i".into());
    let o = komut(ff).args(a).arg(kaynak).stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::piped()).output().ok()?;
    sure_ayikla(&String::from_utf8_lossy(&o.stderr))
}

/// Uzantıya göre zorlanan ffmpeg demuxer'ı (içerik koklanmaz: .mp4 içine konmuş m3u8/HLS listesi ya da başka biçim açılmaz).
pub fn zorunlu_bicim(kaynak: &Path) -> Option<&'static str> {
    match uzanti(kaynak).as_str() {
        "mp4" | "mov" | "m4v" => Some("mov"),
        "mkv" | "webm" => Some("matroska"),
        "avi" => Some("avi"),
        "gif" => Some("gif"),
        "png" | "jpg" | "jpeg" | "bmp" | "webp" => Some("image2"),
        _ => None,
    }
}

/// `-i`'den hemen önce gelen güvenlik argümanları: yalnız yerel dosya/boru protokolü ve uzantıdan zorlanan biçim.
/// image2'de dosya adı kalıp olarak yorumlanmaz (`%d`, `*` gibi karakterler başka dosyaları okutmasın).
pub fn giris_guvenlik(kaynak: &Path) -> Vec<String> {
    let mut v: Vec<String> = vec!["-protocol_whitelist".into(), "file,pipe".into()];
    if let Some(f) = zorunlu_bicim(kaynak) {
        v.extend(["-f".into(), f.into()]);
        if f == "image2" { v.extend(["-pattern_type".into(), "none".into()]); }
    }
    v
}

/// Etkin süre: resimde süre (yoksa 5 sn); videoda kalan süre ile istenen sürenin küçüğü.
pub fn etkin_sure(a: &Ayar, resim: bool, kaynak_sure: Option<f32>) -> f32 {
    if resim { return if a.sure > 0.0 { a.sure } else { 5.0 }; }
    let kalan = kaynak_sure.map(|d| (d - a.baslangic).max(0.0)).unwrap_or(SURE_MAX).min(SURE_MAX);
    if a.sure > 0.0 { a.sure.min(kalan) } else { kalan }
}

/// Giriş argümanları (kaynak + isteğe bağlı palet resmi).
fn giris(a: &Ayar, kaynak: &Path, resim: bool, sure: Option<f32>, palet: Option<&Path>) -> Vec<String> {
    let mut v: Vec<String> = vec!["-hide_banner".into(), "-nostdin".into(), "-loglevel".into(), "error".into()];
    if resim {
        if let Some(s) = sure { v.extend(["-loop".into(), "1".into(), "-framerate".into(), a.fps.to_string(), "-t".into(), format!("{s:.3}")]); }
    } else {
        if a.baslangic > 0.0 { v.extend(["-ss".into(), format!("{:.3}", a.baslangic)]); }
        if let Some(s) = sure { v.extend(["-t".into(), format!("{s:.3}")]); }
    }
    v.extend(giris_guvenlik(kaynak));
    v.push("-i".into()); v.push(kaynak.display().to_string());
    if let Some(p) = palet { v.extend(giris_guvenlik(p)); v.push("-i".into()); v.push(p.display().to_string()); }
    v
}

/// Video dönüştürme argümanları (H.264 MP4, sessiz, ilerleme stdout'a).
pub fn video_argumanlari(a: &Ayar, kaynak: &Path, resim: bool, sure: f32, palet: Option<&Path>, cikis: &Path) -> Vec<String> {
    let pk = if palet.is_some() { PaletKaynak::Girdi } else { PaletKaynak::Icinde };
    let mut v = giris(a, kaynak, resim, Some(sure), palet);
    let mut b = a.clone();
    b.sure = sure; // piksel-giris animasyon süresi etkin süreye göre
    v.extend(["-filter_complex".into(), zincir(&b, pk, false), "-map".into(), "[v]".into(), "-an".into(),
        "-c:v".into(), "libx264".into(), "-pix_fmt".into(), "yuv420p".into(), "-crf".into(), "16".into(), "-preset".into(), "medium".into(), "-threads".into(), "6".into(), // iş parçacığı sınırı: bellek dar makinede x264 çökmesin
        "-movflags".into(), "+faststart".into(), "-progress".into(), "pipe:1".into(), "-nostats".into(), "-y".into(), cikis.display().to_string()]);
    v
}

/// Tek kare PNG önizleme argümanları (stdout'a PNG).
pub fn onizleme_argumanlari(a: &Ayar, kaynak: &Path, resim: bool, t: f32, palet: Option<&Path>, kucuk: bool) -> Vec<String> {
    let mut b = a.clone();
    b.baslangic = if resim { 0.0 } else { a.baslangic + t.max(0.0) };
    let pk = if palet.is_some() { PaletKaynak::Girdi } else { PaletKaynak::Icinde };
    let mut v = giris(&b, kaynak, resim, None, palet);
    v.extend(["-filter_complex".into(), zincir(&b, pk, kucuk), "-map".into(), "[v]".into(), "-frames:v".into(), "1".into(), "-f".into(), "image2pipe".into(), "-c:v".into(), "png".into(), "-".into()]);
    v
}

/// Sabit paletin geçici PNG'si (program geçici klasöründe; her çağrıda aynı adla yeniden yazılır).
fn palet_dosyasi(ad: &str) -> Result<Option<PathBuf>, String> {
    let Some(r) = palet_renkleri(ad) else { return Ok(None) };
    let p = std::env::temp_dir().join(format!("taa-donustur-palet-{ad}.png"));
    let b = palet_png(r);
    // İçerik aynıysa yeniden yazma: eşzamanlı iki iş aynı dosyayı yazarken biri yarım dosya okumasın.
    if std::fs::read(&p).ok().as_deref() != Some(b.as_slice()) {
        let g = std::env::temp_dir().join(format!("taa-donustur-palet-{ad}-{}-{}.png", std::process::id(), SAYAC.fetch_add(1, Ordering::Relaxed)));
        std::fs::write(&g, &b).map_err(|e| format!("palet yazılamadı: {e}"))?;
        if std::fs::rename(&g, &p).is_err() { let _ = std::fs::remove_file(&g); if std::fs::read(&p).ok().as_deref() != Some(b.as_slice()) { return Err("palet yazılamadı".into()); } }
    }
    Ok(Some(p))
}
fn palet_gerekli(a: &Ayar) -> bool { matches!(a.tur.as_str(), "piksel" | "retro8bit") }

/// Önizleme ffmpeg süresi sınırı: aşılırsa süreç öldürülür.
pub const ONIZLE_SURE: Duration = Duration::from_secs(25);
/// Önizleme sırası için en uzun bekleme (aynı anda tek önizleme; sonraki istek sıraya girer).
pub const ONIZLE_BEKLE: Duration = Duration::from_secs(30);
pub const MESGUL: &str = "önizleme meşgul: başka bir önizleme sürüyor, biraz sonra yeniden deneyin";
static ONIZLE_KILIT: Mutex<()> = Mutex::new(());
static ONIZLE_ESZ: AtomicUsize = AtomicUsize::new(0);
static ONIZLE_EN_COK: AtomicUsize = AtomicUsize::new(0);

/// Süreç boyunca görülen en yüksek eşzamanlı önizleme ffmpeg sayısı (tek olmalı; test için).
pub fn onizle_en_cok_esz() -> usize { ONIZLE_EN_COK.load(Ordering::SeqCst) }

/// Tek kare PNG üretir. `t`: başlangıçtan itibaren saniye. Aynı anda tek ffmpeg; 25 sn'de bitmezse öldürülür.
pub fn onizle(a: &Ayar, kaynak: &Path, t: f32, kucuk: bool) -> Result<Vec<u8>, String> { onizle_sinirli(a, kaynak, t, kucuk, ONIZLE_SURE) }

/// `onizle` ile aynı; zaman sınırı verilebilir (test).
pub fn onizle_sinirli(a: &Ayar, kaynak: &Path, t: f32, kucuk: bool, sinir: Duration) -> Result<Vec<u8>, String> {
    let ff = crate::ffmpeg::bul().ok_or_else(|| crate::ffmpeg::YOK_MESAJI.to_string())?;
    // Sıra: kilidi en çok ONIZLE_BEKLE bekle (zehirlenmiş kilit de kullanılabilir; içinde paylaşılan veri yok).
    let bas = Instant::now();
    let _kilit = loop {
        match ONIZLE_KILIT.try_lock() {
            Ok(k) => break k,
            Err(std::sync::TryLockError::Poisoned(p)) => break p.into_inner(),
            Err(std::sync::TryLockError::WouldBlock) => {
                if bas.elapsed() > ONIZLE_BEKLE { return Err(MESGUL.into()); }
                std::thread::sleep(Duration::from_millis(25));
            }
        }
    };
    let n = ONIZLE_ESZ.fetch_add(1, Ordering::SeqCst) + 1;
    ONIZLE_EN_COK.fetch_max(n, Ordering::SeqCst);
    let sonuc = onizle_calistir(&ff, a, kaynak, t, kucuk, sinir);
    ONIZLE_ESZ.fetch_sub(1, Ordering::SeqCst);
    sonuc
}

fn onizle_calistir(ff: &Path, a: &Ayar, kaynak: &Path, t: f32, kucuk: bool, sinir: Duration) -> Result<Vec<u8>, String> {
    let palet = if palet_gerekli(a) { palet_dosyasi(&a.palet)? } else { None };
    let mut c = komut(ff).args(onizleme_argumanlari(a, kaynak, resim_mi(kaynak), t, palet.as_deref(), kucuk)).stdin(Stdio::null()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().map_err(|e| format!("ffmpeg başlatılamadı: {e}"))?;
    // Borular ayrı iş parçacıklarında boşaltılır (büyük PNG'de ffmpeg yazarken tıkanmasın).
    let mut out = c.stdout.take();
    let mut err = c.stderr.take();
    let o_ip = std::thread::spawn(move || { let mut b = vec![]; if let Some(o) = out.as_mut() { let _ = o.read_to_end(&mut b); } b });
    let e_ip = std::thread::spawn(move || { let mut s = String::new(); if let Some(e) = err.as_mut() { let _ = e.read_to_string(&mut s); } s });
    let bas = Instant::now();
    let zaman_asimi = loop {
        match c.try_wait() {
            Ok(Some(_)) => break false,
            Ok(None) if bas.elapsed() >= sinir => { let _ = c.kill(); let _ = c.wait(); break true; }
            Ok(None) => std::thread::sleep(Duration::from_millis(20)),
            Err(_) => { let _ = c.kill(); let _ = c.wait(); break false; }
        }
    };
    let stdout = o_ip.join().unwrap_or_default();
    let stderr = e_ip.join().unwrap_or_default();
    if zaman_asimi { return Err(format!("önizleme zaman aşımına uğradı ({} sn); daha kısa ya da küçük bir kaynak deneyin", sinir.as_secs().max(1))); }
    if stdout.starts_with(b"\x89PNG") { return Ok(stdout); }
    Err(format!("önizleme üretilemedi{}", stderr.lines().filter(|l| !l.trim().is_empty()).last().map(|l| format!(": {l}")).unwrap_or_default()))
}

// ---- İlerlemeli iş (tek seferde bir iş) ----

#[derive(Clone, Debug, Default)]
pub struct Durum { pub calisiyor: bool, pub kare: u64, pub toplam: u64, pub asama: String, pub dosya: String, pub mesaj: String, pub hata: bool }

static DURUM: Mutex<Option<Durum>> = Mutex::new(None);
static COCUK: Mutex<Option<Child>> = Mutex::new(None);
static IPTAL: AtomicBool = AtomicBool::new(false);

pub fn durum() -> Durum { DURUM.lock().unwrap().clone().unwrap_or_default() }
fn durum_yaz(f: impl FnOnce(&mut Durum)) { let mut d = DURUM.lock().unwrap(); f(d.get_or_insert_with(Durum::default)); }

pub fn iptal() {
    IPTAL.store(true, Ordering::Relaxed);
    if let Some(c) = COCUK.lock().unwrap().as_mut() { let _ = c.kill(); }
}

/// Güvenli, var olanı ezmeyen çıktı adı: <kaynak adı>-<tür>[-n].mp4
pub fn cikti_yolu(klasor: &Path, kaynak: &Path, tur: &str) -> PathBuf {
    let ham = kaynak.file_stem().map(|s| s.to_string_lossy().to_string()).unwrap_or_default();
    let g: String = ham.chars().map(|c| if c.is_alphanumeric() || c == '-' || c == '_' { c } else { '_' }).take(60).collect();
    let g = g.trim_matches('_');
    let g = if g.is_empty() { "video" } else { g };
    let mut p = klasor.join(format!("{g}-{tur}.mp4"));
    let mut n = 2;
    while p.exists() { p = klasor.join(format!("{g}-{tur}-{n}.mp4")); n += 1; }
    p
}

/// ffmpeg'i çalıştırır. `arka` true ise (arayüzün tek arka plan işi) süreç ortak kayda yazılır, `-progress` satırlarından
/// kare sayısı durum kaydına işlenir ve iptal bayrağına uyulur; false ise (eşzamanlı çağrı: test/örnek üretimi) hiçbir
/// ortak kayda dokunulmaz, böylece aynı anda süren arka plan işinin iptali bu süreci öldürmez (ve tersi).
fn calistir(ff: &Path, args: &[String], ilerleme: bool, arka: bool) -> Result<(), String> {
    let mut c = komut(ff).args(args).stdin(Stdio::null()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().map_err(|e| format!("ffmpeg başlatılamadı: {e}"))?;
    let out = c.stdout.take();
    let mut err = c.stderr.take();
    let mut yerel = None;
    if arka {
        *COCUK.lock().unwrap() = Some(c);
        if IPTAL.load(Ordering::Relaxed) { iptal(); }
    } else { yerel = Some(c); }
    let hata_ip = std::thread::spawn(move || { let mut s = String::new(); if let Some(e) = err.as_mut() { let _ = e.read_to_string(&mut s); } s });
    if let Some(o) = out {
        for satir in BufReader::new(o).lines().map_while(Result::ok) {
            if ilerleme && arka { if let Some(k) = satir.strip_prefix("frame=").and_then(|x| x.trim().parse::<u64>().ok()) { durum_yaz(|d| d.kare = k.min(d.toplam.max(k))); } }
        }
    }
    let durum = if arka { COCUK.lock().unwrap().take().map(|mut c| c.wait()) } else { yerel.take().map(|mut c| c.wait()) };
    let hata = hata_ip.join().unwrap_or_default();
    if arka && IPTAL.load(Ordering::Relaxed) { return Err("iptal edildi".into()); }
    match durum {
        Some(Ok(s)) if s.success() => Ok(()),
        _ => Err(format!("dönüştürme başarısız{}", hata.lines().filter(|l| !l.trim().is_empty()).last().map(|l| format!(": {l}")).unwrap_or_default())),
    }
}

/// İşi arka planda başlatır; çıktı yolunu döner. Aynı anda tek iş.
pub fn baslat(a: Ayar, kaynak: PathBuf, klasor: &Path) -> Result<PathBuf, String> {
    let ff = crate::ffmpeg::bul().ok_or_else(|| crate::ffmpeg::YOK_MESAJI.to_string())?;
    let resim = resim_mi(&kaynak);
    let sure = etkin_sure(&a, resim, if resim { None } else { sure_oku(&ff, &kaynak) });
    if sure <= 0.05 { return Err("başlangıç kaynağın sonundan sonra; daha küçük bir başlangıç seçin".into()); }
    let fps = if a.tur == "retro8bit" { a.fps.min(15) } else { a.fps };
    let cikis = cikti_yolu(klasor, &kaynak, &a.tur);
    {
        let mut d = DURUM.lock().unwrap();
        if d.as_ref().map(|x| x.calisiyor).unwrap_or(false) { return Err("zaten bir dönüştürme sürüyor".into()); }
        *d = Some(Durum { calisiyor: true, kare: 0, toplam: (sure * fps as f32).ceil() as u64, asama: "hazırlanıyor".into(), dosya: cikis.display().to_string(), mesaj: String::new(), hata: false });
    }
    IPTAL.store(false, Ordering::Relaxed);
    let cikis2 = cikis.clone();
    std::thread::spawn(move || {
        let sonuc = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| is_yap(&ff, &a, &kaynak, resim, sure, &cikis2, true))).unwrap_or_else(|_| Err("dönüştürme beklenmedik şekilde durdu".into()));
        durum_yaz(|d| {
            d.calisiyor = false;
            match sonuc { Ok(m) => { d.mesaj = m; d.asama = "bitti".into(); d.kare = d.toplam; } Err(e) => { d.hata = true; d.mesaj = e; d.asama = "durdu".into(); } }
        });
    });
    Ok(cikis)
}

/// Eşzamanlı dönüştürme. Testler ve örnek üretimi için; arayüz `baslat` kullanır. Arka plan işinin iptal bayrağına,
/// süreç kaydına ve durumuna dokunmaz (paralel koşuda iki yol birbirini iptal etmez).
pub fn donustur_esz(ff: &Path, a: &Ayar, kaynak: &Path, resim: bool, sure: f32, cikis: &Path) -> Result<String, String> {
    is_yap(ff, a, kaynak, resim, sure, cikis, false)
}

static SAYAC: AtomicUsize = AtomicUsize::new(0);

fn is_yap(ff: &Path, a: &Ayar, kaynak: &Path, resim: bool, sure: f32, cikis: &Path, arka: bool) -> Result<String, String> {
    let asama = |m: &str| if arka { let m = m.to_string(); durum_yaz(|d| d.asama = m); };
    let gecici_dir = std::env::temp_dir();
    let kimlik = format!("{}-{}-{}", std::process::id(), std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis()).unwrap_or(0), SAYAC.fetch_add(1, Ordering::Relaxed));
    // Palet: sabitse hazır PNG; otomatikse önce kaynaktan 1. geçişle üretilir (bellek dostu).
    let mut palet = if palet_gerekli(a) { palet_dosyasi(&a.palet)? } else { None };
    let mut gecici_palet = None;
    if palet_gerekli(a) && palet.is_none() {
        asama("palet çıkarılıyor");
        let pp = gecici_dir.join(format!("taa-donustur-{kimlik}-palet.png"));
        let fps = if a.tur == "retro8bit" { a.fps.min(15) } else { a.fps };
        let g = format!("[0:v]fps={fps},{}[f];[f]scale={}:{}:flags=area,palettegen=max_colors={}:reserve_transparent=0[v]", sigdir_zinciri(a.sigdir), W / a.blok, H / a.blok, a.renk);
        let mut args = giris(a, kaynak, resim, Some(sure), None);
        args.extend(["-filter_complex".into(), g, "-map".into(), "[v]".into(), "-frames:v".into(), "1".into(), "-y".into(), pp.display().to_string()]);
        calistir(ff, &args, false, arka)?;
        palet = Some(pp.clone());
        gecici_palet = Some(pp);
    }
    asama("dönüştürülüyor");
    let gecici = gecici_dir.join(format!("taa-donustur-{kimlik}.mp4"));
    let r = calistir(ff, &video_argumanlari(a, kaynak, resim, sure, palet.as_deref(), &gecici), true, arka);
    if let Some(p) = gecici_palet { let _ = std::fs::remove_file(p); } // programın kendi geçici dosyası
    if let Err(e) = r { let _ = std::fs::remove_file(&gecici); return Err(e); }
    asama("kaydediliyor");
    if std::fs::rename(&gecici, cikis).is_err() {
        std::fs::copy(&gecici, cikis).map_err(|e| format!("çıktı kaydedilemedi: {e}"))?;
        let _ = std::fs::remove_file(&gecici);
    }
    Ok(format!("Hazır: {}", cikis.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default()))
}

/// Son çıktıyı (yalnız sunucunun kendi kaydından ve çıktı klasöründen) varsayılan oynatıcıda açar.
pub fn son_cikti(klasor: &Path) -> Option<PathBuf> {
    let d = durum();
    if d.calisiyor || d.hata || d.dosya.is_empty() { return None; }
    let p = PathBuf::from(&d.dosya);
    let k = klasor.canonicalize().ok().map(sade)?;
    let t = p.canonicalize().ok().map(sade)?;
    (t.parent() == Some(k.as_path()) && t.is_file()).then_some(t)
}

/// Dosyayı ya da klasörü sistemde açar (kabuk yok: explorer/xdg-open'a tek argüman).
pub fn sistemde_ac(p: &Path) {
    #[cfg(windows)]
    { use std::os::windows::process::CommandExt; let _ = Command::new("explorer").arg(p).creation_flags(0x0800_0000).spawn(); }
    #[cfg(not(windows))]
    { let _ = Command::new("xdg-open").arg(p).spawn(); }
}

#[cfg(test)]
mod t {
    use super::*;
    fn al(v: &[(&str, &str)]) -> Vec<(String, String)> { v.iter().map(|(a, b)| (a.to_string(), b.to_string())).collect() }

    #[test]
    fn ayar_dogrulama() {
        assert!(ayar_coz(&al(&[])).is_err());
        assert!(ayar_coz(&al(&[("tur", "neon")])).is_err());
        let a = ayar_coz(&al(&[("tur", "piksel"), ("blok", "25"), ("renk", "8"), ("dither", "floyd")])).unwrap();
        assert_eq!((a.blok, a.renk, a.dither.as_str(), a.palet.as_str()), (25, 8, "floyd", "otomatik"));
        assert!(ayar_coz(&al(&[("tur", "piksel"), ("blok", "7")])).is_err(), "950/1800'ü bölmeyen blok");
        assert!(ayar_coz(&al(&[("tur", "piksel"), ("palet", "neon")])).is_err());
        assert!(ayar_coz(&al(&[("tur", "duotone"), ("palet", "retro16")])).is_err(), "duotone yalnız ton listesinden");
        assert!(ayar_coz(&al(&[("tur", "cizgifilm"), ("fps", "0")])).is_err());
        assert!(ayar_coz(&al(&[("tur", "cizgifilm"), ("fps", "NaN")])).is_err());
        assert!(ayar_coz(&al(&[("tur", "cizgifilm"), ("sure", "601")])).is_err());
        assert!(ayar_coz(&al(&[("tur", "cizgifilm"), ("yogunluk", "-1")])).is_err());
        assert!(ayar_coz(&al(&[("tur", "cizgifilm"), ("sigdir", "esnet")])).is_err());
        assert!(ayar_coz(&al(&[("tur", "cizgifilm"), ("dither", "x;rm")])).is_err());
        let r = ayar_coz(&al(&[("tur", "retro8bit")])).unwrap();
        assert_eq!((r.palet.as_str(), r.blok), ("retro16", 10));
        assert_eq!(ayar_coz(&al(&[("tur", "duotone")])).unwrap().palet, "lacivert-altin");
    }

    #[test]
    fn zincirler_950x1800_ve_temiz() {
        for t in TURLER {
            for s in [Sigdir::Kirp, Sigdir::Sigdir, Sigdir::Doldur] {
                for y in [0, 50, 100] {
                    let a = ayar_coz(&al(&[("tur", t.id), ("yogunluk", &y.to_string())])).map(|mut a| { a.sigdir = s; a }).unwrap();
                    for pk in [PaletKaynak::Girdi, PaletKaynak::Icinde] {
                        let z = zincir(&a, pk, false);
                        assert!(z.starts_with("[0:v]fps=") && z.ends_with("[v]"), "{}: {z}", t.id);
                        assert!(z.contains(&format!("{W}:{H}")), "{}: çıkış 950x1800 olmalı", t.id);
                        assert!(!z.contains('"') && !z.contains('\n'), "{}", t.id);
                        // tırnaklar dengeli
                        assert_eq!(z.matches('\'').count() % 2, 0, "{}: {z}", t.id);
                        let k = zincir(&a, pk, true);
                        assert!(k.ends_with(&format!("scale={KUCUK_W}:{KUCUK_H}:flags={}[v]", if matches!(t.id, "piksel" | "retro8bit" | "piksel-giris") { "neighbor" } else { "area" })), "{k}");
                    }
                }
            }
        }
        let p = ayar_coz(&al(&[("tur", "piksel"), ("blok", "10")])).unwrap();
        assert!(zincir(&p, PaletKaynak::Girdi, false).contains("scale=95:180:flags=area[k];[k][1:v]paletteuse"));
        assert!(zincir(&p, PaletKaynak::Icinde, false).contains("palettegen=max_colors=24:reserve_transparent=0"));
        let r = ayar_coz(&al(&[("tur", "retro8bit"), ("fps", "30")])).unwrap();
        assert!(zincir(&r, PaletKaynak::Girdi, false).starts_with("[0:v]fps=15,"), "retro kare hızı 15'e iner");
    }

    #[test]
    fn posterize_255_asmaz() {
        for l in 2..=12u32 { let q = 256.0 / l as f32; let v = (255.0 / q).trunc() * q + q / 2.0; assert!(v < 256.0, "{l}"); }
    }

    #[test]
    fn palet_png_256_girdi() {
        let p = palet_png(&[0x112233]);
        assert!(p.starts_with(b"\x89PNG"));
        for (ad, _, r) in PALETLER { if *ad != "otomatik" { assert!(!r.is_empty() && r.len() <= 256, "{ad}"); } }
        assert!(palet_renkleri("otomatik").is_none());
    }

    #[test]
    fn sure_ayiklama() {
        assert_eq!(sure_ayikla("  Duration: 00:00:22.40, start: 0.0"), Some(22.4));
        assert_eq!(sure_ayikla("Duration: 01:02:03.50,"), Some(3723.5));
        assert_eq!(sure_ayikla("Duration: N/A, start"), None);
        assert_eq!(sure_ayikla("yok"), None);
    }

    #[test]
    fn etkin_sure_hesabi() {
        let mut a = Ayar::default();
        assert_eq!(etkin_sure(&a, true, None), 5.0);
        assert_eq!(etkin_sure(&a, false, Some(22.4)), 22.4);
        a.sure = 4.0; assert_eq!(etkin_sure(&a, false, Some(22.4)), 4.0);
        a.baslangic = 20.0; assert!((etkin_sure(&a, false, Some(22.4)) - 2.4).abs() < 1e-4);
        a.baslangic = 30.0; assert_eq!(etkin_sure(&a, false, Some(22.4)), 0.0);
        a.sure = 0.0; a.baslangic = 0.0; assert_eq!(etkin_sure(&a, false, Some(5000.0)), SURE_MAX);
    }

    #[test]
    fn argumanlar_kabuksuz_dizi() {
        let a = ayar_coz(&al(&[("tur", "cizgifilm"), ("baslangic", "2")])).unwrap();
        let k = PathBuf::from(r"C:\a b\x & y.mp4");
        let v = video_argumanlari(&a, &k, false, 3.0, None, Path::new(r"C:\c\o.mp4"));
        let i = v.iter().position(|x| x == "-i").unwrap();
        assert_eq!(v[i + 1], k.display().to_string(), "yol tek argüman, olduğu gibi");
        assert!(v.windows(2).any(|w| w[0] == "-ss" && w[1] == "2.000"));
        assert!(v.windows(2).any(|w| w[0] == "-t" && w[1] == "3.000"));
        assert!(v.contains(&"-an".to_string()) && v.contains(&"libx264".to_string()));
        let r = onizleme_argumanlari(&a, Path::new(r"C:\r.png"), true, 1.0, None, true);
        assert!(!r.contains(&"-loop".to_string()) && r.contains(&"image2pipe".to_string()));
    }

    /// Dalga 4 [Claude#SEC-FIX]: her `-i`'den önce protokol beyaz listesi ve uzantıdan zorlanan biçim.
    #[test]
    fn giriste_protokol_ve_bicim_zorlanir() {
        let a = ayar_coz(&al(&[("tur", "piksel"), ("palet", "retro16")])).unwrap();
        for (k, f) in [(r"C:\v.mp4", "mov"), (r"C:\v.MOV", "mov"), (r"C:\v.m4v", "mov"), (r"C:\v.webm", "matroska"), (r"C:\v.mkv", "matroska"), (r"C:\v.avi", "avi"), (r"C:\v.gif", "gif"), (r"C:\r.jpg", "image2")] {
            let p = Path::new(k);
            let v = video_argumanlari(&a, p, resim_mi(p), 2.0, Some(Path::new(r"C:\p.png")), Path::new(r"C:\o.mp4"));
            let girisler: Vec<usize> = v.iter().enumerate().filter(|(_, x)| *x == "-i").map(|(i, _)| i).collect();
            assert_eq!(girisler.len(), 2, "{k}");
            let i = girisler[0];
            let on = &v[..i];
            let w = on.iter().rposition(|x| x == "-protocol_whitelist").expect("whitelist yok");
            assert_eq!(on[w + 1], "file,pipe", "{k}");
            let fi = on.iter().rposition(|x| x == "-f").expect("-f yok");
            assert_eq!(on[fi + 1], f, "{k}");
            if f == "image2" { assert!(on.windows(2).any(|x| x[0] == "-pattern_type" && x[1] == "none"), "{k}: kalıp kapalı olmalı"); }
            // palet girdisi de aynı korumayla
            let ikinci = &v[i + 2..girisler[1]];
            assert!(ikinci.windows(2).any(|x| x[0] == "-protocol_whitelist" && x[1] == "file,pipe") && ikinci.windows(2).any(|x| x[0] == "-f" && x[1] == "image2"), "{k}: {ikinci:?}");
            let o = onizleme_argumanlari(&a, p, resim_mi(p), 0.0, None, false);
            let oi = o.iter().position(|x| x == "-i").unwrap();
            assert!(o[..oi].windows(2).any(|x| x[0] == "-f" && x[1] == f), "{k} önizleme");
        }
        assert_eq!(zorunlu_bicim(Path::new("x.m3u8")), None);
    }

    #[test]
    fn uzak_masaustu_alt_klasorleri_dislanir() {
        for d in ["RustDesk", "rustdesk", "AnyDesk", "TeamViewer", "RustDesk Kayıtları", "Parsec"] { assert!(alt_dislanir(d), "{d}"); }
        for d in ["Tatil", "TAA Vidyo Yapıcı", "Rusty", "Ekran kayıtları"] { assert!(!alt_dislanir(d), "{d}"); }
        let kok = std::env::temp_dir().join(format!("taa-donustur-liste-{}", std::process::id()));
        let cikti = kok.join("cikti");
        let video = kok.join("Videos");
        for k in [&cikti, &video.join("RustDesk"), &video.join("Tatil")] { std::fs::create_dir_all(k).unwrap(); }
        std::fs::write(cikti.join("eski.mp4"), b"x").unwrap();
        std::thread::sleep(std::time::Duration::from_millis(30));
        std::fs::write(video.join("RustDesk").join("uzak.mp4"), b"x").unwrap();
        std::fs::write(video.join("Tatil").join("yeni.mp4"), b"x").unwrap();
        let kokler = vec![cikti.canonicalize().map(sade).unwrap(), video.canonicalize().map(sade).unwrap()];
        let l: Vec<String> = kaynak_listesi(&kokler).iter().map(|(p, _)| p.file_name().unwrap().to_string_lossy().to_string()).collect();
        assert_eq!(l, vec!["eski.mp4".to_string(), "yeni.mp4".to_string()], "çıktı klasörü en üstte, RustDesk dışarıda");
    }

    #[test]
    fn yol_guvenligi() {
        let kok = std::env::temp_dir().join(format!("taa-donustur-test-{}", std::process::id()));
        let ic = kok.join("izinli");
        std::fs::create_dir_all(&ic).unwrap();
        let v = ic.join("a.mp4"); std::fs::write(&v, b"x").unwrap();
        let txt = ic.join("a.txt"); std::fs::write(&txt, b"x").unwrap();
        let dis = kok.join("dis.mp4"); std::fs::write(&dis, b"x").unwrap();
        let kokler = vec![ic.canonicalize().map(sade).unwrap()];
        assert!(kaynak_dogrula(&v.display().to_string(), &kokler).is_ok());
        assert!(kaynak_dogrula(&txt.display().to_string(), &kokler).is_err(), "uzantı");
        assert!(kaynak_dogrula(&dis.display().to_string(), &kokler).is_err(), "kök dışı");
        assert!(kaynak_dogrula(&ic.join("..").join("dis.mp4").display().to_string(), &kokler).is_err(), "yol geçişi");
        assert!(kaynak_dogrula("a.mp4", &kokler).is_err(), "göreli yol");
        assert!(kaynak_dogrula("", &kokler).is_err());
        assert!(kaynak_dogrula(&ic.join("yok.mp4").display().to_string(), &kokler).is_err());
        assert!(kaynak_dogrula(&format!("{}\0.mp4", v.display()), &kokler).is_err());
        let l = kaynak_listesi(&kokler);
        assert_eq!(l.len(), 1, "yalnız video/resim listelenir");
        // çıktı adı var olanı ezmez
        let c1 = cikti_yolu(&ic, &v, "piksel");
        assert!(c1.ends_with("a-piksel.mp4"));
        std::fs::write(&c1, b"x").unwrap();
        assert!(cikti_yolu(&ic, &v, "piksel").ends_with("a-piksel-2.mp4"));
        assert!(cikti_yolu(&ic, Path::new(r"..\..\k?t*ü.mp4"), "duotone").file_name().unwrap().to_string_lossy().starts_with("k_t_ü-duotone"));
    }
}
