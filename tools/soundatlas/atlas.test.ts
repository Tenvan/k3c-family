// Reine Teile des Atlas-Baus (aus atlas_test.go); der Lauf mit ffmpeg ist task gen:soundatlas.

import { describe, expect, it } from 'vitest';
import { atlasJson, isLocal, layout, pickEntries, type Slot } from './atlas';

const slot = (id: string, files: string[], extra: Partial<Slot> = {}): Slot => ({
  id,
  art: 'sound',
  bus: 'sfx',
  wahl: files.map((datei) => ({ datei })),
  ...extra,
});

describe('atlas', () => {
  it('nimmt die erste Präferenz ohne Schleifen', () => {
    const slots = [
      slot('sfx:warn', ['audio/w.ogg', 'audio/w2.ogg']),
      slot('sfx:coin', ['audio/c.ogg']),
      slot('sfx:ride', ['audio/r.ogg']),
      slot('music:tag', ['audio/m.ogg'], { bus: 'music' }),
      slot('sfx:leer', []),
      slot('sfx:alt', [], { wahl: [{ datei: 'audio/atlas.ogg', sprite: 'coin' }] }),
      slot('bild', ['x.png'], { art: 'grafik' }),
    ];
    expect(pickEntries(slots)).toEqual([
      { slot: 'sfx:coin', file: 'audio/c.ogg' },
      { slot: 'sfx:warn', file: 'audio/w.ogg' },
    ]);
  });

  it('legt Pausen zwischen die Effekte', () => {
    const entries = ['a', 'b', 'c'].map((s) => ({ slot: s, file: '' }));
    const { sprites, pads } = layout(entries, [44100, 22050, 4410]);
    expect(pads).toEqual([0, 6615, 6615]); // 0,15 s
    expect(sprites.get('a')).toEqual({ start: 0, duration: 1 });
    expect(sprites.get('b')).toEqual({ start: 1.15, duration: 0.5 });
    expect(sprites.get('c')).toEqual({ start: 1.8, duration: 0.1 });
  });

  it('schreibt den Index sortiert mit Schlusszeilenumbruch', () => {
    const text = atlasJson(new Map([['sfx:b', { start: 1.15, duration: 0.5 }], ['sfx:a', { start: 0, duration: 1 }]]));
    expect(text.indexOf('"mp3"')).toBeLessThan(text.indexOf('"ogg"'));
    expect(text.indexOf('sfx:a')).toBeLessThan(text.indexOf('sfx:b'));
    expect(text).toContain('    "sfx:a": {\n      "start": 0,\n      "duration": 1\n    },');
    expect(text.endsWith('}\n')).toBe(true);
  });

  it('lässt nur Pfade unter public/ zu', () => {
    expect(isLocal('audio/x.ogg')).toBe(true);
    expect(isLocal('audio/../x.ogg')).toBe(true);
    for (const bad of ['', '..', '../x', 'audio/../../x', '/etc/x', '\\x', 'C:\\x', 'audio\\..\\..\\x', 'nul', 'audio/CON.ogg']) {
      expect(isLocal(bad), bad).toBe(false);
    }
  });
});
