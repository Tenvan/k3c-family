import { describe, expect, it } from 'vitest';
import devPause from '../../testdata/protocol/c2s-dev-pause.json';
import { DEV_ACTIONS, devMessage, pauseMessage, roomDevMode } from './debugActions';

describe('devMessage (B-179/AC-01)', () => {
  it('Beispiele aus testdata/protocol/: Gold und Zeitfaktor wörtlich', () => {
    // c2s-dev-gold.json
    expect(devMessage('gold:50', 0)).toEqual({ t: 'dev', action: 'gold', slot: 0, amount: 50 });
    // c2s-dev-timescale.json
    expect(devMessage('timescale:4', 0)).toEqual({ t: 'dev', action: 'timescale', factor: 4 });
    // c2s-dev-material.json mit der Menge der Liste (50 statt 100)
    expect(devMessage('material:wood', 0)).toEqual({ t: 'dev', action: 'material', slot: 0, resource: 'wood', amount: 50 });
  });

  it('jede Aktion der Liste ergibt die erwartete Nachricht', () => {
    const expected = [
      { t: 'dev', action: 'gold', slot: 1, amount: 10 },
      { t: 'dev', action: 'gold', slot: 1, amount: 50 },
      { t: 'dev', action: 'gold', slot: 1, amount: 100 },
      { t: 'dev', action: 'material', slot: 1, resource: 'wood', amount: 50 },
      { t: 'dev', action: 'material', slot: 1, resource: 'stone', amount: 50 },
      { t: 'dev', action: 'material', slot: 1, resource: 'copper', amount: 50 },
      { t: 'dev', action: 'material', slot: 1, resource: 'iron', amount: 50 },
      { t: 'dev', action: 'material', slot: 1, resource: 'crystal', amount: 50 },
      { t: 'dev', action: 'timescale', factor: 1 },
      { t: 'dev', action: 'timescale', factor: 2 },
      { t: 'dev', action: 'timescale', factor: 4 },
      { t: 'dev', action: 'timescale', factor: 8 },
    ];
    expect(DEV_ACTIONS.map((a) => devMessage(a.key, 1))).toEqual(expected);
  });

  it('Beschriftungen wie im Ticket', () => {
    expect(DEV_ACTIONS.map((a) => a.label)).toEqual([
      'Gold 10', 'Gold 50', 'Gold 100',
      'Holz 50', 'Stein 50', 'Kupfer 50', 'Eisen 50', 'Kristall 50',
      'Zeit 1×', 'Zeit 2×', 'Zeit 4×', 'Zeit 8×',
    ]);
  });

  it('unbekannte Auswahl, Menge oder Faktor → null', () => {
    expect(devMessage('gold:7', 0)).toBeNull();
    expect(devMessage('timescale:3', 0)).toBeNull();
    expect(devMessage('material:gold', 0)).toBeNull();
    expect(devMessage('restart', 0)).toBeNull();
    expect(devMessage('', 0)).toBeNull();
  });
});

describe('roomDevMode und pauseMessage (B-231)', () => {
  it('pause hält an und lässt weiterlaufen (c2s-dev-pause.json)', () => {
    expect(pauseMessage(true)).toEqual(devPause);
    expect(pauseMessage(false)).toEqual({ t: 'dev', action: 'pause', paused: false });
  });

  it('Dev-Mode am Feld devTimescale (s2c-snapshot-delta-timescale.json), auch bei Faktor 1', () => {
    expect(roomDevMode({ devTimescale: 4 })).toBe(true);
    expect(roomDevMode({ devTimescale: 1 })).toBe(true);
    expect(roomDevMode({})).toBe(false);
    expect(roomDevMode(null)).toBe(false);
  });
});
