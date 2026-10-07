import { afterEach, beforeEach, describe, expect, it } from 'vitest';
import { setLanguage } from '../core/texts';
import type { Site, World } from '../model/types';
import { hubText, hubWait, siteWait, stockTexts, waitInfo } from './wirtschaftAnzeige';

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
