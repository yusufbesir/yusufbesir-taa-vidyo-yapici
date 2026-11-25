/* TAA Vidyo Yapıcı: "Yapay zekâ" paneli (Yaz → Hazırla → Uygula → Önizle).
 * Sahip: Yusuf. Yazan: [Claude#AI Opus 5.5, yenileme-1.0, 2026-11-18]; dalga 3 [Claude#AI Opus 5.5, yenileme-1.0, 2026-11-25]:
 *   adım göstergesi, Hazırla metin kutusunun hemen altında, Geri al (önceki senaryoya dön), 35 sn YZ bütçesi + Durdur (AbortController → anında yerel sonuç),
 *   geçici önizleme (TaaApp.geciciOnizle), kapalı "Diğer araçlar" (İstemi göster, Kuraldan çöz, Panodaki tarifi uygula, Kural listesi/ekle, + Site),
 *   işletme adı ayarı ('taa-isletme', kapatılabilir), "Kaydet ve aç" onayı kalıcı kapatmaz ("Her seferinde sor", varsayılan açık).
 * Yalnız yazan yapay zekâ ya da Yusuf'un izniyle değişir; silme yok.
 * Arayüz: window.TaaAI = { mount(el, app) }. app = window.TaaApp (UI ajanı sağlar): senaryoMetni(), senaryoDegistir(m), geciciOnizle(m) | onizle(m).
 * Uçlar: docs/TARIF-API.md (POST /tarif/hazirla, /tarif/iptal, /tarif/istem, /tarif/uygula, /tarif/kural-coz, /tarif/kural-ekle, /tarif/site-ekle, /tarif/ac,
 *   GET /tarif/siteler, /tarif/kurallar, /tarif/llm-durum, POST /tarif/llm-ayar).
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
    'Yılbaşı kampanyası, havai fişek olsun',
    'Kar yağsın, kış indirimi',
    'Önce hoş geldiniz, sonra %30 indirim konfetili'
  ];
  const KAYNAK_AD = { 'yapay-zeka': 'Yapay zekâ', kural: 'Kural', sablon: 'Şablon' };
  const ADIMLAR = ['Yaz', 'Hazırla', 'Uygula', 'Önizle'];
  /** Yapay zekâ için toplam bekleme (ms). Sonra istek kesilir ve yerel kurallarla hazırlanır. */
  const YZ_BUTCE_MS = 35000;
  const ISLETME_ANAHTAR = 'taa-isletme';
  const ISLETME_VARSAYILAN = 'TERZİ ADEM ALTUN';
  const KATEGORI_AD = { giris: 'Giriş hareketi', cikis: 'Çıkış hareketi', efekt: 'Efekt', palet: 'Renk', font: 'Yazı tipi', gecis: 'Geçiş', gorunum: 'Görünüm' };

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
    if (/abort/i.test(m) || (e && e.name === 'AbortError')) return 'İşlem durduruldu.';
    if (/^HTTP 4/.test(m)) return 'İstek kabul edilmedi.';
    if (/^HTTP 5/.test(m)) return 'Program içinde bir hata oluştu.';
    return m && m.length < 160 ? m : 'Bir sorun oluştu. Yeniden deneyin.';
  }
  /** İşletme adı ayarı: undefined → sunucu varsayılanı; '' → işletme adı sahnesi yok; aksi halde ad. */
  function isletmeAyari(depo) {
    let v = null;
    try { v = depo && depo.getItem ? depo.getItem(ISLETME_ANAHTAR) : null; } catch (e) { v = null; }
    if (v === null || v === undefined) return undefined;
    v = String(v).trim();
    return v === 'kapali' ? '' : v.slice(0, 80);
  }
  /** /tarif/hazirla istek gövdesi. ek: { isletme, yz } (isteğe bağlı). */
  function istekGovdesi(metin, cesit, onay, ek) {
    const g = { metin: String(metin || '').slice(0, 4000), cesit: Math.max(0, cesit | 0) };
    if (onay) g.onay = true;
    if (ek && typeof ek.isletme === 'string') g.isletme = ek.isletme;
    if (ek && ek.yz === false) g.yz = false;
    return JSON.stringify(g);
  }
  /** /tarif/istem gövdesi (işletme ayarı varsa JSON, yoksa düz metin: eski sunucularla uyumlu). */
  function istemGovdesi(metin, isletme) { return typeof isletme === 'string' ? JSON.stringify({ metin: String(metin || '').slice(0, 4000), isletme }) : String(metin || ''); }
  /** Bir sahnenin satırları (yeni "satirlar" dizisi ya da eski "metin" alanı). */
  function sahneSatirlari(s) { return Array.isArray(s.satirlar) ? s.satirlar : String(s.metin || '').split(' / ').filter(x => x !== ''); }

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
.taai textarea{width:100%;min-height:104px;padding:12px 14px;font:inherit;color:var(--tx);background:var(--panel2);border:1px solid var(--line);border-radius:var(--r-kart,10px);resize:vertical}
.taai textarea:focus-visible,.taai button:focus-visible,.taai select:focus-visible,.taai input:focus-visible,.taai summary:focus-visible{outline:3px solid var(--ac);outline-offset:2px}
.taai-cipler{display:flex;flex-wrap:wrap;gap:8px}
.taai button{min-height:48px;padding:0 18px;font:inherit;font-size:16px;color:var(--tx);background:var(--panel2);border:1px solid var(--line);border-radius:var(--r-btn,10px);cursor:pointer}
.taai button:hover{border-color:var(--ac)}
.taai button[disabled]{opacity:.55;cursor:default}
.taai .taai-cip{min-height:48px;border-radius:24px;font-size:15px}
.taai .taai-ana{min-height:56px;padding:0 28px;font-weight:700;background:var(--panel2);color:var(--tx);border:2px solid var(--ac)}
.taai-sira{display:flex;flex-wrap:wrap;gap:10px;align-items:center}
.taai-durum{color:var(--mut);min-height:24px}
.taai-adim{display:flex;flex-wrap:wrap;gap:6px;margin:0;padding:0;list-style:none}
.taai-adim li{display:inline-flex;align-items:center;gap:6px;min-height:32px;padding:0 12px;border:1px solid var(--line);border-radius:16px;font-size:14px;color:var(--mut);background:var(--panel)}
.taai-adim li[data-d="tamam"]{color:var(--ok);border-color:var(--ok)}
.taai-adim li[aria-current="step"]{color:var(--tx);border:2px solid var(--ac);font-weight:700}
.taai-rozet{display:inline-flex;align-items:center;min-height:32px;padding:0 12px;border-radius:16px;border:1px solid var(--line);background:var(--panel);font-size:14px;font-weight:700}
.taai-rozet[data-k="yapay-zeka"]{border-color:var(--ok);color:var(--ok)}
.taai-rozet[data-k="kural"]{border-color:var(--ac);color:var(--ac)}
.taai-rozet[data-k="sablon"]{border-color:var(--uy);color:var(--uy)}
.taai-sonuc{display:grid;gap:10px;padding:14px;border:1px solid var(--line);border-radius:var(--r-kart,12px);background:var(--panel)}
.taai-sahneler{display:grid;grid-template-columns:repeat(auto-fill,minmax(120px,1fr));gap:10px}
.taai-sahne{display:grid;gap:4px;align-content:start;padding:10px;border:1px solid var(--line);border-radius:10px;background:var(--bg);min-height:120px}
.taai-sahne b{font-size:17px;line-height:1.15;overflow-wrap:anywhere}
.taai-sahne small,.taai small{color:var(--mut);font-size:13px}
.taai ul{margin:0;padding-left:20px}
.taai .taai-uyari{color:var(--uy)}
.taai .taai-hata{color:var(--uy);font-weight:700}
.taai details{border:1px solid var(--line);border-radius:var(--r-kart,10px);background:var(--panel)}
.taai details>summary{min-height:48px;display:flex;align-items:center;gap:6px;padding:0 14px;cursor:pointer;font-weight:600}
.taai details>div{display:grid;gap:10px;padding:0 14px 14px}
.taai fieldset{display:grid;gap:8px;margin:0;padding:10px 12px;border:1px solid var(--line);border-radius:10px}
.taai legend{font-weight:700;padding:0 4px}
.taai select,.taai input[type=text],.taai input[type=url],.taai input[type=password]{min-height:48px;padding:0 12px;font:inherit;color:var(--tx);background:var(--panel2);border:1px solid var(--line);border-radius:10px}
.taai label.taai-kutu{display:flex;align-items:center;gap:10px;min-height:48px;cursor:pointer}
.taai label.taai-kutu input{width:24px;height:24px;accent-color:var(--ac)}
.taai textarea[readonly]{min-height:160px;font-size:13px;color:var(--mut)}
@media (prefers-reduced-motion:no-preference){.taai-sonuc{animation:taai-gir .18s ease-out}@keyframes taai-gir{from{opacity:0;transform:translateY(4px)}to{opacity:1;transform:none}}}
`;

  function mount(el, app) {
    if (!el) throw new Error('TaaAI.mount: hedef öğe yok');
    const belge = el.ownerDocument || kok.document;
    const fetchFn = (kok.fetch || (typeof fetch !== 'undefined' ? fetch : null));
    const api = (yol, govde, sinyal) => fetchFn(yol, { method: govde === undefined ? 'GET' : 'POST', headers: { 'X-LED': '1' }, body: govde, credentials: 'same-origin', signal: sinyal })
      .then(r => { if (!r.ok) return r.text().then(t => { throw new Error(t && t.length < 200 ? t : 'HTTP ' + r.status); }); return r; });
    app = app || kok.TaaApp || {};
    const depo = (() => { try { return kok.localStorage || null; } catch (e) { return null; } })();
    const durum = { cesit: 0, son: null, mesgul: false, onceki: null, adim: 0, kontrol: null, sure: null };

    if (belge.getElementById && !belge.getElementById('taai-stil')) { const st = h(belge, 'style', { id: 'taai-stil' }); st.textContent = CSS; (belge.head || el).appendChild(st); }

    // ── adım göstergesi ──
    const adimListe = h(belge, 'ol', { class: 'taai-adim', 'aria-label': 'Adımlar' }, ADIMLAR.map((a, i) => h(belge, 'li', { text: (i + 1) + '. ' + a })));
    function adim(n) {
      durum.adim = n;
      adimListe.childNodes.forEach((li, i) => {
        if (i < n) li.setAttribute('data-d', 'tamam'); else li.setAttribute('data-d', '');
        if (i === n) li.setAttribute('aria-current', 'step'); else if (li.removeAttribute) li.removeAttribute('aria-current'); else li.setAttribute('aria-current', 'false');
      });
    }

    // ── üst: anlatım ──
    const alan = h(belge, 'textarea', { id: 'taai-metin', rows: '3', spellcheck: 'true', placeholder: 'Örn: Takım elbiselerde yüzde 20 indirim, kırmızı olsun, sonra 3 taksit.', 'aria-describedby': 'taai-ipucu' });
    const hazirlaBtn = h(belge, 'button', { type: 'button', class: 'taai-ana', id: 'taai-hazirla', text: 'Hazırla' });
    const iptalBtn = h(belge, 'button', { type: 'button', id: 'taai-iptal', text: 'Durdur', hidden: true });
    const durumYazi = h(belge, 'div', { class: 'taai-durum', role: 'status', 'aria-live': 'polite' });
    const cipler = h(belge, 'div', { class: 'taai-cipler', role: 'group', 'aria-label': 'Örnek öneriler' },
      ORNEKLER.map(o => h(belge, 'button', { type: 'button', class: 'taai-cip', text: o, on: { click: () => { alan.value = o; adim(0); alan.focus && alan.focus(); } } })));
    const sonucKutu = h(belge, 'div', { id: 'taai-sonuc', 'aria-live': 'polite' });
    const geriBtn = h(belge, 'button', { type: 'button', id: 'taai-geri', text: 'Geri al (önceki senaryoya dön)', hidden: true, on: { click: () => geriAl() } });

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

    // ── ikincil: diğer araçlar (kapalı) ──
    const isletmeAd = h(belge, 'input', { id: 'taai-isletme', type: 'text', autocomplete: 'off', 'aria-label': 'İşletme adı', placeholder: ISLETME_VARSAYILAN });
    const isletmeAcik = h(belge, 'input', { id: 'taai-isletme-acik', type: 'checkbox' });
    const istemAlan = h(belge, 'textarea', { id: 'taai-istem', readonly: true, hidden: true, 'aria-label': 'Yapay zekâya gidecek istem' });
    const kuralListe = h(belge, 'ul', { id: 'taai-kurallar', 'aria-live': 'polite' });
    const kuralK = h(belge, 'select', { id: 'taai-kural-k', 'aria-label': 'Kural türü' }, Object.keys(KATEGORI_AD).map(k => h(belge, 'option', { value: k, text: KATEGORI_AD[k] })));
    const kuralI = h(belge, 'input', { id: 'taai-kural-i', type: 'text', autocomplete: 'off', 'aria-label': 'Yazdığınız ifade', placeholder: 'Sizin ifadeniz (örn. hoplayarak)' });
    const kuralH = h(belge, 'input', { id: 'taai-kural-h', type: 'text', autocomplete: 'off', 'aria-label': 'Programdaki karşılığı', placeholder: 'Program adı (örn. zipla)' });
    const siteAd = h(belge, 'input', { id: 'taai-site-ad', type: 'text', autocomplete: 'off', 'aria-label': 'Site adı', placeholder: 'Site adı' });
    const siteAdres = h(belge, 'input', { id: 'taai-site-adres', type: 'url', autocomplete: 'off', 'aria-label': 'Site adresi', placeholder: 'https://…' });
    const digerBolum = h(belge, 'details', { id: 'taai-diger' }, [
      h(belge, 'summary', { text: 'Diğer araçlar' }),
      h(belge, 'div', {}, [
        h(belge, 'fieldset', {}, [h(belge, 'legend', { text: 'İşletme adı' }),
          h(belge, 'label', { class: 'taai-kutu', for: 'taai-isletme-acik' }, [isletmeAcik, 'Kampanya videosunun sonuna işletme adını ekle']),
          isletmeAd,
          h(belge, 'div', { class: 'taai-sira' }, [h(belge, 'button', { type: 'button', id: 'taai-isletme-kaydet', text: 'Kaydet', on: { click: () => isletmeKaydet() } })])]),
        h(belge, 'fieldset', {}, [h(belge, 'legend', { text: 'Tarif araçları' }),
          h(belge, 'div', { class: 'taai-sira' }, [
            h(belge, 'button', { type: 'button', id: 'taai-istem-goster', text: 'İstemi göster', on: { click: () => istemGoster() } }),
            h(belge, 'button', { type: 'button', id: 'taai-kural-coz', text: 'Kuraldan çöz', on: { click: () => kuraldanCoz() } }),
            h(belge, 'button', { type: 'button', id: 'taai-pano', text: 'Panodaki tarifi uygula', on: { click: () => panodanUygula() } })]),
          istemAlan]),
        h(belge, 'fieldset', {}, [h(belge, 'legend', { text: 'Kurallar' }),
          h(belge, 'small', { text: 'Kendi sözcüğünüzü programdaki bir harekete, renge ya da efekte bağlayın.' }),
          kuralK, kuralI, kuralH,
          h(belge, 'div', { class: 'taai-sira' }, [
            h(belge, 'button', { type: 'button', id: 'taai-kural-ekle', text: 'Kural ekle', on: { click: () => kuralEkle() } }),
            h(belge, 'button', { type: 'button', id: 'taai-kural-liste', text: 'Kural listesi', on: { click: () => kurallariYukle() } })]),
          kuralListe]),
        h(belge, 'fieldset', {}, [h(belge, 'legend', { text: 'Sohbet sitesi ekle' }),
          siteAd, siteAdres,
          h(belge, 'div', { class: 'taai-sira' }, [h(belge, 'button', { type: 'button', id: 'taai-site-ekle', text: '+ Site', on: { click: () => siteEkle() } })])])
      ])
    ]);

    // ── ikincil: doğrudan bağlantı ayarı ──
    const llmDurum = h(belge, 'small', { text: '' });
    const llmSec = h(belge, 'select', { id: 'taai-saglayici', 'aria-label': 'Yapay zekâ hizmeti' });
    const llmModel = h(belge, 'input', { id: 'taai-model', type: 'text', placeholder: 'Model (boş: önerilen)', 'aria-label': 'Model adı', autocomplete: 'off' });
    const llmAdres = h(belge, 'input', { id: 'taai-adres', type: 'url', placeholder: 'https://… (yalnız “Diğer” için)', 'aria-label': 'Özel adres', autocomplete: 'off' });
    const llmAnahtar = h(belge, 'input', { id: 'taai-anahtar', type: 'password', placeholder: 'Anahtar (kaydedilir, bir daha gösterilmez)', 'aria-label': 'Anahtar', autocomplete: 'off' });
    const llmSor = h(belge, 'input', { id: 'taai-sor', type: 'checkbox', checked: true });
    llmSor.checked = true;
    const llmAyarBolum = h(belge, 'details', { id: 'taai-llm' }, [
      h(belge, 'summary', {}, ['Doğrudan bağlantı (isteğe bağlı) ', llmDurum]),
      h(belge, 'div', {}, [
        h(belge, 'small', { text: 'Açarsanız tarifiniz ve program katalog adları seçtiğiniz hizmete internetten gönderilir. Anahtar Windows kullanıcı şifrelemesiyle saklanır. Bağlantı olmazsa ya da 35 sn içinde yanıt gelmezse program yerel kurallarla hazırlar.' }),
        llmSec, llmModel, llmAdres, llmAnahtar,
        h(belge, 'label', { class: 'taai-kutu', for: 'taai-sor' }, [llmSor, 'Her seferinde sor (tarif gönderilmeden önce onay iste)']),
        h(belge, 'div', { class: 'taai-sira' }, [
          h(belge, 'button', { type: 'button', text: 'Kaydet ve aç', on: { click: () => llmKaydet(true) } }),
          h(belge, 'button', { type: 'button', text: 'Kapat', on: { click: () => llmKaydet(false) } }),
          h(belge, 'button', { type: 'button', text: 'Anahtarı kaldır', on: { click: () => llmKaydet(false, true) } })])
      ])
    ]);

    const kokEl = h(belge, 'section', { class: 'taai', 'aria-label': 'Yapay zekâ ile video hazırla' }, [
      adimListe,
      h(belge, 'label', { class: 'taai-bas', for: 'taai-metin', text: 'Videoyu anlatın' }),
      h(belge, 'small', { id: 'taai-ipucu', text: 'Ne yazsın, hangi renk ya da hareket olsun, düz cümleyle yazın. Kişisel bilgi (telefon, müşteri adı) yazmayın.' }),
      alan,
      h(belge, 'div', { class: 'taai-sira' }, [hazirlaBtn, iptalBtn, geriBtn]),
      durumYazi, cipler, sonucKutu, disBolum, digerBolum, llmAyarBolum
    ]);
    while (el.firstChild) el.removeChild(el.firstChild);
    el.appendChild(kokEl);
    adim(0);

    function mesgul(v, yazi) { durum.mesgul = v; hazirlaBtn.disabled = v; iptalBtn.hidden = !v; durumYazi.textContent = yazi || ''; }
    function temizle(e) { while (e.firstChild) e.removeChild(e.firstChild); }
    function liste(baslik, v, sinif) { if (!v || !v.length) return null; return h(belge, 'div', { class: sinif || '' }, [h(belge, 'b', { text: baslik }), h(belge, 'ul', {}, v.map(x => h(belge, 'li', { text: x })))]); }
    function yaz(m) { durumYazi.textContent = m; }

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
      adim(2);
      const k = j.kaynak || 'kural';
      const sahneler = h(belge, 'div', { class: 'taai-sahneler' }, (j.sahneler || []).map((s, i) => h(belge, 'div', { class: 'taai-sahne' }, [
        h(belge, 'small', { text: 'Sahne ' + (i + 1) + ' · ' + s.sure + ' sn' }),
        ...sahneSatirlari(s).map(x => h(belge, 'b', { text: x })),
        s.malzeme ? h(belge, 'small', { text: 'Görsel: ' + s.malzeme }) : null,
        h(belge, 'small', { text: [s.giris, s.efekt, s.palet].filter(Boolean).join(' · ') })
      ])));
      const onayKutu = j.yz_onay_gerekli ? h(belge, 'div', { class: 'taai-sira' }, [
        h(belge, 'small', { text: 'Daha iyi sonuç için tarifi bağlı yapay zekâ hizmetine gönderebilirsiniz.' }),
        h(belge, 'button', { type: 'button', text: 'Yapay zekâya gönder', on: { click: () => hazirla(true) } })]) : null;
      sonucKutu.appendChild(h(belge, 'div', { class: 'taai-sonuc' }, [
        h(belge, 'div', { class: 'taai-sira' }, [h(belge, 'span', { class: 'taai-rozet', 'data-k': k, text: kaynakAdi(k) }), h(belge, 'small', { text: (j.sahneler || []).length + ' sahne · ' + j.sure + ' sn · ' + (durum.ekBilgi || adimOzeti(j)) })]),
        sahneler,
        liste('Varsayımlarım:', j.varsayimlar, 'taai-uyari'),
        liste('Not:', j.notlar),
        j.anlasilmayan && j.anlasilmayan.length ? h(belge, 'small', { text: 'Kullanmadığım sözcükler: ' + j.anlasilmayan.join(', ') }) : null,
        onayKutu,
        h(belge, 'div', { class: 'taai-sira' }, [
          h(belge, 'button', { type: 'button', class: 'taai-ana', id: 'taai-uygula', text: 'Uygula', on: { click: () => uygula(j) } }),
          (app.geciciOnizle || app.onizle) ? h(belge, 'button', { type: 'button', id: 'taai-onizle', text: 'Önizle', on: { click: () => onizle(j) } }) : null,
          h(belge, 'button', { type: 'button', id: 'taai-baska', text: 'Başka öneri', on: { click: () => { durum.cesit++; hazirla(false); } } })
        ])
      ]));
      durum.ekBilgi = null;
    }

    function sureyiBitir() { if (durum.sure) { clearTimeout(durum.sure); durum.sure = null; } durum.kontrol = null; }
    /** Hazırla: yapay zekâ en çok 35 sn; Durdur ya da süre dolarsa istek kesilir ve yerel kurallarla hazırlanır. */
    function hazirla(onay, yerel) {
      const m = alan.value.trim();
      if (durum.mesgul) return Promise.resolve();
      if (!onay && durum.sonMetin !== m) durum.cesit = 0;
      durum.sonMetin = m;
      adim(1);
      mesgul(true, yerel ? 'Yerel kurallarla hazırlanıyor…' : m ? 'Hazırlanıyor…' : 'Tarif boş; hazır öneri getiriliyor…');
      const Kontrol = kok.AbortController || (typeof AbortController !== 'undefined' ? AbortController : null);
      const kontrol = Kontrol ? new Kontrol() : null;
      durum.kontrol = kontrol;
      const govde = istekGovdesi(m, durum.cesit, onay, { isletme: isletmeAyari(depo), yz: yerel ? false : undefined });
      if (kontrol && !yerel) durum.sure = setTimeout(() => durdur('sure'), YZ_BUTCE_MS);
      return api('/tarif/hazirla', govde, kontrol ? kontrol.signal : undefined).then(r => r.json())
        .then(j => { if (durum.kontrol !== kontrol) return j; sureyiBitir(); mesgul(false, durum.ekBilgi || adimOzeti(j)); goster(j); return j; })
        .catch(e => {
          if (durum.kontrol !== kontrol) return; // durduruldu: yerel istek sürüyor
          sureyiBitir(); mesgul(false, ''); goster({ ok: false, ozet: hataMetni(e) });
        });
    }
    /** Durdur (ya da 35 sn bütçe doldu): YZ isteğini keser, sunucuya iptal bildirir, anında yerel sonuca geçer. */
    function durdur(neden) {
      if (!durum.mesgul) return Promise.resolve();
      const k = durum.kontrol;
      sureyiBitir();
      try { if (k) k.abort(); } catch (e) { /* yoksay */ }
      api('/tarif/iptal', '').catch(() => {});
      durum.mesgul = false;
      durum.ekBilgi = neden === 'sure' ? 'Yapay zekâ 35 sn içinde yanıt vermedi; yerel kurallarla hazırlandı.' : 'Durduruldu; yerel kurallarla hazırlandı.';
      return hazirla(false, true);
    }
    function uygula(j) {
      try {
        const once = typeof app.senaryoMetni === 'function' ? app.senaryoMetni() : null;
        if (app.senaryoDegistir) app.senaryoDegistir(j.senaryo, 'Yapay zekâ');
        else if (app.senaryoyaEkle) app.senaryoyaEkle(j.senaryo, 'Yapay zekâ');
        if (typeof once === 'string') { durum.onceki = once; geriBtn.hidden = false; }
        adim(3);
        yaz('Senaryo uygulandı. Önizleyin ya da “Videoyu yap”a basın.');
      } catch (e) { yaz(hataMetni(e)); }
    }
    /** Geri al: Uygula'dan önceki senaryoya döner. */
    function geriAl() {
      if (typeof durum.onceki !== 'string' || !app.senaryoDegistir) { yaz('Geri alınacak bir değişiklik yok.'); return false; }
      try { app.senaryoDegistir(durum.onceki, 'Yapay zekâ (geri al)'); } catch (e) { yaz(hataMetni(e)); return false; }
      durum.onceki = null; geriBtn.hidden = true; adim(2);
      yaz('Önceki senaryoya dönüldü.');
      return true;
    }
    /** Önizle: senaryoyu değiştirmeden geçici önizleme (TaaApp.geciciOnizle); yoksa eski onizle. */
    function onizle(j) {
      try {
        if (typeof app.geciciOnizle === 'function') { app.geciciOnizle(j.senaryo); yaz('Önizleme gösteriliyor; senaryonuz değişmedi. Beğendiyseniz “Uygula”ya basın.'); }
        else if (typeof app.onizle === 'function') { app.onizle(j.senaryo); }
        adim(durum.adim >= 3 ? 4 : durum.adim);
      } catch (e) { yaz(hataMetni(e)); }
    }
    function kopyala(m, ok) {
      const pano = kok.navigator && kok.navigator.clipboard;
      const p = pano && pano.writeText ? pano.writeText(m) : Promise.reject(new Error('pano yok'));
      return p.then(() => { yaz(ok); return true; }, () => { yaz('Panoya kopyalanamadı.'); return false; });
    }
    function istemAl() {
      const m = alan.value.trim();
      if (!m) { yaz('Önce videoyu yukarıya yazıyla anlatın.'); return Promise.resolve(null); }
      return api('/tarif/istem', istemGovdesi(m, isletmeAyari(depo))).then(r => r.text()).catch(e => { yaz(hataMetni(e)); return null; });
    }
    function istemKopyala() { return istemAl().then(t => t === null ? false : kopyala(t, 'İstem panoya kopyalandı. Sohbet sitesine yapıştırın.')); }
    function istemGoster() {
      return istemAl().then(t => { if (t === null) return false; istemAlan.value = t; istemAlan.hidden = false; yaz('Yapay zekâya gidecek istem aşağıda.'); return true; });
    }
    function siteAc(ad) { return istemKopyala().then(ok => { if (ok) return api('/tarif/ac', ad).catch(e => { yaz(hataMetni(e)); }); }); }
    function yanitIsle(y) {
      return api('/tarif/uygula', y).then(r => r.json()).then(j => { if (j.ok) { j.kaynak = 'yapay-zeka'; j.adimlar = []; } goster(j); return j; }).catch(e => goster({ ok: false, ozet: hataMetni(e) }));
    }
    function yanitUygula() {
      const y = yanitAlan.value.trim();
      if (!y) { yaz('Önce yanıtı yapıştırın.'); return Promise.resolve(); }
      return yanitIsle(y);
    }
    /** Panodaki tarifi (yapay zekâ yanıtı JSON'u) doğrulayıp gösterir. */
    function panodanUygula() {
      const pano = kok.navigator && kok.navigator.clipboard;
      if (!pano || !pano.readText) { yaz('Pano okunamadı; yanıtı “Sohbet sitesinde hazırlat” bölümüne yapıştırın.'); return Promise.resolve(); }
      return pano.readText().then(t => { t = String(t || '').trim(); if (!t) { yaz('Pano boş.'); return; } return yanitIsle(t); }, () => { yaz('Pano okunamadı; tarayıcı izin vermedi.'); });
    }
    /** Kuraldan çöz: yapay zekâsız, yalnız kural sözlüğüyle (metin aynen). */
    function kuraldanCoz() {
      const m = alan.value.trim();
      if (!m) { yaz('Önce videoyu yukarıya yazıyla anlatın.'); return Promise.resolve(); }
      return api('/tarif/kural-coz', m).then(r => r.json()).then(j => { if (j.ok) { j.kaynak = 'kural'; j.adimlar = []; } else if (!j.ozet) j.ozet = 'Kurallarla çözülemedi.'; goster(j); return j; }).catch(e => goster({ ok: false, ozet: hataMetni(e) }));
    }
    function kurallariYukle() {
      return api('/tarif/kurallar').then(r => r.json()).then(l => {
        temizle(kuralListe);
        const benim = l.filter(r => r.kaynak === 'benim');
        kuralListe.appendChild(h(belge, 'li', { text: l.length + ' kural (' + benim.length + ' tanesi sizin).' }));
        benim.concat(l.filter(r => r.kaynak !== 'benim').slice(0, 20)).forEach(r => kuralListe.appendChild(h(belge, 'li', { text: (KATEGORI_AD[r.k] || r.k) + ': ' + (r.i || []).join(', ') + ' → ' + r.h + (r.kaynak === 'benim' ? ' (sizin)' : '') })));
        return l;
      }).catch(e => { yaz(hataMetni(e)); return null; });
    }
    function kuralEkle() {
      const g = { k: kuralK.value || 'giris', i: kuralI.value.trim(), h: kuralH.value.trim() };
      if (!g.i || !g.h) { yaz('İfade ve program adı boş olamaz.'); return Promise.resolve(false); }
      return api('/tarif/kural-ekle', JSON.stringify(g)).then(() => { kuralI.value = ''; kuralH.value = ''; yaz('Kural eklendi.'); return kurallariYukle().then(() => true); })
        .catch(e => { yaz(hataMetni(e)); return false; });
    }
    function sitelerYukle() {
      return api('/tarif/siteler').then(r => r.json()).then(l => { temizle(siteSira); l.forEach(ad => siteSira.appendChild(h(belge, 'button', { type: 'button', text: ad + '’de aç', on: { click: () => siteAc(ad) } }))); }).catch(() => {});
    }
    function siteEkle() {
      const g = { ad: siteAd.value.trim(), adres: siteAdres.value.trim() };
      if (!g.ad || !g.adres) { yaz('Site adı ve https adresi gerekli.'); return Promise.resolve(false); }
      return api('/tarif/site-ekle', JSON.stringify(g)).then(() => { siteAd.value = ''; siteAdres.value = ''; yaz('Site eklendi.'); return sitelerYukle().then(() => true); })
        .catch(e => { yaz(hataMetni(e)); return false; });
    }
    function isletmeYukle() {
      const v = isletmeAyari(depo);
      isletmeAcik.checked = v !== '';
      isletmeAd.value = v === undefined ? ISLETME_VARSAYILAN : v;
    }
    /** İşletme adı ayarı: işaretsiz → 'kapali' (imza sahnesi yok); boş ad → varsayılan. */
    function isletmeKaydet() {
      const ad = isletmeAd.value.trim();
      const deger = !isletmeAcik.checked ? 'kapali' : (ad || ISLETME_VARSAYILAN);
      try { if (depo) depo.setItem(ISLETME_ANAHTAR, deger); } catch (e) { yaz('Ayar kaydedilemedi.'); return false; }
      yaz(deger === 'kapali' ? 'İşletme adı videolara eklenmeyecek.' : 'İşletme adı kaydedildi: ' + deger);
      return true;
    }
    function llmYukle() {
      return api('/tarif/llm-durum').then(r => r.json()).then(d => {
        temizle(llmSec);
        (d.saglayicilar || []).forEach(s => { const o = h(belge, 'option', { value: s.id, text: s.ad }); if (s.id === d.saglayici) o.setAttribute('selected', ''); llmSec.appendChild(o); });
        llmSec.value = d.saglayici; llmModel.value = d.model || ''; llmAdres.value = d.adres || '';
        llmSor.checked = !d.sorma;
        llmDurum.textContent = d.acik ? '(açık)' : '(kapalı)';
      }).catch(() => { llmDurum.textContent = '(okunamadı)'; });
    }
    function llmKaydet(ac, kaldir) {
      // "Her seferinde sor" işaretliyse (varsayılan) gönderim öncesi onay sorulmaya devam eder; "Kaydet ve aç" onayı kalıcı kapatmaz.
      const g = { acik: !!ac, sorma: !!ac && !llmSor.checked, saglayici: llmSec.value, model: llmModel.value.trim(), adres: llmAdres.value.trim() };
      if (llmAnahtar.value.trim()) g.anahtar = llmAnahtar.value.trim();
      if (kaldir) g.anahtar_kaldir = true;
      return api('/tarif/llm-ayar', JSON.stringify(g)).then(() => { llmAnahtar.value = ''; yaz(kaldir ? 'Anahtar kaldırıldı.' : ac ? 'Doğrudan bağlantı açıldı.' : 'Doğrudan bağlantı kapatıldı.'); return llmYukle(); })
        .catch(e => { llmAnahtar.value = ''; yaz(hataMetni(e)); });
    }

    hazirlaBtn.addEventListener('click', () => hazirla(false));
    iptalBtn.addEventListener('click', () => durdur('kullanici'));
    alan.addEventListener('keydown', e => { if (e.key === 'Enter' && (e.ctrlKey || e.metaKey)) { e.preventDefault && e.preventDefault(); hazirla(false); } });
    alan.addEventListener('input', () => { if (durum.adim !== 0 && !durum.mesgul) adim(0); });
    isletmeYukle();
    sitelerYukle();
    llmYukle();

    return { hazirla, durdur, uygula, geriAl, onizle, goster, yanitUygula, panodanUygula, kuraldanCoz, istemKopyala, istemGoster, kurallariYukle, kuralEkle, siteAc, siteEkle, isletmeKaydet, el: kokEl, durum, alan };
  }

  const TaaAI = { mount, _ic: { kaynakAdi, adimOzeti, hataMetni, istekGovdesi, istemGovdesi, isletmeAyari, sahneSatirlari, ORNEKLER, ADIMLAR, YZ_BUTCE_MS } };
  kok.TaaAI = TaaAI;
  if (typeof module !== 'undefined' && module.exports) module.exports = TaaAI;
})(typeof window !== 'undefined' ? window : globalThis);
