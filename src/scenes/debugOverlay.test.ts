import { describe, expect, it } from 'vitest';
import { debugEnabled, debugLines, type DebugClient, type DebugInput, type DebugWorld } from './debugOverlay';

const client = (over: Partial<DebugClient> = {}): DebugClient => ({
  status: 'room',
  roomCode: 'FAMILIE',
  deviceId: 'a3f1c9e0',
  tickHz: 30,
  snapshotHz: 29.6,
  lastSnapshotAt: 969,
  offlineSince: null,
  notice: null,
  errorCode: null,
  ...over,
});
const world: DebugWorld = { cycle: { day: 2 }, enemies: [1, 2, 3], troops: [1], players: [1, 2] };
const input = (over: Partial<DebugInput> = {}): DebugInput => ({ client: client(), protocol: 2, world, fps: 59.7, now: 1000, ...over });

describe('debugLines (AC-01)', () => {
  it('verbunden: Raum, Gerät, Status, Takt, Alter, Entitäten, FPS', () => {
    expect(debugLines(input())).toEqual([
      'Raum FAMILIE · Gerät a3f1',
      'verbunden · v2 · 30 Hz (Soll 30)',
      'letzter Snapshot 31 ms',
      'Tag 2 · 3 Gegner · 1 Truppen · 2 Spieler',
      '60 FPS',
    ]);
  });

  it('getrennt: Sekunden seit dem Verlust, eine Nachkommastelle', () => {
    const c = client({ status: 'reconnecting', offlineSince: 1000 - 2400 });
    expect(debugLines(input({ client: c }))[1]).toBe('getrennt seit 2.4 s');
  });

  it('verbindet: keine erfundenen Zahlen, nur FPS', () => {
    const c = client({ status: 'connecting', roomCode: null, snapshotHz: null, lastSnapshotAt: null });
    expect(debugLines(input({ client: c, world: null }))).toEqual(['Raum – · Gerät a3f1', 'verbindet…', '60 FPS']);
  });

  it('fehlende Werte werden zu –', () => {
    const c = client({ roomCode: null, deviceId: '', snapshotHz: null, lastSnapshotAt: null });
    const lines = debugLines(input({ client: c, world: null, fps: null }));
    expect(lines).toEqual(['Raum – · Gerät –', 'verbunden · v2 · – Hz (Soll 30)', 'letzter Snapshot – ms', '– FPS']);
  });

  it('getrennt ohne Zeitpunkt: –', () => {
    const c = client({ status: 'reconnecting', offlineSince: null });
    expect(debugLines(input({ client: c }))[1]).toBe('getrennt seit – s');
  });

  it('ohne Welt keine Entitäten-Zeile, mit Welt eine', () => {
    expect(debugLines(input({ world: null })).some((l) => l.startsWith('Tag'))).toBe(false);
    expect(debugLines(input()).some((l) => l.startsWith('Tag'))).toBe(true);
  });

  it('Lobby und verloren zeigen ihren Status', () => {
    expect(debugLines(input({ client: client({ status: 'lobby' }) }))[1]).toContain('Lobby');
    expect(debugLines(input({ client: client({ status: 'lost' }) }))[1]).toContain('verloren');
  });
});

describe('debugEnabled (AC-02)', () => {
  it('standardmäßig an, nur ?dev=0 schaltet ab', () => {
    expect(debugEnabled('')).toBe(true);
    expect(debugEnabled('?x=1')).toBe(true);
    expect(debugEnabled('?dev=1')).toBe(true);
    expect(debugEnabled('?seed=x&dev=0')).toBe(false);
  });
  it('Versionszeile steht zuletzt, auch beim Verbinden', () => {
    const v = 'Client v0.4.0 · Server v0.4.0';
    expect(debugLines(input({ version: v })).at(-1)).toBe(v);
    expect(debugLines(input({ version: v, client: client({ status: 'connecting' }) })).at(-1)).toBe(v);
    expect(debugLines(input()).at(-1)).toBe('60 FPS');
  });
  it('Zeitfaktor nur im Dev-Mode des Raums (B-179/AC-03)', () => {
    expect(debugLines(input({ world: { ...world, devTimescale: 8 } }))).toContain('Zeit 8×');
    expect(debugLines(input({ world: { ...world, devTimescale: 1 } }))).toContain('Zeit 1×');
    expect(debugLines(input()).some((l) => l.startsWith('Zeit'))).toBe(false);
  });

  it('forbidden nach einer Dev-Aktion: Hinweis im Overlay', () => {
    const c = client({ errorCode: 'forbidden', notice: 'Nur im Dev-Mode erlaubt' });
    expect(debugLines(input({ client: c }))).toContain('Dev abgelehnt: Nur im Dev-Mode erlaubt');
    expect(debugLines(input({ client: client({ errorCode: 'room_full', notice: 'Raum voll' }) })).some((l) => l.startsWith('Dev'))).toBe(false);
  });
});
