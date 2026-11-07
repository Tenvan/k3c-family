// @vitest-environment happy-dom
import { pickValue } from '@webkit/testing/select';
import { act } from 'react';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { Compare } from './Compare';
import { button, click, fakeApi, fixture, flush, key, mount, unmount } from './testing';
import type { Review } from './types';

// RV1.4 / AC-04: Vorher und Nachher laufen synchron mit einem Frame-Schieber.

const fake = vi.hoisted(() => ({ api: {} as ReturnType<typeof import('./testing').fakeApi> }));
vi.mock('./api', () => ({ reviewApi: new Proxy({}, { get: (_t, k: string) => (fake.api as Record<string, unknown>)[k] }) }));

const monarch = fixture<Review>('AssetsReview').assets.find((a) => a.id === 'monarch')!;
/** Frames, die je Seite (Basis HEAD~1 bzw. Arbeitsstand '') zuletzt angefragt wurden. */
const letzte = () => {
  const calls = fake.api.frame.mock.calls;
  const seite = (b: string) => calls.filter((c) => c[4] === b).at(-1);
  return { vorher: seite('HEAD~1'), nachher: seite('') };
};
/** Lässt n Frames im Takt von 125 ms (fps 8) laufen, jeden einzeln gerendert. */
const tick = async (ms: number) => {
  for (let t = 0; t < ms; t += 125) {
    await act(async () => {
      await vi.advanceTimersByTimeAsync(Math.min(125, ms - t));
    });
    await flush();
  }
};

beforeEach(() => {
  fake.api = fakeApi();
});
afterEach(() => {
  unmount();
  vi.useRealTimers();
});

describe('AnimationSync', () => {
  it('Schieber setzt beide Seiten', async () => {
    const host = await mount(<Compare asset={monarch} basis="HEAD~1" />);
    const regler = host.querySelector('[aria-label="Frame"] [role="slider"]')!;
    for (let i = 0; i < 5; i++) await key(regler, 'ArrowRight');
    await click(host);
    expect(letzte().vorher?.slice(0, 3)).toEqual(['monarch', 'idle', 5]);
    expect(letzte().nachher?.slice(0, 3)).toEqual(['monarch', 'idle', 5]);
  });

  it('Wiedergabe synchron mit fps 8, danach wieder Frame 0', async () => {
    vi.useFakeTimers({ toFake: ['setInterval', 'clearInterval'] });
    const host = await mount(<Compare asset={monarch} basis="HEAD~1" />);
    await tick(10);
    await act(async () => button(host, 'Abspielen')!.click());
    await tick(125);
    expect(letzte().vorher?.[2]).toBe(1);
    expect(letzte().nachher?.[2]).toBe(1);
    await tick(125 * 7);
    // Frame 0 kommt jetzt aus dem Cache, daher am Schieber prüfen
    expect(host.querySelector('[aria-label="Frame"] [role="slider"]')?.getAttribute('aria-valuenow')).toBe('0');
    expect(host.textContent).toContain('Frame 1/8');
  });

  it('Animation wechseln setzt beide Seiten auf Frame 0', async () => {
    const host = await mount(<Compare asset={monarch} basis="HEAD~1" />);
    await pickValue(host, 'Animation', 'reiten');
    await click(host);
    expect(letzte().vorher?.slice(1, 3)).toEqual(['reiten', 0]);
    expect(letzte().nachher?.slice(1, 3)).toEqual(['reiten', 0]);
  });

  it('Frame fehlt in der Basis: Hinweis, Nachher läuft weiter', async () => {
    const ok = fake.api.frame.getMockImplementation()!;
    fake.api.frame.mockImplementation((id, a, f, v, b) => (b === 'HEAD~1' ? Promise.reject(new Error('Animation „idle“ fehlt')) : ok(id, a, f, v, b)));
    const host = await mount(<Compare asset={monarch} basis="HEAD~1" />);
    expect(host.textContent).toContain('Frame fehlt in der Basis');
    expect(host.querySelector('img[alt="Nachher"]')).not.toBeNull();
  });

  it('Cache: ein zweiter Durchlauf fragt keinen Frame neu an', async () => {
    vi.useFakeTimers({ toFake: ['setInterval', 'clearInterval'] });
    const host = await mount(<Compare asset={monarch} basis="HEAD~1" />);
    await tick(10);
    await act(async () => button(host, 'Abspielen')!.click());
    await tick(125 * 8);
    const nachEinemDurchlauf = fake.api.frame.mock.calls.length;
    await tick(125 * 8);
    expect(fake.api.frame.mock.calls.length).toBe(nachEinemDurchlauf);
    expect(nachEinemDurchlauf).toBe(16);
  });
});

describe('Feste Steuerung (WB2/AC-01)', () => {
  it('Frame-Player mit festen Breiten, Zähler tabular-nums; Abspielen ändert die Breiten nicht', async () => {
    vi.useFakeTimers({ toFake: ['setInterval', 'clearInterval'] });
    const host = await mount(<Compare asset={monarch} basis="HEAD~1" />);
    await tick(10);
    const teile = () => ['play', 'slider', 'count'].map((p) => host.querySelector<HTMLElement>(`[data-part="${p}"]`)?.style.width);
    const vorher = teile();
    expect(vorher.every(Boolean)).toBe(true);
    expect(host.querySelector<HTMLElement>('[data-part="count"]')!.style.fontVariantNumeric).toBe('tabular-nums');
    await act(async () => button(host, 'Abspielen')!.click());
    await tick(125 * 3);
    expect(teile()).toEqual(vorher);
  });
});

describe('Pixel-Diff beim Abspielen (Befund 4)', () => {
  it('fragt jeden Diff nur einmal an', async () => {
    vi.useFakeTimers({ toFake: ['setInterval', 'clearInterval'] });
    const host = await mount(<Compare asset={monarch} basis="HEAD~1" />);
    await click(button(host, 'Unterschiede'));
    await act(async () => button(host, 'Abspielen')!.click());
    await tick(125 * 16);
    expect(fake.api.pixelDiff.mock.calls.length).toBe(8);
  });
});
