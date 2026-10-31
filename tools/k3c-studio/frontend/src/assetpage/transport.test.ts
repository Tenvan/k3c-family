import { beforeEach, describe, expect, it, vi } from 'vitest';

// B-678 AC-02: Die Abspielleiste lässt nie zwei Töne zugleich klingen. Der Audio-Kern ist ein Spion mit Zählung der
// laufenden Töne je Bus; Musik überblendet (ersetzt die laufende), alles andere stoppt vorher.
const active = new Set<string>();
const kern = {
  getVolume: () => 0.5,
  stop: vi.fn((bus: string) => { for (const k of [...active]) if (k.startsWith(`${bus}:`)) active.delete(k); }),
};
let n = 0;
vi.mock('../sound/audio', () => ({
  core: () => kern,
  stopAll: () => { kern.stop('music'); kern.stop('sfx'); kern.stop('ambient'); },
  playChoice: async (slot: { bus?: string }) => {
    const bus = slot.bus ?? 'music';
    if (bus === 'music') kern.stop('music');
    active.add(`${bus}:${n++}`);
    return true;
  },
}));

import { playFile, playOn } from './transport';

const slot = (bus: 'music' | 'sfx') => ({ id: `${bus}:x`, beschreibung: '', bus });

beforeEach(() => { active.clear(); n = 0; });

describe('Abspielleiste (B-678)', () => {
  it('zwei Dateivorschauen nacheinander: höchstens ein Ton', async () => {
    await playFile('a', 'audio/a.ogg');
    await playFile('b', 'audio/b.ogg');
    expect(active.size).toBe(1);
  });
  it('Musik, dann Datei, dann Effekt, dann Musik: immer genau ein Ton', async () => {
    await playOn(slot('music'), { datei: 'audio/m.ogg' });
    await playFile('v', 'audio/v.ogg');
    expect(active.size).toBe(1);
    await playOn(slot('sfx'), { datei: 'audio/s.ogg' });
    expect(active.size).toBe(1);
    await playOn(slot('music'), { datei: 'audio/m2.ogg' });
    expect(active.size).toBe(1);
    await playOn(slot('music'), { datei: 'audio/m3.ogg' });
    expect(active.size).toBe(1);
  });
});
