// Optionale Elemente an den Plätzen des Monarchen: das Schwert aus der Spec und zwei Platzhalter nur für den
// Prototyp (B-778 › Offene Fragen: keine Spec in public/grafik). Lagen in Einheiten der Figur, Ruhelage.
import type { Lage, Spec } from '../../../tools/paper/schema/types';
import { lageSvg } from '../../../tools/paper/render/svg';
import { WAFFEN_TEIL } from './figure';
import { type Kontext, rastere, runde } from './raster';
import type { Dateien, Element, Rig } from './types';

type Vorlage = { id: string; platz: string; lagen: Lage[] };

/** Streitaxt am Gürtel statt des Schwerts. */
const AXT: Vorlage = { id: 'axt', platz: 'waffe', lagen: [
  { art: 'flaeche', form: 'M-6.2 -57.6L-23.4 -31.2L-21.4 -29.8L-4.2 -56.2Z', material: 'holz' },
  { art: 'falz', form: 'M-7 -54L-20 -34', breite: 0.8, material: 'holz', ton: 'licht' },
  { art: 'flaeche', form: 'M-19 -41C-25 -45 -31 -43 -33 -37C-30 -33 -27 -30 -24 -26C-22 -29 -20 -33 -17 -38Z', material: 'eisen' },
  { art: 'flaeche', form: 'M-33 -37C-30 -33 -27 -30 -24 -26C-25 -30 -27 -34 -30 -38Z', material: 'eisen', ton: 'licht', auflage: true },
  { art: 'punkt', mitte: [-19.5, -37.5], radius: 1.2, material: 'gold', ton: 'schatten' },
] };

/** Brustpanzer über dem Rock, unter Kopf und vorderem Arm. */
const BRUSTPANZER: Vorlage = { id: 'brustpanzer', platz: 'ruestung', lagen: [
  { art: 'flaeche', form: 'M-11.5 -87C-14 -78 -15 -64 -14.5 -50C-8 -46.5 8 -46.5 14.5 -50C15 -64 14 -78 11.5 -87C4 -90 -4 -90 -11.5 -87Z', material: 'eisen' },
  { art: 'flaeche', form: 'M-11.5 -87C-14 -78 -15 -64 -14.5 -50C-11 -48.2 -8 -47.5 -5 -47.2C-6.5 -60 -6.5 -76 -4.5 -89Z', material: 'eisen', ton: 'schatten', auflage: true },
  { art: 'flaeche', form: 'M7.5 -86C10 -77 11.3 -64 11.6 -52L13.2 -52.6C13 -64 12 -77 9.6 -86.8Z', material: 'eisen', ton: 'licht', auflage: true },
  { art: 'falz', form: 'M1.5 -88V-48', breite: 1, material: 'eisen', ton: 'licht' },
  { art: 'flaeche', form: 'M-14.8 -58H14.8V-54.5H-14.8Z', material: 'gold', auflage: true },
  { art: 'punkt', mitte: [-9, -80], radius: 1.1, material: 'gold' },
  { art: 'punkt', mitte: [9, -80], radius: 1.1, material: 'gold' },
] };

/** Elemente des Monarchen; `versatz` zählt ab dem Punkt des Platzes. */
export function exportElemente(spec: Spec, rig: Rig, k: Kontext, dateien: Dateien): Element[] {
  const schwert: Vorlage = { id: WAFFEN_TEIL, platz: 'waffe', lagen: spec.teile.find((t) => t.id === WAFFEN_TEIL)!.lagen };
  return [schwert, AXT, BRUSTPANZER].map((v) => {
    const platz = rig.plaetze.find((p) => p.name === v.platz);
    if (!platz) throw new Error(`Platz ${v.platz} fehlt im Rig ${rig.id}`);
    const roh = rastere(k, `<g>${v.lagen.map((l) => lageSvg(l, k.stil)).join('')}</g>`, v.id);
    dateien.set(`elemente/${v.id}.png`, roh.png);
    const versatz: [number, number] = [runde(roh.versatz[0] - platz.punkt[0]), runde(roh.versatz[1] - platz.punkt[1])];
    return { id: v.id, platz: v.platz, dichte: k.dichte, bild: `${v.id}.png`, versatz, groesse: roh.groesse };
  });
}
