import { describe, expect, it } from 'vitest';
import { setLanguage } from '../core/texts';
import { newGameHref, PAGES } from './pages';

/** Parameter, die game.html kennt: `parseStartParams` in src/scenes/lobbyLogic.ts, dazu `dev` (debugOverlay) und `touch` (touchInput). */
const KNOWN_PARAMS = ['autostart', 'fresh', 'save', 'mock', 'room', 'dev', 'touch'];

describe('Landing-Kacheln', () => {
  it('nur Spieler-Kacheln und genau eine kleine Kachel zur Entwicklerseite (B-335/AC-01)', () => {
    expect(PAGES.map((p) => p.id)).toEqual(['continue', 'new', 'online', 'licenses', 'dev']);
    const small = PAGES.filter((p) => p.small);
    expect(small.map((p) => p.href)).toEqual(['dev.html']);
    expect(small[0]!.primary).toBeFalsy();
  });

  it('Kacheln mit Ziel game.html nutzen nur der Lobby bekannte Parameter', () => {
    const games = PAGES.filter((p) => (typeof p.href === 'function' ? p.href() : p.href).startsWith('game.html'));
    expect(games.length).toBeGreaterThan(0);
    for (const p of games) {
      const href = typeof p.href === 'function' ? p.href() : p.href;
      const unknown = [...new URL(href, 'http://x/').searchParams.keys()].filter((k) => !KNOWN_PARAMS.includes(k));
      expect(unknown, `${p.title}: ${href}`).toEqual([]);
    }
  });

  it('Neues Spiel (B-292): fresh mit eigenem, gültigem Spielstandnamen je Aufruf', () => {
    const tile = PAGES.find((p) => p.id === 'new');
    expect(typeof tile?.href).toBe('function');
    const a = newGameHref(1_000_000_000_000);
    const b = newGameHref(1_000_000_000_001);
    for (const href of [a, b, (tile!.href as () => string)()]) {
      const params = new URL(href, 'http://x/').searchParams;
      expect(params.get('fresh')).toBe('1');
      expect(params.get('save')).toMatch(/^[a-z0-9-]{1,32}$/);
    }
    expect(a).not.toBe(b);
    expect(tile!.description).not.toMatch(/k3c|gesichert/);
  });

  it('Titel und Beschreibung folgen der Sprache, Rückfall Deutsch (B-369)', () => {
    setLanguage('en');
    expect(PAGES[0]!.title).toBe('Continue');
    setLanguage('fr');
    expect(PAGES[0]!.title).toBe('Weiterspielen');
    expect(PAGES.every((p) => p.title && p.description)).toBe(true);
  });
});
