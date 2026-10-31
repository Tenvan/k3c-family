import { afterEach, describe, expect, it, vi } from 'vitest';
import { httpBackend } from './http';

// WZ29.6, WZ33.1: das HTTP-Backend ruft die RPC-Methoden des Studios mit den Argumenten als JSON-Array.
afterEach(() => vi.unstubAllGlobals());

describe('HTTP-Backend: RPC-Aufrufe', () => {
  it('Quellen: Sources, SourceDownload, SourceAdopt (WZ29.6)', async () => {
    const calls = stubFetch();
    await httpBackend().sources();
    await httpBackend().sourceDownload(['q1', 'q2'], 300);
    await httpBackend().sourceAdopt('q1');
    expect(calls).toEqual([
      { url: '/api/studio/rpc/Sources', body: '[]' },
      { url: '/api/studio/rpc/SourceDownload', body: '[["q1","q2"],300]' },
      { url: '/api/studio/rpc/SourceAdopt', body: '["q1"]' },
    ]);
  });

  it('Kandidaten: AssetCandidates und AssetCandidateAssign (WZ33.1)', async () => {
    const calls = stubFetch();
    await httpBackend().assetCandidates('tavern', 'taverne', 60);
    await httpBackend().assetCandidateAssign('tavern', 'sug:q05', 'abc', 24_500);
    expect(calls).toEqual([
      { url: '/api/studio/rpc/AssetCandidates', body: '["tavern","taverne",60]' },
      { url: '/api/studio/rpc/AssetCandidateAssign', body: '["tavern","sug:q05","abc",24500]' },
    ]);
  });
});

function stubFetch(): { url: string; body: string }[] {
  const calls: { url: string; body: string }[] = [];
  vi.stubGlobal('fetch', (url: string, init: RequestInit) => {
    calls.push({ url, body: String(init.body) });
    return Promise.resolve(new Response(JSON.stringify({ result: null }), { status: 200 }));
  });
  return calls;
}
