/* Oynatıcı çıkış motoru: DOM birleştirici (üst üste <video>/<img> + opacity + mix-blend-mode), A/B geçiş, kipler, komutlar, canlı durum.
   Sahip Yusuf; yazan [Claude#Oynatici Opus 5.5, yenileme-1.0, 2026-11-25]; yalnız yazan yapay zekâ ya da Yusuf'un izniyle değişir; silme yok.
   Karar: tasarim/yenileme/karar-oynatici.md 2.3-2.7. Tek doğruluk kaynağı sunucu (uzun yoklama); BroadcastChannel yalnız hızlandırıcı.
   Ekranda hiçbir zaman hata yazısı çıkmaz; hatalı klip atlanır ve canlı duruma yazılır. Metin yalnız textContent ile. */
(function () {
  'use strict';
  var M = window.OynaticiMotor;
  var sahne = document.getElementById('sahne');
  var karartEl = document.getElementById('karart');
  var proje = null, surum = null, komutNo = null;
  var katmanlar = new Map();
  var olaylar = [];
  var artiVar = !!(window.CSS && CSS.supports && CSS.supports('mix-blend-mode', 'plus-lighter'));
  var uyanik = false, kilitNesne = null;
  var kanal = null;
  try { kanal = new BroadcastChannel('taa-oynatici'); } catch (e) { kanal = null; }

  function olay(m) { olaylar.push(String(m).slice(0, 300)); if (olaylar.length > 20) olaylar.shift(); }
  function medyaUrl(ad) { return '/oynatici/medya/' + encodeURIComponent(ad); }
  function bekle(ms) { return new Promise(function (r) { setTimeout(r, ms); }); }

  function olcekle() {
    if (!proje) return;
    var g = proje.ekranGen * proje.ekranSayisi, y = proje.ekranYuk;
    sahne.style.width = g + 'px';
    sahne.style.height = y + 'px';
    // 1:1 fiziksel piksel: Windows ölçeği %125 olsa bile 950 piksel 950 LED pikseline düşer
    sahne.style.transform = 'scale(' + (1 / (window.devicePixelRatio || 1)) + ')';
  }
  window.addEventListener('resize', olcekle);

  /* ---------------------------------------------------------------- Katman oynatıcısı */
  function KatmanOyn(id) {
    this.id = id;
    this.kok = document.createElement('div');
    this.kok.className = 'katman';
    sahne.insertBefore(this.kok, karartEl);
    this.kutular = [];     // [{el, ogeler: [öğe]}]
    this.kutuAnahtar = '';
    this.k = null;
    this.etkin = 0;
    this.klipAd = null;
    this.torba = null;
    this.eksik = new Set();
    this.zaman = null;      // resim zamanlayıcısı
    this.resimBas = 0; this.resimKalan = 0;
    this.bekleme = 0;       // takılma başlangıcı
    this.bitti = false;     // dur/kapat sonrası
    this.kapali = false;
    this.nesil = 0;
  }
  KatmanOyn.prototype.yoket = function () { this.temizle(); this.kok.remove(); };
  KatmanOyn.prototype.temizle = function () {
    clearTimeout(this.zaman);
    this.kutular.forEach(function (k) { k.ogeler.forEach(function (o) { if (o.tagName === 'VIDEO') { o.pause(); o.removeAttribute('src'); o.load(); } o.remove(); }); k.ogeler = []; });
  };
  KatmanOyn.prototype.kutuKur = function (rects) {
    this.temizle();
    this.kok.textContent = '';
    this.kutular = rects.map(function (r) {
      var el = document.createElement('div');
      el.className = 'kutu';
      el.style.left = r.x + 'px'; el.style.top = r.y + 'px'; el.style.width = r.w + 'px'; el.style.height = r.h + 'px';
      return { el: el, ogeler: [] };
    });
    var kok = this.kok;
    this.kutular.forEach(function (k) { kok.appendChild(k.el); });
  };
  KatmanOyn.prototype.gorunur = function () { return proje && this.k && M.gorunurMu(proje, this.k) && this.k.opaklik > 0 && !this.kapali; };
  KatmanOyn.prototype.oynamali = function () { return this.gorunur() && this.k.oynuyor && !this.bitti; };
  KatmanOyn.prototype.etkinOgeler = function () { return this.kutular.map(function (k) { return k.ogeler[k.ogeler.length - 1]; }).filter(Boolean); };
  KatmanOyn.prototype.anaVideo = function () { var o = this.etkinOgeler()[0]; return o && o.tagName === 'VIDEO' ? o : null; };

  KatmanOyn.prototype.uygula = function (k) {
    var eski = this.k;
    this.k = k;
    var rects = M.kutular(proje, k);
    var anahtar = JSON.stringify(rects);
    var kutuDegisti = anahtar !== this.kutuAnahtar;
    if (kutuDegisti) { this.kutuAnahtar = anahtar; this.kutuKur(rects); }
    this.kok.style.opacity = this.gorunur() || this.kapali ? String(this.kapali ? 0 : k.opaklik) : '0';
    this.kok.style.mixBlendMode = M.karisimCss(k, artiVar);
    var fit = M.sigdirCss(k);
    this.kutular.forEach(function (b) { b.el.dataset.sigdir = fit; b.ogeler.forEach(function (o) { o.style.objectFit = fit; }); });
    // Klip listesi değiştiyse etkin klibi adıyla izle; çıkışın etkin sırası kontrol sayfasınca ezilmez
    var yukle = kutuDegisti || this.klipAd === null;
    if (!k.klipler.length) { this.klipAd = null; this.temizle(); return; }
    if (!yukle) {
      var su = k.klipler[this.etkin];
      if (!su || su.ad !== this.klipAd) {
        var bul = -1;
        for (var i = 0; i < k.klipler.length; i++) if (k.klipler[i].ad === this.klipAd) { bul = i; break; }
        if (bul >= 0) this.etkin = bul; else { this.etkin = Math.min(this.etkin, k.klipler.length - 1); yukle = true; }
      }
    } else if (this.klipAd === null) this.etkin = Math.min(k.etkin, k.klipler.length - 1);
    if (yukle) { this.bitti = false; this.kapali = false; this.yukle(this.etkin, false); return; }
    if (eski && (eski.kip !== k.kip || eski.hiz !== k.hiz || eski.bitince !== k.bitince)) this.torba = null;
    this.ozellikler();
  };
  KatmanOyn.prototype.ozellikler = function () {
    var k = this.k, self = this;
    var calis = this.oynamali();
    this.etkinOgeler().forEach(function (o) {
      if (o.tagName !== 'VIDEO') return;
      o.loop = k.kip === 'dongu';
      o.playbackRate = k.hiz;
      o.muted = k.sessiz;
      o.volume = Math.max(0, Math.min(1, k.ses * proje.anaSes));
      if (calis && o.paused) { var p = o.play(); if (p && p.catch) p.catch(function () {}); }
      if (!calis && !o.paused) o.pause();
    });
    // ikisi hedefinde yalnız ilk kopya ses verir
    var v = this.etkinOgeler(); for (var i = 1; i < v.length; i++) if (v[i].tagName === 'VIDEO') v[i].muted = true;
    var cur = k.klipler[this.etkin];
    if (cur && cur.tur === 'resim') {
      if (calis && !this.zaman) this.resimZamanla(this.resimKalan || cur.sure * 1000 / k.hiz);
      if (!calis && this.zaman) { clearTimeout(this.zaman); this.zaman = null; this.resimKalan = Math.max(0, this.resimKalan - (Date.now() - this.resimBas)); }
    }
    self.kok.style.opacity = this.gorunur() ? String(k.opaklik) : '0';
  };
  KatmanOyn.prototype.resimZamanla = function (ms) {
    var self = this;
    clearTimeout(this.zaman);
    this.resimBas = Date.now(); this.resimKalan = ms;
    if (this.k.kip === 'dongu') { this.zaman = null; return; }
    this.zaman = setTimeout(function () { self.zaman = null; self.resimKalan = 0; self.ilerle(); }, Math.max(200, ms));
  };
  /* Klibi her kutuda yeni öğeye yükler; hazır olunca eskisinin üstünde belirir (gecisMs ile çapraz solma). Siyah kare yok: eski öğe yenisi hazır olana dek kalır. */
  KatmanOyn.prototype.yukle = function (i, gecis) {
    var k = this.k, self = this;
    var klip = k.klipler[i];
    if (!klip) return;
    clearTimeout(this.zaman); this.zaman = null; this.resimKalan = 0;
    this.etkin = i; this.klipAd = klip.ad; this.bekleme = 0;
    var nesil = ++this.nesil;
    var sure = gecis ? k.gecisMs : 0;
    var hazir = 0, gerek = this.kutular.length, yeni = [];
    var fit = M.sigdirCss(k);
    this.kutular.forEach(function (b) {
      var o;
      if (klip.tur === 'video') {
        o = document.createElement('video');
        o.muted = true; o.playsInline = true; o.preload = 'auto'; o.loop = k.kip === 'dongu';
        o.addEventListener('ended', function () { if (self.nesil === nesil && o === self.anaVideo()) self.ilerle(); });
        o.addEventListener('error', function () { if (self.nesil === nesil) self.hata(klip.ad, 'çözülemedi'); });
        o.addEventListener('waiting', function () { if (self.nesil === nesil && !self.bekleme) self.bekleme = Date.now(); });
        o.addEventListener('playing', function () { self.bekleme = 0; });
        o.addEventListener('canplay', function hz() { o.removeEventListener('canplay', hz); if (self.nesil === nesil && ++hazir === gerek) self.goster(yeni, sure, nesil); });
      } else {
        o = document.createElement('img');
        o.decoding = 'async';
        o.addEventListener('load', function () { if (self.nesil === nesil && ++hazir === gerek) self.goster(yeni, sure, nesil); });
        o.addEventListener('error', function () { if (self.nesil === nesil) self.hata(klip.ad, 'açılamadı'); });
      }
      o.className = 'oge';
      o.style.objectFit = fit;
      o.style.opacity = '0';
      o.style.transitionDuration = sure + 'ms';
      o.src = medyaUrl(klip.ad);
      b.el.appendChild(o);
      b.ogeler.push(o);
      yeni.push(o);
    });
  };
  KatmanOyn.prototype.goster = function (yeni, sure, nesil) {
    var self = this, k = this.k;
    yeni.forEach(function (o) {
      if (o.tagName === 'VIDEO') {
        o.playbackRate = k.hiz; o.muted = k.sessiz; o.volume = Math.max(0, Math.min(1, k.ses * proje.anaSes));
        if (self.oynamali()) { var p = o.play(); if (p && p.catch) p.catch(function () { o.muted = true; o.play().catch(function () {}); }); }
      }
    });
    for (var i = 1; i < yeni.length; i++) if (yeni[i].tagName === 'VIDEO') yeni[i].muted = true;
    // Bir kare sonra solma başlar; eski öğeler YALNIZ yeni öğe görünür olduktan sonra (geçiş + 60 ms) kaldırılır.
    // (Tarayıcı denemesinde kaldırma zamanlayıcısı rAF'tan önce çalışıp geçişte tek siyah kare bırakıyordu; zincirleme bunu kapatır.)
    // rAF gizli sayfada durur: 250 ms'de gelmezse zamanlayıcı yedeği aynı işi bir kez yapar (öğe birikmesin).
    var basladi = false;
    function belir() {
      if (basladi) return;
      basladi = true;
      if (self.nesil === nesil) yeni.forEach(function (o) { o.style.opacity = '1'; });
      setTimeout(kaldir, sure + 60);
    }
    requestAnimationFrame(belir);
    setTimeout(belir, 250);
    function kaldir() {
      self.kutular.forEach(function (b) {
        b.ogeler = b.ogeler.filter(function (o) {
          if (yeni.indexOf(o) >= 0 || self.nesil !== nesil) return true;
          if (o.tagName === 'VIDEO') { o.pause(); o.removeAttribute('src'); o.load(); }
          o.remove();
          return false;
        });
      });
    }
    var klip = k.klipler[this.etkin];
    if (klip && klip.tur === 'resim') { if (this.oynamali()) this.resimZamanla(klip.sure * 1000 / k.hiz); else this.resimKalan = klip.sure * 1000 / k.hiz; }
    this.ozellikler();
  };
  KatmanOyn.prototype.hata = function (ad, neden) {
    olay('klip atlandı: ' + ad + ' (' + neden + ')');
    this.eksik.add(ad);
    this.ilerle();
  };
  KatmanOyn.prototype.ilerle = function () {
    var r = M.sonraki(this.k, { etkin: this.etkin, torba: this.torba }, this.eksik);
    this.torba = r.torba;
    if (r.eylem === 'kapat') { this.kapali = true; this.bitti = true; this.ozellikler(); this.kok.style.opacity = '0'; return; }
    if (r.eylem === 'dur') { this.bitti = true; this.ozellikler(); return; }
    this.bitti = false; this.kapali = false;
    if (r.etkin === this.etkin && this.k.klipler[r.etkin] && this.k.klipler[r.etkin].tur === 'video' && this.anaVideo()) {
      this.etkinOgeler().forEach(function (o) { o.currentTime = 0; var p = o.play(); if (p && p.catch) p.catch(function () {}); });
      return;
    }
    this.yukle(r.etkin, true);
  };
  KatmanOyn.prototype.komut = function (c) {
    var k = this.k;
    if (!k || !k.klipler.length) return;
    this.bitti = false; this.kapali = false;
    if (c.tur === 'sonraki') { var r = M.sonraki(Object.assign({}, k, { bitince: 'basa' }), { etkin: this.etkin, torba: this.torba }, this.eksik); this.torba = r.torba; if (r.eylem === 'oynat') this.yukle(r.etkin, true); }
    else if (c.tur === 'onceki') { var o = M.onceki(k, this.etkin, this.eksik); if (o >= 0) this.yukle(o, true); }
    else if (c.tur === 'basa') { this.torba = null; this.eksik.clear(); var b = M.ilkAday(k, this.eksik); if (b >= 0) this.yukle(b, true); }
    else if (c.tur === 'tetikle') { var t = Math.trunc(c.deger || 0); if (t >= 0 && t < k.klipler.length) { this.eksik.delete(k.klipler[t].ad); this.yukle(t, true); } }
    else if (c.tur === 'sar' || c.tur === 'ileri' || c.tur === 'geri') {
      var v = this.anaVideo(); if (!v) return;
      var hedef = c.tur === 'sar' ? (c.deger || 0) : v.currentTime + (c.tur === 'ileri' ? 5 : -5);
      hedef = Math.max(0, Math.min(isFinite(v.duration) ? v.duration - 0.05 : hedef, hedef));
      this.etkinOgeler().forEach(function (o) { if (o.tagName === 'VIDEO') o.currentTime = hedef; });
    }
    this.ozellikler();
  };
  /* Eşlenik (ikisi) kopyaları 1 kareden fazla kayarsa eşitle; takılan klibi 5 sn sonra atla; düşen kareyi say. */
  KatmanOyn.prototype.denetle = function () {
    var v = this.etkinOgeler().filter(function (o) { return o.tagName === 'VIDEO'; });
    if (v.length > 1 && !v[0].paused) for (var i = 1; i < v.length; i++) if (Math.abs(v[i].currentTime - v[0].currentTime) > 0.04) v[i].currentTime = v[0].currentTime;
    if (this.bekleme && Date.now() - this.bekleme > 5000 && this.oynamali()) { this.bekleme = 0; this.hata(this.klipAd, 'takıldı'); }
  };
  KatmanOyn.prototype.rapor = function () {
    var v = this.anaVideo(), q = v && v.getVideoPlaybackQuality ? v.getVideoPlaybackQuality() : null;
    var klip = this.k && this.k.klipler[this.etkin];
    var konum = 0, sure = 0;
    if (v) { konum = v.currentTime; sure = isFinite(v.duration) ? v.duration : 0; }
    else if (klip && klip.tur === 'resim') { sure = klip.sure / this.k.hiz; konum = this.zaman ? (Date.now() - this.resimBas) / 1000 + (sure - this.resimKalan / 1000) : sure - this.resimKalan / 1000; konum = Math.max(0, Math.min(sure, konum)); }
    return { id: this.id, etkin: this.etkin, ad: klip ? klip.ad : '', konum: Math.round(konum * 100) / 100, sure: Math.round(sure * 100) / 100,
      dusen: q ? q.droppedVideoFrames : 0, kare: q ? q.totalVideoFrames : 0, oynuyor: this.oynamali(), bitti: this.bitti, eksik: Array.from(this.eksik).slice(0, 20) };
  };

  /* ---------------------------------------------------------------- Proje ve komutlar */
  function projeUygula(p) {
    proje = p;
    olcekle();
    var idler = new Set(p.katmanlar.map(function (k) { return k.id; }));
    katmanlar.forEach(function (o, id) { if (!idler.has(id)) { o.yoket(); katmanlar.delete(id); } });
    p.katmanlar.forEach(function (k, i) {
      var o = katmanlar.get(k.id);
      if (!o) { o = new KatmanOyn(k.id); katmanlar.set(k.id, o); }
      o.kok.style.zIndex = String(i + 1);
      o.uygula(k);
    });
    karartEl.style.opacity = p.karart ? '1' : '0';
  }
  function komutUygula(c) {
    if (c.tur === 'yenile') { location.reload(); return; }
    katmanlar.forEach(function (o) { if (c.katman === undefined || c.katman === o.id) o.komut(c); });
  }
  if (kanal) kanal.onmessage = function (e) {
    var m = e.data || {};
    if (m.tur !== 'ara' || !proje) return;
    var o = katmanlar.get(m.id); if (!o || !o.k) return;
    if (m.alan === 'opaklik') { o.k.opaklik = Math.max(0, Math.min(1, +m.deger || 0)); o.ozellikler(); }
    if (m.alan === 'ses') { o.k.ses = Math.max(0, Math.min(1, +m.deger || 0)); o.ozellikler(); }
  };

  async function yoklama() {
    for (;;) {
      try {
        var q = surum === null ? '' : '?surum=' + surum + '&komut=' + komutNo;
        var r = await fetch('/oynatici/olay' + q, { cache: 'no-store' });
        if (!r.ok) { await bekle(1000); continue; }
        var j = await r.json();
        if (j.proje) projeUygula(j.proje);
        surum = j.surum;
        if (komutNo !== null) j.komutlar.forEach(komutUygula);
        komutNo = j.komutNo;
      } catch (e) { await bekle(1000); }
    }
  }
  /* Her saniye canlı durum: kontrol sayfası 5 sn bildirim alamazsa "Çıkış yanıt vermiyor" der. */
  async function bildir() {
    var d = { zaman: Date.now(), artiVar: artiVar, uyanik: uyanik, olcek: window.devicePixelRatio || 1, katmanlar: [], olaylar: olaylar.splice(0) };
    katmanlar.forEach(function (o) { o.denetle(); d.katmanlar.push(o.rapor()); });
    if (kanal) try { kanal.postMessage({ tur: 'canli', durum: d }); } catch (e) { /* yok */ }
    try { await fetch('/oynatici/canli', { method: 'POST', headers: { 'X-LED': '1', 'Content-Type': 'application/json' }, body: JSON.stringify(d) }); } catch (e) { /* sunucu yok */ }
  }
  setInterval(bildir, 1000);
  // Pencere yok (--tarayici-yok) kipinde sunucu 90 sn nabız almazsa kapanır; ekran açıkken nabız ver (arayuz.rs /nabiz).
  function nabiz() { fetch('/nabiz', { method: 'POST', headers: { 'X-LED': '1' }, body: '' }).catch(function () {}); }
  setInterval(nabiz, 20000); nabiz();

  async function uyanikKal() {
    try { if (navigator.wakeLock && document.visibilityState === 'visible') { kilitNesne = await navigator.wakeLock.request('screen'); uyanik = true; kilitNesne.addEventListener('release', function () { uyanik = false; }); } }
    catch (e) { uyanik = false; }
  }
  document.addEventListener('visibilitychange', function () { if (document.visibilityState === 'visible') uyanikKal(); });
  // Acil kapatma: yalnız Ctrl+Shift+Q (Esc pencereyi kapatmaz)
  document.addEventListener('keydown', function (e) {
    if (e.ctrlKey && e.shiftKey && e.code === 'KeyQ') { e.preventDefault(); fetch('/oynatici/cikis', { method: 'POST', headers: { 'X-LED': '1' }, body: '{"ac":false}' }).catch(function () {}); }
  });
  uyanikKal();
  yoklama();
})();
