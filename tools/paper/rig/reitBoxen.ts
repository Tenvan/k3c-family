// Geometrie des Reittiers im Reit-Bild (B-709, paper-spec.md › 8): jedes Teil mit Präfix `reittier:`, damit `frame` es
// neben gleichnamigen Teilen des Reiters (rumpf, kopf) als eigene Box liefert. Ober- und Unterbein haben keine Lagen
// (der Beinpfad zeichnet sie); ihre Box entsteht aus den Stützpunkten des Beinpfads samt halber Breite und Wulst (B-845).
import type { Punkt, Spec, Teil } from '../schema/types';
import { geometrie, type Geometrie } from '../render/bild';
import { beinPunkte, WULST } from './beinpfad';
import { BEINE, type Bein, type GangPose } from './gangarten';
import { vierbeinerMatrizen } from './vierbeiner';

export const REITTIER_PRAEFIX = 'reittier:';

/** Umriss der Stützpunkte, je Punkt um die halbe Breite nach allen Seiten erweitert (nur für die Box). */
function umriss(pts: Punkt[], breiten: number[]): string {
  const ecken = pts.flatMap(([x, y], i) => {
    const w = (breiten[i] ?? 0) / 2;
    return [[x - w, y - w], [x + w, y + w]];
  });
  return 'M' + ecken.map(([x, y]) => `${x} ${y}`).join('L');
}

/** Ein Bein ist aus, wenn Ober- oder Unterbein in `ohne` steht: der Beinpfad ist ein Papierstück (B-845). */
export const beinAus = (b: Bein, ohne?: ReadonlySet<string>) => !!ohne && (ohne.has(`oberbein-${b}`) || ohne.has(`unterbein-${b}`));

function beinGeometrie(reittier: Spec, p: GangPose, m: Map<string, Geometrie[number]['m']>, ohne?: ReadonlySet<string>): Geometrie {
  const teil = (id: string) => reittier.teile.find((t) => t.id === id);
  return (Object.keys(BEINE) as Bein[]).flatMap((b) => {
    const ober = teil(`oberbein-${b}`), unter = teil(`unterbein-${b}`), basis = ober?.eltern && m.get(ober.eltern);
    if (!ober || !unter || !basis || !reittier.bein || beinAus(b, ohne)) return [];
    const P = beinPunkte(reittier, b, p.winkel), vorn = BEINE[b].vorn;
    // Breite samt Wulst (Stützpunkt 2 der Vorderbeine, beinpfad.ts): die Box erweitert nach allen Seiten um die Hälfte
    const W = (vorn ? reittier.bein.breiten_vorn : reittier.bein.breiten_hinten).map((w, i) => (vorn && i === 2 ? w + 2 * WULST : w));
    const mit = (t: Teil, von: number, bis: number): Geometrie[number] =>
      ({ teil: { ...t, lagen: [{ art: 'flaeche', form: umriss(P.slice(von, bis), W.slice(von, bis)), material: 'stoff' }] }, m: basis });
    return [mit(ober, 1, 4), mit(unter, 3, 6)];
  });
}

/** Geometrie aller Teile des Reittiers in der Pose, ids mit Präfix `reittier:`; Beinteile mit Box aus dem Beinpfad.
 *  `ohne`: Teile (ohne Präfix) ohne Box, wie `frame --ohne reittier:<teil>` (B-845). */
export function reittierGeometrie(reittier: Spec, p: GangPose, ohne?: ReadonlySet<string>): Geometrie {
  const m = vierbeinerMatrizen(reittier, p), beine = beinGeometrie(reittier, p, m, ohne);
  const aus = new Set([...(ohne ?? []), ...reittier.teile.filter((t) => /^(ober|unter)bein-/.test(t.id)).map((t) => t.id)]);
  return [...geometrie(reittier.teile, m, aus), ...beine].map((g) => ({ teil: { ...g.teil, id: REITTIER_PRAEFIX + g.teil.id }, m: g.m }));
}
