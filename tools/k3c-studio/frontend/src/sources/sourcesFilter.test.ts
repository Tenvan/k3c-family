import { beforeEach, describe, expect, it } from 'vitest';
import { mockSources, resetMockSources } from '../api/mockSources';
import { applyFilter, DEFAULT_FILTER, extraFilters, groupBySlot, isSourceFilter, searchPrompt, type SourceFilter } from './sourcesFilter';

// B-673/AC-02: Reiter, Art, Status und Zusatzfilter wirken zusammen; die Liste ist nach Slot gruppiert (Lücke/belegt).
beforeEach(resetMockSources);

const all = async () => (await mockSources().sources()).vorschlaege;
const ids = (items: { id: string }[]) => items.map((i) => i.id);
const f = (p: Partial<SourceFilter>): SourceFilter => ({ ...DEFAULT_FILTER, ...p });

describe('Filter der Quellen', () => {
  it('verbindet Reiter, Art und Status', async () => {
    const list = await all();
    expect(ids(applyFilter(list, f({ tab: 'Kenney' })))).toEqual(['q05', 'q08']);
    expect(ids(applyFilter(list, f({ art: 'sound', status: 'geladen' })))).toEqual(['q02']); // q01 ist verwendet
    expect(ids(applyFilter(list, f({ tab: 'OpenGameArt', art: 'grafik', status: 'offen' })))).toEqual([]); // q07 gesperrt, q09 gesperrt
    expect(ids(applyFilter(list, f({ status: 'verwendet' })))).toEqual(['q01']);
  });

  it('blendet gesperrte aus, bis sie unter Filter gezeigt werden', async () => {
    const list = await all();
    expect(ids(applyFilter(list, f({}))).filter((i) => ['q04', 'q07', 'q09'].includes(i))).toEqual([]);
    expect(ids(applyFilter(list, f({ gesperrte: true, tab: 'OpenGameArt', art: 'grafik' })))).toEqual(['q07', 'q09', 'q10']);
    expect(extraFilters(f({ slot: 'tavern', gesperrte: true }))).toBe(2);
  });

  it('sucht alle Wörter und filtert nach Slot und Lizenz', async () => {
    const list = await all();
    expect(ids(applyFilter(list, f({ query: 'taverne pixel' })))).toEqual(['q05']);
    expect(ids(applyFilter(list, f({ slot: 'tavern', lizenz: 'CC BY-SA 3.0' })))).toEqual(['q06']);
  });

  it('gruppiert nach Slot, Lücken zuerst; belegt ist ein Slot mit verwendeter Datei', async () => {
    const list = await all();
    const groups = groupBySlot(applyFilter(list, f({})), list);
    expect(groups.map((g) => [g.slot, g.gap])).toEqual([['beetle', true], ['music:abend', true], ['pickaxe', true], ['tavern', true], ['sfx:skill:healer', false]]);
    expect(ids(groups.find((g) => g.slot === 'sfx:skill:healer')!.items)).toEqual(['q01', 'q02']); // nach Priorität
    // belegt wird über alle Einträge bestimmt, nicht nur über die sichtbaren
    expect(groupBySlot(applyFilter(list, f({ status: 'geladen' })), list).find((g) => g.slot === 'sfx:skill:healer')?.gap).toBe(false);
  });

  it('prüft den gemerkten Filter und baut den Suchauftrag', () => {
    expect(isSourceFilter(DEFAULT_FILTER)).toBe(true);
    expect(isSourceFilter({ ...DEFAULT_FILTER, art: 'x' })).toBe(false);
    expect(isSourceFilter(null)).toBe(false);
    expect(searchPrompt('Kenney', 'tavern', 'grafik')).toContain('Grafiken von Kenney für den Slot tavern');
    expect(searchPrompt('', '', 'alle')).toContain('alle Lücken');
  });
});
