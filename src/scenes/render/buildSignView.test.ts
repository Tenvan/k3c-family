import { describe, expect, it } from 'vitest';
import { showsBuildSign, signVariant } from './buildSignView';

describe('signVariant', () => {
  it('Mauer, Turm und Tor tragen das Verteidigungs-Schild', () => {
    for (const k of ['wall', 'tower'] as const) expect(signVariant(k)).toBe('defense');
  });
  it('Treppen tragen das Treppen-Schild', () => {
    expect(signVariant('stairsUp')).toBe('stairs');
    expect(signVariant('stairsDown')).toBe('stairs');
  });
  it('übrige Gebäude tragen das Siedlungs-Schild', () => {
    for (const k of ['workshop', 'storage', 'farm', 'barracks', 'tavern', 'healer', 'smithy', 'armory'] as const) expect(signVariant(k)).toBe('settlement');
  });
});

describe('showsBuildSign (B-587/AC-02)', () => {
  it('Schild nur, wenn der Server payable meldet', () => {
    expect(showsBuildSign({ state: 'unpaid', payable: true })).toBe(true);
    expect(showsBuildSign({ state: 'unpaid', payable: false })).toBe(false);
  });
  it('ohne Feld (älterer Server) nie ein Schild', () => {
    expect(showsBuildSign({ state: 'unpaid' })).toBe(false);
  });
  it('gebaute Plätze zeigen kein Bauschild, auch wenn der Ausbau bezahlbar ist', () => {
    expect(showsBuildSign({ state: 'built', payable: true })).toBe(false);
  });
});
