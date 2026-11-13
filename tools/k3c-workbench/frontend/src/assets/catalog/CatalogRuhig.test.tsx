// @vitest-environment happy-dom
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { act } from 'react';
import { afterEach, describe, expect, it, vi } from 'vitest';
import { backend } from '../../api';
import type { AssetEintrag, Frame, Spec } from '../api';
import { buttons, click, flush, mount, unmount, wait } from '../testing';
import { mountCatalog, playFrames, select, until } from './catalogTesting';
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
    const real = backend.assetsIndex.bind(backend);
    vi.spyOn(backend, 'assetsIndex').mockReturnValue(list.promise);
    const host = await mount(<CatalogPage />);
    expect(loading(host, 'list')?.textContent).toContain('Lädt Assets');
    list.resolve(await real());
    await until('Liste geladen', () => entry(host, 'monarch'));
    expect(loading(host, 'list')).toBeNull();
  });

  it('bei der Wahl eines Assets, bis die Spec da ist', async () => {
    const spec = deferred<Spec>();
    const real = backend.assetsGet.bind(backend);
    const host = await mountCatalog();
    vi.spyOn(backend, 'assetsGet').mockReturnValue(spec.promise);
    await click(entry(host, 'monarch'));
    expect(loading(host, 'asset')?.textContent).toContain('Lädt monarch');
    spec.resolve(await real('monarch'));
    await until('Spec da', () => !loading(host, 'asset'));
  });

  it('beim Frame-Laden: altes Bild bleibt, Spinner liegt über der Grafik', async () => {
    const host = await mountCatalog();
    await select(host, 'monarch');
    const real = backend.assetsFrame.bind(backend);
    const second = deferred<Frame>();
    vi.spyOn(backend, 'assetsFrame').mockImplementation((id, a, f, v, b) => (f === 1 ? second.promise : real(id, a, f, v, b)));
    await pressRight(host);
    const stage = host.querySelector('.as-stage')!;
    expect(stage.querySelector('img[alt="monarch idle Frame 1"]')).toBeTruthy();
    expect(stage.querySelector('[data-loading="frame"]')).toBeTruthy();
    second.resolve(await real('monarch', 'idle', 1, '', ''));
    await until('Frame 2 gezeigt', () => stage.querySelector('img[alt="monarch idle Frame 2"]'));
    expect(stage.querySelector('[data-loading="frame"]')).toBeNull();
  });
});

describe('Katalog: ruhige Ansicht (WB2/AC-04, AC-05)', () => {
  it('Liste und Vorschau je eigener Scrollbereich', async () => {
    const host = await mountCatalog();
    await select(host, 'monarch');
    const list = host.querySelector('[data-scroll="list"]')!;
    const detail = host.querySelector('[data-scroll="detail"]')!;
    expect(list.contains(entry(host, 'monarch'))).toBe(true);
    expect(detail.querySelector('.as-stage')).toBeTruthy();
    expect(list.contains(detail)).toBe(false);
  });

  it('Frame-Player mit festen Breiten; jeder Frame wird nur einmal geladen', async () => {
    const frame = vi.spyOn(backend, 'assetsFrame');
    const host = await mountCatalog();
    await select(host, 'monarch');
    const widths = () => ['play', 'slider', 'count'].map((p) => host.querySelector<HTMLElement>(`[data-part="${p}"]`)?.style.width);
    const before = widths();
    expect(before.every(Boolean)).toBe(true);
    // eine volle Runde und einen Frame weiter: ein zweites Laden fiele als doppelter Schlüssel auf
    await playFrames(host, 9);
    expect(widths()).toEqual(before);
    const keys = frame.mock.calls.map((c) => c.slice(0, 3).join('|'));
    expect(new Set(keys).size).toBe(keys.length);
  });
});

describe('Review-Befunde WB2.3', () => {
  it('Befund 1: Filter klebt in einer deckenden Hülle, das Feld behält seinen Rahmen', async () => {
    const host = await mountCatalog();
    const wrap = host.querySelector('.as-list-filter')!;
    expect(wrap.querySelector('input[aria-label="Filter"]')).toBeTruthy();
    expect(wrap.tagName).not.toBe('INPUT');
    const css = readFileSync(resolve(__dirname, '../../styles/assets.css'), 'utf8');
    const rule = /\.as-list-filter\s*\{([^}]*)\}/.exec(css)?.[1] ?? '';
    expect(rule).toMatch(/position:\s*sticky/);
    expect(rule).toMatch(/background:\s*var\(--color-background\)/);
    expect(rule).not.toMatch(/box-shadow/);
  });

  it('Befund 2: Vorabladen nacheinander (höchstens zwei offen), Abbruch bei Asset-Wechsel', async () => {
    const host = await mountCatalog();
    await select(host, 'monarch');
    const real = backend.assetsFrame.bind(backend);
    const offen: { resolve: () => void }[] = [];
    const frame = vi.spyOn(backend, 'assetsFrame').mockImplementation((id, a, f, v, b) => new Promise<Frame>((r) => offen.push({ resolve: () => r(real(id, a, f, v, b)) })));
    await click(buttons(host, 'Abspielen')[0]);
    await wait(400);
    expect(frame.mock.calls.length).toBeLessThanOrEqual(2);
    await click(entry(host, 'mauer'));
    const vorher = frame.mock.calls.filter((c) => c[0] === 'monarch').length;
    for (const o of offen.splice(0)) o.resolve();
    await flush();
    await wait(200);
    expect(frame.mock.calls.filter((c) => c[0] === 'monarch').length).toBe(vorher);
  });

  it('Befund 4: defekter Frame lässt das letzte Bild stehen und wird nicht jede Runde neu geladen', async () => {
    const host = await mountCatalog();
    await select(host, 'monarch');
    const real = backend.assetsFrame.bind(backend);
    const frame = vi.spyOn(backend, 'assetsFrame').mockImplementation((id, a, f, v, b) => (f === 1 ? Promise.reject(new Error('Frame kaputt')) : real(id, a, f, v, b)));
    await pressRight(host);
    const stage = host.querySelector('.as-stage')!;
    await until('Fehler am Frame', () => stage.textContent?.includes('Frame kaputt'));
    expect(stage.querySelector('img')).toBeTruthy();
    // eine volle Runde und einen Frame weiter: der defekte Frame 2 kommt ein zweites Mal dran
    await playFrames(host, 9);
    expect(frame.mock.calls.filter((c) => c[2] === 1)).toHaveLength(1);
  });
});
