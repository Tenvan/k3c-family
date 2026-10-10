import { describe, expect, it } from 'vitest';
import inputMsg from '../../testdata/protocol/c2s-input.json';
import learnMsg from '../../testdata/protocol/c2s-learn.json';
import respecMsg from '../../testdata/protocol/c2s-respec.json';
import deltaMsg from '../../testdata/protocol/s2c-snapshot-delta.json';
import snapMsg from '../../testdata/protocol/s2c-snapshot-full.json';
import joined from '../../testdata/protocol/s2c-joined.json';
import levelMsg from '../../testdata/protocol/s2c-level.json';
import rooms from '../../testdata/protocol/s2c-rooms.json';
import welcome from '../../testdata/protocol/s2c-welcome.json';
import { RoomClient, type SocketLike } from './clientConnection';
import { PROTOCOL_VERSION, type ClientMessage } from './clientProtocol';

// S2.1 (B-123): Schlag, Skills und Aktionen im Protokoll v4, Beispiele aus testdata/protocol/.

class FakeSocket implements SocketLike {
  sent: unknown[] = [];
  onopen: (() => void) | null = null;
  onmessage: ((e: { data: unknown }) => void) | null = null;
  onclose: (() => void) | null = null;
  send(data: string): void {
    this.sent.push(JSON.parse(data));
  }
  close(): void {}
  recv(message: unknown): void {
    this.onmessage?.({ data: JSON.stringify(message) });
  }
}

function lobby() {
  const sock = new FakeSocket();
  const client = new RoomClient({ connect: () => sock, now: () => 0, setTimer: () => 0, clearTimer: () => {}, deviceId: 'dev-1' });
  sock.onopen?.();
  sock.recv(welcome);
  sock.recv(rooms);
  return { client, sock };
}

function room() {
  const t = lobby();
  t.client.create('familie', true, 0, [0]);
  t.sock.recv(joined);
  t.sock.recv(levelMsg);
  t.sock.recv(snapMsg);
  return t;
}

describe('Protokoll v4: Skills und Aktionen', () => {
  it('hello trägt v 6 (K4.2), welcome aus dem Beispiel auch', () => {
    expect(PROTOCOL_VERSION).toBe(6);
    expect(lobby().sock.sent[0]).toEqual({ t: 'hello', v: 6, device: 'dev-1' });
    expect(welcome.v).toBe(PROTOCOL_VERSION);
  });

  it('learn und respec senden die Nachrichten der Beispiele, nur im Raum', () => {
    const l = lobby();
    l.client.learn(0, 'taunt');
    l.client.respec(0);
    expect(l.sock.sent).toHaveLength(1); // nur hello
    const r = room();
    r.client.learn(learnMsg.slot, learnMsg.skill);
    r.client.respec(respecMsg.slot);
    expect(r.sock.sent.slice(-2)).toEqual([learnMsg, respecMsg]);
  });

  it('snap und delta liefern je Spieler skills, points und actions', () => {
    const t = room();
    t.sock.recv(deltaMsg);
    const [snap, delta] = t.client.takeFrames();
    const p0 = snap?.state.players[0];
    expect(p0?.skills).toEqual(['taunt']);
    expect(p0?.points).toBe(1);
    expect(p0?.actions).toEqual([{ action: 'skill', slot: 1, skill: 'taunt' }, { action: 'learn' }]);
    expect(snap?.state.players[1]?.skills).toBeUndefined(); // fehlt = keine
    expect(delta?.state.players[0]?.cooldowns).toEqual([8]);
    expect(delta?.state.players[0]?.actions).toEqual([{ action: 'learn' }]);
  });

  it('c2s-input.json ist eine gültige ClientMessage mit attack und skill', () => {
    const msg: ClientMessage = { ...inputMsg, t: 'input' }; // ohne Cast: der Typ prüft p[] samt attack und skill
    expect(msg.t === 'input' && msg.p.map((p) => [p.attack, p.skill])).toEqual([
      [true, 0],
      [false, 2],
    ]);
  });
});
