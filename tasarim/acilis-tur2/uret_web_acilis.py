# Açıklama: tasarim/acilis-tur2 ve Opus animasyonlarından web/acilis/*.html üretir (İkiz Kabartma logosu + uygulama modu).
# Sahip: Yusuf. Yazan: [Claude#Sonnet-5.5, 2026-11-17]. Yalnız yazan yapay zekâ ya da Yusuf'un izniyle değişir; silme yok.
import re, sys
SP = sys.argv[1]           # Opus artifact dosyalarının klasörü
T2 = "D:/Programlar/TAA Vidyo Yapıcı/tasarim/acilis-tur2"
OUT = "D:/Programlar/TAA Vidyo Yapıcı/web/acilis"
rd = lambda p: open(p, encoding='utf-8').read()
GRUP = '<g id="ls" data-ikiz="sol"></g><g id="lg" data-ikiz="sag"></g>'
LK = '<svg id="lk" viewBox="0 0 128 128" aria-hidden="true"><g data-ikiz="sol"></g><g data-ikiz="sag"></g></svg>'
UYG = ('<style>html.uyg #tekrar,html.uyg #rep{display:none!important}</style>\n'
       '<script>/* uygulama modu (?uygulama=1): Tekrar oynat gizli; tıklama/Esc/Geç ana sayfaya bildirilir */'
       'if(/[?&]uygulama=1/.test(location.search)){document.documentElement.classList.add("uyg");'
       'var g=function(){parent.postMessage("taa-acilis-gec","*")};addEventListener("click",g);'
       'addEventListener("keydown",function(e){if(e.key==="Escape")g()})}</script>\n')

def ortak(h, anlat):
    h = h.replace('</head>', UYG + '</head>', 1)
    j = h.rindex('<script>')
    h = h[:j] + '<script src="logo.js"></script>\n' + h[j:]
    if '<!-- Açıklama' not in h:
        h = h.replace('<!doctype html>', "<!doctype html>\n<!-- Açıklama: TAA Vidyo Yapıcı açılış animasyonu (%s). Logo: logo.js (İkiz Kabartma, tek yer). Sahip: Yusuf. Yazan: [Claude#Sonnet-5.5]. Yalnız yazan yapay zekâ ya da Yusuf'un izniyle değişir; silme yok. -->" % anlat, 1)
    return h

def yaz(ad, h):
    open(OUT + '/' + ad + '.html', 'w', encoding='utf-8', newline='\n').write(h)

LGSTIL = '#lg{left:544px;top:98px;width:512px;height:512px;background:var(--v)}'
LKSTIL = '#lk{left:544px;top:98px;width:512px;height:512px;overflow:visible}'
for ad, sat in [('zaman', '.3,M+MD'), ('kalibrasyon', '.3,M+MD'), ('katlama', '.25,M+MD')]:
    h = rd(T2 + '/anim-%s.html' % ad)
    assert '<div id="lg"></div>' in h and LGSTIL in h
    h = h.replace('<div id="lg"></div>', '<div id="lg"></div>\n ' + LK, 1)
    h = h.replace(LGSTIL, LGSTIL + '\n' + LKSTIL, 1)
    a = "A('#lg',[{opacity:0},{opacity:1}],%s);" % sat
    assert a in h, (ad, a)
    h = h.replace(a, a + "\n A('#lk',[{opacity:0},{opacity:1}],%s);" % sat, 1)
    yaz(ad, ortak(h, 'tur 2, ' + ad))

h = rd(T2 + '/anim-gece.html')
assert '</div>\n <div id="tt">' in h and LGSTIL in h
h = h.replace('</div>\n <div id="tt">', '</div>\n ' + LK + '\n <div id="tt">', 1)
h = h.replace(LGSTIL, LKSTIL, 1)
a = "A('#tt',[{opacity:0,transform:'translateY(18px)'}"
assert a in h
h = h.replace(a, "A('#lk',[{opacity:0},{opacity:1}],.3,2.75);\n  " + a, 1)
yaz('gece', ortak(h, 'tur 2, gece'))

h = rd(T2 + '/anim-dikis.html')
s1 = '#lg svg{width:100%;height:100%;overflow:visible}'
assert s1 in h and ' </svg></div>\n <div id="tt">' in h
h = h.replace(s1, s1 + '\n#lg #lk{position:absolute;left:0;top:0}', 1)
h = h.replace(' </svg></div>\n <div id="tt">', ' </svg>' + LK + '</div>\n <div id="tt">', 1)
a = " A('#inn',[{opacity:0},{opacity:.6}],.5,2.1);"
assert a in h
h = h.replace(a, a + "\n A('#lk',[{opacity:0},{opacity:1}],.4,2.3);", 1)
assert 'id="lk"' in h
yaz('dikis', ortak(h, 'tur 2, dikiş'))

for src, ad in [('opus-led', 'led'), ('opus-perde', 'perde'), ('opus-kurgu', 'kurgu'), ('opus-iki-ekran', 'iki-ekran'), ('opus-isik', 'isik')]:
    h = rd(SP + '/%s.html' % src)
    h2 = re.sub(r'(<div id="logo"><svg viewBox="0 0 128 128">).*?(</svg></div>)', lambda m: m.group(1) + GRUP + m.group(2), h, count=1, flags=re.S)
    assert h2 != h and 'data-ikiz="sag"' in h2, ad
    yaz(ad, ortak(h2, 'tur 2, Opus ' + ad))

m = rd(T2 + '/agy-sonnet46.md')
h = re.search(r'```html\n(.*?)```', m, re.S).group(1)
h = re.sub(r'<svg id="pile-svg".*?</svg>', '<svg id="pile-svg" viewBox="0 0 128 128" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="İkiz Kabartma logosu"><g data-ikiz="sol"></g><g data-ikiz="sag"></g></svg>', h, count=1, flags=re.S)
assert 'data-ikiz' in h
h = h.replace('width:clamp(96px,11vw,152px);', 'width:clamp(200px,24vw,380px);', 1)
h = h.replace('#tekrar{bottom:18px;right:18px;opacity:0;}', '#tekrar{bottom:18px;right:18px;opacity:0;}\nhtml.uyg #tekrar{display:none!important}', 1)
yaz('odak', ortak(h, 'tur 2, odak'))
print('tamam')
