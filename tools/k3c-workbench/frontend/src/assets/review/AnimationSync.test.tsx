// @vitest-environment happy-dom
import { pickValue } from '@webkit/testing/select';
import { act } from 'react';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { Compare } from './Compare';
import { button, click, fakeApi, fixture, key, mount, settle, unmount, until } from './testing';
import type { Review } from '../api';

// RV1.4 / AC-04: Vorher und Nachher laufen synchron mit einem Frame-Schieber.

const fake = vi.hoisted(() => ({ api: {} as ReturnType<typeof import('./testing').fakeApi> }));
vi.mock('../../api', async (echt) => {
  const m = await echt<typeof import('../../api')>();
  return { ...m, backend: new Proxy(m.backend, { get: (t, k: string) => (fake.api as Record<string, unknown>)[k] ?? Reflect.get(t, k) }) };
});

const monarch = fixture<Review>('AssetsReview').assets.find((a) => a.id === 'monarch')!;
/** Frames, die je Seite (Basis HEAD~1 bzw. Arbeitsstand '') zuletzt angefragt wurden. */
const letzte = () => {
  const calls = fake.api.assetsFrame.mock.calls;
  const seite = (b: string) => calls.filter((c) => c[4] === b).at(-1);
  return { vorher: seite('HEAD~1'), nachher: seite('') };
};
/** Zeitlimit der Tests, die 16 Frames einzeln rendern: unter Volllast (task check:diff) reichen 5 s nicht (B-704). */
const LANG = 15_000;
/** Lässt n Frames im Takt von 125 ms (fps 8) laufen, jeden einzeln gerendert. */
const tick = async (ms: number) => {
  for (let t = 0; t < ms; t += 125) {
    await act(async () => {
      await vi.advanceTimersByTimeAsync(Math.min(125, ms - t));
    });
    await settle();
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
    const regler = await until('Frame-Schieber', () => host.querySelector('[aria-label="Frame"] [role="slider"]'));
    for (let i = 0; i < 5; i++) await key(regler, 'ArrowRight');
    await until('Frame 5 beider Seiten', () => letzte().vorher?.[2] === 5 && letzte().nachher?.[2] === 5);
    expect(letzte().vorher?.slice(0, 3)).toEqual(['monarch', 'idle', 5]);
    expect(letzte().nachher?.slice(0, 3)).toEqual(['monarch', 'idle', 5]);
  });

  it('Wiedergabe synchron mit fps 8, danach wieder Frame 0', async () => {
    vi.useFakeTimers({ toFake: ['setInterval', 'clearInterval'] });
    const host = await mount(<Compare asset={monarch} basis="HEAD~1" />);
    await until('Abspielen-Knopf', () => button(host, 'Abspielen'));
    await act(async () => button(host, 'Abspielen')!.click());
    await tick(125);
    await until('Frame 1 beider Seiten', () => letzte().vorher?.[2] === 1 && letzte().nachher?.[2] === 1);
    await tick(125 * 7);
    // Frame 0 kommt jetzt aus dem Cache, daher am Schieber prüfen
    await until('Schieber auf 0', () => host.querySelector('[aria-label="Frame"] [role="slider"]')?.getAttribute('aria-valuenow') === '0');
    expect(host.querySelector('[aria-label="Frame"] [role="slider"]')?.getAttribute('aria-valuenow')).toBe('0');
    expect(host.textContent).toContain('Frame 1/8');
  });

  it('Animation wechseln setzt beide Seiten auf Frame 0', async () => {
    const host = await mount(<Compare asset={monarch} basis="HEAD~1" />);
    await until('Animations-Auswahl', () => host.querySelector('[data-part="animation-select"]'));
    await pickValue(host, 'Animation', 'reiten');
    await until('Frame 0 von reiten', () => letzte().vorher?.[1] === 'reiten' && letzte().nachher?.[1] === 'reiten');
    expect(letzte().vorher?.slice(1, 3)).toEqual(['reiten', 0]);
    expect(letzte().nachher?.slice(1, 3)).toEqual(['reiten', 0]);
  });

  it('Frame fehlt in der Basis: Hinweis, Nachher läuft weiter', async () => {
    const ok = fake.api.assetsFrame.getMockImplementation()!;
    fake.api.assetsFrame.mockImplementation((id, a, f, v, b) => (b === 'HEAD~1' ? Promise.reject(new Error('Animation „idle“ fehlt')) : ok(id, a, f, v, b)));
    const host = await mount(<Compare asset={monarch} basis="HEAD~1" />);
    await until('Hinweis und Nachher', () => host.textContent?.includes('Frame fehlt in der Basis') && host.querySelector('img[alt="Nachher"]'));
  });

  it('Cache: ein zweiter Durchlauf fragt keinen Frame neu an', async () => {
    vi.useFakeTimers({ toFake: ['setInterval', 'clearInterval'] });
    const host = await mount(<Compare asset={monarch} basis="HEAD~1" />);
    await until('Abspielen-Knopf', () => button(host, 'Abspielen'));
    await act(async () => button(host, 'Abspielen')!.click());
    await tick(125 * 8);
    await until('16 Frames angefragt', () => fake.api.assetsFrame.mock.calls.length >= 16);
    const nachEinemDurchlauf = fake.api.assetsFrame.mock.calls.length;
    await tick(125 * 8);
    expect(fake.api.assetsFrame.mock.calls.length).toBe(nachEinemDurchlauf);
    expect(nachEinemDurchlauf).toBe(16);
  }, LANG);
});

describe('Feste Steuerung (WB2/AC-01)', () => {
  it('Frame-Player mit festen Breiten, Zähler tabular-nums; Abspielen ändert die Breiten nicht', async () => {
    vi.useFakeTimers({ toFake: ['setInterval', 'clearInterval'] });
    const host = await mount(<Compare asset={monarch} basis="HEAD~1" />);
    await until('Frame-Player', () => host.querySelector('[data-part="count"]'));
    const teile = () => ['play', 'slider', 'count'].map((p) => host.querySelector<HTMLElement>(`[data-part="${p}"]`)?.style.width);
    const vorher = teile();
    expect(vorher.every(Boolean)).toBe(true);
    expect(host.querySelector<HTMLElement>('[data-part="count"]')!.style.fontVariantNumeric).toBe('tabular-nums');
    await act(async () => button(host, 'Abspielen')!.click());
    await tick(125 * 3);
    expect(teile()).toEqual(vorher);
  });
});

describe('Review-Befund 6 (WB2.3)', () => {
  it('Animations-Auswahl hat eine feste Mindestbreite', async () => {
    const host = await mount(<Compare asset={monarch} basis="HEAD~1" />);
    const box = await until('Animations-Auswahl', () => host.querySelector<HTMLElement>('[data-part="animation-select"]'));
    expect(box.style.width).not.toBe('');
  });
});

describe('Pixel-Diff beim Abspielen (Befund 4)', () => {
  it('fragt jeden Diff nur einmal an', async () => {
    vi.useFakeTimers({ toFake: ['setInterval', 'clearInterval'] });
    const host = await mount(<Compare asset={monarch} basis="HEAD~1" />);
    await until('Unterschiede-Knopf', () => button(host, 'Unterschiede'));
    await click(button(host, 'Unterschiede'));
    await until('Abspielen-Knopf', () => button(host, 'Abspielen'));
    await act(async () => button(host, 'Abspielen')!.click());
    await tick(125 * 16);
    await until('8 Diffs angefragt', () => fake.api.assetsPixelDiff.mock.calls.length >= 8);
    expect(fake.api.assetsPixelDiff.mock.calls.length).toBe(8);
  }, LANG);
});
