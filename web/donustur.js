// Açıklama: Dönüştürücü arayüzü (videoyu/resmi çizgi film, piksel art, duotone vb. görünüme çevirir). window.TaaDonustur = { mount(el, app), _ic }.
// Sunucu uçları: GET /donustur/turler, GET /donustur/kaynaklar, POST /donustur/onizle, POST /donustur/baslat, GET /donustur/durum,
// POST /donustur/iptal, POST /donustur/oynat, POST /donustur/klasor. Ayrıntı: docs/DONUSTURUCU.md.
// "Oynatıcıya ekle": app.oynaticiyaEkle(yol) varsa çağrılır; ayrıca document'e 'taa:oynaticiya-ekle' olayı ({detail:{dosya}}) gönderilir.
// Tema: yalnız CSS değişkenleri (--bg --panel --panel2 --line --tx --mut --ac --onac --ok --uy --r-btn --r-kart). Hareket yok (yalnız opacity). Dış bağımlılık yok.
// Sahip Yusuf; yazan [Claude#Dönüştürücü Opus 5.5, yenileme-1.0, 2026-11-25]; yalnız yazan yapay zekâ ya da Yusuf'un izniyle değişir; silme yok.
(function (kok) {
  'use strict';

  // ---------------------------------------------------------------- saf mantık (Node testi: tests/donustur_test.js)
  var BLOKLAR = [2, 5, 10, 25, 50];
  var SINIR = { sure: [0, 600], baslangic: [0, 36000], fps: [1, 60], yogunluk: [0, 100], renk: [4, 64] };
  function sinirla(ad, v) {
    var s = SINIR[ad]; var x = Number(v);
    if (!s) return v;
    if (!isFinite(x)) x = s[0];
    return Math.min(s[1], Math.max(s[0], x));
  }
  function enYakinBlok(v) {
    var x = Number(v); if (!isFinite(x)) return 10;
    return BLOKLAR.reduce(function (a, b) { return Math.abs(b - x) < Math.abs(a - x) ? b : a; }, BLOKLAR[0]);
  }
  // Tür hangi parametreleri kullanır
  function alanlar(tur) {
    var ortak = ['sigdir', 'yogunluk', 'sure', 'baslangic', 'fps'];
    if (tur === 'piksel' || tur === 'retro8bit') return ortak.concat(['palet', 'renk', 'dither', 'blok']);
    if (tur === 'duotone' || tur === 'kontur') return ortak.concat(['ton']);
    return ortak;
  }
  function varsayilanAyar(tur) {
    return { tur: tur || 'cizgifilm', sigdir: 'kirp', yogunluk: 50, sure: 0, baslangic: 0, fps: 25, palet: tur === 'retro8bit' ? 'retro16' : 'otomatik', ton: 'lacivert-altin', renk: 24, dither: 'yok', blok: 10 };
  }
  // Sunucuya giden gövde: yalnız türün kullandığı alanlar, sınırlanmış.
  function istekGovdesi(kaynak, ayar, ek) {
    var a = ayar || varsayilanAyar();
    var g = { kaynak: String(kaynak || ''), tur: a.tur };
    alanlar(a.tur).forEach(function (k) {
      if (k === 'ton') g.palet = a.ton;
      else if (k === 'blok') g.blok = enYakinBlok(a.blok);
      else if (k === 'sigdir' || k === 'palet' || k === 'dither') g[k] = a[k];
      else g[k] = sinirla(k, a[k]);
    });
    if (ek) Object.keys(ek).forEach(function (k) { g[k] = ek[k]; });
    return g;
  }
  function boyutYazi(n) {
    n = Number(n) || 0;
    if (n >= 1073741824) return (n / 1073741824).toFixed(1).replace('.', ',') + ' GB';
    if (n >= 1048576) return (n / 1048576).toFixed(1).replace('.', ',') + ' MB';
    return Math.max(1, Math.round(n / 1024)) + ' KB';
  }
  var TR = { 'ç': 'c', 'ğ': 'g', 'ı': 'i', 'ö': 'o', 'ş': 's', 'ü': 'u' };
  function normal(s) { return String(s == null ? '' : s).replace(/İ/g, 'i').replace(/I/g, 'ı').toLowerCase().replace(/[çğıöşü]/g, function (c) { return TR[c]; }).trim(); }
  function kaynakFiltrele(liste, arama) {
    var q = normal(arama); if (!q) return (liste || []).slice();
    return (liste || []).filter(function (k) { return normal(k.ad + ' ' + k.klasor).indexOf(q) >= 0; });
  }
  function durumYazi(d) {
    if (!d) return '';
    if (d.calisiyor) return (d.asama || 'çalışıyor') + ' · %' + Math.round(d.yuzde || 0) + (d.toplam ? ' (' + d.kare + '/' + d.toplam + ' kare)' : '');
    if (d.hata) return 'Durdu: ' + (d.mesaj || 'bilinmeyen hata');
    return d.mesaj || '';
  }
  function dosyaAdi(yol) { return String(yol || '').split(/[\\/]/).pop(); }

  // ---------------------------------------------------------------- sunucu
  function istek(yol, govde) {
    var o = govde === undefined ? { credentials: 'same-origin' } : { method: 'POST', credentials: 'same-origin', headers: { 'x-led': '1', 'Content-Type': 'application/json' }, body: JSON.stringify(govde) };
    return fetch(yol, o).then(function (r) {
      return r.text().then(function (t) {
        var j = null; try { j = JSON.parse(t); } catch (e) { /* düz metin */ }
        if (!r.ok) throw new Error((j && j.hata) || t || ('HTTP ' + r.status));
        return j;
      });
    });
  }

  // ---------------------------------------------------------------- görünüm
  var STIL = [
    '.td{color:var(--tx);font-size:16px;display:flex;flex-direction:column;gap:14px;min-width:0}',
    '.td *{box-sizing:border-box}',
    '.td h3{font-size:18px;margin:0;font-weight:700}',
    '.td-blok{background:var(--panel);border:2px solid var(--line);border-radius:var(--r-kart,12px);padding:14px;display:flex;flex-direction:column;gap:10px}',
    '.td-satir{display:flex;flex-wrap:wrap;gap:10px;align-items:center}',
    '.td button,.td select,.td input[type=search]{min-height:48px;font-size:16px;font-family:inherit;color:var(--tx);background:var(--panel2);border:2px solid var(--line);border-radius:var(--r-btn,10px);padding:8px 14px}',
    '.td button{cursor:pointer;font-weight:700}',
    '.td button.td-ana{min-height:56px;background:var(--ac);color:var(--onac);border-color:var(--ac);padding:10px 22px}',
    '.td button:disabled{opacity:.5;cursor:not-allowed}',
    '.td :focus-visible{outline:3px solid var(--ac);outline-offset:2px}',
    '.td-kaynaklar{display:flex;flex-direction:column;gap:6px;max-height:300px;overflow:auto}',
    '.td-kaynak{display:flex;justify-content:space-between;gap:10px;text-align:left;width:100%}',
    '.td-kaynak[aria-pressed=true]{border-color:var(--ac);background:var(--bg)}',
    '.td-kaynak small,.td-mut{color:var(--mut);font-size:14px}',
    '.td-turler{display:grid;grid-template-columns:repeat(auto-fill,minmax(150px,1fr));gap:10px}',
    '.td-tur{display:flex;flex-direction:column;align-items:stretch;gap:6px;padding:8px;text-align:left;min-height:48px}',
    '.td-tur[aria-pressed=true]{border-color:var(--ac);background:var(--bg)}',
    '.td-tur img,.td-tur .td-bos{width:100%;aspect-ratio:190/360;object-fit:cover;background:var(--bg);border-radius:6px;display:block}',
    '.td-tur span{font-size:16px}.td-tur small{font-size:13px;color:var(--mut);font-weight:400}',
    '.td-alan{display:flex;flex-direction:column;gap:4px;min-width:200px;flex:1}',
    '.td-alan label{font-size:16px;font-weight:600}',
    '.td input[type=range]{width:100%;min-height:48px;accent-color:var(--ac)}',
    '.td-onizleme{display:flex;gap:14px;flex-wrap:wrap;align-items:flex-start}',
    '.td-onizleme img{width:min(100%,285px);aspect-ratio:950/1800;background:var(--bg);border:2px solid var(--line);border-radius:8px;object-fit:contain}',
    '.td-cubuk{height:14px;background:var(--panel2);border:2px solid var(--line);border-radius:7px;overflow:hidden}',
    '.td-cubuk i{display:block;height:100%;width:100%;background:var(--ac);transform-origin:left center;transform:scaleX(0)}',
    '.td-durum{min-height:24px;font-size:16px}',
    '.td-durum[data-tur=hata]{color:var(--uy)}.td-durum[data-tur=ok]{color:var(--ok)}',
    '@media (prefers-reduced-motion:reduce){.td *{transition:none!important}}'
  ].join('\n');

  function el(ad, oz, cocuklar) {
    var e = document.createElement(ad);
    Object.keys(oz || {}).forEach(function (k) {
      if (k === 'metin') e.textContent = oz[k];
      else if (k === 'sinif') e.className = oz[k];
      else if (k === 'tikla') e.addEventListener('click', oz[k]);
      else e.setAttribute(k, oz[k]);
    });
    (cocuklar || []).forEach(function (c) { if (c) e.appendChild(c); });
    return e;
  }

  function mount(kap, app) {
    app = app || kok.TaaApp || {};
    if (!document.getElementById('td-stil')) document.head.appendChild(el('style', { id: 'td-stil', metin: STIL }));
    kap.innerHTML = '';
    var d = { bilgi: null, kaynaklar: [], kaynak: '', ayar: varsayilanAyar('cizgifilm'), izle: null, sonDosya: '' };
    var durum = el('div', { sinif: 'td-durum', role: 'status', 'aria-live': 'polite' });
    function bildir(m, tur) { durum.textContent = m || ''; if (tur) durum.setAttribute('data-tur', tur); else durum.removeAttribute('data-tur'); }

    // 1) Kaynak
    var ara = el('input', { type: 'search', placeholder: 'Dosya ara', 'aria-label': 'Kaynak dosya ara' });
    var liste = el('div', { sinif: 'td-kaynaklar', role: 'group', 'aria-label': 'Kaynak dosyalar' });
    var yenile = el('button', { type: 'button', metin: 'Listeyi yenile', tikla: kaynaklariYukle });
    var kaynakBlok = el('section', { sinif: 'td-blok', 'aria-label': 'Kaynak seç' }, [el('h3', { metin: '1. Kaynak video ya da resim' }), el('div', { sinif: 'td-satir' }, [ara, yenile]), liste,
      el('p', { sinif: 'td-mut', metin: 'Çıktı klasörü, Videolar, Resimler, Masaüstü ve İndirilenler klasörlerindeki dosyalar listelenir.' })]);

    // 2) Tür kartları
    var turIzgara = el('div', { sinif: 'td-turler', role: 'group', 'aria-label': 'Dönüştürme türleri' });
    var turBlok = el('section', { sinif: 'td-blok', 'aria-label': 'Tür seç' }, [el('h3', { metin: '2. Görünüm' }), turIzgara]);

    // 3) Parametreler
    var param = el('div', { sinif: 'td-satir' });
    var paramBlok = el('section', { sinif: 'td-blok', 'aria-label': 'Ayarlar' }, [el('h3', { metin: '3. Ayarlar' }), param]);

    // 4) Önizleme ve üretim
    var onizImg = el('img', { alt: 'Dönüştürme önizlemesi (950x1800)' });
    var onizBtn = el('button', { type: 'button', metin: 'Önizle', tikla: onizle });
    var baslaBtn = el('button', { type: 'button', sinif: 'td-ana', metin: 'Dönüştür', tikla: baslat });
    var iptalBtn = el('button', { type: 'button', metin: 'İptal', disabled: '', tikla: function () { istek('/donustur/iptal', {}).catch(function () {}); } });
    var cubukIc = el('i'); var cubuk = el('div', { sinif: 'td-cubuk', role: 'progressbar', 'aria-label': 'Dönüştürme ilerlemesi', 'aria-valuemin': '0', 'aria-valuemax': '100', 'aria-valuenow': '0' }, [cubukIc]);
    var oynatBtn = el('button', { type: 'button', metin: 'Oynat', hidden: '', tikla: function () { istek('/donustur/oynat', {}).catch(function (e) { bildir(e.message, 'hata'); }); } });
    var klasorBtn = el('button', { type: 'button', metin: 'Klasörü aç', hidden: '', tikla: function () { istek('/donustur/klasor', {}).catch(function () {}); } });
    var ekleBtn = el('button', { type: 'button', metin: 'Oynatıcıya ekle', hidden: '', tikla: oynaticiyaEkle });
    var uretBlok = el('section', { sinif: 'td-blok', 'aria-label': 'Önizleme ve dönüştürme' }, [el('h3', { metin: '4. Önizle ve dönüştür' }),
      el('div', { sinif: 'td-onizleme' }, [onizImg, el('div', { sinif: 'td-satir' }, [onizBtn, baslaBtn, iptalBtn])]), cubuk, durum, el('div', { sinif: 'td-satir' }, [oynatBtn, klasorBtn, ekleBtn])]);

    var kokEl = el('div', { sinif: 'td' }, [kaynakBlok, turBlok, paramBlok, uretBlok]);
    kap.appendChild(kokEl);

    function kaynakCiz() {
      liste.innerHTML = '';
      var l = kaynakFiltrele(d.kaynaklar, ara.value);
      if (!l.length) { liste.appendChild(el('p', { sinif: 'td-mut', metin: d.kaynaklar.length ? 'Aramaya uyan dosya yok.' : 'Dosya bulunamadı. Videoyu Videolar klasörüne koyup listeyi yenileyin.' })); return; }
      l.slice(0, 120).forEach(function (k) {
        liste.appendChild(el('button', { type: 'button', sinif: 'td-kaynak', 'aria-pressed': String(k.yol === d.kaynak), tikla: function () { d.kaynak = k.yol; kaynakCiz(); kartOnizlemeleri(); onizle(); } },
          [el('span', { metin: (k.resim ? 'Resim · ' : 'Video · ') + k.ad }), el('small', { metin: k.klasor + ' · ' + boyutYazi(k.boyut) })]));
      });
    }
    function kaynaklariYukle() {
      return istek('/donustur/kaynaklar').then(function (j) { d.kaynaklar = (j && j.kaynaklar) || []; kaynakCiz(); }).catch(function (e) { bildir(e.message, 'hata'); });
    }
    ara.addEventListener('input', kaynakCiz);

    var kartImg = {};
    function turCiz() {
      turIzgara.innerHTML = '';
      (d.bilgi.turler || []).forEach(function (t) {
        var img = kartImg[t.id] || el('div', { sinif: 'td-bos', 'aria-hidden': 'true' });
        turIzgara.appendChild(el('button', { type: 'button', sinif: 'td-tur', 'aria-pressed': String(d.ayar.tur === t.id), tikla: function () {
          var eski = d.ayar; d.ayar = varsayilanAyar(t.id); ['sigdir', 'sure', 'baslangic', 'fps', 'yogunluk'].forEach(function (k) { d.ayar[k] = eski[k]; });
          turCiz(); paramCiz(); onizle();
        } }, [img, el('span', { metin: t.ad }), el('small', { metin: t.aciklama })]));
      });
    }
    function kartOnizlemeleri() {
      if (!d.kaynak || !d.bilgi) return;
      var sira = (d.bilgi.turler || []).slice();
      (function sonraki() { // sırayla: aynı anda tek ffmpeg
        var t = sira.shift(); if (!t) return;
        istek('/donustur/onizle', istekGovdesi(d.kaynak, varsayilanAyar(t.id), { kucuk: true, t: 1 })).then(function (j) {
          kartImg[t.id] = el('img', { src: j.png, alt: '' }); turCiz();
        }).catch(function () {}).then(sonraki);
      })();
    }

    function secim(ad, etiket, secenekler) {
      var s = el('select', { id: 'td-' + ad });
      secenekler.forEach(function (o) { var op = el('option', { value: o[0], metin: o[1] }); if (String(d.ayar[ad]) === String(o[0])) op.selected = true; s.appendChild(op); });
      s.addEventListener('change', function () { d.ayar[ad] = ad === 'blok' ? Number(s.value) : s.value; onizle(); });
      return el('div', { sinif: 'td-alan' }, [el('label', { 'for': 'td-' + ad, metin: etiket }), s]);
    }
    function surgu(ad, etiket, min, max, adim, birim) {
      var g = el('input', { type: 'range', id: 'td-' + ad, min: String(min), max: String(max), step: String(adim), value: String(d.ayar[ad]) });
      var lb = el('label', { 'for': 'td-' + ad });
      function yaz() { lb.textContent = etiket + ': ' + (ad === 'sure' && Number(g.value) === 0 ? 'tamamı' : g.value + (birim || '')); }
      yaz();
      g.addEventListener('input', function () { d.ayar[ad] = Number(g.value); yaz(); });
      g.addEventListener('change', onizle);
      return el('div', { sinif: 'td-alan' }, [lb, g]);
    }
    function paramCiz() {
      param.innerHTML = '';
      var a = alanlar(d.ayar.tur);
      param.appendChild(secim('sigdir', 'Ekrana yerleşim', [['kirp', 'Kırp (ekranı doldur)'], ['sigdir', 'Sığdır (siyah kenar)'], ['doldur', 'Doldur (koyu arka plan)']]));
      param.appendChild(surgu('yogunluk', 'Yoğunluk', 0, 100, 5, ''));
      param.appendChild(surgu('sure', 'Süre', 0, 120, 1, ' sn'));
      param.appendChild(surgu('baslangic', 'Başlangıç', 0, 600, 1, ' sn'));
      param.appendChild(surgu('fps', 'Kare hızı', 10, 60, 1, ' fps'));
      if (a.indexOf('palet') >= 0) {
        param.appendChild(secim('palet', 'Palet', (d.bilgi.paletler || []).map(function (p) { return [p.id, p.ad]; })));
        param.appendChild(secim('blok', 'Piksel boyutu', BLOKLAR.map(function (b) { return [b, b + ' px (' + Math.round(950 / b) + 'x' + Math.round(1800 / b) + ')']; })));
        param.appendChild(secim('dither', 'Dithering', [['yok', 'Yok (düz renk)'], ['bayer', 'Düzenli desen'], ['floyd', 'Dağınık (Floyd)']]));
        param.appendChild(surgu('renk', 'Renk sayısı (otomatik palet)', 4, 64, 1, ''));
      }
      if (a.indexOf('ton') >= 0) param.appendChild(secim('ton', 'Ton', (d.bilgi.tonlar || []).map(function (t) { return [t.id, t.ad]; })));
    }

    var onizSira = 0;
    function onizle() {
      if (!d.kaynak) { bildir('Önce bir kaynak dosya seçin.'); return; }
      var no = ++onizSira;
      onizBtn.disabled = true; bildir('Önizleme hazırlanıyor…');
      istek('/donustur/onizle', istekGovdesi(d.kaynak, d.ayar, { t: 1 })).then(function (j) {
        if (no !== onizSira) return;
        onizImg.src = j.png; bildir('Önizleme hazır.', 'ok');
      }).catch(function (e) { if (no === onizSira) bildir(e.message, 'hata'); }).then(function () { if (no === onizSira) onizBtn.disabled = false; });
    }

    function ilerlemeYaz(j) {
      var y = Math.max(0, Math.min(100, Number(j.yuzde) || 0));
      cubukIc.style.transform = 'scaleX(' + (y / 100) + ')';
      cubuk.setAttribute('aria-valuenow', String(Math.round(y)));
      bildir(durumYazi(j), j.hata ? 'hata' : (!j.calisiyor && j.mesaj ? 'ok' : ''));
    }
    function izle() {
      clearTimeout(d.izle);
      istek('/donustur/durum').then(function (j) {
        ilerlemeYaz(j);
        if (j.calisiyor) { d.izle = setTimeout(izle, 500); return; }
        baslaBtn.disabled = false; iptalBtn.disabled = true;
        var bitti = !j.hata && j.dosya;
        if (bitti) d.sonDosya = j.dosya;
        [oynatBtn, klasorBtn, ekleBtn].forEach(function (b) { if (bitti) b.removeAttribute('hidden'); else b.setAttribute('hidden', ''); });
      }).catch(function (e) { bildir(e.message, 'hata'); baslaBtn.disabled = false; iptalBtn.disabled = true; });
    }
    function baslat() {
      if (!d.kaynak) { bildir('Önce bir kaynak dosya seçin.', 'hata'); return; }
      baslaBtn.disabled = true; iptalBtn.disabled = false;
      [oynatBtn, klasorBtn, ekleBtn].forEach(function (b) { b.setAttribute('hidden', ''); });
      istek('/donustur/baslat', istekGovdesi(d.kaynak, d.ayar)).then(function (j) { bildir('Başladı: ' + dosyaAdi(j.dosya)); izle(); })
        .catch(function (e) { bildir(e.message, 'hata'); baslaBtn.disabled = false; iptalBtn.disabled = true; });
    }
    function oynaticiyaEkle() {
      if (!d.sonDosya) return;
      var ok = false;
      if (typeof app.oynaticiyaEkle === 'function') { try { app.oynaticiyaEkle(d.sonDosya); ok = true; } catch (e) { /* uygulama işlevi yoksa olay yeter */ } }
      try { document.dispatchEvent(new CustomEvent('taa:oynaticiya-ekle', { detail: { dosya: d.sonDosya } })); ok = true; } catch (e) { /* eski tarayıcı */ }
      bildir(ok ? '"' + dosyaAdi(d.sonDosya) + '" oynatıcıya gönderildi.' : 'Oynatıcı bulunamadı.', ok ? 'ok' : 'hata');
    }

    istek('/donustur/turler').then(function (j) {
      d.bilgi = j; turCiz(); paramCiz();
      if (!j.ffmpeg) bildir('ffmpeg bulunamadı: ffmpeg.exe dosyasını programın klasörüne koyun ya da LED_FFMPEG ile yolunu verin.', 'hata');
    }).catch(function (e) { bildir(e.message, 'hata'); });
    kaynaklariYukle();
    istek('/donustur/durum').then(function (j) { if (j && j.calisiyor) { baslaBtn.disabled = true; iptalBtn.disabled = false; izle(); } }).catch(function () {});
    return { yenile: kaynaklariYukle };
  }

  var api = { mount: mount, _ic: { sinirla: sinirla, enYakinBlok: enYakinBlok, alanlar: alanlar, varsayilanAyar: varsayilanAyar, istekGovdesi: istekGovdesi, boyutYazi: boyutYazi, normal: normal, kaynakFiltrele: kaynakFiltrele, durumYazi: durumYazi, dosyaAdi: dosyaAdi } };
  kok.TaaDonustur = api;
  if (typeof module !== 'undefined' && module.exports) module.exports = api;
})(typeof window !== 'undefined' ? window : globalThis);
