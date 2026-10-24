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
const params = (p: Partial<StartParams> = {}): StartParams => ({ autostart: false, fresh: false, save: 'familie', mock: 0, room: null, players: 1, depth: 0, seed: '', world: '', ...p });

describe('Start-Parameter (AC-14, B-082/AC-01)', () => {
  it('gültige Werte', () => {
    expect(parseStartParams('?autostart=1&fresh=1&save=test-ab12&mock=3&room=krnz')).toEqual({
      autostart: true,
      fresh: true,
      save: 'test-ab12',
      mock: 3,
      room: 'KRNZ',
      players: 1,
      depth: 0,
      seed: '',
      world: '',
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
    const p = parseStartParams('?autostart=yes&fresh=true&continue=1&online=X');
    expect(p).toEqual(params());
  });
});

describe('Seed und Tiefe per URL (B-095/AC-02)', () => {
  it('wertet ?seed= und ?depth= aus', () => {
    const p = parseStartParams('?autostart=1&fresh=1&save=probe&seed=Familie-2026&depth=2');
    expect(p).toMatchObject({ save: 'probe', seed: 'Familie-2026', depth: 2 });
  });

  it.each(['depth=-1', 'depth=1.5', 'depth=', 'depth=abc', 'depth=1234'])('ignoriert ungültiges %s', (q) => {
    expect(parseStartParams(`?${q}`).depth).toBe(0);
  });

  it('Tiefe und Seed außerhalb der Grenzen gehen unverändert an den Server (bad_request, B-095)', () => {
    expect(parseStartParams('?depth=10').depth).toBe(10);
    expect(parseStartParams(`?seed=${'ü'.repeat(65)}`).seed).toBe('ü'.repeat(65));
  });

  it('create trägt seed und depth nur bei fresh', () => {
    const p = { autostart: true, fresh: true, save: 'probe', seed: 'Familie-2026', depth: 1 };
    expect(new LobbyFlow(params(p)).step(lobby)).toEqual({ t: 'create', save: 'probe', fresh: true, depth: 1, slots: [0], seed: 'Familie-2026' });
    const load = new LobbyFlow(params({ ...p, fresh: false })).step(lobby);
    expect(load).not.toHaveProperty('seed');
  });

  it('ohne seed trägt create kein seed-Feld (alte Server)', () => {
    expect(new LobbyFlow(params({ autostart: true, fresh: true })).step(lobby)).not.toHaveProperty('seed');
  });

  it('create trägt ?world= nur bei fresh, ohne world kein Feld (WZ18.5)', () => {
    const p = parseStartParams('?autostart=1&fresh=1&save=nord&world=nordmeer');
    expect(p.world).toBe('nordmeer');
    expect(new LobbyFlow(p).step(lobby)).toEqual({ t: 'create', save: 'nord', fresh: true, depth: 0, slots: [0], world: 'nordmeer' });
    expect(new LobbyFlow({ ...p, fresh: false }).step(lobby)).not.toHaveProperty('world');
    expect(new LobbyFlow(params({ autostart: true, fresh: true })).step(lobby)).not.toHaveProperty('world');
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
    applyCommand(client, { t: 'create', save: 's', fresh: true, depth: 0, slots: [0, 1], seed: 'x', world: 'nordmeer' });
    applyCommand(client, { t: 'join', room: 'KRNZ', slots: [0] });
    expect(calls).toEqual([
      ['create', 's', true, 0, [0, 1], 'x', 'nordmeer', undefined],
      ['join', 'KRNZ', [0]],
    ]);
  });
});

describe('Lobby-Auswahl (AC-03)', () => {
  it('„Spielen“ steht oben, danach die Räume', () => {
    expect(lobbyEntries([room]).map((e) => e.kind)).toEqual(['play', 'new', 'room']);
  });

  it('Auswahl läuft am Ende zum Anfang und umgekehrt', () => {
    expect(moveSelection(0, -1, 3)).toBe(2);
    expect(moveSelection(0, 1, 3)).toBe(1);
    expect(moveSelection(2, 1, 3)).toBe(0);
  });

  it('Zeile unter dem Zeiger', () => {
    expect(rowAt(130, 120, 50, 3)).toBe(0);
    expect(rowAt(175, 120, 50, 3)).toBe(1);
    expect(rowAt(100, 120, 50, 3)).toBe(-1);
    expect(rowAt(400, 120, 50, 3)).toBe(-1);
  });

  it('„Spielen“ erstellt, ein Raum aus der Liste tritt bei', () => {
    const flow = new LobbyFlow(params({ mock: 1 }));
    const [play, , entry] = lobbyEntries([room]);
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
    expect(gameNotice({ status: 'room', notice: 'Raum ist voll', errorCode: 'room_full' })).toBeNull();
  });

  it('„Server startet neu“ steht im Spiel schon vor dem Abbruch (B-507)', () => {
    expect(gameNotice({ status: 'room', notice: 'Server startet neu', errorCode: 'restarting' })).toBe('Server startet neu');
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
    expect(lobbyEntries([room], 'lobby').map((e) => e.kind)).toEqual(['play', 'new', 'room']);
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

describe('Spielstand wählen (LB1.1, B-037/AC-04)', () => {
  const save = (name: string, savedAt: string, day?: number) => ({ name, savedAt, depths: [0, 1], ...(day ? { day } : {}) });
  const saves = Array.from({ length: 8 }, (_, i) => save(`s${i}`, `2026-10-0${i + 1}T10:00:00Z`));

  it('die neuesten sechs unter den Räumen, ohne schon offene', () => {
    const entries = lobbyEntries([room], 'lobby', [...saves, save('familie', '2026-10-09T10:00:00Z')]);
    expect(entries.map((e) => e.kind)).toEqual(['play', 'new', 'room', 'save', 'save', 'save', 'save', 'save', 'save']);
    expect(entries.flatMap((e) => (e.kind === 'save' ? [e.save.name] : []))).toEqual(['s7', 's6', 's5', 's4', 's3', 's2']);
  });

  it('ohne Räume direkt unter „Spielen“; ohne Verbindung nur retry/reload', () => {
    expect(lobbyEntries([], 'lobby', [saves[0]!]).map((e) => e.kind)).toEqual(['play', 'new', 'save']);
    expect(lobbyEntries([], 'lost', saves)).toEqual([{ kind: 'retry' }]);
    expect(lobbyEntries([], 'ended', saves)).toEqual([{ kind: 'reload' }]);
  });

  it('Auswahl sendet create mit diesem Namen und fresh: false, kein leerer Neuversuch', () => {
    const flow = new LobbyFlow(params({ fresh: true }));
    expect(flow.choose({ kind: 'save', save: save('burg', '2026-10-01T10:00:00Z') })).toEqual({ t: 'create', save: 'burg', fresh: false, depth: 0, slots: [0] });
    expect(flow.step({ status: 'lobby', errorCode: 'save_not_found' })).toBeNull();
  });

  it('Spielstand-Eintrag mit dem Namen params.save: save_not_found liefert keinen Neuversuch, „Spielen“ und Autostart schon (B-382/AC-01)', () => {
    const notFound = { status: 'lobby' as Status, errorCode: 'save_not_found' };
    const flow = new LobbyFlow(params());
    expect(flow.choose({ kind: 'save', save: save('familie', '2026-10-01T10:00:00Z') })).toMatchObject({ save: 'familie', fresh: false });
    expect(flow.step(notFound)).toBeNull();
    flow.choose({ kind: 'play' });
    expect(flow.step(notFound)).toMatchObject({ t: 'create', save: 'familie', fresh: true });
    expect(flow.step(notFound)).toBeNull();
    const auto = new LobbyFlow(params({ autostart: true }));
    auto.step(lobby);
    expect(auto.step(notFound)).toMatchObject({ t: 'create', fresh: true });
    expect(auto.step(notFound)).toBeNull();
  });

  it('ungültiges savedAt sortiert hinter die gültigen, übrige nach Datum (B-382/AC-02)', () => {
    const entries = lobbyEntries([], 'lobby', [save('x', 'x'), save('alt', '2026-10-01T10:00:00Z'), save('neu', '2026-10-03T10:00:00Z'), save('leer', '')]);
    expect(entries.flatMap((e) => (e.kind === 'save' ? [e.save.name] : []))).toEqual(['neu', 'alt', 'x', 'leer']);
  });

  it('Beschriftung mit Name, Tag und Stufe', () => {
    expect(entryLabel({ kind: 'save', save: save('burg', '2026-10-01T10:00:00Z', 3) }, 'familie')).toBe('Spielstand burg  ·  Welt standard  ·  Tag 3  ·  Stufe 0, 1');
    expect(entryLabel({ kind: 'save', save: { name: 'alt', savedAt: '', depths: [] } }, 'familie')).toBe('Spielstand alt  ·  Welt standard  ·  Tag –  ·  Stufe –');
    expect(entryLabel({ kind: 'save', save: { name: 'nord', savedAt: '', world: 'nordmeer', depths: [0] } }, 'familie')).toBe('Spielstand nord  ·  Welt nordmeer  ·  Tag –  ·  Stufe 0');
  });
});

describe('Neuer Spielstand (BD5.2, B-506/AC-02)', () => {
  it('nächster freier Name, auch gegen offene Räume; legt frisch in der Start-Welt an', () => {
    const saves = [{ name: 'spiel-1', savedAt: '2026-10-01T10:00:00Z', depths: [0] }];
    const [, entry] = lobbyEntries([{ ...room, name: 'spiel-2' }], 'lobby', saves);
    expect(entry).toEqual({ kind: 'new', name: 'spiel-3' });
    expect(entryLabel(entry!, 'familie')).toBe('Neuer Spielstand  (spiel-3)');
    const flow = new LobbyFlow(params({ world: 'nordmeer', seed: 'abc' }));
    expect(flow.choose(entry!)).toEqual({ t: 'create', save: 'spiel-3', fresh: true, depth: 0, slots: [0], seed: 'abc', world: 'nordmeer' });
  });
});

describe('Anlegen-Dialog (K5.2, B-105/AC-02)', () => {
  it('„Neuer Spielstand“ öffnet den Dialog; „Raum anlegen“ sendet grade, goal und defeat wie c2s-create.json', async () => {
    const createMsg = (await import('../../testdata/protocol/c2s-create.json')).default;
    const flow = new LobbyFlow(params());
    flow.openDialog('spiel-2');
    expect(flow.dialog).toMatchObject({ save: 'spiel-2', row: 0 });
    const down = (n: number) => { for (let i = 0; i < n; i++) flow.dialogInput({ dir: 1, side: 0, confirm: false }); };
    flow.dialogInput({ dir: 0, side: 1, confirm: false }); // Grad: normal → hard, Modus folgt dem Standard (stage)
    down(4);
    const cmd = flow.dialogInput({ dir: 0, side: 0, confirm: true });
    expect(cmd).toMatchObject({ t: 'create', save: 'spiel-2', fresh: true, options: { grade: 'hard', goal: 'endboss', defeat: 'stage' } });
    expect(Object.keys((cmd as { options: object }).options).sort()).toEqual(['defeat', 'goal', 'grade'].filter((k) => k in createMsg).sort());
    expect(flow.dialog).toBeNull();
    const sent: unknown[][] = [];
    applyCommand({ create: (...a: unknown[]) => sent.push(a), join: () => {} }, cmd!);
    expect(sent[0]?.at(-1)).toEqual({ grade: 'hard', goal: 'endboss', defeat: 'stage' });
  });

  it('„Abbrechen“ schließt ohne Befehl; Autostart sendet ohne Optionen; abgelehnte Option hat eigenen Hinweis', () => {
    const flow = new LobbyFlow(params());
    flow.openDialog('x');
    flow.dialogInput({ dir: 5, side: 0, confirm: false });
    expect(flow.dialogInput({ dir: 0, side: 0, confirm: true })).toBeNull();
    expect(flow.dialog).toBeNull();
    const auto = new LobbyFlow(params({ autostart: true })).step(lobby);
    expect(auto).not.toHaveProperty('options');
    expect(lobbyNotice({ status: 'lobby', notice: null, errorCode: 'bad_request' })).toBeTruthy();
  });
});

describe('Welt-Auswahl im Anlegen-Dialog (BD8.1, B-566/AC-01)', () => {
  const toWorld = (flow: LobbyFlow) => flow.dialogInput({ dir: 3, side: 0, confirm: false });

  it('die gewählte Welt geht in create.world; `standard` lässt das Feld weg', () => {
    const flow = new LobbyFlow(params(), true);
    flow.worlds = ['standard', 'nordmeer'];
    flow.openDialog('spiel-1');
    toWorld(flow);
    flow.dialogInput({ dir: 0, side: 1, confirm: false }); // Welt: standard → nordmeer
    expect(flow.dialog?.options.world).toBe('nordmeer');
    expect(flow.dialogInput({ dir: 1, side: 0, confirm: false })).toBeNull();
    expect(flow.dialogInput({ dir: 0, side: 0, confirm: true })).toMatchObject({ t: 'create', save: 'spiel-1', fresh: true, world: 'nordmeer' });

    flow.openDialog('spiel-2');
    flow.dialogInput({ dir: 4, side: 0, confirm: false });
    expect(flow.dialogInput({ dir: 0, side: 0, confirm: true })).not.toHaveProperty('world');
  });

  it('Welt per A/Enter im Kreis, links/rechts begrenzt; ?world= ist vorgewählt und wählbar', () => {
    const flow = new LobbyFlow(params({ world: 'suedsee' }), true);
    flow.worlds = ['standard', 'nordmeer'];
    expect(flow.dialogWorlds()).toEqual(['standard', 'nordmeer', 'suedsee']);
    flow.openDialog('x');
    expect(flow.dialog?.options.world).toBe('suedsee');
    toWorld(flow);
    flow.dialogInput({ dir: 0, side: 0, confirm: true }); // suedsee → standard (Kreis)
    expect(flow.dialog?.options.world).toBe('standard');
    flow.dialogInput({ dir: 0, side: -1, confirm: false });
    expect(flow.dialog?.options.world).toBe('standard');
  });
});
