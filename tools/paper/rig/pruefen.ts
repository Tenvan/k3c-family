// Trockenprüfung der Render-Regeln ohne Rastern (für validate): Animationen der Figur (idle/lauf oder Keyframes),
// Reittier, Gangart und Beinteile des Vierbeiners (README › D-09, B-701 D-RD1-07). Liefert alle Fehler.
import type { Spec, Stil } from '../schema/types';
import { Abgelehnt, type CliFehler } from '../render/fehler';
import { figurPose } from './figur';
import { ladeReittier, reitFeld } from './reiten';
import { gangartVon, pruefeVierbeiner } from './vierbeinerBild';

const sammle = (out: CliFehler[], f: () => void) => {
  try {
    f();
  } catch (e) {
    if (!(e instanceof Abgelehnt)) throw e;
    out.push(...e.fehler);
  }
};

export function pruefeRenderRegeln(spec: Spec, specPfad: string, stil: Stil): CliFehler[] {
  const out: CliFehler[] = [];
  const anims = Object.entries(spec.animationen ?? {});
  if (spec.rig === 'vierbeiner') {
    sammle(out, () => pruefeVierbeiner(spec));
    for (const [name, a] of anims) sammle(out, () => gangartVon(a, name));
  }
  if (spec.rig === 'figur')
    for (const [name, a] of anims) sammle(out, () => (a.reittier ? ladeReittier(specPfad, a.reittier, stil, reitFeld(name)) : figurPose(spec, name, 0)));
  return out;
}
