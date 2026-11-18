/* TAA Vidyo Yapıcı: "Yapay zekâ" paneli (Videoyu anlatın → Hazırla → Uygula).
 * Sahip: Yusuf. Yazan: [Claude#AI Opus 5.5, yenileme-1.0, 2026-11-18]. Yalnız yazan yapay zekâ ya da Yusuf'un izniyle değişir; silme yok.
 * Arayüz: window.TaaAI = { mount(el, app) }. app = window.TaaApp (UI ajanı sağlar): senaryoMetni(), senaryoyaEkle(m), senaryoDegistir(m), onizle(m), uret(), durum().
 * Uçlar: docs/TARIF-API.md (POST /tarif/hazirla, /tarif/iptal, /tarif/istem, /tarif/uygula, /tarif/ac, GET /tarif/siteler, /tarif/llm-durum, POST /tarif/llm-ayar).
 * Tema: YALNIZ CSS değişkenleri (--bg --panel --panel2 --line --tx --mut --ac --onac --ok --uy). Dış bağımlılık yok. Güvenlik: sunucudan gelen metin yalnız textContent ile yazılır.
 */
(function (kok) {
  'use strict';

  const ORNEKLER = [
    'Takım elbiselerde yüzde 20 indirim',
    'Bayramınız kutlu olsun',
    'Kredi kartına 6 taksit',
    'Yeni sezon geldi, mağazamıza bekleriz',
    'Hafta içi 9-19, cumartesi 10-18 açığız',
    'Bugün tadilat nedeniyle kapalıyız',
    'Gömlek 499 TL, altın renk',
    'Önce hoş geldiniz, sonra %30 indirim konfetili'
  ];
  const KAYNAK_AD = { 'yapay-zeka': 'Yapay zekâ', kural: 'Kural', sablon: 'Şablon' };

  // ───── saf yardımcılar (Node testinde de çalışır) ─────
  function kaynakAdi(k) { return KAYNAK_AD[k] || 'Şablon'; }
  /** Sunucu yanıtındaki adımlardan kullanıcıya gösterilecek tek satırlık bilgi. */
  function adimOzeti(j) {
    if (!j || !Array.isArray(j.adimlar)) return '';
    const yz = j.adimlar.find(a => a.ad === 'yapay-zeka');
    if (j.kaynak === 'yapay-zeka') return 'Yapay zekâ hazırladı.';
    if (yz && (yz.durum === 'hata' || yz.durum === 'iptal')) return yz.mesaj + ' Yerel kurallarla hazırlandı.';
    return j.kaynak === 'sablon' ? 'Hazır şablon önerildi.' : 'Yerel kurallarla hazırlandı.';
  }
  /** Ağ/sunucu hatasını sade Türkçeye çevirir. */
  function hataMetni(e) {
    const m = String((e && e.message) || e || '');
    if (/Failed to fetch|NetworkError|ECONNREFUSED|Load failed/i.test(m)) return 'Programla bağlantı kurulamadı. Programı yeniden başlatmayı deneyin.';
    if (/abort/i.test(m)) return 'İşlem iptal edildi.';
    if (/^HTTP 4/.test(m)) return 'İstek kabul edilmedi.';
    if (/^HTTP 5/.test(m)) return 'Program içinde bir hata oluştu.';
    return m && m.length < 160 ? m : 'Bir sorun oluştu. Yeniden deneyin.';
  }
  /** /tarif/hazirla istek gövdesi. */
  function istekGovdesi(metin, cesit, onay) {
    const g = { metin: String(metin || '').slice(0, 4000), cesit: Math.max(0, cesit | 0) };
    if (onay) g.onay = true;
    return JSON.stringify(g);
  }
  /** Bir sahnenin satırları (yeni "satirlar" dizisi ya da eski "metin" alanı). */
  function sahneSatirlari(s) { return Array.isArray(s.satirlar) ? s.satirlar : String(s.metin || '').split(' / '); }

  // ───── küçük DOM yardımcısı (createElement + textContent; innerHTML kullanılmaz) ─────
  function h(belge, etiket, ozel, cocuklar) {
    const e = belge.createElement(etiket);
    for (const k in (ozel || {})) {
      const v = ozel[k];
      if (v === undefined || v === null || v === false) continue;
      if (k === 'text') e.textContent = v;
      else if (k === 'on') { for (const o in v) e.addEventListener(o, v[o]); }
      else if (k === 'style') e.setAttribute('style', v);
      else e.setAttribute(k, v === true ? '' : String(v));
    }
    for (const c of [].concat(cocuklar || [])) { if (c) e.appendChild(typeof c === 'string' ? belge.createTextNode(c) : c); }
    return e;
  }

  const CSS = `
.taai{display:grid;gap:14px;color:var(--tx);font-size:16px;line-height:1.5}
.taai *{box-sizing:border-box}
.taai label.taai-bas{font-size:18px;font-weight:700}
.taai textarea{width:100%;min-height:120px;padding:12px 14px;font:inherit;color:var(--tx);background:var(--panel2);border:1px solid var(--line);border-radius:10px;resize:vertical}
.taai textarea:focus-visible,.taai button:focus-visible,.taai select:focus-visible,.taai input:focus-visible,.taai summary:focus-visible{outline:3px solid var(--ac);outline-offset:2px}
.taai-cipler{display:flex;flex-wrap:wrap;gap:8px}
.taai button{min-height:48px;padding:0 18px;font:inherit;font-size:16px;color:var(--tx);background:var(--panel2);border:1px solid var(--line);border-radius:10px;cursor:pointer}
.taai button:hover{border-color:var(--ac)}
.taai button[disabled]{opacity:.55;cursor:default}
.taai .taai-cip{min-height:48px;border-radius:24px;font-size:15px}
.taai .taai-ana{min-height:56px;padding:0 28px;font-weight:700;background:var(--ac);color:var(--onac);border-color:var(--ac)}
.taai-sira{display:flex;flex-wrap:wrap;gap:10px;align-items:center}
.taai-durum{color:var(--mut);min-height:24px}
.taai-rozet{display:inline-flex;align-items:center;min-height:32px;padding:0 12px;border-radius:16px;border:1px solid var(--line);background:var(--panel);font-size:14px;font-weight:700}
.taai-rozet[data-k="yapay-zeka"]{border-color:var(--ok);color:var(--ok)}
.taai-rozet[data-k="kural"]{border-color:var(--ac);color:var(--ac)}
.taai-rozet[data-k="sablon"]{border-color:var(--uy);color:var(--uy)}
.taai-sonuc{display:grid;gap:10px;padding:14px;border:1px solid var(--line);border-radius:12px;background:var(--panel)}
.taai-sahneler{display:grid;grid-template-columns:repeat(auto-fill,minmax(120px,1fr));gap:10px}
.taai-sahne{display:grid;gap:4px;align-content:start;padding:10px;border:1px solid var(--line);border-radius:10px;background:var(--bg);min-height:120px}
.taai-sahne b{font-size:17px;line-height:1.15;word-break:break-word}
.taai-sahne small,.taai small{color:var(--mut);font-size:13px}
.taai ul{margin:0;padding-left:20px}
.taai .taai-uyari{color:var(--uy)}
.taai .taai-hata{color:var(--uy);font-weight:700}
.taai details{border:1px solid var(--line);border-radius:10px;background:var(--panel)}
.taai details>summary{min-height:48px;display:flex;align-items:center;padding:0 14px;cursor:pointer;font-weight:600}
.taai details>div{display:grid;gap:10px;padding:0 14px 14px}
.taai select,.taai input{min-height:48px;padding:0 12px;font:inherit;color:var(--tx);background:var(--panel2);border:1px solid var(--line);border-radius:10px}
@media (prefers-reduced-motion:no-preference){.taai-sonuc{animation:taai-gir .18s ease-out}@keyframes taai-gir{from{opacity:0;transform:translateY(4px)}to{opacity:1;transform:none}}}
`;

  function mount(el, app) {
    if (!el) throw new Error('TaaAI.mount: hedef öğe yok');
    const belge = el.ownerDocument || kok.document;
    const fetchFn = (kok.fetch || (typeof fetch !== 'undefined' ? fetch : null));
    const api = (yol, govde) => fetchFn(yol, { method: govde === undefined ? 'GET' : 'POST', headers: { 'X-LED': '1' }, body: govde, credentials: 'same-origin' })
      .then(r => { if (!r.ok) return r.text().then(t => { throw new Error(t && t.length < 200 ? t : 'HTTP ' + r.status); }); return r; });
    app = app || kok.TaaApp || {};
    const durum = { cesit: 0, son: null, mesgul: false };

    if (belge.getElementById && !belge.getElementById('taai-stil')) { const st = h(belge, 'style', { id: 'taai-stil' }); st.textContent = CSS; (belge.head || el).appendChild(st); }

    // ── üst: anlatım ──
    const alan = h(belge, 'textarea', { id: 'taai-metin', rows: '4', spellcheck: 'true', placeholder: 'Örn: Takım elbiselerde yüzde 20 indirim, kırmızı olsun, sonra 3 taksit.', 'aria-describedby': 'taai-ipucu' });
    const cipler = h(belge, 'div', { class: 'taai-cipler', role: 'group', 'aria-label': 'Örnek öneriler' },
      ORNEKLER.map(o => h(belge, 'button', { type: 'button', class: 'taai-cip', text: o, on: { click: () => { alan.value = o; alan.focus && alan.focus(); } } })));
    const hazirlaBtn = h(belge, 'button', { type: 'button', class: 'taai-ana', id: 'taai-hazirla', text: 'Hazırla' });
    const iptalBtn = h(belge, 'button', { type: 'button', id: 'taai-iptal', text: 'Durdur', hidden: true });
    const durumYazi = h(belge, 'div', { class: 'taai-durum', role: 'status', 'aria-live': 'polite' });
    const sonucKutu = h(belge, 'div', { id: 'taai-sonuc', 'aria-live': 'polite' });

    // ── ikincil: dış sohbet siteleri ──
    const siteSira = h(belge, 'div', { class: 'taai-sira' });
    const yanitAlan = h(belge, 'textarea', { id: 'taai-yanit', rows: '3', spellcheck: 'false', placeholder: 'Yapay zekânın verdiği yanıtın tamamını buraya yapıştırın.' });
    const disBolum = h(belge, 'details', { id: 'taai-dis' }, [
      h(belge, 'summary', { text: 'Sohbet sitesinde hazırlat (ChatGPT, Gemini, DeepSeek, Claude)' }),
      h(belge, 'div', {}, [
        h(belge, 'small', { text: '1) Bir siteye basın: istem panoya kopyalanır ve site açılır. 2) Yapıştırıp gönderin. 3) Yanıtı kopyalayıp aşağıya yapıştırın.' }),
        siteSira,
        h(belge, 'label', { for: 'taai-yanit', text: 'Yanıtı yapıştırın' }), yanitAlan,
        h(belge, 'div', { class: 'taai-sira' }, [h(belge, 'button', { type: 'button', id: 'taai-yapistir', text: 'Yanıtı uygula', on: { click: () => yanitUygula() } }),
          h(belge, 'button', { type: 'button', text: 'İstemi kopyala', on: { click: () => istemKopyala() } })])
      ])
    ]);

    // ── ikincil: doğrudan bağlantı ayarı ──
    const llmDurum = h(belge, 'small', { text: '' });
    const llmSec = h(belge, 'select', { id: 'taai-saglayici', 'aria-label': 'Yapay zekâ hizmeti' });
    const llmModel = h(belge, 'input', { id: 'taai-model', type: 'text', placeholder: 'Model (boş: önerilen)', 'aria-label': 'Model adı', autocomplete: 'off' });
    const llmAdres = h(belge, 'input', { id: 'taai-adres', type: 'url', placeholder: 'https://… (yalnız “Diğer” için)', 'aria-label': 'Özel adres', autocomplete: 'off' });
    const llmAnahtar = h(belge, 'input', { id: 'taai-anahtar', type: 'password', placeholder: 'Anahtar (kaydedilir, bir daha gösterilmez)', 'aria-label': 'Anahtar', autocomplete: 'off' });
    const llmAyarBolum = h(belge, 'details', { id: 'taai-llm' }, [
      h(belge, 'summary', {}, ['Doğrudan bağlantı (isteğe bağlı) ', llmDurum]),
      h(belge, 'div', {}, [
        h(belge, 'small', { text: 'Açarsanız tarifiniz ve program katalog adları seçtiğiniz hizmete internetten gönderilir. Anahtar Windows kullanıcı şifrelemesiyle saklanır. Bağlantı olmazsa program yerel kurallarla yine hazırlar.' }),
        llmSec, llmModel, llmAdres, llmAnahtar,
        h(belge, 'div', { class: 'taai-sira' }, [
          h(belge, 'button', { type: 'button', text: 'Kaydet ve aç', on: { click: () => llmKaydet(true) } }),
          h(belge, 'button', { type: 'button', text: 'Kapat', on: { click: () => llmKaydet(false) } }),
          h(belge, 'button', { type: 'button', text: 'Anahtarı kaldır', on: { click: () => llmKaydet(false, true) } })])
      ])
    ]);

    const kokEl = h(belge, 'section', { class: 'taai', 'aria-label': 'Yapay zekâ ile video hazırla' }, [
      h(belge, 'label', { class: 'taai-bas', for: 'taai-metin', text: 'Videoyu anlatın' }),
      h(belge, 'small', { id: 'taai-ipucu', text: 'Ne yazsın, hangi renk ya da hareket olsun, düz cümleyle yazın. Kişisel bilgi (telefon, müşteri adı) yazmayın.' }),
      alan, cipler,
      h(belge, 'div', { class: 'taai-sira' }, [hazirlaBtn, iptalBtn]),
      durumYazi, sonucKutu, disBolum, llmAyarBolum
    ]);
    while (el.firstChild) el.removeChild(el.firstChild);
    el.appendChild(kokEl);

    function mesgul(v, yazi) { durum.mesgul = v; hazirlaBtn.disabled = v; iptalBtn.hidden = !v; durumYazi.textContent = yazi || ''; }
    function temizle(e) { while (e.firstChild) e.removeChild(e.firstChild); }
    function liste(baslik, v, sinif) { if (!v || !v.length) return null; return h(belge, 'div', { class: sinif || '' }, [h(belge, 'b', { text: baslik }), h(belge, 'ul', {}, v.map(x => h(belge, 'li', { text: x })))]); }

    function goster(j) {
      durum.son = j; temizle(sonucKutu);
      if (!j || !j.ok) {
        sonucKutu.appendChild(h(belge, 'div', { class: 'taai-sonuc' }, [
          h(belge, 'p', { class: 'taai-hata', text: (j && j.ozet) || 'Kullanılamadı.' }),
          liste('Ayrıntı:', j && j.hatalar, 'taai-uyari'),
          j && j.duzelt ? h(belge, 'button', { type: 'button', text: 'Düzelt istemini kopyala', on: { click: () => kopyala(j.duzelt, 'Düzelt istemi kopyalandı; yapay zekâya yapıştırın.') } }) : null
        ]));
        return;
      }
      const k = j.kaynak || 'kural';
      const sahneler = h(belge, 'div', { class: 'taai-sahneler' }, (j.sahneler || []).map((s, i) => h(belge, 'div', { class: 'taai-sahne' }, [
        h(belge, 'small', { text: 'Sahne ' + (i + 1) + ' · ' + s.sure + ' sn' }),
        ...sahneSatirlari(s).map(x => h(belge, 'b', { text: x })),
        h(belge, 'small', { text: [s.giris, s.efekt, s.palet].filter(Boolean).join(' · ') })
      ])));
      const onayKutu = j.yz_onay_gerekli ? h(belge, 'div', { class: 'taai-sira' }, [
        h(belge, 'small', { text: 'Daha iyi sonuç için tarifi bağlı yapay zekâ hizmetine gönderebilirsiniz.' }),
        h(belge, 'button', { type: 'button', text: 'Yapay zekâya gönder', on: { click: () => hazirla(true) } })]) : null;
      sonucKutu.appendChild(h(belge, 'div', { class: 'taai-sonuc' }, [
        h(belge, 'div', { class: 'taai-sira' }, [h(belge, 'span', { class: 'taai-rozet', 'data-k': k, text: kaynakAdi(k) }), h(belge, 'small', { text: (j.sahneler || []).length + ' sahne · ' + j.sure + ' sn · ' + adimOzeti(j) })]),
        sahneler,
        liste('Varsayımlarım:', j.varsayimlar, 'taai-uyari'),
        liste('Not:', j.notlar),
        j.anlasilmayan && j.anlasilmayan.length ? h(belge, 'small', { text: 'Kullanmadığım sözcükler: ' + j.anlasilmayan.join(', ') }) : null,
        onayKutu,
        h(belge, 'div', { class: 'taai-sira' }, [
          h(belge, 'button', { type: 'button', class: 'taai-ana', id: 'taai-uygula', text: 'Uygula', on: { click: () => uygula(j) } }),
          h(belge, 'button', { type: 'button', id: 'taai-baska', text: 'Başka öneri', on: { click: () => { durum.cesit++; hazirla(false); } } }),
          app.onizle ? h(belge, 'button', { type: 'button', text: 'Önizle', on: { click: () => { try { app.onizle(j.senaryo); } catch (e) { durumYazi.textContent = hataMetni(e); } } } }) : null
        ])
      ]));
    }

    function hazirla(onay) {
      const m = alan.value.trim();
      if (durum.mesgul) return Promise.resolve();
      if (!onay && durum.sonMetin !== m) durum.cesit = 0;
      durum.sonMetin = m;
      mesgul(true, m ? 'Hazırlanıyor…' : 'Tarif boş; hazır öneri getiriliyor…');
      return api('/tarif/hazirla', istekGovdesi(m, durum.cesit, onay)).then(r => r.json())
        .then(j => { mesgul(false, adimOzeti(j)); goster(j); return j; })
        .catch(e => { mesgul(false, ''); goster({ ok: false, ozet: hataMetni(e) }); });
    }
    function uygula(j) {
      try {
        if (app.senaryoDegistir) app.senaryoDegistir(j.senaryo);
        else if (app.senaryoyaEkle) app.senaryoyaEkle(j.senaryo);
        durumYazi.textContent = 'Senaryo uygulandı. Videoyu yapmak için “Videoyu yap”a basın.';
      } catch (e) { durumYazi.textContent = hataMetni(e); }
    }
    function kopyala(m, ok) {
      const pano = kok.navigator && kok.navigator.clipboard;
      const p = pano && pano.writeText ? pano.writeText(m) : Promise.reject(new Error('pano yok'));
      return p.then(() => { durumYazi.textContent = ok; return true; }, () => { durumYazi.textContent = 'Panoya kopyalanamadı.'; return false; });
    }
    function istemKopyala() {
      const m = alan.value.trim();
      if (!m) { durumYazi.textContent = 'Önce videoyu yukarıya yazıyla anlatın.'; return Promise.resolve(false); }
      return api('/tarif/istem', m).then(r => r.text()).then(t => kopyala(t, 'İstem panoya kopyalandı. Sohbet sitesine yapıştırın.')).catch(e => { durumYazi.textContent = hataMetni(e); return false; });
    }
    function siteAc(ad) { return istemKopyala().then(ok => { if (ok) return api('/tarif/ac', ad).catch(e => { durumYazi.textContent = hataMetni(e); }); }); }
    function yanitUygula() {
      const y = yanitAlan.value.trim();
      if (!y) { durumYazi.textContent = 'Önce yanıtı yapıştırın.'; return Promise.resolve(); }
      return api('/tarif/uygula', y).then(r => r.json()).then(j => { if (j.ok) j.kaynak = 'yapay-zeka', j.adimlar = []; goster(j); }).catch(e => goster({ ok: false, ozet: hataMetni(e) }));
    }
    function llmYukle() {
      return api('/tarif/llm-durum').then(r => r.json()).then(d => {
        temizle(llmSec);
        (d.saglayicilar || []).forEach(s => { const o = h(belge, 'option', { value: s.id, text: s.ad }); if (s.id === d.saglayici) o.setAttribute('selected', ''); llmSec.appendChild(o); });
        llmSec.value = d.saglayici; llmModel.value = d.model || ''; llmAdres.value = d.adres || '';
        llmDurum.textContent = d.acik ? '(açık)' : '(kapalı)';
      }).catch(() => { llmDurum.textContent = '(okunamadı)'; });
    }
    function llmKaydet(ac, kaldir) {
      const g = { acik: !!ac, sorma: !!ac, saglayici: llmSec.value, model: llmModel.value.trim(), adres: llmAdres.value.trim() };
      if (llmAnahtar.value.trim()) g.anahtar = llmAnahtar.value.trim();
      if (kaldir) g.anahtar_kaldir = true;
      return api('/tarif/llm-ayar', JSON.stringify(g)).then(() => { llmAnahtar.value = ''; durumYazi.textContent = kaldir ? 'Anahtar kaldırıldı.' : ac ? 'Doğrudan bağlantı açıldı.' : 'Doğrudan bağlantı kapatıldı.'; return llmYukle(); })
        .catch(e => { llmAnahtar.value = ''; durumYazi.textContent = hataMetni(e); });
    }

    hazirlaBtn.addEventListener('click', () => hazirla(false));
    iptalBtn.addEventListener('click', () => { api('/tarif/iptal', '').catch(() => {}); durumYazi.textContent = 'Durduruluyor…'; });
    alan.addEventListener('keydown', e => { if (e.key === 'Enter' && (e.ctrlKey || e.metaKey)) { e.preventDefault && e.preventDefault(); hazirla(false); } });
    api('/tarif/siteler').then(r => r.json()).then(l => { temizle(siteSira); l.forEach(ad => siteSira.appendChild(h(belge, 'button', { type: 'button', text: ad + '’de aç', on: { click: () => siteAc(ad) } }))); }).catch(() => {});
    llmYukle();

    return { hazirla, uygula, goster, yanitUygula, istemKopyala, siteAc, el: kokEl, durum, alan };
  }

  const TaaAI = { mount, _ic: { kaynakAdi, adimOzeti, hataMetni, istekGovdesi, sahneSatirlari, ORNEKLER } };
  kok.TaaAI = TaaAI;
  if (typeof module !== 'undefined' && module.exports) module.exports = TaaAI;
})(typeof window !== 'undefined' ? window : globalThis);
