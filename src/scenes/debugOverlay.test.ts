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
  it('nur mit dev-Parameter', () => {
    expect(debugEnabled('?dev=1')).toBe(true);
    expect(debugEnabled('?seed=x&dev=1')).toBe(true);
    expect(debugEnabled('')).toBe(false);
    expect(debugEnabled('?x=1')).toBe(false);
  });
});
