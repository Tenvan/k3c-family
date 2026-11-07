// Fixture-Spec mauer (Gebäude, zwei Stufen, Material stein) im Stil von fixtures/svg/mauer-1.svg und mauer-2.svg:
// Quader als Papierstücke mit Schatten-Lage unten und Licht-Falz oben; Stufe 2 setzt eine Reihe und Zinnen auf.
import { num } from '../schema/hash';
import type { Lage, Spec } from '../schema/types';

const REIHE = 18;

/** Feste kleine Unregelmäßigkeit (kein Zufall), wie im SVG-Generator von PV1.6. */
const jit = (i: number, j: number, k: number) => (((i * 7 + j * 13 + k * 5) % 7) - 3) * 0.25;
const pfad = (pts: [number, number][]) => 'M' + pts.map(([x, y]) => `${num(x)} ${num(y)}`).join('L') + 'Z';

function quader(x0: number, y0: number, x1: number, y1: number, i: number, j: number): Lage[] {
  const a: [number, number] = [x0 + jit(i, j, 1), y0 + jit(i, j, 2)], b: [number, number] = [x1 + jit(i, j, 3), y0 + jit(i, j, 4)];
  const c: [number, number] = [x1 + jit(i, j, 5), y1], d: [number, number] = [x0 + jit(i, j, 6), y1];
  return [
    { art: 'flaeche', form: pfad([a, b, c, d]), material: 'stein' },
    { art: 'flaeche', form: pfad([[d[0], d[1] - 3], [c[0], c[1] - 3], c, d]), material: 'stein', ton: 'schatten', auflage: true },
    { art: 'falz', form: `M${num(a[0] + 1.5)} ${num(a[1] + 1.6)}L${num(b[0] - 1.5)} ${num(b[1] + 1.6)}`, breite: 1.2, material: 'stein', ton: 'licht' },
  ];
}

/** Eine Reihe Quader im Läuferverband; ungerade Reihen um einen halben Stein versetzt. */
function reihe(j: number, x0: number, x1: number): Lage[] {
  const y1 = -j * REIHE, y0 = y1 - REIHE + 1, off = j % 2 === 0 ? 0 : 12;
  const xs = [x0, ...Array.from({ length: 6 }, (_, k) => x0 + 24 * (k + 1) - off).filter((x) => x0 + 6 < x && x < x1 - 6), x1];
  return xs.slice(0, -1).flatMap((x, i) => quader(x + 0.5, y0, xs[i + 1] - 0.5, y1 - 0.5, i, j));
}

const platte = (x0: number, x1: number, y0: number, y1: number): Lage =>
  ({ art: 'flaeche', form: pfad([[x0, y1], [x0, y0], [x1, y0], [x1, y1]]), material: 'stein', ton: 'schatten' });

function zinnen(x0: number, x1: number, top: number): Lage[] {
  const w = (x1 - x0) / 7;
  const steine = [0, 2, 4, 6].flatMap((i) => quader(x0 + i * w + 0.5, top - 16, x0 + (i + 1) * w - 0.5, top - 0.5, i, 9));
  return [...steine, { art: 'flaeche', form: pfad([[x0 - 2, top + 1], [x1 + 2, top + 1], [x1 + 2, top - 2], [x0 - 2, top - 2]]), material: 'stein', ton: 'licht', auflage: true }];
}

export function mauerSpec(): Spec {
  return {
    id: 'mauer', rig: 'gebaeude', version: 1,
    beschreibung: 'Steinmauer aus Quadern im Läuferverband; Stufe 2 eine Reihe höher mit Zinnen.',
    teile: [{ id: 'korpus', pivot: [0, 0], lagen: [platte(-61, 61, -73, 0), ...[0, 1, 2, 3].flatMap((j) => reihe(j, -60, 60))] }],
    stufen: [{ zusaetze: [
      { id: 'aufsatz', eltern: 'korpus', pivot: [0, -72], lagen: [platte(-61, 61, -91, -72), ...reihe(4, -60, 60)] },
      { id: 'zinnen', eltern: 'aufsatz', pivot: [0, -90], lagen: zinnen(-60, 60, -90) },
    ] }],
    animationen: { stand: { frames: 1 } },
    ausgabe: { hoehe_px: 128, rahmen: [-66, -112, 132, 116] },
  };
}
