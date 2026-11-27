import { afterEach, describe, expect, it, vi } from 'vitest';
import { hasTauri, tauriBackend, tauriRpc } from './tauri';

type Listener = (e: { payload: unknown }) => void;

/** Fake von `window.__TAURI__`: merkt Aufrufe und Listener. */
function fakeTauri(invoke: (cmd: string, args?: Record<string, unknown>) => Promise<unknown> = () => Promise.resolve([])) {
  const listeners = new Map<string, Listener>();
  const unlisten = vi.fn();
  const core = { invoke: vi.fn(invoke) };
  const event = {
    listen: vi.fn((name: string, fn: Listener) => {
      listeners.set(name, fn);
      return Promise.resolve(unlisten);
    }),
  };
  vi.stubGlobal('window', { __TAURI__: { core, event } });
  return { core, listeners, unlisten };
}

afterEach(() => vi.unstubAllGlobals());

describe('Tauri-Adapter', () => {
  it('erkennt das Tauri-Fenster', () => {
    vi.stubGlobal('window', {});
    expect(hasTauri()).toBe(false);
    fakeTauri();
    expect(hasTauri()).toBe(true);
  });

  it('ruft Methoden über den Command rpc', async () => {
    const t = fakeTauri();
    const backend = tauriBackend();
    expect(backend.mock).toBe(false);
    await backend.info();
    expect(t.core.invoke).toHaveBeenLastCalledWith('rpc', { method: 'Info', args: [] });
    await backend.serviceStop('a', true);
    expect(t.core.invoke).toHaveBeenLastCalledWith('rpc', { method: 'ServiceStop', args: ['a', true] });
    await tauriRpc('McpUsage');
    expect(t.core.invoke).toHaveBeenLastCalledWith('rpc', { method: 'McpUsage', args: [] });
  });

  it('öffnet Links über den Command open_url', () => {
    const t = fakeTauri();
    tauriBackend().openUrl('https://example.org');
    expect(t.core.invoke).toHaveBeenLastCalledWith('open_url', { url: 'https://example.org' });
  });

  it('macht aus einem abgelehnten Aufruf ein Error', async () => {
    fakeTauri(() => Promise.reject('Unbekannte Methode Services'));
    const call = tauriBackend().services();
    await expect(call).rejects.toBeInstanceOf(Error);
    await expect(call).rejects.toThrow('Unbekannte Methode Services');
  });

  it('meldet Ereignisse über listen und meldet sich ab', async () => {
    const t = fakeTauri();
    const seen = vi.fn();
    const off = tauriBackend().on('mcp:state', seen);
    const state = { addr: '127.0.0.1:5280', listening: true, error: '' };
    t.listeners.get('mcp:state')!({ payload: state });
    expect(seen).toHaveBeenCalledWith(state);
    await Promise.resolve();
    off();
    expect(t.unlisten).toHaveBeenCalledTimes(1);
    t.listeners.get('mcp:state')!({ payload: state });
    expect(seen).toHaveBeenCalledTimes(1);
  });

  it('meldet sich auch ab, wenn listen noch läuft', async () => {
    const t = fakeTauri();
    tauriBackend().on('mcp:state', vi.fn())();
    await Promise.resolve();
    expect(t.unlisten).toHaveBeenCalledTimes(1);
  });
});
