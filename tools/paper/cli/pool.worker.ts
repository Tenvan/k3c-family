// Worker des RasterPool (B-751): ein SVG rein, PNG und RGBA-Pixel raus. Bewusst ohne Projekt-Importe, damit Bun und Node
// (Vitest, Typen nur entfernt) die Datei ohne Build laden. Die resvg-Optionen gleichen `rastere` in render/raster.ts;
// weichen sie ab, weichen die Pixel ab (der Gleichheitstest in genParallel.test.ts fängt das).
import { parentPort } from 'node:worker_threads';
import { Resvg } from '@resvg/resvg-js';

parentPort?.on('message', (svg: string) => {
  try {
    const r = new Resvg(svg, { font: { loadSystemFonts: false } }).render();
    parentPort?.postMessage({ png: r.asPng(), rgba: r.pixels, breite: r.width, hoehe: r.height });
  } catch (e) {
    parentPort?.postMessage({ fehler: (e as Error).message });
  }
});
