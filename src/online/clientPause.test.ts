import { describe, expect, it } from 'vitest';
import joined from '../../testdata/protocol/s2c-joined.json';
import welcome from '../../testdata/protocol/s2c-welcome.json';
import { RoomClient, type ClientEnv, type SocketLike } from './clientConnection';
import { pauseMessage } from './clientPause';

// BD7.3, AC-04: Das Pause-Menü pausiert das Gerät am Server (Protokoll v8, `pause`/`resume`).

describe('pauseMessage (reine Entscheidung)', () => {
  it('öffnen → pause, schließen → resume, unverändert → nichts', () => {
    expect(pauseMessage(true, false, true)).toBe('pause');
    expect(pauseMessage(false, true, true)).toBe('resume');
    expect(pauseMessage(true, true, true)).toBeNull();
    expect(pauseMessage(false, false, true)).toBeNull();
  });

  it('außerhalb des Raums nichts, auch beim Schließen', () => {
    expect(pauseMessage(true, false, false)).toBeNull();
    expect(pauseMessage(false, true, false)).toBeNull();
  });
});

class FakeSocket implements SocketLike {
  sent: { t: string }[] = [];
  onopen: (() => void) | null = null;
  onmessage: ((e: { data: unknown }) => void) | null = null;
  onclose: (() => void) | null = null;
  send(data: string): void {
    this.sent.push(JSON.parse(data) as { t: string });
  }
  close(): void {}
  recv(message: unknown): void {
    this.onmessage?.({ data: JSON.stringify(message) });
  }
}

/** Client im Raum; `timers` sammelt die Wiederverbinde-Zeitgeber, `last` ist der aktuelle Socket. */
function inRoom() {
  const socks: FakeSocket[] = [];
  const timers: (() => void)[] = [];
  const env: ClientEnv = {
    connect: () => {
      const s = new FakeSocket();
      socks.push(s);
      return s;
    },
    now: () => 0,
    setTimer: (fn) => timers.push(fn),
    clearTimer: () => {},
    deviceId: 'dev-1',
  };
  const client = new RoomClient(env);
  const last = () => socks[socks.length - 1]!;
  const enter = () => {
    last().onopen?.();
    last().recv(welcome);
    last().recv({ ...joined, you: [{ slot: 0, monarch: 0, depth: 0 }, { slot: 1, monarch: 1, depth: 0 }] });
  };
  enter();
  const pauses = () => socks.flatMap((s) => s.sent.filter((m) => m.t === 'pause' || m.t === 'resume').map((m) => m.t));
  return { client, last, timers, enter, pauses };
}

describe('RoomClient.setPaused', () => {
  it('Menü offen über viele Frames: genau ein pause, beim Schließen genau ein resume', () => {
    const t = inRoom();
    for (let i = 0; i < 30; i++) t.client.setPaused(true);
    for (let i = 0; i < 30; i++) t.client.setPaused(false);
    expect(t.pauses()).toEqual(['pause', 'resume']);
  });

  it('Abbruch mit offenem Menü: nichts senden, nach dem Wiederbeitritt erneut pause (B-535)', () => {
    const t = inRoom();
    t.client.setPaused(true);
    t.last().onclose?.();
    t.client.setPaused(true);
    t.client.setPaused(false); // geschlossen während der Trennung: der Server hat die Pause schon beendet
    t.client.setPaused(true);
    expect(t.pauses()).toEqual(['pause']);
    t.timers.shift()!();
    t.enter();
    t.client.setPaused(true);
    t.client.setPaused(false);
    expect(t.pauses()).toEqual(['pause', 'pause', 'resume']);
  });

  it('Menü bei der Trennung geschlossen: nach dem Wiederbeitritt kein resume', () => {
    const t = inRoom();
    t.client.setPaused(true);
    t.last().onclose?.();
    t.timers.shift()!();
    t.enter();
    t.client.setPaused(false);
    expect(t.pauses()).toEqual(['pause']);
  });

  it('leave() aus dem offenen Menü: leave statt resume', () => {
    const t = inRoom();
    t.client.setPaused(true);
    t.client.leave();
    t.client.setPaused(false);
    expect(t.pauses()).toEqual(['pause']);
    expect(t.last().sent.at(-1)).toEqual({ t: 'leave' });
  });
});
