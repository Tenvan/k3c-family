// Figuren mit Teile-Baum (B-789/AC-03): alle Specs unter public/grafik/<gruppe>/ mit Rig-Typ `figur` oder `vierbeiner`;
// `requisite` und die übrigen Rig-Typen haben keinen Teile-Baum und bleiben außen vor. Ziel-Ordner `<gruppe>/<id>`,
// weil dieselbe id in zwei Gruppen vorkommt (Gegner `wolf`, Reittier `wolf`).
import { readdirSync, readFileSync, statSync } from 'node:fs';
import { join } from 'node:path';
import type { Spec } from '../../../tools/paper/schema/types';
import { ROOT } from '../../../tools/paper/render/stil';

export type Eintrag = { ordner: string; gruppe: string; spec: Spec };

/** Der Reiter aller Reittiere und die einzige Figur mit wählbaren Elementen (Waffe, Rüstung). */
export const REITER = 'figuren/monarch';
/** Gruppe der Reittiere: Vierbeiner hier tragen den Reiter (Plätze `sattel`, `maul`, Zügel). */
export const REITTIERE = 'reittiere';

const GRAFIK = join(ROOT, 'public/grafik');

export function katalog(): Eintrag[] {
  const out: Eintrag[] = [];
  for (const gruppe of readdirSync(GRAFIK).sort()) {
    if (!statSync(join(GRAFIK, gruppe)).isDirectory()) continue;
    for (const datei of readdirSync(join(GRAFIK, gruppe)).sort()) {
      if (!datei.endsWith('.paper.json') || datei === 'index.paper.json') continue;
      const spec = JSON.parse(readFileSync(join(GRAFIK, gruppe, datei), 'utf8')) as Spec;
      if (spec.rig === 'figur' || spec.rig === 'vierbeiner') out.push({ ordner: `${gruppe}/${spec.id}`, gruppe, spec });
    }
  }
  const doppelt = out.filter((e, i) => out.findIndex((x) => x.ordner === e.ordner) !== i);
  if (doppelt.length) throw new Error(`Ziel-Ordner doppelt: ${doppelt.map((e) => e.ordner).join(', ')}`);
  return out;
}
