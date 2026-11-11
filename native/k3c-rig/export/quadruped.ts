// Rig eines vierbeiner (Pferd): starre Teile als Bilder, Beine als Band (Ober- und Unterbein sind ein gebogenes
// Papierstück), Platz `sattel` für den Reiter, `maul` für die Zügel. Bilder in der Dichte der Reit-Sheets.
import type { Punkt, Spec, Stil } from '../../../tools/paper/schema/types';
import { farbe } from '../../../tools/paper/render/stil';
import { beinpfad, beinPunkte } from '../../../tools/paper/rig/beinpfad';
import { BEINE, type Bein } from '../../../tools/paper/rig/gangarten';
import { FUESSE, fussVon } from '../../../tools/paper/rig/proportionen';
import { reitRahmen } from '../../../tools/paper/rig/reiten';
import { blattDichte, SITZ } from './figure';
import { type Bereich, type Kontext, runde } from './raster';
import { bildMitVarianten, ebeneVon, neuesZiel, type Quelle, quelle, rigTeile, type Ziel } from './parts';
import { BAND_RAND, type Dateien, type Rig, type RigBand, type RigPlatz } from './types';

/** Versatz des Reiters zum Reittier in Welt-Einheiten (tools/paper/rig/gangarten.ts › gangPose, ohne riderY). */
const REITER_VERSATZ: Punkt = [-2, -54];
/** Maul am Teil `hals` (reiten.ts › zuegel) und Durchhang der Zügel. */
const MAUL: Punkt = [60, -91];
const DURCHHANG = 8;
const RAND = 30;

function kontext(spec: Spec, reiter: Spec, stil: Stil): Kontext {
  const m = spec.ausgabe.massstab ?? 1, [x, y, b, h] = spec.ausgabe.rahmen.map((v) => v * m), g = reitRahmen(reiter, spec);
  // Das Reit-Sheet rendert der Reiter: Papierstruktur mit seinem Seed, Filter in der Dichte des Reit-Rahmens.
  return { stil, papierId: reiter.id, blattDichte: g.frameH / g.rahmen[3], dichte: 2 * blattDichte(reiter), massstab: m,
    bereich: [x - RAND, y - RAND, b + 2 * RAND, h + 2 * RAND] };
}

/** Bein in Ruhelage (alle Winkel 0): gerade nach unten, Stützpunkte samt Spitze des Fußes. */
function beinBand(k: Kontext, q: Quelle, z: Ziel, b: Bein): RigBand {
  const spec = q.basis, ober = spec.teile.find((t) => t.id === `oberbein-${b}`)!, P = beinPunkte(spec, b, {});
  const ruhe = [...P.map((p) => p[1]), P[5][1] + FUESSE[fussVon(spec)].spitze], m = k.massstab;
  const bereich: Bereich = [(ober.pivot[0] - 26) * m, (ruhe[0] - 14) * m, 52 * m, (ruhe[6] - ruhe[0] + 28) * m];
  const bild = bildMitVarianten({ ...k, bereich }, q, `bein-${b}`, (s) => beinpfad(s, b, {}, k.stil), z);
  // Kanten wie beinpfad: halbe Breite je Stützpunkt, vorn rechts der Wulst an der Oberbein-Mitte; an der Spitze läuft das Bein zu
  const W = BEINE[b].vorn ? spec.bein!.breiten_vorn : spec.bein!.breiten_hinten;
  const breite = ruhe.map((_, i): [number, number] => {
    const halb = i < W.length ? W[i] / 2 : 1, wulst = BEINE[b].vorn && i === 2 ? 1.6 : 0;
    return [runde(halb + BAND_RAND[0]), runde(halb + wulst + BAND_RAND[1])];
  });
  return { id: `bein-${b}`, teil: ober.eltern!, ebene: ebeneVon(spec, ober.id), mitte: ober.pivot[0], ruhe, breite, ...bild };
}

function plaetze(spec: Spec): RigPlatz[] {
  const m = spec.ausgabe.massstab ?? 1, hals = ebeneVon(spec, 'hals');
  const sattel: Punkt = [(REITER_VERSATZ[0] + SITZ[0]) / m, (REITER_VERSATZ[1] + SITZ[1]) / m];
  return [
    { name: 'sattel', teil: 'rumpf-vorn', punkt: sattel, drehung: 0, ebene: hals - 5 },
    { name: 'maul', teil: 'hals', punkt: MAUL, drehung: 0, ebene: hals + 5 },
  ];
}

/** Gangarten des Tiers: Animationen ohne Ebene, dazu der Stand (Reit-Idle). */
function gangarten(spec: Spec): Rig['animationen'] {
  const eigen = Object.entries(spec.animationen ?? {}).filter(([, a]) => !a.ebene);
  return { ...Object.fromEntries(eigen), stand: { frames: 8, gangart: 'stand' } };
}

/** `reiter` liefert die Varianten (Spielerfarben färben auch die Satteldecke) und die Dichte der Reit-Sheets. */
export function exportVierbeiner(spec: Spec, reiter: Spec, stil: Stil, dateien: Dateien): Rig {
  const k = kontext(spec, reiter, stil), q = quelle(spec, reiter.varianten), z = neuesZiel(spec.id, dateien, q);
  const teile = rigTeile(k, q, z);
  const baender = (Object.keys(BEINE) as Bein[]).map((b) => beinBand(k, q, z, b));
  return {
    id: spec.id, rig: 'vierbeiner', version: spec.version, dichte: k.dichte * k.massstab, massstab: k.massstab,
    rahmen: reitRahmen(reiter, spec).rahmen, teile, plaetze: plaetze(spec), baender,
    bein: { spitze: FUESSE[fussVon(spec)].spitze }, ...(spec.proportionen ? { proportionen: spec.proportionen } : {}),
    animationen: gangarten(spec), varianten: z.varianten,
    zuegel: { farbe: farbe(stil, 'leder', 'schatten'), deckkraft: stil.papier.falz_deckkraft, breite: 1.3, durchhang: DURCHHANG,
      ebene: ebeneVon(spec, 'hals') + 5 },
  };
}
