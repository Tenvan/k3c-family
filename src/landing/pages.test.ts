import { describe, expect, it } from 'vitest';
import { PAGES } from './pages';

/** Parameter, die game.html kennt: `parseStartParams` in src/scenes/lobbyLogic.ts, dazu `dev` (debugOverlay) und `touch` (touchInput). */
const KNOWN_PARAMS = ['autostart', 'fresh', 'save', 'mock', 'room', 'dev', 'touch'];

describe('Landing-Kacheln', () => {
  it('Kacheln mit Ziel game.html nutzen nur der Lobby bekannte Parameter', () => {
    const games = PAGES.filter((p) => (typeof p.href === 'function' ? p.href() : p.href).startsWith('game.html'));
    expect(games.length).toBeGreaterThan(0);
    for (const p of games) {
      const href = typeof p.href === 'function' ? p.href() : p.href;
      const unknown = [...new URL(href, 'http://x/').searchParams.keys()].filter((k) => !KNOWN_PARAMS.includes(k));
      expect(unknown, `${p.title}: ${href}`).toEqual([]);
    }
  });
});
