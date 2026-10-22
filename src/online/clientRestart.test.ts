import { describe, expect, it } from 'vitest';
import snapMsg from '../../testdata/protocol/s2c-snapshot-full.json';
import joined from '../../testdata/protocol/s2c-joined.json';
import levelMsg from '../../testdata/protocol/s2c-level.json';
import welcome from '../../testdata/protocol/s2c-welcome.json';
import { RoomClient, type SocketLike } from './clientConnection';

class FakeSocket implements SocketLike {
  onopen: (() => void) | null = null;
  onmessage: ((e: { data: unknown }) => void) | null = null;
  onclose: (() => void) | null = null;
  sent: unknown[] = [];
  send(message: string): void {
    this.sent.push(JSON.parse(message));
  }
  close(): void {}
  recv(message: unknown): void {
    this.onmessage?.({ data: JSON.stringify(message) });
  }
}

const err = (code: string, message: string) => ({ t: 'error', code, message });

describe('Neustart des Servers (B-507)', () => {
  it('restarting zeigt „Server startet neu“ im Raum und behält den Hinweis bis zum nächsten welcome', () => {
    const socks: FakeSocket[] = [];
    let timer: (() => void) | null = null;
    const client = new RoomClient({
      connect: () => {
        socks.push(new FakeSocket());
        return socks[socks.length - 1]!;
      },
      now: () => 0,
      setTimer: (fn) => ((timer = fn), 1),
      clearTimer: () => undefined,
      deviceId: 'dev-1',
    });
    const last = () => socks[socks.length - 1]!;
    last().onopen?.();
    last().recv(welcome);
    client.create('familie', true, 0, [0]);
    last().recv({ ...joined, you: [{ slot: 0, monarch: 0, depth: 0 }] });
    last().recv(levelMsg);
    last().recv(snapMsg);

    last().recv(err('restarting', 'Server startet neu'));
    expect(client.status).toBe('room');
    expect(client.roomCode).toBe('KRNZ');
    expect(client.notice).toBe('Server startet neu');
    expect(client.errorCode).toBe('restarting');

    last().recv(err('room_closed', 'Raum ist geschlossen'));
    expect(client.roomCode).toBe('KRNZ'); // der Raum bleibt gemerkt (B-508)
    expect(client.notice).toBe('Server startet neu');

    last().onclose?.();
    expect(client.status).toBe('reconnecting');
    expect(client.notice).toBe('Server startet neu');

    (timer as (() => void) | null)?.();
    last().onopen?.();
    last().recv(welcome);
    // wieder verbunden: Beitritt zum alten Raum mit demselben Code, ohne Umweg über die Raumliste
    expect(last().sent.at(-1)).toMatchObject({ t: 'join', room: 'KRNZ', slots: [0] });
    last().recv({ ...joined, you: [{ slot: 0, monarch: 0, depth: 0 }] });
    expect(client.status).toBe('room');
    expect(client.roomCode).toBe('KRNZ');
  });

  it('ist der Raum nach dem Neustart nicht mehr da (room_not_found), landet das Gerät in der Raumliste', () => {
    const socks: FakeSocket[] = [];
    let timer: (() => void) | null = null;
    const client = new RoomClient({
      connect: () => {
        socks.push(new FakeSocket());
        return socks[socks.length - 1]!;
      },
      now: () => 0,
      setTimer: (fn) => ((timer = fn), 1),
      clearTimer: () => undefined,
      deviceId: 'dev-1',
    });
    const last = () => socks[socks.length - 1]!;
    last().onopen?.();
    last().recv(welcome);
    client.create('familie', true, 0, [0]);
    last().recv({ ...joined, you: [{ slot: 0, monarch: 0, depth: 0 }] });
    last().recv(err('restarting', 'Server startet neu'));
    last().recv(err('room_closed', 'Raum ist geschlossen'));
    last().onclose?.();
    (timer as (() => void) | null)?.();
    last().onopen?.();
    last().recv(welcome);
    last().recv(err('room_not_found', 'Raum nicht gefunden'));
    expect(client.status).toBe('lobby');
    expect(client.roomCode).toBeNull();
  });
});
