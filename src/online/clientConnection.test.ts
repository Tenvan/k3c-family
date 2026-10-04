import { afterEach, describe, expect, it, vi } from 'vitest';
import deltaMsg from '../../testdata/protocol/s2c-snapshot-delta.json';
import snapMsg from '../../testdata/protocol/s2c-snapshot-full.json';
import joined from '../../testdata/protocol/s2c-joined.json';
import levelMsg from '../../testdata/protocol/s2c-level.json';
import rooms from '../../testdata/protocol/s2c-rooms.json';
import welcome from '../../testdata/protocol/s2c-welcome.json';
import { RECONNECT_LIMIT_MS, RoomClient, getDeviceId, type ClientEnv, type SocketLike } from './clientConnection';
import { PROTOCOL_VERSION, type SlotInput } from './clientProtocol';

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
  open(): void {
    this.onopen?.();
  }
  recv(message: unknown): void {
    this.onmessage?.({ data: JSON.stringify(message) });
  }
  drop(): void {
    this.onclose?.();
  }
}

function setup() {
  const socks: FakeSocket[] = [];
  const timers: { at: number; fn: () => void; id: number }[] = [];
  let now = 0;
  let nextId = 1;
  const env: ClientEnv = {
    connect: () => {
      const s = new FakeSocket();
      socks.push(s);
      return s;
    },
    now: () => now,
    setTimer: (fn, ms) => {
      timers.push({ at: now + ms, fn, id: nextId });
      return nextId++;
    },
    clearTimer: (h) => void timers.splice(0, timers.length, ...timers.filter((t) => t.id !== h)),
    deviceId: 'dev-1',
  };
  const client = new RoomClient(env);
  const advance = (ms: number) => {
    const end = now + ms;
    for (;;) {
      const next = timers.filter((t) => t.at <= end).sort((a, b) => a.at - b.at)[0];
      if (!next) break;
      timers.splice(timers.indexOf(next), 1);
      now = next.at;
      next.fn();
    }
    now = end;
  };
  const last = () => socks[socks.length - 1]!;
  return { client, socks, advance, last, time: () => now };
}

/** Verbunden, in der Lobby */
function inLobby() {
  const t = setup();
  t.last().open();
  t.last().recv(welcome);
  t.last().recv(rooms);
  return t;
}

/** Im Raum KRNZ mit Level und vollem Zustand */
function inRoom(slots = [0]) {
  const t = inLobby();
  t.client.create('familie', true, 0, slots);
  t.last().recv({ ...joined, you: slots.map((slot) => ({ slot, monarch: slot, depth: 0 })) });
  t.last().recv(levelMsg);
  t.last().recv(snapMsg);
  return t;
}

const input = (moveX: number): SlotInput[] => [{ slot: 0, moveX, sprint: false, pay: false }];

afterEach(() => vi.restoreAllMocks());

describe('Handschlag und Geräte-ID (AC-06)', () => {
  it('hello mit v 2 und Geräte-ID ist die erste Nachricht, vorher wird nichts gesendet', () => {
    const t = setup();
    expect(t.last().sent).toEqual([]);
    t.last().open();
    expect(t.last().sent).toEqual([{ t: 'hello', v: PROTOCOL_VERSION, device: 'dev-1' }]);
    t.last().recv(welcome);
    expect(t.client.status).toBe('lobby');
    expect(t.client.tickHz).toBe(30);
    expect(t.client.limits?.rooms).toBe(4);
  });

  it('die Geräte-ID bleibt im Speicher erhalten', () => {
    const data = new Map<string, string>();
    const storage = { getItem: (k: string) => data.get(k) ?? null, setItem: (k: string, v: string) => void data.set(k, v) };
    const id = getDeviceId(storage);
    expect(id).toMatch(/^[0-9a-f]{32}$/);
    expect(getDeviceId(storage)).toBe(id);
    expect(data.get('k3c-client')).toBe(id);
  });

  it('ohne oder mit kaputtem Speicher gibt es eine Zufalls-ID, zu lange IDs werden ersetzt', () => {
    expect(getDeviceId(null)).toMatch(/^[0-9a-f]{32}$/);
    expect(getDeviceId(undefined)).not.toBe(getDeviceId(undefined));
    const broken = {
      getItem: () => {
        throw new Error('gesperrt');
      },
      setItem: () => undefined,
    };
    expect(getDeviceId(broken)).toMatch(/^[0-9a-f]{32}$/);
    const stored = { getItem: () => 'x'.repeat(65), setItem: vi.fn() };
    expect(getDeviceId(stored)).toHaveLength(32);
    expect(stored.setItem).toHaveBeenCalled();
  });
});

describe('Eingabe-Takt (AC-07)', () => {
  it('sendet eine Änderung sofort, auch kurz nach der letzten; unverändert nur als Lebenszeichen nach 500 ms; seq ab 1 (B-277/AC-04)', () => {
    const t = inRoom();
    const inputs = () => t.last().sent.filter((m) => m.t === 'input');
    t.client.sendInput(input(1));
    expect(inputs()).toHaveLength(1);
    expect(inputs()[0]).toMatchObject({ seq: 1, p: input(1) });
    t.advance(10);
    t.client.sendInput(input(-1)); // Änderung 10 ms nach der letzten: sofort
    expect(inputs()).toHaveLength(2);
    expect(inputs()[1]).toMatchObject({ seq: 2, p: input(-1) });
    t.advance(1);
    t.client.sendInput(input(1)); // Änderung im selben Bild (1 ms): Drossel INPUT_MIN_GAP_MS
    expect(inputs()).toHaveLength(2);
    t.advance(10);
    t.client.sendInput(input(1));
    expect(inputs()).toHaveLength(3);
    t.advance(400);
    t.client.sendInput(input(1)); // unverändert, noch keine 500 ms
    expect(inputs()).toHaveLength(3);
    t.advance(100);
    t.client.sendInput(input(1)); // Lebenszeichen
    expect(inputs()).toHaveLength(4);
    expect(inputs()[3]).toMatchObject({ seq: 4 });
  });

  it('Latenz: Zeit bis zum ersten Zustand mit ack ≥ seq, ohne Messung null (B-181)', () => {
    const t = inRoom();
    expect(t.client.latency).toBeNull();
    t.client.sendInput(input(1));
    t.advance(40);
    t.last().recv({ ...snapMsg, ack: 1 });
    expect(t.client.latency).toEqual({ mean: 40, p95: 40 });
  });

  it('seq beginnt nach dem Wiederverbinden neu bei 1', () => {
    const t = inRoom();
    t.client.sendInput(input(1));
    t.last().drop();
    t.advance(500);
    t.last().open();
    t.last().recv(welcome);
    t.last().recv({ ...joined });
    t.client.sendInput(input(1));
    expect(t.last().sent.find((m) => m.t === 'input')).toMatchObject({ seq: 1 });
  });
});

describe('Level und Zustand (AC-08)', () => {
  it('level liefert Layout und Biom aus data/, snap und delta liefern je Tick den vollen Zustand', () => {
    const t = inRoom();
    expect(t.client.level?.biome.id).toBe('forest');
    expect(t.client.level?.layout.widthUnits).toBe(1000);
    const [first] = t.client.takeFrames();
    expect(first?.tick).toBe(299);
    expect(first?.ack).toBe(41);
    expect(first?.state.players).toHaveLength(3);
    t.last().recv(deltaMsg);
    const [second] = t.client.takeFrames();
    expect(second?.tick).toBe(300);
    expect(second?.state.players[0]?.x).toBeCloseTo(546.4546340117788);
    expect(second?.state.players[1]?.x).toBeCloseTo(413.9816587787841);
    expect(second?.state.hubX).toBe(500);
    expect(second?.state.events.map((e) => e.type)).toEqual(['hit', 'coinPickup']); // F4/AC-02: Ereignisse aus dem Delta
    expect(first?.state.events).toEqual([]);
    expect(first?.state.players[0]?.x).toBeCloseTo(546.2879673451122); // früherer Frame bleibt unverändert
    expect(t.client.takeFrames()).toEqual([]);
  });

  it('ein neues level beginnt den Zustand neu: delta wird bis zum nächsten snap verworfen', () => {
    vi.spyOn(console, 'warn').mockImplementation(() => undefined);
    const t = inRoom();
    t.client.takeFrames();
    t.last().recv(levelMsg);
    t.last().recv(deltaMsg);
    expect(t.client.takeFrames()).toEqual([]);
    t.last().recv(snapMsg);
    t.last().recv(deltaMsg);
    expect(t.client.takeFrames().map((f) => f.tick)).toEqual([299, 300]);
  });
});

describe('Fehler-Codes (AC-09)', () => {
  const err = (code: string, message = `Meldung ${code}`) => ({ t: 'error', code, message });

  it.each(['room_full', 'too_many_slots', 'too_many_rooms', 'save_exists', 'save_not_found', 'room_not_found'])(
    '%s in der Lobby: Hinweis, bleibt bei der Raumliste',
    (code) => {
      const t = inLobby();
      t.last().recv(err(code));
      expect(t.client.status).toBe('lobby');
      expect(t.client.notice).toBe(`Meldung ${code}`);
      expect(t.client.rooms).toHaveLength(2);
    },
  );

  it.each(['room_full', 'too_many_slots'])('%s im Raum: Hinweis, der Raum bleibt', (code) => {
    const t = inRoom();
    t.last().recv(err(code));
    expect(t.client.status).toBe('room');
    expect(t.client.roomCode).toBe('KRNZ');
    expect(t.client.notice).toBe(`Meldung ${code}`);
  });

  it('room_closed führt zur Raumliste', () => {
    const t = inRoom();
    t.last().recv(err('room_closed', 'Raum ist geschlossen'));
    expect(t.client.status).toBe('lobby');
    expect(t.client.roomCode).toBeNull();
    expect(t.client.level).toBeNull();
    expect(t.client.notice).toBe('Raum ist geschlossen');
  });

  it('replaced zeigt „An anderer Stelle geöffnet“ und verbindet nicht neu', () => {
    const t = inRoom();
    t.last().recv(err('replaced'));
    expect(t.client.status).toBe('ended');
    expect(t.client.notice).toBe('An anderer Stelle geöffnet');
    expect(t.last().closed).toBe(true);
    t.last().drop();
    t.advance(RECONNECT_LIMIT_MS);
    expect(t.socks).toHaveLength(1);
  });

  it('version zeigt „Seite neu laden“ und verbindet nicht neu', () => {
    const t = setup();
    t.last().open();
    t.last().recv(err('version'));
    expect(t.client.status).toBe('ended');
    expect(t.client.notice).toContain('Seite neu laden');
    t.last().drop();
    t.advance(RECONNECT_LIMIT_MS);
    expect(t.socks).toHaveLength(1);
  });

  it('bad_request wird protokolliert und ändert den Zustand nicht', () => {
    const warn = vi.spyOn(console, 'warn').mockImplementation(() => undefined);
    const t = inRoom();
    t.last().recv(err('bad_request', 'kaputt'));
    expect(warn).toHaveBeenCalledWith('bad_request: kaputt');
    expect(t.client.status).toBe('room');
    expect(t.client.notice).toBeNull();
  });
});

describe('Wiederverbinden (AC-10)', () => {
  it('verbindet nach 0,5 s mit derselben Geräte-ID, demselben Code und denselben Slots neu', () => {
    const t = inRoom([0, 1]);
    t.last().drop();
    expect(t.client.status).toBe('reconnecting');
    expect(t.client.notice).toContain('verbinde neu');
    t.advance(499);
    expect(t.socks).toHaveLength(1);
    t.advance(1);
    expect(t.socks).toHaveLength(2);
    t.last().open();
    expect(t.last().sent[0]).toEqual({ t: 'hello', v: PROTOCOL_VERSION, device: 'dev-1' });
    t.last().recv(welcome);
    expect(t.last().sent[1]).toEqual({ t: 'join', room: 'KRNZ', slots: [0, 1] });
    t.last().recv({ ...joined, you: [{ slot: 0, monarch: 0, depth: 0 }, { slot: 1, monarch: 1, depth: 0 }] });
    expect(t.client.status).toBe('room');
    expect(t.client.notice).toBeNull();
  });

  it('wartet 0,5 s, 1 s, 2 s, dann 4 s und gibt nach 120 s mit „Server nicht erreichbar“ auf', () => {
    const t = inRoom();
    t.last().drop();
    const starts: number[] = [];
    while (t.client.status === 'reconnecting' && starts.length < 100) {
      const before = t.socks.length;
      t.advance(250);
      if (t.socks.length > before) {
        starts.push(t.time());
        t.last().drop(); // Server weiter weg
      }
    }
    expect(starts.slice(0, 4)).toEqual([500, 1500, 3500, 7500]);
    expect(t.client.status).toBe('lost');
    expect(t.client.notice).toBe('Server nicht erreichbar');
    expect(t.client.roomCode).toBeNull();
    expect(t.time()).toBeGreaterThanOrEqual(RECONNECT_LIMIT_MS);
    expect(t.time()).toBeLessThan(RECONNECT_LIMIT_MS + 5000);
    const count = t.socks.length;
    t.client.retry();
    expect(t.client.status).toBe('connecting');
    expect(t.socks).toHaveLength(count + 1);
  });

  it.each(['room_not_found', 'room_closed'])('%s nach dem Neuverbinden führt zur Raumliste', (code) => {
    const t = inRoom();
    t.last().drop();
    t.advance(500);
    t.last().open();
    t.last().recv(welcome);
    t.last().recv({ t: 'error', code, message: 'weg' });
    expect(t.client.status).toBe('lobby');
    expect(t.client.roomCode).toBeNull();
    expect(t.client.notice).toBe('weg');
  });

  it('nach leave() wird kein Raum mehr betreten', () => {
    const t = inRoom();
    t.client.leave();
    expect(t.last().sent.at(-1)).toEqual({ t: 'leave' });
    expect(t.client.status).toBe('lobby');
    t.last().drop();
    t.advance(500);
    t.last().open();
    t.last().recv(welcome);
    expect(t.last().sent.some((m) => m.t === 'join')).toBe(false);
    expect(t.client.status).toBe('lobby');
  });
});

describe('Befehle', () => {
  it('create und join gehen nur aus der Lobby, addSlot und removeSlot nur im Raum', () => {
    const t = inLobby();
    t.client.addSlot(1);
    expect(t.last().sent).toHaveLength(1); // nur hello
    t.client.create('familie', true, 0, [0]);
    expect(t.last().sent.at(-1)).toEqual({ t: 'create', save: 'familie', fresh: true, depth: 0, slots: [0] });
    t.client.join('KRNZ', [0, 1]);
    expect(t.last().sent.at(-1)).toEqual({ t: 'join', room: 'KRNZ', slots: [0, 1] });
    t.last().recv(joined);
    t.client.join('BWTQ', [0]); // im Raum: wird nicht gesendet (der Server würde bad_request antworten)
    t.client.create('x', true, 0, [0]);
    expect(t.last().sent).toHaveLength(3); // hello, create, join
  });

  it('seats aktualisiert Zuordnung und Monarchen-Zustände', () => {
    const t = inRoom();
    t.last().recv({ t: 'seats', you: [{ slot: 0, monarch: 0, depth: 0 }], monarchs: ['taken', 'free', 'taken'] });
    expect(t.client.monarchs).toEqual(['taken', 'free', 'taken']);
    t.client.addSlot(1);
    t.client.removeSlot(1);
    expect(t.last().sent.slice(-2)).toEqual([{ t: 'addSlot', slot: 1 }, { t: 'removeSlot', slot: 1 }]);
  });
});

describe('Dev-Aktionen (B-179)', () => {
  it('sendDev geht nur im Raum raus', () => {
    const lobby = inLobby();
    lobby.client.sendDev({ t: 'dev', action: 'timescale', factor: 8 });
    expect(lobby.last().sent).toHaveLength(1); // nur hello
    const t = inRoom();
    t.client.sendDev({ t: 'dev', action: 'gold', slot: 0, amount: 50 });
    expect(t.last().sent.at(-1)).toEqual({ t: 'dev', action: 'gold', slot: 0, amount: 50 });
  });
});

describe('Snapshot-Zeiten für das Debug-Overlay (B-093)', () => {
  it('ohne Snapshot: Zeit und Takt null, Geräte-ID lesbar', () => {
    const t = inLobby();
    expect(t.client.lastSnapshotAt).toBeNull();
    expect(t.client.snapshotHz).toBeNull();
    expect(t.client.deviceId).toBe('dev-1');
  });

  it('ein Snapshot gibt die Zeit, erst zwei den Takt', () => {
    const t = inRoom();
    expect(t.client.lastSnapshotAt).toBe(t.time());
    expect(t.client.snapshotHz).toBeNull();
    t.advance(50);
    t.last().recv(deltaMsg);
    expect(t.client.lastSnapshotAt).toBe(t.time());
    expect(t.client.snapshotHz).toBeCloseTo(20);
  });

  it('offlineSince nur während des Wiederverbindens', () => {
    const t = inRoom();
    expect(t.client.offlineSince).toBeNull();
    t.advance(1000);
    t.last().drop();
    expect(t.client.offlineSince).toBe(1000);
  });
});
