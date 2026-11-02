import { describe, expect, it } from 'vitest';
import { LIGHT } from '../../model/data';
import type { CycleInfo, World } from '../../model/types';
import torchCase from '../../../testdata/torches.json';
import { NIGHT_BRIGHTNESS, backdropBrightnessOf, brightnessOf, glowAlpha, lightLevelAt, lightSources, shade, torchXs, type LitSource } from './nightLight';

const cycle = (phase: CycleInfo['phase'], progress: number) => ({ phase, progress });
/** Burg auf Hub-Stufe 1, Gold offen und nichts bezahlt: Look `lagerfeuer` (hubLookOf) */
const camp = { castle: { x: 40 }, hubX: 40, sites: [], hubLevel: 1, hubUpgrade: { gold: 10, paid: 0 }, danger: false } as unknown as World;
const castle = { castle: { x: 40 }, hubX: 40, sites: [], hubLevel: 2, hubUpgrade: null, danger: false } as unknown as World;

describe('Nacht-Helligkeit (W13.1, B-472/AC-01)', () => {
  it('Untergrenze kommt aus data/light.json und ist dunkler als vor W13 (0,45)', () => {
    expect(NIGHT_BRIGHTNESS).toBe(LIGHT.nightBrightness);
    expect(NIGHT_BRIGHTNESS).toBeLessThan(0.45);
  });

  it('Tag bleibt 1, Nacht = Untergrenze, Dämmerung fällt, Morgengrauen steigt linear', () => {
    expect(brightnessOf(cycle('day', 0.5))).toBe(1);
    expect(brightnessOf(cycle('night', 0.5))).toBe(NIGHT_BRIGHTNESS);
    expect(brightnessOf(cycle('dawn', 0))).toBeCloseTo(NIGHT_BRIGHTNESS, 10);
    expect(brightnessOf(cycle('dawn', 0.5))).toBeCloseTo((NIGHT_BRIGHTNESS + 1) / 2, 10);
    expect(brightnessOf(cycle('dusk', 0.5))).toBeCloseTo((NIGHT_BRIGHTNESS + 1) / 2, 10);
    expect(brightnessOf(cycle('night', 0), 0.2)).toBe(0.2);
  });
});

describe('Hintergrund und Lichtkarte (Nacht ausleuchten)', () => {
  it('Hintergrund ist nachts dunkler als der Vordergrund, am Tag beide hell', () => {
    expect(backdropBrightnessOf(cycle('night', 0))).toBeCloseTo(LIGHT.backgroundBrightness, 10);
    expect(backdropBrightnessOf(cycle('night', 0))).toBeLessThan(brightnessOf(cycle('night', 0)));
    expect(backdropBrightnessOf(cycle('day', 0))).toBe(1);
  });

  it('Lichtkarte: außen Nacht-Helligkeit, in der Mitte volle Helligkeit, dazwischen linear, am Tag überall 1', () => {
    const fire: LitSource[] = [{ kind: 'campfire', x: 10, radius: 12, strength: 1 }];
    const night = cycle('night', 0);
    expect(lightLevelAt(40, fire, night)).toBeCloseTo(NIGHT_BRIGHTNESS, 10);
    expect(lightLevelAt(10, fire, night)).toBe(1);
    expect(lightLevelAt(16, fire, night)).toBeCloseTo((NIGHT_BRIGHTNESS + 1) / 2, 10);
    expect(lightLevelAt(10, fire, cycle('day', 0))).toBe(1);
  });

  it('shade dunkelt Farben ab und begrenzt den Faktor', () => {
    expect(shade('#80c0ff', 0.5)).toBe('#406080');
    expect(shade('#80c0ff', 1)).toBe('#80c0ff');
    expect(shade('#80c0ff', -1)).toBe('#000000');
  });

  it('Reichweite aus data/ ist die verdoppelte (Lagerfeuer 12, Fackel 6 Units)', () => {
    expect(LIGHT.sources.campfire!.radius).toBe(12);
    expect(LIGHT.sources.torch!.radius).toBe(6);
  });
});

describe('Lichtquellen (W13.1, B-472/AC-02)', () => {
  it('Lagerfeuer leuchtet an der Burg mit Radius und Stärke aus data/', () => {
    const { lit, missing } = lightSources(camp);
    expect(missing).toEqual([]);
    expect(lit).toEqual([{ kind: 'campfire', x: 40, ...LIGHT.sources.campfire! }]);
  });

  it('ausgebaute Burg ohne gebaute Mauer und Turm gibt kein Licht', () => {
    expect(lightSources(castle).lit).toEqual([]);
  });

  it('fehlen Werte in data/, gibt die Quelle kein Licht und steht in missing', () => {
    expect(lightSources(camp, {})).toEqual({ lit: [], missing: ['campfire'] });
    expect(lightSources(camp, { campfire: { radius: 0, strength: 0.5 } }).lit).toEqual([]);
  });

  it('Deckkraft: am Tag 0, in der Nacht die Stärke, im Morgengrauen fallend, nie über 1', () => {
    expect(glowAlpha(0.7, cycle('day', 0.3))).toBe(0);
    expect(glowAlpha(0.7, cycle('night', 0.3))).toBeCloseTo(0.7, 10);
    const dawn = [0, 0.5, 0.9].map((p) => glowAlpha(0.7, cycle('dawn', p)));
    expect(dawn[0]).toBeGreaterThan(dawn[1]!);
    expect(dawn[1]).toBeGreaterThan(dawn[2]!);
    expect(glowAlpha(3, cycle('night', 0))).toBe(1);
    expect(glowAlpha(-1, cycle('night', 0))).toBe(0);
  });
});

describe('Fackeln an Mauer und Turm (W15.1, B-622/AC-01)', () => {
  it('gleiche Plätze wie die Sim (testdata/torches.json, engine/sim/torches_test.go)', () => {
    expect(torchXs(torchCase.sites, torchCase.hubX, torchCase.offsets)).toEqual(torchCase.torches);
  });

  it('gebauter Turm leuchtet mit dem Licht `torch` aus data/, zerstörte Mauer nicht', () => {
    const sites = [
      { kind: 'tower', x: 60, state: 'built' },
      { kind: 'wall', x: 20, state: 'unpaid' },
    ];
    const { lit, missing } = lightSources({ ...castle, sites } as unknown as World);
    expect(missing).toEqual([]);
    expect(lit).toHaveLength(LIGHT.torches.tower!.length);
    expect(lit.every((s) => s.kind === 'torch' && s.radius === LIGHT.sources.torch!.radius)).toBe(true);
  });

  it('am Tag ohne Schein, nachts mit (Deckkraft der Fackel)', () => {
    const strength = LIGHT.sources.torch!.strength;
    expect(glowAlpha(strength, cycle('day', 0.5))).toBe(0);
    expect(glowAlpha(strength, cycle('night', 0.5))).toBeGreaterThan(0);
  });
});
