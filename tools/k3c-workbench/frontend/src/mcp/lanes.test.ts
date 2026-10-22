import { describe, expect, it } from 'vitest';
import type { McpCall } from '../api';
import { buildGraph, callFooter, filterCalls, prettyArgs } from './lanes';

/** Aufruf mit Start- und Endnummer; end 0 heißt „läuft“. */
const call = (id: number, start: number, end: number, over: Partial<McpCall> = {}): McpCall => ({
  id, ref: `#${id}`, startedAt: '', ts: '', startSeq: start, endSeq: end, running: end === 0, tool: 'check_run',
  args: '{}', durationMs: 10, ok: true, summary: '', ...over,
});
const shape = (calls: McpCall[]) => buildGraph(calls).rows.map((r) => `${r.kind[0]}${r.call.id}@${r.lane}`);

describe('Spuren des Aufruf-Logs', () => {
  it('nacheinander: eine Spur, neueste oben', () => {
    const g = buildGraph([call(2, 3, 4), call(1, 1, 2)]);
    expect(g.lanes).toBe(1);
    expect(shape([call(2, 3, 4), call(1, 1, 2)])).toEqual(['e2@0', 's2@0', 'e1@0', 's1@0']);
  });

  it('zwei parallel: zwei Spuren, die jeweils andere läuft durch', () => {
    const calls = [call(1, 1, 3), call(2, 2, 4)];
    const g = buildGraph(calls);
    expect(g.lanes).toBe(2);
    expect(shape(calls)).toEqual(['e2@1', 'e1@0', 's2@1', 's1@0']);
    const endOf1 = g.rows.find((r) => r.kind === 'end' && r.call.id === 1)!;
    expect(endOf1.through.map((t) => t.lane)).toEqual([1]);
  });

  it('drei verschachtelt und eine freie Spur wird wiederverwendet', () => {
    // 1 läuft lange, 2 und 3 überlappen darin; 4 startet nach dem Ende von 2 und nimmt Spur 1
    const calls = [call(1, 1, 10), call(2, 2, 4), call(3, 3, 6), call(4, 5, 7)];
    const g = buildGraph(calls);
    const lane = (id: number) => g.rows.find((r) => r.call.id === id)!.lane;
    expect([lane(1), lane(2), lane(3), lane(4)]).toEqual([0, 1, 2, 1]);
    expect(g.lanes).toBe(3);
  });

  it('laufender Aufruf hat nur eine Startzeile und ist oben offen', () => {
    const calls = [call(1, 1, 0), call(2, 2, 3)];
    const g = buildGraph(calls);
    expect(shape(calls)).toEqual(['e2@1', 's2@1', 's1@0']);
    const endOf2 = g.rows[0];
    expect(endOf2.through).toEqual([{ lane: 0, color: 1, running: true }]);
  });

  it('Farbe je Aufruf und leeres Log', () => {
    const g = buildGraph([call(9, 1, 2)]);
    expect(g.rows.every((r) => r.color === 1)).toBe(true);
    expect(buildGraph([])).toEqual({ rows: [], lanes: 0 });
  });
});

describe('Filter und Fußzeile', () => {
  const calls = [call(1, 1, 2), call(2, 3, 4, { tool: 'logs_query', ok: false }), call(3, 5, 0, { tool: 'neu_nach_neustart' })];

  it('nach Tool, nur Fehler; unbekanntes Tool bleibt unter Alle', () => {
    expect(filterCalls(calls, { tool: '', errorsOnly: false }).length).toBe(3);
    expect(filterCalls(calls, { tool: 'logs_query', errorsOnly: false }).map((c) => c.id)).toEqual([2]);
    expect(filterCalls(calls, { tool: '', errorsOnly: true }).map((c) => c.id)).toEqual([2]);
  });

  it('Fußzeile und Argumente', () => {
    expect(callFooter(calls)).toBe('3 Aufrufe · 1 laufend · 1 Fehler');
    expect(prettyArgs('{"target":"task:test"}')).toBe('{\n  "target": "task:test"\n}');
    expect(prettyArgs('kein json')).toBe('kein json');
  });
});
