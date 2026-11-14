// Beinpfad des Vierbeiners (Prototyp drawLegs): Ober- und Unterbein als ein Papierstück entlang der Stützpunkte,
// Breiten aus `bein`, Material = erste flaeche von rumpf-vorn bzw. rumpf-hinten (README › D-11). Koordinaten im Rahmen
// des Rumpf-Teils (Eltern von oberbein-*); der Aufrufer setzt dessen Transformation.
import type { Lage, Punkt, Spec, Stil, Ton } from '../schema/types';
import { lageSvg } from '../render/svg';
import { BEINE, type Bein } from './gangarten';
import { crPath } from './kurve';
import { FUESSE, fussVon } from './proportionen';

const teil = (spec: Spec, id: string) => spec.teile.find((t) => t.id === id)!;
const richtung = (grad: number): Punkt => {
  const r = (grad * Math.PI) / 180;
  return [-Math.sin(r), Math.cos(r)];
};
const plus = (p: Punkt, d: Punkt, l: number): Punkt => [p[0] + d[0] * l, p[1] + d[1] * l];

/** Stützpunkte: Ansatz 16 über der Hüfte, Hüfte, Oberbein-Mitte, Knie, Unterbein-Mitte, Fessel. */
export function beinPunkte(spec: Spec, b: Bein, winkel: Record<string, number>): Punkt[] {
  const H = teil(spec, `oberbein-${b}`).pivot, knieY = teil(spec, `unterbein-${b}`).pivot[1], fesselY = teil(spec, `huf-${b}`).pivot[1];
  const a1 = winkel[`oberbein-${b}`] ?? 0, d1 = richtung(a1), d2 = richtung(a1 + (winkel[`unterbein-${b}`] ?? 0));
  const ober = knieY - H[1], unter = fesselY - knieY, K = plus(H, d1, ober);
  return [[H[0], H[1] - 16], H, plus(H, d1, ober * 0.48), K, plus(K, d2, unter * 0.5), plus(K, d2, unter)];
}

function material(spec: Spec, vorn: boolean): string {
  const lage = teil(spec, vorn ? 'rumpf-vorn' : 'rumpf-hinten').lagen.find((l) => l.art === 'flaeche');
  return lage?.material ?? 'stoff';
}

/** Wulst der Vorderbeine: die rechte Kante steht am Stützpunkt 2 um so viele Einheiten weiter hinaus (auch in der Box, B-845). */
export const WULST = 1.6;

/** Kanten des Beinpfads je Stützpunkt: links (−Normale · w), rechts (+Normale · (w + Wulst)), Schatten- und Lichtlinie. */
export function beinKanten(spec: Spec, b: Bein, winkel: Record<string, number>) {
  const P = beinPunkte(spec, b, winkel), vorn = BEINE[b].vorn;
  const W = vorn ? spec.bein!.breiten_vorn : spec.bein!.breiten_hinten;
  const links: Punkt[] = [], rechts: Punkt[] = [], schatten: Punkt[] = [], licht: Punkt[] = [];
  P.forEach((p, i) => {
    const q0 = P[Math.max(i - 1, 0)], q1 = P[Math.min(i + 1, P.length - 1)], tx = q1[0] - q0[0], ty = q1[1] - q0[1];
    const d = Math.hypot(tx, ty) || 1, nx = ty / d, ny = -tx / d, w = W[i] / 2, wulst = vorn && i === 2 ? WULST : 0;
    links.push([p[0] - nx * w, p[1] - ny * w]);
    rechts.push([p[0] + nx * (w + wulst), p[1] + ny * (w + wulst)]);
    schatten.push([p[0] - nx * w * 0.15, p[1] - ny * w * 0.15]);
    licht.push([p[0] + nx * w * 0.55, p[1] + ny * w * 0.55]);
  });
  return { P, links, rechts, schatten, licht };
}

export function beinpfad(spec: Spec, b: Bein, winkel: Record<string, number>, stil: Stil): string {
  const { P, links: Lp, rechts: Rp, schatten: Sp, licht: Hp } = beinKanten(spec, b, winkel);
  const vorn = BEINE[b].vorn, fern = b.endsWith('f');
  const F = P[5], unter = Math.hypot(F[0] - P[3][0], F[1] - P[3][1]) || 1;
  const spitze = plus(F, [(F[0] - P[3][0]) / unter, (F[1] - P[3][1]) / unter], FUESSE[fussVon(spec)].spitze);
  const m = material(spec, vorn), [koerper, licht]: Ton[] = fern ? ['schatten', 'basis'] : ['basis', 'licht'];
  const lagen: Lage[] = [
    { art: 'flaeche', form: crPath([...Lp, spitze]) + crPath([spitze, ...[...Rp].reverse()], true) + 'Z', material: m, ton: koerper },
    { art: 'flaeche', form: crPath(Lp.slice(1)) + crPath(Sp.slice(1).reverse(), true) + 'Z', material: m, ton: 'schatten', auflage: true },
    { art: 'falz', form: crPath(Hp.slice(1, 5)), breite: 1.3, material: m, ton: licht },
  ];
  return lagen.map((l) => lageSvg(l, stil)).join('');
}
