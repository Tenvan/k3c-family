// @vitest-environment happy-dom
import { act } from 'react';
import { afterEach, describe, expect, it, vi } from 'vitest';
import { backend } from '../../api';
import type { AssetEintrag, Frame, Spec } from '../api';
import { buttons, click, flush, mount, unmount, wait } from '../testing';
import { CatalogPage } from './CatalogPage';

// WB2/AC-03 bis AC-05: Lade-Anzeige (über der Grafik, ohne Layoutsprung), getrennte Scrollbereiche, fester Frame-Player.

afterEach(() => {
  unmount();
  vi.restoreAllMocks();
});

/** Promise, das der Test von außen erfüllt. */
function deferred<T>() {
  let resolve!: (v: T) => void;
  const promise = new Promise<T>((r) => (resolve = r));
  return { promise, resolve };
}

const entry = (host: ParentNode, id: string) => host.querySelector<HTMLElement>(`[data-asset="${id}"]`)!;
const loading = (host: ParentNode, what: string) => host.querySelector(`[data-loading="${what}"]`);

async function pressRight(host: ParentNode) {
  const thumb = host.querySelector<HTMLElement>('[aria-label="Frame"] [role="slider"]')!;
  await act(async () => {
    thumb.dispatchEvent(new KeyboardEvent('keydown', { key: 'ArrowRight', bubbles: true }));
  });
  await flush();
}

describe('Katalog: Lade-Anzeige (WB2/AC-03)', () => {
  it('beim Laden der Liste', async () => {
    const list = deferred<AssetEintrag[]>();
    const real = backend.assetsList.bind(backend);
    vi.spyOn(backend, 'assetsList').mockReturnValue(list.promise);
    const host = await mount(<CatalogPage />);
    expect(loading(host, 'list')?.textContent).toContain('Lädt Assets');
    list.resolve(await real());
    await flush();
    expect(loading(host, 'list')).toBeNull();
    expect(entry(host, 'monarch')).toBeTruthy();
  });

  it('bei der Wahl eines Assets, bis die Spec da ist', async () => {
    const spec = deferred<Spec>();
    const real = backend.assetsGet.bind(backend);
    const host = await mount(<CatalogPage />);
    vi.spyOn(backend, 'assetsGet').mockReturnValue(spec.promise);
    await click(entry(host, 'monarch'));
    expect(loading(host, 'asset')?.textContent).toContain('Lädt monarch');
    spec.resolve(await real('monarch'));
    await flush();
    expect(loading(host, 'asset')).toBeNull();
  });

  it('beim Frame-Laden: altes Bild bleibt, Spinner liegt über der Grafik', async () => {
    const host = await mount(<CatalogPage />);
    await click(entry(host, 'monarch'));
    const real = backend.assetsFrame.bind(backend);
    const second = deferred<Frame>();
    vi.spyOn(backend, 'assetsFrame').mockImplementation((id, a, f, v, b) => (f === 1 ? second.promise : real(id, a, f, v, b)));
    await pressRight(host);
    const stage = host.querySelector('.as-stage')!;
    expect(stage.querySelector('img[alt="monarch idle Frame 1"]')).toBeTruthy();
    expect(stage.querySelector('[data-loading="frame"]')).toBeTruthy();
    second.resolve(await real('monarch', 'idle', 1, '', ''));
    await flush();
    expect(stage.querySelector('[data-loading="frame"]')).toBeNull();
    expect(stage.querySelector('img[alt="monarch idle Frame 2"]')).toBeTruthy();
  });
});

describe('Katalog: ruhige Ansicht (WB2/AC-04, AC-05)', () => {
  it('Liste und Vorschau je eigener Scrollbereich', async () => {
    const host = await mount(<CatalogPage />);
    await click(entry(host, 'monarch'));
    const list = host.querySelector('[data-scroll="list"]')!;
    const detail = host.querySelector('[data-scroll="detail"]')!;
    expect(list.contains(entry(host, 'monarch'))).toBe(true);
    expect(detail.querySelector('.as-stage')).toBeTruthy();
    expect(list.contains(detail)).toBe(false);
  });

  it('Frame-Player mit festen Breiten; jeder Frame wird nur einmal geladen', async () => {
    const frame = vi.spyOn(backend, 'assetsFrame');
    const host = await mount(<CatalogPage />);
    await click(entry(host, 'monarch'));
    const widths = () => ['play', 'slider', 'count'].map((p) => host.querySelector<HTMLElement>(`[data-part="${p}"]`)?.style.width);
    const before = widths();
    expect(before.every(Boolean)).toBe(true);
    await click(buttons(host, 'Abspielen')[0]);
    await wait(1200);
    await click(buttons(host, 'Anhalten')[0]);
    expect(widths()).toEqual(before);
    const keys = frame.mock.calls.map((c) => c.slice(0, 3).join('|'));
    expect(new Set(keys).size).toBe(keys.length);
  });
});
