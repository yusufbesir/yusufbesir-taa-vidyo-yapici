# Açıklama: web/acilis/*.html (14 açılış) dosyalarını tek, kendi kendine yeten galeri sayfasına gömer (srcdoc, logo.js satır içi).
# Çıktılar: galeri-0.9.6.html (tam belge) ve galeri-0.9.6.artifact.html (Artifact için gövde parçası). Sahip: Yusuf. Yazan: [Claude#Sonnet-5.5, 2026-11-17].
import json
W = "D:/Programlar/TAA Vidyo Yapıcı/web/acilis"
OUT = "D:/Programlar/TAA Vidyo Yapıcı/tasarim/acilis-tur2"
BS = chr(92)
L = [("zaman", "Zaman çizelgesi", "Claude Sonnet"), ("gece", "Gece cephesi", "Claude Sonnet"), ("kalibrasyon", "Kalibrasyon", "Claude Sonnet"),
     ("dikis", "Dikiş", "Claude Sonnet"), ("katlama", "Katlama", "Claude Sonnet"),
     ("led", "LED uyanış", "Claude Opus"), ("perde", "Kumaş perde", "Claude Opus"), ("kurgu", "Kurgu kesmesi", "Claude Opus"),
     ("iki-ekran", "İki ekran", "Claude Opus"), ("isik", "Sahne ışığı", "Claude Opus"),
     ("odak", "Odak açılışı", "agy Sonnet 4.6"), ("zaman-sol", "Zaman çizelgesi", "GPT Sol"),
     ("gece-terra", "Gece cephesi", "GPT Terra"), ("kalibrasyon-luna", "Kalibrasyon", "GPT Luna")]
logo = open(W + "/logo.js", encoding="utf-8").read()
veri = []
for ad, baslik, model in L:
    h = open(W + "/%s.html" % ad, encoding="utf-8").read()
    assert '<script src="logo.js"></script>' in h, ad
    h = h.replace('<script src="logo.js"></script>', "<script>" + logo + "</script>", 1)
    h = h.replace("</head>", "<style>button{display:none!important}</style></head>", 1)  # kartın kendi Tekrar oynat düğmesi var
    veri.append({"ad": ad, "baslik": baslik, "model": model, "html": h})
js = json.dumps(veri, ensure_ascii=False).replace("</", "<" + BS + "/")

STIL = """<style>
:root{color-scheme:dark;--bg:#14131C;--panel:#1D1C29;--cer:#34324A;--tx:#F1F0EC;--mut:#A9A5C2;--ac:#D2486A;--fh:"Segoe UI Variable","Segoe UI",system-ui,sans-serif}
*{box-sizing:border-box}
body{margin:0;background:var(--bg);color:var(--tx);font:16px/1.5 var(--fh);padding-block:24px 40px;padding-left:max(16px,env(safe-area-inset-left,0px));padding-right:max(16px,env(safe-area-inset-right,0px))}
header{display:flex;flex-wrap:wrap;gap:12px 24px;align-items:center;justify-content:space-between;max-width:1400px;margin:0 auto 20px}
h1{margin:0;font-size:clamp(22px,4vw,30px);line-height:1.2;text-wrap:balance}
header p{margin:4px 0 0;color:var(--mut);font-size:14px;max-width:60ch}
button{min-height:44px;padding:0 18px;border-radius:999px;border:1px solid var(--cer);background:var(--panel);color:var(--tx);font:inherit;font-size:15px;cursor:pointer}
button:hover{border-color:var(--ac)}button:focus-visible{outline:3px solid var(--ac);outline-offset:2px}
main{display:grid;grid-template-columns:repeat(auto-fill,minmax(min(100%,400px),1fr));gap:16px;max-width:1400px;margin:0 auto}
.k{background:var(--panel);border:1px solid var(--cer);border-radius:10px;overflow:hidden;min-width:0}
.k iframe{display:block;width:100%;max-width:100%;aspect-ratio:16/9;border:0;background:#0b0b10}
.alt{display:flex;align-items:center;justify-content:space-between;gap:10px;padding:10px 14px}
.alt b{display:block;font-size:15px}.alt small{color:var(--mut);font-size:13px}
.alt span{min-width:0}
</style>"""
GOVDE = """<header><div><h1>TAA Vidyo Açılışları</h1><p>14 açılış animasyonu, hepsinde İkiz Kabartma logosu. Programda her açılışta rastgele biri oynar. Kart görünür olunca başlar.</p></div><button id="hepsi" type="button">Hepsini yeniden oynat</button></header>
<main id="ana"></main>
<script>
var V=__VERI__;
var ana=document.getElementById('ana'),kartlar=[];
function oynat(k){k.f.srcdoc='';requestAnimationFrame(function(){k.f.srcdoc=k.d.html})}
V.forEach(function(d,i){var s=document.createElement('section');s.className='k';
 s.innerHTML='<iframe sandbox="allow-scripts" title="'+d.baslik+'"></iframe><div class="alt"><span><b>'+(i+1)+'. '+d.baslik+'</b><small>'+d.model+'</small></span><button type="button">Tekrar oynat</button></div>';
 var k={d:d,f:s.querySelector('iframe'),s:s,g:false};kartlar.push(k);
 s.querySelector('button').onclick=function(){oynat(k)};ana.appendChild(s)});
if('IntersectionObserver' in window){var io=new IntersectionObserver(function(es){es.forEach(function(e){var k=kartlar.filter(function(x){return x.s===e.target})[0];
 if(e.isIntersecting&&!k.g){k.g=true;oynat(k)}else if(!e.isIntersecting&&k.g){k.g=false;k.f.srcdoc=''}})},{threshold:.5});kartlar.forEach(function(k){io.observe(k.s)})}
else kartlar.forEach(oynat);
document.getElementById('hepsi').onclick=function(){kartlar.forEach(function(k){if(k.g||!('IntersectionObserver' in window))oynat(k)})};
</script>"""
govde = GOVDE.replace("__VERI__", js)
parca = '<title>TAA Vidyo Açılışları</title>\n<meta name="claude:viewport-fit-sides" content="cover">\n' + STIL + "\n" + govde + "\n"
tam = '<!doctype html>\n<!-- Açıklama: TAA Vidyo Yapıcı açılış animasyonları galerisi 0.9.6 (14 adet, tek dosya, dış bağlantı yok). Sahip: Yusuf. Yazan: [Claude#Sonnet-5.5, 2026-11-17]. Yalnız yazan yapay zekâ ya da Yusuf\'un izniyle değişir; silme yok. -->\n<html lang="tr"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1,viewport-fit=cover">\n' + parca.replace('<meta name="claude:viewport-fit-sides" content="cover">\n', '') + "</head><body>\n" + govde + "\n</body></html>\n"
tam = '<!doctype html>\n<!-- Açıklama: TAA Vidyo Yapıcı açılış animasyonları galerisi 0.9.6 (14 adet, tek dosya, dış bağlantı yok). Sahip: Yusuf. Yazan: [Claude#Sonnet-5.5, 2026-11-17]. Yalnız yazan yapay zekâ ya da Yusuf\'un izniyle değişir; silme yok. -->\n<html lang="tr"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1,viewport-fit=cover">\n<title>TAA Vidyo Açılışları</title>\n' + STIL + "\n</head><body>\n" + govde + "\n</body></html>\n"
open(OUT + "/galeri-0.9.6.html", "w", encoding="utf-8", newline="\n").write(tam)
open(OUT + "/galeri-0.9.6.artifact.html", "w", encoding="utf-8", newline="\n").write(parca)
print(len(tam), len(parca))
