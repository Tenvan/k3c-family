// Umhang: Verlet-Kette, oben am Anker, Schwerkraft + Fahrtwind, fester Zeitschritt 1/120 s (paper-spec.md › 6).
// Konstanten wie im Prototyp (docs/decisions/005/prototyp-monarch-rig.html › cloth.step, cloth.draw).
import type { Lage, Punkt, Spec, Stil } from '../schema/types';
import { lageSvg } from '../render/svg';
import { crPath } from './kurve';

export const DT = 1 / 120;
type UmhangSpec = NonNullable<Spec['umhang']>;
type P = { x: number; y: number; px: number; py: number };

export class Umhang {
  private readonly pts: P[];
  private readonly seg: number;
  private zeit = 0;

  constructor(u: UmhangSpec, [ax, ay]: Punkt) {
    const n = u.segmente ?? 8;
    this.seg = u.laenge / (n - 1);
    this.pts = Array.from({ length: n }, (_, i) => {
      const x = ax - i * 1.5, y = ay + i * this.seg;
      return { x, y, px: x, py: y };
    });
  }

  /** Ein Schritt: wind = Fahrtwind (Einheiten/s, 0 = Stand mit Böe), boden = tiefster erlaubter y-Wert. */
  schritt(anker: Punkt, wind: number, boden: number): void {
    this.zeit += DT;
    const t = this.zeit, P = this.pts;
    const boe = wind ? 0 : 22 + 18 * (0.5 + 0.5 * Math.sin(t * 1.1)) + 6 * Math.sin(t * 2.9);
    const luft = -wind * 0.8 - 8 - 6 * Math.sin(t * 1.7) - boe;
    for (let i = 1; i < P.length; i++) {
      const p = P[i], vx = (p.x - p.px) / DT, vy = (p.y - p.py) / DT;
      const ax = 2.4 * (luft - vx), ay = 260 - 1.2 * vy;
      const nx = p.x + (p.x - p.px) * 0.99 + ax * DT * DT, ny = p.y + (p.y - p.py) * 0.99 + ay * DT * DT;
      [p.px, p.py, p.x, p.y] = [p.x, p.y, nx, ny];
    }
    for (let it = 0; it < 8; it++) this.binde(anker, boden);
  }

  private binde([ax, ay]: Punkt, boden: number) {
    const P = this.pts;
    [P[0].x, P[0].y] = [ax, ay];
    for (let i = 1; i < P.length; i++) {
      const a = P[i - 1], b = P[i], dx = b.x - a.x, dy = b.y - a.y, d = Math.hypot(dx, dy) || 1, k = (d - this.seg) / d;
      if (i > 1) {
        a.x += dx * k * 0.5; a.y += dy * k * 0.5; b.x -= dx * k * 0.5; b.y -= dy * k * 0.5;
      } else {
        b.x -= dx * k; b.y -= dy * k;
      }
      if (b.x > ax + 2) b.x = ax + 2;
      if (b.y > boden) b.y = boden;
    }
  }

  punkte(): Punkt[] {
    return this.pts.map((p) => [p.x, p.y]);
  }
}

/** Band mit wachsender Breite: Außenfläche, Innenseite (Schatten) und Licht-Kante; der Saum entfällt in v1. */
export function umhangSvg(P: Punkt[], u: UmhangSpec, stil: Stil): string {
  const n = P.length, L: Punkt[] = [], R: Punkt[] = [], I: Punkt[] = [];
  P.forEach((p, i) => {
    const a = P[Math.max(i - 1, 0)], b = P[Math.min(i + 1, n - 1)], tx = b[0] - a[0], ty = b[1] - a[1], d = Math.hypot(tx, ty) || 1;
    const nx = ty / d, ny = -tx / d, w = u.breite[0] + (i * (u.breite[1] - u.breite[0])) / (n - 1);
    L.push([p[0] - nx * w, p[1] - ny * w]);
    R.push([p[0] + nx * w * 0.6, p[1] + ny * w * 0.6]);
    I.push([p[0] - nx * w * 0.35, p[1] - ny * w * 0.35]);
  });
  const kante = R.slice(1, -1).map(([x, y], i): Punkt => [x - (x - P[i + 1][0]) * 0.35, y - (y - P[i + 1][1]) * 0.35]);
  const lagen: Lage[] = [
    { art: 'flaeche', form: crPath(L) + crPath([...R].reverse(), true) + 'Z', material: u.material },
    { art: 'flaeche', form: crPath(L.slice(2)) + crPath(I.slice(2).reverse(), true) + 'Z', material: u.material, ton: 'schatten', auflage: true },
    { art: 'falz', form: crPath(kante), breite: 1.8, material: u.material, ton: 'licht' },
  ];
  // Bei wenigen Segmenten fehlen Innenseite und Licht-Kante (zu wenig Punkte).
  const genug = [true, L.length > 3, kante.length > 1];
  return lagen.filter((_, i) => genug[i]).map((l) => lageSvg(l, stil)).join('');
}
