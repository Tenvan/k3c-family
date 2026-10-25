import { describe, expect, it } from 'vitest';
import sample from '../../../testdata/protocol/s2c-snapshot-gesichert.json';
import type { GameEvent } from '../../model/types';
import { SAVED_SECONDS, stepSaved } from './savedHint';

const saved = sample.s.events as GameEvent[];

describe('stepSaved (B-186)', () => {
  it('zeigt „gesichert“ nach dem Ereignis saved aus dem Beispiel', () => {
    expect(stepSaved(0, saved, 0.016)).toBe(SAVED_SECONDS);
  });

  it('läuft ohne neues saved ab und bleibt bei 0', () => {
    let left = stepSaved(0, saved, 0);
    left = stepSaved(left, [], SAVED_SECONDS - 0.5);
    expect(left).toBeCloseTo(0.5);
    expect(stepSaved(left, [{ type: 'dusk' }], 1)).toBe(0);
  });

  it('zeigt ohne saved nichts', () => {
    expect(stepSaved(0, [{ type: 'dawn', day: 2 }], 0.016)).toBe(0);
  });
});
