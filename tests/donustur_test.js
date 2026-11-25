// Açıklama: web/donustur.js saf mantık testi (sınırlar, blok yuvarlama, türe göre alanlar, istek gövdesi, boyut yazısı, Türkçe arama, durum yazısı).
// Çalıştırma: node tests/donustur_test.js  (Rust testi tests/donustur_testleri.rs bunu çağırır; node yoksa atlanır)
// Sahip Yusuf; yazan [Claude#Dönüştürücü Opus 5.5, yenileme-1.0, 2026-11-25]; yalnız yazan yapay zekâ ya da Yusuf'un izniyle değişir; silme yok.
const path = require('path'), assert = require('assert');
const D = require(path.join(__dirname, '..', 'web', 'donustur.js'));
const I = D._ic;
assert.strictEqual(typeof D.mount, 'function');
assert.strictEqual(globalThis.TaaDonustur, D);

// sınırlar
assert.strictEqual(I.sinirla('fps', 0), 1);
assert.strictEqual(I.sinirla('fps', 999), 60);
assert.strictEqual(I.sinirla('yogunluk', 'abc'), 0);
assert.strictEqual(I.sinirla('sure', 601), 600);
assert.strictEqual(I.sinirla('renk', 2), 4);
assert.strictEqual(I.sinirla('bilinmeyen', 'x'), 'x');
// blok her zaman 950 ve 1800'ü bölen değere yuvarlanır
[[1, 2], [4, 5], [7, 5], [9, 10], [20, 25], [40, 50], [999, 50], ['x', 10]].forEach(([g, b]) => assert.strictEqual(I.enYakinBlok(g), b, String(g)));
I.alanlar('piksel').forEach(k => assert.ok(typeof k === 'string'));

// türe göre alanlar
assert.ok(I.alanlar('piksel').includes('blok') && I.alanlar('piksel').includes('palet'));
assert.ok(!I.alanlar('cizgifilm').includes('blok'));
assert.ok(I.alanlar('duotone').includes('ton') && !I.alanlar('duotone').includes('palet'));

// istek gövdesi
const g1 = I.istekGovdesi('C:\\v\\a.mp4', Object.assign(I.varsayilanAyar('cizgifilm'), { fps: 500, blok: 7 }));
assert.deepStrictEqual(g1, { kaynak: 'C:\\v\\a.mp4', tur: 'cizgifilm', sigdir: 'kirp', yogunluk: 50, sure: 0, baslangic: 0, fps: 60 });
const g2 = I.istekGovdesi('x.mp4', Object.assign(I.varsayilanAyar('piksel'), { blok: 7 }), { t: 1, kucuk: true });
assert.strictEqual(g2.blok, 5); assert.strictEqual(g2.palet, 'otomatik'); assert.strictEqual(g2.kucuk, true); assert.strictEqual(g2.t, 1);
const g3 = I.istekGovdesi('x.mp4', Object.assign(I.varsayilanAyar('duotone'), { ton: 'bordo-krem' }));
assert.strictEqual(g3.palet, 'bordo-krem'); assert.ok(!('ton' in g3));
assert.strictEqual(I.varsayilanAyar('retro8bit').palet, 'retro16');
assert.strictEqual(I.istekGovdesi(null).kaynak, '');

// biçimler
assert.strictEqual(I.boyutYazi(2048), '2 KB');
assert.strictEqual(I.boyutYazi(5 * 1048576), '5,0 MB');
assert.strictEqual(I.boyutYazi(3 * 1073741824), '3,0 GB');
assert.strictEqual(I.dosyaAdi('C:\\a\\b\\c-piksel.mp4'), 'c-piksel.mp4');
assert.strictEqual(I.dosyaAdi('/a/b.mp4'), 'b.mp4');

// Türkçe arama
const l = [{ ad: 'Yılbaşı.mp4', klasor: 'Videos' }, { ad: 'kar.png', klasor: 'Masaüstü' }];
assert.deepStrictEqual(I.kaynakFiltrele(l, 'YILBAŞI').map(k => k.ad), ['Yılbaşı.mp4']);
assert.deepStrictEqual(I.kaynakFiltrele(l, 'masaustu').map(k => k.ad), ['kar.png']);
assert.strictEqual(I.kaynakFiltrele(l, '').length, 2);
assert.deepStrictEqual(I.kaynakFiltrele(null, 'x'), []);

// durum yazısı
assert.ok(I.durumYazi({ calisiyor: true, asama: 'dönüştürülüyor', yuzde: 42.4, kare: 10, toplam: 24 }).includes('%42'));
assert.ok(I.durumYazi({ calisiyor: false, hata: true, mesaj: 'iptal edildi' }).startsWith('Durdu'));
assert.strictEqual(I.durumYazi(null), '');
console.log('donustur_test.js: tamam');
