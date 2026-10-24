import { describe, expect, it } from 'vitest';
import assets from '../../data/assets.json';
import { SFX_URLS } from './events';
import { UI_SLOTS, uiSoundFor } from './ui';

describe('UI-Klick-Töne (B-575/AC-01)', () => {
  it('Navigieren (Richtung, Seite) spielt Auswahl, Bestätigen schlägt Auswahl, ohne Eingabe nichts', () => {
    expect(uiSoundFor({ dir: 1 })).toBe('select');
    expect(uiSoundFor({ dir: 0, side: -1 })).toBe('select');
    expect(uiSoundFor({ dir: 1, confirm: true })).toBe('confirm');
    expect(uiSoundFor({ dir: 0, side: 0, confirm: false })).toBeNull();
  });
  it('beide Töne sind Effekt-Slots mit Datei im Register', () => {
    for (const slot of Object.values(UI_SLOTS)) {
      expect(assets.slots.some((s) => s.id === slot && s.art === 'sound' && s.bus === 'sfx'), slot).toBe(true);
      expect(SFX_URLS[slot], slot).toMatch(/^audio\/.+\.ogg$/);
    }
  });
});
