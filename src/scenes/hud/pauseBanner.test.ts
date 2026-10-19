import { describe, expect, it } from 'vitest';
import { setLanguage, t } from '../../core/texts';
import { pauseShown } from './pauseBanner';

describe('pauseShown: Pause-Anzeige (AZ1.2, B-333/AC-01)', () => {
  it('nur bei devPaused === true', () => {
    expect(pauseShown({ devPaused: true }, null)).toBe(true);
    expect(pauseShown({ devPaused: false }, null)).toBe(false);
  });

  it('Zustand ohne das Feld (Couch-Raum) und ohne Welt: keine Anzeige', () => {
    expect(pauseShown({}, null)).toBe(false);
    expect(pauseShown(null, null)).toBe(false);
    expect(pauseShown(undefined, null)).toBe(false);
  });

  it('der Verbindungs-Hinweis hat Vorrang und wird nicht verdeckt', () => {
    expect(pauseShown({ devPaused: true }, 'Verbindung verloren')).toBe(false);
    expect(pauseShown({ devPaused: true }, '')).toBe(true);
  });

  it('Text „Pausiert“ in de und en', () => {
    setLanguage('de');
    expect(t('hud.paused')).toBe('Pausiert');
    setLanguage('en');
    expect(t('hud.paused')).toBe('Paused');
    setLanguage('de');
  });
});
