// web/ai.js mantık testi (Node, DOM'suz sahte ortam). Çalıştırma: node tests/ai_test.js
// Sahip: Yusuf. Yazan: [Claude#AI Opus 5.5, yenileme-1.0, 2026-11-18]; dalga 3 ek [Claude#AI Opus 5.5, 2026-11-25]: adım göstergesi, Geri al, Durdur→yerel,
// geçici önizleme, Diğer araçlar, işletme adı ayarı, Her seferinde sor. Yalnız yazan yapay zekâ ya da Yusuf'un izniyle değişir; silme yok.
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
  if (yol === '/tarif/ac' || yol === '/tarif/iptal' || yol === '/tarif/llm-ayar' || yol === '/tarif/kural-ekle' || yol === '/tarif/site-ekle') return yanit('ok');
  if (yol === '/tarif/kurallar') return yanit([{ kaynak: 'gömülü', k: 'giris', h: 'zipla', i: ['zıplayarak'] }, { kaynak: 'benim', k: 'giris', h: 'zipla', i: ['hoplayarak'] }]);
  if (yol === '/tarif/kural-coz') return yanit({ ok: true, senaryo: 'KURAL-SENARYO', sure: 6, sahneler: [{ satirlar: ['HOŞ'], sure: 6 }] });
  return yanit('yok', false, 404);
};
Object.defineProperty(global, "navigator", { value: { clipboard: { yazilan: null, okunacak: '', writeText(m) { this.yazilan = m; return Promise.resolve(); }, readText() { return Promise.resolve(this.okunacak); } } }, configurable: true, writable: true });
const depoVeri = {};
global.localStorage = { getItem: k => (k in depoVeri ? depoVeri[k] : null), setItem: (k, v) => { depoVeri[k] = String(v); }, removeItem: k => { delete depoVeri[k]; } };
global.AbortController = class { constructor() { this.signal = { aborted: false }; } abort() { this.signal.aborted = true; } };

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
  assert.ok(ic.YZ_BUTCE_MS <= 35000, 'YZ bütçesi en çok 35 sn');
  assert.deepStrictEqual(ic.ADIMLAR, ['Yaz', 'Hazırla', 'Uygula', 'Önizle']);
  assert.strictEqual(ic.isletmeAyari({ getItem: () => null }), undefined, 'ayar yoksa sunucu varsayılanı');
  assert.strictEqual(ic.isletmeAyari({ getItem: () => 'kapali' }), '', 'kapalı → boş (imza yok)');
  assert.strictEqual(ic.isletmeAyari({ getItem: () => ' Moda Evi ' }), 'Moda Evi');
  assert.strictEqual(ic.isletmeAyari({ getItem: () => { throw new Error('x'); } }), undefined);
  assert.deepStrictEqual(JSON.parse(ic.istekGovdesi('a', 0, false, { isletme: '', yz: false })), { metin: 'a', cesit: 0, isletme: '', yz: false });
  assert.strictEqual(ic.istemGovdesi('a', undefined), 'a', 'ayar yoksa düz metin (eski uç)');
  assert.deepStrictEqual(JSON.parse(ic.istemGovdesi('a', 'X')), { metin: 'a', isletme: 'X' });

  // mount
  const belge = belgeYap();
  const el = belge.createElement('div'); belge.govde.appendChild(el);
  const uygulanan = [];
  const gecici = [];
  let mevcutSenaryo = 'ESKI-SENARYO';
  const app = { senaryoDegistir: m => { uygulanan.push(m); mevcutSenaryo = m; }, onizle: () => { throw new Error('geciciOnizle varken onizle çağrılmamalı'); }, geciciOnizle: m => { gecici.push(m); return true; }, senaryoMetni: () => mevcutSenaryo, senaryoyaEkle: () => {}, uret: () => {}, durum: () => ({}) };
  const p = TaaAI.mount(el, app);
  await bekle(); await bekle();
  assert.ok(belge.getElementById('taai-stil'), 'stil eklenmeli');
  const css = belge.getElementById('taai-stil').textContent;
  assert.ok(!/#[0-9a-f]{3,6}\b|rgb\(/i.test(css), 'CSS yalnız değişken kullanmalı, sabit renk yok');
  assert.ok(/min-height:48px/.test(css) && /min-height:56px/.test(css) && /font-size:16px/.test(css));
  // .taai-ana dolu değil (çerçeveli vurgu; dolu yalnız "Videoyu yap")
  const anaKural = css.split('\n').find(l => l.startsWith('.taai .taai-ana'));
  assert.ok(anaKural && !/background:var\(--ac\)/.test(anaKural) && /border:2px solid var\(--ac\)/.test(anaKural), anaKural);
  // Hazırla metin kutusunun hemen altında (1366x768'de ekran içinde)
  const kokSec = bul(el, d => d.tagName === 'SECTION');
  const ci = kokSec.childNodes.indexOf(belge.getElementById('taai-metin'));
  assert.ok(bul(kokSec.childNodes[ci + 1], d => d.id === 'taai-hazirla'), 'Hazırla metin kutusunun hemen ardından gelmeli');
  // adım göstergesi
  const adimlar = hepsi(el, d => d.tagName === 'LI' && /^\d\. /.test(d.textContent));
  assert.deepStrictEqual(adimlar.map(x => x.textContent), ['1. Yaz', '2. Hazırla', '3. Uygula', '4. Önizle']);
  assert.strictEqual(adimlar[0].attrs['aria-current'], 'step');
  // Diğer araçlar kapalı açılır bölümde
  const diger = belge.getElementById('taai-diger');
  assert.ok(diger && diger.tagName === 'DETAILS' && diger.attrs.open === undefined, 'Diğer araçlar kapalı details');
  for (const t of ['+ Site', 'Kural listesi', 'Kural ekle', 'İstemi göster', 'Kuraldan çöz', 'Panodaki tarifi uygula']) assert.ok(bul(diger, metinli(t)), 'eksik düğme: ' + t);
  // işletme adı: varsayılan açık ve mevcut ad
  assert.strictEqual(belge.getElementById('taai-isletme').value, 'TERZİ ADEM ALTUN');
  assert.strictEqual(belge.getElementById('taai-isletme-acik').checked, true);
  // Her seferinde sor varsayılan açık
  assert.strictEqual(belge.getElementById('taai-sor').checked, true);
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
  assert.strictEqual(adimlar[2].attrs['aria-current'], 'step', 'hazırlandı → Uygula adımı');
  // Önizle: senaryoyu değiştirmeden geçici önizleme
  bul(sonuc, metinli('Önizle')).tikla();
  assert.deepStrictEqual(gecici, ['SENARYO-0']); assert.deepStrictEqual(uygulanan, []);
  bul(sonuc, metinli('Uygula')).tikla();
  assert.deepStrictEqual(uygulanan, ['SENARYO-0']);
  assert.strictEqual(adimlar[3].attrs['aria-current'], 'step');
  // Geri al: önceki senaryoya döner
  const geri = belge.getElementById('taai-geri');
  assert.strictEqual(geri.hidden, false, 'Uygula sonrası Geri al görünür');
  geri.tikla();
  assert.deepStrictEqual(uygulanan, ['SENARYO-0', 'ESKI-SENARYO']);
  assert.strictEqual(mevcutSenaryo, 'ESKI-SENARYO'); assert.strictEqual(geri.hidden, true);
  assert.strictEqual(p.geriAl(), false, 'ikinci geri al yok');
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

  // Durdur: YZ isteği kesilir, iptal bildirilir, anında yerel (yz:false) sonuç gelir
  hazirlaYaniti = sec => JSON.parse(sec.body).yz === false ? yanit({ ok: true, kaynak: 'kural', senaryo: 'YEREL', sure: 6, sahneler: [{ satirlar: ['YEREL'], sure: 6 }], adimlar: [] }) : new Promise(() => {});
  p.alan.value = 'yavaş yapay zekâ';
  const bekleyen = p.hazirla(false); await bekle();
  assert.strictEqual(belge.getElementById('taai-iptal').hidden, false, 'meşgulken Durdur görünür');
  const kontrol = p.durum.kontrol;
  await p.durdur('kullanici'); await bekle();
  assert.ok(kontrol.signal.aborted, 'fetch kesildi');
  assert.ok(istekler.some(x => x.yol === '/tarif/iptal'), 'sunucuya iptal');
  assert.strictEqual(JSON.parse(istekler.filter(x => x.yol === '/tarif/hazirla').pop().sec.body).yz, false);
  assert.ok(/YEREL/.test(belge.getElementById('taai-sonuc').textContent) && /Durduruldu/.test(belge.getElementById('taai-sonuc').textContent));
  assert.strictEqual(p.durum.mesgul, false); void bekleyen;

  // işletme adı ayarı: kapatılınca 'kapali' saklanır ve isteğe isletme:'' gider
  belge.getElementById('taai-isletme-acik').checked = false;
  p.isletmeKaydet();
  assert.strictEqual(depoVeri['taa-isletme'], 'kapali');
  hazirlaYaniti = () => yanit({ ok: true, kaynak: 'kural', senaryo: 's', sure: 6, sahneler: [], adimlar: [] });
  await p.hazirla(false);
  assert.strictEqual(JSON.parse(istekler.filter(x => x.yol === '/tarif/hazirla').pop().sec.body).isletme, '');
  belge.getElementById('taai-isletme-acik').checked = true; belge.getElementById('taai-isletme').value = 'Moda Dikiş';
  p.isletmeKaydet();
  assert.strictEqual(depoVeri['taa-isletme'], 'Moda Dikiş');
  await p.istemKopyala();
  assert.deepStrictEqual(JSON.parse(istekler.filter(x => x.yol === '/tarif/istem').pop().sec.body), { metin: 'yavaş yapay zekâ', isletme: 'Moda Dikiş' });

  // Kaydet ve aç: "Her seferinde sor" işaretliyken sorma:false (onay kalıcı kapanmaz); işaret kalkınca sorma:true
  const kaydetVeAc = bul(belge.getElementById('taai-llm'), metinli('Kaydet ve aç'));
  kaydetVeAc.tikla(); await bekle();
  assert.strictEqual(JSON.parse(istekler.filter(x => x.yol === '/tarif/llm-ayar').pop().sec.body).sorma, false);
  belge.getElementById('taai-sor').checked = false;
  kaydetVeAc.tikla(); await bekle();
  assert.strictEqual(JSON.parse(istekler.filter(x => x.yol === '/tarif/llm-ayar').pop().sec.body).sorma, true);

  // Diğer araçlar
  await p.istemGoster();
  assert.strictEqual(belge.getElementById('taai-istem').value, 'İSTEM METNİ'); assert.strictEqual(belge.getElementById('taai-istem').hidden, false);
  await p.kuraldanCoz();
  assert.ok(/HOŞ/.test(belge.getElementById('taai-sonuc').textContent));
  assert.strictEqual(bul(belge.getElementById('taai-sonuc'), d => d.attrs.class === 'taai-rozet').textContent, 'Kural');
  global.navigator.clipboard.okunacak = '';
  await p.panodanUygula();
  global.navigator.clipboard.okunacak = '{"sahneler":[]}';
  await p.panodanUygula();
  assert.ok(istekler.filter(x => x.yol === '/tarif/uygula').pop().sec.body === '{"sahneler":[]}');
  const kl = await p.kurallariYukle();
  assert.strictEqual(kl.length, 2); assert.ok(/hoplayarak → zipla \(sizin\)/.test(belge.getElementById('taai-kurallar').textContent));
  assert.strictEqual(await p.kuralEkle(), false, 'boş kural eklenmez');
  belge.getElementById('taai-kural-i').value = 'hoplayarak'; belge.getElementById('taai-kural-h').value = 'zipla';
  assert.strictEqual(await p.kuralEkle(), true);
  assert.deepStrictEqual(JSON.parse(istekler.filter(x => x.yol === '/tarif/kural-ekle').pop().sec.body), { k: 'giris', i: 'hoplayarak', h: 'zipla' });
  assert.strictEqual(await p.siteEkle(), false);
  belge.getElementById('taai-site-ad').value = 'Ornek'; belge.getElementById('taai-site-adres').value = 'https://ornek.example/';
  assert.strictEqual(await p.siteEkle(), true);

  console.log('ai_test.js: tüm denetimler geçti');
})().catch(e => { console.error('ai_test.js BAŞARISIZ:', e); process.exit(1); });
