import { daylight } from '@game/scenes/viewRules';
import type { CycleInfo } from '@game/model/types';
import { describe, expect, it } from 'vitest';
import { MAX_PROGRESS, NIGHT_BRIGHTNESS, TIMES_OF_DAY, brightnessOf, simTime } from './timeOfDay';

const cycle = (phase: CycleInfo['phase'], progress: number): CycleInfo => ({ phase, progress, day: 1, secondsLeft: 0 });

describe('Tageszeit: vier Einträge, drei Sim-Phasen (B-460/AC-05)', () => {
  it('Tag, Dämmerung, Nacht und Morgengrauen stehen zur Auswahl, keiner ausgegraut', () => {
    expect(TIMES_OF_DAY.map((t) => t.label)).toEqual(['Tag', 'Dämmerung', 'Nacht', 'Morgengrauen']);
  });

  it('Tag, Dämmerung und Nacht gehen unverändert an die Sim-Phase', () => {
    for (const id of ['day', 'dusk', 'night'] as const) {
      expect(simTime(id, 0)).toEqual({ phase: id, progress: 0 });
      expect(simTime(id, 0.5)).toEqual({ phase: id, progress: 0.5 });
    }
  });

  it('Morgengrauen = Nacht mit Fortschritt 0,9 + 0,1·p', () => {
    for (const [p, want] of [[0, 0.9], [0.5, 0.95], [0.9, 0.99]] as const) {
      const t = simTime('dawn', p);
      expect(t.phase).toBe('night');
      expect(t.progress).toBeCloseTo(want, 10);
    }
  });

  it('Fortschritt p bleibt in [0, 0,99]: Go lehnt 1 ab, unlesbare Werte werden 0', () => {
    expect(simTime('day', 1).progress).toBe(MAX_PROGRESS);
    expect(simTime('dawn', 1).progress).toBeLessThan(1);
    expect(simTime('dawn', 5).progress).toBeLessThan(1);
    expect(simTime('dusk', -3).progress).toBe(0);
    expect(simTime('night', Number.NaN).progress).toBe(0);
  });

  it('Helligkeit nach daylight(): beim Morgengrauen gleich p, bei Tag 1, bei Dämmerung 1 − p', () => {
    for (const p of [0, 0.25, 0.5, 0.9]) {
      const dawn = simTime('dawn', p);
      expect(daylight(cycle(dawn.phase, dawn.progress))).toBeCloseTo(p, 10);
      const dusk = simTime('dusk', p);
      expect(daylight(cycle(dusk.phase, dusk.progress))).toBeCloseTo(1 - p, 10);
      expect(daylight(cycle('day', simTime('day', p).progress))).toBe(1);
    }
    expect(daylight(cycle('night', simTime('night', 0.5).progress))).toBe(0);
  });

  it('Kamerahelligkeit: Nacht = Untergrenze, Tag = 1, Morgengrauen steigt', () => {
    expect(brightnessOf(cycle('day', 0))).toBe(1);
    expect(brightnessOf(cycle('night', 0.5))).toBe(NIGHT_BRIGHTNESS);
    const dawn = [0, 0.5, 0.9].map((p) => brightnessOf(simTime('dawn', p)));
    expect(dawn[0]).toBeCloseTo(NIGHT_BRIGHTNESS, 10);
    expect(dawn[1]).toBeGreaterThan(dawn[0]!);
    expect(dawn[2]).toBeGreaterThan(dawn[1]!);
    expect(dawn[2]).toBeLessThan(1);
  });
});
