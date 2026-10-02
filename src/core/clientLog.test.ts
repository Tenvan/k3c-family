import { afterEach, describe, expect, it, vi } from 'vitest';

async function load() {
  vi.resetModules();
  return import('./clientLog');
}

function stubBrowser() {
  const listeners: Record<string, (e: unknown) => void> = {};
  const fetchMock = vi.fn().mockResolvedValue({});
  vi.stubGlobal('window', { addEventListener: (n: string, f: (e: unknown) => void) => (listeners[n] = f), innerWidth: 1, innerHeight: 1, devicePixelRatio: 1 });
  vi.stubGlobal('document', { addEventListener: () => undefined, hidden: false });
  vi.stubGlobal('navigator', { userAgent: 'test', maxTouchPoints: 0 });
  vi.stubGlobal('location', { href: 'http://x/game.html' });
  vi.stubGlobal('localStorage', { getItem: () => 'geraet-1' });
  vi.stubGlobal('fetch', fetchMock);
  return { listeners, fetchMock };
}

const sent = (fetchMock: ReturnType<typeof vi.fn>) => fetchMock.mock.calls.flatMap((c) => (JSON.parse(String((c[1] as { body: string }).body)) as { entries: { level: string; msg: string }[] }).entries);

afterEach(() => vi.unstubAllGlobals());

describe('clientLog', () => {
  it('tut vor der Installation nichts', async () => {
    const { fetchMock } = stubBrowser();
    const { clientLog } = await load();
    clientLog('error', 'x');
    expect(fetchMock).not.toHaveBeenCalled();
  });

  it('schickt Fehler sofort und zählt Wiederholungen', async () => {
    const { listeners, fetchMock } = stubBrowser();
    const { installClientLog } = await load();
    installClientLog();
    const error = new Error('kaputt');
    listeners.error!({ message: 'kaputt', error });
    listeners.error!({ message: 'kaputt', error });
    expect(sent(fetchMock).filter((e) => e.level === 'error')).toHaveLength(1);
    listeners.pagehide!({});
    expect(sent(fetchMock).some((e) => e.msg.includes('1× wiederholt'))).toBe(true);
  });
});
