import { afterEach, describe, expect, it, vi } from 'vitest';
import { httpBackend } from './http';

// Client-Server-Betrieb: Methoden als POST auf /api/dev/rpc/<Methode>, Ereignisse per EventSource von /api/dev/events.

afterEach(() => vi.unstubAllGlobals());

function stubFetch(status: number, body: unknown) {
  const fetchMock = vi.fn().mockResolvedValue(new Response(JSON.stringify(body), { status }));
  vi.stubGlobal('fetch', fetchMock);
  return fetchMock;
}

/** EventSource-Attrappe: hält die Listener, emit schickt ein Ereignis wie der Server. */
class FakeSource {
  static last: FakeSource | null = null;
  static count = 0;
  readonly listeners = new Map<string, ((e: { data: string }) => void)[]>();
  constructor(readonly url: string) {
    FakeSource.last = this;
    FakeSource.count++;
  }
  addEventListener(name: string, fn: (e: { data: string }) => void) {
    this.listeners.set(name, [...(this.listeners.get(name) ?? []), fn]);
  }
  emit(name: string, data: unknown) {
    this.listeners.get(name)?.forEach((fn) => fn({ data: JSON.stringify(data) }));
  }
}

describe('RPC', () => {
  it.each([
    ['info', 'Info', []],
    ['serviceStop', 'ServiceStop', ['vite', true]],
    ['taskStart', 'TaskStart', ['check', ['--', 'x']]],
    ['taskGates', 'TaskGatesView', []],
    ['planningSet', 'PlanningSet', ['B-1', 'Prio', 'hoch']],
    ['githubStatus', 'GitHubStatus', [false]],
  ] as const)('%s ruft %s mit Argumenten und Kopfzeile', async (method, rpc, args) => {
    const fetchMock = stubFetch(200, { result: { ok: rpc } });
    const call = httpBackend()[method] as (...a: unknown[]) => Promise<unknown>;
    expect(await call(...args)).toEqual({ ok: rpc });
    const [url, init] = fetchMock.mock.calls[0];
    expect(url).toBe(`/api/dev/rpc/${rpc}`);
    expect(init.method).toBe('POST');
    expect(init.headers['X-K3C-Dev']).toBe('1');
    expect(JSON.parse(init.body)).toEqual(args);
  });

  it('ein Fehler aus Go kommt als Ablehnung mit dem Text', async () => {
    stubFetch(422, { error: 'dienst unbekannt: x' });
    await expect(httpBackend().serviceStart('x')).rejects.toThrow('dienst unbekannt: x');
  });

  it('eine Antwort ohne JSON nennt den Status', async () => {
    vi.stubGlobal('fetch', vi.fn().mockResolvedValue(new Response('kaputt', { status: 502 })));
    await expect(httpBackend().info()).rejects.toThrow('Fehler 502');
  });
});

describe('Ereignisse', () => {
  it('ein Strom für alle Abonnenten, Nutzlast als JSON, Abmelden wirkt', () => {
    FakeSource.count = 0;
    vi.stubGlobal('EventSource', FakeSource);
    const api = httpBackend();
    const a = vi.fn();
    const b = vi.fn();
    const offA = api.on('mcp:state', a);
    api.on('planning:changed', b);
    expect(FakeSource.count).toBe(1);
    expect(FakeSource.last?.url).toBe('/api/dev/events');
    FakeSource.last?.emit('mcp:state', { addr: '127.0.0.1:5190', listening: true, error: '' });
    FakeSource.last?.emit('planning:changed', null);
    expect(a).toHaveBeenCalledWith({ addr: '127.0.0.1:5190', listening: true, error: '' });
    expect(b).toHaveBeenCalledWith(null);
    offA();
    FakeSource.last?.emit('mcp:state', { addr: '', listening: false, error: 'x' });
    expect(a).toHaveBeenCalledTimes(1);
  });
});

it('openUrl öffnet einen neuen Tab', () => {
  const open = vi.fn();
  vi.stubGlobal('window', { open });
  httpBackend().openUrl('https://example.org');
  expect(open).toHaveBeenCalledWith('https://example.org', '_blank', 'noopener');
});
