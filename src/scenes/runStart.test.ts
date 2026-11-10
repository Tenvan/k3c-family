import { afterEach, describe, expect, it } from 'vitest';
import { resetRun, runSearch } from '../core/runConfig';
import type { RoomClient } from '../online/clientConnection';
import { parseStartParams } from './lobbyLogic';
import { startRun } from './runStart';

afterEach(() => resetRun());

const client = {} as RoomClient;

describe('Szenario aus dem Menü starten (B-768/AC-06)', () => {
  it('setzt die Konfiguration und startet die Lobby neu, ohne die Seite zu laden', () => {
    const started: { key: string; data?: object }[] = [];
    startRun({ scene: { start: (key, data) => started.push({ key, data }) } }, client, { dev: '1', autostart: '1', fresh: '1', save: 'stress-wald', seed: 'stress', players: '2' });
    expect(started).toEqual([{ key: 'lobby', data: { client } }]);
    expect(runSearch()).toBe('?dev=1&autostart=1&fresh=1&save=stress-wald&seed=stress&players=2');
  });

  it('die Lobby liest daraus dieselben Werte wie aus einer URL', () => {
    startRun({ scene: { start: () => undefined } }, client, { autostart: '1', fresh: '1', save: 'stress-mine', world: 'mine', players: '2' });
    expect(parseStartParams(runSearch())).toMatchObject({ autostart: true, fresh: true, save: 'stress-mine', world: 'mine', players: 2 });
  });

  it('ein neues Szenario erbt nichts vom vorigen', () => {
    startRun({ scene: { start: () => undefined } }, client, { autostart: '1', save: 'a', seed: 'x' });
    startRun({ scene: { start: () => undefined } }, client, { dev: '1' });
    expect(runSearch()).toBe('?dev=1');
  });
});
