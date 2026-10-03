import { describe, expect, it } from 'vitest';
import devGold from '../../testdata/protocol/c2s-dev-gold.json';
import devMaterial from '../../testdata/protocol/c2s-dev-material.json';
import devTimescale from '../../testdata/protocol/c2s-dev-timescale.json';
import forbidden from '../../testdata/protocol/s2c-error-forbidden.json';
import joined from '../../testdata/protocol/s2c-joined.json';
import levelMsg from '../../testdata/protocol/s2c-level.json';
import rooms from '../../testdata/protocol/s2c-rooms.json';
import snapMsg from '../../testdata/protocol/s2c-snapshot-full.json';
import welcome from '../../testdata/protocol/s2c-welcome.json';
import { RoomClient, type ClientEnv, type SocketLike } from './clientConnection';
import type { ClientMessage, ServerMessage } from './clientProtocol';

// Dev-Aktionen (B-178, DBG1.1): Beispiele aus testdata/protocol/ passen zu den Typen; forbidden lässt Raum und
// Verbindung stehen. Hilfen nach dem Vorbild von clientConnection.test.ts.

class FakeSocket implements SocketLike {
  sent: Record<string, unknown>[] = [];
  closed = false;
  onopen: (() => void) | null = null;
  onmessage: ((e: { data: unknown }) => void) | null = null;
  onclose: (() => void) | null = null;
  send(data: string): void {
    this.sent.push(JSON.parse(data) as Record<string, unknown>);
  }
  close(): void {
    this.closed = true;
  }
  recv(message: unknown): void {
    this.onmessage?.({ data: JSON.stringify(message) });
  }
}

function inRoom() {
  const sock = new FakeSocket();
  const env: ClientEnv = {
    connect: () => sock,
    now: () => 0,
    setTimer: () => 1,
    clearTimer: () => {},
    deviceId: 'dev-1',
  };
  const client = new RoomClient(env);
  sock.onopen?.();
  sock.recv(welcome);
  sock.recv(rooms);
  client.create('familie', true, 0, [0]);
  sock.recv({ ...joined, you: [{ slot: 0, monarch: 0, depth: 0 }] });
  sock.recv(levelMsg);
  sock.recv(snapMsg);
  return { client, sock };
}

describe('Dev-Aktionen im Protokoll (B-178)', () => {
  it('die drei dev-Beispiele sind ClientMessage', () => {
    const msgs = [devGold, devMaterial, devTimescale] as ClientMessage[];
    expect(msgs.map((m) => m.t)).toEqual(['dev', 'dev', 'dev']);
    const gold = { t: 'dev', action: 'gold', slot: 0, amount: 50 } satisfies ClientMessage;
    const material = { t: 'dev', action: 'material', slot: 0, resource: 'wood', amount: 100 } satisfies ClientMessage;
    const timescale = { t: 'dev', action: 'timescale', factor: 4 } satisfies ClientMessage;
    expect([gold, material, timescale]).toEqual([devGold, devMaterial, devTimescale]);
  });

  it('s2c-error-forbidden ist ein Fehler mit Code forbidden', () => {
    const msg = forbidden as ServerMessage;
    expect(msg).toEqual({ t: 'error', code: 'forbidden', message: 'Nur im Dev-Mode erlaubt' });
  });

  it('forbidden im Raum: Hinweis, Raum und Verbindung bleiben', () => {
    const { client, sock } = inRoom();
    sock.recv(forbidden);
    expect(client.status).toBe('room');
    expect(client.roomCode).toBe('KRNZ');
    expect(client.notice).toBe('Nur im Dev-Mode erlaubt');
    expect(client.errorCode).toBe('forbidden');
    expect(sock.closed).toBe(false);
  });
});
