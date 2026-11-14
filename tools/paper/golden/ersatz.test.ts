// PT2.2 (B-735): stroh, lava, geisterweiss, fell-grau stehen in der Stil-Datei; die Assets aus B-735 › Ausgangslage
// nutzen keine Ersatz-Abbildung mehr (Strohhut, Lava, Geisterweiß, graues Fell, Eisen und Kristall bei Gegnern).
import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { describe, expect, it } from 'vitest';
import { checkStil } from '../schema/check';
import type { Spec } from '../schema/types';
import { ladeStil, ROOT } from '../render/stil';

const GRAFIK = join(ROOT, 'public/grafik');
const stil = ladeStil();
const spec = (pfad: string) => JSON.parse(readFileSync(join(GRAFIK, `${pfad}.paper.json`), 'utf8')) as Spec;
/** Materialien je Asset, optional nur der genannten Teile */
const materialien = (pfad: string, nur?: string[]): Set<string> =>
  new Set(spec(pfad).teile.filter((t) => !nur || nur.includes(t.id)).flatMap((t) => t.lagen.map((l) => l.material)));

describe('Stil-Datei (B-735/AC-01)', () => {
  it.each(['stroh', 'lava', 'geisterweiss', 'fell-grau'])('Material %s mit licht, basis, schatten', (m) => {
    expect(Object.keys(stil.materialien[m]).sort()).toEqual(['basis', 'licht', 'schatten']);
  });

  it('Stil-Datei erfüllt das Schema', () => {
    expect(checkStil(stil)).toEqual([]);
  });
});

describe('Keine Ersatz-Abbildung mehr (B-735/AC-02)', () => {
  // [Asset, Teile (leer = alle), Ersatz, das nicht mehr vorkommt, neues Material]
  const faelle: [string, string[] | undefined, string[], string][] = [
    ['truppen/bauer', ['hut'], ['leinen'], 'stroh'],
    ['bosse/lava-golem', undefined, ['koenigsrot'], 'lava'],
    ['gegner/lavaschleim', undefined, ['koenigsrot'], 'lava'],
    ['gegner/feuergeist', undefined, ['koenigsrot', 'fell-fuchs'], 'lava'],
    ['gegner/grubengeist', undefined, ['hermelin'], 'geisterweiss'],
    ['gegner/alphawolf', undefined, ['hermelin', 'stein'], 'geisterweiss'],
    ['gegner/wolf', undefined, ['stein'], 'fell-grau'],
    ['reittiere/wolf', undefined, ['stoff'], 'fell-grau'],
    ['gegner/eisenkaefer', undefined, ['stein'], 'eisen'],
    ['gegner/skelett', ['schwert', 'schild'], ['stein'], 'eisen'],
    ['gegner/goblin', ['dolch'], ['stein'], 'eisen'],
    ['bosse/goblin-anfuehrer', ['schwert', 'schild'], ['stein'], 'eisen'],
    ['bosse/troll-koenig', ['schild'], ['stein'], 'eisen'],
    ['bosse/ratten-koenigin', ['unterarm-h', 'unterarm-v'], ['stein'], 'eisen'],
    ['gegner/goblin-bogenschuetze', ['pfeil'], ['stein'], 'eisen'],
    ['gegner/hoehlentroll', ['keule'], ['stein'], 'eisen'],
    ['gegner/kristallspinne', undefined, ['edelstein', 'koenigsblau'], 'kristall'],
    ['gegner/splitterling', undefined, ['edelstein', 'koenigsblau'], 'kristall'],
    ['gegner/kristallwaechter', undefined, ['edelstein', 'koenigsblau'], 'kristall'],
  ];
  it.each(faelle)('%s (%s): ohne %s, mit %s', (pfad, nur, ersatz, neu) => {
    const ms = materialien(pfad, nur);
    for (const m of ersatz) expect(ms.has(m), m).toBe(false);
    expect(ms.has(neu)).toBe(true);
  });
});
