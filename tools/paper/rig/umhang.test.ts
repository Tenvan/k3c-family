// RD1.2: Umhang als Verlet-Kette mit festem Zeitschritt (Prototyp cloth.step) und seine Zeichnung (cloth.draw).
import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { describe, expect, it } from 'vitest';
import type { Punkt, Spec } from '../schema/types';
import { ladeStil } from '../render/stil';
import { DT, Umhang, umhangSvg } from './umhang';

const monarch = JSON.parse(readFileSync(join(import.meta.dirname, '../fixtures/specs/monarch.paper.json'), 'utf8')) as Spec;
const u = monarch.umhang!;
const A: Punkt = [-4, -91];
const lauf = (h: Umhang, n: number, wind = 0, boden = 0) => {
  for (let i = 0; i < n; i++) h.schritt(A, wind, boden);
};

describe('Umhang-Physik', () => {
  it('Startkette: segmente Punkte, Abstand laenge / (segmente − 1) = 7.6', () => {
    const p = new Umhang(u, A).punkte();
    expect(p).toHaveLength(8);
    expect(p[0]).toEqual(A);
    expect(p[3]).toEqual([A[0] - 4.5, A[1] + 3 * 7.6]);
  });

  it('schritt: dt = 1/120, Dämpfung 0.99, ax = 2.4·(luft − vx), ay = 260 − 1.2·vy, Abstand zum Anker = Segment', () => {
    expect(DT).toBe(1 / 120);
    const kurz = { ...u, laenge: 10, segmente: 2 };
    const h = new Umhang(kurz, A);
    h.schritt(A, 0, 0);
    const zeit = DT, boe = 22 + 18 * (0.5 + 0.5 * Math.sin(zeit * 1.1)) + 6 * Math.sin(zeit * 2.9);
    const luft = -8 - 6 * Math.sin(zeit * 1.7) - boe;
    const nx = A[0] - 1.5 + 2.4 * luft * DT * DT, ny = A[1] + 10 + 260 * DT * DT;
    const d = Math.hypot(nx - A[0], ny - A[1]);
    const [x, y] = h.punkte()[1];
    expect(x).toBeCloseTo(A[0] + ((nx - A[0]) * 10) / d, 9);
    expect(y).toBeCloseTo(A[1] + ((ny - A[1]) * 10) / d, 9);
  });

  it('kein Punkt vor anker.x + 2 und keiner unter dem Boden (y > 0)', () => {
    const h = new Umhang(u, A);
    lauf(h, 600, -400);
    for (const [x, y] of h.punkte()) {
      expect(x).toBeLessThanOrEqual(A[0] + 2 + 1e-9);
      expect(y).toBeLessThanOrEqual(0);
    }
    const r = new Umhang(u, A);
    lauf(r, 300, 240, -103);
    for (const [, y] of r.punkte().slice(1)) expect(y).toBeLessThanOrEqual(-103);
  });

  it('Böe nur ohne Fahrtwind: mit Wind weht der Umhang weiter nach hinten', () => {
    const still = new Umhang(u, A), wind = new Umhang(u, A);
    lauf(still, 240);
    lauf(wind, 240, 240);
    expect(wind.punkte()[7][0]).toBeLessThan(still.punkte()[7][0]);
    expect(still.punkte()[7][0]).toBeLessThan(A[0]);
  });

  it('deterministisch: zwei Simulationen über 240 + 60 Schritte gleich bis aufs Bit', () => {
    const a = new Umhang(u, A), b = new Umhang(u, A);
    lauf(a, 300, 30);
    lauf(b, 300, 30);
    expect(a.punkte()).toEqual(b.punkte());
  });
});

describe('Umhang zeichnen', () => {
  const stil = ladeStil();
  const punkte = new Umhang(u, A).punkte();

  it('umhangSvg: Fläche basis mit ds, Innenseite schatten mit ds2, Licht-Kante als falz', () => {
    const svg = umhangSvg(punkte, u, stil);
    expect(svg).toContain(`fill="${stil.materialien.koenigsrot.basis}" filter="url(#ds)"`);
    expect(svg).toContain(`fill="${stil.materialien.koenigsrot.schatten}" filter="url(#ds2)"`);
    expect(svg).toContain(`stroke="${stil.materialien.koenigsrot.licht}" stroke-width="1.8"`);
    expect(svg.match(/<path/g)).toHaveLength(3);
  });

  it('Breite wächst linear von breite[0] bis breite[1]', () => {
    const gerade: Punkt[] = Array.from({ length: 8 }, (_, i) => [0, i * 10]);
    const svg = umhangSvg(gerade, u, stil);
    expect(svg).toMatch(/d="M-4 0C/);
    expect(svg).toContain("-17.3 70");
  });

  it('Varianten-Tausch gilt auch für das Umhang-Material', () => {
    expect(umhangSvg(punkte, { ...u, material: 'purpur' }, stil)).toContain(stil.materialien.purpur.basis);
  });
});

describe('Umhang: Randfälle (Review RD1.2)', () => {
  it('segmente = 2 zeichnet ohne Absturz (Schema erlaubt 2–32)', () => {
    const kurz = { ...u, segmente: 2, laenge: 10 };
    expect(() => umhangSvg(new Umhang(kurz, A).punkte(), kurz, ladeStil())).not.toThrow();
  });
});
