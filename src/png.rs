//! Bağımlılıksız PNG yazıcı (sıkıştırmasız deflate): arayüz önizlemesi için.
fn crc32(veri: &[u8]) -> u32 {
    let mut c = !0u32;
    for &b in veri { c ^= b as u32; for _ in 0..8 { c = if c & 1 != 0 { 0xEDB8_8320 ^ (c >> 1) } else { c >> 1 }; } }
    !c
}
fn adler(veri: &[u8]) -> u32 {
    let (mut a, mut b) = (1u32, 0u32);
    for &x in veri { a = (a + x as u32) % 65521; b = (b + a) % 65521; }
    (b << 16) | a
}
fn parca(out: &mut Vec<u8>, tur: &[u8; 4], veri: &[u8]) {
    out.extend((veri.len() as u32).to_be_bytes());
    let mut t = tur.to_vec(); t.extend(veri);
    out.extend(&t);
    out.extend(crc32(&t).to_be_bytes());
}
/// RGB24 (w*h*3) → PNG baytları.
pub fn rgb_png(rgb: &[u8], w: u32, h: u32) -> Vec<u8> {
    let mut ham = Vec::with_capacity((w * 3 + 1) as usize * h as usize);
    for y in 0..h as usize { ham.push(0); ham.extend(&rgb[y * w as usize * 3..(y + 1) * w as usize * 3]); }
    let mut z = vec![0x78, 0x01];
    let mut parcalar = ham.chunks(65535).peekable();
    while let Some(p) = parcalar.next() {
        z.push(if parcalar.peek().is_none() { 1 } else { 0 });
        z.extend((p.len() as u16).to_le_bytes());
        z.extend((!(p.len() as u16)).to_le_bytes());
        z.extend(p);
    }
    z.extend(adler(&ham).to_be_bytes());
    let mut out = vec![0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A];
    let mut ihdr = vec![];
    ihdr.extend(w.to_be_bytes()); ihdr.extend(h.to_be_bytes()); ihdr.extend([8, 2, 0, 0, 0]);
    parca(&mut out, b"IHDR", &ihdr);
    parca(&mut out, b"IDAT", &z);
    parca(&mut out, b"IEND", &[]);
    out
}
/// Tamsayı kat ile kutu süzgeciyle küçültür (önizleme için).
pub fn kucult(rgb: &[u8], w: u32, h: u32, k: u32) -> (Vec<u8>, u32, u32) {
    if k <= 1 { return (rgb.to_vec(), w, h); }
    let (nw, nh) = (w / k, h / k);
    let mut out = vec![0u8; (nw * nh * 3) as usize];
    for y in 0..nh { for x in 0..nw { for c in 0..3 {
        let mut t = 0u32;
        for dy in 0..k { for dx in 0..k { t += rgb[(((y * k + dy) * w + x * k + dx) * 3 + c) as usize] as u32; } }
        out[((y * nw + x) * 3 + c) as usize] = (t / (k * k)) as u8;
    } } }
    (out, nw, nh)
}
#[cfg(test)]
mod t {
    use super::*;
    #[test]
    fn imza_ve_boyut() {
        let p = rgb_png(&vec![200u8; 4 * 3 * 3], 4, 3);
        assert_eq!(&p[..8], &[0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A]);
        assert_eq!(&p[12..16], b"IHDR");
        assert_eq!(u32::from_be_bytes(p[16..20].try_into().unwrap()), 4);
    }
    #[test]
    fn crc_bilinen_deger() { assert_eq!(crc32(b"IEND"), 0xAE42_6082); }
    #[test]
    fn kucult_ortalama() { let (o, w, h) = kucult(&[0, 0, 0, 100, 100, 100, 0, 0, 0, 100, 100, 100], 2, 2, 2); assert_eq!((w, h, o[0]), (1, 1, 50)); }
}
