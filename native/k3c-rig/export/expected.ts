// Sollwerte für den Test des Go-Rigs (B-778/AC-02): Teil-Matrizen und Stützpunkte je Frame, gerechnet von tools/paper.
import type { Punkt, Spec } from '../../../tools/paper/schema/types';
import { weltMatrizen } from '../../../tools/paper/render/baum';
import { anwenden, drehung, type M, mul, skaliere, verschiebe } from '../../../tools/paper/render/matrix';
import { ladeStil } from '../../../tools/paper/render/stil';
import { simuliere } from '../../../tools/paper/rig/animation';
import { beinPunkte } from '../../../tools/paper/rig/beinpfad';
import { BEINE, type Bein, type GangPose } from '../../../tools/paper/rig/gangarten';
import { reitZustand } from '../../../tools/paper/rig/reiten';
import { rideMatrix, vierbeinerMatrizen } from '../../../tools/paper/rig/vierbeiner';

const matrizen = (m: Map<string, M>) => Object.fromEntries(m);

/** Figur zu Fuß: Winkel, Versatz, Welt-Matrizen (mit Versatz) und Umhang-Kette je Frame. */
export function sollFigur(spec: Spec, animation: string) {
  const frames = simuliere(spec, animation).map((f) => ({
    winkel: f.winkel, versatz: f.versatz, wind: f.wind,
    matrizen: matrizen(weltMatrizen(spec, f.winkel, verschiebe(...f.versatz))), umhang: f.umhang,
  }));
  return { figur: spec.id, animation, frames };
}

/** Basis des Reiters in der Ride-Gruppe (tools/paper/rig/reiten.ts › reiterBasis). */
const reiterBasis = (p: GangPose): M => mul(verschiebe(...p.reiter.versatz), drehung(p.reiter.drehung, 0, -50));

/** Enden der Zügel in der Ride-Gruppe (reiten.ts › zuegel). */
function zuegel(reiter: Spec, reittier: Spec, p: GangPose): { hand: Punkt; maul: Punkt } {
  const hand = weltMatrizen(reiter, p.reiter.winkel, reiterBasis(p)).get('unterarm-v')!;
  const maul = weltMatrizen(reittier, p.winkel, skaliere(p.massstab)).get('hals')!;
  return { hand: anwenden(hand, [0, -50]), maul: anwenden(maul, [60, -91]) };
}

/** Reiter auf dem Reittier: Pose, Matrizen beider Figuren (Welt), Stützpunkte der Beine, Umhang und Zügel (Ride-Gruppe). */
export function sollReiten(reiter: Spec, reittier: Spec, animation: string) {
  const a = { schluessel: animation, animation, stufe: 1 };
  const frames = reitZustand(reiter, reittier, a, { stil: ladeStil() }).map((f) => {
    const p = f.pose, R0 = rideMatrix(p.ride.dy, p.ride.pitch, 1);
    const beine = Object.fromEntries((Object.keys(BEINE) as Bein[]).map((b) => [b, beinPunkte(reittier, b, p.winkel)]));
    return {
      ride: p.ride, wind: p.wind, winkel: p.winkel, reiter: p.reiter,
      matrizen: matrizen(vierbeinerMatrizen(reittier, p)), beine,
      reiterMatrizen: matrizen(weltMatrizen(reiter, p.reiter.winkel, mul(R0, reiterBasis(p)))),
      umhang: f.umhang, zuegel: zuegel(reiter, reittier, p),
    };
  });
  return { reiter: reiter.id, reittier: reittier.id, animation, frames };
}

/** JSON mit 5 Nachkommastellen (stabil und klein), ein Frame je Zeile. */
export function sollText(soll: { frames: object[] } & Record<string, unknown>): string {
  const zahl = (_k: string, v: unknown) => (typeof v === 'number' ? Math.round(v * 1e5) / 1e5 : v);
  const { frames, ...kopf } = soll;
  return `{${Object.entries(kopf).map(([k, v]) => `${JSON.stringify(k)}:${JSON.stringify(v)}`).join(',')},"frames":[\n` +
    `${frames.map((f) => JSON.stringify(f, zahl)).join(',\n')}\n]}\n`;
}
