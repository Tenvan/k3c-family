import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { mockBackend } from './mock';

beforeEach(() => vi.useFakeTimers());
afterEach(() => vi.useRealTimers());

describe('Mock des Starters', () => {
  it('startet von selbst und läuft nach kurzer Zeit mit Adressen', async () => {
    const b = mockBackend();
    expect(b.mock).toBe(true);
    expect((await b.state()).starting).toBe(true);
    await vi.advanceTimersByTimeAsync(1500);
    const s = await b.state();
    expect(s).toMatchObject({ running: true, starting: false, message: 'Server läuft.' });
    expect(s.urls.length).toBeGreaterThan(0);
    expect(s.log.length).toBeGreaterThan(0);
  });

  it('Beenden stoppt ihn, Starten startet ihn wieder', async () => {
    const b = mockBackend();
    await vi.advanceTimersByTimeAsync(1500);
    await b.stop();
    expect((await b.state()).message).toBe('Server wird beendet …');
    await vi.advanceTimersByTimeAsync(1000);
    expect(await b.state()).toMatchObject({ running: false, starting: false, urls: [], message: 'Server ist gestoppt.' });
    await b.start();
    expect((await b.state()).starting).toBe(true);
  });
});
