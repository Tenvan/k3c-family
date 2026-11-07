// Proportionen je Tier und Figur (B-715): Fußform, Beinlängen aus den Pivots, Gangarten des Vierbeiners und die
// Faktoren der Figur auf die Pose-Winkel. Ohne `proportionen` gilt das Verhalten von RD1 (Pferd, Krieger-Grundform).
import type { Fuss, GangartName, Spec } from '../schema/types';
import { Abgelehnt } from '../render/fehler';

/** Fußform: `spitze` = Verlängerung des Beinpfads über die Fessel. Die IK-Längen ändert sie nicht (der Fuß bleibt am Boden). */
export const FUESSE: Record<Fuss, { spitze: number }> = {
  huf: { spitze: 3.4 },
  pfote: { spitze: 2.2 },
  elefantenfuss: { spitze: 1.2 },
};

export const ALLE_GANGARTEN: GangartName[] = ['schritt', 'trab', 'galopp'];
export const fussVon = (spec: Spec): Fuss => spec.proportionen?.fuss ?? 'huf';
export const gangartenVon = (spec: Spec): GangartName[] => spec.proportionen?.gangarten ?? ALLE_GANGARTEN;

/** Längen der beiden IK-Knochen: Oberbein = Knie − Hüfte (Pivots von oberbein-hf, unterbein-hf), Unterbein bis zum Boden (Hüfthöhe − Oberbein); ohne die Teile 25 und 31 (Pferd). */
export function beinLaengen(spec: Spec): { ober: number; unter: number } {
  const y = (id: string) => spec.teile.find((t) => t.id === id)?.pivot[1];
  const hueft = y('oberbein-hf'), knie = y('unterbein-hf');
  if (hueft === undefined || knie === undefined) return { ober: 25, unter: 31 };
  return { ober: knie - hueft, unter: -hueft - (knie - hueft) };
}

/** Jede Gangart des Tiers braucht eine Animation, und keine Animation nutzt eine fremde Gangart (Regeln gangart-fehlt, gangart-nicht-erlaubt). */
export function pruefeGangarten(spec: Spec): void {
  const erlaubt: GangartName[] = [...gangartenVon(spec), 'stand'], anims = Object.entries(spec.animationen ?? {});
  const fehler = [
    ...gangartenVon(spec).filter((g) => !anims.some(([, a]) => a.gangart === g))
      .map((g) => ({ feld: 'animationen', regel: 'gangart-fehlt', text: `${spec.id} braucht eine Animation mit gangart ${g} (oder proportionen.gangarten ohne ${g})` })),
    ...anims.filter(([, a]) => a.gangart && !erlaubt.includes(a.gangart))
      .map(([n, a]) => ({ feld: `animationen.${n}.gangart`, regel: 'gangart-nicht-erlaubt', text: `gangart ${a.gangart} steht nicht in proportionen.gangarten (${erlaubt.join(', ')})` })),
  ];
  if (fehler.length) throw new Abgelehnt(fehler);
}

const FIGUR_TEILE: Record<'arme' | 'beine' | 'kopf', RegExp> = { arme: /^(ober|unter)arm-/, beine: /^(ober|unter)schenkel-/, kopf: /^(kopf|krone)$/ };

/** Faktor der Figur auf den Winkel eines Teils in den Posen idle und lauf (arme, beine, kopf); 1 ohne Eintrag. */
export function figurFaktor(spec: Spec, id: string): number {
  const p = spec.proportionen;
  if (!p) return 1;
  for (const [feld, re] of Object.entries(FIGUR_TEILE) as ['arme' | 'beine' | 'kopf', RegExp][]) if (re.test(id)) return p[feld] ?? 1;
  return 1;
}
