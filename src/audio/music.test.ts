import { describe, expect, it, vi } from 'vitest';
import { MUSIC_SLOTS, MUSIC_STATES, MUSIC_TRACKS, MusicDirector, musicTracks, nextMusicState, trackUrl, type MusicState, type MusicView } from './music';

const view = (v: Partial<MusicView> = {}): MusicView => ({ phase: 'day', enemies: 0, depth: 0, ...v });

describe('Zustands-Automat (SO4.2 AC-01)', () => {
  it('Tageszeit: Tag → Abend → Nacht', () => {
    expect((['day', 'dusk', 'night'] as const).map((phase) => nextMusicState(undefined, view({ phase })))).toEqual(['day', 'dusk', 'night']);
  });

  it('Priorität: Lobby > Ende > Boss > Kampf > Tiefe > Tageszeit', () => {
    const all = view({ phase: 'night', enemies: 3, depth: 2, boss: true, victory: true });
    expect(nextMusicState(undefined, { ...all, lobby: true })).toBe('lobby');
    expect(nextMusicState(undefined, all)).toBe('ending');
    expect(nextMusicState(undefined, { ...all, victory: false })).toBe('boss');
    expect(nextMusicState(undefined, { ...all, victory: false, boss: false })).toBe('battle');
    expect(nextMusicState(undefined, { ...all, victory: false, boss: false, enemies: 0 })).toBe('depth');
    expect(nextMusicState(undefined, view({ phase: 'night' }))).toBe('night');
  });

  it('Niederlage über castleFallen bleibt bis zur Lobby; andere Ereignisse ändern nichts', () => {
    expect(nextMusicState('night', view({ phase: 'night' }), [{ type: 'hit' }])).toBe('night');
    const lost = nextMusicState('night', view({ phase: 'night', enemies: 2 }), [{ type: 'castleFallen' }]);
    expect(lost).toBe('defeat');
    expect(nextMusicState(lost, view({ phase: 'day' }))).toBe('defeat');
    expect(nextMusicState(lost, view({ phase: 'day', victory: true }))).toBe('defeat');
    expect(nextMusicState(lost, view({ lobby: true }))).toBe('lobby');
  });

  it('Boss und Sieg ohne Quelle: künstlicher Zustand genügt', () => {
    expect(nextMusicState('day', view({ boss: true }))).toBe('boss');
    expect(nextMusicState('day', view({ victory: true }))).toBe('ending');
    expect(nextMusicState('ending', view({ phase: 'day' }))).toBe('ending');
  });

  it('Niederlage und Sieg sind getrennte Zustände (B-573/AC-01)', () => {
    expect(nextMusicState('day', view(), [{ type: 'castleFallen' }])).not.toBe(nextMusicState('day', view({ victory: true })));
  });
});

describe('Mapping Zustand → Stück (AC-01)', () => {
  it('jeder der neun Zustände hat genau einen Eintrag: ein Stück oder Lücke; kein Stück doppelt', () => {
    expect(Object.keys(MUSIC_TRACKS).sort()).toEqual([...MUSIC_STATES].sort());
    for (const s of MUSIC_STATES) expect(['string', 'object']).toContain(typeof MUSIC_TRACKS[s]); // Text oder null (Lücke)
    const tracks = Object.values(MUSIC_TRACKS).filter((t) => t !== null);
    expect(new Set(tracks).size).toBe(tracks.length);
  });

  it('trackUrl nimmt das erste abspielbare Format; Lücke oder kein Format → undefined', () => {
    expect(trackUrl('tag', () => true)).toBe('audio/tag.ogg');
    expect(trackUrl('tag', (m) => m === 'audio/mpeg')).toBe('audio/tag.mp3');
    expect(trackUrl('tag', () => false)).toBeUndefined();
    expect(trackUrl(null, () => true)).toBeUndefined();
  });

  it('trackUrl mit Endung: die Datei selbst, wenn das Format abspielbar ist', () => {
    expect(trackUrl('audio/kandidaten/x.ogg', () => true)).toBe('audio/kandidaten/x.ogg');
    expect(trackUrl('audio/kandidaten/x.ogg', (m) => m === 'audio/mpeg')).toBeUndefined();
  });
});

describe('Auswahl aus dem Slot-Register (SO4.1, SO4.3)', () => {
  it('musicTracks nimmt die erste Präferenz je Slot; fehlender oder leerer Slot → Lücke', () => {
    const slots = [
      { id: 'music:tag', art: 'sound', bus: 'music', wahl: [{ datei: 'audio/a.ogg' }, { datei: 'audio/b.ogg' }] },
      { id: 'music:nacht', art: 'sound', bus: 'music', wahl: [] },
      { id: 'music:abend', art: 'sound', bus: 'sfx', wahl: [{ datei: 'audio/c.ogg' }] },
    ];
    const t = musicTracks(slots);
    expect(t.day).toBe('audio/a.ogg');
    expect(t.night).toBeNull();
    expect(t.dusk).toBeNull(); // falscher Bus zählt nicht
    expect(t.ending).toBeNull();
  });

  it('das Register belegt alle neun Zustände mit Ogg-Dateien; Niederlage und Sieg haben je ein eigenes Stück (B-573)', () => {
    for (const s of MUSIC_STATES) expect(MUSIC_TRACKS[s]).toMatch(/^audio\/.+\.ogg$/);
    expect(MUSIC_SLOTS.defeat).toBe('music:niederlage');
    expect(MUSIC_SLOTS.ending).toBe('music:ende');
    expect(MUSIC_TRACKS.defeat).not.toBe(MUSIC_TRACKS.ending);
  });
});

function setup(tracks: Partial<Record<MusicState, string | null>>, opts: { running?: boolean; load?: (url: string) => Promise<boolean> } = {}) {
  const core = {
    running: opts.running ?? true,
    preload: vi.fn(opts.load ?? (async () => true)),
    playFile: vi.fn(async (_url: string, _opts?: unknown) => true),
    stop: vi.fn(),
  };
  const full = Object.fromEntries(MUSIC_STATES.map((s) => [s, null])) as Record<MusicState, string | null>;
  const director = new MusicDirector(() => core, () => true, { ...full, ...tracks });
  const settle = () => new Promise((r) => setTimeout(r, 0));
  return { core, director, settle };
}

describe('MusicDirector', () => {
  it('spielt das Stück des Zustands per Crossfade auf dem Bus Musik, ein Zustand genau einmal', async () => {
    const { core, director, settle } = setup({ day: 'tag', night: 'nacht' });
    director.update(view());
    await settle();
    director.update(view());
    director.update(view({ phase: 'night' }));
    await settle();
    director.update(view({ phase: 'night' }));
    expect(core.playFile.mock.calls).toEqual([
      ['audio/tag.ogg', { bus: 'music', loop: true, fade: 2 }],
      ['audio/nacht.ogg', { bus: 'music', loop: true, fade: 2 }],
    ]);
  });

  it('Lücke: Stille (Ausblenden), kein Ladeversuch, kein Fehler', async () => {
    const { core, director, settle } = setup({});
    director.update(view());
    await settle();
    expect(core.stop).toHaveBeenCalledWith('music', 2);
    expect(core.preload).not.toHaveBeenCalled();
    director.update(view());
    expect(core.stop).toHaveBeenCalledTimes(1);
  });

  it('fehlende Datei: nichts bricht, es wird nicht jeden Frame neu versucht', async () => {
    const { core, director, settle } = setup({ day: 'weg' }, { load: async () => false });
    for (let i = 0; i < 3; i++) {
      director.update(view());
      await settle();
    }
    expect(core.preload).toHaveBeenCalledTimes(1);
    expect(director.current).toBe('day');
  });

  it('gesperrt (keine Eingabe): kein Request, nach der Entsperrung startet das Stück', async () => {
    const { core, director, settle } = setup({ day: 'tag' }, { running: false });
    director.update(view());
    await settle();
    expect(core.preload).not.toHaveBeenCalled();
    core.running = true;
    director.update(view());
    await settle();
    expect(core.playFile).toHaveBeenCalledTimes(1);
  });

  it('Wechsel während des Ladens: das alte, später fertige Stück startet nicht mehr', async () => {
    let release!: () => void;
    const slow = new Promise<boolean>((r) => (release = () => r(true)));
    const { core, director, settle } = setup({ day: 'tag', night: 'nacht' }, { load: (url) => (url.includes('tag') ? slow : Promise.resolve(true)) });
    director.update(view());
    director.update(view({ phase: 'night' })); // Tag lädt noch
    release();
    await settle();
    director.update(view({ phase: 'night' }));
    await settle();
    expect(core.playFile.mock.calls.map((c) => c[0])).toEqual(['audio/nacht.ogg']);
  });

  it('Lobby meldet sich selbst; reset vergisst das Ende', () => {
    const { director } = setup({});
    director.update(view(), [{ type: 'castleFallen' }]);
    expect(director.current).toBe('defeat');
    director.reset();
    director.lobby();
    expect(director.current).toBe('lobby');
  });
});
