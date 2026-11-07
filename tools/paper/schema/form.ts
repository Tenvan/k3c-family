// Begrenzungsrahmen eines SVG-Pfads (Attribut d) aus End- und Kontrollpunkten: konservativ (Kontrollpunkte zählen mit),
// reicht für die Proportionsregel in check.ts. Unterstützt M L H V C S Q T Z, absolut und relativ (keine Bögen).

export type Box = { x0: number; y0: number; x1: number; y1: number };

const ARGS: Record<string, number> = { M: 2, L: 2, H: 1, V: 1, C: 6, S: 4, Q: 4, T: 2, Z: 0 };

/** Zerlegt einen Pfad in Befehle mit Zahlen; Wiederholungen ohne Buchstaben gelten als derselbe Befehl (M → L). */
export function commands(d: string): { cmd: string; args: number[] }[] {
  const out: { cmd: string; args: number[] }[] = [];
  for (const m of d.matchAll(/([MmLlHhVvCcSsQqTtZz])([^MmLlHhVvCcSsQqTtZz]*)/g)) {
    const nums = (m[2].match(/[+-]?(?:\d+\.?\d*|\.\d+)(?:[eE][+-]?\d+)?/g) ?? []).map(Number);
    const n = ARGS[m[1].toUpperCase()];
    if (n === 0) {
      out.push({ cmd: m[1], args: [] });
      continue;
    }
    for (let i = 0; i < nums.length; i += n) {
      const repeat = i > 0 && m[1] === 'M' ? 'L' : i > 0 && m[1] === 'm' ? 'l' : m[1];
      out.push({ cmd: repeat, args: nums.slice(i, i + n) });
    }
  }
  return out;
}

/** Absolute Punkte (Endpunkte und Kontrollpunkte) aller Befehle. */
export function points(d: string): [number, number][] {
  const pts: [number, number][] = [];
  let cx = 0, cy = 0, sx = 0, sy = 0;
  for (const { cmd, args } of commands(d)) {
    const rel = cmd === cmd.toLowerCase(), up = cmd.toUpperCase();
    if (up === 'Z') {
      [cx, cy] = [sx, sy];
      continue;
    }
    if (up === 'H') cx = (rel ? cx : 0) + args[0];
    else if (up === 'V') cy = (rel ? cy : 0) + args[0];
    else {
      const bx = rel ? cx : 0, by = rel ? cy : 0;
      for (let i = 0; i < args.length; i += 2) pts.push([bx + args[i], by + args[i + 1]]);
      [cx, cy] = pts[pts.length - 1];
    }
    if (up === 'H' || up === 'V') pts.push([cx, cy]);
    if (up === 'M') [sx, sy] = [cx, cy];
  }
  return pts;
}

/** Rahmen über mehrere Pfade; undefined, wenn keiner einen Punkt hat. */
export function box(forms: string[]): Box | undefined {
  const pts = forms.flatMap(points);
  if (pts.length === 0) return undefined;
  const xs = pts.map((p) => p[0]), ys = pts.map((p) => p[1]);
  return { x0: Math.min(...xs), y0: Math.min(...ys), x1: Math.max(...xs), y1: Math.max(...ys) };
}
