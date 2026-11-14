import { describe, expect, it } from 'vitest';
import type { McpCall } from '../api';
import { applyCall, MAX_CALLS } from './callBuffer';

function call(id: number, running: boolean): McpCall {
  return { id, ref: `#${id}`, startedAt: '', ts: '', startSeq: id, endSeq: running ? 0 : id, running, tool: 'plan_list',
    args: '{}', durationMs: running ? 0 : 5, ok: true, summary: '' };
}

describe('applyCall', () => {
  it('B-895/AC-01: 10 000 Live-Ereignisse halten das Aufruf-Log begrenzt', () => {
    let calls: McpCall[] = [];
    for (let id = 1; id <= 5000; id++) {
      calls = applyCall(calls, call(id, true)); // mcp:start
      calls = applyCall(calls, call(id, false)); // mcp:call
    }
    expect(calls).toHaveLength(MAX_CALLS);
    expect(calls[0].id).toBe(5000);
    expect(calls.at(-1)?.id).toBe(5000 - MAX_CALLS + 1);
    expect(calls.every((c) => !c.running)).toBe(true);
  });

  it('Ende ersetzt den Start an seiner Stelle, neueste zuerst', () => {
    let calls = [call(3, false), call(1, false)];
    calls = applyCall(calls, call(2, true));
    expect(calls.map((c) => c.id)).toEqual([3, 2, 1]);
    calls = applyCall(calls, call(2, false));
    expect(calls.map((c) => [c.id, c.running])).toEqual([[3, false], [2, false], [1, false]]);
  });

  it('ein Aufruf, der älter als alle gehaltenen ist, fällt bei vollem Log heraus', () => {
    const full = Array.from({ length: 3 }, (_, i) => call(10 - i, false));
    expect(applyCall(full, call(1, false), 3).map((c) => c.id)).toEqual([10, 9, 8]);
  });
});
