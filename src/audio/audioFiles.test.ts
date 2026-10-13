import { describe, expect, it } from 'vitest';
import { AudioCore, type FullContext } from './audioCore';
import { crossfadeCurves, fadeOutFrom } from './crossfade';

/** Dateien in Schleife mit Crossfade (B-250, SO4.6, AC-08), Lautstärke je Bus (AC-05). */
interface Src { loop?: boolean; started: number[][]; stops: number[]; onended?: (() => void) | null }
interface Gn { value: number; curves: { curve: Float32Array; at: number; dur: number }[]; cancelled: number[] }

const files = { ogg: 'a.ogg', mp3: 'a.mp3' };
const tick = () => new Promise((r) => setTimeout(r, 0));

function setup() {
  const sources: Src[] = [];
  const gains: Gn[] = [];
  const fetched: string[] = [];
  const ctx: FullContext = {
    destination: {},
    state: 'suspended',
    currentTime: 5,
    createGain: () => {
      const g: Gn = { value: 1, curves: [], cancelled: [] };
      gains.push(g);
      return {
        gain: {
          get value() { return g.value; },
          set value(v: number) { g.value = v; },
          setValueCurveAtTime: (curve: Float32Array, at: number, dur: number) => void g.curves.push({ curve, at, dur }),
          cancelScheduledValues: (t: number) => void g.cancelled.push(t),
        },
        connect: () => undefined,
      };
    },
    resume: async () => { ctx.state = 'running'; },
    decodeAudioData: async () => ({ buf: 1 }),
    createBufferSource: () => {
      const s: Src = { started: [], stops: [] };
      sources.push(s);
      return {
        buffer: null,
        set loop(v: boolean | undefined) { s.loop = v; },
        get loop() { return s.loop; },
        set onended(f: (() => void) | null | undefined) { s.onended = f; },
        get onended() { return s.onended; },
        connect: () => undefined,
        start: (...a: number[]) => void s.started.push(a),
        stop: (t?: number) => void s.stops.push(t ?? 0),
      };
    },
  };
  const core = new AudioCore({
    createContext: () => ctx,
    canPlay: () => true,
    fetchJson: async () => ({ files, sprites: {} }),
    fetchBytes: async (u) => {
      fetched.push(u);
      if (u.includes('kaputt')) throw new Error('404');
      return new ArrayBuffer(1);
    },
    storage: { getItem: () => null, setItem: () => undefined },
  });
  const run = async () => { await core.onInput(); await tick(); };
  return { core, sources, gains, fetched, run };
}

describe('Audio-Kern: ganze Dateien (AC-08)', () => {
  it('spielt eine Datei in Schleife auf dem Bus Musik; vor der Entsperrung nichts', async () => {
    const t = setup();
    expect(await t.core.playFile('audio/m1.ogg')).toBe(false);
    expect(t.fetched).toEqual([]);
    await t.run();
    expect(await t.core.playFile('audio/m1.ogg')).toBe(true);
    expect(t.sources.at(-1)!.loop).toBe(true);
    expect(t.sources.at(-1)!.started).toEqual([[5, 0]]);
    expect(t.core.isPlaying('music', 'audio/m1.ogg')).toBe(true);
    await t.core.playFile('audio/b.ogg', { bus: 'sfx' });
    expect(t.sources.at(-1)!.loop).toBe(false);
  });

  it('lädt jede Datei nur einmal; dasselbe Stück läuft einfach weiter', async () => {
    const t = setup();
    await t.run();
    await t.core.playFile('audio/m1.ogg');
    const n = t.sources.length;
    expect(await t.core.playFile('audio/m1.ogg')).toBe(true);
    expect(t.sources.length).toBe(n);
    expect(t.fetched.filter((u) => u === 'audio/m1.ogg')).toHaveLength(1);
  });

  it('blendet zwei Stücke über: neues mit der In-Kurve, altes mit der Out-Kurve und endet danach', async () => {
    const t = setup();
    await t.run();
    await t.core.playFile('audio/m1.ogg');
    await t.core.playFile('audio/b.ogg', { fade: 2 });
    const [oldSrc, newSrc] = t.sources.slice(-2);
    const [oldGain, newGain] = t.gains.slice(-2);
    expect(newGain.curves).toHaveLength(1);
    expect(newGain.curves[0]).toMatchObject({ at: 5, dur: 2 });
    expect(Array.from(newGain.curves[0].curve)).toEqual(Array.from(crossfadeCurves().in));
    expect(oldGain.curves.at(-1)).toMatchObject({ at: 5, dur: 2 });
    expect(Array.from(oldGain.curves.at(-1)!.curve)).toEqual(Array.from(fadeOutFrom(1)));
    expect(oldSrc.stops).toEqual([7]);
    expect(newSrc.stops).toEqual([]);
    expect(t.core.isPlaying('music', 'audio/m1.ogg')).toBe(false);
    expect(t.core.isPlaying('music', 'audio/b.ogg')).toBe(true);
  });

  it('lenkt einen laufenden Übergang vom aktuellen Pegel aus um, ohne Sprung', async () => {
    const t = setup();
    await t.run();
    await t.core.playFile('audio/m1.ogg');
    await t.core.playFile('audio/b.ogg', { fade: 2 });
    const mid = t.gains.at(-1)!; // b blendet noch ein, Pegel 0.4
    mid.value = 0.4;
    await t.core.playFile('audio/c.ogg', { fade: 2 });
    const last = mid.curves.at(-1)!;
    expect(last.curve[0]).toBeCloseTo(0.4, 5);
    expect(last.curve.at(-1)).toBeCloseTo(0, 5);
    expect(mid.cancelled).toEqual([5]);
  });

  it('fehlende Datei: false, kein Absturz, das laufende Stück bleibt, nur ein Request', async () => {
    const t = setup();
    await t.run();
    await t.core.playFile('audio/m1.ogg');
    expect(await t.core.playFile('audio/kaputt.ogg', { fade: 2 })).toBe(false);
    expect(await t.core.preload('audio/kaputt.ogg')).toBe(false);
    expect(t.core.isPlaying('music', 'audio/m1.ogg')).toBe(true);
    expect(t.fetched.filter((u) => u.includes('kaputt'))).toHaveLength(1);
  });

  it('stop blendet aus; das Ende eines Effekts meldet onEnded genau einmal', async () => {
    const t = setup();
    await t.run();
    let ended = 0;
    await t.core.playFile('audio/m1.ogg', { bus: 'sfx', onEnded: () => void ended++ });
    t.sources.at(-1)!.onended?.();
    t.sources.at(-1)!.onended?.();
    expect(ended).toBe(1);
    await t.core.playFile('audio/m.ogg');
    t.core.stop('music');
    expect(t.sources.at(-1)!.stops).toHaveLength(1);
    expect(t.core.isPlaying('music')).toBe(false);
  });

  it('Lautstärke je Bus getrennt (AC-05): Musik ändert Effekte nicht', async () => {
    const t = setup();
    await t.run();
    t.core.setVolume('sfx', 30);
    t.core.setVolume('music', 70);
    expect([t.core.getVolume('music'), t.core.getVolume('sfx'), t.core.getVolume('ambient')]).toEqual([70, 30, 100]);
  });
});
