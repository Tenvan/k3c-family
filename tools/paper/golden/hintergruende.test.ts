// MG11.2/MG11.3 (B-722): Hintergründe ohne Bäume (AC-01), fünf Varianten je Biom und Ebene (AC-03).
import { readFileSync } from 'node:fs';
import { basename, join } from 'node:path';
import { describe, expect, it } from 'vitest';
import { specsIn } from '../render/ausgabe';
import { ROOT } from '../render/stil';
import type { Spec } from '../schema/types';

const ordner = join(ROOT, 'public/grafik/hintergruende');
const specs = specsIn(ordner).map((p) => JSON.parse(readFileSync(p, 'utf8')) as Spec);
const BAUM = /baum|baeume|tanne|birke|pappel|eiche|stamm|laub/i;

describe('Hintergründe (B-722)', () => {
  it('kein Teil ist ein Baum, keine Beschreibung nennt Bäume als Motiv (AC-01)', () => {
    const treffer = specs.flatMap((s) => [
      ...s.teile.filter((t) => BAUM.test(t.id)).map((t) => `${s.id}:${t.id}`),
      ...(BAUM.test(s.beschreibung.replace(/ohne Bäume/g, '')) ? [`${s.id}:beschreibung`] : []),
    ]);
    expect(treffer).toEqual([]);
  });

  it('Specs heißen hg-<biom>-<ebene>-v<n>', () => {
    for (const p of specsIn(ordner)) expect(basename(p)).toMatch(/^hg-(wald|hoehle|mine|eisenstollen|kristall)-(fern|mitte|nah)-v[1-5]\.paper\.json$/);
  });
});
