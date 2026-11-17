Model: gpt-6.1-terra (Codex)

AD: Gece Cephesi
FIKIR: Karanlık cephede açılan iki dikey ekran, karşılıklı kumaş pilelerine dönüşerek programın logosunu oluşturur.

```html
<!DOCTYPE html>
<html lang="tr">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>TAA Vidyo Yapıcı — Gece Cephesi</title>
<style>
  :root {
    color-scheme: dark;
    font-family: "Segoe UI Variable", "Segoe UI", system-ui, sans-serif;
    background: #191826;
    color: #f1f0ec;
  }

  * { box-sizing: border-box; }

  body {
    margin: 0;
    min-height: 100vh;
    overflow: hidden;
  }

  .sahne {
    position: relative;
    display: grid;
    place-items: center;
    width: 100%;
    height: 100vh;
    overflow: hidden;
    isolation: isolate;
    background: #191826;
  }

  /* Cephe çizgileri ışık yerine mimari bir derinlik kurar. */
  .cephe {
    position: absolute;
    inset: 0;
    background:
      linear-gradient(90deg,
        transparent 0 9%, #242331 9% 9.12%, transparent 9.12% 90.88%,
        #242331 90.88% 91%, transparent 91%),
      linear-gradient(0deg,
        transparent 0 15%, #242331 15% 15.16%, transparent 15.16% 84.84%,
        #242331 84.84% 85%, transparent 85%);
    opacity: 0;
  }

  .ekranlar {
    position: absolute;
    inset: 0;
    display: flex;
    justify-content: center;
    align-items: center;
    gap: clamp(12px, 1.5vw, 30px);
    pointer-events: none;
  }

  .ekran {
    width: min(24vw, 42vh);
    aspect-ratio: 950 / 1800;
    max-height: 86vh;
    border: 1px solid #633044;
    background: #5a0c20;
    opacity: 0;
  }

  .ekran.sol { transform-origin: right center; }
  .ekran.sag { transform-origin: left center; }

  .ekran-ic {
    width: 100%;
    height: 100%;
    background: linear-gradient(160deg, #9a1b3c, #5a0c20 75%);
  }

  .imza {
    position: relative;
    display: flex;
    flex-direction: column;
    align-items: center;
    width: min(90vw, 900px);
    text-align: center;
  }

  .logo {
    display: block;
    width: clamp(180px, 23vh, 250px);
    height: auto;
    overflow: visible;
  }

  .pile {
    fill: #d2486a;
    opacity: 0;
  }

  .metin {
    margin-top: clamp(16px, 2.5vh, 28px);
    opacity: 0;
  }

  h1 {
    margin: 0;
    font-size: clamp(32px, 4.5vh, 52px);
    font-weight: 650;
    letter-spacing: -0.035em;
    line-height: 1.1;
  }

  p {
    margin: 14px 0 0;
    color: #c5c1c5;
    font-size: clamp(15px, 1.9vh, 20px);
    font-weight: 400;
    line-height: 1.4;
  }

  button {
    position: absolute;
    z-index: 2;
    padding: 11px 16px;
    border: 1px solid #77717c;
    border-radius: 8px;
    background: #191826;
    color: #f1f0ec;
    font: inherit;
    font-size: 14px;
    cursor: pointer;
  }

  button:hover { background: #302b38; }
  button:focus-visible { outline: 2px solid #d2486a; outline-offset: 3px; }
  .gec { top: 28px; right: 32px; }
  .tekrar { right: 32px; bottom: 28px; }

  .oynuyor .cephe {
    animation: cephe 3.6s ease both;
  }

  .oynuyor .ekran.sol {
    animation: sol-ekran 3.6s cubic-bezier(.65, 0, .2, 1) both;
  }

  .oynuyor .ekran.sag {
    animation: sag-ekran 3.6s cubic-bezier(.65, 0, .2, 1) both;
  }

  .oynuyor .pile.sol {
    animation: sol-pile 3.6s cubic-bezier(.22, 1, .36, 1) both;
  }

  .oynuyor .pile.sag {
    animation: sag-pile 3.6s cubic-bezier(.22, 1, .36, 1) both;
  }

  .oynuyor .metin {
    animation: metin 3.6s ease both;
  }

  @keyframes cephe {
    0% { opacity: 0; transform: scale(1.08); }
    16%, 38% { opacity: 1; }
    68%, 100% { opacity: 0; transform: scale(1); }
  }

  @keyframes sol-ekran {
    0% { opacity: 0; transform: translateX(-60vw) scaleY(.9); }
    20%, 35% { opacity: 1; transform: translateX(0) scaleY(1); }
    65%, 100% { opacity: 0; transform: translateX(18vw) scale(.32); }
  }

  @keyframes sag-ekran {
    0% { opacity: 0; transform: translateX(60vw) scaleY(.9); }
    20%, 35% { opacity: 1; transform: translateX(0) scaleY(1); }
    65%, 100% { opacity: 0; transform: translateX(-18vw) scale(.32); }
  }

  @keyframes sol-pile {
    0%, 37% { opacity: 0; transform: translateX(-24px); }
    66%, 100% { opacity: 1; transform: translateX(0); }
  }

  @keyframes sag-pile {
    0%, 37% { opacity: 0; transform: translateX(24px); }
    66%, 100% { opacity: 1; transform: translateX(0); }
  }

  @keyframes metin {
    0%, 58% { opacity: 0; transform: translateY(12px); }
    83%, 100% { opacity: 1; transform: translateY(0); }
  }

  .son .pile,
  .son .metin {
    opacity: 1;
    transform: none;
  }

  @media (prefers-reduced-motion: reduce) {
    .oynuyor .cephe,
    .oynuyor .ekran { animation: none; }

    .oynuyor .pile,
    .oynuyor .metin {
      animation: sade 120ms ease-out both;
    }

    @keyframes sade {
      from { opacity: 0; }
      to { opacity: 1; }
    }
  }
</style>
</head>
<body>
<main class="sahne oynuyor" id="sahne" aria-label="TAA Vidyo Yapıcı açılış ekranı">
  <div class="cephe" aria-hidden="true"></div>
  <div class="ekranlar" aria-hidden="true">
    <div class="ekran sol"><div class="ekran-ic"></div></div>
    <div class="ekran sag"><div class="ekran-ic"></div></div>
  </div>

  <div class="imza">
    <svg class="logo" viewBox="0 0 128 128" role="img" aria-label="İkiz Pile logosu">
      <path class="pile sol" d="M16 16 H40 L56 64 L40 112 H16 L32 64 Z"/>
      <path class="pile sag" d="M112 16 H88 L72 64 L88 112 H112 L96 64 Z"/>
    </svg>
    <div class="metin">
      <h1>TAA Vidyo Yapıcı</h1>
      <p>Yaz, önizle, tek tıkla videoyu yap.</p>
    </div>
  </div>

  <button class="gec" type="button" id="gec">Geç</button>
  <button class="tekrar" type="button" id="tekrar">Tekrar oynat</button>
</main>

<script>
  const sahne = document.getElementById("sahne");

  document.getElementById("gec").addEventListener("click", () => {
    sahne.classList.remove("oynuyor");
    sahne.classList.add("son");
  });

  document.getElementById("tekrar").addEventListener("click", () => {
    sahne.classList.remove("oynuyor", "son");
    void sahne.offsetWidth;
    sahne.classList.add("oynuyor");
  });
</script>
</body>
</html>
```