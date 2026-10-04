import { describe, expect, it } from 'vitest';
import { UNIT_PX } from '../core/constants';
import type { GameEvent } from '../model/types';
import { effectFor, type BuildSpots } from './effects';

const none = () => undefined;
const fx = (e: GameEvent, spots: BuildSpots = new Map()) => effectFor(e, (p) => (p === 0 ? 5 : undefined), spots);

describe('effectFor (GR5.1 AC-01, AC-03)', () => {
  it('ordnet je Feedback-Event genau einen Effekt zu, Ort in Pixeln', () => {
    expect(fx({ type: 'hit', x: 3, target: 'enemy', id: 1, damage: 2 })).toMatchObject({ kind: 'hit', x: 3 * UNIT_PX });
    expect(fx({ type: 'kill', kind: 'gnoll', x: 4, gold: 1 })?.kind).toBe('kill');
    expect(fx({ type: 'coinPickup', player: 0, x: 2 })?.kind).toBe('coin');
    expect(fx({ type: 'coinGive', player: 0, x: 2, to: 'site' })?.kind).toBe('coinGive');
    expect(fx({ type: 'buildProgress', site: 1, kind: 'wall', x: 7, percent: 25 })?.kind).toBe('build');
    expect(fx({ type: 'playerDown', player: 0 })).toMatchObject({ kind: 'down', x: 5 * UNIT_PX });
  });

  it('built nutzt den Ort des letzten Baufortschritts derselben Bauart', () => {
    const spots: BuildSpots = new Map();
    expect(fx({ type: 'built', kind: 'wall' }, spots)).toBeNull();
    fx({ type: 'buildProgress', site: 1, kind: 'wall', x: 7, percent: 75 }, spots);
    expect(fx({ type: 'built', kind: 'wall' }, spots)).toMatchObject({ kind: 'built', x: 7 * UNIT_PX });
  });

  it('übergeht unbekannte Typen und unbekannte Spieler ohne Fehler', () => {
    expect(fx({ type: 'dusk' })).toBeNull();
    expect(fx({ type: 'zukunft', x: 1 } as unknown as GameEvent)).toBeNull();
    expect(effectFor({ type: 'playerDown', player: 3 }, none, new Map())).toBeNull();
  });

  it('verändert das Ereignis nicht', () => {
    const e: GameEvent = { type: 'hit', x: 3, target: 'enemy', id: 1, damage: 2 };
    const copy = structuredClone(e);
    fx(e);
    expect(e).toEqual(copy);
  });
});
