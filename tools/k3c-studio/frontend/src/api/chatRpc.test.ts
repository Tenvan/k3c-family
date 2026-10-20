import { afterEach, describe, expect, it, vi } from 'vitest';
import { httpBackend } from './http';

// Chat-Methoden des Backends (Studio-Chat) als RPC auf /api/studio/rpc/<Methode> mit den Argumenten als JSON-Array.

afterEach(() => vi.unstubAllGlobals());

function stub(result: unknown) {
  const fetchMock = vi.fn().mockResolvedValue(new Response(JSON.stringify({ result }), { status: 200 }));
  vi.stubGlobal('fetch', fetchMock);
  return fetchMock;
}

describe('Chat-RPC', () => {
  it.each([
    ['chatList', 'ChatList', [], [{ id: 'c1' }]],
    ['chatGet', 'ChatGet', ['c1'], { id: 'c1', messages: [] }],
    ['chatDelete', 'ChatDelete', ['c1'], null],
    ['chatOptions', 'ChatOptions', [], { providers: [], models: [], efforts: [] }],
  ] as const)('%s ruft %s mit den Argumenten', async (method, rpc, args, result) => {
    const fetchMock = stub(result);
    const call = httpBackend()[method] as (...a: unknown[]) => Promise<unknown>;
    expect(await call(...args)).toEqual(result);
    expect(fetchMock.mock.calls[0][0]).toBe(`/api/studio/rpc/${rpc}`);
    expect(JSON.parse(fetchMock.mock.calls[0][1].body)).toEqual(args);
  });
});
