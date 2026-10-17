// @vitest-environment happy-dom
import { Theme } from '@radix-ui/themes';
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, describe, expect, it, vi } from 'vitest';
import { backend } from '../api';
import { mockWorldLevel } from '../api/mockWorld';
import { WorldPage } from './WorldPage';

// Die Phaser-Bühne braucht ein Canvas; ihr Zusammenspiel prüft LivePreview.test.tsx mit einer Attrappe.
vi.mock('../stage/PreviewStage', () => ({ PreviewStage: class { show() {} refresh() {} destroy() {} } }));

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
const chunks = (h: HTMLElement) => h.querySelector<HTMLElement>('.wd-chunks')!;

afterEach(() => {
  vi.restoreAllMocks();
  act(() => root?.unmount());
  host?.remove();
});

describe('Seite Welt-Betrachter: Chunk-Vorschau (B-460/AC-01, AC-02)', () => {
  it('ist beim Öffnen maximiert: Live-Vorschau eingeklappt, Knopf „Verkleinern“', async () => {
    const h = await mount();
    expect(chunks(h).dataset.maximized).toBe('true');
    expect(chunks(h).classList.contains('wd-max')).toBe(true);
    expect(live(h).hidden).toBe(true);
    expect(button(h, 'Verkleinern')).toBeDefined();
  });

  it('verkleinern und wieder maximieren, die Live-Vorschau folgt', async () => {
    const h = await mount();
    await act(async () => button(h, 'Verkleinern').click());
    expect(chunks(h).dataset.maximized).toBe('false');
    expect(chunks(h).classList.contains('wd-max')).toBe(false);
    expect(live(h).hidden).toBe(false);
    await act(async () => button(h, 'Maximieren').click());
    expect(chunks(h).dataset.maximized).toBe('true');
    expect(live(h).hidden).toBe(true);
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

  it('Zoom verbreitert den Streifen und endet an der Grenze', async () => {
    const h = await mount();
    const width = () => parseFloat(h.querySelector<HTMLElement>('.wd-strip')!.style.width);
    const before = width();
    await act(async () => button(h, 'Hineinzoomen').click());
    expect(width()).toBeGreaterThan(before);
    for (let i = 0; i < 20; i++) await act(async () => button(h, 'Hineinzoomen').click());
    expect(width()).toBe(mockWorldLevel('k3c', { biome: 'forest' }).widthUnits * 24);
  });

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
    expect(chunks(h).dataset.maximized).toBe('false');
    await press('M');
    expect(chunks(h).dataset.maximized).toBe('true');
    await press('m', h.querySelector('input')!);
    expect(chunks(h).dataset.maximized).toBe('true');
  });

  it('Tasten + und − zoomen, Pfeile scrollen (wie leveltest.html)', async () => {
    const h = await mount();
    const press = (key: string) => act(async () => { document.body.dispatchEvent(new KeyboardEvent('keydown', { key, bubbles: true })); });
    const width = () => parseFloat(h.querySelector<HTMLElement>('.wd-strip')!.style.width);
    const base = width();
    await press('+');
    expect(width()).toBeGreaterThan(base);
    await press('-');
    await press('-');
    expect(width()).toBeLessThan(base);
    const viewport = h.querySelector<HTMLElement>('.wd-viewport')!;
    viewport.scrollLeft = 300;
    await press('ArrowLeft');
    expect(viewport.scrollLeft).toBe(180);
    await press('ArrowRight');
    await press('ArrowRight');
    expect(viewport.scrollLeft).toBe(420);
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
