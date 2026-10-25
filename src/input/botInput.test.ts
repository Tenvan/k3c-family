import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import * as clientLogModule from '../core/clientLog';
import { BotFeed, BotInput, RETRY_MS, STALE_MS, botInputs, parseBotFeed, parseCommand, type FeedDeps, type FeedSocket } from './botInput';

/** Feed-Verbindungen ohne Netz: jede `open` legt einen Socket an, `later` sammelt die Wiederholungen. */
function fakeDeps() {
  const sockets: (FeedSocket & { url: string; closed: boolean })[] = [];
  const pending: (() => void)[] = [];
  const deps: FeedDeps = {
    open: (url) => {
      const s = { url, closed: false, onopen: null, onmessage: null, onclose: null, close: () => void (s.closed = true) } as (typeof sockets)[number];
      sockets.push(s);
      return s;
    },
    later: (fn, ms) => {
      expect(ms).toBe(RETRY_MS);
      pending.push(fn);
    },
  };
  return { deps, sockets, pending };
}

const msg = (c: object) => ({ data: JSON.stringify(c) });

describe('BotInput (B-349/AC-01)', () => {
  it('liefert je Slot die Aktionen aus den Feed-Nachrichten', () => {
    const { deps, sockets } = fakeDeps();
    const [a, b] = botInputs('?botfeed=ws://127.0.0.1:5180/bot/run-4/0&players=2', 'localhost', deps);
    sockets[0]!.onmessage!(msg({ slot: 0, moveX: -1, sprint: true, pay: false, attack: true, skill: 0 }));
    sockets[0]!.onmessage!(msg({ slot: 1, moveX: 0.5, sprint: false, pay: true, attack: false, skill: 3 }));
    a!.update();
    b!.update();
    expect([a!.moveX(), a!.sprint(), a!.held('attack'), a!.held('confirm')]).toEqual([-1, true, true, false]);
    expect([b!.moveX(), b!.sprint(), b!.held('confirm'), b!.held('skill3'), b!.held('skill1')]).toEqual([0.5, false, true, true, false]);
    expect(b!.justPressed('confirm')).toBe(true);
    b!.update(); // letzte Nachricht gilt bis zur nächsten, gedrückt ist sie nur im ersten Frame
    expect([b!.held('confirm'), b!.justPressed('confirm')]).toEqual([true, false]);
  });

  it('ohne Feed oder vor der ersten Nachricht keine Aktion', () => {
    const idle = new BotInput(0, { command: () => null, takePressed: () => new Set() });
    idle.update();
    expect([idle.moveX(), idle.sprint(), idle.held('confirm'), idle.justPressed('attack')]).toEqual([0, false, false, false]);
  });

  it('Feed bricht ab: keine Eingabe mehr, neuer Versuch nach RETRY_MS', () => {
    const { deps, sockets, pending } = fakeDeps();
    const feed = new BotFeed('ws://127.0.0.1:5180/bot/1/0', deps);
    const input = new BotInput(0, feed);
    sockets[0]!.onmessage!(msg({ slot: 0, moveX: 1 }));
    input.update();
    expect(input.moveX()).toBe(1);
    sockets[0]!.onclose!();
    input.update();
    expect(input.moveX()).toBe(0);
    expect(sockets).toHaveLength(1);
    pending.shift()!();
    expect(sockets).toHaveLength(2);
    sockets[1]!.onmessage!(msg({ slot: 0, moveX: -0.25 }));
    input.update();
    expect(input.moveX()).toBe(-0.25);
    feed.stop();
    sockets[1]!.onclose!();
    expect(pending).toHaveLength(0);
  });

  it('verwirft ungültige Nachrichten und begrenzt moveX', () => {
    expect(parseCommand('kein json')).toBeNull();
    expect(parseCommand(JSON.stringify({ slot: 4 }))).toBeNull();
    expect(parseCommand(JSON.stringify({ slot: 'x' }))).toBeNull();
    expect(parseCommand(JSON.stringify({ slot: 2, moveX: 7, skill: 9, pay: 'ja' }))).toEqual({ slot: 2, moveX: 1, sprint: false, pay: false, attack: false, skill: 0 });
  });
});

describe('BotFeed: kurze Drücke und stummer Feed (B-354)', () => {
  beforeEach(() => void vi.useFakeTimers());
  afterEach(() => {
    vi.useRealTimers();
    vi.restoreAllMocks();
  });

  it('AC-01: ein Druck zwischen zwei update() zählt im nächsten Frame einmal', () => {
    const { deps, sockets } = fakeDeps();
    const input = new BotInput(0, new BotFeed('ws://127.0.0.1:5180/bot/1/0', deps));
    input.update();
    sockets[0]!.onmessage!(msg({ slot: 0, attack: true, skill: 2 }));
    vi.advanceTimersByTime(5);
    sockets[0]!.onmessage!(msg({ slot: 0, attack: false }));
    vi.advanceTimersByTime(25);
    input.update();
    expect([input.justPressed('attack'), input.held('attack'), input.justPressed('skill2')]).toEqual([true, true, true]);
    input.update();
    expect([input.justPressed('attack'), input.held('attack'), input.justPressed('skill2')]).toEqual([false, false, false]);
  });

  it('AC-01: loslassen und neu drücken zwischen zwei Frames zählt als neuer Druck', () => {
    const { deps, sockets } = fakeDeps();
    const input = new BotInput(0, new BotFeed('ws://127.0.0.1:5180/bot/1/0', deps));
    sockets[0]!.onmessage!(msg({ slot: 0, pay: true }));
    input.update();
    input.update();
    expect([input.held('confirm'), input.justPressed('confirm')]).toEqual([true, false]);
    sockets[0]!.onmessage!(msg({ slot: 0, pay: false }));
    sockets[0]!.onmessage!(msg({ slot: 0, pay: true }));
    input.update();
    expect(input.justPressed('confirm')).toBe(true);
  });

  it('AC-02: ohne Nachricht länger als STALE_MS liefert der Slot keine Eingabe', () => {
    const { deps, sockets } = fakeDeps();
    const input = new BotInput(0, new BotFeed('ws://127.0.0.1:5180/bot/1/0', deps));
    sockets[0]!.onmessage!(msg({ slot: 0, moveX: 1, sprint: true, attack: true }));
    vi.advanceTimersByTime(STALE_MS);
    input.update();
    expect([input.moveX(), input.held('attack')]).toEqual([1, true]);
    vi.advanceTimersByTime(1);
    input.update();
    expect([input.moveX(), input.sprint(), input.held('attack')]).toEqual([0, false, false]);
    sockets[0]!.onmessage!(msg({ slot: 0, moveX: -1 }));
    input.update();
    expect(input.moveX()).toBe(-1);
  });

  it('AC-03: dauerhaft fehlender Feed loggt nur den ersten Abbruch', () => {
    const log = vi.spyOn(clientLogModule, 'clientLog').mockImplementation(() => {});
    const { deps, sockets, pending } = fakeDeps();
    new BotFeed('ws://127.0.0.1:5180/bot/1/0', deps);
    for (let i = 0; i < 5; i++) {
      sockets[i]!.onclose!();
      pending.shift()!();
    }
    const lost = () => log.mock.calls.filter(([, text]) => /🤒|👋/u.test(text));
    expect(lost()).toHaveLength(1);
    expect(lost()[0]![1]).toContain('🤒');
    sockets[5]!.onopen!();
    sockets[5]!.onclose!();
    expect(lost()).toHaveLength(2);
    expect(lost()[1]![1]).toContain('👋');
  });
});

describe('?botfeed (B-349/AC-02)', () => {
  it('ohne Parameter keine Bot-Eingabe, die Eingabe bleibt unverändert', () => {
    const { deps, sockets } = fakeDeps();
    expect(parseBotFeed('?room=ABCD&players=2', 'localhost')).toEqual({ kind: 'none' });
    expect(botInputs('', 'localhost', deps)).toEqual([]);
    expect(sockets).toHaveLength(0);
  });

  it.each(['ws://evil.example:5180/bot/1/0', 'http://127.0.0.1:5180/bot/1/0', 'ws://192.168.1.9:5180/bot', 'kaputt'])('lehnt %s ab', (url) => {
    const { deps, sockets } = fakeDeps();
    expect(parseBotFeed(`?botfeed=${encodeURIComponent(url)}`, 'localhost')).toEqual({ kind: 'rejected', url });
    expect(botInputs(`?botfeed=${encodeURIComponent(url)}&players=2`, 'localhost', deps)).toEqual([]);
    expect(sockets).toHaveLength(0);
  });

  it('erlaubt Loopback und den Host der Seite, players 1–4', () => {
    expect(parseBotFeed('?botfeed=ws://localhost:5180/bot/1/0&players=4', 'x')).toMatchObject({ kind: 'feed', players: 4 });
    expect(parseBotFeed('?botfeed=ws://[::1]:5180/bot/1/0', 'x')).toMatchObject({ kind: 'feed', players: 1 });
    expect(parseBotFeed('?botfeed=wss://192.168.1.9/bot/1/0&players=9', '192.168.1.9')).toMatchObject({ kind: 'feed', players: 1 });
    const { deps, sockets } = fakeDeps();
    expect(botInputs('?botfeed=ws://127.0.0.1:5180/bot/1/0&players=3', 'localhost', deps).map((i) => [i.slot, i.label])).toEqual([
      [0, 'Bot 1'],
      [1, 'Bot 2'],
      [2, 'Bot 3'],
    ]);
    expect(sockets.map((s) => s.url)).toEqual(['ws://127.0.0.1:5180/bot/1/0']);
  });
});
