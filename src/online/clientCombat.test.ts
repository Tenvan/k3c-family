import { describe, expect, it } from 'vitest';
import levelMsg from '../../testdata/protocol/s2c-level.json';
import boss from '../../testdata/protocol/s2c-snapshot-boss.json';
import event from '../../testdata/protocol/s2c-snapshot-event.json';
import type { BiomeConfig } from '../model/biome';
import type { LevelLayout } from '../model/types';
import { applyDelta } from './clientDelta';
import type { WorldState } from './clientProtocol';
import { createViewWorld } from './clientWorld';

// K4.1 (B-154/AC-01): Boss, Phase, Warnkreis, Event und Inselwechsel aus den Beispielen landen im World des Clients.

const level = { depth: 0, layout: levelMsg.layout as unknown as LevelLayout, biome: {} as BiomeConfig };
const world = (s: Record<string, unknown>) => createViewWorld(level, s as unknown as WorldState);

describe('Kampf im Zustand', () => {
  it('snap einer Bosswelle nennt HP, Phase und Warnkreis', () => {
    const e = world(boss.s).enemies[0];
    expect(e).toMatchObject({ boss: true, hp: 4200, maxHp: 6000, phase: 2, warn: { x: 388.4, r: 5, in: 3.4 } });
  });

  it('snap einer Eventnacht nennt Event mit Restzeit und Inselwechsel', () => {
    const w = world(event.s);
    expect(w.nightEvents).toEqual([{ id: 'fullMoon', secondsLeft: 95.5 }]);
    expect(w.islandSwitch).toEqual({ open: true, progress: 0.4, ready: false });
  });

  it('delta ändert die Felder, unset entfernt sie', () => {
    const s = event.s as unknown as Record<string, unknown>;
    const next = applyDelta(s, { nightEvents: [{ id: 'fullMoon', secondsLeft: 95.4 }], islandSwitch: { open: true, progress: 1, ready: true } });
    expect(world(next).nightEvents?.[0].secondsLeft).toBe(95.4);
    expect(world(next).islandSwitch?.ready).toBe(true);
    const gone = world(applyDelta(next, { unset: ['nightEvents', 'islandSwitch'] }));
    expect(gone.nightEvents).toBeUndefined();
    expect(gone.islandSwitch).toBeUndefined();

    const hit = applyDelta(boss.s as unknown as Record<string, unknown>, { enemies: { set: [{ ...boss.s.enemies[0], hp: 4100 }], del: [] } });
    expect(world(hit).enemies[0]).toMatchObject({ hp: 4100, phase: 2 });
  });

  it('ein Zustand ohne die Felder (älterer Server) bleibt gültig', () => {
    const { nightEvents: _n, islandSwitch: _i, ...old } = event.s;
    const w = world(old);
    expect(w.nightEvents).toBeUndefined();
    expect(w.islandSwitch).toBeUndefined();
    expect(w.enemies).toEqual([]);
  });
});
