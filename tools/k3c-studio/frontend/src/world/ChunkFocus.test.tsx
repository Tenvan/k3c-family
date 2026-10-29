// @vitest-environment happy-dom
import { Theme } from '@radix-ui/themes';
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { backend } from '../api';
import { mockWorldLevel } from '../api/mockWorld';
import { WorldPage } from './WorldPage';
import { planSlot } from './worldTesting';
import { pickWhere } from '../../../../webkit/testing/options';
import { pickOption } from '../../../../webkit/testing/select';

// Beim Ändern eines geplanten Chunks springen beide Vorschauen auf ihn (B-485, WZ17.5; Wunsch von Ralf), wie der Klick-Sprung.
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

const button = (h: HTMLElement, label: string) => [...h.querySelectorAll('button')].find((b) => b.textContent === label || b.getAttribute('aria-label') === label)!;
const field = (h: HTMLElement, label: string) => h.querySelector<HTMLInputElement>(`[aria-label="${label}"]`)!;
const wait = (ms: number) => act(async () => { await new Promise((r) => setTimeout(r, ms)); });
const settle = async () => { await wait(260); await flush(); }; // Prüfung ist um 200 ms verzögert, danach lädt das Level neu

async function type(el: HTMLInputElement, value: string) {
  Object.getOwnPropertyDescriptor(HTMLInputElement.prototype, 'value')!.set!.call(el, value);
  await act(async () => el.dispatchEvent(new Event('input', { bubbles: true })));
}

/** Chunk am Platz +1 (Abschnitt 6) planen, dann die Ansicht mit den Pfeiltasten wegbewegen; liefert die neue Bildmitte der Live-Vorschau */
async function plannedAway(h: HTMLElement) {
  await planSlot(h, 1);
  expect(liveCenter()).toBeCloseTo(mid(6));
  await act(async () => { for (let i = 0; i < 3; i++) document.dispatchEvent(new KeyboardEvent('keydown', { key: 'ArrowRight', bubbles: true })); });
  const away = liveCenter();
  expect(Math.abs(away - mid(6))).toBeGreaterThan(5);
  return away;
}

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

describe('Beim Ändern eines Chunks zentrieren (B-485, WZ17.5)', () => {
  it('Chunk hinzufügen und Art ändern: beide Bildmitten stehen auf der Chunk-Mitte, der Zoom bleibt', async () => {
    const h = await mount();
    await plannedAway(h);
    const scale = views.at(-1)!.scale;
    await type(field(h, 'Art Platz +1'), 'clearing');
    expect(liveCenter()).toBeCloseTo(mid(6));
    expect(chunkCenter(h)).toBeCloseTo(mid(6));
    expect(views.at(-1)!.scale).toBe(scale);
    await settle(); // das neu geladene Level (gleiche Stufe) setzt die Mitte nicht auf den Hub zurück
    expect(liveCenter()).toBeCloseTo(mid(6));
  });

  it('Objekt hinzufügen springt ebenfalls zum Chunk', async () => {
    const h = await mount();
    await plannedAway(h);
    await act(async () => button(h, '+ Objekt').click());
    expect(liveCenter()).toBeCloseTo(mid(6));
    expect(chunkCenter(h)).toBeCloseTo(mid(6));
  });

  it('Löschen ändert die Ansicht nicht', async () => {
    const h = await mount();
    const away = await plannedAway(h);
    await act(async () => button(h, 'Chunk Platz +1 löschen').click());
    await settle();
    expect(liveCenter()).toBeCloseTo(away);
  });

  it('Tippen im Seed-Feld: der Fokus bleibt im Feld, ist der Chunk schon zentriert, bewegt sich nichts', async () => {
    const h = await mount();
    await plannedAway(h);
    const seed = field(h, 'Chunk-Seed Platz +1');
    seed.focus();
    await type(seed, 'l');
    expect(liveCenter()).toBeCloseTo(mid(6));
    const moves = views.length;
    await type(seed, 'li');
    await type(seed, 'lic');
    expect(views.length).toBe(moves);
    expect(document.activeElement).toBe(seed); // Schutz gegen ein künftiges focus() beim Sprung
  });

  it('eine andere Inselgröße springt nicht zum Chunk (das neue Level beginnt beim Hub)', async () => {
    const h = await mount();
    const away = await plannedAway(h);
    await pickOption(h, 'Inselgröße', 'klein · 24');
    expect(liveCenter()).toBeCloseTo(away);
    await settle();
    expect(liveCenter()).toBeCloseTo(mid(12)); // Mock mit 24 Abschnitten: Hub = Abschnitt 12
  });
});

describe('Ein neues Level beginnt weiter beim Hub (WZ17.5)', () => {
  it('neuer Seed der Chunk-Vorschau zentriert den Hub', async () => {
    const h = await mount();
    await plannedAway(h);
    await type(field(h, 'Seed'), 'neu');
    await act(async () => button(h, 'Laden').click());
    await flush();
    expect(liveCenter()).toBeCloseTo(mid(5));
  });

  it('andere Stufe zentriert den Hub', async () => {
    const h = await mount();
    await act(async () => chunk(h, 8).click());
    await pickWhere(h, 'Stufe', (t) => t.startsWith('Stufe 2 '));
    await settle();
    expect(liveCenter()).toBeCloseTo(mid(5));
  });

  it('Objekt entfernen springt zum Chunk', async () => {
    const h = await mount();
    await plannedAway(h);
    await act(async () => button(h, '+ Objekt').click());
    await act(async () => { for (let i = 0; i < 3; i++) document.dispatchEvent(new KeyboardEvent('keydown', { key: 'ArrowRight', bubbles: true })); });
    await act(async () => button(h, 'Objekt 1 Platz +1 löschen').click());
    expect(liveCenter()).toBeCloseTo(mid(6));
  });
});
