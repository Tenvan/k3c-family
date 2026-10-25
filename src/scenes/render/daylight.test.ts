import { describe, expect, it } from 'vitest';
import { daylight } from './viewRules';
import type { CycleInfo } from '../../model/types';

const at = (phase: string, progress: number) => ({ phase, progress, day: 1, secondsLeft: 0 }) as unknown as CycleInfo;

// B-470 (W12): Morgengrauen ist eine eigene Phase mit linearem Aufhellen; die Nacht bleibt über die ganze Phase dunkel.
describe('daylight', () => {
  it('Tag hell, Dämmerung dunkelt ab', () => {
    expect(daylight(at('day', 0.5))).toBe(1);
    expect(daylight(at('dusk', 0.25))).toBe(0.75);
  });

  it('Nacht bleibt dunkel, auch am Ende', () => {
    expect(daylight(at('night', 0.95))).toBe(0);
  });

  it('Morgengrauen hellt linear auf', () => {
    expect(daylight(at('dawn', 0))).toBe(0);
    expect(daylight(at('dawn', 0.5))).toBe(0.5);
    expect(daylight(at('dawn', 1))).toBe(1);
  });

  it('unbekannte Phase gilt als Nacht', () => {
    expect(daylight(at('eclipse', 0.95))).toBe(0);
  });
});
