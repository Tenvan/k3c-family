// MG9.1 (B-716): Reittiere der Gruppe teilen den Maßstab, Wölfe und Elefant haben Fußform und Gangarten über `proportionen`.
import { readdirSync, readFileSync } from 'node:fs';
import { join } from 'node:path';
import { describe, expect, it } from 'vitest';
import type { Spec } from '../schema/types';
import { ROOT } from '../render/stil';

const DIR = join(ROOT, 'public/grafik/reittiere');
const IDS = readdirSync(DIR).filter((f) => f.endsWith('.paper.json') && f !== 'index.paper.json').map((f) => f.replace('.paper.json', ''));
const spec = (id: string) => JSON.parse(readFileSync(join(DIR, `${id}.paper.json`), 'utf8')) as Spec;

describe('Reittiere (B-716)', () => {
  it('Gruppe hat elf Reittiere', () => expect(IDS.length).toBe(11));

  it('AC-01: gleicher Maßstab (hoehe_px / Rahmenhöhe) in der ganzen Gruppe', () => {
    const faktoren = new Set(IDS.map((id) => { const a = spec(id).ausgabe!; return a.hoehe_px / a.rahmen![3]; }));
    expect(faktoren.size).toBe(1);
  });

  it.each(['wolf', 'weisser-wolf', 'hoellenhund'])('AC-02: %s hat Pfoten', (id) => expect(spec(id).proportionen?.fuss).toBe('pfote'));

  it('AC-02: Elefant hat Elefantenfüße', () => expect(spec('elefant').proportionen?.fuss).toBe('elefantenfuss'));

  it('AC-03: Elefant ohne galopp, Spec gültig', () => {
    const e = spec('elefant');
    expect(e.proportionen?.gangarten).toEqual(['schritt', 'trab']);
    expect(Object.keys(e.animationen!).sort()).toEqual(['schritt', 'trab']);
  });
});
