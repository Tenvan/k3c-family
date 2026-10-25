import { describe, expect, it } from 'vitest';
import { LIGHT } from '../../model/data';
import type { CycleInfo, World } from '../../model/types';
import { NIGHT_BRIGHTNESS, brightnessOf, glowAlpha, lightSources } from './nightLight';

const cycle = (phase: CycleInfo['phase'], progress: number) => ({ phase, progress });
/** Burg auf Hub-Stufe 1, Gold offen und nichts bezahlt: Look `lagerfeuer` (hubLookOf) */
const camp = { castle: { x: 40 }, hubLevel: 1, hubUpgrade: { gold: 10, paid: 0 }, danger: false } as unknown as World;
const castle = { castle: { x: 40 }, hubLevel: 2, hubUpgrade: null, danger: false } as unknown as World;

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

describe('Lichtquellen (W13.1, B-472/AC-02)', () => {
  it('Lagerfeuer leuchtet an der Burg mit Radius und Stärke aus data/', () => {
    const { lit, missing } = lightSources(camp);
    expect(missing).toEqual([]);
    expect(lit).toEqual([{ kind: 'campfire', x: 40, ...LIGHT.sources.campfire! }]);
  });

  it('ausgebaute Burg ist kein Lagerfeuer, Fackeln gibt es als Spielobjekt noch nicht', () => {
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
