import { describe, expect, it } from 'vitest';
import { applyDelta } from './clientDelta';

const base = {
  time: 1,
  travel: { progress: 0.5 },
  events: [{ type: 'x' }],
  players: [
    { id: 1, x: 10 },
    { id: 2, x: 20 },
  ],
};

describe('applyDelta', () => {
  it('ersetzt geänderte Einträge, hängt neue an und löscht entfernte', () => {
    const next = applyDelta(base, { players: { set: [{ id: 2, x: 21 }, { id: 3, x: 30 }], del: [1] } });
    expect(next.players).toEqual([{ id: 2, x: 21 }, { id: 3, x: 30 }]);
  });

  it('lässt fehlende Felder unverändert und ändert den alten Zustand nicht', () => {
    const next = applyDelta(base, { time: 2 });
    expect(next.time).toBe(2);
    expect(next.players).toBe(base.players);
    expect(base.time).toBe(1);
  });

  it('null ist ein Wert, Objekte ohne id kommen ganz', () => {
    const next = applyDelta(base, { travel: null });
    expect(next.travel).toBeNull();
    expect(applyDelta(base, { travel: { progress: 1 } }).travel).toEqual({ progress: 1 });
  });

  it('events fehlt: leer; events vorhanden: ersetzt', () => {
    expect(applyDelta(base, { time: 2 }).events).toEqual([]);
    expect(applyDelta(base, { events: [{ type: 'y' }] }).events).toEqual([{ type: 'y' }]);
  });

  it('ein Eintrag, der im selben Delta gesetzt und gelöscht wird, bleibt gelöscht', () => {
    expect(applyDelta(base, { players: { set: [{ id: 1, x: 99 }], del: [1] } }).players).toEqual([{ id: 2, x: 20 }]);
  });

  it('Liste fehlt im alten Zustand: wird aus set aufgebaut', () => {
    expect(applyDelta({}, { coins: { set: [{ id: 5 }], del: [] } }).coins).toEqual([{ id: 5 }]);
  });
});
