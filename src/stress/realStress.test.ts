import { describe, expect, it } from 'vitest';
import type { RenderInfo } from '../core/renderInfo';
import type { DevMessage } from '../online/clientProtocol';
import { KNOWN } from './known';
import { figuresAt, parseScenarios, type Scenario } from './stressPlan';
import { RealStress, SPAWN_CHUNK, realStressId, spawnMessages, type RealHost } from './realStress';
import data from '../../data/stress.json';

const scenarios = parseScenarios(data as Record<string, unknown>, KNOWN);
const typ = scenarios.typisch!;
const worst = scenarios.worstCase!;

type Spawn = Extract<DevMessage, { action: 'spawn' }>;
const sum = (msgs: DevMessage[]): number => msgs.reduce((n, m) => n + (m as Spawn).amount, 0);

describe('Szenario-ID aus der URL', () => {
  it('nur mit ?dev=1 und ?stress=', () => {
    expect(realStressId('?dev=1&stress=typisch')).toBe('typisch');
    expect(realStressId('?stress=typisch')).toBeNull();
    expect(realStressId('?dev=1')).toBeNull();
    expect(realStressId('')).toBeNull();
  });
});

describe('Dev-Nachrichten je Stufe', () => {
  it('Stufe 1 setzt alle Figuren, Stufe 2 nur die fehlenden', () => {
    expect(sum(spawnMessages(typ, KNOWN, 1, [0]))).toBe(figuresAt(typ, KNOWN, 1, 1).length);
    const delta = figuresAt(typ, KNOWN, 2, 1).length - figuresAt(typ, KNOWN, 1, 1).length;
    expect(sum(spawnMessages(typ, KNOWN, 2, [0]))).toBe(delta);
  });

  it('zwei Spieler: je Slot die Hälfte, Slot-Nummern des Geräts', () => {
    const msgs = spawnMessages(worst, KNOWN, 1, [3, 5]) as Spawn[];
    expect(new Set(msgs.map((m) => m.slot))).toEqual(new Set([3, 5]));
    expect(sum(msgs)).toBe(60);
    const per = (slot: number) => sum(msgs.filter((m) => m.slot === slot));
    expect(Math.abs(per(3) - per(5))).toBeLessThanOrEqual(1);
  });

  it('jede Nachricht hat höchstens 200 Figuren, gültige Art und Sorte', () => {
    const big: Scenario = { ...typ, enemies: [{ type: 'greed', count: 450 }], troops: [], sprites: { mix: 'typical', distinctSheets: 5 } };
    const msgs = spawnMessages(big, KNOWN, 1, [0]) as Spawn[];
    expect(msgs.map((m) => m.amount)).toEqual([SPAWN_CHUNK, SPAWN_CHUNK, 50]);
    for (const m of spawnMessages(worst, KNOWN, 3, [0, 1]) as Spawn[]) {
      expect(m).toMatchObject({ t: 'dev', action: 'spawn' });
      expect(['enemy', 'troop']).toContain(m.kind);
      expect(KNOWN[m.kind === 'enemy' ? 'enemies' : 'troops']).toContain(m.unit);
    }
  });

  it('über alle Stufen kommt in Summe die Zahl der Figuren der letzten Stufe an', () => {
    let total = 0;
    for (let s = 1; s <= worst.ramp.steps; s++) total += sum(spawnMessages(worst, KNOWN, s, [0, 1]));
    expect(total).toBe(figuresAt(worst, KNOWN, worst.ramp.steps, 2).length);
  });
});

const INFO: RenderInfo = { kind: 'WebGL2', gpu: 'Mali', maxTexture: 4096, width: 1920, height: 1080, devicePixelRatio: 1, viewport: '1×1', textures: 1, textureMiB: 1, request: 'auto' };

function fakeHost() {
  const sent: DevMessage[] = [];
  const host = {
    game: { loop: { rawDelta: 20, actualFps: 50 }, renderer: { type: 1 }, textures: { list: {} } },
    events: { on: () => undefined, off: () => undefined, once: () => undefined },
    delayMs: 100,
    world: undefined as { enemies: unknown[]; troops: unknown[] } | undefined,
    client: { status: 'lobby', latency: { mean: 12.4 } as never, you: [] as { slot: number }[], sendDev: (m: DevMessage) => void sent.push(m) },
  };
  return { host: host as unknown as RealHost & typeof host, sent };
}

describe('Lauf im echten Client', () => {
  const tiny: Scenario = { ...typ, ramp: { everyMs: 4000, steps: 2, factor: 2 }, enemies: [{ type: 'greed', count: 2 }], troops: [], sprites: { mix: 'typical', distinctSheets: 5 } };

  function setup(search = '?dev=1&stress=typisch&depth=1&seed=stress') {
    const { host, sent } = fakeHost();
    const lines: { level: string; msg: string; ctx?: Record<string, unknown> }[] = [];
    const rs = new RealStress(host, tiny, 'typisch', search, () => INFO, (level, msg, ctx) => lines.push({ level, msg, ctx }));
    return { host, sent, lines, rs };
  }

  it('wartet auf Raum, Monarchen und Zustand', () => {
    const { host, sent, rs } = setup();
    rs.update();
    host.client.status = 'room';
    rs.update();
    host.client.you = [{ slot: 0 }];
    rs.update();
    expect(sent).toHaveLength(0);
  });

  it('beginnt mit Stufe 1, setzt die Figuren und schreibt Start- und Echt-Zeile mit runId', () => {
    const { host, sent, lines, rs } = setup();
    host.client.status = 'room';
    host.client.you = [{ slot: 0 }];
    host.world = { enemies: [], troops: [] };
    rs.update();
    expect(sent).toEqual([{ t: 'dev', action: 'spawn', slot: 0, kind: 'enemy', unit: 'greed', amount: 2 }]);
    expect(lines[0]!.msg).toContain('🚀 Stress typisch: 1 Spieler, Seed stress, 2 Stufen à 4 s');
    expect(lines[0]!.ctx).toMatchObject({ params: '?dev=1&stress=typisch&depth=1&seed=stress' });
    expect(lines[1]!.msg).toContain('Echter Client');
    expect(String(lines[0]!.ctx!.runId)).toMatch(/-real-typisch-p1-d1-/);
  });
});
