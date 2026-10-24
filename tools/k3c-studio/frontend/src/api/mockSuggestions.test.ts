import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { httpBackend } from './http';
import { mockSuggestions, resetMockSuggestions, setMockSuggestionsOffline } from './mockSuggestions';

// WZ30.5: Mock der Chat-Vorschläge; WZ29.7: RPC des HTTP-Backends.
beforeEach(resetMockSuggestions);
afterEach(() => vi.unstubAllGlobals());

describe('Mock der Vorschläge', () => {
  it('liefert mehrere Vorschläge, einen mit Lizenzverstoß, und filtert nach Slot', async () => {
    const api = mockSuggestions();
    const all = await api.assetSuggestions('');
    expect(all.offline).toBe(false);
    expect(all.vorschlaege.length).toBeGreaterThan(3);
    expect(all.vorschlaege.filter((s) => !s.lizenzErlaubt).every((s) => s.lizenzGrund)).toBe(true);
    expect((await api.assetSuggestions('tavern')).vorschlaege.map((s) => s.id)).toEqual(['g1', 'g2']);
  });

  it('verweigert Lizenzverstoß, fehlenden Vorrat, alten Stand und Offline beim Übernehmen', async () => {
    const api = mockSuggestions();
    const { stand } = await api.assetSuggestions('');
    await expect(api.assetSuggestionAdopt('sfx:skill:shield', 's3', stand)).rejects.toThrow('Lizenz nicht erlaubt');
    await expect(api.assetSuggestionAdopt('tavern', 'g2', stand)).rejects.toThrow('Quellen');
    await expect(api.assetSuggestionAdopt('music:abend', 's1', 'alt')).rejects.toThrow('Register geändert');
    const next = await api.assetSuggestionAdopt('music:abend', 's1', stand);
    expect(next).not.toBe(stand);
    expect((await api.assetSuggestions('')).vorschlaege.some((s) => s.id === 's1')).toBe(false);
    setMockSuggestionsOffline(true);
    expect(await api.assetSuggestions('')).toMatchObject({ offline: true, vorschlaege: [] });
    await expect(api.assetSuggestionAdopt('tavern', 'g1', next)).rejects.toThrow('offline');
  });
});

describe('HTTP-Backend', () => {
  it('ruft die RPC der Vorschläge mit den Argumenten als Array', async () => {
    const calls: { url: string; body: string }[] = [];
    vi.stubGlobal('fetch', async (url: string, init: RequestInit) => {
      calls.push({ url, body: String(init.body) });
      return new Response(JSON.stringify({ result: url.endsWith('Adopt') ? 'neu' : { offline: false, stand: 's', vorschlaege: [] } }));
    });
    const b = httpBackend();
    expect(await b.assetSuggestions('tavern')).toEqual({ offline: false, stand: 's', vorschlaege: [] });
    expect(await b.assetSuggestionAdopt('tavern', 'g1', 's')).toBe('neu');
    await b.assetSuggestionDrop('tavern', 'g2');
    expect(calls).toEqual([
      { url: '/api/studio/rpc/AssetSuggestions', body: '["tavern"]' },
      { url: '/api/studio/rpc/AssetSuggestionAdopt', body: '["tavern","g1","s"]' },
      { url: '/api/studio/rpc/AssetSuggestionDrop', body: '["tavern","g2"]' },
    ]);
  });

  it('meldet den Fehler des Backends (Lizenz, Stand) als Ausnahme', async () => {
    vi.stubGlobal('fetch', async () => new Response(JSON.stringify({ error: 'Lizenz nicht erlaubt: NC' }), { status: 422 }));
    await expect(httpBackend().assetSuggestionAdopt('a', 'b', 's')).rejects.toThrow('Lizenz nicht erlaubt');
  });
});
