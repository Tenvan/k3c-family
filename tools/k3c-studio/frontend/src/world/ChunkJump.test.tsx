// @vitest-environment happy-dom
import { Theme } from '@radix-ui/themes';
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { backend } from '../api';
import { mockWorldLevel } from '../api/mockWorld';
import { WorldPage } from './WorldPage';

// Klick auf einen Chunk springt dorthin (B-485, WZ17.4): Streifen, Bodenstreifen der Live-Vorschau und Planungsliste.
// Bühne 600 × 400 px samt Rahmen, Streifen 1000 px (happy-dom misst sonst 0); Mock-Level: Chunks 30 Units, Hub = Abschnitt 5.
const views: { left: number; scale: number }[] = [];
vi.mock('../stage/PreviewStage', () => ({ PreviewStage: class {
  show() {} refresh() {} setLook() {} setPosition(v: { left: number; scale: number }) { views.push(v); } destroy() {}
} }));
(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

let root: Root | null = null;
let host: HTMLElement | null = null;
const restore: (() => void)[] = [];
const level = mockWorldLevel('k3c', { biome: 'forest' });
const flush = () => act(async () => { for (let i = 0; i < 5; i++) await new Promise((r) => setTimeout(r, 0)); });

function stub(prop: 'offsetWidth' | 'offsetHeight' | 'clientWidth', value: (el: HTMLElement) => number) {
  const original = Object.getOwnPropertyDescriptor(HTMLElement.prototype, prop)!;
  Object.defineProperty(HTMLElement.prototype, prop, { configurable: true, get(this: HTMLElement) { return value(this); } });
  restore.push(() => Object.defineProperty(HTMLElement.prototype, prop, original));
}

async function mount(): Promise<HTMLElement> {
  host = document.createElement('div');
  document.body.append(host);
  root = createRoot(host);
  await act(async () => root!.render(<Theme><WorldPage /></Theme>));
  await flush();
  return host;
}

/** Bildmitte der Chunk-Vorschau (Streifen 1000 px) und der Live-Vorschau (Bühne 600 px) in Units */
const chunkCenter = (h: HTMLElement) => {
  const scale = parseFloat(h.querySelector<HTMLElement>('.wd-strip')!.style.width) / level.widthUnits;
  return h.querySelector<HTMLElement>('.wd-viewport')!.scrollLeft / scale + 1000 / scale / 2;
};
const liveCenter = () => views.at(-1)!.left + 600 / views.at(-1)!.scale / 2;
const mid = (index: number) => index * 30 + 15;
const chunk = (h: HTMLElement, index: number) => [...h.querySelectorAll<HTMLElement>('.wd-chunk')][index];

/** Tippen auf die Bühne bei (x, y) in Pixeln der Bühne; `toX` ≠ x = Ziehen */
const tap = (h: HTMLElement, x: number, y: number, toX = x) => act(async () => {
  const el = h.querySelector<HTMLElement>('.wd-stage')!;
  el.getBoundingClientRect = () => ({ left: 0, top: 0, width: 600, height: 400, right: 600, bottom: 400, x: 0, y: 0, toJSON: () => ({}) });
  el.dispatchEvent(new MouseEvent('pointerdown', { bubbles: true, clientX: x, clientY: y, button: 0 }));
  el.dispatchEvent(new MouseEvent('pointerup', { bubbles: true, clientX: toX, clientY: y, button: 0 }));
});
const BAND_Y = 5; // im Overlay am oberen Rand

beforeEach(async () => {
  views.length = 0;
  stub('offsetWidth', (el) => (el.classList.contains('wd-stage') ? 600 : 0));
  stub('offsetHeight', (el) => (el.classList.contains('wd-stage') ? 400 : 0));
  stub('clientWidth', (el) => (el.classList.contains('wd-viewport') ? 1000 : 0));
  const view = await backend.planLoad();
  await backend.planSave('island1', 0, [], 0, view.stand);
});
afterEach(() => {
  act(() => root?.unmount());
  host?.remove();
  while (restore.length > 0) restore.pop()!();
});

describe('Klick auf einen Chunk im Streifen (B-485)', () => {
  it('stellt beide Vorschauen auf die Chunk-Mitte, der Zoom bleibt', async () => {
    const h = await mount();
    const scale = views.at(-1)!.scale;
    await act(async () => chunk(h, 8).click());
    expect(chunkCenter(h)).toBeCloseTo(mid(8));
    expect(liveCenter()).toBeCloseTo(mid(8));
    expect(views.at(-1)!.scale).toBe(scale);
  });

  it('ein Rand-Chunk wird begrenzt wie beim Schieber (die Ansicht bleibt in der Welt)', async () => {
    const h = await mount();
    await act(async () => chunk(h, 0).click());
    expect(views.at(-1)!.left).toBe(0); // die Live-Vorschau steht am Weltanfang, Mitte = halbe Bühne (25 Units)
    expect(liveCenter()).toBeCloseTo(600 / 12 / 2);
    expect(h.querySelector<HTMLElement>('.wd-viewport')!.scrollLeft).toBe(0); // der breitere Streifen hält ebenfalls am Rand
  });
});

describe('Tippen auf das Chunk-Overlay der Live-Vorschau (B-485)', () => {
  it('ein kurzer Klick auf eine Chunk-Info springt zu diesem Chunk', async () => {
    const h = await mount();
    const { left, scale } = views.at(-1)!;
    const x = (185 - left) * scale; // Pixel der Bühne über Abschnitt 6 (180–210 Units, sichtbar bis 190)
    await tap(h, x, BAND_Y);
    expect(liveCenter()).toBeCloseTo(mid(6));
    expect(chunkCenter(h)).toBeCloseTo(mid(6));
  });

  it('Ziehen löst keinen Sprung aus; ein Klick unter dem Overlay auch nicht', async () => {
    const h = await mount();
    const before = liveCenter();
    await tap(h, 100, BAND_Y, 160);
    expect(liveCenter()).toBeCloseTo(before);
    await tap(h, 500, 200);
    expect(liveCenter()).toBeCloseTo(before);
  });
});

describe('Klick auf einen Eintrag der Planungsliste (B-485)', () => {
  it('springt zum Chunk-Platz; ein Klick in ein Feld des Eintrags nicht', async () => {
    const h = await mount();
    await act(async () => [...h.querySelectorAll('button')].find((b) => b.textContent === 'Chunk hinzufügen')!.click());
    await flush();
    const before = liveCenter();
    await act(async () => h.querySelector<HTMLElement>('[aria-label="Chunk-Seed Platz +1"]')!.click());
    expect(liveCenter()).toBeCloseTo(before);
    await act(async () => h.querySelector<HTMLElement>('.wd-planned')!.click());
    expect(liveCenter()).toBeCloseTo(mid(6)); // Platz +1 = Abschnitt rechts vom Hub
    expect(chunkCenter(h)).toBeCloseTo(mid(6));
  });
});
