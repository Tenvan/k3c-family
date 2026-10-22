// Vorher/Nachher der Grafik-Pipeline (B-331/AC-03): Quelle aus public/grafik, Ergebnis aus public/grafik-db32 (task gen:pipeline).
import { installPageChrome } from '../core/shell';
import liste from '../../docs/assets/pipeline-liste.json';
import { installPadScroll } from './spriteReference';
import { applyTexts, t } from './texts';

installPageChrome();
applyTexts();
installPadScroll();

/** Bildschirm-Pixel je Ergebnis-Pixel (Stilbeschluss Q13: 16 px, ×2). */
const ZOOM = 2;

function image(src: string, label: string): HTMLElement {
  const figure = document.createElement('figure');
  const img = document.createElement('img');
  img.src = src;
  img.alt = label;
  const caption = document.createElement('figcaption');
  caption.textContent = label;
  figure.append(img, caption);
  return figure;
}

/** Beide Bilder in der Weltgröße des Ergebnisses: Ergebnis ×2, Quelle auf dieselbe Breite gestreckt. */
function pair(path: string): HTMLElement {
  const row = document.createElement('section');
  row.className = 'pair';
  const before = image(`grafik/${path}`, t('pipeline.before'));
  const after = image(`grafik-db32/${path}`, t('pipeline.after'));
  const [beforeImg, afterImg] = [before.querySelector('img')!, after.querySelector('img')!];
  afterImg.addEventListener('load', () => {
    beforeImg.style.width = afterImg.style.width = `${afterImg.naturalWidth * ZOOM}px`;
  });
  const title = document.createElement('h2');
  title.textContent = path;
  const ground = document.createElement('div');
  ground.className = 'ground';
  ground.append(before, after);
  row.append(title, ground);
  return row;
}

document.getElementById('pairs')?.append(...Object.keys(liste).sort().map(pair));
