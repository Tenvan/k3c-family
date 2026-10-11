import { describe, expect, it } from 'vitest';
import devPause from '../../testdata/protocol/c2s-dev-pause.json';
import { DEV_ACTIONS, DEV_GROUPS, NO_TOGGLES, actionLabel, devMessage, nextToggles, pauseMessage, roomDevMode } from './debugActions';

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
      { t: 'dev', action: 'maxStock', amount: 100 },
      { t: 'dev', action: 'fillStock' },
      { t: 'dev', action: 'cheapBuild', on: true },
      { t: 'dev', action: 'buildAll', level: 1 },
      { t: 'dev', action: 'buildAll', level: 2 },
      { t: 'dev', action: 'buildAll', level: 3 },
      { t: 'dev', action: 'buildAll', level: 4 },
      { t: 'dev', action: 'buildAll', level: 5 },
      { t: 'dev', action: 'timescale', factor: 1 },
      { t: 'dev', action: 'timescale', factor: 2 },
      { t: 'dev', action: 'timescale', factor: 4 },
      { t: 'dev', action: 'timescale', factor: 8 },
      { t: 'dev', action: 'timescaleAuto', on: true },
      { t: 'dev', action: 'nextPhase' },
    ];
    expect(DEV_ACTIONS.map((a) => devMessage(a.key, 1))).toEqual(expected);
  });

  it('Beschriftungen wie im Ticket', () => {
    expect(DEV_ACTIONS.map((a) => a.label)).toEqual([
      'Gold 10', 'Gold 50', 'Gold 100',
      'Holz 50', 'Stein 50', 'Kupfer 50', 'Eisen 50', 'Kristall 50',
      'Max +100', 'Alle Ressourcen füllen', 'Baukosten 1',
      'Alles ausbauen: Stufe 1', 'Alles ausbauen: Stufe 2', 'Alles ausbauen: Stufe 3', 'Alles ausbauen: Stufe 4', 'Alles ausbauen: Stufe 5',
      'Zeit 1×', 'Zeit 2×', 'Zeit 4×', 'Zeit 8×',
      'Zeitraffer → 1× bei Tageszeitwechsel', 'Nächste Tageszeit',
    ]);
  });

  it('unbekannte Auswahl, Menge oder Faktor → null', () => {
    expect(devMessage('gold:7', 0)).toBeNull();
    expect(devMessage('timescale:3', 0)).toBeNull();
    expect(devMessage('material:gold', 0)).toBeNull();
    expect(devMessage('restart', 0)).toBeNull();
    expect(devMessage('', 0)).toBeNull();
    expect(devMessage('upgrade:6', 0)).toBeNull();
  });

  it('Umschalter senden den umgekehrten Zustand und merken ihn', () => {
    const on = nextToggles('buildcost1', NO_TOGGLES);
    expect(on).toEqual({ cheapBuild: true, timescaleAuto: false });
    expect(devMessage('buildcost1', 0, on)).toEqual({ t: 'dev', action: 'cheapBuild', on: false });
    expect(nextToggles('autoslow', on)).toEqual({ cheapBuild: true, timescaleAuto: true });
    expect(nextToggles('maxplus', on)).toBe(on);
    const toggle = DEV_ACTIONS.find((a) => a.key === 'autoslow')!;
    expect(actionLabel(toggle, on)).toBe('Zeitraffer → 1× bei Tageszeitwechsel: aus');
    expect(actionLabel(toggle, { ...on, timescaleAuto: true })).toBe('Zeitraffer → 1× bei Tageszeitwechsel: an');
  });

  it('Liste steht in der Reihenfolge der Gruppen (Auf/Ab folgt der Anzeige)', () => {
    const order = DEV_ACTIONS.map((a) => DEV_GROUPS.indexOf(a.group));
    expect(order).toEqual([...order].sort((x, y) => x - y));
    expect(order).not.toContain(-1);
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
