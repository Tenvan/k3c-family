import { describe, expect, it } from 'vitest';
import { signVariant } from './buildSignView';

describe('signVariant', () => {
  it('Mauer, Turm und Tor tragen das Verteidigungs-Schild', () => {
    for (const k of ['wall', 'tower', 'gate'] as const) expect(signVariant(k)).toBe('defense');
  });
  it('Treppen tragen das Treppen-Schild', () => {
    expect(signVariant('stairsUp')).toBe('stairs');
    expect(signVariant('stairsDown')).toBe('stairs');
  });
  it('übrige Gebäude tragen das Siedlungs-Schild', () => {
    for (const k of ['workshop', 'storage', 'farm', 'barracks', 'tavern', 'healer', 'smithy', 'armory'] as const) expect(signVariant(k)).toBe('settlement');
  });
});
