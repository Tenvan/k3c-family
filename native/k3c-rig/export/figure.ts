// Rig einer figur: starre Teile als Bilder, Umhang als Band (falls vorhanden); beim Monarchen Plätze für Waffe,
// Rüstung, Kopfbedeckung und Zügel-Hand.
import type { Punkt, Spec, Stil } from '../../../tools/paper/schema/types';
import { mitNachkommen } from '../../../tools/paper/render/svg';
import { umhangSvg } from '../../../tools/paper/rig/umhang';
import { type Bereich, type Kontext, runde } from './raster';
import { bildMitVarianten, ebeneVon, neuesZiel, quelle, rigTeile, type Ziel } from './parts';
import { BAND_RAND, type Dateien, type Rig, type RigBand, type RigPlatz } from './types';

/** Teil, das zum Element am Platz `waffe` wird (kein festes Teil des Rigs). */
export const WAFFEN_TEIL = 'schwert';
/** Sitzpunkt des Reiters: Drehpunkt der Reit-Neigung (tools/paper/rig/reiten.ts › reiterBasis). */
export const SITZ: Punkt = [0, -50];
/** Hand, die die Zügel hält (reiten.ts › zuegel). */
const HAND: Punkt = [0, -50];
const RAND = 30;

/** Pixel je Einheit im Sheet der Figur. */
export const blattDichte = (spec: Spec) => spec.ausgabe.hoehe_px / spec.ausgabe.rahmen[3];

export function figurKontext(spec: Spec, stil: Stil): Kontext {
  const [x, y, b, h] = spec.ausgabe.rahmen, px = blattDichte(spec);
  return { stil, papierId: spec.id, blattDichte: px, dichte: 2 * px, massstab: 1, bereich: [x - RAND, y - RAND, b + 2 * RAND, h + 2 * RAND] };
}

function plaetze(spec: Spec): RigPlatz[] {
  const waffe = spec.teile.find((t) => t.id === WAFFEN_TEIL)!;
  return [
    { name: 'waffe', teil: waffe.eltern!, punkt: waffe.pivot, drehung: 0, ebene: ebeneVon(spec, WAFFEN_TEIL) },
    { name: 'ruestung', teil: 'rumpf', punkt: [0, -70], drehung: 0, ebene: ebeneVon(spec, 'rumpf') + 5 },
    { name: 'kopfbedeckung', teil: 'kopf', punkt: spec.teile.find((t) => t.id === 'krone')!.pivot, drehung: 0, ebene: ebeneVon(spec, 'krone') + 5 },
    { name: 'hand', teil: 'unterarm-v', punkt: HAND, drehung: 0, ebene: ebeneVon(spec, 'unterarm-v') + 5 },
  ];
}

/** Umhang in Ruhelage: Kette senkrecht nach unten, Bild wie im Sheet (Fläche, Innenseite, Licht-Kante). */
function umhangBand(k: Kontext, spec: Spec, z: Ziel, q: ReturnType<typeof quelle>): RigBand {
  const u = spec.umhang!, n = u.segmente ?? 8, seg = u.laenge / (n - 1), w = u.breite[1] + 14;
  const ruhe = Array.from({ length: n }, (_, i) => i * seg);
  const bereich: Bereich = [-w, -14, 2 * w, u.laenge + 28];
  const bild = bildMitVarianten({ ...k, bereich }, q, 'umhang', (s) => umhangSvg(ruhe.map((y): Punkt => [0, y]), s.umhang!, k.stil), z);
  // Kanten wie umhangSvg: links die volle Breite, rechts 0,6 davon
  const breite = ruhe.map((_, i): [number, number] => {
    const b = u.breite[0] + (i * (u.breite[1] - u.breite[0])) / (n - 1);
    return [runde(b + BAND_RAND[0]), runde(b * 0.6 + BAND_RAND[1])];
  });
  return { id: 'umhang', ebene: -10, mitte: 0, ruhe, breite, ...bild };
}

/** Animationen ohne Reittier (Reiten kommt aus dem Rig des Reittiers). */
export const eigeneAnimationen = (spec: Spec) =>
  Object.fromEntries(Object.entries(spec.animationen ?? {}).filter(([, a]) => !a.reittier));

/** Figur reitet, wenn eine Animation ein Reittier nennt (Monarch). */
const reitet = (spec: Spec) => Object.values(spec.animationen ?? {}).some((a) => a.reittier);

/** `elemente`: Figur mit wählbaren Elementen (Monarch): Waffe als Element, Plätze für Rüstung, Kopfbedeckung und
 * Zügel-Hand. Sonst bleiben alle Teile fest (auch die Waffe) und das Rig hat keine Plätze. */
export function exportFigur(spec: Spec, stil: Stil, dateien: Dateien, ordner: string, elemente: boolean): Rig {
  const k = figurKontext(spec, stil), q = quelle(spec, spec.varianten), z = neuesZiel(ordner, dateien, q);
  const teile = rigTeile(k, q, z, new Set(elemente ? [WAFFEN_TEIL] : []));
  return {
    id: spec.id, rig: 'figur', version: spec.version, dichte: k.dichte, massstab: 1, rahmen: spec.ausgabe.rahmen,
    teile, plaetze: elemente ? plaetze(spec) : [], baender: spec.umhang ? [umhangBand(k, spec, z, q)] : [],
    ...(spec.umhang ? { umhang: spec.umhang } : {}), ...(spec.proportionen ? { proportionen: spec.proportionen } : {}),
    animationen: eigeneAnimationen(spec), varianten: z.varianten, ...(elemente ? { standard: { waffe: WAFFEN_TEIL } } : {}),
    ...(reitet(spec) ? { reiten: { sitz: SITZ, ausblenden: [...mitNachkommen(spec.teile, 'oberschenkel-h')] } } : {}),
  };
}
