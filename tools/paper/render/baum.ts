// Teile-Baum: Welt-Matrix je Teil. Ein Teil dreht um seinen Pivot, Kinder erben die Drehung (paper-spec.md › 3).
import type { Spec, Teil } from '../schema/types';
import { drehung, EINHEIT, type M, mul } from './matrix';

/** Welt-Matrizen aller Teile in Listenreihenfolge; `basis` wirkt auf alle (z. B. Versatz, Maßstab). */
export function weltMatrizen(spec: Pick<Spec, 'teile'>, winkel: Record<string, number>, basis: M = EINHEIT): Map<string, M> {
  const nachId = new Map<string, Teil>(spec.teile.map((t) => [t.id, t]));
  const fertig = new Map<string, M>();
  const welt = (t: Teil): M => {
    const da = fertig.get(t.id);
    if (da) return da;
    const eltern = t.eltern ? nachId.get(t.eltern) : undefined;
    const m = mul(eltern ? welt(eltern) : basis, drehung(winkel[t.id] ?? 0, t.pivot[0], t.pivot[1]));
    fertig.set(t.id, m);
    return m;
  };
  return new Map(spec.teile.map((t) => [t.id, welt(t)]));
}
