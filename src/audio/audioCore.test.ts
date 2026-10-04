import { describe, expect, it } from 'vitest';
import { FORMAT_ORDER, parseAtlas, pickFile } from './atlas';
import { AudioCore, type FullContext } from './audioCore';

const files = { ogg: 'a.ogg', mp3: 'a.mp3' };

describe('pickFile (AC-04)', () => {
  it('nimmt das erste unterstützte Format der Reihenfolge, ogg vor mp3', () => {
    expect(FORMAT_ORDER[0]).toBe('ogg');
    expect(pickFile(files, () => true)).toBe('a.ogg');
  });
  it('fällt auf mp3 zurück, wenn ogg nicht geht', () => {
    expect(pickFile(files, (m) => m === 'audio/mpeg')).toBe('a.mp3');
  });
  it('ohne unterstütztes Format keine Datei', () => {
    expect(pickFile(files, () => false)).toBeUndefined();
    expect(pickFile({ wav: 'a.wav' }, (m) => m === 'audio/mpeg')).toBeUndefined();
  });
});

describe('parseAtlas', () => {
  it('behält gültige Sprites, verwirft kaputte, lehnt Müll ab', () => {
    const a = parseAtlas({ files, sprites: { ok: { start: 0, duration: 0.2 }, bad: { start: -1, duration: 1 }, nan: {} } });
    expect(Object.keys(a!.sprites)).toEqual(['ok']);
    expect(parseAtlas(null)).toBeUndefined();
    expect(parseAtlas({ files })).toBeUndefined();
  });
});

function setup(state = 'suspended', resumes = true) {
  const log = { created: 0, resumes: 0, fetched: [] as string[], started: [] as number[][] };
  const ctx: FullContext = {
    destination: {},
    state,
    currentTime: 1,
    createGain: () => ({ gain: { value: 1 }, connect: () => undefined }),
    resume: async () => {
      log.resumes++;
      if (resumes) ctx.state = 'running';
    },
    decodeAudioData: async () => ({ buf: 1 }),
    createBufferSource: () => ({ buffer: null, connect: () => undefined, start: (...a) => void log.started.push(a) }),
  };
  const core = new AudioCore({
    createContext: () => (log.created++, ctx),
    canPlay: () => true,
    fetchJson: async (u) => (log.fetched.push(u), { files, sprites: { coin: { start: 0.3, duration: 0.2 } } }),
    fetchBytes: async (u) => (log.fetched.push(u), new ArrayBuffer(1)),
    storage: { getItem: () => null, setItem: () => undefined },
  });
  return { core, log };
}
const tick = () => new Promise((r) => setTimeout(r, 0));

describe('AudioCore (AC-03, AC-05)', () => {
  it('vor der ersten Eingabe: kein Context, keine Requests, play ist stumm', () => {
    const { core, log } = setup();
    expect(core.play('coin')).toBe(false);
    expect(log.created + log.fetched.length).toBe(0);
  });
  it('erste Eingabe: Context, resume, genau 2 Requests (Atlas + Datei), danach Ton', async () => {
    const { core, log } = setup();
    await core.onInput();
    await tick();
    expect(log.resumes).toBe(1);
    expect(log.fetched).toEqual(['audio/atlas.json', 'audio/a.ogg']);
    expect(core.play('coin')).toBe(true);
    expect(log.started).toEqual([[1, 0.3, 0.2]]);
    expect(core.play('gibtsnicht')).toBe(false);
    await core.onInput();
    expect(log.created).toBe(1);
    expect(log.resumes).toBe(1);
  });
  it('bleibt der Context gesperrt (Gamepad zählt nicht), wird bei jeder Eingabe erneut resumed, ohne neue Requests', async () => {
    const { core, log } = setup('suspended', false);
    await core.onInput();
    await core.onInput();
    await tick();
    expect(log.resumes).toBe(2);
    expect(log.created).toBe(1);
    expect(log.fetched.length).toBe(2);
    expect(core.play('coin')).toBe(false);
  });
  it('onEvent (AC-07): built löst den Demo-Ton aus, außerhalb gedämpft, weit weg stumm, vor Entsperrung stumm', async () => {
    const { core, log } = setup();
    const view = [{ center: 20, span: 20 }];
    expect(core.onEvent({ type: 'built', kind: 'wall' }, 20, view)).toBe(false);
    await core.onInput();
    await tick();
    expect(core.onEvent({ type: 'built', kind: 'wall' }, 20, view)).toBe(true);
    expect(core.onEvent({ type: 'built', kind: 'wall' }, 35, view)).toBe(true);
    expect(core.onEvent({ type: 'built', kind: 'wall' }, 500, view)).toBe(false);
    expect(core.onEvent({ type: 'dusk' }, 20, view)).toBe(false);
    expect(log.started.length).toBe(2);
  });
  it('wirft nie, wenn der Context nicht entsteht', async () => {
    const core = new AudioCore({ createContext: () => { throw new Error('nein'); }, canPlay: () => true, fetchJson: async () => ({}), fetchBytes: async () => new ArrayBuffer(0) });
    await expect(core.onInput()).resolves.toBeUndefined();
  });
});
