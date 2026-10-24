import { beforeEach, describe, expect, it } from 'vitest';
import { httpBackend } from './http';
import { mockSuggestions, resetMockSuggestions, setMockSuggestionsOffline } from './mockSuggestions';

// WZ30.5: Mock der Chat-Vorschläge und Stubs des HTTP-Backends (WZ29.7).
beforeEach(resetMockSuggestions);

describe('Mock der Vorschläge', () => {
  it('liefert mehrere Vorschläge, einen mit Lizenzverstoß, und filtert nach Slot', async () => {
    const api = mockSuggestions();
    const all = await api.assetSuggestions('');
    expect(all.offline).toBe(false);
    expect(all.vorschlaege.length).toBeGreaterThan(3);
    expect(all.vorschlaege.filter((s) => !s.lizenzErlaubt).every((s) => s.lizenzGrund)).toBe(true);
    expect((await api.assetSuggestions('tavern')).vorschlaege.map((s) => s.id)).toEqual(['g1', 'g2']);
  });

  it('verweigert Lizenzverstoß und Offline beim Übernehmen', async () => {
    const api = mockSuggestions();
    await expect(api.assetSuggestionAdopt('sfx:skill:shield', 's3')).rejects.toThrow('Lizenz nicht erlaubt');
    await api.assetSuggestionAdopt('music:abend', 's1');
    expect((await api.assetSuggestions('')).vorschlaege.some((s) => s.id === 's1')).toBe(false);
    setMockSuggestionsOffline(true);
    expect(await api.assetSuggestions('')).toEqual({ offline: true, vorschlaege: [] });
    await expect(api.assetSuggestionAdopt('tavern', 'g1')).rejects.toThrow('offline');
  });
});

describe('HTTP-Backend', () => {
  it('meldet die Vorschläge als „kommt mit WZ29.7“', async () => {
    const b = httpBackend();
    await expect(b.assetSuggestions('')).rejects.toThrow('WZ29.7');
    await expect(b.assetSuggestionAdopt('a', 'b')).rejects.toThrow('WZ29.7');
    await expect(b.assetSuggestionDrop('a', 'b')).rejects.toThrow('WZ29.7');
  });
});
