import { describe, expect, it } from 'vitest';
import type { Status } from '../online/clientConnection';
import {
  LobbyFlow,
  applyCommand,
  entryLabel,
  gameNotice,
  leavesGame,
  lobbyEntries,
  lobbyNotice,
  moveSelection,
  parseStartParams,
  rowAt,
  slotsFor,
  type LobbyCommand,
  type StartParams,
} from './lobbyLogic';

const room = { code: 'KRNZ', name: 'familie', depth: 1, grade: 'normal', taken: 2, free: 2, running: true };
const lobby = { status: 'lobby' as Status, errorCode: null };
const params = (p: Partial<StartParams> = {}): StartParams => ({ autostart: false, fresh: false, save: 'familie', mock: 0, room: null, players: 1, ...p });

describe('Start-Parameter (AC-14, B-082/AC-01)', () => {
  it('gültige Werte', () => {
    expect(parseStartParams('?autostart=1&fresh=1&save=test-ab12&mock=3&room=krnz')).toEqual({
      autostart: true,
      fresh: true,
      save: 'test-ab12',
      mock: 3,
      room: 'KRNZ',
      players: 1,
    });
  });

  it('ohne Parameter gelten die Vorgaben', () => {
    expect(parseStartParams('')).toEqual(params());
  });

  it.each(['mock=7', 'mock=-1', 'mock=1.5', 'mock=', 'mock=abc'])('ignoriert ungültiges %s', (q) => {
    expect(parseStartParams(`?${q}`).mock).toBe(0);
  });

  it.each(['save=Ab%20C', 'save=UPPER', 'save=', `save=${'a'.repeat(33)}`, 'save=a_b'])('ignoriert ungültiges %s', (q) => {
    expect(parseStartParams(`?${q}`).save).toBe('familie');
  });

  it.each(['room=KRN', 'room=KRNO', 'room=KRNZZ'])('ignoriert ungültigen Code %s', (q) => {
    expect(parseStartParams(`?${q}`).room).toBeNull();
  });

  it('autostart und fresh nur mit 1; alte Parameter werden ignoriert', () => {
    const p = parseStartParams('?autostart=yes&fresh=true&continue=1&online=X&seed=5&depth=2');
    expect(p).toEqual(params());
  });
});

describe('create-Befehl mit slots[] (AC-14, B-082/AC-02)', () => {
  it.each([0, 1, 2, 3])('mock=%i → %i+1 Slots', (mock) => {
    const cmd = new LobbyFlow(params({ autostart: true, mock })).step(lobby);
    expect(cmd).toEqual({ t: 'create', save: 'familie', fresh: false, depth: 0, slots: [0, 1, 2, 3].slice(0, mock + 1) });
    expect(slotsFor(mock)).toHaveLength(mock + 1);
  });

  it('autostart mit fresh und save', () => {
    expect(new LobbyFlow(params({ autostart: true, fresh: true, save: 'test-1' })).step(lobby)).toEqual({
      t: 'create',
      save: 'test-1',
      fresh: true,
      depth: 0,
      slots: [0],
    });
  });

  it('ohne autostart und ohne room sendet die Lobby nichts von selbst', () => {
    expect(new LobbyFlow(params()).step(lobby)).toBeNull();
  });

  it('applyCommand ruft create bzw. join am Client auf', () => {
    const calls: unknown[][] = [];
    const client = { create: (...a: unknown[]) => calls.push(['create', ...a]), join: (...a: unknown[]) => calls.push(['join', ...a]) };
    applyCommand(client, { t: 'create', save: 's', fresh: true, depth: 0, slots: [0, 1] });
    applyCommand(client, { t: 'join', room: 'KRNZ', slots: [0] });
    expect(calls).toEqual([
      ['create', 's', true, 0, [0, 1]],
      ['join', 'KRNZ', [0]],
    ]);
  });
});

describe('Lobby-Auswahl (AC-03)', () => {
  it('„Spielen“ steht oben, danach die Räume', () => {
    expect(lobbyEntries([room]).map((e) => e.kind)).toEqual(['play', 'room']);
  });

  it('Auswahl bleibt in der Liste', () => {
    expect(moveSelection(0, -1, 3)).toBe(0);
    expect(moveSelection(0, 1, 3)).toBe(1);
    expect(moveSelection(2, 1, 3)).toBe(2);
  });

  it('Zeile unter dem Zeiger', () => {
    expect(rowAt(130, 120, 50, 3)).toBe(0);
    expect(rowAt(175, 120, 50, 3)).toBe(1);
    expect(rowAt(100, 120, 50, 3)).toBe(-1);
    expect(rowAt(400, 120, 50, 3)).toBe(-1);
  });

  it('„Spielen“ erstellt, ein Raum aus der Liste tritt bei', () => {
    const flow = new LobbyFlow(params({ mock: 1 }));
    const [play, entry] = lobbyEntries([room]);
    expect(flow.choose(play!)).toEqual({ t: 'create', save: 'familie', fresh: false, depth: 0, slots: [0, 1] });
    expect(flow.choose(entry!)).toEqual({ t: 'join', room: 'KRNZ', slots: [0, 1] });
  });

  it('?room=CODE tritt einmal automatisch bei', () => {
    const flow = new LobbyFlow(params({ room: 'KRNZ' }));
    expect(flow.step(lobby)).toEqual({ t: 'join', room: 'KRNZ', slots: [0] });
    expect(flow.step({ status: 'lobby', errorCode: 'room_not_found' })).toBeNull();
  });

  it('wartet, bis der Client in der Lobby ist', () => {
    expect(new LobbyFlow(params({ autostart: true })).step({ status: 'connecting', errorCode: null })).toBeNull();
  });
});

describe('save_not_found → zweiter Versuch (AC-03)', () => {
  const notFound = { status: 'lobby' as Status, errorCode: 'save_not_found' };

  it('genau ein Versuch mit fresh und depth 0', () => {
    const flow = new LobbyFlow(params({ save: 'neu' }));
    flow.choose({ kind: 'play' });
    const retry = flow.step(notFound);
    expect(retry).toEqual({ t: 'create', save: 'neu', fresh: true, depth: 0, slots: [0] });
    expect(flow.step(notFound)).toBeNull();
  });

  it('auch nach Autostart', () => {
    const flow = new LobbyFlow(params({ autostart: true }));
    flow.step(lobby);
    expect((flow.step(notFound) as LobbyCommand | null)?.t).toBe('create');
  });

  it('nicht nach einem Beitritt und nicht, wenn schon fresh', () => {
    const joined = new LobbyFlow(params({ room: 'KRNZ' }));
    joined.step(lobby);
    expect(joined.step(notFound)).toBeNull();
    const fresh = new LobbyFlow(params({ fresh: true }));
    fresh.choose({ kind: 'play' });
    expect(fresh.step(notFound)).toBeNull();
  });

  it('eine neue Auswahl darf wieder einmal wiederholen', () => {
    const flow = new LobbyFlow(params());
    flow.choose({ kind: 'play' });
    flow.step(notFound);
    flow.choose({ kind: 'play' });
    expect(flow.step(notFound)).not.toBeNull();
  });
});

describe('Hinweise (AC-09, AC-10)', () => {
  const codes = ['room_full', 'too_many_slots', 'too_many_rooms', 'room_not_found', 'save_exists', 'save_not_found', 'room_closed', 'replaced', 'version'];

  it.each(codes)('%s zeigt die Server-Meldung bzw. den Hinweistext', (code) => {
    const status: Status = code === 'replaced' || code === 'version' ? 'ended' : 'lobby';
    expect(lobbyNotice({ status, notice: `Text zu ${code}` })).toBe(`Text zu ${code}`);
  });

  it('Verbindungsstände', () => {
    expect(lobbyNotice({ status: 'connecting', notice: null })).toBe('Verbinde …');
    expect(lobbyNotice({ status: 'lost', notice: 'Server nicht erreichbar' })).toContain('Server nicht erreichbar');
    expect(lobbyNotice({ status: 'lobby', notice: null })).toBeNull();
  });

  it('Abbruch-Hinweis im Spiel nur beim Wiederverbinden', () => {
    expect(gameNotice({ status: 'reconnecting', notice: 'Verbindung weg, verbinde neu …' })).toBe('Verbindung weg, verbinde neu …');
    expect(gameNotice({ status: 'room', notice: null })).toBeNull();
  });

  it('Rückkehr zur Lobby nach room_closed/room_not_found (lobby), 120 s (lost) und replaced/version (ended)', () => {
    for (const s of ['lobby', 'lost', 'ended'] as Status[]) expect(leavesGame(s)).toBe(true);
    for (const s of ['room', 'reconnecting', 'connecting'] as Status[]) expect(leavesGame(s)).toBe(false);
  });
});

describe('Lobby ohne Verbindung (B-083)', () => {
  const room = { code: 'KRNZ', name: 'familie', depth: 0, grade: 'normal', taken: 1, free: 3, running: true };

  it('lost bietet nur „Erneut versuchen“, ended nur „Seite neu laden“, ohne Verbindung gibt es keine Einträge', () => {
    expect(lobbyEntries([room], 'lost')).toEqual([{ kind: 'retry' }]);
    expect(lobbyEntries([room], 'ended')).toEqual([{ kind: 'reload' }]);
    expect(lobbyEntries([room], 'connecting')).toEqual([]);
    expect(lobbyEntries([room], 'reconnecting')).toEqual([]);
    expect(lobbyEntries([room], 'lobby').map((e) => e.kind)).toEqual(['play', 'room']);
  });

  it('?room und ?players=n treten mit den Slots 0…n-1 bei, ohne ?players mit einem (B-353/AC-02)', () => {
    expect(parseStartParams('?room=ABCD&players=3').players).toBe(3);
    for (const q of ['players=0', 'players=5', 'players=x', '']) expect(parseStartParams(`?${q}`).players).toBe(1);
    expect(new LobbyFlow(parseStartParams('?room=ABCD&players=3')).step(lobby)).toEqual({ t: 'join', room: 'ABCD', slots: [0, 1, 2] });
    expect(new LobbyFlow(parseStartParams('?room=ABCD')).step(lobby)).toEqual({ t: 'join', room: 'ABCD', slots: [0] });
    expect(new LobbyFlow(parseStartParams('?autostart=1&players=2')).step(lobby)).toMatchObject({ t: 'create', slots: [0, 1] });
    expect(slotsFor(2, 2)).toEqual([0, 1, 2]);
  });

  it('beschriftet die Aktionen und sendet für sie keinen Befehl an den Server', () => {
    expect(entryLabel({ kind: 'retry' }, 'familie')).toBe('Erneut versuchen');
    expect(entryLabel({ kind: 'reload' }, 'familie')).toBe('Seite neu laden');
    const flow = new LobbyFlow(parseStartParams(''));
    expect(flow.choose({ kind: 'retry' })).toBeNull();
    expect(flow.choose({ kind: 'reload' })).toBeNull();
  });
});
