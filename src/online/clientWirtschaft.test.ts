import { describe, expect, it } from 'vitest';
import levelMsg from '../../testdata/protocol/s2c-level.json';
import wirtschaft from '../../testdata/protocol/s2c-snapshot-wirtschaft.json';
import type { BiomeConfig } from '../model/biome';
import type { GameEvent, LevelLayout } from '../model/types';
import { applyDelta } from './clientDelta';
import type { WorldState } from './clientProtocol';
import { createViewWorld } from './clientWorld';

// W5.1 (B-153/AC-01, AC-03, AC-06): Wirtschaftsfelder und neue Ereignisse aus dem Beispiel landen im World des Clients.

const level = { depth: 0, layout: levelMsg.layout as unknown as LevelLayout, biome: {} as BiomeConfig };
const snap = wirtschaft.s as unknown as WorldState;
const world = (s: Record<string, unknown>) => createViewWorld(level, s as unknown as WorldState);

describe('Wirtschaft im Zustand', () => {
  it('snap liefert Hub-Stufe, Ausbau, Lager mit Maximum, Wartegrund, Händler und Beruf', () => {
    const w = createViewWorld(level, snap);
    expect(w.hubLevel).toBe(1);
    expect(w.hubUpgrade).toEqual({ gold: 50, material: { wood: 0, stone: 100, copper: 0 }, paid: 50, state: 'waitingMaterial' });
    expect(w.stockMax).toBe(1200);
    expect(w.stock).toEqual({ wood: 40, stone: 30, copper: 12 });
    expect(w.sites.some((s) => s.state === 'waitingMaterial')).toBe(true);
    expect(w.merchant).toEqual({ resource: 'stone', leaves: 3 });
    expect(w.troops.some((t) => t.profession === 'builder')).toBe(true);
    expect(w.danger).toBeUndefined();
    expect(w.fighters).toBe(2);
    expect(w.troopLimit).toBe(10);
  });

  it('delta ändert die Felder, unset entfernt den Händler, drops kommen als Liste mit id', () => {
    const next = applyDelta(snap as unknown as Record<string, unknown>, {
      hubUpgrade: { ...snap.hubUpgrade, state: 'waitingWorker' },
      danger: true,
      drops: { set: [{ id: 900, kind: 'archer', x: 431.2 }], del: [] },
      unset: ['merchant'],
    });
    const w = world(next);
    expect(w.hubUpgrade?.state).toBe('waitingWorker');
    expect(w.danger).toBe(true);
    expect(w.merchant).toBeUndefined();
    expect(w.drops).toEqual([{ id: 900, kind: 'archer', x: 431.2 }]);
    expect(applyDelta(next, { drops: { set: [], del: [900] } }).drops).toEqual([]);
  });

  it('ein Zustand ohne die Felder (älterer Server) bleibt gültig', () => {
    const { hubLevel: _h, hubUpgrade: _u, stockMax: _m, merchant: _r, fighters: _f, troopLimit: _l, ...old } = snap;
    const w = world(old);
    expect(w.hubLevel).toBeUndefined();
    expect(w.hubUpgrade).toBeUndefined();
    expect(w.stockMax).toBeUndefined();
    expect(w.fighters).toBeUndefined();
    expect(w.troopLimit).toBeUndefined();
    expect(w.stock).toEqual(snap.stock);
  });

  it('Ereignisse revived, disarmed und equipmentTaken parsen mit ihren Feldern', () => {
    const events = snap.events as GameEvent[];
    const byType = Object.fromEntries(events.map((e) => [e.type, e]));
    expect(byType.revived).toEqual({ type: 'revived', player: 1, x: 497.5, stage: 0 });
    expect(byType.disarmed).toEqual({ type: 'disarmed', kind: 'archer', x: 431.2, cause: 'goblin', stage: 0 });
    expect(byType.equipmentTaken).toEqual({ type: 'equipmentTaken', kind: 'archer', x: 428.9, stage: 0 });
  });
});
