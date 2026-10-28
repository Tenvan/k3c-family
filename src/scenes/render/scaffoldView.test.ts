import { describe, expect, it } from 'vitest';
import type { SiteKind, SiteState } from '../../model/types';
import { BUILDING_TEXTURES } from './buildingSprites';
import { SCAFFOLD_CLASS, SCAFFOLD_CLASSES, scaffoldTexture, showsScaffold, siteMarker } from './scaffoldView';

describe('Gerüst-Größenklasse (B-586)', () => {
  it('jede Platzart hat die Breite ihres Gebäude-Sprites als Klasse', () => {
    const erwartet: Record<SiteKind, number> = {
      wall: 1, tower: 3, nursery: 4, stairsUp: 4, stairsDown: 4,
      workshop: 6, storage: 6, barracks: 6, tavern: 6, healer: 6, smithy: 6, armory: 6,
    };
    expect(SCAFFOLD_CLASS).toEqual(erwartet);
  });
  it('jede Klasse lädt ihre 32-px-Grafik aus tools/buildings (Datei geprüft über data/assets.json)', () => {
    for (const c of SCAFFOLD_CLASSES) {
      const file = BUILDING_TEXTURES[`grafik:geruest-${c}`];
      expect(file).toBe(`gebaeude/geruest-${c}-32.png`);
    }
    expect(scaffoldTexture('workshop')).toBe('grafik:geruest-6');
    expect(scaffoldTexture('tower')).toBe('grafik:geruest-3');
  });
});

describe('Gerüst XOR Bauschild (B-586/AC-02)', () => {
  const states: SiteState[] = ['unpaid', 'waitingMaterial', 'waitingWorker', 'built'];
  it('im Bau steht das Gerüst, auch wenn der Server payable meldet', () => {
    for (const state of ['waitingMaterial', 'waitingWorker'] as const) {
      expect(showsScaffold({ state })).toBe(true);
      expect(siteMarker({ state, payable: true })).toBe('scaffold');
    }
  });
  it('unbezahlt nur das Schild bei payable, gebaut keins von beiden', () => {
    expect(siteMarker({ state: 'unpaid', payable: true })).toBe('sign');
    expect(siteMarker({ state: 'unpaid' })).toBeNull();
    expect(siteMarker({ state: 'built', payable: true })).toBeNull();
    expect(showsScaffold({ state: 'unpaid' })).toBe(false);
  });
  it('nie Schild und Gerüst zugleich', () => {
    for (const state of states) for (const payable of [true, false, undefined]) {
      const m = siteMarker({ state, payable });
      expect(m === 'sign' && showsScaffold({ state })).toBe(false);
    }
  });
});
