import { describe, expect, it } from 'vitest';
import { MAX_ONLINE_PLAYERS, applySnapshot, sanitizeInput, snapshotWorld, type ServerMessage } from './protocol';
import { Room } from './room';
import { biomeForDepth } from '../model/biome';
import { createWorld } from '../world/sim/world';

function client() {
  const messages: ServerMessage[] = [];
  return { messages, conn: { send: (m: string) => void messages.push(JSON.parse(m) as ServerMessage) } };
}

describe('Online-Raum', () => {
  it('gibt jedem Client einen eigenen Monarchen, derselbe Client bekommt seinen wieder', () => {
    const room = new Room('t', { depth: 0, fast: false });
    const [a, b] = [client(), client()];
    expect(room.join('a', a.conn)).toBe(0);
    expect(room.join('b', b.conn)).toBe(1);
    room.leave('a', a.conn);
    const a2 = client();
    expect(room.join('a', a2.conn)).toBe(0);
    expect(room.world.players).toHaveLength(2);
  });

  it('ist bei MAX_ONLINE_PLAYERS voll', () => {
    const room = new Room('t', { depth: 0, fast: false });
    for (let i = 0; i < MAX_ONLINE_PLAYERS; i++) expect(room.join(`c${i}`, client().conn)).toBe(i);
    expect(room.join('extra', client().conn)).toBeNull();
  });

  it('Eingaben bewegen nur den eigenen Monarchen, Zustand geht an alle', () => {
    const room = new Room('t', { depth: 0, fast: false });
    const [a, b] = [client(), client()];
    room.join('a', a.conn);
    room.join('b', b.conn);
    const [x0, x1] = room.world.players.map((p) => p.x);
    room.input('a', { moveX: 1, sprint: false, pay: false });
    for (let i = 0; i < 30; i++) room.tick(1 / 30);
    expect(room.world.players[0].x).toBeGreaterThan(x0);
    expect(room.world.players[1].x).toBeCloseTo(x1);
    expect(a.messages.at(-1)?.t).toBe('state');
    expect(b.messages.at(-1)?.t).toBe('state');
  });

  it('pausiert ohne verbundene Spieler', () => {
    const room = new Room('t', { depth: 0, fast: false });
    const a = client();
    room.join('a', a.conn);
    room.leave('a', a.conn);
    const time = room.world.time;
    room.tick(1);
    expect(room.world.time).toBe(time);
  });

  it('ein Client-Spiegel folgt per Snapshot dem Server (JSON-Weg)', () => {
    const room = new Room('t', { depth: 0, fast: false });
    room.join('a', client().conn);
    room.input('a', { moveX: 1, sprint: true, pay: false });
    for (let i = 0; i < 60; i++) room.tick(1 / 30);
    const mirror = createWorld(biomeForDepth(0), room.seed);
    applySnapshot(mirror, JSON.parse(JSON.stringify(snapshotWorld(room.world))));
    expect(mirror.players[0].x).toBe(room.world.players[0].x);
    expect(mirror.troops).toHaveLength(room.world.troops.length);
    expect(mirror.level).toBe(mirror.level); // statischer Teil bleibt lokal
  });

  it('Stufenwechsel: alle am Tiefen-Eingang => neue Stufe, Snapshot meldet die Tiefe', () => {
    const room = new Room('t', { depth: 0, fast: false });
    const a = client();
    room.join('a', a.conn);
    const exit = room.world.level.entities.find((e) => e.kind === 'exit')!;
    room.world.players[0].x = exit.x;
    for (let i = 0; i < 30 * 12 && room.world.biome.depth === 0; i++) room.tick(1 / 30);
    expect(room.world.biome.depth).toBe(1);
    const last = a.messages.at(-1);
    expect(last?.t === 'state' && last.s.depth).toBe(1);
  });

  it('sanitizeInput begrenzt und normalisiert', () => {
    expect(sanitizeInput({ moveX: 99, sprint: true, pay: true })).toEqual({ moveX: 1, sprint: true, pay: true });
    expect(sanitizeInput({ moveX: NaN, sprint: 'ja' as never })).toEqual({ moveX: 0, sprint: false, pay: false });
  });
});
