// web/ai.js mantık testi (Node, DOM'suz sahte ortam). Çalıştırma: node tests/ai_test.js
// Sahip: Yusuf. Yazan: [Claude#AI Opus 5.5, yenileme-1.0, 2026-11-18]. Yalnız yazan yapay zekâ ya da Yusuf'un izniyle değişir; silme yok.
'use strict';
const assert = require('assert');
const path = require('path');

// ───── en küçük sahte DOM ─────
class Dugum {
  constructor(etiket, belge) { this.tagName = etiket.toUpperCase(); this.ownerDocument = belge; this.childNodes = []; this.attrs = {}; this.dinleyici = {}; this.textContent_ = ''; this.value = ''; this.hidden = false; this.disabled = false; }
  get firstChild() { return this.childNodes[0] || null; }
  appendChild(c) { this.childNodes.push(c); c.parentNode = this; return c; }
  removeChild(c) { this.childNodes = this.childNodes.filter(x => x !== c); return c; }
  setAttribute(k, v) { this.attrs[k] = String(v); if (k === 'hidden') this.hidden = true; if (k === 'id') this.id = v; }
  getAttribute(k) { return this.attrs[k]; }
  addEventListener(o, f) { (this.dinleyici[o] = this.dinleyici[o] || []).push(f); }
  tikla() { (this.dinleyici.click || []).forEach(f => f({})); }
  focus() {}
  set textContent(v) { this.textContent_ = String(v); this.childNodes = []; }
  get textContent() { return this.textContent_ + this.childNodes.map(c => c.textContent).join(''); }
}
function belgeYap() {
  const b = { kimlik: {} };
  b.createElement = e => new Dugum(e, b);
  b.createTextNode = t => { const d = new Dugum('#text', b); d.textContent_ = t; return d; };
  b.head = new Dugum('head', b);
  b.getElementById = id => (function ara(d) { if (d.id === id || d.attrs.id === id) return d; for (const c of d.childNodes) { const x = ara(c); if (x) return x; } return null; })(b.govde);
  b.govde = new Dugum('body', b); b.govde.appendChild(b.head);
  return b;
}
function bul(d, f) { if (f(d)) return d; for (const c of d.childNodes) { const x = bul(c, f); if (x) return x; } return null; }
function hepsi(d, f, o = []) { if (f(d)) o.push(d); d.childNodes.forEach(c => hepsi(c, f, o)); return o; }
const metinli = t => d => d.tagName === 'BUTTON' && d.textContent === t;

// ───── sahte fetch ─────
const istekler = [];
let hazirlaYaniti = null;
function yanit(govde, ok = true, durum = 200) { return Promise.resolve({ ok, status: durum, json: () => Promise.resolve(typeof govde === 'string' ? JSON.parse(govde) : govde), text: () => Promise.resolve(typeof govde === 'string' ? govde : JSON.stringify(govde)) }); }
global.window = global;
global.fetch = (yol, sec) => {
  istekler.push({ yol, sec });
  if (yol === '/tarif/siteler') return yanit(['ChatGPT', 'Gemini']);
  if (yol === '/tarif/llm-durum') return yanit({ acik: false, saglayici: 'chatgpt', model: 'gpt-4o-mini', adres: '', saglayicilar: [{ id: 'chatgpt', ad: 'ChatGPT', model: 'gpt-4o-mini' }] });
  if (yol === '/tarif/hazirla') return hazirlaYaniti ? hazirlaYaniti(sec) : Promise.reject(new TypeError('Failed to fetch'));
  if (yol === '/tarif/istem') return yanit('İSTEM METNİ');
  if (yol === '/tarif/uygula') return yanit({ ok: false, ozet: 'Yapıştırılan yanıtta video planı bulunamadı.', hatalar: ['JSON bulunamadı'], duzelt: 'düzelt' });
  if (yol === '/tarif/ac' || yol === '/tarif/iptal') return yanit('ok');
  return yanit('yok', false, 404);
};
Object.defineProperty(global, "navigator", { value: { clipboard: { yazilan: null, writeText(m) { this.yazilan = m; return Promise.resolve(); } } }, configurable: true, writable: true });

const TaaAI = require(path.join(__dirname, '..', 'web', 'ai.js'));
const bekle = () => new Promise(r => setTimeout(r, 0));

(async () => {
  // saf yardımcılar
  const ic = TaaAI._ic;
  assert.strictEqual(ic.kaynakAdi('yapay-zeka'), 'Yapay zekâ');
  assert.strictEqual(ic.kaynakAdi('kural'), 'Kural');
  assert.strictEqual(ic.kaynakAdi('???'), 'Şablon');
  assert.strictEqual(JSON.parse(ic.istekGovdesi('x'.repeat(5000), 2, true)).metin.length, 4000);
  assert.deepStrictEqual(JSON.parse(ic.istekGovdesi('a', -3)), { metin: 'a', cesit: 0 });
  assert.ok(/bağlantı kurulamadı/.test(ic.hataMetni(new TypeError('Failed to fetch'))));
  assert.ok(ic.adimOzeti({ kaynak: 'kural', adimlar: [{ ad: 'yapay-zeka', durum: 'hata', mesaj: 'İnternete bağlanılamadı.' }] }).startsWith('İnternete bağlanılamadı.'));
  assert.deepStrictEqual(ic.sahneSatirlari({ metin: 'A / B' }), ['A', 'B']);
  assert.deepStrictEqual(ic.sahneSatirlari({ satirlar: ['X'] }), ['X']);
  assert.ok(ic.ORNEKLER.length >= 6);

  // mount
  const belge = belgeYap();
  const el = belge.createElement('div'); belge.govde.appendChild(el);
  const uygulanan = [];
  const app = { senaryoDegistir: m => uygulanan.push(m), onizle: () => {}, senaryoMetni: () => '', senaryoyaEkle: () => {}, uret: () => {}, durum: () => ({}) };
  const p = TaaAI.mount(el, app);
  await bekle(); await bekle();
  assert.ok(belge.getElementById('taai-stil'), 'stil eklenmeli');
  const css = belge.getElementById('taai-stil').textContent;
  assert.ok(!/#[0-9a-f]{3,6}\b|rgb\(/i.test(css), 'CSS yalnız değişken kullanmalı, sabit renk yok');
  assert.ok(/min-height:48px/.test(css) && /min-height:56px/.test(css) && /font-size:16px/.test(css));
  assert.ok(bul(el, d => d.tagName === 'LABEL' && d.textContent === 'Videoyu anlatın'));
  const cipler = hepsi(el, d => d.attrs.class === 'taai-cip');
  assert.ok(cipler.length >= 6, 'örnek çipler');
  cipler[0].tikla();
  assert.strictEqual(p.alan.value, ic.ORNEKLER[0], 'çip metni doldurur');
  assert.ok(hepsi(el, d => d.tagName === 'BUTTON' && /’de aç$/.test(d.textContent)).length === 2, 'site düğmeleri');

  // ağ hatası → sade mesaj, çökme yok
  await p.hazirla(false);
  assert.ok(/bağlantı kurulamadı/.test(belge.getElementById('taai-sonuc').textContent));

  // başarılı hazırlık → rozet, sahneler, uygula
  hazirlaYaniti = sec => { const g = JSON.parse(sec.body); return yanit({ ok: true, kaynak: g.cesit ? 'sablon' : 'kural', senaryo: 'SENARYO-' + g.cesit, sure: 7.5, sahneler: [{ satirlar: ['%20', 'İNDİRİM'], metin: '%20 / İNDİRİM', giris: 'zipla', cikis: 'sondur', efekt: 'konfeti', palet: 'kirmizi-altin', font: 'anton', sure: 4 }], notlar: [], anladim: [], anlasilmayan: ['falan'], varsayimlar: ['Sona işletme adını ekledim.'], adimlar: [{ ad: 'yapay-zeka', durum: 'atlandi', mesaj: 'kapalı' }], yz_onay_gerekli: false }); };
  await p.hazirla(false);
  const sonuc = belge.getElementById('taai-sonuc');
  const rozet = bul(sonuc, d => d.attrs.class === 'taai-rozet');
  assert.strictEqual(rozet.textContent, 'Kural'); assert.strictEqual(rozet.attrs['data-k'], 'kural');
  assert.ok(/İNDİRİM/.test(sonuc.textContent) && /Sona işletme adını/.test(sonuc.textContent) && /falan/.test(sonuc.textContent));
  bul(sonuc, metinli('Uygula')).tikla();
  assert.deepStrictEqual(uygulanan, ['SENARYO-0']);
  bul(sonuc, metinli('Başka öneri')).tikla(); await bekle(); await bekle();
  assert.strictEqual(JSON.parse(istekler.filter(x => x.yol === '/tarif/hazirla').pop().sec.body).cesit, 1, 'Başka öneri çeşidi artırır');
  assert.strictEqual(bul(belge.getElementById('taai-sonuc'), d => d.attrs.class === 'taai-rozet').textContent, 'Şablon');
  // istek başlığı ve yöntem
  const son = istekler.filter(x => x.yol === '/tarif/hazirla').pop();
  assert.strictEqual(son.sec.method, 'POST'); assert.strictEqual(son.sec.headers['X-LED'], '1');

  // sunucu metni HTML olarak yorumlanmaz (textContent)
  hazirlaYaniti = () => yanit({ ok: true, kaynak: 'kural', senaryo: 's', sure: 1, sahneler: [{ satirlar: ['<img src=x onerror=alert(1)>'], giris: '', efekt: '', palet: '', sure: 1 }], adimlar: [] });
  await p.hazirla(false);
  assert.ok(!bul(belge.getElementById('taai-sonuc'), d => d.tagName === 'IMG'), 'HTML öğesi oluşmamalı');

  // yapıştırılan bozuk yanıt → sade özet + düzelt istemi
  belge.getElementById('taai-yanit').value = 'bozuk';
  await p.yanitUygula();
  assert.ok(/video planı bulunamadı/.test(belge.getElementById('taai-sonuc').textContent));
  bul(belge.getElementById('taai-sonuc'), metinli('Düzelt istemini kopyala')).tikla(); await bekle();
  assert.strictEqual(global.navigator.clipboard.yazilan, 'düzelt');

  // istem kopyalama: boşken uyarı, doluyken panoya
  p.alan.value = '';
  assert.strictEqual(await p.istemKopyala(), false);
  p.alan.value = 'indirim';
  assert.strictEqual(await p.istemKopyala(), true);
  assert.strictEqual(global.navigator.clipboard.yazilan, 'İSTEM METNİ');

  // onay gerektiğinde "Yapay zekâya gönder" düğmesi onay:true gönderir
  hazirlaYaniti = sec => yanit({ ok: true, kaynak: 'kural', senaryo: 's', sure: 1, sahneler: [], adimlar: [], yz_onay_gerekli: !JSON.parse(sec.body).onay });
  await p.hazirla(false);
  bul(belge.getElementById('taai-sonuc'), metinli('Yapay zekâya gönder')).tikla(); await bekle(); await bekle();
  assert.strictEqual(JSON.parse(istekler.filter(x => x.yol === '/tarif/hazirla').pop().sec.body).onay, true);

  console.log('ai_test.js: tüm denetimler geçti');
})().catch(e => { console.error('ai_test.js BAŞARISIZ:', e); process.exit(1); });
