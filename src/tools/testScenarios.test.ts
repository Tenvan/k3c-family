import { describe, expect, it } from 'vitest';
import { isOpenable } from '../core/shell';
import { parseStartParams } from '../scenes/lobbyLogic';
import { SCENARIOS, saveName, scenarioUrl } from './testScenarios';

describe('Szenarien (B-081)', () => {
  it('gibt es für 1 bis 4 Spieler', () => {
    expect(SCENARIOS.map((s) => [s.title, s.players])).toEqual([['1 Spieler', 1], ['2 Spieler', 2], ['3 Spieler', 3], ['4 Spieler', 4]]);
  });

  it.each(SCENARIOS)('$title: URL mit mock = Spieler − 1, die Lobby versteht sie und die Shell öffnet sie', (s) => {
    const url = scenarioUrl(s, 'k3x9');
    expect(url).toBe(`game.html?autostart=1&fresh=1&save=test-k3x9&mock=${s.players - 1}`);
    expect(isOpenable(url)).toBe(true);
    expect(parseStartParams(url.slice(url.indexOf('?')))).toMatchObject({ autostart: true, fresh: true, save: 'test-k3x9', mock: s.players - 1, room: null });
  });

  it('der Spielstandname ist immer gültig und eindeutig je Kennung', () => {
    const valid = /^[a-z0-9-]{1,32}$/;
    for (const nonce of ['k3x9', 'ABC DEF!', '', '###', 'x'.repeat(100), 'ä€']) expect(saveName(nonce)).toMatch(valid);
    expect(saveName('ABC DEF!')).toBe('test-abcdef');
    expect(saveName('')).toBe('test-0');
    expect(saveName('x'.repeat(100))).toHaveLength(32);
    expect(saveName('a1')).not.toBe(saveName('a2'));
  });
});
