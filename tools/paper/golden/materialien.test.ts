// RD2.2 (B-712/AC-02, AC-03): neue Materialien und Biom-Töne stehen in der Stil-Datei, die Ersatz-Abbildungen sind ersetzt.
import { readdirSync, readFileSync } from 'node:fs';
import { join } from 'node:path';
import { describe, expect, it } from 'vitest';
import type { Spec } from '../schema/types';
import { ladeStil, ROOT } from '../render/stil';

const GRAFIK = join(ROOT, 'public/grafik');
const stil = ladeStil();
const materialien = (gruppe: string, id: string): Set<string> => {
  const spec = JSON.parse(readFileSync(join(GRAFIK, gruppe, `${id}.paper.json`), 'utf8')) as Spec;
  return new Set(spec.teile.flatMap((t) => t.lagen.map((l) => l.material)));
};

describe('Stil-Datei (AC-02)', () => {
  it.each(['schwarz', 'eisen', 'leinen', 'kupfer', 'kristall'])('Material %s mit licht, basis, schatten', (m) => {
    expect(Object.keys(stil.materialien[m]).sort()).toEqual(['basis', 'licht', 'schatten']);
  });

  it.each(['wald', 'hoehle', 'mine', 'eisenstollen', 'kristall'])('Biom-Ton biom-%s', (b) => {
    expect(Object.keys(stil.materialien[`biom-${b}`]).sort()).toEqual(['basis', 'licht', 'schatten']);
  });
});

describe('Ersatz-Materialien sind ersetzt (AC-03)', () => {
  const ohne: [string, string, string[]][] = [
    ['reittiere', 'maehne', ['rappe', 'albtraum', 'hoellenhund']],
    ['truppen', 'stein', ['bauer', 'krieger', 'elite-krieger', 'bogenschuetze', 'elite-bogenschuetze']],
    ['icons', 'fell-fuchs', ['icon-kupfer', 'icon-eisen']],
    ['objekte', 'fell-fuchs', ['ader-kupfer', 'ader-eisen', 'erzbrocken']],
    ['icons', 'koenigsblau', ['icon-kristall']],
    ['objekte', 'koenigsblau', ['ader-kristall']],
  ];
  it.each(ohne.flatMap(([g, m, ids]) => ids.map((id) => [g, m, id] as const)))('%s/%s: %s nutzt %s nicht mehr', (g, m, id) => {
    expect(materialien(g, id).has(m)).toBe(false);
  });

  it('Hintergründe nutzen das Material ihres Bioms und nicht mehr stein/smaragd/koenigsblau als Biom-Ersatz', () => {
    const dateien = readdirSync(join(GRAFIK, 'hintergruende')).filter((f) => /^hg-.*\.paper\.json$/.test(f) && f !== 'index.paper.json');
    expect(dateien.length).toBe(75); // MG11 (B-722): fünf Varianten je Biom und Ebene
    for (const f of dateien) {
      const [, biom] = f.split('-'), ms = materialien('hintergruende', f.replace('.paper.json', ''));
      expect(ms.has(`biom-${biom}`), f).toBe(true);
      const ersatz = { wald: ['smaragd', 'stein'], hoehle: ['stein'], mine: ['stein'], eisenstollen: ['stein'], kristall: ['koenigsblau', 'stein'] }[biom] ?? [];
      for (const m of ersatz) expect(ms.has(m), `${f}: ${m}`).toBe(false);
    }
  });

  it('neue Materialien werden benutzt', () => {
    expect(materialien('reittiere', 'rappe').has('schwarz')).toBe(true);
    expect(materialien('truppen', 'krieger').has('eisen')).toBe(true);
    expect(materialien('truppen', 'bauer').has('leinen')).toBe(true);
    expect(materialien('objekte', 'ader-kupfer').has('kupfer')).toBe(true);
    expect(materialien('objekte', 'ader-kristall').has('kristall')).toBe(true);
    expect(materialien('objekte', 'ader-eisen').has('eisen')).toBe(true);
  });
});
