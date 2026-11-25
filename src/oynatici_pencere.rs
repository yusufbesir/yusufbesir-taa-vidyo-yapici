//! Oynatıcı çıkış penceresi köprüsü: ikinci tao penceresi (kenarlıksız, en üstte, seçilen monitörde, 1:1 piksel) + wry görünümü (/cikis).
//! Sahip: Yusuf. Yazan: [Claude#Oynatici Opus 5.5, yenileme-1.0, 2026-11-25]. Yalnız yazan yapay zekâ ya da Yusuf'un izniyle değişir; silme yok.
//! Karar: tasarim/yenileme/karar-oynatici.md 2.3. Olay döngüsüne bağlama arayuz.rs'tedir (UI ajanı; ISTEK-oynatici.md 4).
//! Köprü kurulmadıysa `gonder` false döner ve uç 503 "pencere yok" yanıtlar. Olay adres TAŞIMAZ: pencere yalnız kendi `/cikis` adresini açar.

/// Köprü olayları. `Monitorler` yanıtı kanal üzerinden JSON dizi olarak döner.
pub enum Olay { Ac { monitor: usize, x: i32, y: i32, gen: u32, yuk: u32 }, Kapat, Monitorler(std::sync::mpsc::Sender<String>) }

#[cfg(windows)]
static PROXY: std::sync::Mutex<Option<tao::event_loop::EventLoopProxy<Olay>>> = std::sync::Mutex::new(None);

/// arayuz.rs `pencere()` içinde `el.create_proxy()` ile çağrılır.
#[cfg(windows)]
pub fn kopru_kur(proxy: tao::event_loop::EventLoopProxy<Olay>) {
    // Karar 1.4 / Ek 1 A6: projede "acilistaCikisAc" açıksa çıkış penceresi program açılışında açılır (olay döngüsü başlayınca işlenir).
    if let Some(o) = crate::oynatici_http::acilis_olayi() { let _ = proxy.send_event(o); }
    *PROXY.lock().unwrap_or_else(|e| e.into_inner()) = Some(proxy);
}

pub fn gonder(o: Olay) -> bool {
    #[cfg(windows)]
    { return PROXY.lock().unwrap_or_else(|e| e.into_inner()).as_ref().map(|p| p.send_event(o).is_ok()).unwrap_or(false); }
    #[cfg(not(windows))]
    { let _ = o; false }
}

/// Monitör listesi (JSON dizi: no, ad, x, y, gen, yuk, olcek). Köprü yoksa None.
pub fn monitorler() -> Option<String> {
    let (tx, rx) = std::sync::mpsc::channel();
    if !gonder(Olay::Monitorler(tx)) { return None; }
    rx.recv_timeout(std::time::Duration::from_secs(2)).ok()
}

/// Açık çıkış penceresi. Alan sırası önemli: görünüm pencereden önce bırakılır.
#[derive(Default)]
pub struct Cikis {
    #[cfg(windows)] gorunum: Option<wry::WebView>,
    #[cfg(windows)] pencere: Option<tao::window::Window>,
}

#[cfg(windows)]
pub fn pencere_mi(c: &Cikis, id: tao::window::WindowId) -> bool { c.pencere.as_ref().map(|p| p.id() == id).unwrap_or(false) }

pub fn kapat(c: &mut Cikis) {
    #[cfg(windows)]
    { c.gorunum = None; c.pencere = None; }
    #[cfg(not(windows))]
    { let _ = c; }
}

/// Olay döngüsünde `Event::UserEvent(o)` için. `kok`: "http://127.0.0.1:<port>" (sonda / yok).
#[cfg(windows)]
pub fn isle(o: Olay, hedef: &tao::event_loop::EventLoopWindowTarget<Olay>, baglam: &mut wry::WebContext, kok: &str, c: &mut Cikis) {
    use tao::dpi::{PhysicalPosition, PhysicalSize};
    match o {
        Olay::Monitorler(tx) => {
            let v: Vec<String> = hedef.available_monitors().enumerate().map(|(i, m)| {
                let (p, s) = (m.position(), m.size());
                format!("{{\"no\":{i},\"ad\":\"{}\",\"x\":{},\"y\":{},\"gen\":{},\"yuk\":{},\"olcek\":{}}}", crate::oynatici_medya::json_kacis(&m.name().unwrap_or_default()), p.x, p.y, s.width, s.height, m.scale_factor())
            }).collect();
            let _ = tx.send(format!("[{}]", v.join(",")));
        }
        Olay::Kapat => kapat(c),
        Olay::Ac { monitor, x, y, gen, yuk } => {
            let m = hedef.available_monitors().nth(monitor).or_else(|| hedef.primary_monitor());
            let (mx, my) = m.map(|m| (m.position().x, m.position().y)).unwrap_or((0, 0));
            let konum = PhysicalPosition::new(mx + x, my + y);
            let boyut = PhysicalSize::new(gen.clamp(64, 8192), yuk.clamp(64, 8192));
            if let Some(p) = &c.pencere { p.set_outer_position(konum); p.set_inner_size(boyut); p.set_focus(); return; }
            // Görev çubuğunda simge yok (karar Ek 1 A2): ikinci simge yanlışlıkla kapatmaya davet eder; kapatma kontrol sayfasından ya da Ctrl+Shift+Q ile.
            use tao::platform::windows::WindowBuilderExtWindows;
            let p = match tao::window::WindowBuilder::new().with_title("TAA Vidyo Yapıcı - Çıkış").with_decorations(false).with_always_on_top(true).with_resizable(false).with_skip_taskbar(true).with_inner_size(boyut).with_position(konum).build(hedef) {
                Ok(p) => p,
                Err(e) => { crate::oynatici_http::gunluk(&format!("çıkış penceresi açılamadı: {e}")); return; }
            };
            let izinli = format!("{kok}/");
            let g = wry::WebViewBuilder::new_with_web_context(baglam).with_url(format!("{kok}/cikis")).with_navigation_handler(move |u| u.starts_with(&izinli)).build(&p);
            match g {
                Ok(g) => { p.set_cursor_visible(false); c.gorunum = Some(g); c.pencere = Some(p); }
                Err(e) => crate::oynatici_http::gunluk(&format!("çıkış görünümü açılamadı: {e}")),
            }
        }
    }
}
