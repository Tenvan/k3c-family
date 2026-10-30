import { afterEach, beforeEach, describe, expect, it } from 'vitest';
import { setLanguage } from '../../core/texts';
import type { Site, World } from '../../model/types';
import { buildCost, hubOfferTitle, hubText, hubWait, siteWait, stockTexts, waitInfo } from './economyLabels';

beforeEach(() => setLanguage('de'));
afterEach(() => setLanguage('de'));

const site = (over: Partial<Site>): Pick<Site, 'kind' | 'state' | 'upgrade' | 'level'> => ({ kind: 'storage', state: 'unpaid', ...over });
type StockWorld = Parameters<typeof stockTexts>[0];
const stockWorld = (stock: Record<string, number>, stockMax?: number, primary = 'wood'): StockWorld =>
  ({ stock, stockMax, biome: { primaryResource: primary } }) as unknown as StockWorld;

describe('Wartegrund (AC-01, B-117)', () => {
  it('Bauer fehlt, Gefahr, Material mit Rohstoffname, kein Grund', () => {
    expect(waitInfo('waitingWorker', false, [], false)).toEqual({ reason: 'worker', icon: 'wait-worker', text: 'wartet auf Bauer' });
    expect(waitInfo('waitingWorker', true, [], false)).toEqual({ reason: 'danger', icon: 'wait-danger', text: 'wartet: Gefahr' });
    expect(waitInfo('waitingMaterial', false, ['stone'], false)).toEqual({ reason: 'material', icon: 'wait-material', text: 'wartet auf Material (Stein)' });
    expect(waitInfo('unpaid', false, ['stone'], false)).toBeNull();
    expect(waitInfo('built', true, [], false)).toBeNull();
    expect(waitInfo(undefined, true, [], false)).toBeNull();
  });

  it('kurze Form', () => {
    expect(waitInfo('waitingMaterial', false, ['stone'], true)?.text).toBe('Material fehlt');
    expect(waitInfo('waitingWorker', false, [], true)?.text).toBe('kein Bauer');
    expect(waitInfo('waitingWorker', true, [], true)?.text).toBe('Gefahr');
  });

  it('Material ohne bekannte Rohstoffe → allgemeiner Text', () => {
    expect(waitInfo('waitingMaterial', false, [], false)?.text).toBe('wartet auf Material');
  });

  it('Bauplatz: Bau nennt das Material der Daten, Ausbau das der nächsten Stufe', () => {
    expect(siteWait(site({ state: 'waitingMaterial' }), false, false)?.text).toBe('wartet auf Material (Stein)');
    expect(siteWait(site({ kind: 'wall', state: 'built', upgrade: 'waitingMaterial' }), false, false)?.text).toBe('wartet auf Material (Stein)');
    expect(siteWait(site({ kind: 'wall', state: 'built', level: 3, upgrade: 'waitingMaterial' }), false, false)?.text).toBe('wartet auf Material (Eisen)');
    expect(siteWait(site({ kind: 'wall', state: 'built', upgrade: 'waitingWorker' }), true, false)?.reason).toBe('danger');
    expect(siteWait(site({ state: 'built' }), true, false)).toBeNull();
  });

  it('Hub-Ausbau: Wartegrund aus hubUpgrade.state, ohne Ausbau oder bei offenem Gold → null', () => {
    const up = { gold: 40, paid: 40, material: { stone: 100, iron: 20 }, state: 'waitingMaterial' as const };
    expect(hubWait({ hubUpgrade: up }, false)?.text).toBe('wartet auf Material (Stein, Eisen)');
    expect(hubWait({ hubUpgrade: { ...up, state: undefined } }, false)).toBeNull();
    expect(hubWait({}, false)).toBeNull();
  });
});

describe('Lagerstand (AC-01, B-117)', () => {
  it('Vorrat/Maximum, nur Rohstoffe mit Vorrat oder der Primärrohstoff', () => {
    expect(stockTexts(stockWorld({ wood: 0, stone: 40, copper: 0 }, 300), false)).toEqual(['Holz 0/300', 'Stein 40/300']);
  });

  it('voll bei Vorrat = Maximum, lang und kurz', () => {
    expect(stockTexts(stockWorld({ wood: 300, stone: 0, copper: 0 }, 300), false)).toEqual(['Holz 300/300 voll']);
    expect(stockTexts(stockWorld({ wood: 300, stone: 10, copper: 0 }, 300), true)).toEqual(['Holz voll', 'Stein 10/300']);
  });

  it('fünf Rohstoffe in fester Reihenfolge, Eisen und Kristall mit Namen', () => {
    const texts = stockTexts(stockWorld({ crystal: 5, iron: 7, copper: 3, stone: 2, wood: 1 }, 600), false);
    expect(texts).toEqual(['Holz 1/600', 'Stein 2/600', 'Kupfer 3/600', 'Eisen 7/600', 'Kristall 5/600']);
  });

  it('Primärrohstoff fehlt im Vorrat (Eisen bei 0 nicht gesendet) → steht mit 0 da', () => {
    expect(stockTexts(stockWorld({ wood: 0, stone: 0, copper: 4 }, 300, 'iron'), false)).toEqual(['Kupfer 4/300', 'Eisen 0/300']);
  });

  it('Maximum fehlt oder 0 → nur die Menge', () => {
    expect(stockTexts(stockWorld({ wood: 12, stone: 0, copper: 0 }), false)).toEqual(['Holz 12']);
    expect(stockTexts(stockWorld({ wood: 12, stone: 0, copper: 0 }, 0), true)).toEqual(['Holz 12']);
  });

  it('englisch', () => {
    setLanguage('en');
    expect(stockTexts(stockWorld({ wood: 0, iron: 300 }, 300), false)).toEqual(['Wood 0/300', 'Iron 300/300 full']);
  });

  it('Zustand ohne Vorrat → leer, kein Fehler', () => {
    expect(stockTexts({} as StockWorld, false)).toEqual([]);
  });
});

describe('Hub-Stufe mit Kosten (AC-01, B-117)', () => {
  const up: NonNullable<World['hubUpgrade']> = { gold: 40, paid: 15, material: { iron: 20, stone: 100 } };

  it('nächste Stufe mit Gold (bezahlt/gesamt) und Material, lang und kurz', () => {
    expect(hubText({ hubLevel: 2, hubUpgrade: up }, false)).toBe('Hub-Stufe 2 → 3: 15/40 Gold, 100 Stein, 20 Eisen');
    expect(hubText({ hubLevel: 2, hubUpgrade: up }, true)).toBe('Hub 2→3: 15/40 Gold, 100 Stein, 20 Eisen');
  });

  it('höchste Stufe (kein Ausbau) → keine Kosten', () => {
    expect(hubText({ hubLevel: 5 }, false)).toBe('Hub-Stufe 5');
    expect(hubText({ hubLevel: 5 }, true)).toBe('Hub 5');
  });

  it('älterer Server ohne hubLevel → null', () => {
    expect(hubText({}, false)).toBeNull();
  });
});

describe('Ausbauschritt und Material der nächsten Insel (GEB17.5, B-592/AC-03)', () => {
  const up: NonNullable<World['hubUpgrade']> = { gold: 25, paid: 25, material: { stone: 50, copper: 20 }, steps: 3 };

  it('HUD nennt den Schritt, lang und kurz', () => {
    expect(hubText({ hubLevel: 1, hubStep: 2, hubUpgrade: { ...up, paid: 0 } }, false)).toBe('Hub-Stufe 1 → 2, Ausbau 2/3: 0/25 Gold, 50 Stein, 20 Kupfer');
    expect(hubText({ hubLevel: 1, hubStep: 2, hubUpgrade: { ...up, paid: 0 } }, true)).toBe('Hub 1→2 2/3: 0/25 Gold, 50 Stein, 20 Kupfer');
  });

  it('Preisschild der Burg: „Ausbau 2/3“, ohne Schritt (älterer Server) „Ausbauen“', () => {
    expect(hubOfferTitle({ hubStep: 2, hubUpgrade: up })).toBe('Ausbau 2/3');
    expect(hubOfferTitle({ hubUpgrade: { ...up, steps: undefined } })).toBe('Ausbauen');
    expect(hubOfferTitle({})).toBe('Ausbauen');
  });

  it('Wartegrund waitingNextIsland: „Tor-Material fehlt“, lang und kurz, Symbol Material', () => {
    const wait = hubWait({ hubUpgrade: { ...up, state: 'waitingNextIsland' } }, false);
    expect(wait).toEqual({ reason: 'material', icon: 'wait-material', text: 'Tor-Material fehlt (Stein, Kupfer)' });
    expect(hubWait({ hubUpgrade: { ...up, state: 'waitingNextIsland' } }, true)?.text).toBe('Tor-Material');
  });

  it('Material-Tor nennt nur, was im Vorrat fehlt (Review GEB17)', () => {
    const wait = hubWait({ hubUpgrade: { ...up, state: 'waitingNextIsland' }, stock: { stone: 80 } as never }, false);
    expect(wait?.text).toBe('Tor-Material fehlt (Kupfer)');
  });

  it('englisch', () => {
    setLanguage('en');
    expect(hubOfferTitle({ hubStep: 3, hubUpgrade: up })).toBe('Upgrade 3/3');
    expect(hubWait({ hubUpgrade: { ...up, state: 'waitingNextIsland' } }, false)?.text).toBe('gate material missing (Stone, Copper)');
    expect(hubText({ hubLevel: 1, hubStep: 3, hubUpgrade: up }, false)).toBe('Hub level 1 → 2, step 3/3: 25/25 gold, 50 Stone, 20 Copper');
  });
});

describe('Treppen-Kosten je Tiefe (B-631/AC-02)', () => {
  it('Treppe runter: Material der eigenen Stufe, Treppe hoch: Material der Stufe darüber, je 10 Gold', () => {
    expect(buildCost('stairsDown', 0)).toEqual({ wood: 100, gold: 10 });
    expect(buildCost('stairsDown', 2)).toEqual({ copper: 100, gold: 10 });
    expect(buildCost('stairsUp', 1)).toEqual({ wood: 100, gold: 10 });
    expect(buildCost('stairsUp', 4)).toEqual({ iron: 100, gold: 10 });
  });

  it('ohne Eintrag der Tiefe oder ohne Tiefe: cost der Art', () => {
    expect(buildCost('stairsUp', 0)).toEqual({ stone: 100, gold: 10 });
    expect(buildCost('stairsDown', undefined)).toEqual({ stone: 100, gold: 10 });
    expect(buildCost('nursery', 3)).toEqual({ wood: 30, gold: 3 });
  });

  it('Wartegrund am Platz nennt das Material der Tiefe', () => {
    expect(siteWait(site({ kind: 'stairsUp', state: 'waitingMaterial' }), false, false, 1)?.text).toBe('wartet auf Material (Holz)');
  });
});
