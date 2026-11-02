import { describe, expect, it } from 'vitest';
import type { AssetSlot, SourceProposal } from '../api';
import {
  activeChip, BUILTIN_CHIPS, chipCount, facetGroups, facetsOf, fileGroup, fileItem, groupItems, hasFacets, matches, noFacets, quellenOf, slotGroup, slotItem, suggestName,
  type Facets, type Item,
} from './filter';
import { buildRows } from './model';

// B-677 (AC-02, AC-03): Facetten (je Gruppe „oder“, zwischen Gruppen „und“), Suche, feste Chips, Gruppen.
const item = (o: Partial<Item> = {}): Item => ({ art: 'grafik', kategorie: 'gebaeude', status: 'aktiv', stufen: ['spiel'], quellen: ['Kenney'], marken: ['frei'], text: 'taverne pixel', ...o });
const facets = (o: Partial<Facets>): Facets => ({ ...noFacets(), ...o });

describe('Treffer: Facetten und Suche (B-677)', () => {
  it('trifft ohne Facetten und Suche jeden Eintrag', () => {
    expect(matches(item(), noFacets(), '')).toBe(true);
  });

  it('verbindet die Werte einer Gruppe mit „oder“', () => {
    const f = facets({ kategorie: ['figuren', 'gebaeude'] });
    expect(matches(item({ kategorie: 'gebaeude' }), f, '')).toBe(true);
    expect(matches(item({ kategorie: 'figuren' }), f, '')).toBe(true);
    expect(matches(item({ kategorie: 'hud' }), f, '')).toBe(false);
  });

  it('verbindet verschiedene Gruppen mit „und“', () => {
    const f = facets({ art: ['grafik'], status: ['luecke'] });
    expect(matches(item({ art: 'grafik', status: 'luecke' }), f, '')).toBe(true);
    expect(matches(item({ art: 'grafik', status: 'aktiv' }), f, '')).toBe(false);
    expect(matches(item({ art: 'sound', status: 'luecke' }), f, '')).toBe(false);
  });

  it('trifft bei Stufe, Quelle und Lizenz, sobald einer der Werte des Eintrags passt (Slot mit mehreren Dateien)', () => {
    const it1 = item({ stufen: ['netz', 'vorrat'], quellen: ['Kenney', 'freesound'], marken: ['frei', 'gesperrt'] });
    expect(matches(it1, facets({ stufe: ['vorrat'] }), '')).toBe(true);
    expect(matches(it1, facets({ stufe: ['spiel'] }), '')).toBe(false);
    expect(matches(it1, facets({ quelle: ['freesound'] }), '')).toBe(true);
    expect(matches(it1, facets({ lizenz: ['gesperrt'] }), '')).toBe(true);
    expect(matches(it1, facets({ lizenz: ['sa'] }), '')).toBe(false);
  });

  it('verlangt bei der Suche alle Wörter, ohne Rücksicht auf Groß- und Kleinschreibung', () => {
    expect(matches(item(), noFacets(), 'TAVERNE pixel')).toBe(true);
    expect(matches(item(), noFacets(), '  taverne   ')).toBe(true);
    expect(matches(item(), noFacets(), 'taverne wald')).toBe(false);
  });

  it('wirkt Suche und Facetten zusammen', () => {
    expect(matches(item({ art: 'sound' }), facets({ art: ['grafik'] }), 'taverne')).toBe(false);
    expect(matches(item({ art: 'grafik' }), facets({ art: ['grafik'] }), 'taverne')).toBe(true);
  });

  it('lässt mit skip eine Gruppe aus', () => {
    const f = facets({ art: ['sound'], status: ['aktiv'] });
    expect(matches(item({ art: 'grafik' }), f, '')).toBe(false);
    expect(matches(item({ art: 'grafik' }), f, '', 'art')).toBe(true);
    expect(matches(item({ art: 'grafik', status: 'luecke' }), f, '', 'art')).toBe(false);
  });
});

describe('Facetten-Zähler (B-677)', () => {
  const items = [item({ art: 'grafik', status: 'aktiv' }), item({ art: 'grafik', status: 'luecke' }), item({ art: 'sound', status: 'luecke', kategorie: 'sfx', quellen: ['freesound'] })];
  const group = (g: ReturnType<typeof facetGroups>, id: string) => g.find((x) => x.id === id)!;
  const count = (g: ReturnType<typeof facetGroups>, id: string, value: string) => group(g, id).options.find((o) => o.value === value)?.count;

  it('liefert die Gruppen in fester Reihenfolge', () => {
    expect(facetGroups(items, noFacets(), '', []).map((g) => g.id)).toEqual(['art', 'kategorie', 'status', 'stufe', 'quelle', 'lizenz']);
  });

  it('zählt je Option die passenden Einträge', () => {
    const g = facetGroups(items, noFacets(), '', ['Kenney', 'freesound']);
    expect(count(g, 'art', 'grafik')).toBe(2);
    expect(count(g, 'art', 'sound')).toBe(1);
    expect(count(g, 'status', 'luecke')).toBe(2);
    expect(count(g, 'status', 'fehlt')).toBe(0);
    expect(count(g, 'quelle', 'freesound')).toBe(1);
  });

  it('zählt eine Gruppe ohne ihre eigene Auswahl, aber mit den übrigen Gruppen', () => {
    const g = facetGroups(items, facets({ status: ['luecke'], art: ['grafik'] }), '', []);
    expect(count(g, 'art', 'sound')).toBe(1); // eigene Auswahl (grafik) zählt nicht, Status Lücke schon
    expect(count(g, 'status', 'aktiv')).toBe(1); // eigener Status zählt nicht, Art grafik schon
  });

  it('beachtet die Suche in den Zählern', () => {
    const g = facetGroups([item({ text: 'baum' }), item({ text: 'haus' })], noFacets(), 'haus', []);
    expect(count(g, 'art', 'grafik')).toBe(1);
  });

  it('beschriftet die Optionen mit den Wörtern der Seite', () => {
    const g = facetGroups(items, noFacets(), '', []);
    expect(group(g, 'status').options.map((o) => o.label)).toEqual(['aktiv', 'Lücke', 'Datei fehlt']);
    expect(group(g, 'stufe').options.map((o) => o.label)).toEqual(['Netz', 'Vorrat', 'Spiel']);
    expect(group(g, 'lizenz').options.map((o) => o.label)).toEqual(['CC0/BY', 'SA', 'prüfen', 'gesperrt']);
    expect(group(g, 'art').options.map((o) => o.label)).toEqual(['Grafik', 'Sound']);
  });

  it('bietet alle Kategorien beider Arten an; Quellen nur mit Treffern, häufigste zuerst, gewählte immer', () => {
    const g = facetGroups(items, noFacets(), '', ['A', 'B']);
    expect(group(g, 'kategorie').options.map((o) => o.value)).toEqual(['figuren', 'gebaeude', 'umgebung', 'objekte', 'hud', 'sfx', 'musik', 'ambient']);
    expect(group(g, 'quelle').options).toEqual([]); // keine Zeile hat die Quellen A oder B
    const withB = facetGroups(items, { ...noFacets(), quelle: ['B'] }, '', ['A', 'B']);
    expect(group(withB, 'quelle').options.map((o) => o.value)).toEqual(['B']);
    const many = [item({ quellen: ['Kenney'] }), item({ quellen: ['Kenney'] }), item({ quellen: ['freesound'] })];
    expect(group(facetGroups(many, noFacets(), '', ['freesound', 'Kenney', 'leer']), 'quelle').options.map((o) => [o.value, o.count])).toEqual([['Kenney', 2], ['freesound', 1]]);
  });
});

describe('Fokus-Chips (B-677)', () => {
  const lib = [
    item({ art: 'grafik', status: 'aktiv', stufen: ['spiel'] }),
    item({ art: 'grafik', status: 'luecke', stufen: ['netz'] }),
    item({ art: 'sound', status: 'fehlt', stufen: ['vorrat'] }),
    item({ art: 'sound', status: 'aktiv', stufen: ['spiel'], marken: ['pruefen'], text: 'ton' }),
  ];
  const chip = (id: string) => BUILTIN_CHIPS.find((c) => c.id === id)!;

  it('kennt die sieben festen Chips', () => {
    expect(BUILTIN_CHIPS.map((c) => c.id)).toEqual(['alles', 'grafik', 'sound', 'luecken', 'netz', 'pruefen', 'vorrat']);
  });

  it('zählt Lücken als Status Lücke oder Datei fehlt', () => {
    expect(chip('luecken').facets).toEqual({ status: ['luecke', 'fehlt'] });
    expect(chipCount(lib, chip('luecken'), '')).toBe(2);
  });

  it('zählt je Chip die Einträge seiner Facetten', () => {
    expect(chipCount(lib, chip('alles'), '')).toBe(4);
    expect(chipCount(lib, chip('grafik'), '')).toBe(2);
    expect(chipCount(lib, chip('sound'), '')).toBe(2);
    expect(chipCount(lib, chip('netz'), '')).toBe(1);
    expect(chipCount(lib, chip('vorrat'), '')).toBe(1);
    expect(chipCount(lib, chip('pruefen'), '')).toBe(1);
  });

  it('zählt je Chip bei der Suche', () => {
    expect(chipCount(lib, chip('sound'), 'ton')).toBe(1);
    expect(chipCount(lib, chip('grafik'), 'ton')).toBe(0);
  });

  it('erkennt den Chip, dessen Facetten genau den aktuellen entsprechen', () => {
    expect(activeChip(noFacets(), BUILTIN_CHIPS)).toBe('alles');
    expect(activeChip(facets({ status: ['fehlt', 'luecke'] }), BUILTIN_CHIPS)).toBe('luecken');
    expect(activeChip(facets({ art: ['grafik'] }), BUILTIN_CHIPS)).toBe('grafik');
  });

  it('erkennt keinen Chip, wenn die Facetten von allen abweichen', () => {
    expect(activeChip(facets({ art: ['grafik'], status: ['luecke'] }), BUILTIN_CHIPS)).toBe('');
    expect(activeChip(facets({ status: ['luecke'] }), BUILTIN_CHIPS)).toBe('');
  });

  it('füllt die Facetten eines Chips mit leeren Gruppen auf', () => {
    expect(facetsOf(chip('grafik'))).toEqual(facets({ art: ['grafik'] }));
  });

  it('erkennt, ob irgendeine Facette gewählt ist', () => {
    expect(hasFacets(noFacets())).toBe(false);
    expect(hasFacets(facets({ lizenz: ['sa'] }))).toBe(true);
  });
});

describe('Name für „Ansicht merken“ (B-677)', () => {
  it('nennt eine Ansicht ohne Facetten „Ansicht“', () => {
    expect(suggestName(noFacets(), [])).toBe('Ansicht');
  });

  it('setzt die gewählten Werte in Reihenfolge der Gruppen zusammen', () => {
    const name = suggestName(facets({ status: ['luecke'], art: ['grafik'] }), []);
    expect(name.indexOf('grafik')).toBeGreaterThanOrEqual(0);
    expect(name.indexOf('grafik')).toBeLessThan(name.indexOf('luecke'));
  });

  it('zählt hoch, wenn der Name schon vergeben ist', () => {
    expect(suggestName(noFacets(), ['Ansicht'])).toBe('Ansicht 2');
    expect(suggestName(noFacets(), ['Ansicht', 'Ansicht 2'])).toBe('Ansicht 3');
  });
});

describe('Gruppen (B-677)', () => {
  it('gibt ohne Gruppierung eine Gruppe mit allen Einträgen und keine bei leerer Liste', () => {
    expect(groupItems([1, 2], null)).toEqual([{ key: '', title: '', items: [1, 2] }]);
    expect(groupItems([], null)).toEqual([]);
  });

  it('gruppiert alphabetisch und behält die Reihenfolge der Einträge', () => {
    const groups = groupItems(['zebra', 'Apfel', 'ziege', 'Ast'], (s) => s[0]!.toUpperCase());
    expect(groups.map((g) => g.key)).toEqual(['A', 'Z']);
    expect(groups.map((g) => g.items)).toEqual([['Apfel', 'Ast'], ['zebra', 'ziege']]);
  });

  const slots: AssetSlot[] = [
    { id: 'farm', art: 'grafik', kategorie: 'gebaeude', beschreibung: 'Gebäude: farm', status: 'zugeordnet', wahl: [{ datei: 'grafik/town/well.png' }] },
    { id: 'tavern', art: 'grafik', kategorie: 'gebaeude', beschreibung: 'Gebäude: tavern', status: 'luecke', wahl: [] },
  ];
  const proposal: SourceProposal = {
    id: 'q1', slot: 'tavern', art: 'grafik', titel: 'Taverne', quelle: 'Kenney', seite: '', dateiUrl: '', lizenz: 'CC0 1.0', lizenzErlaubt: true, dateiname: 'q1.png', groesse: 100,
    prioritaet: 1, status: 'offen', verwendet: false,
  };
  const rows = buildRows(slots, [proposal], [{ datei: 'grafik/town/well.png', art: 'grafik', pack: 'town', lizenz: 'CC0 1.0' }]);

  it('gruppiert Slots nach Kategorie, Spielbereich, Quelle oder Pack und bei Slot gar nicht', () => {
    const farm = rows.slots[0]!;
    expect(slotGroup('kategorie')!(farm)).toBe('gebaeude');
    expect(slotGroup('bereich')!(farm)).toBe('Gebäude');
    expect(slotGroup('quelle')!(farm)).toBe('–'); // der Katalog-Eintrag der Fixture nennt keine Quelle
    expect(slotGroup('pack')!(farm)).toBe('town');
    expect(slotGroup('slot')).toBeNull();
  });

  it('gruppiert Dateien nach Slot, Kategorie, Spielbereich, Quelle oder Pack', () => {
    const [well, offer] = [rows.files[0]!, rows.files[1]!];
    expect(fileGroup('slot')(offer)).toBe('tavern');
    expect(fileGroup('kategorie')(offer)).toBe('gebaeude');
    expect(fileGroup('quelle')(offer)).toBe('Kenney');
    expect(fileGroup('pack')(well)).toBe('town');
  });

  it('nimmt als Spielbereich einer Datei den Teil der Slot-ID vor dem Doppelpunkt', () => {
    const row = { ...rows.files[0]!, slotId: 'sfx:skill:healer' };
    expect(fileGroup('bereich')(row)).toBe('sfx');
  });

  it('gruppiert Dateien ohne Kategorie unter einem Strich', () => {
    expect(fileGroup('kategorie')({ ...rows.files[0]!, kategorie: '' })).toBe('–');
  });
});

describe('Einträge aus Slots und Dateien (B-677)', () => {
  const slots: AssetSlot[] = [{ id: 'tavern', art: 'grafik', kategorie: 'gebaeude', beschreibung: 'Gebäude: tavern', status: 'luecke', wahl: [{ datei: 'grafik/town/Well.png' }] }];
  const proposals: SourceProposal[] = ['a', 'b'].map((id) => ({
    id, slot: 'tavern', art: 'grafik', titel: `Titel ${id}`, quelle: 'Kenney', seite: '', dateiUrl: '', lizenz: 'CC0 1.0', lizenzErlaubt: true, dateiname: `${id}.png`, groesse: 10, prioritaet: 1,
    status: id === 'a' ? 'geladen' : 'offen', verwendet: false, vorschau: '/vorrat/a.png',
  }));
  const rows = buildRows(slots, proposals, []);

  it('fasst bei einem Slot die Werte seiner Dateien ohne Dubletten zusammen', () => {
    const it1 = slotItem(rows.slots[0]!);
    expect([...it1.stufen].sort()).toEqual(['netz', 'spiel', 'vorrat']);
    expect([...it1.quellen].sort()).toEqual(['Kenney', '–'].sort());
    expect(it1).toMatchObject({ art: 'grafik', kategorie: 'gebaeude', status: 'luecke' });
  });

  it('durchsucht beim Slot Beschreibung, Dateien und Titel der Vorschläge in Kleinbuchstaben', () => {
    const text = slotItem(rows.slots[0]!).text;
    expect(text).toContain('gebäude: tavern');
    expect(text).toContain('well.png');
    expect(text).toContain('titel a');
  });

  it('liefert bei einer Datei je einen Wert und durchsucht Slot, Titel, Quelle und Lizenz', () => {
    const offer = rows.files.find((f) => f.key === 'sug:b')!;
    const it1 = fileItem(offer);
    expect(it1).toMatchObject({ stufen: ['netz'], quellen: ['Kenney'], marken: ['frei'], status: 'luecke' });
    expect(it1.text).toContain('tavern');
    expect(it1.text).toContain('titel b');
    expect(it1.text).toContain('cc0 1.0');
  });

  it('sammelt die Quellen aller Dateien sortiert und ohne Dubletten', () => {
    expect(quellenOf(rows.files)).toEqual(['–', 'Kenney']);
  });
});
