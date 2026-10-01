import { describe, expect, it } from 'vitest';
import { PAGES } from './pages';
import { needsServer, serverReachable } from './serverCheck';

const reply = (status: number, body: string): typeof fetch => (async () => new Response(body, { status })) as unknown as typeof fetch;

describe('serverReachable', () => {
  it('Server antwortet mit {"ok":true} → erreichbar', async () => {
    expect(await serverReachable(reply(200, '{"ok":true}'))).toBe(true);
  });

  it('GitHub Pages: 404-Seite → nicht erreichbar', async () => {
    expect(await serverReachable(reply(404, '<html>Not found</html>'))).toBe(false);
  });

  it('statischer Server liefert HTML mit 200 → nicht erreichbar', async () => {
    expect(await serverReachable(reply(200, '<html></html>'))).toBe(false);
  });

  it('Netzwerkfehler → nicht erreichbar, kein Fehler', async () => {
    const down = (async () => {
      throw new TypeError('Failed to fetch');
    }) as unknown as typeof fetch;
    expect(await serverReachable(down)).toBe(false);
  });
});

describe('Kacheln ohne Server (B-032)', () => {
  it('alle Spiel-Kacheln brauchen den Server (AC-01)', () => {
    const play = PAGES.filter((p) => p.section === 'play');
    expect(play.length).toBeGreaterThan(0);
    expect(play.every(needsServer)).toBe(true);
  });

  it('Testseiten und Infoseiten bleiben ohne Server nutzbar (AC-02)', () => {
    const rest = PAGES.filter((p) => p.section !== 'play');
    expect(rest.map((p) => p.href)).toEqual(expect.arrayContaining(['gamepad-test.html', 'aufstellung.html', 'figuren.html', 'grafiken.html', 'lizenzen.html']));
    expect(rest.some(needsServer)).toBe(false);
  });
});
