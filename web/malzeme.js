// Açıklama: Malzeme galerisi (LED içerik kitaplığı) arayüzü. window.TaaMalzeme = { mount(el, app), birakmaHedefiYap(el, app), _ic }.
// Sunucu uçları: GET /malzeme/liste, POST /malzeme/uret, POST /malzeme/kucuk, GET /malzeme/kucuk/<id>. Ayrıntı: docs/MALZEME.md.
// Tema: yalnız CSS değişkenleri (--bg --panel --panel2 --line --tx --mut --ac --onac --ok --uy). Dış bağımlılık yok.
// Sahip: Yusuf. Yazan: [Claude#Malzeme Opus 5.5, yenileme-1.0, 2026-11-18]. Yalnız yazan yapay zekâ ya da Yusuf'un izniyle değişir; silme yok.
(function (kok) {
  'use strict';

  // ---------------------------------------------------------------- saf mantık (Node testi: tests/malzeme_test.js)
  var TR = { 'ç': 'c', 'ğ': 'g', 'ı': 'i', 'i̇': 'i', 'ö': 'o', 'ş': 's', 'ü': 'u', 'â': 'a', 'î': 'i', 'û': 'u' };
  function normal(s) {
    s = String(s == null ? '' : s).replace(/İ/g, 'i').replace(/I/g, 'ı').toLowerCase();
    return s.replace(/i̇|[çğıöşüâîû]/g, function (c) { return TR[c] || c; }).replace(/\s+/g, ' ').trim();
  }
  function filtrele(liste, f) {
    f = f || {};
    var kat = f.kategori || '', q = normal(f.arama || '');
    var parcalar = q ? q.split(' ') : [];
    return (liste || []).filter(function (m) {
      if (kat && m.kategori !== kat) return false;
      if (!parcalar.length) return true;
      var hay = normal([m.id, m.ad, m.aciklama, (m.etiketler || []).join(' ')].join(' '));
      return parcalar.every(function (p) { return hay.indexOf(p) >= 0; });
    });
  }
  function kategoriSayilari(liste) {
    var o = {};
    (liste || []).forEach(function (m) { o[m.kategori] = (o[m.kategori] || 0) + 1; });
    return o;
  }
  function varsayilanDegerler(m) {
    var d = {};
    (m && m.sema || []).forEach(function (s) { d[s.ad] = s.varsayilan; });
    return d;
  }
  function sinirla(x, s) {
    var v = Number(x);
    if (!isFinite(v)) v = Number(s.varsayilan);
    if (typeof s.min === 'number') v = Math.max(s.min, v);
    if (typeof s.max === 'number') v = Math.min(s.max, v);
    return Math.round(v * 100) / 100;
  }
  // Sunucuya gidecek gövde: yalnız şemadaki alanlar, sayılar sınır içinde, metin kırpılmış; tırnak/# sunucuda da temizlenir.
  function istekGovdesi(m, degerler) {
    var g = { id: m.id };
    degerler = degerler || {};
    (m.sema || []).forEach(function (s) {
      var v = degerler.hasOwnProperty(s.ad) ? degerler[s.ad] : s.varsayilan;
      if (s.tur === 'sayi') g[s.ad] = sinirla(v, s);
      else if (s.tur === 'metin') g[s.ad] = String(v == null ? '' : v).replace(/[\u0000-\u001f"#]/g, ' ').replace(/\s+/g, ' ').trim().slice(0, s.max || 120);
      else if (v != null && v !== '') g[s.ad] = String(v);
    });
    return g;
  }
  function bekletici(fn, ms) {
    var z = null;
    var f = function () { var a = arguments, b = this; clearTimeout(z); z = setTimeout(function () { z = null; fn.apply(b, a); }, ms); };
    f.iptal = function () { clearTimeout(z); z = null; };
    return f;
  }

  // ---------------------------------------------------------------- sunucu
  function istek(yol, govde) {
    var o = govde === undefined ? { credentials: 'same-origin' } : { method: 'POST', credentials: 'same-origin', headers: { 'x-led': '1', 'Content-Type': 'application/json' }, body: JSON.stringify(govde) };
    return fetch(yol, o).then(function (r) {
      return r.text().then(function (t) {
        if (!r.ok) { var m = t; try { m = JSON.parse(t).hata || t; } catch (e) { /* düz metin */ } throw new Error(m || ('HTTP ' + r.status)); }
        return t;
      });
    });
  }

  // ---------------------------------------------------------------- görünüm
  var STIL = [
    '.tm{color:var(--tx);font-size:16px;display:flex;flex-direction:column;gap:12px;min-width:0}',
    '.tm *{box-sizing:border-box}',
    '.tm-ara{width:100%;min-height:48px;padding:0 14px;font:inherit;color:var(--tx);background:var(--panel2);border:1px solid var(--line);border-radius:10px}',
    '.tm-cipler{display:flex;flex-wrap:wrap;gap:8px}',
    '.tm-cip{min-height:48px;padding:0 16px;font:inherit;font-size:16px;color:var(--tx);background:var(--panel2);border:1px solid var(--line);border-radius:24px;cursor:pointer}',
    '.tm-cip[aria-pressed="true"]{background:var(--ac);color:var(--onac);border-color:var(--ac)}',
    '.tm-cip small{font-size:13px;color:inherit;opacity:.8;margin-left:6px}',
    '.tm-izgara{display:grid;grid-template-columns:repeat(auto-fill,minmax(132px,1fr));gap:12px}',
    '.tm-kart{display:flex;flex-direction:column;align-items:stretch;gap:6px;padding:8px;min-height:48px;font:inherit;text-align:left;color:var(--tx);background:var(--panel);border:1px solid var(--line);border-radius:12px;cursor:pointer;transition:transform .15s ease,opacity .15s ease}',
    '.tm-kart:hover{transform:translateY(-2px)}',
    '.tm-kart[aria-current="true"]{border-color:var(--ac);outline:2px solid var(--ac);outline-offset:0}',
    '.tm-resim{width:100%;aspect-ratio:190/360;border-radius:8px;background:var(--bg);display:block;object-fit:cover}',
    '.tm-ad{font-size:16px;font-weight:600;line-height:1.25}',
    '.tm-kat{font-size:13px;color:var(--mut)}',
    '.tm-panel{background:var(--panel);border:1px solid var(--line);border-radius:14px;padding:16px;display:grid;grid-template-columns:minmax(0,1fr) 150px;gap:16px}',
    '@media (max-width:640px){.tm-panel{grid-template-columns:1fr}}',
    '.tm-panel h3{margin:0;font-size:20px}',
    '.tm-panel p{margin:0;color:var(--mut);font-size:16px}',
    '.tm-alan{display:flex;flex-direction:column;gap:6px}',
    '.tm-alan label{font-size:16px}',
    '.tm-satir{display:flex;gap:10px;align-items:center}',
    '.tm-satir input[type=range]{flex:1;min-height:48px;accent-color:var(--ac)}',
    '.tm-sayi,.tm-metin,.tm-secim{min-height:48px;padding:0 12px;font:inherit;color:var(--tx);background:var(--panel2);border:1px solid var(--line);border-radius:10px}',
    '.tm-sayi{width:96px}.tm-metin,.tm-secim{width:100%}',
    '.tm-yonler{display:flex;gap:8px;flex-wrap:wrap}',
    '.tm-eylem{display:flex;gap:10px;flex-wrap:wrap;margin-top:6px}',
    '.tm-btn{min-height:48px;padding:0 18px;font:inherit;font-size:16px;color:var(--tx);background:var(--panel2);border:1px solid var(--line);border-radius:10px;cursor:pointer}',
    '.tm-ana{min-height:56px;padding:0 24px;background:var(--ac);color:var(--onac);border-color:var(--ac);font-weight:700}',
    '.tm-durum{font-size:16px;color:var(--mut);min-height:24px}',
    '.tm-durum[data-tur="ok"]{color:var(--ok)}.tm-durum[data-tur="uy"]{color:var(--uy)}',
    '.tm-buyuk{width:100%;aspect-ratio:190/360;border-radius:10px;background:var(--bg)}',
    '.tm-renk{display:inline-block;width:20px;height:20px;border-radius:50%;border:1px solid var(--line);vertical-align:middle}',
    '.tm :focus-visible{outline:3px solid var(--ac);outline-offset:2px}',
    '.tm-bos{color:var(--mut);padding:16px 0}',
    '@media (prefers-reduced-motion:reduce){.tm-kart{transition:none}.tm-kart:hover{transform:none}}'
  ].join('\n');

  function el(tag, ozellik, cocuk) {
    var e = document.createElement(tag);
    if (ozellik) Object.keys(ozellik).forEach(function (k) {
      if (k === 'metin') e.textContent = ozellik[k];
      else if (k === 'sinif') e.className = ozellik[k];
      else if (k.indexOf('on') === 0) e.addEventListener(k.slice(2), ozellik[k]);
      else e.setAttribute(k, ozellik[k]);
    });
    (cocuk || []).forEach(function (c) { if (c) e.appendChild(c); });
    return e;
  }

  var MIME = 'text/x-taa-malzeme';

  function mount(kap, app) {
    app = app || kok.TaaApp || {};
    if (!document.getElementById('tm-stil')) document.head.appendChild(el('style', { id: 'tm-stil', metin: STIL }));
    kap.innerHTML = '';
    var d = { liste: [], kategoriler: [], paletler: [], yonler: [], kategori: '', arama: '', secili: null, degerler: {}, parca: '', onizlemeMetni: '' };
    var kokEl = el('section', { sinif: 'tm', 'aria-label': 'Malzeme galerisi' });
    var ara = el('input', { sinif: 'tm-ara', type: 'search', placeholder: 'Malzeme ara (örn. kalp, sayaç, yutkun)', 'aria-label': 'Malzeme ara' });
    var cipler = el('div', { sinif: 'tm-cipler', role: 'group', 'aria-label': 'Kategoriler' });
    var izgara = el('div', { sinif: 'tm-izgara' });
    var panel = el('div', { sinif: 'tm-panel', hidden: '' });
    var durum = el('div', { sinif: 'tm-durum', role: 'status', 'aria-live': 'polite' });
    kokEl.appendChild(ara); kokEl.appendChild(cipler); kokEl.appendChild(panel); kokEl.appendChild(izgara); kokEl.appendChild(durum);
    kap.appendChild(kokEl);

    function bildir(m, tur) { durum.textContent = m || ''; if (tur) durum.setAttribute('data-tur', tur); else durum.removeAttribute('data-tur'); }

    // Tembel küçük resim yükleme
    var gozcu = ('IntersectionObserver' in kok) ? new IntersectionObserver(function (gir) {
      gir.forEach(function (g) { if (g.isIntersecting) { gozcu.unobserve(g.target); resimYukle(g.target); } });
    }, { rootMargin: '200px' }) : null;
    var onbellek = {};
    function resimYukle(img) {
      var id = img.getAttribute('data-id');
      if (onbellek[id]) { img.src = onbellek[id]; return; }
      istek('/malzeme/kucuk/' + encodeURIComponent(id)).then(function (u) { onbellek[id] = u; img.src = u; }).catch(function () { img.alt = 'Önizleme yok'; });
    }

    function cipCiz() {
      cipler.innerHTML = '';
      var say = kategoriSayilari(d.liste);
      var hepsi = [{ id: '', ad: 'Tümü', adet: d.liste.length }].concat(d.kategoriler.map(function (k) { return { id: k.id, ad: k.ad, adet: say[k.id] || 0 }; }));
      hepsi.forEach(function (k) {
        var b = el('button', { sinif: 'tm-cip', type: 'button', 'aria-pressed': String(d.kategori === k.id), onclick: function () { d.kategori = k.id; cipCiz(); izgaraCiz(); } }, [document.createTextNode(k.ad), el('small', { metin: String(k.adet) })]);
        cipler.appendChild(b);
      });
    }

    function izgaraCiz() {
      izgara.innerHTML = '';
      var l = filtrele(d.liste, { kategori: d.kategori, arama: d.arama });
      if (!l.length) { izgara.appendChild(el('p', { sinif: 'tm-bos', metin: 'Aramaya uygun malzeme yok.' })); return; }
      var katAd = {}; d.kategoriler.forEach(function (k) { katAd[k.id] = k.ad; });
      l.forEach(function (m) {
        var img = el('img', { sinif: 'tm-resim', alt: '', 'data-id': m.id, width: '190', height: '360', loading: 'lazy' });
        var kart = el('button', { sinif: 'tm-kart', type: 'button', draggable: 'true', 'aria-current': String(!!(d.secili && d.secili.id === m.id)), title: m.aciklama,
          onclick: function () { sec(m); },
          ondragstart: function (e) { try { e.dataTransfer.setData(MIME, m.senaryo); e.dataTransfer.setData('text/plain', m.senaryo + '\n'); e.dataTransfer.effectAllowed = 'copy'; } catch (x) { /* eski tarayıcı */ } }
        }, [img, el('span', { sinif: 'tm-ad', metin: m.ad }), el('span', { sinif: 'tm-kat', metin: katAd[m.kategori] || m.kategori })]);
        izgara.appendChild(kart);
        if (gozcu) gozcu.observe(img); else resimYukle(img);
      });
    }

    var buyukResim = el('img', { sinif: 'tm-buyuk', alt: 'Seçili malzemenin önizlemesi' });
    var canli = bekletici(function () { yenile(true); }, 350);

    function yenile(onizleGonder) {
      if (!d.secili) return Promise.resolve();
      var g = istekGovdesi(d.secili, d.degerler);
      var no = (yenile.no = (yenile.no || 0) + 1);
      return istek('/malzeme/uret', g).then(function (t) {
        if (no !== yenile.no) return;
        var j = JSON.parse(t);
        d.parca = j.senaryo; d.onizlemeMetni = j.onizleme;
        bildir('');
        if (onizleGonder && typeof app.onizle === 'function') { try { app.onizle(j.onizleme); } catch (e) { /* uygulama önizlemesi yoksa sessiz */ } }
        istek('/malzeme/kucuk', Object.assign({ t: null }, g)).then(function (u) { if (no === yenile.no) buyukResim.src = u; }).catch(function () {});
      }).catch(function (e) { if (no === yenile.no) bildir(e.message, 'uy'); });
    }

    function alan(s) {
      var kutu = el('div', { sinif: 'tm-alan' });
      var kimlik = 'tm-' + s.ad;
      kutu.appendChild(el('label', { 'for': kimlik, metin: s.etiket }));
      var v = d.degerler[s.ad];
      var degis = function (yeni) { d.degerler[s.ad] = yeni; canli(); };
      if (s.tur === 'sayi') {
        var aralik = el('input', { type: 'range', min: String(s.min), max: String(s.max), step: String(s.adim || 1), value: String(v), 'aria-label': s.etiket });
        var sayi = el('input', { sinif: 'tm-sayi', id: kimlik, type: 'number', min: String(s.min), max: String(s.max), step: String(s.adim || 1), value: String(v), inputmode: 'decimal' });
        aralik.addEventListener('input', function () { sayi.value = aralik.value; degis(Number(aralik.value)); });
        sayi.addEventListener('change', function () { var x = sinirla(sayi.value, s); sayi.value = x; aralik.value = x; degis(x); });
        kutu.appendChild(el('div', { sinif: 'tm-satir' }, [aralik, sayi]));
      } else if (s.tur === 'metin') {
        var m = el('input', { sinif: 'tm-metin', id: kimlik, type: 'text', maxlength: String(s.max || 120), value: v == null ? '' : String(v) });
        m.addEventListener('input', function () { degis(m.value); });
        kutu.appendChild(m);
      } else if (s.tur === 'palet') {
        var sec = el('select', { sinif: 'tm-secim', id: kimlik });
        d.paletler.forEach(function (p) { var o = el('option', { value: p.ad, metin: p.acik + ' (' + p.ad + ')' }); if (p.ad === v) o.selected = true; sec.appendChild(o); });
        var nokta = el('span', { sinif: 'tm-renk', 'aria-hidden': 'true' });
        var boya = function () { var p = d.paletler.filter(function (x) { return x.ad === sec.value; })[0]; if (p) { nokta.style.background = p.yuz; nokta.style.boxShadow = 'inset 0 0 0 4px ' + p.zemin; } };
        sec.addEventListener('change', function () { boya(); degis(sec.value); });
        boya();
        kutu.appendChild(el('div', { sinif: 'tm-satir' }, [sec, nokta]));
      } else if (s.tur === 'yon') {
        var grup = el('div', { sinif: 'tm-yonler', role: 'group', 'aria-label': s.etiket, id: kimlik });
        d.yonler.forEach(function (y) {
          var b = el('button', { sinif: 'tm-cip', type: 'button', 'aria-pressed': String(y.id === v), metin: y.ad, onclick: function () { Array.prototype.forEach.call(grup.children, function (c) { c.setAttribute('aria-pressed', 'false'); }); b.setAttribute('aria-pressed', 'true'); degis(y.id); } });
          grup.appendChild(b);
        });
        kutu.appendChild(grup);
      }
      return kutu;
    }

    function ekle() {
      var p = d.parca ? Promise.resolve() : yenile(false);
      return p.then(function () {
        if (!d.parca) return;
        if (typeof app.senaryoyaEkle === 'function') { app.senaryoyaEkle(d.parca); bildir('"' + d.secili.ad + '" videoya eklendi.', 'ok'); }
        else bildir('Uygulama bağlantısı yok: senaryo parçası panoya kopyalanabilir.', 'uy');
      });
    }

    function sec(m) {
      d.secili = m; d.degerler = varsayilanDegerler(m); d.parca = m.senaryo || ''; d.onizlemeMetni = '';
      panel.innerHTML = ''; panel.removeAttribute('hidden');
      var sol = el('div', { sinif: 'tm-alan' }, [el('h3', { metin: m.ad }), el('p', { metin: m.aciklama + (m.canli ? ' (Canlı: video üretildiği ana göre.)' : '') })]);
      (m.sema || []).forEach(function (s) { sol.appendChild(alan(s)); });
      sol.appendChild(el('div', { sinif: 'tm-eylem' }, [
        el('button', { sinif: 'tm-btn tm-ana', type: 'button', metin: 'Videoya ekle', onclick: ekle }),
        el('button', { sinif: 'tm-btn', type: 'button', metin: 'Önizle', onclick: function () { yenile(true); } }),
        el('button', { sinif: 'tm-btn', type: 'button', metin: 'Kapat', onclick: function () { panel.setAttribute('hidden', ''); d.secili = null; izgaraCiz(); } })
      ]));
      panel.appendChild(sol);
      panel.appendChild(el('div', {}, [buyukResim]));
      buyukResim.removeAttribute('src');
      if (onbellek[m.id]) buyukResim.src = onbellek[m.id];
      izgaraCiz();
      yenile(true);
      panel.scrollIntoView && panel.scrollIntoView({ block: 'nearest' });
    }

    ara.addEventListener('input', bekletici(function () { d.arama = ara.value; izgaraCiz(); }, 120));

    bildir('Malzemeler yükleniyor…');
    var hazir = istek('/malzeme/liste').then(function (t) {
      var j = JSON.parse(t);
      d.liste = j.malzemeler || []; d.kategoriler = j.kategoriler || []; d.paletler = j.paletler || []; d.yonler = j.yonler || [];
      cipCiz(); izgaraCiz(); bildir(d.liste.length + ' malzeme hazır. Karta dokunun, ayarlayın, "Videoya ekle".');
    }).catch(function (e) { bildir('Malzeme listesi alınamadı: ' + e.message, 'uy'); });

    return {
      hazir: hazir,
      sec: function (id) { var m = d.liste.filter(function (x) { return x.id === id; })[0]; if (m) sec(m); return !!m; },
      durum: function () { return { secili: d.secili && d.secili.id, degerler: Object.assign({}, d.degerler), parca: d.parca, kategori: d.kategori, arama: d.arama, adet: d.liste.length }; },
      kaldir: function () { canli.iptal(); if (gozcu) gozcu.disconnect(); kap.innerHTML = ''; }
    };
  }

  // Bir öğeyi (örn. zaman çizelgesi ya da senaryo kutusu) sürükle-bırak hedefi yapar: bırakılan malzeme app.senaryoyaEkle ile eklenir.
  function birakmaHedefiYap(hedef, app) {
    app = app || kok.TaaApp || {};
    hedef.addEventListener('dragover', function (e) { if (e.dataTransfer && Array.prototype.indexOf.call(e.dataTransfer.types || [], MIME) >= 0) { e.preventDefault(); e.dataTransfer.dropEffect = 'copy'; } });
    hedef.addEventListener('drop', function (e) {
      var p = e.dataTransfer && e.dataTransfer.getData(MIME);
      if (!p) return;
      e.preventDefault();
      if (/^malzeme [a-z0-9-]+( |$)/.test(p) && typeof app.senaryoyaEkle === 'function') app.senaryoyaEkle(p);
    });
  }

  var api = { mount: mount, birakmaHedefiYap: birakmaHedefiYap, MIME: MIME, _ic: { normal: normal, filtrele: filtrele, kategoriSayilari: kategoriSayilari, varsayilanDegerler: varsayilanDegerler, sinirla: sinirla, istekGovdesi: istekGovdesi, bekletici: bekletici } };
  kok.TaaMalzeme = api;
  if (typeof module !== 'undefined' && module.exports) module.exports = api;
})(typeof window !== 'undefined' ? window : globalThis);
