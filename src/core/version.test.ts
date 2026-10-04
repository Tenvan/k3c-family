import { describe, expect, it } from 'vitest';
import { CLIENT, fetchServerBuild, formatBuild, versionLine, versionMismatch } from './version';

const reply = (status: number, body: string): typeof fetch => (async () => new Response(body, { status })) as unknown as typeof fetch;

describe('Versionsanzeige', () => {
  it('Tag und Buildzeit in deutscher Ortszeit', () => {
    expect(formatBuild({ version: 'v0.4.0', built: '2026-10-03T12:38:00Z' }, 'Europe/Berlin')).toBe('v0.4.0 (03.10.2026, 14:38)');
  });

  it('ohne oder mit ungültiger Buildzeit nur die Version', () => {
    expect(formatBuild({ version: 'dev', built: '' })).toBe('dev');
    expect(formatBuild({ version: 'dev', built: 'kaputt' })).toBe('dev');
  });

  it('Zeile mit Client und Server, Server fehlt → Strich', () => {
    const c = { version: 'v0.4.0', built: '' };
    expect(versionLine(c, { version: 'v0.3.0', built: '' })).toBe('Client v0.4.0 · Server v0.3.0');
    expect(versionLine(c, null)).toBe('Client v0.4.0 · Server –');
  });

  it('Abweichung nur bei erreichbarem Server mit anderer Version (AC-04)', () => {
    const c = { version: 'v0.4.0', built: '' };
    expect(versionMismatch(c, { version: 'v0.4.0', built: '' })).toBe(false);
    expect(versionMismatch(c, { version: 'v0.3.0', built: '' })).toBe(true);
    expect(versionMismatch(c, null)).toBe(false);
  });

  it('der Build setzt Version und Buildzeit des Clients', () => {
    expect(CLIENT.version.length).toBeGreaterThan(0);
    expect(Number.isNaN(new Date(CLIENT.built).getTime())).toBe(false);
  });
});

describe('fetchServerBuild', () => {
  it('liest version und built aus api/health', async () => {
    expect(await fetchServerBuild(reply(200, '{"ok":true,"version":"v0.4.0","built":"2026-10-03T12:00:00Z"}'))).toEqual({
      version: 'v0.4.0',
      built: '2026-10-03T12:00:00Z',
    });
  });

  it('älterer Server ohne Felder → erreichbar, Version unbekannt', async () => {
    expect(await fetchServerBuild(reply(200, '{"ok":true}'))).toEqual({ version: '?', built: '' });
  });

  it('kein Go-Server (404, HTML, Netzwerkfehler) → null', async () => {
    expect(await fetchServerBuild(reply(404, '<html>Not found</html>'))).toBeNull();
    expect(await fetchServerBuild(reply(200, '<html></html>'))).toBeNull();
    const down = (async () => {
      throw new TypeError('Failed to fetch');
    }) as unknown as typeof fetch;
    expect(await fetchServerBuild(down)).toBeNull();
  });
});
