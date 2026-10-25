import { beforeEach, describe, expect, it } from 'vitest';
import { mockSources, resetMockSources, setMockSourcesOffline } from '../api/mockSources';
import { buildView, emptyState } from '@webkit/lib/listFilter';
import { downloadable, formatBytes, percent, running, sourceFilter, summarize } from './sourcesLogic';

// WZ30.4 (AC-04): Logik der Seite Quellen und der Mock der Vorschlagsliste.
beforeEach(resetMockSources);

const view = () => mockSources().sources();
const drain = async (api: ReturnType<typeof mockSources>) => {
  for (let i = 0; i < 10; i++) if (!running(await api.sourceProgress())) return;
};

describe('Logik', () => {
  it('formatiert Größen und fasst die Auswahl nach Quellen zusammen', async () => {
    expect(formatBytes(86_000)).toBe('86 kB');
    expect(formatBytes(1_480_000)).toBe('1,5 MB');
    const { vorschlaege } = await view();
    const sel = summarize(vorschlaege, new Set(['q03', 'q05', 'q04', 'q02']));
    expect(sel.count).toBe(2); // q04 gesperrt, q02 schon geladen
    expect(sel.bytes).toBe(1_480_000 + 24_500);
    expect(sel.quellen.map((q) => q.name)).toEqual(['Kenney', 'OpenGameArt']);
  });

  it('filtert nach Quelle, Art und Status', async () => {
    const { vorschlaege } = await view();
    const by = (chips: Record<string, string[]>) => buildView(vorschlaege, sourceFilter, { ...emptyState('prio'), chips }).groups.flatMap((g) => g.items.map((i) => i.id));
    expect(by({ quelle: ['Kenney'] })).toEqual(['q05', 'q08']);
    expect(by({ art: ['sound'], status: ['geladen'] })).toEqual(['q01', 'q02']);
    expect(by({ verwendet: ['unverwendet'] }).sort()).toEqual(['q02', 'q10']);
  });

  it('läßt nur offene Einträge mit erlaubter Lizenz laden', async () => {
    const { vorschlaege } = await view();
    expect(vorschlaege.filter(downloadable).map((s) => s.id)).toEqual(['q03', 'q05', 'q06']);
  });
});

describe('Mock der Quellen', () => {
  it('zählt je Quelle vorgeschlagen, geladen und verwendet', async () => {
    const oga = (await view()).quellen.find((q) => q.name === 'OpenGameArt');
    expect(oga).toMatchObject({ vorgeschlagen: 5, geladen: 2, verwendet: 1 });
  });

  it('lädt mit Fortschritt und meldet den Fehlerfall; Wiederholen bleibt möglich', async () => {
    const api = mockSources();
    await api.sourceDownload(['q05', 'q06'], 0);
    const first = await api.sourceProgress();
    expect(first.map((j) => [j.id, j.status, percent(j)])).toEqual([['q05', 'laedt', 25], ['q06', 'laedt', 25]]);
    await drain(api);
    const done = await view();
    expect(done.vorschlaege.find((s) => s.id === 'q05')).toMatchObject({ status: 'geladen', vorschau: expect.any(String) });
    expect(done.vorschlaege.find((s) => s.id === 'q06')).toMatchObject({ status: 'offen', fehler: expect.stringContaining('abgebrochen') });
    expect(downloadable(done.vorschlaege.find((s) => s.id === 'q06')!)).toBe(true);
  });

  it('verweigert gesperrte Einträge und alles offline, zeigt die Liste aber weiter', async () => {
    const api = mockSources();
    await expect(api.sourceDownload(['q04'], 0)).rejects.toThrow('gesperrt');
    setMockSourcesOffline(true);
    await expect(api.sourceDownload(['q03'], 0)).rejects.toThrow('offline');
    const v = await api.sources();
    expect(v.offline).toBe(true);
    expect(v.vorschlaege.filter((s) => s.status === 'geladen')).toHaveLength(3);
  });

  it('räumt unverwendete geladene Dateien auf; Verwendetes bleibt', async () => {
    const api = mockSources();
    const preview = await api.sourceCleanupPreview();
    expect(preview.dateien.map((d) => d.id)).toEqual(['q02', 'q10']);
    expect(preview.groesse).toBe(142_000);
    await api.sourceCleanup();
    const v = await api.sources();
    expect(v.vorschlaege.find((s) => s.id === 'q01')?.status).toBe('geladen');
    expect(v.vorschlaege.find((s) => s.id === 'q02')?.status).toBe('offen');
    expect((await api.sourceCleanupPreview()).dateien).toEqual([]);
  });

  it('übernimmt nur Geladenes', async () => {
    const api = mockSources();
    await expect(api.sourceAdopt('q03')).rejects.toThrow('nicht geladen');
    await api.sourceAdopt('q02');
    expect((await api.sources()).vorschlaege.find((s) => s.id === 'q02')?.verwendet).toBe(true);
  });
});
