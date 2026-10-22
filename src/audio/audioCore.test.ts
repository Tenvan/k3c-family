import { describe, expect, it } from 'vitest';
import { FORMAT_ORDER, parseAtlas, pickFile } from './atlas';
import { AudioCore, MAX_SAMPLE_VOICES, type FullContext, type SourceLike } from './audioCore';
import { SFX_URLS } from './events';
import atlasFile from '../../public/audio/atlas.json';

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

function setup(state = 'suspended', resumes = true, sfx: Record<string, string> = {}, bad: string[] = [], sprites: Record<string, { start: number; duration: number }> = { coin: { start: 0.3, duration: 0.2 } }) {
  const log = { created: 0, resumes: 0, fetched: [] as string[], started: [] as number[][], ended: [] as (() => void)[], gains: [] as { value: number }[], sources: [] as SourceLike[] };
  const ctx: FullContext = {
    destination: {},
    state,
    currentTime: 1,
    createGain: () => {
      const g = { gain: { value: 1 }, connect: () => undefined };
      log.gains.push(g.gain);
      return g;
    },
    resume: async () => {
      log.resumes++;
      if (resumes) ctx.state = 'running';
    },
    decodeAudioData: async () => ({ buf: 1 }),
    createBufferSource: () => {
      const src: SourceLike = { buffer: null, connect: () => undefined, start: (...a) => void log.started.push(a as number[]) };
      log.sources.push(src);
      return src;
    },
  };
  const core = new AudioCore({
    createContext: () => (log.created++, ctx),
    canPlay: () => true,
    fetchJson: async (u) => (log.fetched.push(u), { files, sprites }),
    fetchBytes: async (u) => {
      log.fetched.push(u);
      if (bad.includes(u)) throw new Error('404');
      return new ArrayBuffer(1);
    },
    storage: { getItem: () => null, setItem: () => undefined },
    sfx,
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
  it('onEvent (B-167/AC-03): built spielt den Slot sfx:build, außerhalb gedämpft, weit weg stumm, vor Entsperrung stumm', async () => {
    const { core, log } = setup('suspended', true, { 'sfx:build': 'audio/b.ogg', 'sfx:warn': 'audio/w.ogg' });
    const view = [{ center: 20, span: 20 }];
    expect(core.onEvent({ type: 'built', kind: 'wall' }, 20, view)).toBe(false);
    await core.onInput();
    await tick();
    expect(core.onEvent({ type: 'built', kind: 'wall' }, 20, view)).toBe(true);
    expect(core.onEvent({ type: 'built', kind: 'wall' }, 35, view)).toBe(true);
    expect(core.onEvent({ type: 'built', kind: 'wall' }, 500, view)).toBe(false);
    expect(core.onEvent({ type: 'dusk' }, 500, view)).toBe(true); // Warnung: global
    expect(core.onEvent({ type: 'night', day: 2 }, 20, view)).toBe(false); // kein Sound im Mapping
    expect(log.started.length).toBe(3);
  });
  it('Nacht leise (B-167/AC-06): nachts ist der Pegel des Effekts kleiner als am Tag', async () => {
    const { core, log } = setup('suspended', true, { 'sfx:build': 'audio/b.ogg' });
    await core.onInput();
    await tick();
    const buses = log.gains.length;
    core.onEvent({ type: 'built', kind: 'wall' }, undefined, [], 'day');
    expect(log.gains).toHaveLength(buses); // Tag: volle Lautstärke, kein Dämpfungsknoten
    core.onEvent({ type: 'built', kind: 'wall' }, undefined, [], 'night');
    expect(log.gains.at(-1)!.value).toBeLessThan(1);
  });
  it('Dichte: höchstens MAX_SAMPLE_VOICES gleichzeitige Instanzen je Datei, danach wieder Platz', async () => {
    const { core, log } = setup('suspended', true, { 'sfx:hit': 'audio/h.ogg' });
    await core.onInput();
    await tick();
    const hit = { type: 'hit', x: 5, target: 'enemy', id: 1, damage: 1 } as const;
    const results = Array.from({ length: MAX_SAMPLE_VOICES + 3 }, () => core.onEvent(hit, 5, []));
    expect(results.filter(Boolean)).toHaveLength(MAX_SAMPLE_VOICES);
    log.sources[0]!.onended?.();
    expect(core.onEvent(hit, 5, [])).toBe(true);
  });
  it('Rückfall (B-167/AC-05): fehlende Datei bleibt stumm ohne Fehler, die anderen spielen', async () => {
    const { core, log } = setup('suspended', true, { 'sfx:hit': 'audio/fehlt.ogg', 'sfx:kill': 'audio/k.ogg' }, ['audio/fehlt.ogg']);
    await core.onInput();
    await tick();
    expect(core.onEvent({ type: 'hit', x: 5, target: 'enemy', id: 1, damage: 1 }, 5, [])).toBe(false);
    expect(core.onEvent({ type: 'kill', kind: 'goblin', x: 5, gold: 1 }, 5, [])).toBe(true);
    expect(log.fetched).toContain('audio/fehlt.ogg');
  });
  it('Atlas (B-494/AC-02): Effekte im Atlas spielen als Sprite ohne eigene Datei, fehlende Slots laden einzeln', async () => {
    const { core, log } = setup('suspended', true, { 'sfx:build': 'audio/b.ogg', 'sfx:kill': 'audio/k.ogg' }, [], { 'sfx:build': { start: 1, duration: 0.5 } });
    await core.onInput();
    await tick();
    await tick();
    expect(log.fetched).toEqual(['audio/atlas.json', 'audio/a.ogg', 'audio/k.ogg']);
    expect(core.onEvent({ type: 'built', kind: 'wall' }, undefined, [])).toBe(true);
    expect(log.started.at(-1)).toEqual([1, 1, 0.5]);
    expect(core.onEvent({ type: 'kill', kind: 'goblin', x: 5, gold: 1 }, undefined, [])).toBe(true);
    const results = Array.from({ length: MAX_SAMPLE_VOICES + 2 }, () => core.onEvent({ type: 'built', kind: 'wall' }, undefined, []));
    expect(results.filter(Boolean)).toHaveLength(MAX_SAMPLE_VOICES - 1); // Stimmen-Grenze gilt auch im Atlas
  });
  it('Hub-Ausbau (SO6.4): nur ein Anstieg der Hub-Stufe spielt sfx:hub, undefined vergisst den Stand', async () => {
    const { core } = setup('suspended', true, { 'sfx:hub': 'audio/hub.ogg' });
    await core.onInput();
    await tick();
    await tick();
    expect(core.onHubLevel(1)).toBe(false); // erster Wert: kein Ausbau
    expect(core.onHubLevel(1)).toBe(false);
    expect(core.onHubLevel(2)).toBe(true);
    expect(core.onHubLevel(undefined)).toBe(false);
    expect(core.onHubLevel(4)).toBe(false); // neues Spiel
  });
  it('der erzeugte Atlas enthält jeden aktiven Effekt-Slot (B-494/AC-01, task gen:soundatlas)', () => {
    const effects = Object.keys(SFX_URLS).filter((s) => s !== 'sfx:ride').sort();
    expect(Object.keys(parseAtlas(atlasFile)!.sprites).sort()).toEqual(effects);
  });
  it('wirft nie, wenn der Context nicht entsteht', async () => {
    const core = new AudioCore({ createContext: () => { throw new Error('nein'); }, canPlay: () => true, fetchJson: async () => ({}), fetchBytes: async () => new ArrayBuffer(0) });
    await expect(core.onInput()).resolves.toBeUndefined();
  });
});
