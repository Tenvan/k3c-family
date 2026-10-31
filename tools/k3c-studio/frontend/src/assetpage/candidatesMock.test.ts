// @vitest-environment happy-dom
import { beforeEach, describe, expect, it } from 'vitest';
import type { AssetCandidate, AssetHit } from '../api';
import { isStale, STALE } from '../api/stale';
import { mockBackend } from '../api/mock';
import { mockAssets } from '../api/mockAssets';
import { mockCandidates } from '../api/mockCandidates';
import { mockSources, resetMockSources, setMockSourcesOffline } from '../api/mockSources';

// B-677 (AC-05, AC-06): Kandidaten und Zuordnen im Mock, gleiche Regeln wie das Go-Backend. Jeder Test bekommt ein frisches Register
// (mockBackend()) und die zurückgesetzte Vorschlagsliste.
let b: ReturnType<typeof mockBackend>;
beforeEach(() => {
  resetMockSources();
  setMockSourcesOffline(false);
  b = mockBackend();
});

const view = (slot: string, query = '', limit = 60) => b.assetCandidates(slot, query, limit);
const ids = (list: AssetCandidate[]) => list.map((c) => c.id);
const find = (list: AssetCandidate[], id: string) => list.find((c) => c.id === id)!;
const stand = async () => (await b.assetsList()).stand;
const wahl = async (slot: string) => (await b.assetsList()).slots.find((s) => s.id === slot)!.wahl;

/** Lädt einen Netz-Vorschlag in den Vorrat (Download, Fortschritt bis fertig), ohne ihn zuzuordnen. */
async function toVorrat(id: string, bytes: number): Promise<void> {
  await b.sourceDownload([id], bytes);
  for (let i = 0; i < 5; i += 1) await b.sourceProgress();
}

describe('Kandidaten eines Slots (B-677/AC-05)', () => {
  it('stellt die Zuordnungen in Reihenfolge der Präferenz voran', async () => {
    const { kandidaten } = await view('farm', 'props');
    expect(kandidaten.slice(0, 2).map((c) => [c.id, c.stufe, c.praef, c.zuordenbar])).toEqual([
      ['wahl:grafik/gothicvania-town/props-einzeln/well.png', 'spiel', 1, false],
      ['wahl:grafik/gothicvania-town/props-einzeln/wagon.png', 'spiel', 2, false],
    ]);
    expect(kandidaten.slice(2).every((c) => c.praef === 0)).toBe(true);
  });

  it('hängt Katalog-Treffer ohne die schon zugeordneten Dateien an und beachtet das Limit', async () => {
    const all = await view('tavern', 'png', 60);
    const cat = all.kandidaten.filter((c) => c.id.startsWith('cat:'));
    expect(cat.length).toBeGreaterThanOrEqual(3);
    expect((await view('tavern', 'png', 2)).kandidaten.filter((c) => c.id.startsWith('cat:'))).toHaveLength(2);
    const farmCat = (await view('farm', 'png', 60)).kandidaten.filter((c) => c.id.startsWith('cat:')).map((c) => c.datei);
    expect(farmCat).not.toContain('grafik/gothicvania-town/props-einzeln/well.png');
  });

  it('stellt geladene Vorschläge (Vorrat) vor die offenen (Netz) und beide vor den Katalog', async () => {
    await toVorrat('q05', 24_500);
    const { kandidaten } = await view('tavern', 'png');
    const order = kandidaten.map((c) => c.stufe);
    expect(ids(kandidaten).slice(0, 2)).toEqual(['sug:q05', 'sug:q06']);
    expect(order.slice(0, 2)).toEqual(['vorrat', 'netz']);
    expect(find(kandidaten, 'sug:q05').vorschau).not.toBe('');
    expect(find(kandidaten, 'sug:q06').vorschau).toBe('');
    expect(ids(kandidaten).slice(2).every((id) => id.startsWith('cat:'))).toBe(true);
  });

  it('lässt verwendete Vorschläge weg (sie erscheinen als Zuordnung)', async () => {
    const healer = await view('sfx:skill:healer');
    expect(ids(healer.kandidaten)).not.toContain('sug:q01');
    expect(ids(healer.kandidaten)).toContain('sug:q02');
  });

  it('führt Treffer der Recherche mit gleicher Datei-URL mit dem Vorschlag zusammen', async () => {
    const hit: AssetHit = { id: 'h1', titel: 'Taverne Treffer', link: 'https://example.org/h1', quelle: 'Kenney', lizenz: 'CC0 1.0', lizenzStatus: 'erkannt', erstellt: '2026-10-14', download: 'https://example.org/q05/datei' };
    const other: AssetHit = { ...hit, id: 'h2', download: 'https://example.org/andere/datei' };
    const api = mockCandidates({ ...mockAssets(() => undefined), assetsHits: async () => [hit, other] }, mockSources());
    const { kandidaten } = await api.assetCandidates('tavern', '', 60);
    expect(ids(kandidaten)).not.toContain('hit:h1');
    expect(find(kandidaten, 'sug:q05').herkunft).toEqual(['vorschlag', 'recherche']);
    expect(find(kandidaten, 'hit:h2')).toMatchObject({ stufe: 'netz', herkunft: ['recherche'], zuordenbar: true });
  });

  it('zeigt Recherche-Treffer mit unklarer Lizenz als prüfen, bestätigbar und nicht zuordenbar; ausgeschlossene fehlen', async () => {
    await b.assetsResearch('tavern', '');
    const { kandidaten } = await view('tavern');
    expect(find(kandidaten, 'hit:c-tavern')).toMatchObject({ marke: 'sa', zuordenbar: true, stufe: 'netz' });
    expect(find(kandidaten, 'hit:o-tavern')).toMatchObject({ marke: 'pruefen', zuordenbar: false, lizenzBestaetigen: true, grund: 'Lizenz prüfen' });
    expect(ids(kandidaten).some((id) => id.startsWith('hit:p-'))).toBe(false);
  });

  it('sperrt Vorschläge mit nicht erlaubter Lizenz samt Grund', async () => {
    const q07 = find((await view('portal')).kandidaten, 'sug:q07');
    expect(q07).toMatchObject({ marke: 'gesperrt', zuordenbar: false });
    expect(q07.grund).toContain('ND');
  });

  it('sperrt Katalog-Dateien mit NC-Lizenz wie das Backend (Q70)', async () => {
    const flower = find((await view('tavern', 'flower')).kandidaten, 'cat:grafik/plants/flower.png');
    expect(flower.marke).toBe('gesperrt');
    expect(flower.zuordenbar).toBe(false);
    expect(flower.grund).toBeTruthy();
  });

  it('sperrt im Offline-Betrieb alles aus dem Netz, nicht aber den Vorrat', async () => {
    await toVorrat('q05', 24_500);
    await b.setNetwork('offline');
    const v = await view('tavern');
    expect(v.offline).toBe(true);
    expect(find(v.kandidaten, 'sug:q06')).toMatchObject({ zuordenbar: false, grund: 'offline' });
    expect(find(v.kandidaten, 'sug:q05').zuordenbar).toBe(true);
  });

  it('lehnt einen unbekannten Slot ab', async () => {
    await expect(view('gibt-es-nicht')).rejects.toThrow(/unbekannter Asset-Slot/);
  });
});

describe('Zuordnen (B-677/AC-06)', () => {
  it('lädt einen Netz-Kandidaten, übernimmt ihn und hängt ihn als letzte Präferenz an', async () => {
    const v = await view('tavern');
    expect(find(v.kandidaten, 'sug:q05').stufe).toBe('netz');
    const saved = await b.assetCandidateAssign('tavern', 'sug:q05', v.stand, 24_500);
    expect(saved.slot.wahl).toHaveLength(1);
    expect(saved.stand).not.toBe(v.stand);
    expect(saved.stand).toBe(await stand());
    expect(await wahl('tavern')).toHaveLength(1);
    const after = (await view('tavern')).kandidaten;
    expect(ids(after)).not.toContain('sug:q05');
    expect(after[0]).toMatchObject({ praef: 1, stufe: 'spiel' });
  });

  it('ordnet einen Vorrat-Kandidaten ohne Bestätigung zu; die Wahl wächst um 1', async () => {
    const before = (await wahl('sfx:skill:healer')).length;
    const v = await view('sfx:skill:healer');
    expect(find(v.kandidaten, 'sug:q02').stufe).toBe('vorrat');
    await b.assetCandidateAssign('sfx:skill:healer', 'sug:q02', v.stand, 0);
    expect(await wahl('sfx:skill:healer')).toHaveLength(before + 1);
  });

  it('ordnet einen Katalog-Treffer zu, der nur über eine eigene Suche gefunden wird', async () => {
    const v = await view('storage', 'well');
    await b.assetCandidateAssign('storage', 'cat:grafik/gothicvania-town/props-einzeln/well.png', v.stand, 0);
    expect((await wahl('storage')).map((c) => c.datei)).toEqual(['grafik/gothicvania-town/props-einzeln/crate.png', 'grafik/gothicvania-town/props-einzeln/well.png']);
  });

  it('lehnt eine falsche Bestätigung der Größe ab und ordnet nichts zu', async () => {
    const v = await view('tavern');
    await expect(b.assetCandidateAssign('tavern', 'sug:q05', v.stand, 1)).rejects.toThrow(/bestätigt weicht von der Größe ab/);
    expect(await wahl('tavern')).toHaveLength(0);
    expect((await b.sources()).vorschlaege.find((p) => p.id === 'q05')).toMatchObject({ status: 'offen', verwendet: false });
  });

  it('lehnt einen veralteten Stand mit dem Text der Schreibregel ab', async () => {
    const v = await view('tavern');
    b.external('farm', []); // fremder Schreiber erhöht den Stand
    const err = await b.assetCandidateAssign('tavern', 'sug:q05', v.stand, 24_500).catch((e: unknown) => e);
    expect(err).toBeInstanceOf(Error);
    expect(isStale(err)).toBe(true);
    expect((err as Error).message).toContain(STALE);
    expect(await wahl('tavern')).toHaveLength(0);
    expect((await b.sources()).vorschlaege.find((p) => p.id === 'q05')?.status).toBe('offen');
  });

  it('lehnt einen gesperrten Kandidaten mit seinem Grund ab', async () => {
    const v = await view('portal');
    const before = await wahl('portal');
    await expect(b.assetCandidateAssign('portal', 'sug:q07', v.stand, 18_900)).rejects.toThrow(/ND erlaubt keinen Zuschnitt/);
    expect(await wahl('portal')).toEqual(before);
  });

  it('ordnet nichts zu, wenn der Download scheitert, und nennt den Fehler', async () => {
    const v = await view('tavern');
    await expect(b.assetCandidateAssign('tavern', 'sug:q06', v.stand, 31_200)).rejects.toThrow(/Verbindung abgebrochen/);
    expect(await wahl('tavern')).toHaveLength(0);
    expect((await b.sources()).vorschlaege.find((p) => p.id === 'q06')?.verwendet).toBe(false);
  });

  it('lehnt einen Netz-Kandidaten offline ab', async () => {
    const v = await view('tavern');
    await b.setNetwork('offline');
    await expect(b.assetCandidateAssign('tavern', 'sug:q05', v.stand, 24_500)).rejects.toThrow(/offline/);
    expect(await wahl('tavern')).toHaveLength(0);
  });

  it('lehnt einen unbekannten Kandidaten und einen unbekannten Slot ab', async () => {
    const v = await view('tavern');
    await expect(b.assetCandidateAssign('tavern', 'sug:gibtsnicht', v.stand, 0)).rejects.toThrow(/unbekannt/);
    await expect(b.assetCandidateAssign('gibt-es-nicht', 'sug:q05', v.stand, 0)).rejects.toThrow(/unbekannter Asset-Slot/);
  });
});
