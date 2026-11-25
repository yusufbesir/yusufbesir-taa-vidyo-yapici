/* Oynatıcı motoru node testi (web/oynatici-motor.js). tests/oynatici.rs çağırır; node yoksa atlanır.
   Sahip Yusuf; yazan [Claude#Oynatici Opus 5.5, yenileme-1.0, 2026-11-25]; yalnız yazan yapay zekâ ya da Yusuf'un izniyle değişir; silme yok. */
'use strict';
const assert = require('assert');
const path = require('path');
const M = require(path.join(__dirname, '..', 'web', 'oynatici-motor.js'));

let say = 0;
function test(ad, f) { f(); say++; }
function katman(n, ek) { const k = M.varsayilanKatman(1); for (let i = 0; i < n; i++) k.klipler.push({ tur: 'video', ad: 'k' + i + '.mp4' }); return Object.assign(k, ek || {}); }
function sirayla(k, bas, adim, eksik) { const v = []; let d = { etkin: bas }; for (let i = 0; i < adim; i++) { d = M.sonraki(k, d, eksik); v.push(d.eylem === 'oynat' ? d.etkin : d.eylem); } return v; }

test('sirali + basa: son klipten sonra ilk klibe döner', () => {
  assert.deepStrictEqual(sirayla(katman(3, { kip: 'sirali', bitince: 'basa' }), 0, 5), [1, 2, 0, 1, 2]);
});
test('sirali + dur / kapat', () => {
  assert.deepStrictEqual(sirayla(katman(2, { bitince: 'dur' }), 0, 2), [1, 'dur']);
  assert.deepStrictEqual(sirayla(katman(2, { bitince: 'kapat' }), 0, 2), [1, 'kapat']);
});
test('dongu: etkin klip tekrar eder', () => {
  assert.deepStrictEqual(sirayla(katman(3, { kip: 'dongu' }), 1, 3), [1, 1, 1]);
});
test('bulunamayan klip atlanır, hepsi bozuksa katman kapanır', () => {
  const k = katman(3);
  assert.deepStrictEqual(sirayla(k, 0, 3, new Set(['k1.mp4'])), [2, 0, 2]);
  assert.strictEqual(M.sonraki(k, { etkin: 0 }, new Set(['k0.mp4', 'k1.mp4', 'k2.mp4'])).eylem, 'kapat');
  assert.strictEqual(M.sonraki(katman(0), { etkin: 0 }).eylem, 'kapat');
  assert.strictEqual(M.sonraki(katman(2, { kip: 'dongu' }), { etkin: 0 }, new Set(['k0.mp4'])).etkin, 1);
});
test('rastgele: tur bitmeden aynı klip gelmez; basa ile sürer, dur ile durur', () => {
  let r = 0.37; const rnd = () => { r = (r * 9301 + 49297) % 233280 / 233280; return r; };
  const k = katman(5, { kip: 'rastgele', bitince: 'basa' });
  let d = { etkin: 2 }; const gorulen = [2];
  for (let i = 0; i < 4; i++) { d = M.sonraki(k, d, null, rnd); gorulen.push(d.etkin); }
  assert.deepStrictEqual(gorulen.slice().sort(), [0, 1, 2, 3, 4]);
  const son = d.etkin; d = M.sonraki(k, d, null, rnd);
  assert.strictEqual(d.eylem, 'oynat'); assert.notStrictEqual(d.etkin, son);
  const kd = katman(3, { kip: 'rastgele', bitince: 'dur' });
  let e = { etkin: 0 }; for (let i = 0; i < 2; i++) e = M.sonraki(kd, e, null, rnd);
  assert.strictEqual(M.sonraki(kd, e, null, rnd).eylem, 'dur');
  assert.strictEqual(M.sonraki(katman(1, { kip: 'rastgele' }), { etkin: 0 }, null, rnd).etkin, 0);
});
test('önceki: başta sona sarar, eksiği atlar', () => {
  const k = katman(4);
  assert.strictEqual(M.onceki(k, 2), 1);
  assert.strictEqual(M.onceki(k, 0), 3);
  assert.strictEqual(M.onceki(k, 2, new Set(['k1.mp4'])), 0);
  assert.strictEqual(M.onceki(katman(0), 0), -1);
});
test('solo kuralı', () => {
  const p = M.varsayilanProje(); p.katmanlar.push(M.varsayilanKatman(2)); p.katmanlar.push(M.varsayilanKatman(3));
  assert.ok(p.katmanlar.every(k => M.gorunurMu(p, k)));
  p.katmanlar[1].solo = true;
  assert.deepStrictEqual(p.katmanlar.map(k => M.gorunurMu(p, k)), [false, true, false]);
  p.katmanlar[1].acik = false;
  assert.ok(!M.gorunurMu(p, p.katmanlar[1]));
});
test('hedef kutuları: ikisi iki kutu, yayili 1900, tek kipte hedef yok sayılır', () => {
  const p = M.varsayilanProje(); const k = p.katmanlar[0];
  assert.deepStrictEqual(M.kutular(p, k), [{ x: 0, y: 0, w: 950, h: 1800 }, { x: 950, y: 0, w: 950, h: 1800 }]);
  k.hedef = 'sag'; assert.deepStrictEqual(M.kutular(p, k), [{ x: 950, y: 0, w: 950, h: 1800 }]);
  k.hedef = 'yayili'; assert.deepStrictEqual(M.kutular(p, k), [{ x: 0, y: 0, w: 1900, h: 1800 }]);
  p.ekranSayisi = 1; assert.deepStrictEqual(M.kutular(p, k), [{ x: 0, y: 0, w: 950, h: 1800 }]);
});
test('karışım eşlemesi', () => {
  assert.strictEqual(M.karisimCss({ karisim: 'ekle' }, true), 'plus-lighter');
  assert.strictEqual(M.karisimCss({ karisim: 'ekle' }, false), 'screen');
  assert.strictEqual(M.karisimCss({ karisim: 'carp' }, true), 'multiply');
  assert.strictEqual(M.karisimCss({ karisim: 'normal' }, true), 'normal');
});
test('ad kuralı sunucuyla aynı', () => {
  for (const a of ['..\\x.mp4', 'a/b.mp4', 'C:x.mp4', 'x.mp4.', 'x.mp4 ', '.gizli.mp4', 'x.exe', '', 'a\u0000.mp4', 'a|b.mp4', 'x.mov']) assert.ok(!M.adGecerli(a), a);
  for (const a of ['x.MP4', 'Kış Kampanyası.mp4', 'a b.webm', 'r.jpeg', 'r+1.png']) assert.ok(M.adGecerli(a), a);
  assert.ok(!M.adGecerli('a'.repeat(197) + '.mp4')); assert.ok(M.adGecerli('a'.repeat(196) + '.mp4'));
});
test('proje doğrulama: sınırlar, kırpma, bilinmeyen alan, beyaz liste', () => {
  const p = M.dogrula({ ekranGen: 99999, anaSes: 7, bilinmeyen: 1, katmanlar: [{ opaklik: -3, hiz: 9, gecisMs: 5000, etkin: 50, klipler: [{ tur: 'resim', ad: 'a.png', sure: 0 }], fazla: true }] });
  assert.strictEqual(p.ekranGen, 4096); assert.strictEqual(p.anaSes, 1); assert.ok(!('bilinmeyen' in p));
  const k = p.katmanlar[0];
  assert.strictEqual(k.opaklik, 0); assert.strictEqual(k.hiz, 2); assert.strictEqual(k.gecisMs, 2000); assert.strictEqual(k.etkin, 0);
  assert.strictEqual(k.klipler[0].sure, 1); assert.ok(!('fazla' in k)); assert.strictEqual(k.sessiz, true);
  assert.throws(() => M.dogrula({ katmanlar: Array.from({ length: 9 }, () => ({})) }), /8 katman/);
  assert.throws(() => M.dogrula({ katmanlar: [{ klipler: Array.from({ length: 201 }, () => ({ tur: 'video', ad: 'a.mp4' })) }] }), /200 klip/);
  assert.throws(() => M.dogrula({ katmanlar: [{ kip: 'ters' }] }), /kip/);
  assert.throws(() => M.dogrula({ katmanlar: [{ klipler: [{ tur: 'video', ad: '../x.mp4' }] }] }), /klip adı/);
  assert.throws(() => M.dogrula({ katmanlar: [{ klipler: [{ tur: 'video', ad: 'x.png' }] }] }), /uyuşmuyor/);
  const iki = M.dogrula({ katmanlar: [{ id: 3 }, { id: 3 }, {}] });
  assert.strictEqual(new Set(iki.katmanlar.map(k => k.id)).size, 3);
});
test('serileştirme gidiş-dönüş', () => {
  const p = M.varsayilanProje(); p.katmanlar[0].klipler.push({ tur: 'video', ad: 'Kış.mp4' }, { tur: 'resim', ad: 'a.webp', sure: 12 });
  assert.deepStrictEqual(M.dogrula(JSON.parse(JSON.stringify(p))), M.dogrula(p));
  assert.deepStrictEqual(M.dogrula(M.dogrula(p)), M.dogrula(p));
});
test('süre yazısı', () => { assert.strictEqual(M.sureYazi(65.9), '1:05'); assert.strictEqual(M.sureYazi(NaN), '0:00'); });

console.log('oynatici_test.js: ' + say + ' test geçti');
