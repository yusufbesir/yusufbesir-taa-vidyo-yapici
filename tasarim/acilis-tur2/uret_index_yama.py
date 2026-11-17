# Açıklama: web/index.html'e açılış katmanı + İkiz Kabartma symbol'ü ekler (0.9.5). Tek seferlik yama; sahip Yusuf, yazan [Claude#Sonnet-5.5].
import re
P = "D:/Programlar/TAA Vidyo Yapıcı/web/index.html"
h = open(P, encoding='utf-8').read()

# 1) eski LED yükleme ekranı CSS'i: açılış katmanı kuralları ile değiştir
bas = h.index('/* açılış ekranı: LED şerit')
son = h.index('/* bina görünümleri (karşıdan / üstten) */')
yeni_css = ('/* açılış katmanı (0.9.5): web/acilis/*.html içinden her açılışta rastgele biri, tam pencere iframe; en geç 4,5 sn */\n'
            '#yukleEkrani{position:fixed;inset:0;z-index:9999;background:var(--bg);transition:opacity .2s}\n'
            'body:has(#yukleEkrani){overflow:hidden}\n'
            '#yukleEkrani.kapan{opacity:0;pointer-events:none}\n'
            '#yukleEkrani iframe{display:block;width:100%;height:100%;border:0}\n'
            '/* logo: İkiz Kabartma, renkler --ac\'den türer (ön=--ac, üst=beyazla %35, yan=siyahla %30, alt yan=siyahla %45) */\n'
            '.marka{--ik-on:var(--ac);--ik-ust:color-mix(in srgb,var(--ac) 65%,#fff);--ik-yan:color-mix(in srgb,var(--ac) 70%,#000);--ik-alt:color-mix(in srgb,var(--ac) 55%,#000)}\n')
h = h[:bas] + yeni_css + h[son:]
# ikinci (sade tema) LED kuralları
for kural in ['#yukleEkrani b{font:600 24px/1 var(--fh);letter-spacing:0;text-transform:none}\n',
              '#yukleEkrani small{font:14px var(--fh);letter-spacing:0}\n',
              '#yukleEkrani .ledler i{box-shadow:none}\n',
              '@keyframes acilisLed{0%,100%{background:var(--line)}50%{background:var(--ac)}}\n']:
    assert kural in h, kural
    h = h.replace(kural, '', 1)

# 2) başlık logosu: symbol + <use>
h = h.replace('header img{width:28px;height:28px}', 'header img,header .marka{width:28px;height:28px}', 1)
h = h.replace('header img{width:32px;height:32px}', 'header img,header .marka{width:32px;height:32px}', 1)
assert '<img src="/logo.svg" alt="">' in h
h = h.replace('<img src="/logo.svg" alt="">', '<svg class="marka" viewBox="0 0 128 128" aria-hidden="true"><use href="#ikiz-kabartma"/></svg>', 1)

# 3) body başı: symbol (tek tanım) + açılış katmanı
b0 = h.index('<div id="yukleEkrani"')
b1 = h.index('<header>')
sym = ('<svg width="0" height="0" style="position:absolute" aria-hidden="true"><defs><symbol id="ikiz-kabartma" viewBox="0 0 128 128">'
       '<path style="fill:var(--ik-ust,#C9566F)" d="M16 16 H40 L46 10 H22 Z"/><path style="fill:var(--ik-yan,#6E1229)" d="M40 16 L56 64 L62 58 L46 10 Z"/>'
       '<path style="fill:var(--ik-alt,#560D20)" d="M56 64 L40 112 L46 106 L62 58 Z"/><path style="fill:var(--ik-on,#9A1B3C)" d="M16 16 H40 L56 64 L40 112 H16 L32 64 Z"/>'
       '<path style="fill:var(--ik-ust,#C9566F)" d="M88 16 H112 L106 10 H82 Z"/><path style="fill:var(--ik-yan,#6E1229)" d="M88 16 L72 64 L66 58 L82 10 Z"/>'
       '<path style="fill:var(--ik-alt,#560D20)" d="M72 64 L88 112 L82 106 L66 58 Z"/><path style="fill:var(--ik-on,#9A1B3C)" d="M88 16 H112 L96 64 L112 112 H88 L72 64 Z"/>'
       '</symbol></defs></svg>\n')
kat = '<div id="yukleEkrani" role="status" aria-label="Açılış animasyonu"><iframe id="acilisCerceve" title="Açılış animasyonu" tabindex="-1"></iframe></div>\n'
h = h[:b0] + sym + kat + h[b1:]

# 4) betik: acilisKapat + rastgele seçim
eski = "window.acilisKapat = () => { const a = document.getElementById('yukleEkrani'); if (!a) return; a.classList.add('kapan'); setTimeout(() => a.remove(), 300); };\nsetTimeout(() => window.acilisKapat(), 6000);\n"
assert eski in h
yeni = r"""// Açılış: web/acilis/ içinden rastgele biri (son seçilen tekrar gelmez), en geç 4,5 sn; Esc/tıklama/Geç anında geçer.
const ACILISLAR = ['zaman', 'gece', 'kalibrasyon', 'dikis', 'katlama', 'led', 'perde', 'kurgu', 'iki-ekran', 'isik', 'odak'];
(() => {
  const a = document.getElementById('yukleEkrani'), f = document.getElementById('acilisCerceve');
  if (!a || !f) return;
  let bitti = false, yuklendi = false, kapali = false;
  const kapat = () => { if (kapali) return; kapali = true; a.classList.add('kapan'); setTimeout(() => a.remove(), 220); window.removeEventListener('keydown', tus, true); };
  const dene = () => { if (bitti && yuklendi) kapat(); };
  const gec = () => { bitti = true; yuklendi = true; kapat(); };
  const tus = e => { if (e.key === 'Escape') gec(); };
  window.acilisKapat = () => { yuklendi = true; dene(); };
  window.addEventListener('message', e => { if (e.source === f.contentWindow && e.data === 'taa-acilis-gec') gec(); });
  window.addEventListener('keydown', tus, true);
  a.addEventListener('click', gec);
  let son = ''; try { son = localStorage.getItem('taa-acilis-son') || ''; } catch (e) { /* kalıcılık yok */ }
  const adaylar = ACILISLAR.filter(x => x !== son);
  const secim = adaylar[Math.floor(Math.random() * adaylar.length)];
  try { localStorage.setItem('taa-acilis-son', secim); } catch (e) { /* kalıcılık yok */ }
  const azHareket = window.matchMedia && matchMedia('(prefers-reduced-motion: reduce)').matches;
  f.src = '/acilis/' + secim + '.html?uygulama=1';
  setTimeout(() => { bitti = true; dene(); }, azHareket ? 400 : 4300); // animasyon ~3,5-4 sn; kapanış 0,2 sn solma ile toplam <= 4,5 sn
  setTimeout(gec, 6000); // yükleme ne olursa olsun katman 6 sn'de kalkar
})();
"""
h = h.replace(eski, yeni, 1)
open(P, 'w', encoding='utf-8', newline='\n').write(h)
print('tamam', len(h))
