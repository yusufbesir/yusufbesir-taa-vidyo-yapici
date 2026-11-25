/* Oynatıcı motoru: DOM'suz saf mantık (sıradaki klip, torba, bitince, solo, hedef kutuları, proje doğrulama).
   Çıkış sayfası, kontrol sayfası ve node testi (tests/oynatici_test.js) aynı dosyayı kullanır.
   Sahip Yusuf; yazan [Claude#Oynatici Opus 5.5, yenileme-1.0, 2026-11-25]; yalnız yazan yapay zekâ ya da Yusuf'un izniyle değişir; silme yok.
   Karar: tasarim/yenileme/karar-oynatici.md 1.1-1.3. Sınırlar src/oynatici_http.rs ile aynıdır. */
(function (kok) {
  'use strict';
  var SINIR = { katman: 8, klip: 200, metin: 200 };
  var KIPLER = ['sirali', 'dongu', 'rastgele'];
  var BITINCE = ['basa', 'dur', 'kapat'];
  var KARISIM = ['normal', 'ekle', 'carp'];
  var HEDEF = ['ikisi', 'sol', 'sag', 'yayili'];
  var SIGDIR = ['doldur', 'sigdir', 'esnet'];
  var HIZLAR = [0.25, 0.5, 0.75, 1, 1.25, 1.5, 2];
  var UZANTI = { mp4: 'video', webm: 'video', png: 'resim', jpg: 'resim', jpeg: 'resim', webp: 'resim' };

  function medyaTuru(ad) {
    var i = String(ad).lastIndexOf('.');
    return i < 0 ? null : (UZANTI[String(ad).slice(i + 1).toLowerCase()] || null);
  }
  function adGecerli(ad) {
    if (typeof ad !== 'string' || !ad || Array.from(ad).length > 200) return false;
    if (/[\/\\:<>"|?*\u0000-\u001f\u007f]/.test(ad)) return false;
    if (ad[0] === '.' || ad.indexOf('..') >= 0 || /[ .]$/.test(ad)) return false;
    return medyaTuru(ad) !== null;
  }
  function kirp(x, a, b, v) { x = typeof x === 'number' && isFinite(x) ? x : v; return Math.min(b, Math.max(a, x)); }
  function secim(x, liste, alan) {
    if (x === undefined || x === null) return liste[0];
    if (liste.indexOf(x) < 0) throw new Error(alan + ' geçersiz');
    return x;
  }
  function metin(s, v) {
    s = typeof s === 'string' ? Array.from(s.replace(/[\u0000-\u001f\u007f]/g, '')).slice(0, SINIR.metin).join('') : '';
    return s.trim() ? s : v;
  }
  function varsayilanKatman(id, ad) {
    return { id: id, ad: ad || ('Katman ' + id), acik: true, solo: false, opaklik: 1, karisim: 'normal', hedef: 'ikisi', sigdir: 'doldur',
      kip: 'sirali', bitince: 'basa', hiz: 1, ses: 1, sessiz: true, gecisMs: 0, etkin: 0, oynuyor: true, klipler: [] };
  }
  function varsayilanProje() {
    return { ekranGen: 950, ekranYuk: 1800, ekranSayisi: 2, anaSes: 1, karart: false, acilistaCikisAc: false,
      cikis: { monitor: 0, x: 0, y: 0 }, katmanlar: [varsayilanKatman(1, 'Katman 1')] };
  }
  /* Sunucudaki dogrula() ile aynı kurallar: hata fırlatır ya da normalleştirilmiş kopya döndürür. */
  function dogrula(p) {
    if (!p || typeof p !== 'object' || Array.isArray(p)) throw new Error('proje nesne olmalı');
    var kat = p.katmanlar === undefined ? [] : p.katmanlar;
    if (!Array.isArray(kat)) throw new Error('katmanlar dizi olmalı');
    if (kat.length > SINIR.katman) throw new Error('en çok ' + SINIR.katman + ' katman');
    var c = p.cikis || {};
    var o = {
      ekranGen: Math.round(kirp(p.ekranGen, 64, 4096, 950)), ekranYuk: Math.round(kirp(p.ekranYuk, 64, 4096, 1800)),
      ekranSayisi: kirp(p.ekranSayisi, 1, 2, 2) < 1.5 ? 1 : 2, anaSes: kirp(p.anaSes, 0, 1, 1), karart: p.karart === true,
      acilistaCikisAc: p.acilistaCikisAc === true,
      cikis: { monitor: Math.trunc(kirp(c.monitor, 0, 16, 0)), x: Math.trunc(kirp(c.x, -20000, 20000, 0)), y: Math.trunc(kirp(c.y, -20000, 20000, 0)) },
      katmanlar: []
    };
    var kullanilan = [];
    kat.forEach(function (k, i) {
      if (!k || typeof k !== 'object') throw new Error('katman nesne olmalı');
      var id = typeof k.id === 'number' && k.id >= 1 && k.id <= 2147483647 ? Math.trunc(k.id) : 0;
      if (!id || kullanilan.indexOf(id) >= 0) { id = 1; while (kullanilan.indexOf(id) >= 0 || kat.some(function (q) { return q && q.id === id; })) id++; }
      kullanilan.push(id);
      var kl = k.klipler === undefined ? [] : k.klipler;
      if (!Array.isArray(kl)) throw new Error('klipler dizi olmalı');
      if (kl.length > SINIR.klip) throw new Error('katman başına en çok ' + SINIR.klip + ' klip');
      var klipler = kl.map(function (x) {
        var ad = x && typeof x.ad === 'string' ? x.ad : '';
        if (!adGecerli(ad)) throw new Error('klip adı geçersiz: ' + ad);
        var tur = secim(x.tur, ['video', 'resim'], 'klip türü');
        if (medyaTuru(ad) !== tur) throw new Error('klip türü uzantıyla uyuşmuyor: ' + ad);
        return tur === 'resim' ? { tur: tur, ad: ad, sure: kirp(x.sure, 1, 3600, 8) } : { tur: tur, ad: ad };
      });
      var etkin = Math.trunc(kirp(k.etkin, 0, SINIR.klip, 0));
      o.katmanlar.push({
        id: id, ad: metin(k.ad, 'Katman ' + (i + 1)), acik: k.acik !== false, solo: k.solo === true, opaklik: kirp(k.opaklik, 0, 1, 1),
        karisim: secim(k.karisim, KARISIM, 'karışım'), hedef: secim(k.hedef, HEDEF, 'hedef'), sigdir: secim(k.sigdir, SIGDIR, 'sığdırma'),
        kip: secim(k.kip, KIPLER, 'kip'), bitince: secim(k.bitince, BITINCE, 'bitince'), hiz: kirp(k.hiz, 0.25, 2, 1), ses: kirp(k.ses, 0, 1, 1),
        sessiz: k.sessiz !== false, gecisMs: Math.round(kirp(k.gecisMs, 0, 2000, 0)), etkin: klipler.length ? Math.min(etkin, klipler.length - 1) : 0,
        oynuyor: k.oynuyor !== false, klipler: klipler
      });
    });
    return o;
  }
  function yeniId(p) { var id = 1; while (p.katmanlar.some(function (k) { return k.id === id; })) id++; return id; }
  /* Solo kuralı: herhangi bir katmanda solo açıksa yalnız solo katmanlar çizilir. */
  function gorunurMu(p, k) {
    if (!k.acik) return false;
    var solo = p.katmanlar.some(function (x) { return x.solo; });
    return !solo || k.solo;
  }
  function adaylar(k, eksik) {
    var v = [];
    for (var i = 0; i < k.klipler.length; i++) if (!(eksik && eksik.has && eksik.has(k.klipler[i].ad))) v.push(i);
    return v;
  }
  function karistir(d, rnd) {
    rnd = rnd || Math.random;
    for (var i = d.length - 1; i > 0; i--) { var j = Math.floor(rnd() * (i + 1)); var t = d[i]; d[i] = d[j]; d[j] = t; }
    return d;
  }
  function ilkAday(k, eksik) { var a = adaylar(k, eksik); return a.length ? a[0] : -1; }
  /* Sıradaki klip. durum: {etkin, torba?}. Dönüş: {etkin, eylem: 'oynat'|'dur'|'kapat', torba}.
     sirali: liste sonu bitince'ye göre (basa = ilk klibe döner). rastgele: torba; tur bitmeden aynı klip gelmez. */
  function sonraki(k, durum, eksik, rnd) {
    var a = adaylar(k, eksik);
    var e = durum && typeof durum.etkin === 'number' ? durum.etkin : -1;
    if (!a.length) return { etkin: e, eylem: 'kapat', torba: [] };
    if (k.kip === 'dongu') {
      if (a.indexOf(e) >= 0) return { etkin: e, eylem: 'oynat', torba: [] };
      var s = a.filter(function (i) { return i > e; });
      return { etkin: s.length ? s[0] : a[0], eylem: 'oynat', torba: [] };
    }
    if (k.kip === 'rastgele') {
      var torba = durum && Array.isArray(durum.torba) ? durum.torba.filter(function (i) { return a.indexOf(i) >= 0 && i !== e; }) : null;
      if (!torba) torba = karistir(a.filter(function (i) { return i !== e; }), rnd);
      if (!torba.length) {
        if (durum && Array.isArray(durum.torba) && k.bitince !== 'basa') return { etkin: e, eylem: k.bitince === 'dur' ? 'dur' : 'kapat', torba: [] };
        torba = karistir(a.filter(function (i) { return i !== e || a.length === 1; }), rnd);
      }
      var n = torba.pop();
      return { etkin: n, eylem: 'oynat', torba: torba };
    }
    var ileri = a.filter(function (i) { return i > e; });
    if (ileri.length) return { etkin: ileri[0], eylem: 'oynat', torba: [] };
    if (k.bitince === 'dur') return { etkin: e, eylem: 'dur', torba: [] };
    if (k.bitince === 'kapat') return { etkin: e, eylem: 'kapat', torba: [] };
    return { etkin: a[0], eylem: 'oynat', torba: [] };
  }
  /* "Geri" = önceki klip (tersine oynatma yok). Liste başında sona sarar. */
  function onceki(k, etkin, eksik) {
    var a = adaylar(k, eksik);
    if (!a.length) return -1;
    var g = a.filter(function (i) { return i < etkin; });
    return g.length ? g[g.length - 1] : a[a.length - 1];
  }
  /* Katmanın çizim kutuları (kompozisyon pikseli). tek kipte hedef yok sayılır. */
  function kutular(p, k) {
    var g = p.ekranGen, y = p.ekranYuk;
    if (p.ekranSayisi === 1) return [{ x: 0, y: 0, w: g, h: y }];
    switch (k.hedef) {
      case 'sol': return [{ x: 0, y: 0, w: g, h: y }];
      case 'sag': return [{ x: g, y: 0, w: g, h: y }];
      case 'yayili': return [{ x: 0, y: 0, w: 2 * g, h: y }];
      default: return [{ x: 0, y: 0, w: g, h: y }, { x: g, y: 0, w: g, h: y }];
    }
  }
  function karisimCss(k, artiVar) {
    if (k.karisim === 'carp') return 'multiply';
    if (k.karisim === 'ekle') return artiVar ? 'plus-lighter' : 'screen';
    return 'normal';
  }
  function sigdirCss(k) { return k.sigdir === 'sigdir' ? 'contain' : k.sigdir === 'esnet' ? 'fill' : 'cover'; }
  function sureYazi(sn) {
    if (!isFinite(sn) || sn < 0) sn = 0;
    sn = Math.floor(sn);
    return Math.floor(sn / 60) + ':' + String(sn % 60).padStart(2, '0');
  }
  var api = { SINIR: SINIR, KIPLER: KIPLER, BITINCE: BITINCE, KARISIM: KARISIM, HEDEF: HEDEF, SIGDIR: SIGDIR, HIZLAR: HIZLAR,
    medyaTuru: medyaTuru, adGecerli: adGecerli, varsayilanKatman: varsayilanKatman, varsayilanProje: varsayilanProje, dogrula: dogrula,
    yeniId: yeniId, gorunurMu: gorunurMu, adaylar: adaylar, karistir: karistir, ilkAday: ilkAday, sonraki: sonraki, onceki: onceki,
    kutular: kutular, karisimCss: karisimCss, sigdirCss: sigdirCss, sureYazi: sureYazi };
  if (typeof module === 'object' && module.exports) module.exports = api; else kok.OynaticiMotor = api;
})(typeof window !== 'undefined' ? window : this);
