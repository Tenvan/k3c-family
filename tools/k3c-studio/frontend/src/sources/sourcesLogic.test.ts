import { beforeEach, describe, expect, it } from 'vitest';
import { mockSources, resetMockSources, setMockSourcesOffline } from '../api/mockSources';
import { archiveLabel, downloadable, formatBytes, percent, previewKind, running, summarize } from './sourcesLogic';

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

describe('previewKind und archiveLabel (B-673)', () => {
  const g = (vorschau?: string) => ({ art: 'grafik' as const, vorschau });
  it('nur Bild-Endungen sind Bilder, auch groß und mit Query', () => {
    for (const f of ['a.png', 'a.JPG', 'a.jpeg', 'a.gif', 'a.webp', 'x/a.png?v=1']) expect(previewKind(g(f))).toBe('bild');
  });
  it('Archive und Unbekanntes sind keine Bilder, Ton bleibt Ton', () => {
    for (const f of ['a.zip', 'a.7z', 'a', '']) expect(previewKind(g(f))).toBe('archiv');
    expect(previewKind({ art: 'sound', vorschau: 'a.ogg' })).toBe('ton');
  });
  it('Archiv-Text nennt die Endung', () => {
    expect(archiveLabel('pack.zip')).toBe('Archiv (zip)');
    expect(archiveLabel('pack.7z')).toBe('Archiv (7z)');
    expect(archiveLabel('pack')).toBe('Archiv');
  });
});
