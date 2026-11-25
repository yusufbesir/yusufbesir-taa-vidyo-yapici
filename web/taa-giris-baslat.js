// Sahip: Yusuf. yazan: [Claude#ortak-giris 2026-11-25]
// TAA Vidyo Yapıcı açılışında TAA ortak girişi (KM1). Electron'daki preload köprüsünün yerine
// window.taaGiris yerel arayüz sunucusunun /api/giris/* uçlarına gider (yalnız 127.0.0.1 + X-LED).
// Kural: bu bilgisayarda taa-kimlik kuruluysa giriş zorunlu; kurulu değilse program eskisi gibi açılır.
// Geçerli oturum varsa ekran gösterilmez. Şifre bu dosyada tutulmaz, hiçbir yere yazılmaz.
(function () {
  'use strict';
  var ARALIK_MS = 10 * 60 * 1000;   // süren oturum aralıkla yeniden denetlenir

  function istek(islem, govde) {
    return fetch('/api/giris/' + islem, {
      method: 'POST', cache: 'no-store',
      headers: { 'X-LED': '1', 'Content-Type': 'application/json' },
      body: JSON.stringify(govde || {})
    }).then(function (r) { if (!r.ok) throw new Error('giris ' + r.status); return r.json(); });
  }
  var api = Object.freeze({
    giris: function (kullanici, parola) { return istek('giris', { kullanici: kullanici, parola: parola }); },
    basvur: function (kullanici, yeniParola) { return istek('basvur', { kullanici: kullanici, yeniParola: yeniParola }); },
    baslangic: function () { return istek('baslangic'); },
    durum: function () { return istek('durum'); },
    cikis: function () { return istek('cikis'); }
  });

  // Vidyo Yapıcı temasını ortak ekranın değişkenlerine bağla (tema değişince kendiliğinden uyar).
  var stil = document.createElement('style');
  stil.textContent = ':root{--taa-arka:var(--bg);--taa-panel:var(--panel);--taa-yazi:var(--tx);--taa-soluk:var(--mut);' +
    '--taa-vurgu:var(--ac);--taa-vurgu-yazi:var(--onac);--taa-cizgi:var(--line);--taa-r:var(--r)}' +
    '.taa-oturum-uyari{position:fixed;left:50%;bottom:24px;transform:translateX(-50%);max-width:560px;z-index:2147483000;' +
    'background:var(--panel);color:var(--tx);border:1px solid var(--uy);border-radius:var(--r);padding:12px 44px 12px 16px;' +
    'box-shadow:0 8px 24px rgba(0,0,0,.35);font:inherit;line-height:1.45}' +
    '.taa-oturum-uyari button{position:absolute;right:8px;top:8px;min-width:28px;min-height:28px;padding:0;border:0;background:none;color:inherit;font-size:18px;cursor:pointer}';
  document.head.appendChild(stil);

  function uyariGoster(oturum) {
    var metin = window.TaaGirisEkrani.uyariMetni(oturum);
    if (!metin) return;
    var d = document.createElement('div');
    d.className = 'taa-oturum-uyari';
    d.setAttribute('role', 'alert');
    d.textContent = metin;
    var k = document.createElement('button');
    k.type = 'button'; k.setAttribute('aria-label', 'Kapat'); k.textContent = '×';
    k.addEventListener('click', function () { d.remove(); });
    d.appendChild(k);
    document.body.appendChild(d);
  }

  // Ekran betiği yoksa sayfa kapatılır; program yeniden kurulmalı.
  function kilitle() {
    if (document.getElementById('taa-giris-kilit')) return;
    Array.prototype.forEach.call(document.body.children, function (e) { e.inert = true; });
    var d = document.createElement('div');
    d.id = 'taa-giris-kilit';
    d.setAttribute('role', 'alert');
    d.style.cssText = 'position:fixed;inset:0;z-index:2147483647;display:grid;place-items:center;padding:24px;background:var(--bg);color:var(--tx);font:inherit;text-align:center';
    d.textContent = 'Giriş ekranı yüklenemedi. TAA Vidyo Yapıcı\'yı yeniden kurun ya da yöneticinize haber verin.';
    document.body.appendChild(d);
  }

  // Ortak bilgisayarda giren kişi kalıcı olmasın: köşede "Çıkış" düğmesi (basınca giriş yeniden istenir).
  function cikisGoster(oturum) {
    if (window.TaaGirisEkrani && window.TaaGirisEkrani.cikisDugmesi) {
      window.TaaGirisEkrani.cikisDugmesi({ api: api, oturum: oturum, cikinca: girisAc });
    }
  }

  function girildi(oturum) { uyariGoster(oturum); cikisGoster(oturum); }

  function girisAc() {
    var d = document.getElementById('taa-giris-cikis'); if (d) d.remove();
    if (document.querySelector('.taa-giris')) return;   // zaten açık
    if (!window.TaaGirisEkrani) { kilitle(); return; }
    window.TaaGirisEkrani.ac(document.body, { programAdi: 'TAA Vidyo Yapıcı', api: api, girildi: girildi });
  }

  function denetle() {
    return api.baslangic().then(function (b) {
      if (!b || !b.arac) return;          // taa-kimlik kurulu değil: eski davranış
      if (b.oturum) { cikisGoster(b.oturum); return; }   // geçerli oturum var
      girisAc();
    }).catch(function () { girisAc(); }); // emin olunamıyorsa giriş istenir
  }

  function basla() {
    denetle();
    setInterval(denetle, ARALIK_MS);
    // Başka programda çıkış yapıldıysa ortak bilgisayarda pencere öne gelince hemen fark edilsin (en çok 5 sn'de bir).
    var son = Date.now();
    function odakDenetle() {
      if (document.visibilityState === 'hidden' || Date.now() - son < 5000) return;
      son = Date.now();
      denetle();
    }
    window.addEventListener('focus', odakDenetle);
    document.addEventListener('visibilitychange', odakDenetle);
  }

  if (document.readyState === 'loading') document.addEventListener('DOMContentLoaded', basla);
  else basla();
})();
