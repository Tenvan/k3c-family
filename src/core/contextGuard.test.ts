import { describe, expect, it } from 'vitest';
import { ContextGuard, RESTORE_WAIT_MS, TRIED_KEY, TRIED_TTL_MS, canvasRestartHref, type GuardEnv } from './contextGuard';

/** Umgebung mit steuerbarer Uhr; der Sitzungsspeicher überlebt `newPage()` wie ein Neuladen im selben Tab */
function setup(store = new Map<string, string>()) {
  let now = 0;
  const timers: { at: number; fn: () => void; live: boolean }[] = [];
  const calls = { restart: 0, notices: [] as string[], logs: [] as string[] };
  const env: GuardEnv = {
    now: () => now,
    read: (k) => store.get(k) ?? null,
    write: (k, v) => void store.set(k, v),
    after: (fn, ms) => {
      const t = { at: now + ms, fn, live: true };
      timers.push(t);
      return () => void (t.live = false);
    },
    restartWithCanvas: () => void calls.restart++,
    notice: (kind) => void calls.notices.push(kind),
    log: (msg) => void calls.logs.push(msg),
  };
  const advance = (ms: number) => {
    now += ms;
    for (const t of timers) if (t.live && t.at <= now) {
      t.live = false;
      t.fn();
    }
  };
  return { guard: new ContextGuard(env), calls, advance, store, newPage: () => setup(store) };
}

describe('Kontextverlust (AC-05)', () => {
  it('der erste Verlust ohne Wiederherstellung löst genau einen Canvas-Neustart aus', () => {
    const { guard, calls, advance, store } = setup();
    guard.lost();
    expect(guard.current).toBe('waiting');
    advance(RESTORE_WAIT_MS - 1);
    expect(calls.restart).toBe(0);
    advance(1);
    expect(calls.restart).toBe(1);
    expect(calls.notices).toEqual(['restart']);
    expect(store.get(TRIED_KEY)).toBe(String(RESTORE_WAIT_MS));
    expect(guard.current).toBe('restarting');
  });

  it('dreimal Verlust hintereinander (wie auf der Xbox) ergibt trotzdem nur einen Neustart', () => {
    const { guard, calls, advance } = setup();
    guard.lost();
    guard.lost();
    guard.lost();
    advance(RESTORE_WAIT_MS * 3);
    expect(calls.restart).toBe(1);
  });

  it('Wiederherstellung innerhalb der Frist: kein Neustart', () => {
    const { guard, calls, advance } = setup();
    guard.lost();
    advance(1000);
    guard.restored();
    advance(RESTORE_WAIT_MS * 2);
    expect(calls.restart).toBe(0);
    expect(guard.current).toBe('ok');
    expect(calls.logs.some((l) => l.includes('wiederhergestellt'))).toBe(true);
  });

  it('ein zweiter Verlust nach der Wiederherstellung startet sofort einen Neustart (instabil)', () => {
    const { guard, calls, advance } = setup();
    guard.lost();
    guard.restored();
    guard.lost();
    expect(calls.restart).toBe(1);
    advance(RESTORE_WAIT_MS * 2);
    expect(calls.restart).toBe(1);
  });
});

describe('keine Schleife (AC-02)', () => {
  it('nach dem Neustart gibt es bei einem Verlust keinen weiteren Versuch, nur den Hinweis', () => {
    const first = setup();
    first.guard.lost();
    first.advance(RESTORE_WAIT_MS);
    expect(first.calls.restart).toBe(1);
    const second = first.newPage();
    second.guard.lost();
    second.advance(RESTORE_WAIT_MS * 5);
    expect(second.calls.restart).toBe(0);
    expect(second.calls.notices).toEqual(['canvas']);
    expect(second.guard.current).toBe('gaveUp');
    second.guard.lost();
    second.advance(RESTORE_WAIT_MS);
    expect(second.calls.restart).toBe(0);
  });

  it('nach Ablauf der Marke (Seite später im WebGL-Modus neu geöffnet) ist wieder ein Versuch erlaubt', () => {
    const store = new Map([[TRIED_KEY, '1']]);
    const { guard, calls, advance } = setup(store);
    advance(TRIED_TTL_MS + 5);
    guard.lost();
    advance(RESTORE_WAIT_MS);
    expect(calls.restart).toBe(1);
    expect(calls.notices).toEqual(['restart']);
  });

  it('Verluste während des Wartens zählen nicht als eigener Verlust: nach der Wiederherstellung bleibt der Zustand ok', () => {
    const { guard, calls } = setup();
    guard.lost();
    guard.lost();
    guard.restored();
    expect(guard.current).toBe('ok');
    expect(calls.restart).toBe(0);
  });

  it('ein Verlust nach dem Aufgeben bleibt folgenlos', () => {
    const store = new Map([[TRIED_KEY, '1000']]);
    const { guard, calls } = setup(store);
    guard.lost();
    guard.lost();
    expect(calls.restart).toBe(0);
    expect(calls.notices).toEqual(['canvas']);
  });
});

describe('Adresse des Neustarts', () => {
  it('Canvas und Beitritt zum offenen Raum, ohne neues Spiel', () => {
    const href = canvasRestartHref('game.html', '?dev=1&autostart=1&fresh=1&save=stress-wald&players=2&stress=typisch', 'ABCD');
    const p = new URLSearchParams(href.split('?')[1]);
    expect(href.startsWith('game.html?')).toBe(true);
    expect(p.get('renderer')).toBe('canvas');
    expect(p.get('room')).toBe('ABCD');
    expect(p.get('autostart')).toBe('1');
    expect(p.has('fresh')).toBe(false);
    expect(p.has('save')).toBe(false);
    expect(p.get('players')).toBe('2');
  });

  it('ohne Raum bleiben die Parameter, nur renderer kommt dazu', () => {
    const href = canvasRestartHref('game.html', '?gfx=1', null);
    expect(href).toBe('game.html?gfx=1&renderer=canvas');
  });
});

describe('Absichtliche Freigabe (RX1)', () => {
  it('stop bricht das Warten ab, danach löst auch ein Verlust keinen Neustart und keinen Hinweis aus', () => {
    const { guard, calls, advance } = setup();
    guard.lost();
    guard.stop();
    advance(RESTORE_WAIT_MS * 2);
    guard.lost();
    advance(RESTORE_WAIT_MS * 2);
    expect(calls.restart).toBe(0);
    expect(calls.notices).toEqual([]);
  });
});
