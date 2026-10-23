// Reine Rechnung für die Diagramme der Seite Monitoring: Achsenmaß, Linienpfade, Zeitmarken, Treffer unter der Maus.
// Alles ohne DOM, damit es sich testen lässt.

export const W = 1000;
export const H = 100;

export interface Line {
  label: string;
  values: number[];
  /** CSS-Klasse der Linie (`mon-c0` … `mon-c3`). */
  cls: string;
}

/** Rundet auf 1, 2, 5 oder 10 mal Zehnerpotenz auf: ein ruhiger Achsenwert (0 → 1). */
export function niceMax(v: number): number {
  if (!(v > 0)) return 1;
  const p = 10 ** Math.floor(Math.log10(v));
  const f = v / p;
  return (f <= 1 ? 1 : f <= 2 ? 2 : f <= 5 ? 5 : 10) * p;
}

/** Größter Wert aller Linien. */
export function peakOf(lines: Line[]): number {
  let m = 0;
  for (const l of lines) for (const v of l.values) if (v > m) m = v;
  return m;
}

/** SVG-Pfad einer Linie in einer Fläche von W mal H (y nach unten); weniger als zwei Punkte ergeben einen Strich. */
export function linePath(values: number[], max: number): string {
  if (values.length === 0) return '';
  const n = values.length;
  return values
    .map((v, i) => {
      const x = n > 1 ? (i * W) / (n - 1) : 0;
      const y = H - (H * 0.94 * Math.min(Math.max(v, 0), max)) / max;
      return `${i === 0 ? 'M' : 'L'} ${x.toFixed(1)} ${y.toFixed(1)}`;
    })
    .join(' ');
}

/** Pfad einer Fläche unter der Linie. */
export function areaPath(values: number[], max: number): string {
  const d = linePath(values, max);
  return d ? `${d} L ${W} ${H} L 0 ${H} Z` : '';
}

export interface Tick {
  /** Lage 0 … 1 von links. */
  at: number;
  label: string;
}

/** Gleichmäßige Zeitmarken über die Zeiten (Unix-ms); Uhrzeit, bei mehr als 20 h Spanne mit Tag. */
export function timeTicks(times: number[], count = 5): Tick[] {
  if (times.length < 2) return [];
  const first = times[0];
  const span = times[times.length - 1] - first;
  const long = span > 20 * 3_600_000;
  return Array.from({ length: count }, (_, i) => {
    const at = i / (count - 1);
    const d = new Date(first + span * at);
    const hm = d.toLocaleTimeString('de-DE', { hour: '2-digit', minute: '2-digit' });
    return { at, label: long ? `${d.toLocaleDateString('de-DE', { day: '2-digit', month: '2-digit' })} ${hm}` : hm };
  });
}

/** Index des Punkts unter der Maus: frac 0 … 1 von links, n Punkte. */
export function nearestIndex(frac: number, n: number): number {
  if (n <= 1) return 0;
  return Math.min(n - 1, Math.max(0, Math.round(frac * (n - 1))));
}

/** Zuwachs je Punkt aus einem laufenden Zähler (GC-Läufe); der erste Punkt hat keinen Vorgänger und zählt 0. */
export function deltas(values: number[]): number[] {
  return values.map((v, i) => (i === 0 ? 0 : Math.max(0, v - values[i - 1])));
}
