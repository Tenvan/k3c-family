import { describe, expect, it } from 'vitest';
import type { McpCall } from '../api';
import { byTarget, parseDuration, runOf, runsIn } from './checkRuns';

const T0 = Date.parse('2026-11-14T10:00:00.000Z');
let id = 0;
function call(over: Partial<McpCall>): McpCall {
  id++;
  return { id, ref: `#${id}`, startedAt: '', ts: new Date(T0 + id * 1000).toISOString(), startSeq: id, endSeq: id,
    running: false, tool: 'check_run', args: '{}', durationMs: 500, ok: true, summary: '', ...over };
}

describe('parseDuration', () => {
  it('liest alle Formate von formatMs', () => {
    expect(parseDuration('850 ms')).toBe(850);
    expect(parseDuration('4,2 s')).toBe(4200);
    expect(parseDuration('3 min 5 s')).toBe(185_000);
    expect(parseDuration('1 h 2 min')).toBe(3_720_000);
    expect(parseDuration('bald')).toBeNull();
  });
});

describe('runOf', () => {
  it('Ziel aus dem Feld target, sonst Argument, sonst Kopfzeile', () => {
    expect(runOf(call({ target: 'task:test', args: '{"target":"x"}' }))?.target).toBe('task:test');
    expect(runOf(call({ args: '{"target":"go:test","wait":true}' }))?.target).toBe('go:test');
    expect(runOf(call({ args: '{"run":"r7"}', summary: 'task:lint · exit 0 · 2,0 s' }))?.target).toBe('task:lint');
  });

  it('exit ≠ 0 und Zeitlimit sind Fehlschläge, die Dauer kommt aus der Kopfzeile', () => {
    const bad = runOf(call({ summary: 'task:test · exit 1 · 12,5 s', durationMs: 13_000 }));
    expect(bad).toMatchObject({ state: 'failed', exit: 1, ms: 12_500 });
    expect(runOf(call({ summary: 'task:test · exit 0 · 850 ms' }))).toMatchObject({ state: 'ok', exit: 0, ms: 850 });
    expect(runOf(call({ summary: 'task:test · Zeitlimit 5 min 0 s überschritten · 5 min 0 s' }))?.state).toBe('failed');
    expect(runOf(call({ ok: false, error: 'unbekanntes Ziel', summary: 'unbekanntes Ziel' }))?.state).toBe('failed');
  });

  it('Hintergrund-Antwort ist offen, andere Tools und laufende Aufrufe fallen weg', () => {
    expect(runOf(call({ summary: 'task:test läuft noch seit 30,0 s · Lauf-ID r1' }))?.state).toBe('pending');
    expect(runOf(call({ tool: 'logs_query' }))).toBeNull();
    expect(runOf(call({ running: true, ts: '' }))).toBeNull();
  });
});

describe('runsIn und byTarget', () => {
  const calls = [
    call({ target: 'task:test', summary: 'task:test · exit 0 · 2,0 s' }),
    call({ target: 'task:test', summary: 'task:test · exit 1 · 4,0 s' }),
    call({ target: 'task:test', summary: 'task:test läuft noch seit 30,0 s · Lauf-ID r1' }),
    call({ target: 'go:test', summary: 'go:test · exit 0 · 1,0 s' }),
    call({ tool: 'svc_status' }),
  ].reverse(); // das Log kommt neueste zuerst

  it('filtert auf das Fenster und sortiert aufsteigend', () => {
    const runs = runsIn(calls, { from: T0, to: T0 + 3500 });
    expect(runs.map((r) => r.call.id)).toEqual([calls[4].id, calls[3].id, calls[2].id]);
  });

  it('Anzahl, Ø, max und Fehlerquote je Ziel ohne offene Läufe', () => {
    const stats = byTarget(runsIn(calls, { from: 0, to: Infinity }));
    expect(stats[0]).toEqual({ target: 'task:test', count: 3, failed: 1, avgMs: 3000, maxMs: 4000, failRate: 50 });
    expect(stats[1]).toMatchObject({ target: 'go:test', count: 1, failRate: 0, avgMs: 1000 });
  });
});
