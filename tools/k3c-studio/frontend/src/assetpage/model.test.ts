import { describe, expect, it } from 'vitest';
import type { AssetSlot, CatalogAsset, SourceProposal } from '../api';
import { buildRows, stageCounts, stageLabel } from './model';

// B-677 (AC-03): Zeilen der Sichten Slots und Dateien aus Register, Katalog und Vorschlagsliste.
const TOWN = 'grafik/town';
const farm: AssetSlot = { id: 'farm', art: 'grafik', kategorie: 'gebaeude', beschreibung: 'Gebäude: farm', status: 'zugeordnet', wahl: [{ datei: `${TOWN}/well.png` }, { datei: `${TOWN}/wagon.png` }] };
const tavern: AssetSlot = { id: 'tavern', art: 'grafik', kategorie: 'gebaeude', beschreibung: 'Gebäude: tavern', status: 'luecke', wahl: [] };
const coin: AssetSlot = { id: 'sfx:coin', art: 'sound', kategorie: 'sfx', beschreibung: 'Münze', status: 'zugeordnet', wahl: [{ datei: 'audio/atlas.ogg', sprite: 'coin' }] };
const slots = [farm, tavern, coin];

const catalog: CatalogAsset[] = [
  { datei: `${TOWN}/well.png`, art: 'grafik', pack: 'town', quelle: 'Kenney', lizenz: 'CC0 1.0' },
  { datei: `${TOWN}/wagon.png`, art: 'grafik', pack: 'plants', lizenz: 'CC BY-NC 4.0', kennzeichen: 'NC' },
];

const prop = (id: string, o: Partial<SourceProposal> = {}): SourceProposal => ({
  id, slot: 'tavern', art: 'grafik', titel: `Titel ${id}`, quelle: 'Kenney', seite: `https://example.org/${id}`, dateiUrl: `https://example.org/${id}/datei`, lizenz: 'CC0 1.0',
  lizenzErlaubt: true, dateiname: `${id}.png`, groesse: 24_500, prioritaet: 2, status: 'offen', verwendet: false, ...o,
});

const proposals: SourceProposal[] = [
  prop('offen'),
  prop('geladen', { status: 'geladen', vorschau: '/vorrat/geladen.png' }),
  prop('verwendet', { status: 'geladen', verwendet: true, vorschau: '/vorrat/verwendet.png' }),
  prop('verworfen', { status: 'verworfen' }),
  prop('gesperrt', { status: 'gesperrt', lizenzErlaubt: false, lizenz: 'CC BY-ND 4.0', sperrgrund: 'ND erlaubt keinen Zuschnitt (Q70)' }),
  prop('unerlaubt', { lizenzErlaubt: false, lizenz: 'CC BY-NC 4.0' }),
];

const build = () => buildRows(slots, proposals, catalog);
const file = (key: string) => build().files.find((f) => f.key === key)!;

describe('Zeilen der Assets-Seite: Dateien (B-677)', () => {
  it('führt Zuordnungen mit Präferenz n und Stufe Spiel vor den Vorschlägen auf', () => {
    expect(build().files.map((f) => f.key)).toEqual(['farm#0', 'farm#1', 'sfx:coin#0', 'sug:offen', 'sug:geladen', 'sug:gesperrt', 'sug:unerlaubt']);
    expect(file('farm#0')).toMatchObject({ stufe: 'spiel', praef: 1, titel: 'well.png', slotId: 'farm', kategorie: 'gebaeude', status: 'aktiv', ladbar: false });
    expect(file('farm#1')).toMatchObject({ stufe: 'spiel', praef: 2, titel: 'wagon.png' });
  });

  it('nimmt Quelle, Pack und Lizenz-Marke einer Zuordnung aus dem Katalog', () => {
    expect(file('farm#0')).toMatchObject({ quelle: 'Kenney', pack: 'town', lizenz: 'CC0 1.0', marke: 'frei', vorschau: `${TOWN}/well.png` });
    expect(file('farm#1')).toMatchObject({ quelle: '–', pack: 'plants', marke: 'gesperrt' }); // ohne Quelle im Katalog zählt der Pack nicht als Anbieter
  });

  it('zeigt eine Zuordnung ohne Katalog-Eintrag mit Strich und Marke prüfen', () => {
    expect(file('sfx:coin#0')).toMatchObject({ quelle: '–', pack: '–', lizenz: '', marke: 'pruefen', praef: 1 });
  });

  it('macht aus einem offenen Vorschlag eine Netz-Zeile ohne Vorschau, die sich laden lässt', () => {
    expect(file('sug:offen')).toMatchObject({ stufe: 'netz', praef: 0, proposalId: 'offen', vorschau: '', ladbar: true, marke: 'frei', slotId: 'tavern', status: 'luecke', kategorie: 'gebaeude' });
  });

  it('macht aus einem geladenen Vorschlag eine Vorrat-Zeile mit Vorschau, die nicht mehr zu laden ist', () => {
    expect(file('sug:geladen')).toMatchObject({ stufe: 'vorrat', vorschau: '/vorrat/geladen.png', ladbar: false });
  });

  it('lässt verwendete und verworfene Vorschläge aus', () => {
    const keys = build().files.map((f) => f.key);
    expect(keys).not.toContain('sug:verwendet');
    expect(keys).not.toContain('sug:verworfen');
  });

  it('sperrt Vorschläge mit nicht erlaubter Lizenz samt Grund', () => {
    expect(file('sug:gesperrt')).toMatchObject({ marke: 'gesperrt', ladbar: false, grund: 'ND erlaubt keinen Zuschnitt (Q70)' });
    expect(file('sug:unerlaubt')).toMatchObject({ marke: 'gesperrt', ladbar: false, grund: 'Lizenz CC BY-NC 4.0 nicht erlaubt (Q70)' });
  });

  it('gibt erlaubten Vorschlägen keinen Grund', () => {
    expect(file('sug:offen').grund).toBeUndefined();
  });

  it('behandelt einen Vorschlag zu einem unbekannten Slot als Lücke ohne Kategorie', () => {
    const rows = buildRows(slots, [prop('wild', { slot: 'portal' })], catalog);
    expect(rows.files.find((f) => f.key === 'sug:wild')).toMatchObject({ status: 'luecke', kategorie: '', slotId: 'portal' });
    expect(rows.slots.flatMap((s) => s.files).map((f) => f.key)).not.toContain('sug:wild');
  });
});

describe('Zeilen der Assets-Seite: Slots (B-677)', () => {
  const slotRow = (id: string) => build().slots.find((s) => s.slot.id === id)!;

  it('bildet je Slot eine Zeile in Reihenfolge des Registers', () => {
    expect(build().slots.map((s) => s.slot.id)).toEqual(['farm', 'tavern', 'sfx:coin']);
  });

  it('übersetzt den Status in die Wörter der Seite', () => {
    expect(slotRow('farm').status).toBe('aktiv');
    expect(slotRow('tavern').status).toBe('luecke');
    expect(buildRows([{ ...tavern, status: 'fehlt' }], [], []).slots[0]!.status).toBe('fehlt');
  });

  it('fasst die Dateien des Slots zusammen, Zuordnungen vor Vorschlägen', () => {
    expect(slotRow('farm').files.map((f) => f.key)).toEqual(['farm#0', 'farm#1']);
    expect(slotRow('tavern').files.map((f) => f.key)).toEqual(['sug:offen', 'sug:geladen', 'sug:gesperrt', 'sug:unerlaubt']);
  });

  it('übernimmt Quelle und Pack von der aktiven Zuordnung (Präferenz 1)', () => {
    expect(slotRow('farm')).toMatchObject({ quelle: 'Kenney', pack: 'town' });
  });

  it('zeigt Quelle und Pack ohne aktive Zuordnung mit Strich', () => {
    expect(slotRow('tavern')).toMatchObject({ quelle: '–', pack: '–' });
  });

  it('leitet den Spielbereich aus der Beschreibung ab', () => {
    expect(slotRow('farm').bereich).toBe('Gebäude');
    expect(slotRow('sfx:coin').bereich).toBe('Sonstige');
  });
});

describe('Stufen (B-677)', () => {
  it('zählt Vorschläge je Stufe für die Zeile eines Slots', () => {
    const row = build().slots.find((s) => s.slot.id === 'tavern')!;
    expect(stageCounts(row)).toEqual({ netz: 3, vorrat: 1 });
  });

  it('zählt Zuordnungen nicht mit', () => {
    const row = build().slots.find((s) => s.slot.id === 'farm')!;
    expect(stageCounts(row)).toEqual({ netz: 0, vorrat: 0 });
  });

  it('beschriftet Zuordnungen mit der Präferenz', () => {
    expect(stageLabel({ stufe: 'spiel', praef: 1 })).toBe('Präf. 1');
    expect(stageLabel({ stufe: 'spiel', praef: 3 })).toBe('Präf. 3');
  });

  it('beschriftet nicht zugeordnete Dateien mit ihrer Stufe', () => {
    expect(stageLabel({ stufe: 'netz', praef: 0 })).toBe('Netz');
    expect(stageLabel({ stufe: 'vorrat', praef: 0 })).toBe('Vorrat');
    expect(stageLabel({ stufe: 'spiel', praef: 0 })).toBe('Spiel');
  });
});
