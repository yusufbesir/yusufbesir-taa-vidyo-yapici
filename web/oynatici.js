/* Oynatıcı kontrol sayfası (kumanda): katman listesi, klip listesi, taşıma, sürgüler, medya seçici, kısayollar, canlı durum.
   Sahip Yusuf; yer tutucu [Claude#UI Opus 5.5]; içerik yazan [Claude#Oynatici Opus 5.5, yenileme-1.0, 2026-11-25]; yalnız yazan yapay zekâ ya da Yusuf'un izniyle değişir; silme yok.
   Karar: tasarim/yenileme/karar-oynatici.md 1-3. Proje sunucuya yazılır (temelSurum ile; 409'da güncel proje uygulanır).
   Kullanıcı metni ve dosya adları yalnız textContent ile basılır. Dosya silme yok: "Listeden çıkar" yalnız listeyi değiştirir. */
(function () {
  'use strict';
  var M = window.OynaticiMotor;
  var $ = function (id) { return document.getElementById(id); };
  var TEMALAR = ['vitrin', 'vurus', 'cephe', 'kadraj', 'manset', 'eslik'];
  (function tema() {
    var t = null;
    try { t = localStorage.getItem('taa-tema'); } catch (e) { t = null; }
    if (TEMALAR.indexOf(t) >= 0) { document.documentElement.dataset.tema = t; return; }
    document.documentElement.dataset.tema = 'vurus';
    fetch('/ayar', { cache: 'no-store' }).then(function (r) { return r.ok ? r.json() : {}; }).then(function (a) { if (TEMALAR.indexOf(a['taa-tema']) >= 0) document.documentElement.dataset.tema = a['taa-tema']; }).catch(function () {});
  })();

  var proje = null, surum = 0, seciliId = null, medyalar = [], canli = null, canliYas = null;
  var kaydediliyor = false, bekleyenKayit = false, kayitZaman = null;
  var geriAlVeri = null, bildirimZaman = null;
  var kanal = null;
  try { kanal = new BroadcastChannel('taa-oynatici'); } catch (e) { kanal = null; }
  var konumSurukleniyor = false;

  function el(tag, sinif, yazi) { var e = document.createElement(tag); if (sinif) e.className = sinif; if (yazi !== undefined) e.textContent = yazi; return e; }
  function dug(yazi, sinif, etiket) { var b = el('button', 'dug ' + (sinif || ''), yazi); b.type = 'button'; if (etiket) { b.setAttribute('aria-label', etiket); b.title = etiket; } return b; }
  function secili() { return proje && proje.katmanlar.find(function (k) { return k.id === seciliId; }) || null; }
  function post(yol, govde) { return fetch(yol, { method: 'POST', headers: { 'X-LED': '1', 'Content-Type': 'application/json' }, body: typeof govde === 'string' ? govde : JSON.stringify(govde) }); }
  function bildir(m, geriAl) {
    $('bildirimYazi').textContent = m;
    $('bildirim').hidden = false;
    $('geriAl').hidden = !geriAl;
    geriAlVeri = geriAl || null;
    clearTimeout(bildirimZaman);
    bildirimZaman = setTimeout(function () { $('bildirim').hidden = true; geriAlVeri = null; }, geriAl ? 5000 : 3500);
  }
  function boyutYazi(b) { return b > 1e9 ? (b / 1e9).toFixed(1) + ' GB' : b > 1e6 ? (b / 1e6).toFixed(1) + ' MB' : Math.max(1, Math.round(b / 1e3)) + ' KB'; }

  /* ---------------------------------------------------------------- Sunucu */
  async function projeYukle() {
    var r = await fetch('/oynatici/proje', { cache: 'no-store' });
    var j = await r.json();
    proje = j.proje; surum = j.surum;
    if (!secili()) seciliId = proje.katmanlar.length ? proje.katmanlar[proje.katmanlar.length - 1].id : null;
    ciz();
  }
  function kaydet() {
    clearTimeout(kayitZaman);
    kayitZaman = setTimeout(kaydetSimdi, 120);
  }
  async function kaydetSimdi() {
    if (kaydediliyor) { bekleyenKayit = true; return; }
    kaydediliyor = true;
    try {
      var p = M.dogrula(proje);
      var r = await post('/oynatici/proje', { temelSurum: surum, proje: p });
      if (r.status === 409) { var j = await r.json(); proje = j.proje; surum = j.surum; ciz(); bildir('Proje başka yerde değişti; güncel hâli yüklendi.'); }
      else if (r.ok) { surum = (await r.json()).surum; }
      else bildir('Kaydedilemedi: ' + (await r.text()));
    } catch (e) { bildir('Kaydedilemedi: ' + e.message); }
    kaydediliyor = false;
    if (bekleyenKayit) { bekleyenKayit = false; kaydetSimdi(); }
  }
  function degistir(f) { f(); ciz(); kaydet(); }
  function komut(tur, deger, tumu) {
    var k = secili();
    var g = { tur: tur };
    if (!tumu && k) g.katman = k.id;
    if (deger !== undefined) g.deger = deger;
    return post('/oynatici/komut', g).catch(function () {});
  }
  async function medyaYukle() {
    try { var r = await fetch('/oynatici/medyalar', { cache: 'no-store' }); medyalar = await r.json(); } catch (e) { medyalar = []; }
    medyaCiz();
  }

  /* ---------------------------------------------------------------- Çizim */
  function ciz() {
    if (!proje) return;
    $('kompozisyon').value = String(proje.ekranSayisi);
    $('karartBtn').setAttribute('aria-pressed', String(!!proje.karart));
    $('karartBtn').textContent = proje.karart ? 'Karartmayı kaldır' : 'Karart';
    var hepsiOynuyor = proje.katmanlar.some(function (k) { return k.oynuyor; });
    $('hepsiBtn').textContent = hepsiOynuyor ? 'Hepsini duraklat' : 'Hepsini oynat';
    $('katmanEkle').disabled = proje.katmanlar.length >= M.SINIR.katman;
    $('monitor').value = String(proje.cikis.monitor);
    $('acilistaAc').checked = !!proje.acilistaCikisAc;
    if (document.activeElement !== $('cikisX')) $('cikisX').value = proje.cikis.x;
    if (document.activeElement !== $('cikisY')) $('cikisY').value = proje.cikis.y;
    katmanCiz();
    ayrintiCiz();
    var video = proje.katmanlar.filter(function (k) { return M.gorunurMu(proje, k) && k.oynuyor && k.klipler.some(function (c) { return c.tur === 'video'; }); }).length;
    var u = $('yukUyari');
    u.hidden = video <= 4;
    u.textContent = 'Aynı anda ' + video + ' video katmanı oynuyor; takılma olursa bazılarını kapatın.';
  }
  function katmanCiz() {
    var ol = $('katmanListe');
    ol.textContent = '';
    var liste = proje.katmanlar.slice().reverse();
    liste.forEach(function (k) {
      var sira = proje.katmanlar.indexOf(k) + 1;
      var li = el('li', 'katman-satir' + (k.id === seciliId ? ' secili' : '') + (M.gorunurMu(proje, k) ? '' : ' sonuk'));
      var sec = el('button', 'katman-sec');
      sec.type = 'button';
      sec.setAttribute('aria-pressed', String(k.id === seciliId));
      var no = el('span', 'no', String(sira));
      var ad = el('span', 'ad', k.ad);
      var klip = k.klipler[k.etkin];
      var alt = el('span', 'alt', k.klipler.length ? (klip ? klip.ad : '') + ' · ' + k.klipler.length + ' klip' : 'Klip yok');
      var metin = el('span', 'metin'); metin.append(ad, alt);
      sec.append(no, metin);
      sec.addEventListener('click', function () { seciliId = k.id; ciz(); });
      sec.addEventListener('dragover', function (e) { e.preventDefault(); li.classList.add('birak'); });
      sec.addEventListener('dragleave', function () { li.classList.remove('birak'); });
      sec.addEventListener('drop', function (e) { e.preventDefault(); li.classList.remove('birak'); var ad = e.dataTransfer.getData('text/plain'); if (ad) klipEkle(k, ad); });
      var bt = el('div', 'katman-dug');
      var a = dug(k.acik ? 'Açık' : 'Kapalı', 'kucuk' + (k.acik ? ' etkin' : ''), 'Katman ' + sira + ' aç/kapat');
      a.setAttribute('aria-pressed', String(k.acik));
      a.addEventListener('click', function () { degistir(function () { k.acik = !k.acik; }); });
      var s = dug('Solo', 'kucuk' + (k.solo ? ' etkin' : ''), 'Katman ' + sira + ' solo');
      s.setAttribute('aria-pressed', String(k.solo));
      s.addEventListener('click', function () { degistir(function () { k.solo = !k.solo; }); });
      var m = dug(k.sessiz ? 'Sessiz' : 'Sesli', 'kucuk' + (k.sessiz ? '' : ' etkin'), 'Katman ' + sira + ' ses');
      m.setAttribute('aria-pressed', String(!k.sessiz));
      m.addEventListener('click', function () { degistir(function () { k.sessiz = !k.sessiz; }); });
      bt.append(a, s, m);
      var op = el('input', 'op');
      op.type = 'range'; op.min = 0; op.max = 100; op.value = Math.round(k.opaklik * 100);
      op.setAttribute('aria-label', 'Katman ' + sira + ' opaklık');
      surgu(op, k, 'opaklik', 100);
      li.append(sec, bt, op);
      ol.appendChild(li);
    });
  }
  /* Sürgü: sürüklerken ara değer BroadcastChannel ile çıkışa (yoksa 10 Hz sunucu yazımı), bırakınca kesin değer sunucuya. */
  var sonAra = 0;
  function surgu(input, k, alan, bolen) {
    input.addEventListener('input', function () {
      k[alan] = input.value / bolen;
      var o = $(alan + 'Yazi'); if (o && k === secili()) o.textContent = String(input.value);
      if (kanal) kanal.postMessage({ tur: 'ara', id: k.id, alan: alan, deger: k[alan] });
      else if (Date.now() - sonAra > 100) { sonAra = Date.now(); kaydetSimdi(); }
    });
    input.addEventListener('change', function () { k[alan] = input.value / bolen; ciz(); kaydet(); });
  }
  function ayrintiCiz() {
    var k = secili();
    var kapali = !k;
    document.querySelectorAll('.ayrinti input, .ayrinti select, .ayrinti button').forEach(function (e) { e.disabled = kapali; });
    if (!k) { $('katmanAd').value = ''; $('klipListe').textContent = ''; $('etkinAd').textContent = 'Katman yok. “Yeni katman” ile başlayın.'; return; }
    if (document.activeElement !== $('katmanAd')) $('katmanAd').value = k.ad;
    var i = proje.katmanlar.indexOf(k);
    $('katYukari').disabled = i >= proje.katmanlar.length - 1;
    $('katAsagi').disabled = i <= 0;
    $('katCogalt').disabled = proje.katmanlar.length >= M.SINIR.katman;
    $('kip').value = k.kip; $('bitince').value = k.bitince; $('hiz').value = String(k.hiz);
    $('karisim').value = k.karisim; $('hedef').value = k.hedef; $('sigdir').value = k.sigdir;
    $('hedef').disabled = proje.ekranSayisi === 1;
    $('bitince').disabled = k.kip === 'dongu';
    $('gecis').value = k.gecisMs; $('gecisYazi').textContent = k.gecisMs + ' ms';
    $('opaklik').value = Math.round(k.opaklik * 100); $('opaklikYazi').textContent = String(Math.round(k.opaklik * 100));
    $('ses').value = Math.round(k.ses * 100); $('sesYazi').textContent = String(Math.round(k.ses * 100));
    $('sessiz').checked = k.sessiz;
    $('oynatBtn').textContent = k.oynuyor ? 'Duraklat' : 'Oynat';
    $('oynatBtn').setAttribute('aria-pressed', String(!k.oynuyor));
    klipCiz(k);
    canliCiz();
  }
  function klipCiz(k) {
    var ol = $('klipListe');
    ol.textContent = '';
    var cl = canliKatman(k);
    var etkin = cl ? cl.etkin : k.etkin;
    var eksik = new Set(cl && cl.eksik || []);
    var var_ = new Set(medyalar.map(function (m) { return m.ad; }));
    if (!k.klipler.length) { var b = el('li', 'bos-yazi', 'Liste boş. Sağdaki medyadan klip ekleyin.'); ol.appendChild(b); }
    k.klipler.forEach(function (c, i) {
      var li = el('li', 'klip-satir' + (i === etkin ? ' etkin' : ''));
      var bul = medyalar.length && !var_.has(c.ad);
      var t = el('button', 'klip-tetik');
      t.type = 'button';
      t.setAttribute('aria-label', (i + 1) + '. klibi oynat: ' + c.ad);
      t.append(el('span', 'no', String(i + 1)), el('span', 'ad', c.ad), el('span', 'tur', c.tur === 'resim' ? 'Resim · ' + c.sure + ' sn' : 'Video'));
      if (bul || eksik.has(c.ad)) t.append(el('span', 'etiket uy', 'bulunamadı'));
      if (i === etkin) t.append(el('span', 'etiket ok', 'oynuyor'));
      t.addEventListener('click', function () { komut('tetikle', i); });
      var d = el('div', 'klip-dug');
      if (c.tur === 'resim') {
        var sure = el('input', 'sure'); sure.type = 'number'; sure.min = 1; sure.max = 3600; sure.value = c.sure;
        sure.setAttribute('aria-label', c.ad + ' süresi (sn)');
        sure.addEventListener('change', function () { degistir(function () { c.sure = Math.min(3600, Math.max(1, +sure.value || 8)); }); });
        d.appendChild(sure);
      }
      var y = dug('▲', 'kare kucuk', 'Yukarı taşı'); y.disabled = i === 0;
      y.addEventListener('click', function () { degistir(function () { k.klipler.splice(i - 1, 0, k.klipler.splice(i, 1)[0]); }); });
      var a = dug('▼', 'kare kucuk', 'Aşağı taşı'); a.disabled = i === k.klipler.length - 1;
      a.addEventListener('click', function () { degistir(function () { k.klipler.splice(i + 1, 0, k.klipler.splice(i, 1)[0]); }); });
      var x = dug('Çıkar', 'kucuk', c.ad + ' listeden çıkar (dosya silinmez)');
      x.addEventListener('click', function () { klipCikar(k, i); });
      d.append(y, a, x);
      li.append(t, d);
      ol.appendChild(li);
    });
  }
  function klipCikar(k, i) {
    var c = k.klipler[i];
    degistir(function () { k.klipler.splice(i, 1); });
    bildir('“' + c.ad + '” listeden çıkarıldı (dosya silinmedi).', function () { degistir(function () { k.klipler.splice(Math.min(i, k.klipler.length), 0, c); }); });
  }
  function klipEkle(k, ad) {
    var m = medyalar.find(function (x) { return x.ad === ad; });
    if (!m) return;
    if (!m.oynatilabilir) { bildir(m.neden || 'Bu dosya oynatılamaz.'); return; }
    if (k.klipler.length >= M.SINIR.klip) { bildir('Katman başına en çok ' + M.SINIR.klip + ' klip.'); return; }
    degistir(function () { k.klipler.push(m.tur === 'resim' ? { tur: 'resim', ad: ad, sure: 8 } : { tur: 'video', ad: ad }); });
    bildir('“' + ad + '” → ' + k.ad);
  }
  function medyaCiz() {
    var ul = $('medyaListe');
    ul.textContent = '';
    var q = $('ara').value.trim().toLocaleLowerCase('tr-TR');
    var v = medyalar.filter(function (m) { return !q || m.ad.toLocaleLowerCase('tr-TR').indexOf(q) >= 0; });
    if (!v.length) ul.appendChild(el('li', 'bos-yazi', medyalar.length ? 'Eşleşen dosya yok.' : 'Çıktı klasöründe mp4/resim yok. Önce video üretin.'));
    v.forEach(function (m) {
      var li = el('li', 'medya-satir' + (m.oynatilabilir ? '' : ' sonuk'));
      li.draggable = m.oynatilabilir;
      li.addEventListener('dragstart', function (e) { e.dataTransfer.setData('text/plain', m.ad); e.dataTransfer.effectAllowed = 'copy'; });
      var bilgi = el('div', 'metin');
      bilgi.append(el('span', 'ad', m.ad), el('span', 'alt', (m.tur === 'resim' ? 'Resim' : 'Video') + ' · ' + boyutYazi(m.boyut) + (m.oynatilabilir ? '' : ' · ' + (m.neden || 'oynatılamaz'))));
      var b = dug('Ekle', 'kucuk', m.ad + ' seçili katmana ekle');
      b.disabled = !m.oynatilabilir || !secili();
      b.addEventListener('click', function () { var k = secili(); if (k) klipEkle(k, m.ad); });
      li.append(bilgi, b);
      ul.appendChild(li);
    });
  }
  function canliKatman(k) { return canli && canli.katmanlar ? canli.katmanlar.find(function (c) { return c.id === k.id; }) : null; }
  var onizlemeAd = null;
  function canliCiz() {
    var d = $('cikisDurum');
    var bagli = canliYas !== null && canliYas < 5000;
    d.textContent = canliYas === null ? 'Çıkış açık değil' : bagli ? 'Çıkış çalışıyor' : 'Çıkış yanıt vermiyor';
    d.dataset.durum = bagli ? 'ok' : canliYas === null ? 'bos' : 'uy';
    $('cikisBtn').textContent = bagli ? 'Çıkışı kapat' : canliYas === null ? 'Çıkışı aç' : 'Çıkışı yeniden aç';
    var k = secili();
    if (!k) return;
    var cl = canliKatman(k);
    var klip = k.klipler[cl ? cl.etkin : k.etkin];
    $('etkinAd').textContent = klip ? klip.ad : 'Klip yok';
    if (!konumSurukleniyor) {
      var sure = cl ? cl.sure : 0, konum = cl ? cl.konum : 0;
      $('konum').value = sure ? Math.round(konum / sure * 1000) : 0;
      $('konumYazi').textContent = M.sureYazi(konum);
      $('sureYazi').textContent = M.sureYazi(sure);
    }
    $('konum').disabled = !klip || klip.tur !== 'video';
    var bilgi = [];
    if (cl && cl.kare) bilgi.push('Düşen kare: ' + cl.dusen + ' / ' + cl.kare + ' (%' + (cl.dusen / Math.max(1, cl.kare) * 100).toFixed(1) + ')');
    if (cl && cl.bitti) bilgi.push('Liste bitti');
    if (canli && canli.uyanik === false && bagli) bilgi.push('Ekran uykusu engellenemedi: Windows güç ayarından kapatın.');
    if (canli && canli.artiVar === false && k.karisim === 'ekle') bilgi.push('“Ekle” bu sürümde “screen” ile yapılıyor.');
    $('canliBilgi').textContent = bilgi.join(' · ');
    // Küçük önizleme: gerçek videoyu ikinci kez çözmez; yalnız ilk kare/resim
    var ad = klip ? klip.ad : null;
    if (ad !== onizlemeAd) {
      onizlemeAd = ad;
      var o = $('onizleme');
      o.textContent = '';
      if (!klip) o.appendChild(el('span', 'bos-yazi', 'Klip yok'));
      else if (klip.tur === 'resim') { var im = el('img'); im.alt = ''; im.src = '/oynatici/kucuk/' + encodeURIComponent(ad); o.appendChild(im); }
      else { var v = el('video'); v.muted = true; v.preload = 'metadata'; v.src = '/oynatici/medya/' + encodeURIComponent(ad) + '#t=0.5'; o.appendChild(v); }
    }
  }
  async function canliYoklama() {
    try {
      var r = await fetch('/oynatici/canli', { cache: 'no-store' });
      var j = await r.json();
      canliYas = j.yasMs; canli = j.durum;
    } catch (e) { canliYas = null; }
    var k = secili();
    canliCiz();
    if (k) { var cl = canliKatman(k); var akt = document.querySelector('#klipListe .klip-satir.etkin .no'); if (cl && (!akt || +akt.textContent !== cl.etkin + 1)) klipCiz(k); }
  }
  setInterval(canliYoklama, 1000);
  // --tarayici-yok kipinde ana sayfa yerine bu sayfa açıkken de sunucu kapanmasın (arayuz.rs /nabiz, 90 sn kuralı).
  function nabiz() { post('/nabiz', '').catch(function () {}); }
  setInterval(nabiz, 20000); nabiz();
  if (kanal) kanal.onmessage = function (e) { if (e.data && e.data.tur === 'canli') { canli = e.data.durum; canliYas = 0; canliCiz(); } };

  /* ---------------------------------------------------------------- Eylemler */
  function katmanEkle() {
    if (proje.katmanlar.length >= M.SINIR.katman) return;
    degistir(function () { var id = M.yeniId(proje); proje.katmanlar.push(M.varsayilanKatman(id, 'Katman ' + (proje.katmanlar.length + 1))); seciliId = id; });
  }
  function katmanTasi(yon) {
    var k = secili(); if (!k) return;
    var i = proje.katmanlar.indexOf(k), j = i + yon;
    if (j < 0 || j >= proje.katmanlar.length) return;
    degistir(function () { proje.katmanlar.splice(j, 0, proje.katmanlar.splice(i, 1)[0]); });
  }
  function oynatDuraklat() { var k = secili(); if (k) degistir(function () { k.oynuyor = !k.oynuyor; }); }
  function hepsi() { var o = proje.katmanlar.some(function (k) { return k.oynuyor; }); degistir(function () { proje.katmanlar.forEach(function (k) { k.oynuyor = !o; }); }); }
  function karart() { degistir(function () { proje.karart = !proje.karart; }); }
  async function cikisAcKapa() {
    var bagli = canliYas !== null && canliYas < 5000;
    var ac = !bagli;
    var r = await post('/oynatici/cikis', { ac: ac, monitor: proje.cikis.monitor, x: proje.cikis.x, y: proje.cikis.y }).catch(function () { return null; });
    if (r && r.ok) bildir(ac ? 'Çıkış penceresi açılıyor.' : 'Çıkış penceresi kapatıldı.');
    else bildir('Çıkış penceresi bu oturumda açılamıyor (pencere köprüsü bağlı değil). “Çıkış ayarı” altındaki bağlantıyla deneyin.');
  }
  function ayarBagla(id, alan, donustur) {
    $(id).addEventListener('change', function () { var k = secili(); if (k) degistir(function () { k[alan] = donustur ? donustur($(id).value) : $(id).value; }); });
  }
  M.HIZLAR.forEach(function (h) { var o = el('option', '', String(h).replace('.', ',') + '×'); o.value = String(h); $('hiz').appendChild(o); });
  ayarBagla('kip', 'kip'); ayarBagla('bitince', 'bitince'); ayarBagla('hiz', 'hiz', Number); ayarBagla('karisim', 'karisim');
  ayarBagla('hedef', 'hedef'); ayarBagla('sigdir', 'sigdir');
  $('gecis').addEventListener('input', function () { $('gecisYazi').textContent = $('gecis').value + ' ms'; });
  ayarBagla('gecis', 'gecisMs', Number);
  ['opaklik', 'ses'].forEach(function (a) {
    var inp = $(a);
    inp.addEventListener('input', function () { var k = secili(); if (!k) return; k[a] = inp.value / 100; $(a + 'Yazi').textContent = inp.value; if (kanal) kanal.postMessage({ tur: 'ara', id: k.id, alan: a, deger: k[a] }); else if (Date.now() - sonAra > 100) { sonAra = Date.now(); kaydetSimdi(); } });
    inp.addEventListener('change', function () { var k = secili(); if (k) degistir(function () { k[a] = inp.value / 100; }); });
  });
  $('sessiz').addEventListener('change', function () { var k = secili(); if (k) degistir(function () { k.sessiz = $('sessiz').checked; }); });
  $('katmanAd').addEventListener('change', function () { var k = secili(); if (k) degistir(function () { k.ad = $('katmanAd').value.trim() || k.ad; }); });
  $('katmanEkle').addEventListener('click', katmanEkle);
  $('katYukari').addEventListener('click', function () { katmanTasi(1); });
  $('katAsagi').addEventListener('click', function () { katmanTasi(-1); });
  $('katCogalt').addEventListener('click', function () {
    var k = secili(); if (!k || proje.katmanlar.length >= M.SINIR.katman) return;
    degistir(function () { var c = JSON.parse(JSON.stringify(k)); c.id = M.yeniId(proje); c.ad = (k.ad + ' kopya').slice(0, 200); proje.katmanlar.splice(proje.katmanlar.indexOf(k) + 1, 0, c); seciliId = c.id; });
  });
  $('katSil').addEventListener('click', function () {
    var k = secili(); if (!k) return;
    var i = proje.katmanlar.indexOf(k);
    degistir(function () { proje.katmanlar.splice(i, 1); seciliId = proje.katmanlar.length ? proje.katmanlar[Math.min(i, proje.katmanlar.length - 1)].id : null; });
    bildir('“' + k.ad + '” katmanı çıkarıldı.', function () { degistir(function () { proje.katmanlar.splice(Math.min(i, proje.katmanlar.length), 0, k); seciliId = k.id; }); });
  });
  $('oynatBtn').addEventListener('click', oynatDuraklat);
  $('oncekiBtn').addEventListener('click', function () { komut('onceki'); });
  $('sonrakiBtn').addEventListener('click', function () { komut('sonraki'); });
  $('geriBtn').addEventListener('click', function () { komut('geri'); });
  $('ileriBtn').addEventListener('click', function () { komut('ileri'); });
  $('basaBtn').addEventListener('click', function () { komut('basa'); });
  $('konum').addEventListener('input', function () { konumSurukleniyor = true; var k = secili(), cl = k && canliKatman(k); if (cl) $('konumYazi').textContent = M.sureYazi($('konum').value / 1000 * cl.sure); });
  $('konum').addEventListener('change', function () { konumSurukleniyor = false; var k = secili(), cl = k && canliKatman(k); if (cl && cl.sure) komut('sar', Math.round($('konum').value / 1000 * cl.sure * 100) / 100); });
  $('hepsiBtn').addEventListener('click', hepsi);
  $('karartBtn').addEventListener('click', karart);
  $('cikisBtn').addEventListener('click', cikisAcKapa);
  $('kompozisyon').addEventListener('change', function () { degistir(function () { proje.ekranSayisi = +$('kompozisyon').value; }); });
  $('monitor').addEventListener('change', function () { degistir(function () { proje.cikis.monitor = +$('monitor').value; }); });
  $('cikisX').addEventListener('change', function () { degistir(function () { proje.cikis.x = Math.trunc(+$('cikisX').value || 0); }); });
  $('cikisY').addEventListener('change', function () { degistir(function () { proje.cikis.y = Math.trunc(+$('cikisY').value || 0); }); });
  $('acilistaAc').addEventListener('change', function () { degistir(function () { proje.acilistaCikisAc = $('acilistaAc').checked; }); });
  // Ana sayfa oynatıcıyı ayrı pencerede açar (index.html window.open): o durumda ikinci bir ana sayfa yüklemek yerine açan pencereye dön.
  $('anaSayfa').addEventListener('click', function (e) {
    try { if (window.opener && !window.opener.closed) { e.preventDefault(); window.opener.focus(); window.close(); } } catch (x) { /* aynı pencerede gezin */ }
  });
  $('medyaYenile').addEventListener('click', medyaYukle);
  $('ara').addEventListener('input', medyaCiz);
  $('geriAl').addEventListener('click', function () { var f = geriAlVeri; $('bildirim').hidden = true; geriAlVeri = null; if (f) f(); });
  var klipListe = $('klipListe');
  klipListe.addEventListener('dragover', function (e) { e.preventDefault(); klipListe.classList.add('birak'); });
  klipListe.addEventListener('dragleave', function () { klipListe.classList.remove('birak'); });
  klipListe.addEventListener('drop', function (e) { e.preventDefault(); klipListe.classList.remove('birak'); var k = secili(), ad = e.dataTransfer.getData('text/plain'); if (k && ad) klipEkle(k, ad); });

  /* Sahne pişirme: senaryo → mp4 klip (oynatici-sahne-<zaman>.mp4), bitince seçili katmana eklenir. */
  var sahneZaman = null;
  $('sahneBtn').addEventListener('click', async function () {
    var r = await post('/oynatici/sahne', $('sahneMetin').value).catch(function () { return null; });
    if (!r) return;
    if (!r.ok) { $('sahneDurum').textContent = await r.text(); return; }
    $('sahneIptal').hidden = false; $('sahneBtn').disabled = true;
    $('sahneDurum').textContent = 'Pişiriliyor… Bu sırada oynatma takılabilir.';
    clearInterval(sahneZaman);
    sahneZaman = setInterval(async function () {
      var j = await (await fetch('/oynatici/sahne', { cache: 'no-store' })).json();
      if (j.calisiyor) { $('sahneDurum').textContent = 'Pişiriliyor: ' + j.kare + ' / ' + (j.toplam || '?') + ' kare'; return; }
      clearInterval(sahneZaman); $('sahneIptal').hidden = true; $('sahneBtn').disabled = false;
      if (j.hata) { $('sahneDurum').textContent = 'Olmadı: ' + j.mesaj; return; }
      $('sahneDurum').textContent = 'Hazır: ' + j.dosya;
      await medyaYukle();
      var k = secili(); if (k) klipEkle(k, j.dosya);
    }, 700);
  });
  $('sahneIptal').addEventListener('click', function () { post('/oynatici/sahne-iptal', ''); });

  /* ---------------------------------------------------------------- Kısayollar (event.code; yazı alanında kapalı) */
  var KISAYOL = [
    ['Boşluk', 'Seçili katmanı oynat/duraklat'], ['Shift+Boşluk', 'Hepsini oynat/duraklat'], ['Sağ / Sol ok', 'Sonraki / önceki klip'],
    ['Shift+Sağ / Shift+Sol', '5 sn ileri / geri'], ['Home', 'Listeyi başa al'], ['Yukarı / Aşağı ok', 'Opaklık +5 / −5'], ['1–8', 'Katman seç'],
    ['Shift+1–8', 'Katmanı aç/kapat'], ['A', 'Seçili katmanı aç/kapat'], ['S', 'Solo'], ['M', 'Sessiz'], ['B', 'Karart'], ['N', 'Yeni katman'],
    ['Ctrl+Yukarı / Aşağı', 'Katmanı öne / arkaya al'], ['Delete', 'Etkin klibi listeden çıkar (dosya silinmez)'], ['O', 'Çıkışı aç/kapat'], ['?', 'Bu liste']];
  KISAYOL.forEach(function (s) { var tr = el('tr'); tr.append(el('th', '', s[0]), el('td', '', s[1])); $('kisayolListe').appendChild(tr); });
  $('yardimBtn').addEventListener('click', function () { $('yardim').showModal(); });
  $('yardimKapat').addEventListener('click', function () { $('yardim').close(); });
  document.addEventListener('keydown', function (e) {
    var t = e.target;
    if (t && (t.tagName === 'INPUT' && t.type !== 'range' && t.type !== 'checkbox' || t.tagName === 'TEXTAREA' || t.tagName === 'SELECT' || t.isContentEditable)) return;
    if ($('yardim').open || !proje) return;
    var k = secili(), c = e.code, ok = true;
    if (c === 'Space') { if (e.shiftKey) hepsi(); else oynatDuraklat(); }
    else if (c === 'ArrowRight' && !e.ctrlKey) komut(e.shiftKey ? 'ileri' : 'sonraki');
    else if (c === 'ArrowLeft' && !e.ctrlKey) komut(e.shiftKey ? 'geri' : 'onceki');
    else if (c === 'Home') komut('basa');
    else if ((c === 'ArrowUp' || c === 'ArrowDown') && e.ctrlKey) katmanTasi(c === 'ArrowUp' ? 1 : -1);
    else if ((c === 'ArrowUp' || c === 'ArrowDown') && k && !(t && t.type === 'range')) degistir(function () { k.opaklik = Math.max(0, Math.min(1, Math.round(k.opaklik * 100 + (c === 'ArrowUp' ? 5 : -5)) / 100)); });
    else if (/^Digit[1-8]$/.test(c)) {
      var n = +c.slice(5) - 1, kat = proje.katmanlar[n];
      if (!kat) ok = false; else if (e.shiftKey) degistir(function () { kat.acik = !kat.acik; }); else { seciliId = kat.id; ciz(); }
    }
    else if (c === 'KeyA' && k) degistir(function () { k.acik = !k.acik; });
    else if (c === 'KeyS' && k) degistir(function () { k.solo = !k.solo; });
    else if (c === 'KeyM' && k) degistir(function () { k.sessiz = !k.sessiz; });
    else if (c === 'KeyB') karart();
    else if (c === 'KeyN') katmanEkle();
    else if (c === 'KeyO') cikisAcKapa();
    else if (c === 'Delete' && k && k.klipler.length) { var cl = canliKatman(k); klipCikar(k, Math.min(cl ? cl.etkin : k.etkin, k.klipler.length - 1)); }
    else if (e.key === '?') $('yardim').showModal();
    else ok = false;
    if (ok) e.preventDefault();
  });

  async function monitorYukle() {
    try {
      var r = await fetch('/oynatici/monitorler', { cache: 'no-store' });
      if (!r.ok) { $('pencereNot').textContent = 'Pencere köprüsü bu oturumda yok: çıkışı aşağıdaki bağlantıyla bu pencerede deneyebilirsiniz.'; return; }
      var v = await r.json(), s = $('monitor');
      s.textContent = '';
      v.forEach(function (m) { var o = el('option', '', (m.no + 1) + '. ' + (m.ad || 'Monitör') + ' (' + m.gen + 'x' + m.yuk + ')'); o.value = String(m.no); s.appendChild(o); });
      if (proje) s.value = String(proje.cikis.monitor);
    } catch (e) { /* yok */ }
  }
  projeYukle().then(function () { medyaYukle(); monitorYukle(); canliYoklama(); }).catch(function () { bildir('Oynatıcı sunucusuna ulaşılamadı.'); });
})();
