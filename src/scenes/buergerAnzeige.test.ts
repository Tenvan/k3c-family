import { afterEach, beforeEach, describe, expect, it } from 'vitest';
import { UNIT_PX } from '../core/constants';
import { setLanguage } from '../core/texts';
import { BUILDINGS } from '../model/data';
import { healerRanges, limitText, merchantText, professionLabel, professionSummary } from './buergerAnzeige';

beforeEach(() => setLanguage('de'));
afterEach(() => setLanguage('de'));

describe('Limit-Text (AC-01, B-126)', () => {
  it('Kämpfer/Limit aus dem Zustand, lang und kurz', () => {
    expect(limitText({ fighters: 12, troopLimit: 20 }, false)).toBe('Kämpfer 12 von 20');
    expect(limitText({ fighters: 12, troopLimit: 20 }, true)).toBe('Limit 12/20');
    expect(limitText({ fighters: 0, troopLimit: 10 }, true)).toBe('Limit 0/10');
  });

  it('Limit erreicht → Hinweis', () => {
    expect(limitText({ fighters: 20, troopLimit: 20 }, false)).toBe('Kämpfer 20 von 20: Limit erreicht');
    expect(limitText({ fighters: 20, troopLimit: 20 }, true)).toBe('Limit 20/20 voll');
  });

  it('Feld fehlt (älterer Server) → null', () => {
    expect(limitText({}, false)).toBeNull();
    expect(limitText({ fighters: 3 }, false)).toBeNull();
  });
});

describe('Berufsanzeige (AC-01, B-126)', () => {
  it('Name und Symbolname je Beruf, lang und kurz', () => {
    expect(professionLabel('miner', false)).toEqual({ icon: 'prof-miner', text: 'Bergmann' });
    expect(professionLabel('builder', false)).toEqual({ icon: 'prof-builder', text: 'Baumeister' });
    expect(professionLabel('craftsman', false)).toEqual({ icon: 'prof-craftsman', text: 'Handwerker' });
    expect(professionLabel('miner', true)?.text).toBe('Bergm.');
  });

  it('unbekannter Beruf → neutraler Text, kein Beruf → null', () => {
    expect(professionLabel('alchemist', false)).toEqual({ icon: 'prof-unknown', text: 'Beruf' });
    expect(professionLabel(undefined, false)).toBeNull();
  });

  it('Zusammenfassung zählt je Beruf in fester Reihenfolge', () => {
    const troops = [{ profession: 'craftsman' }, {}, { profession: 'miner' }, { profession: 'miner' }, { profession: 'alchemist' }];
    expect(professionSummary(troops, false)).toEqual(['Bergmann 2', 'Handwerker 1', 'Beruf 1']);
    expect(professionSummary(troops, true)).toEqual(['Bergm. 2', 'Handw. 1', 'Beruf 1']);
    expect(professionSummary(undefined, false)).toEqual([]);
  });

  it('englisch', () => {
    setLanguage('en');
    expect(professionLabel('builder', false)?.text).toBe('Builder');
  });
});

describe('Händler (B-126)', () => {
  it('anwesend: Angebot und Abreisetag, laufender Kauf in der langen Form', () => {
    expect(merchantText({ merchant: { resource: 'stone', leaves: 7 } }, false)).toBe('Händler: Stein · bis Tag 7');
    expect(merchantText({ merchant: { resource: 'iron', leaves: 7, buyPaid: 3 } }, false)).toBe('Händler: Eisen · bis Tag 7 · Kauf 3 Gold');
    expect(merchantText({ merchant: { resource: 'stone', leaves: 7, buyPaid: 3 } }, true)).toBe('Händler Stein');
  });

  it('nicht anwesend → null', () => {
    expect(merchantText({}, false)).toBeNull();
  });
});

describe('Heilplatz (B-126)', () => {
  const radius = (BUILDINGS.healer as unknown as { heal: { radiusUnits: number } }).heal.radiusUnits;

  it('Reichweite in Pixeln aus Units, nur gebaute Heilplätze', () => {
    const sites = [
      { kind: 'healer' as const, x: 10, state: 'built' as const },
      { kind: 'healer' as const, x: 30, state: 'waitingMaterial' as const },
      { kind: 'wall' as const, x: 5, state: 'built' as const },
    ];
    expect(healerRanges({ sites })).toEqual([{ x: 10 * UNIT_PX, radiusPx: radius * UNIT_PX }]);
  });

  it('ohne Bauplätze → leer', () => {
    expect(healerRanges({})).toEqual([]);
  });
});
