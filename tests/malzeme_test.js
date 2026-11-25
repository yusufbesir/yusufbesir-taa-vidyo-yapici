// Açıklama: web/malzeme.js saf mantık testi (filtre, Türkçe arama, varsayılanlar, sınırlar, istek gövdesi, bekletici, sahte DOM'suz yükleme).
// Çalıştırma: node tests/malzeme_test.js  (Rust testi tests/malzeme_testleri.rs bunu çağırır; node yoksa atlanır)
// Sahip: Yusuf. Yazan: [Claude#Malzeme Opus 5.5, yenileme-1.0, 2026-11-18]. Yalnız yazan yapay zekâ ya da Yusuf'un izniyle değişir; silme yok.
const path = require('path'), assert = require('assert');
const M = require(path.join(__dirname, '..', 'web', 'malzeme.js'));
const I = M._ic;
assert.strictEqual(typeof M.mount, 'function');
assert.strictEqual(typeof M.birakmaHedefiYap, 'function');
assert.strictEqual(globalThis.TaaMalzeme, M, 'window/globalThis.TaaMalzeme atanmalı');

// Türkçe normalleştirme
assert.strictEqual(I.normal('İNDİRİM Şık ÇAĞ ığüşöç'), 'indirim sik cag igusoc');
assert.strictEqual(I.normal('  KALP   atışı '), 'kalp atisi');
assert.strictEqual(I.normal(null), '');

const sema = [
  { ad: 'sure', tur: 'sayi', min: 2, max: 120, varsayilan: 8 },
  { ad: 'renk', tur: 'palet', varsayilan: 'altin' },
  { ad: 'hiz', tur: 'sayi', min: 0.25, max: 4, varsayilan: 1 },
  { ad: 'metin', tur: 'metin', max: 20, varsayilan: 'SEVGİYLE' }
];
const liste = [
  { id: 'kalp-atisi', ad: 'Kalp Atışı', kategori: 'ikon', etiketler: ['kalp', 'sevgi'], aciklama: 'Büyük kalp çift vuruşla atar', sema, senaryo: 'malzeme kalp-atisi sure=8' },
  { id: 'geri-sayim', ad: 'Geri Sayım', kategori: 'zaman', etiketler: ['sayac'], aciklama: 'Büyük rakamlar', sema: sema.concat([{ ad: 'deger', tur: 'sayi', min: 1, max: 999, varsayilan: 10 }]), senaryo: 'malzeme geri-sayim sure=12' },
  { id: 'yutkun-labirent', ad: 'Yutkun Labirentte', kategori: 'oyun', etiketler: ['yutkun', 'oyun'], aciklama: 'Noktaları yutar', sema, senaryo: 'malzeme yutkun-labirent' }
];

// Filtre
assert.deepStrictEqual(I.filtrele(liste, {}).map(m => m.id), ['kalp-atisi', 'geri-sayim', 'yutkun-labirent']);
assert.deepStrictEqual(I.filtrele(liste, { kategori: 'zaman' }).map(m => m.id), ['geri-sayim']);
assert.deepStrictEqual(I.filtrele(liste, { arama: 'ATIŞI' }).map(m => m.id), ['kalp-atisi'], 'Türkçe büyük harf arama');
assert.deepStrictEqual(I.filtrele(liste, { arama: 'sayaç' }).map(m => m.id), ['geri-sayim'], 'şapkasız/ç eşleşmesi');
assert.deepStrictEqual(I.filtrele(liste, { arama: 'yutkun nokta' }).map(m => m.id), ['yutkun-labirent'], 'çok kelime VE');
assert.deepStrictEqual(I.filtrele(liste, { arama: 'yok-boyle' }), []);
assert.deepStrictEqual(I.filtrele(liste, { kategori: 'ikon', arama: 'yutkun' }), []);
assert.deepStrictEqual(I.filtrele(null, {}), []);
assert.deepStrictEqual(I.kategoriSayilari(liste), { ikon: 1, zaman: 1, oyun: 1 });

// Varsayılan ve istek gövdesi
assert.deepStrictEqual(I.varsayilanDegerler(liste[0]), { sure: 8, renk: 'altin', hiz: 1, metin: 'SEVGİYLE' });
assert.deepStrictEqual(I.istekGovdesi(liste[0], {}), { id: 'kalp-atisi', sure: 8, renk: 'altin', hiz: 1, metin: 'SEVGİYLE' });
const g = I.istekGovdesi(liste[1], { sure: 999, hiz: -3, deger: 'abc', metin: 'A"B#C\nD   E çok uzun bir yazı burada', fazla: 1, renk: '' });
assert.strictEqual(g.sure, 120); assert.strictEqual(g.hiz, 0.25); assert.strictEqual(g.deger, 10, 'sayı değilse varsayılan');
assert.strictEqual(g.metin, 'A B C D E çok uzun b'); assert.ok(!('fazla' in g)); assert.ok(!('renk' in g), 'boş palet gönderilmez');
assert.strictEqual(I.sinirla('2.345', { min: 0, max: 10 }), 2.35);
assert.strictEqual(I.sinirla(Infinity, { min: 0, max: 10, varsayilan: 4 }), 4);

// Dalga 3 (Fable KRİTİK): önizleme senaryoyu değiştiren app.onizle'yi kullanmaz; geri al yalnız senaryo değişmediyse
assert.strictEqual(I.onizlemeYolu({ geciciOnizle() {}, onizle() {} }), 'gecici');
assert.strictEqual(I.onizlemeYolu({ onizle() {} }), 'kucuk', 'yalnız onizle varsa senaryoya dokunulmaz, küçük resim');
assert.strictEqual(I.onizlemeYolu(null), 'kucuk');
assert.strictEqual(I.geriAlKarari({ once: 'A', sonra: 'AB' }, 'AB'), 'geri');
assert.strictEqual(I.geriAlKarari({ once: 'A', sonra: 'AB' }, 'ABC'), 'degisti');
assert.strictEqual(I.geriAlKarari(null, 'x'), 'yok');
assert.ok(!/app\.onizle\(/.test(require('fs').readFileSync(path.join(__dirname, '..', 'web', 'malzeme.js'), 'utf8')), 'malzeme.js app.onizle çağırmamalı');

// Bekletici (debounce): yalnız son çağrı çalışır; iptal edilebilir
let say = 0, son = null;
const b = I.bekletici(x => { say++; son = x; }, 20);
b(1); b(2); b(3);
const b2 = I.bekletici(() => { say += 100; }, 10); b2(); b2.iptal();
setTimeout(() => {
  assert.strictEqual(say, 1); assert.strictEqual(son, 3);
  console.log('malzeme_test.js: tamam');
}, 80);
