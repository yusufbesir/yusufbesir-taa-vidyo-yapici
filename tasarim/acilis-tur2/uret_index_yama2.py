# Açıklama: web/index.html'e 0.9.7 açılış mantığını (karıştırılmış torba + ayar bölümü) ekler. Tek seferlik yama; sahip Yusuf, yazan [Claude#Sonnet-5.5, 2026-11-18].
P = "D:/Programlar/TAA Vidyo Yapıcı/web/index.html"
h = open(P, encoding='utf-8').read()

# 1) betik: eski rastgele seçim yerine torba mantığı
bas = h.index("// Açılış: web/acilis/ içinden rastgele biri")
son = h.index("(() => { const s = document.getElementById('tema');")
yeni = r"""// TORBA-BASLA (saf mantık, DOM yok; tests/torba_test.js bu bloğu ayıklayıp dener)
// Açılış: 14 animasyon karıştırılmış torbadan sırayla oynar; hepsi bir kez oynamadan hiçbiri tekrar gelmez, yeni turun ilki öncekinin sonuncusu olmaz.
// Yalnız Ayrıntılar > "Program açılış animasyonları"nda açık olanlar torbaya girer.
const ACILIS_BILGI = [['zaman', 'Zaman çizelgesi', 'Claude Sonnet'], ['gece', 'Gece cephesi', 'Claude Sonnet'], ['kalibrasyon', 'Kalibrasyon', 'Claude Sonnet'], ['dikis', 'Dikiş', 'Claude Sonnet'], ['katlama', 'Katlama', 'Claude Sonnet'],
  ['led', 'LED uyanış', 'Claude Opus'], ['perde', 'Kumaş perde', 'Claude Opus'], ['kurgu', 'Kurgu kesmesi', 'Claude Opus'], ['iki-ekran', 'İki ekran', 'Claude Opus'], ['isik', 'Sahne ışığı', 'Claude Opus'],
  ['odak', 'Odak açılışı', 'agy Sonnet 4.6'], ['zaman-sol', 'Zaman çizelgesi', 'GPT Sol'], ['gece-terra', 'Gece cephesi', 'GPT Terra'], ['kalibrasyon-luna', 'Kalibrasyon', 'GPT Luna']];
const ACILISLAR = ACILIS_BILGI.map(x => x[0]);
const ACILIS_AYAR = 'taa-acilis-ayar', ACILIS_TORBA = 'taa-acilis-torba';
function acilisAyarOku(depo) { // {tam: açılış tamamen kapalı mı, kapali: kapatılan adlar}; bozuk/eksik kayıt = hepsi açık
  let o = null;
  try { o = JSON.parse(depo.get(ACILIS_AYAR) || 'null'); } catch (e) { o = null; }
  if (!o || typeof o !== 'object') o = {};
  return { tam: o.tam === true, kapali: Array.isArray(o.kapali) ? o.kapali.filter(x => ACILISLAR.includes(x)) : [] };
}
function acilisAyarYaz(depo, ayar) { // seçim değişince torba da sıfırlanır
  try { depo.set(ACILIS_AYAR, JSON.stringify({ tam: !!ayar.tam, kapali: ayar.kapali })); } catch (e) { return false; }
  try { depo.del(ACILIS_TORBA); } catch (e) { /* kalıcılık yok */ }
  return true;
}
function acilisAcikListe(ayar) { return ayar.tam ? [] : ACILISLAR.filter(x => !ayar.kapali.includes(x)); }
function acilisKaristir(d, rnd) { const a = d.slice(); for (let i = a.length - 1; i > 0; i--) { const j = Math.floor(rnd() * (i + 1)); [a[i], a[j]] = [a[j], a[i]]; } return a; }
function acilisSec(acik, depo, rnd) { // null = oynatma; tek açıksa hep o; yoksa torbadan
  if (!acik.length) return null;
  if (acik.length === 1) return acik[0];
  const imza = acik.join(',');
  let d = null;
  try { const t = depo.get(ACILIS_TORBA); if (t) d = JSON.parse(t); } catch (e) { d = null; }
  const gecerli = !!d && typeof d === 'object' && d.imza === imza && Array.isArray(d.kalan) && d.kalan.every(x => acik.includes(x)) && new Set(d.kalan).size === d.kalan.length;
  let kalan = gecerli ? d.kalan.slice() : [];
  const son = d && typeof d.son === 'string' && acik.includes(d.son) ? d.son : '';
  if (!kalan.length) {
    kalan = acilisKaristir(acik, rnd);
    if (kalan[0] === son) { const j = 1 + Math.floor(rnd() * (kalan.length - 1)); [kalan[0], kalan[j]] = [kalan[j], kalan[0]]; }
  }
  const secim = kalan.shift();
  try { depo.set(ACILIS_TORBA, JSON.stringify({ imza, kalan, son: secim })); }
  catch (e) { const adaylar = acik.filter(x => x !== son); return adaylar[Math.floor(rnd() * adaylar.length)]; } // depo yazılamıyor: rastgele, son tekrar etmez
  return secim;
}
// TORBA-BITIS
const acilisDepo = { get: k => localStorage.getItem(k), set: (k, v) => localStorage.setItem(k, v), del: k => localStorage.removeItem(k) };
(() => {
  const a = document.getElementById('yukleEkrani'), f = document.getElementById('acilisCerceve');
  if (!a || !f) return;
  const secim = acilisSec(acilisAcikListe(acilisAyarOku(acilisDepo)), acilisDepo, Math.random);
  if (!secim) { a.remove(); window.acilisKapat = () => {}; return; } // hiçbiri açık değil ya da tamamen kapalı: program doğrudan açılır
  let bitti = false, yuklendi = false, kapali = false;
  const kapat = () => { if (kapali) return; kapali = true; a.classList.add('kapan'); setTimeout(() => a.remove(), 220); window.removeEventListener('keydown', tus, true); };
  const dene = () => { if (bitti && yuklendi) kapat(); };
  const gec = () => { bitti = true; yuklendi = true; kapat(); };
  const tus = e => { if (e.key === 'Escape') gec(); };
  window.acilisKapat = () => { yuklendi = true; dene(); };
  window.addEventListener('message', e => { if (e.source === f.contentWindow && e.data === 'taa-acilis-gec') gec(); });
  window.addEventListener('keydown', tus, true);
  a.addEventListener('click', gec);
  const azHareket = window.matchMedia && matchMedia('(prefers-reduced-motion: reduce)').matches;
  f.src = '/acilis/' + secim + '.html?uygulama=1';
  setTimeout(() => { bitti = true; dene(); }, azHareket ? 400 : 4300); // animasyon ~3,5-4 sn; kapanış 0,2 sn solma ile toplam <= 4,5 sn
  setTimeout(gec, 6000); // yükleme ne olursa olsun katman 6 sn'de kalkar
})();
// Ayrıntılar > Program açılış animasyonları: tek tek aç/kapat, önizle, tümünü aç, tamamen kapat
(() => {
  const liste = document.getElementById('acilisListe'), tam = document.getElementById('acilisTam'), say = document.getElementById('acilisSay');
  if (!liste || !tam) return;
  let ayar = acilisAyarOku(acilisDepo);
  const kaydet = () => { acilisAyarYaz(acilisDepo, ayar); ciz(); };
  const onizle = ad => {
    const k = document.createElement('div'); k.style.cssText = 'position:fixed;inset:0;z-index:10000;background:#000';
    const fr = document.createElement('iframe'); fr.style.cssText = 'width:100%;height:100%;border:0'; fr.title = 'Açılış önizleme'; k.appendChild(fr); document.body.appendChild(k);
    const bit = () => { window.removeEventListener('message', ms); window.removeEventListener('keydown', ts, true); k.remove(); };
    const ms = e => { if (e.source === fr.contentWindow && e.data === 'taa-acilis-gec') bit(); }, ts = e => { if (e.key === 'Escape') bit(); };
    window.addEventListener('message', ms); window.addEventListener('keydown', ts, true);
    fr.src = '/acilis/' + ad + '.html?uygulama=1'; setTimeout(bit, 4500);
  };
  function ciz() {
    const acik = acilisAcikListe(ayar);
    tam.checked = ayar.tam;
    say.textContent = ayar.tam ? 'tamamen kapalı' : acik.length + ' / ' + ACILISLAR.length + ' açık';
    liste.innerHTML = '';
    ACILIS_BILGI.forEach(([ad, baslik, model]) => {
      const sat = document.createElement('div'); sat.className = 'sira'; sat.style.cssText = 'justify-content:space-between;gap:8px;margin:2px 0';
      const et = document.createElement('label'); et.style.cssText = 'display:flex;align-items:center;gap:8px;flex:1;min-width:0'; et.className = ayar.tam ? 'mut' : '';
      const kt = document.createElement('input'); kt.type = 'checkbox'; kt.checked = !ayar.kapali.includes(ad); kt.disabled = ayar.tam; kt.id = 'acilis-' + ad; kt.setAttribute('aria-label', baslik + ' (' + model + ')');
      kt.onchange = () => { ayar.kapali = ayar.kapali.filter(x => x !== ad); if (!kt.checked) ayar.kapali.push(ad); kaydet(); };
      const m = document.createElement('span'); m.innerHTML = '<b></b> <small class="mut"></small>'; m.firstChild.textContent = baslik; m.lastChild.textContent = model;
      et.append(kt, m);
      const bt = document.createElement('button'); bt.type = 'button'; bt.className = 'btn'; bt.textContent = 'Önizle'; bt.onclick = () => onizle(ad);
      sat.append(et, bt); liste.appendChild(sat);
    });
  }
  tam.onchange = () => { ayar.tam = tam.checked; kaydet(); };
  document.getElementById('acilisHepsi').onclick = () => { ayar = { tam: false, kapali: [] }; kaydet(); };
  ciz();
})();
"""
h = h[:bas] + yeni + h[son:]

# 2) Ayrıntılar içine bölüm (Gelişmiş senaryo metninden önce)
hedef = '      <details class="bolum"><summary>Gelişmiş senaryo metni</summary><div>'
assert hedef in h
bolum = ('      <details class="bolum" id="acilisBolum"><summary>Program açılış animasyonları <small class="mut" id="acilisSay"></small></summary><div>\n'
         '        <p class="mut" style="margin:0 0 8px">Program her açılışta açık olanlardan birini oynatır; hepsi bir kez oynamadan hiçbiri tekrar gelmez. Tek açıksa hep o oynar, hiçbiri açık değilse program doğrudan açılır. Değişiklik bir sonraki açılışta geçerli olur.</p>\n'
         '        <label class="sira mut"><input type="checkbox" id="acilisTam"> Açılış animasyonunu tamamen kapat</label>\n'
         '        <div class="sira" style="margin:8px 0"><button type="button" class="btn" id="acilisHepsi">Tümünü aç</button></div>\n'
         '        <div id="acilisListe"></div>\n'
         '      </div></details>\n')
h = h.replace(hedef, bolum + hedef, 1)
open(P, 'w', encoding='utf-8', newline='\n').write(h)
print('tamam')
