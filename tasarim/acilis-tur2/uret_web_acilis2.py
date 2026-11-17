# Açıklama: Codex çıktılarından (sol.md, terra.md, luna.md) web/acilis/{zaman-sol,gece-terra,kalibrasyon-luna}.html üretir (İkiz Kabartma logosu + uygulama modu).
# Sahip: Yusuf. Yazan: [Claude#Sonnet-5.5, 2026-11-17]. Yalnız yazan yapay zekâ ya da Yusuf'un izniyle değişir; silme yok.
import re
T2 = "D:/Programlar/TAA Vidyo Yapıcı/tasarim/acilis-tur2"
OUT = "D:/Programlar/TAA Vidyo Yapıcı/web/acilis"
rd = lambda p: open(p, encoding='utf-8').read()
UYG = ('<style>html.uyg #tekrar,html.uyg #rep,html.uyg #replay,html.uyg .replay,html.uyg .tekrar{display:none!important}</style>\n'
       '<script>/* uygulama modu (?uygulama=1): Tekrar oynat gizli; tıklama/Esc/Geç ana sayfaya bildirilir */'
       'if(/[?&]uygulama=1/.test(location.search)){document.documentElement.classList.add("uyg");'
       'var g=function(){parent.postMessage("taa-acilis-gec","*")};addEventListener("click",g);'
       'addEventListener("keydown",function(e){if(e.key==="Escape")g()})}</script>\n')

def html_al(md):
    return re.search(r'```html\n(.*?)```', rd(T2 + '/' + md), re.S).group(1)

def ortak(h, anlat):
    h = h.replace('</head>', UYG + '</head>', 1)
    j = h.rindex('<script>')
    h = h[:j] + '<script src="logo.js"></script>\n' + h[j:]
    h = h.replace('<!doctype html>', "<!doctype html>\n<!-- Açıklama: TAA Vidyo Yapıcı açılış animasyonu (%s; Codex çıktısı, logo İkiz Kabartma ile değiştirildi). Logo: logo.js (tek yer). Sahip: Yusuf. Yazan: [Codex; düzenleyen Claude#Sonnet-5.5]. Yalnız yazan yapay zekâ ya da Yusuf'un izniyle değişir; silme yok. -->" % anlat, 1) if h.lstrip().lower().startswith('<!doctype html>') else h
    return h

def yaz(ad, h):
    open(OUT + '/' + ad + '.html', 'w', encoding='utf-8', newline='\n').write(h)

# sol: .left/.right sınıfları WAAPI ile oynatılıyor, grup üzerinde korunur
h = html_al('sol.md')
a = re.search(r'<path class="left".*?<path class="right"[^>]*/>', h, re.S)
assert a
h = h.replace(a.group(0), '<g class="left" data-ikiz="sol"></g><g class="right" data-ikiz="sag"></g>', 1)
yaz('zaman-sol', ortak(h, 'tur 2, zaman çizelgesi, gpt-6.1-sol'))

# terra: .pile.sol/.sag CSS animasyonlu
h = html_al('terra.md')
a = re.search(r'<path class="pile sol".*?<path class="pile sag"[^>]*/>', h, re.S)
assert a
h = h.replace(a.group(0), '<g class="pile sol" data-ikiz="sol"></g><g class="pile sag" data-ikiz="sag"></g>', 1)
yaz('gece-terra', ortak(h, 'tur 2, gece cephesi, gpt-6.1-terra'))

# luna
h = html_al('luna.md')
a = re.search(r'(<svg class="logo" viewBox="0 0 128 128"[^>]*>).*?(</svg>)', h, re.S)
assert a
h = h.replace(a.group(0), a.group(1) + '<g data-ikiz="sol"></g><g data-ikiz="sag"></g>' + a.group(2), 1)
yaz('kalibrasyon-luna', ortak(h, 'tur 2, kalibrasyon, gpt-6-luna'))
print('tamam')
