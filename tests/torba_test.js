// Açıklama: web/index.html içindeki açılış torbası mantığını (TORBA-BASLA/BITIS arası) Node ile dener. Çalıştırma: node tests/torba_test.js
// Sahip: Yusuf. Yazan: [Claude#Sonnet-5.5, 2026-11-18]. Yalnız yazan yapay zekâ ya da Yusuf'un izniyle değişir; silme yok.
const fs = require('fs'), path = require('path'), assert = require('assert');
const html = fs.readFileSync(path.join(__dirname, '..', 'web', 'index.html'), 'utf8');
const a = html.indexOf('// TORBA-BASLA'), b = html.indexOf('// TORBA-BITIS');
assert(a > 0 && b > a, 'torba bloğu bulunamadı');
const M = new Function(html.slice(a, b) + '\nreturn {ACILIS_BILGI, ACILISLAR, acilisAyarOku, acilisAyarYaz, acilisAcikListe, acilisSec};')();
const depoYap = () => { const m = new Map(); return { m, get: k => (m.has(k) ? m.get(k) : null), set: (k, v) => { m.set(k, v); }, del: k => { m.delete(k); } }; };
const bozuk = { get() { throw new Error('erisim yok'); }, set() { throw new Error('erisim yok'); }, del() { throw new Error('erisim yok'); } };
const yazmaBozuk = Object.assign(depoYap(), { set() { throw new Error('dolu'); } });
let tohum = 12345; const rnd = () => { tohum = (tohum * 1664525 + 1013904223) % 4294967296; return tohum / 4294967296; };
const L = M.ACILISLAR;
assert.strictEqual(L.length, 14); assert.strictEqual(new Set(L).size, 14);

// 1) 14 ardışık açılışta hepsi farklı; her tam tur bir permütasyon; tur geçişinde art arda tekrar yok
{
  const d = depoYap(), ayar = M.acilisAyarOku(d), acik = M.acilisAcikListe(ayar), s = [];
  for (let i = 0; i < 14 * 40; i++) s.push(M.acilisSec(acik, d, rnd));
  for (let t = 0; t < 40; t++) assert.strictEqual(new Set(s.slice(t * 14, t * 14 + 14)).size, 14, 'tur ' + t);
  for (let i = 1; i < s.length; i++) assert.notStrictEqual(s[i], s[i - 1], 'art arda tekrar: ' + i);
}
// 2) uç rastgele değerleri (hep 0 ya da hep ~1) tur geçişinde tekrar üretmez
for (const sabit of [0, 0.999999]) {
  const d = depoYap(), s = [];
  for (let i = 0; i < 14 * 5; i++) s.push(M.acilisSec(L, d, () => sabit));
  for (let i = 1; i < s.length; i++) assert.notStrictEqual(s[i], s[i - 1], 'sabit ' + sabit + ' i=' + i);
  for (let t = 0; t < 5; t++) assert.strictEqual(new Set(s.slice(t * 14, t * 14 + 14)).size, 14);
}
// 3) varsayılan: hepsi açık; bozuk/eksik ayar güvenle hepsi açık
{
  const d = depoYap();
  assert.deepStrictEqual(M.acilisAcikListe(M.acilisAyarOku(d)), L);
  d.set('taa-acilis-ayar', '{bozuk json'); assert.deepStrictEqual(M.acilisAcikListe(M.acilisAyarOku(d)), L);
  d.set('taa-acilis-ayar', '123'); assert.deepStrictEqual(M.acilisAcikListe(M.acilisAyarOku(d)), L);
  d.set('taa-acilis-ayar', JSON.stringify({ kapali: ['yok-boyle', 'led'] })); assert.deepStrictEqual(M.acilisAyarOku(d).kapali, ['led']);
}
// 4) yalnız açık olanlar döner; seçim değişince torba sıfırlanır; tur geçişinde tekrar yok
{
  const d = depoYap(); M.acilisAyarYaz(d, { tam: false, kapali: L.slice(0, 9) }); // 5 açık
  const acik = M.acilisAcikListe(M.acilisAyarOku(d)); assert.strictEqual(acik.length, 5);
  const s = []; for (let i = 0; i < 5 * 30; i++) s.push(M.acilisSec(acik, d, rnd));
  s.forEach(x => assert(acik.includes(x), 'kapalı açılış geldi: ' + x));
  for (let t = 0; t < 30; t++) assert.strictEqual(new Set(s.slice(t * 5, t * 5 + 5)).size, 5);
  for (let i = 1; i < s.length; i++) assert.notStrictEqual(s[i], s[i - 1]);
  M.acilisSec(acik, d, rnd); assert(d.get('taa-acilis-torba'));
  M.acilisAyarYaz(d, { tam: false, kapali: L.slice(0, 12) }); assert.strictEqual(d.get('taa-acilis-torba'), null, 'ayar değişince torba silinmeli');
  const yeni = M.acilisAcikListe(M.acilisAyarOku(d)); assert.strictEqual(yeni.length, 2);
  for (let i = 0; i < 20; i++) assert(yeni.includes(M.acilisSec(yeni, d, rnd)));
}
// 5) liste imzası değişmiş eski torba (ayar silinmeden) güvenle sıfırlanır; kapalı olan asla gelmez
{
  const d = depoYap(); const tam = L.slice(); M.acilisSec(tam, d, rnd);
  const az = L.slice(0, 4); for (let i = 0; i < 40; i++) assert(az.includes(M.acilisSec(az, d, rnd)));
}
// 6) bozuk torba kaydı: sıfırla, çökme yok
for (const kotu of ['{', 'null', '[]', JSON.stringify({ imza: L.join(','), kalan: ['yok'], son: 'x' }), JSON.stringify({ imza: L.join(','), kalan: ['led', 'led'], son: 3 })]) {
  const d = depoYap(); d.set('taa-acilis-torba', kotu); const x = M.acilisSec(L, d, rnd); assert(L.includes(x), 'kötü kayıt: ' + kotu);
}
// 7) hiçbiri açık değil ya da tamamen kapalı: null; tek açık: hep o
{
  const d = depoYap(); M.acilisAyarYaz(d, { tam: true, kapali: [] });
  assert.strictEqual(M.acilisSec(M.acilisAcikListe(M.acilisAyarOku(d)), d, rnd), null);
  M.acilisAyarYaz(d, { tam: false, kapali: L.slice() });
  assert.strictEqual(M.acilisSec(M.acilisAcikListe(M.acilisAyarOku(d)), d, rnd), null);
  M.acilisAyarYaz(d, { tam: false, kapali: L.filter(x => x !== 'perde') });
  const tek = M.acilisAcikListe(M.acilisAyarOku(d)); assert.deepStrictEqual(tek, ['perde']);
  for (let i = 0; i < 10; i++) assert.strictEqual(M.acilisSec(tek, d, rnd), 'perde');
}
// 8) depo erişilemez/yazılamaz: çökmez, açık listeden biri; yazma bozukken tek tek tekrar önlenemese de üye döner
{
  assert.deepStrictEqual(M.acilisAcikListe(M.acilisAyarOku(bozuk)), L);
  for (let i = 0; i < 20; i++) assert(L.includes(M.acilisSec(L, bozuk, rnd)));
  for (let i = 0; i < 20; i++) assert(L.includes(M.acilisSec(L, yazmaBozuk, rnd)));
  assert.strictEqual(M.acilisAyarYaz(bozuk, { tam: false, kapali: [] }), false);
}
console.log('torba_test: tamam');
