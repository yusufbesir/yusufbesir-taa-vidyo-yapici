/* TAA Vidyo Yapıcı: İKİZ KABARTMA logosu, açılış animasyonlarındaki TEK tanım yeri.
   Sahip: Yusuf. Yazan: [Claude#Sonnet-5.5]. Logo değişirse yalnız bu dosya (ve web/index.html içindeki
   <symbol id="ikiz-kabartma">, logo/logo.svg) güncellenir. Animasyonlarda [data-ikiz="sol"|"sag"] işaretli
   <g> elemanları buradan doldurulur. viewBox 0 0 128 128; çizim sırası: üst yüz, yan yüz, alt yan, ön yüz. */
(function () {
  var K = {
    sol: '<path fill="#C9566F" d="M16 16 H40 L46 10 H22 Z"/><path fill="#6E1229" d="M40 16 L56 64 L62 58 L46 10 Z"/><path fill="#560D20" d="M56 64 L40 112 L46 106 L62 58 Z"/><path fill="#9A1B3C" d="M16 16 H40 L56 64 L40 112 H16 L32 64 Z"/>',
    sag: '<path fill="#C9566F" d="M88 16 H112 L106 10 H82 Z"/><path fill="#6E1229" d="M88 16 L72 64 L66 58 L82 10 Z"/><path fill="#560D20" d="M72 64 L88 112 L82 106 L66 58 Z"/><path fill="#9A1B3C" d="M88 16 H112 L96 64 L112 112 H88 L72 64 Z"/>'
  };
  window.IKIZ_KABARTMA = K;
  var g = document.querySelectorAll('[data-ikiz]');
  for (var i = 0; i < g.length; i++) g[i].innerHTML = K[g[i].getAttribute('data-ikiz')] || '';
})();
