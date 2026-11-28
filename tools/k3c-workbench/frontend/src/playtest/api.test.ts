import { afterEach, describe, expect, it, vi } from 'vitest';

/** Lädt das Modul neu: der Zugang wird beim Laden gewählt. */
async function load(window: Record<string, unknown>) {
  vi.resetModules();
  vi.stubGlobal('window', window);
  return import('./api');
}

afterEach(() => vi.unstubAllGlobals());

describe('Zugang der Seite Spieltest', () => {
  it('ruft im Tauri-Fenster über den Command rpc', async () => {
    const invoke = vi.fn((_cmd: string, _args?: unknown) => Promise.resolve([]));
    const { playtestApi } = await load({ __TAURI__: { core: { invoke }, event: { listen: vi.fn() } } });
    await playtestApi.list();
    expect(invoke).toHaveBeenLastCalledWith('rpc', { method: 'PlaytestList', args: [] });
    await playtestApi.describe('2026-11-22-1', '', [2]);
    expect(invoke).toHaveBeenLastCalledWith('rpc', { method: 'PlaytestDescribe', args: ['2026-11-22-1', '', [2]] });
    await playtestApi.skipFrame('2026-11-22-1', 1, true);
    expect(invoke).toHaveBeenLastCalledWith('rpc', { method: 'PlaytestFrameSkip', args: ['2026-11-22-1', 1, true] });
  });

  it('holt Dateien im Tauri-Fenster über das Protokoll k3c', async () => {
    const { mediaUrl, frameUrl } = await load({ __TAURI__: { core: { invoke: vi.fn() }, event: { listen: vi.fn() } } });
    expect(mediaUrl('2026-11-22-1', 'video.mkv')).toBe('http://k3c.localhost/api/dev/playtest/media/2026-11-22-1/video.mkv');
    expect(frameUrl('2026-11-22-1', 2)).toBe('http://k3c.localhost/api/dev/playtest/frame/2026-11-22-1/2');
  });

  it('lässt die Adressen sonst relativ', async () => {
    const { mediaUrl, frameUrl } = await load({ location: { search: '' } });
    expect(mediaUrl('2026-11-22-1', 'voice.wav')).toBe('/api/dev/playtest/media/2026-11-22-1/voice.wav');
    expect(frameUrl('2026-11-22-1', 2)).toBe('/api/dev/playtest/frame/2026-11-22-1/2');
  });
});
