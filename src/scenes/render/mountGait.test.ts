import { describe, expect, it } from 'vitest';
import { mountGait } from './mountGait';

describe('Gangart beritten (US1)', () => {
  it('Stand hält den Schritt an', () => expect(mountGait(0, 5)).toEqual({ gait: 'schritt', moving: false }));
  it('langsam Schritt, mittel Trab, schnell Galopp, auch nach links', () => {
    expect(mountGait(1, 5).gait).toBe('schritt');
    expect(mountGait(-3, 5).gait).toBe('trab');
    expect(mountGait(5, 5).gait).toBe('galopp');
  });
  it('ohne Referenz Galopp', () => expect(mountGait(2, 0).gait).toBe('galopp'));
});
