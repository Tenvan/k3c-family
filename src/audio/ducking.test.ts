import { describe, expect, it } from 'vitest';
import { AudioCore, type FullContext } from './audioCore';
import { DUCK, duckGain, extendDuck } from './ducking';

/** Ducking bei Warnungen (SO4.3, B-168/AC-03). */
describe('Ducking: Pegel der Musik', () => {
  it('ohne Warnung voll; sinkt bei Warnung, hält und steigt zurück', () => {
    expect(duckGain(3, undefined)).toBe(1);
    const w = extendDuck(undefined, 10);
    expect(w.start).toBe(10);
    expect(w.end).toBeCloseTo(10 + DUCK.hold);
    expect(duckGain(10, w)).toBe(1);
    expect(duckGain(10 + DUCK.attack / 2, w)).toBeCloseTo((1 + DUCK.level) / 2);
    expect(duckGain(10 + DUCK.attack, w)).toBeCloseTo(DUCK.level);
    expect(duckGain(w.end, w)).toBeCloseTo(DUCK.level);
    expect(duckGain(w.end + DUCK.release / 2, w)).toBeCloseTo((1 + DUCK.level) / 2);
    expect(duckGain(w.end + DUCK.release, w)).toBe(1);
  });

  it('überlappende Warnungen verlängern statt zu stapeln: nie unter level', () => {
    let w = extendDuck(undefined, 0);
    w = extendDuck(w, 0.5);
    w = extendDuck(w, 1);
    expect(w.start).toBe(0);
    expect(w.end).toBeCloseTo(1 + DUCK.hold);
    for (let t = 0; t < 4; t += 0.05) expect(duckGain(t, w)).toBeGreaterThanOrEqual(DUCK.level - 1e-9);
  });

  it('Warnung in der Rampe hinauf: kein Sprung, die neue Absenkung startet beim aktuellen Pegel', () => {
    const w = extendDuck(undefined, 0);
    const t = w.end + DUCK.release / 2;
    const before = duckGain(t, w);
    const w2 = extendDuck(w, t);
    expect(duckGain(t, w2)).toBeCloseTo(before, 9);
    expect(duckGain(t + DUCK.attack, w2)).toBeCloseTo(DUCK.level);
    expect(w2.end).toBeCloseTo(t + DUCK.hold);
  });

  it('Warnung lange nach der letzten: neues Fenster ab jetzt', () => {
    const w = extendDuck(extendDuck(undefined, 0), 30);
    expect(w).toEqual({ start: 30, end: 30 + DUCK.hold });
  });
});

interface Ramp { kind: string; value: number; at: number }

function setup() {
  const gains: Ramp[][] = [];
  const ctx: FullContext = {
    destination: {},
    state: 'suspended',
    currentTime: 5,
    createGain: () => {
      const log: Ramp[] = [];
      gains.push(log);
      return {
        gain: {
          value: 1,
          cancelScheduledValues: (t: number) => void log.push({ kind: 'cancel', value: 0, at: t }),
          setValueAtTime: (value: number, at: number) => void log.push({ kind: 'set', value, at }),
          linearRampToValueAtTime: (value: number, at: number) => void log.push({ kind: 'ramp', value, at }),
          setValueCurveAtTime: () => undefined,
        },
        connect: () => undefined,
      };
    },
    resume: async () => { ctx.state = 'running'; },
    decodeAudioData: async () => ({ buf: 1 }),
    createBufferSource: () => ({ buffer: null, connect: () => undefined, start: () => undefined, stop: () => undefined }),
  };
  const core = new AudioCore({
    createContext: () => ctx,
    canPlay: () => true,
    fetchJson: async () => ({ files: { ogg: 'a.ogg' }, sprites: {} }),
    fetchBytes: async () => new ArrayBuffer(1),
    storage: { getItem: () => null, setItem: () => undefined },
    sfx: {},
  });
  return { core, ctx, gains };
}

describe('Ducking im Audio-Kern', () => {
  it('gesperrt: nichts; Warn-Ereignis plant Rampe hinab und zurück auf dem Ducking-Gain', async () => {
    const t = setup();
    expect(t.core.duck()).toBe(false);
    await t.core.onInput();
    const duckLog = t.gains[4]; // Master, drei Busse, dann der Ducking-Gain
    t.core.onEvent({ type: 'dusk' } as never, undefined, []);
    expect(duckLog.filter((r) => r.kind === 'ramp')).toEqual([
      { kind: 'ramp', value: DUCK.level, at: 5 + DUCK.attack },
      { kind: 'ramp', value: 1, at: 5 + DUCK.hold + DUCK.release },
    ]);
    expect(duckLog[1]).toEqual({ kind: 'set', value: 1, at: 5 });
  });

  it('kein Ducking für Effekte ohne Warnung', async () => {
    const t = setup();
    await t.core.onInput();
    t.core.onEvent({ type: 'coinPickup', x: 0 } as never, 0, []);
    expect(t.gains[4]).toEqual([]);
  });

  it('zweite Warnung verlängert vom aktuellen Pegel aus', async () => {
    const t = setup();
    await t.core.onInput();
    t.core.onEvent({ type: 'wave' } as never, undefined, []);
    t.ctx.currentTime = 5.5;
    t.core.onEvent({ type: 'castleFallen' } as never, undefined, []);
    const log = t.gains[4];
    const lastSet = log.filter((r) => r.kind === 'set').at(-2)!;
    expect(lastSet).toEqual({ kind: 'set', value: DUCK.level, at: 5.5 });
    expect(log.at(-1)).toEqual({ kind: 'ramp', value: 1, at: 5.5 + DUCK.hold + DUCK.release });
  });
});
