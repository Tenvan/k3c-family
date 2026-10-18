// @vitest-environment happy-dom
import { Theme } from '@radix-ui/themes';
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, describe, expect, it, vi } from 'vitest';
import { backend } from '../api';
import { mockWorldLevel } from '../api/mockWorld';
import { WorldPage } from './WorldPage';
import { MAX_SCALE, scaleOfPercent } from './viewState';

// Die Phaser-Bühne braucht ein Canvas; ihr Zusammenspiel prüft LivePreview.test.tsx mit einer Attrappe.
const views: { left: number; scale: number }[] = [];
vi.mock('../stage/PreviewStage', () => ({ PreviewStage: class { show() {} setPosition(v: { left: number; scale: number }) { views.push(v); } setLook() {} destroy() {} } }));

// B-460/AC-01, AC-02 im DOM: der Mock (ohne Wails) liefert das Level, die Chunk-Vorschau ist beim Öffnen maximiert.
(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

let root: Root | null = null;
let host: HTMLElement | null = null;

const flush = () => act(async () => { for (let i = 0; i < 5; i++) await new Promise((r) => setTimeout(r, 0)); });

async function mount(): Promise<HTMLElement> {
  host = document.createElement('div');
  document.body.append(host);
  root = createRoot(host);
  await act(async () => root!.render(<Theme><WorldPage /></Theme>));
  await flush();
  return host;
}

const button = (h: HTMLElement, label: string) => [...h.querySelectorAll('button')].find((b) => b.textContent === label || b.getAttribute('aria-label') === label)!;
const live = (h: HTMLElement) => h.querySelector<HTMLElement>('.wd-live')!;
const page = (h: HTMLElement) => h.querySelector<HTMLElement>('.wd-page')!;
const position = (h: HTMLElement) => h.querySelector<HTMLInputElement>('input[aria-label="Position"]')!;
const stripWidth = (h: HTMLElement) => parseFloat(h.querySelector<HTMLElement>('.wd-strip')!.style.width);
const setInput = (el: HTMLInputElement, value: string) => act(async () => {
  Object.getOwnPropertyDescriptor(HTMLInputElement.prototype, 'value')!.set!.call(el, value);
  el.dispatchEvent(new Event('input', { bubbles: true }));
});

afterEach(() => {
  views.length = 0;
  vi.restoreAllMocks();
  act(() => root?.unmount());
  host?.remove();
});

describe('Seite Welt-Betrachter: Chunk-Vorschau (B-460/AC-01, AC-02)', () => {
  it('ist beim Öffnen maximiert und zeigt beide Vorschauen (B-477/AC-04)', async () => {
    const h = await mount();
    expect(page(h).dataset.maximized).toBe('true');
    expect(live(h).hidden).toBe(false);
    expect(h.querySelector('.wd-viewport')).not.toBeNull();
    expect(h.querySelector('.wd-stage')).not.toBeNull();
    expect(button(h, 'Verkleinern')).toBeDefined();
  });

  it('verkleinern und wieder maximieren ändert nur die Höhen, beide Vorschauen bleiben sichtbar', async () => {
    const h = await mount();
    await act(async () => button(h, 'Verkleinern').click());
    expect(page(h).dataset.maximized).toBe('false');
    expect(live(h).hidden).toBe(false);
    expect(h.querySelector('.wd-viewport')).not.toBeNull();
    await act(async () => button(h, 'Maximieren').click());
    expect(page(h).dataset.maximized).toBe('true');
    expect(live(h).hidden).toBe(false);
  });

  it('hat genau eine Positions-Leiste und der Streifen scrollt nicht selbst (B-477/AC-01)', async () => {
    const h = await mount();
    expect(h.querySelectorAll('input[aria-label="Position"]')).toHaveLength(1);
    expect(h.querySelector('.wd-viewport')!.tagName).toBe('DIV');
  });

  it('zeigt das Level ohne Spielserver: Abschnitte, Objekte, Legende, Zusammenfassung', async () => {
    const h = await mount();
    const level = mockWorldLevel('k3c', { biome: 'forest' });
    expect(h.querySelectorAll('.wd-chunk')).toHaveLength(level.chunks.length);
    expect(h.querySelectorAll('.wd-marker')).toHaveLength(level.entities.length);
    expect(h.querySelectorAll('.wd-hubline')).toHaveLength(1);
    expect(h.querySelector('.wd-legend')!.textContent).toContain('tree 2');
    expect(h.querySelector('[role="status"]')!.textContent).toContain(`${level.chunks.length} Abschnitte`);
  });

});

describe('Seite Welt-Betrachter: gemeinsame Ansicht (B-477/AC-01 bis AC-03)', () => {
  it('Zoom verbreitert den Streifen und endet an der Grenze, die Bühne bekommt denselben Maßstab (B-477/AC-03)', async () => {
    const h = await mount();
    const before = stripWidth(h);
    await act(async () => button(h, 'Hineinzoomen').click());
    expect(stripWidth(h)).toBeGreaterThan(before);
    for (let i = 0; i < 30; i++) await act(async () => button(h, 'Hineinzoomen').click());
    expect(stripWidth(h)).toBe(mockWorldLevel('k3c', { biome: 'forest' }).widthUnits * MAX_SCALE);
    expect(views.at(-1)!.scale).toBe(MAX_SCALE);
  });

  it('der Zoom-Regler und die festen Stufen setzen den Maßstab für beide (B-477/AC-03)', async () => {
    const h = await mount();
    const units = mockWorldLevel('k3c', { biome: 'forest' }).widthUnits;
    await setInput(h.querySelector<HTMLInputElement>('input[aria-label="Zoom"]')!, '20');
    expect(stripWidth(h)).toBe(units * 20);
    expect(views.at(-1)!.scale).toBe(20);
    const select = h.querySelector<HTMLSelectElement>('select[aria-label="Zoomstufe"]')!;
    await act(async () => { select.value = '50'; select.dispatchEvent(new Event('change', { bubbles: true })); });
    expect(stripWidth(h)).toBe(units * scaleOfPercent(50));
    expect(views.at(-1)!.scale).toBe(scaleOfPercent(50));
  });

  it('die Positions-Leiste bewegt Streifen und Bühne gemeinsam (B-477/AC-01)', async () => {
    const h = await mount();
    const viewport = h.querySelector<HTMLElement>('.wd-viewport')!;
    expect(position(h).max).not.toBe('0');
    await setInput(position(h), '200');
    expect(viewport.scrollLeft).toBe(200 * views.at(-1)!.scale);
    expect(views.at(-1)!.left).toBe(200);
    await setInput(position(h), '0');
    expect(viewport.scrollLeft).toBe(0);
    expect(views.at(-1)!.left).toBe(0);
  });

  it('Sprungpunkte stellen beide Vorschauen auf denselben Ort; fehlende sind ausgegraut (B-477/AC-02)', async () => {
    const h = await mount();
    const level = mockWorldLevel('k3c', { biome: 'forest' });
    const jump = (label: string) => [...h.querySelectorAll<HTMLButtonElement>('[aria-label="Sprungpunkte"] button')].find((b) => b.textContent === label)!;
    await act(async () => jump('Ende').click());
    expect(views.at(-1)!.left).toBeCloseTo(level.widthUnits - 1000 / views.at(-1)!.scale); // Ansicht 1000 px breit (nicht gemessen im Test)
    expect(h.querySelector<HTMLElement>('.wd-viewport')!.scrollLeft).toBe(views.at(-1)!.left * views.at(-1)!.scale);
    const left = views.at(-1)!.left;
    await act(async () => jump('Anfang').click());
    expect(views.at(-1)!.left).toBe(0);
    await act(async () => jump('Hub').click());
    expect(views.at(-1)!.left).toBeGreaterThan(0);
    expect(left).toBeGreaterThan(views.at(-1)!.left);
    expect(level.hubCenterUnits).toBeGreaterThan(0);
  });

});

describe('Seite Welt-Betrachter: Hinweise und Warnungen (B-460/AC-01)', () => {
  it('eine abgelehnte Anfrage zeigt den Hinweis statt eines Streifens', async () => {
    const h = await mount();
    const input = h.querySelector<HTMLInputElement>('input')!;
    Object.getOwnPropertyDescriptor(HTMLInputElement.prototype, 'value')!.set!.call(input, 'fehler');
    await act(async () => input.dispatchEvent(new Event('input', { bubbles: true })));
    await act(async () => button(h, 'Laden').click());
    await flush();
    expect(h.querySelector('.wd-strip')).toBeNull();
    expect(h.querySelector('[role="status"]')!.textContent).toContain('Seed braucht');
  });

  it('Warnungen der Prüfung stehen unter dem Streifen', async () => {
    const h = await mount();
    const input = h.querySelector<HTMLInputElement>('input')!;
    Object.getOwnPropertyDescriptor(HTMLInputElement.prototype, 'value')!.set!.call(input, 'warnung');
    await act(async () => input.dispatchEvent(new Event('input', { bubbles: true })));
    await act(async () => button(h, 'Laden').click());
    await flush();
    expect(h.querySelector('.wd-warnings')!.textContent).toBe('Level zu kurz');
  });
});

describe('Seite Welt-Betrachter: Tasten und Anfragen (B-460/AC-02)', () => {
  it('Taste M verkleinert und maximiert wieder, nicht beim Tippen im Eingabefeld (AC-02)', async () => {
    const h = await mount();
    const press = (key: string, target: EventTarget = document.body) => act(async () => { target.dispatchEvent(new KeyboardEvent('keydown', { key, bubbles: true })); });
    await press('m');
    expect(page(h).dataset.maximized).toBe('false');
    await press('M');
    expect(page(h).dataset.maximized).toBe('true');
    await press('m', h.querySelector('input')!);
    expect(page(h).dataset.maximized).toBe('true');
  });

  it('Tasten + und − zoomen, Pfeile scrollen', async () => {
    const h = await mount();
    const press = (key: string) => act(async () => { document.body.dispatchEvent(new KeyboardEvent('keydown', { key, bubbles: true })); });
    const base = stripWidth(h);
    await press('+');
    expect(stripWidth(h)).toBeGreaterThan(base);
    await press('-');
    await press('-');
    expect(stripWidth(h)).toBeLessThan(base);
    await setInput(position(h), String(Math.floor(Number(position(h).max) / 2)));
    const start = views.at(-1)!.left;
    const step = 120 / views.at(-1)!.scale; // 120 Pixel je Pfeiltaste
    await press('ArrowLeft');
    expect(views.at(-1)!.left).toBeCloseTo(start - step);
    await press('ArrowRight');
    await press('ArrowRight');
    expect(views.at(-1)!.left).toBeCloseTo(start + step);
  });

  it('bei zwei schnellen Anfragen gilt nur die letzte, auch wenn die erste später antwortet', async () => {
    let resolveFirst: (v: ReturnType<typeof mockWorldLevel>) => void = () => {};
    const first = new Promise<ReturnType<typeof mockWorldLevel>>((r) => { resolveFirst = r; });
    const spy = vi.spyOn(backend, 'worldLevel').mockImplementationOnce(() => first);
    const h = await mount(); // erste Anfrage hängt
    spy.mockImplementation(async (seed, source) => mockWorldLevel(seed, source));
    const input = h.querySelector<HTMLInputElement>('input')!;
    Object.getOwnPropertyDescriptor(HTMLInputElement.prototype, 'value')!.set!.call(input, 'zweite');
    await act(async () => input.dispatchEvent(new Event('input', { bubbles: true })));
    await act(async () => button(h, 'Laden').click());
    await flush();
    await act(async () => resolveFirst({ ...mockWorldLevel('erste', { biome: 'forest' }), widthUnits: 30, chunks: [] }));
    await flush();
    expect(h.querySelector('[role="status"]')!.textContent).toContain('„zweite“');
    expect(h.querySelectorAll('.wd-chunk')).toHaveLength(mockWorldLevel('x', { biome: 'forest' }).chunks.length);
  });
});
