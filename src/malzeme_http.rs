//! Malzeme (LED içerik kitaplığı) HTTP uçları. Sahip: Yusuf. Yazan: [Claude#Sonnet-5.5, 2026-11-18]. Başlangıç iskeleti; asıl içerik malzeme ajanı tarafından doldurulur.
/// `/malzeme/...` isteklerini işler. Dönüş: (durum kodu, içerik türü, gövde).
pub fn isle(_yontem: &str, _yol: &str, _govde: &str) -> (u16, &'static str, String) {
    (404, "text/plain; charset=utf-8", "malzeme uçları henüz yok".to_string())
}
