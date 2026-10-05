import { describe, expect, it, vi } from 'vitest';
import deltaMsg from '../../testdata/protocol/s2c-snapshot-delta.json';
import snapMsg from '../../testdata/protocol/s2c-snapshot-full.json';
import joined from '../../testdata/protocol/s2c-joined.json';
import levelMsg from '../../testdata/protocol/s2c-level.json';
import welcome from '../../testdata/protocol/s2c-welcome.json';
import { RoomClient, type SocketLike } from './clientConnection';
import { StageStreams, type StateMessage } from './clientStages';
import type { LevelLayout } from '../model/types';

const layout = levelMsg.layout as unknown as LevelLayout;
const snap = (stage: number, tick: number): StateMessage => ({ ...(snapMsg as unknown as StateMessage), stage, tick });
const delta = (stage: number, tick: number): StateMessage => ({ ...(deltaMsg as unknown as StateMessage), stage, tick });

describe('StageStreams (B-176/AC-01)', () => {
  it('zwei level/snap mit verschiedenen stage ergeben zwei Stufen, delta gilt nur für seine Stufe', () => {
    const s = new StageStreams();
    s.level(1, 1, layout);
    s.level(0, 0, layout);
    s.state(snap(0, 299), 0);
    s.state(snap(1, 299), 0);
    expect(s.stages()).toEqual([0, 1]);
    expect(s.levelOf(1)?.biome.id).toBe('forest');
    s.takeFramesOf(0);
    s.takeFramesOf(1);
    s.state(delta(1, 300), 1);
    expect(s.takeFramesOf(0)).toEqual([]);
    const [d] = s.takeFramesOf(1);
    expect(d?.tick).toBe(300);
    expect(d?.state.hubX).toBe(500); // aus dem delta auf den snap der Stufe 1 angewendet
  });

  it('delta ohne snap seiner Stufe wird verworfen, ein neues level beginnt den Strom neu', () => {
    vi.spyOn(console, 'warn').mockImplementation(() => undefined);
    const s = new StageStreams();
    s.level(0, 0, layout);
    s.state(snap(0, 299), 0);
    expect(s.state(delta(1, 300), 0)).toBeNull();
    s.level(0, 0, layout);
    expect(s.state(delta(0, 300), 0)).toBeNull();
    expect(s.takeFramesOf(0)).toEqual([]);
  });

  it('seats ohne Slot in Stufe 1 entfernt Stufe 1', () => {
    const s = new StageStreams();
    s.level(0, 0, layout);
    s.level(1, 1, layout);
    s.keep([{ slot: 0, monarch: 0, depth: 0, stage: 0 }]);
    expect(s.stages()).toEqual([0]);
    expect(s.levelOf(1)).toBeNull();
  });
});

/** Ein RoomClient mit Fake-Verbindung, im Raum mit den Slots 0 (Stufe 0) und 1 (Stufe 1). */
function twoStages() {
  const sock: SocketLike & { sent: Record<string, unknown>[]; recv(m: unknown): void } = {
    sent: [],
    send(data: string) {
      this.sent.push(JSON.parse(data) as Record<string, unknown>);
    },
    close() {},
    onopen: null,
    onmessage: null,
    onclose: null,
    recv(m: unknown) {
      this.onmessage?.({ data: JSON.stringify(m) });
    },
  };
  let now = 0;
  const client = new RoomClient({ connect: () => sock, now: () => now, setTimer: () => 0, clearTimer: () => undefined, deviceId: 'dev-1' });
  sock.onopen?.();
  sock.recv(welcome);
  const you = [
    { slot: 0, monarch: 0, depth: 1, stage: 1 },
    { slot: 1, monarch: 1, depth: 0, stage: 0 },
  ];
  sock.recv({ ...joined, you });
  sock.recv({ ...levelMsg, stage: 0, depth: 0 });
  sock.recv(snap(0, 299));
  sock.recv({ ...levelMsg, stage: 1, depth: 1 });
  sock.recv(snap(1, 299));
  sock.recv({ t: 'seats', you, monarchs: ['taken', 'taken'] });
  return { client, sock, you, tick: (ms: number) => (now += ms) };
}

describe('RoomClient mit mehreren Stufen (B-176/AC-01)', () => {
  it('level und takeFrames() liefern die Stufe des kleinsten Slots, takeFramesOf die übrigen', () => {
    const t = twoStages();
    expect(t.client.stages()).toEqual([0, 1]);
    expect(t.client.level?.depth).toBe(1); // Slot 0 steht in Stufe 1
    expect(t.client.levelOf(0)?.depth).toBe(0);
    expect(t.client.takeFrames().map((f) => f.tick)).toEqual([299]);
    t.sock.recv(delta(0, 300));
    t.sock.recv(delta(1, 300));
    expect(t.client.takeFrames().map((f) => f.tick)).toEqual([300]);
    expect(t.client.takeFramesOf(0).map((f) => f.tick)).toEqual([299, 300]);
    t.sock.recv({ t: 'seats', you: [t.you[1]], monarchs: ['free', 'taken'] }); // Slot 0 ist weg: nur Stufe 0 bleibt
    expect(t.client.stages()).toEqual([0]);
    expect(t.client.level?.depth).toBe(0);
  });

  it('Latenz: jede Stufe trägt ack, eine Eingabe zählt einmal; der Takt zählt einmal je Tick', () => {
    const t = twoStages();
    t.tick(1000);
    t.client.sendInput([{ slot: 0, moveX: 1, sprint: false, pay: false }]);
    t.tick(50);
    t.sock.recv({ ...delta(0, 300), ack: 1 });
    t.sock.recv({ ...delta(1, 300), ack: 1 });
    t.tick(33);
    t.sock.recv({ ...delta(0, 301), ack: 1 });
    t.sock.recv({ ...delta(1, 301), ack: 1 });
    expect(t.client.latency).toEqual({ mean: 50, p95: 50 });
    expect(t.client.snapshotHz).toBeCloseTo(2000 / 1083); // 3 Ticks (299, 300, 301, je einmal) über 1083 ms
  });
});
