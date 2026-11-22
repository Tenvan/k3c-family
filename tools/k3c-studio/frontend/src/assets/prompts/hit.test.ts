import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { describe, expect, it } from 'vitest';
import type { Frame, Spec } from '../api';
import { bildPunkt, teilAn } from './hit';

// WB1.3/AC-03 (Sprint-README D-05): Klick wählt die kleinste Box, bei Gleichstand das spätere Teil in Spec.teile.

const ui = (name: string) => JSON.parse(readFileSync(resolve(__dirname, '../../../../../paper/fixtures/ui', `${name}.json`), 'utf-8')) as unknown;
const { teile } = ui('AssetsFrame') as Frame;
const reihenfolge = (ui('AssetsGet') as Spec).teile.map((t) => t.id);

describe('teilAn (WB1.3)', () => {
  it('Kopf schlägt den Rumpf (kleinere Box)', () => {
    expect(teilAn(teile, reihenfolge, 80, 60)).toBe('kopf');
  });
  it('gleiche Boxen: das spätere Teil gewinnt', () => {
    expect(teilAn(teile, reihenfolge, 70, 70)).toBe('oberarm-v');
  });
  it('daneben: kein Teil', () => {
    expect(teilAn(teile, reihenfolge, 0, 0)).toBe('');
  });
});

describe('bildPunkt (WB1.3)', () => {
  it('rechnet Anzeige- in Bildpixel um', () => {
    expect(bildPunkt(50, 100, { width: 100, height: 200 }, { width: 200, height: 400 })).toEqual([100, 200]);
  });
  it('ohne bekannte Größe gilt 1:1', () => {
    expect(bildPunkt(5, 7, { width: 0, height: 0 }, { width: 0, height: 0 })).toEqual([5, 7]);
  });
});
