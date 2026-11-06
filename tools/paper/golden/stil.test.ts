// RD1.4 (AC-04): Eine Materialänderung in der Stil-Datei ändert die Hashes der betroffenen Assets ohne Spec-Änderung.
import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { describe, expect, it } from 'vitest';
import type { Spec, Stil } from '../schema/types';
import { rendereAsset } from '../render/ausgabe';
import { ladeStil } from '../render/stil';
import { goldenEintrag, SPECS } from './fixtures';

const basis = ladeStil();
const mit = (material: string, farbe: string): Stil => {
  const s = structuredClone(basis);
  s.materialien[material].basis = farbe;
  return s;
};
const render = (id: string, stil: Stil) => {
  const pfad = join(SPECS, `${id}.paper.json`), text = readFileSync(pfad, 'utf8');
  const e = goldenEintrag(rendereAsset(JSON.parse(text) as Spec, { stil, specPfad: pfad }));
  expect(readFileSync(pfad, 'utf8')).toBe(text);
  return e;
};

describe('Stiländerung (AC-04)', { timeout: 60_000 }, () => {
  it('stein.basis geändert: renderKey und PNG-Hashes der Mauer ändern sich, Spec unverändert', () => {
    const a = render('mauer', basis), b = render('mauer', mit('stein', '#808080'));
    expect(b.renderKey).not.toBe(a.renderKey);
    for (const d of Object.keys(a.dateien)) expect(b.dateien[d], d).not.toBe(a.dateien[d]);
  });

  it('purpur geändert: renderKey der Mauer ändert sich, PNG-Hashes nicht', () => {
    const a = render('mauer', basis), b = render('mauer', mit('purpur', '#000000'));
    expect(b.renderKey).not.toBe(a.renderKey);
    expect(b.dateien).toEqual(a.dateien);
  });

  it('koenigsrot geändert: Monarch-PNGs ändern sich (Umhang), Pferd-PNGs nicht', () => {
    const rot = mit('koenigsrot', '#00ff00');
    const m0 = render('monarch', basis), m1 = render('monarch', rot);
    expect(m1.dateien['idle.png']).not.toBe(m0.dateien['idle.png']);
    expect(m1.dateien['reiten.png']).not.toBe(m0.dateien['reiten.png']);
    expect(render('pferd', rot).dateien).toEqual(render('pferd', basis).dateien);
  }, 120_000);
});
