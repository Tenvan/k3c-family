import { describe, expect, it } from 'vitest';
import { createFullSave, fetchLevel, fullSaveTarget, levelUrl, type FetchLike } from './levelApi';
import { t } from './texts';

const SERVER_UNREACHABLE = t('level.unreachable');
const INVALID_ANSWER = t('level.invalid');

const answer = (status: number, body: unknown): FetchLike => async () => ({ ok: status >= 200 && status < 300, status, json: async () => body });

describe('levelUrl', () => {
  it('kodiert Seed und Biom, nichts wird in die URL eingeschleust', () => {
    expect(levelUrl('probe', 'forest')).toBe('/api/level?seed=probe&biome=forest');
    expect(levelUrl('a b&c=d#e', 'x?y')).toBe('/api/level?seed=a%20b%26c%3Dd%23e&biome=x%3Fy');
    expect(levelUrl('', 'forest')).toBe('/api/level?seed=&biome=forest');
  });
});

describe('fetchLevel (B-092/AC-04)', () => {
  it('Erfolg: das Level aus der Antwort', async () => {
    const level = { seed: 'test', biomeId: 'forest', widthUnits: 1100, chunks: [], entities: [], warnings: [] };
    expect(await fetchLevel('test', 'forest', answer(200, level))).toEqual({ level });
  });

  it('fragt die kodierte URL ab', async () => {
    const urls: string[] = [];
    await fetchLevel('a b', 'cave', async (url) => {
      urls.push(url);
      return { ok: true, status: 200, json: async () => ({}) };
    });
    expect(urls).toEqual(['/api/level?seed=a%20b&biome=cave']);
  });

  it('Server nicht erreichbar: Hinweis zum Starten statt Ausnahme', async () => {
    const down: FetchLike = async () => {
      throw new TypeError('Failed to fetch');
    };
    expect(await fetchLevel('test', 'forest', down)).toEqual({ error: SERVER_UNREACHABLE });
    expect(SERVER_UNREACHABLE).toContain('task start');
  });

  it('HTTP 400: die Meldung des Servers', async () => {
    expect(await fetchLevel('x', 'xyz', answer(400, { error: 'Unbekanntes Biom "xyz" (bekannt: cave, forest, mine)' }))).toEqual({
      error: 'Unbekanntes Biom "xyz" (bekannt: cave, forest, mine)',
    });
  });

  it('andere Fehler ohne lesbare Meldung: Statuscode', async () => {
    expect(await fetchLevel('x', 'forest', answer(500, {}))).toEqual({ error: 'Fehler 500' });
    expect(await fetchLevel('x', 'forest', answer(502, null))).toEqual({ error: 'Fehler 502' });
    const noJson: FetchLike = async () => ({ ok: false, status: 404, json: async () => Promise.reject(new SyntaxError('kein JSON')) });
    expect(await fetchLevel('x', 'forest', noJson)).toEqual({ error: 'Fehler 404' });
  });

  it('kaputte Antwort bei 200: allgemeiner Hinweis', async () => {
    const noJson: FetchLike = async () => ({ ok: true, status: 200, json: async () => Promise.reject(new SyntaxError('kein JSON')) });
    expect(await fetchLevel('x', 'forest', noJson)).toEqual({ error: INVALID_ANSWER });
    expect(await fetchLevel('x', 'forest', answer(200, null))).toEqual({ error: INVALID_ANSWER });
    expect(await fetchLevel('x', 'forest', answer(200, 'text'))).toEqual({ error: INVALID_ANSWER });
  });
});

describe('createFullSave (SV1.3/SV1.4, B-315)', () => {
  it('schickt den Seed per POST an die Dev-API und liefert den Spielstandnamen', async () => {
    const calls: { url: string; init?: RequestInit }[] = [];
    const result = await createFullSave('abc', async (url, init) => {
      calls.push({ url, init });
      return { ok: true, status: 200, json: async () => ({ save: 'voll-abc' }) };
    });
    expect(result).toEqual({ save: 'voll-abc' });
    expect(calls).toHaveLength(1);
    expect(calls[0]!.url).toBe('/api/dev/fullsave');
    expect(calls[0]!.init?.method).toBe('POST');
    expect(JSON.parse(String(calls[0]!.init?.body))).toEqual({ seed: 'abc' });
  });

  it('ohne Dev-Mode (403): erkennbarer Hinweis statt Start', async () => {
    expect(await createFullSave('abc', answer(403, { error: 'forbidden' }))).toEqual({ error: t('level.fullForbidden') });
  });

  it('Seed, dessen Spielstandname die Lobby nicht laden kann, fragt gar nicht erst', async () => {
    let asked = false;
    const spy: FetchLike = async () => {
      asked = true;
      return { ok: true, status: 200, json: async () => ({ save: 'x' }) };
    };
    expect(await createFullSave('A_b', spy)).toEqual({ error: t('level.fullBadSeed') });
    expect(await createFullSave('a'.repeat(28), spy)).toEqual({ error: t('level.fullBadSeed') });
    expect(asked).toBe(false);
  });

  it('Server nicht erreichbar oder unerwartete Antwort: Hinweis', async () => {
    expect(await createFullSave('abc', async () => { throw new TypeError('down'); })).toEqual({ error: SERVER_UNREACHABLE });
    expect(await createFullSave('abc', answer(200, { nope: 1 }))).toEqual({ error: t('level.fullInvalid') });
  });
});

describe('fullSaveTarget', () => {
  it('lädt den vorhandenen Spielstand per Autostart, ohne fresh', () => {
    expect(fullSaveTarget('voll-abc')).toBe('game.html?autostart=1&save=voll-abc');
  });
});
