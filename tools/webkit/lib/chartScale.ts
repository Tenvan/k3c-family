// Reine Skalen-Funktionen für die SVG-Diagramme (molecules/LineChart, BarChart, CorridorBar): Wertebereich mit Rand,
// "schöne" Ticks, Abbildung Wert → Pixel. Keine Abhängigkeit, kein DOM.

export interface Extent {
  min: number;
  max: number;
}

/** Wertebereich über alle endlichen Werte; `pad` ist der Randanteil (0.05 = 5 %). Leer → 0..1; flach → ±1 um den Wert. */
export function extent(values: readonly number[], pad = 0.05): Extent {
  const v = values.filter(Number.isFinite);
  if (v.length === 0) return { min: 0, max: 1 };
  let min = Math.min(...v);
  let max = Math.max(...v);
  if (min === max) return { min: min - 1, max: max + 1 };
  const d = (max - min) * pad;
  min -= d;
  max += d;
  return { min, max };
}

/** Rundet einen Schritt auf 1, 2, 5 oder 10 mal Zehnerpotenz. */
function niceStep(raw: number): number {
  const pow = 10 ** Math.floor(Math.log10(raw));
  const f = raw / pow;
  const m = f <= 1 ? 1 : f <= 2 ? 2 : f <= 5 ? 5 : 10;
  return m * pow;
}

/** Ticks mit rundem Schritt, die `min..max` überdecken (ungefähr `count` Stück). */
export function niceTicks(min: number, max: number, count = 5): number[] {
  if (!(max > min) || count < 1) return [min];
  const step = niceStep((max - min) / count);
  const first = Math.ceil(min / step - 1e-9) * step;
  const out: number[] = [];
  for (let t = first; t <= max + step * 1e-9; t += step) out.push(Number((Math.round(t / step) * step).toPrecision(12)) + 0);
  return out;
}

/** Erweitert den Bereich auf die äußersten Ticks, damit die Achse rund endet. */
export function niceExtent(e: Extent, count = 5): Extent {
  if (!(e.max > e.min)) return e;
  const step = niceStep((e.max - e.min) / count);
  return { min: Math.floor(e.min / step + 1e-9) * step, max: Math.ceil(e.max / step - 1e-9) * step };
}

/** Lineare Abbildung `domain` → `range`; ein flacher Bereich landet in der Mitte. */
export function scaleLinear(domain: Extent, range: readonly [number, number]): (v: number) => number {
  const span = domain.max - domain.min;
  if (!(span > 0)) return () => (range[0] + range[1]) / 2;
  return (v) => range[0] + ((v - domain.min) / span) * (range[1] - range[0]);
}

/** Kurze Zahl für Achsen: ganze Zahlen ohne Nachkommastellen, sonst höchstens 3 gültige Ziffern. */
export function formatTick(v: number): string {
  if (!Number.isFinite(v)) return '';
  if (Number.isInteger(v)) return String(v);
  return String(Number(v.toPrecision(3)));
}

/** SVG-Pfad `M x y L x y …` aus Pixelpunkten; leer bei keinem Punkt. */
export function linePath(pts: readonly { x: number; y: number }[]): string {
  return pts.map((p, i) => `${i === 0 ? 'M' : 'L'}${p.x.toFixed(1)} ${p.y.toFixed(1)}`).join(' ');
}
