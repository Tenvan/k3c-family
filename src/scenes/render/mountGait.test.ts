import { describe, expect, it } from 'vitest';
import { mountGait, mountedAnim } from './mountGait';

describe('Gangart beritten (US1)', () => {
  it('Stand hält den Schritt an', () => expect(mountGait(0, 5)).toEqual({ gait: 'schritt', moving: false }));
  it('langsam Schritt, mittel Trab, schnell Galopp, auch nach links', () => {
    expect(mountGait(1, 5).gait).toBe('schritt');
    expect(mountGait(-3, 5).gait).toBe('trab');
    expect(mountGait(5, 5).gait).toBe('galopp');
  });
  it('ohne Referenz Galopp', () => expect(mountGait(2, 0).gait).toBe('galopp'));
});

describe('Reittier des Monarchen (US2/AC-02)', () => {
  const has = (names: string[]) => (n: string) => names.includes(n);

  it('Pferd nutzt die Gangart direkt, Variante je Spieler', () => {
    expect(mountedAnim('horse', 'trab', undefined, has(['trab']))).toEqual({ anim: 'trab', reittier: 'pferd', fallback: false });
    expect(mountedAnim('horse', 'trab', 'spieler2', has(['trab', 'trab@spieler2']))).toEqual({ anim: 'trab@spieler2', reittier: 'pferd', fallback: false });
  });

  it('anderes Reittier: Animation <gangart>-<reittier>, sonst Rückfall Pferd', () => {
    expect(mountedAnim('wolf', 'galopp', undefined, has(['galopp', 'galopp-wolf']))).toEqual({ anim: 'galopp-wolf', reittier: 'wolf', fallback: false });
    expect(mountedAnim('wolf', 'galopp', undefined, has(['galopp']))).toEqual({ anim: 'galopp', reittier: 'pferd', fallback: true });
    expect(mountedAnim('gibtsnicht', 'schritt', undefined, has(['schritt']))).toEqual({ anim: 'schritt', reittier: 'pferd', fallback: true });
    expect(mountedAnim('wolf', 'galopp', 'spieler2', has(['galopp', 'galopp@spieler2']))).toEqual({ anim: 'galopp@spieler2', reittier: 'pferd', fallback: true });
  });
});
