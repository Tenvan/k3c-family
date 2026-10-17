// @vitest-environment happy-dom
import { Theme } from '@radix-ui/themes';
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { backend } from '../api';
import { LivePreview } from './LivePreview';

// B-460/AC-05 im DOM: Tageszeit mit vier Einträgen und Fortschritt gehen als Sim-Phase an die Vorschau, die Helligkeit
// folgt daylight(), Hintergrund-Schichten sind schaltbar (Biom ohne Schichten ausgegraut). Die Phaser-Bühne ersetzt eine Attrappe.
(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

interface Look { brightness: number; hiddenLayers: ReadonlySet<string> }
const looks: Look[] = [];
vi.mock('./PreviewStage', () => ({
  PreviewStage: class {
    show() {}
    refresh() {}
    destroy() {}
    setLook(look: Look) { looks.push(look); }
  },
}));

let root: Root | null = null;
let host: HTMLElement | null = null;
const flush = () => act(async () => { for (let i = 0; i < 5; i++) await new Promise((r) => setTimeout(r, 0)); });

async function mount(biome = 'forest'): Promise<HTMLElement> {
  host = document.createElement('div');
  document.body.append(host);
  root = createRoot(host);
  await act(async () => root!.render(<Theme><LivePreview seed="k3c" biome={biome} active /></Theme>));
  await flush();
  return host;
}

const slider = (h: HTMLElement) => h.querySelector<HTMLInputElement>('input[type="range"]')!;
const layers = (h: HTMLElement) => [...h.querySelectorAll<HTMLElement>('.wd-bar button[role="checkbox"]')];
const lastLook = () => looks.at(-1)!;

async function setProgress(h: HTMLElement, value: string) {
  Object.getOwnPropertyDescriptor(HTMLInputElement.prototype, 'value')!.set!.call(slider(h), value);
  await act(async () => slider(h).dispatchEvent(new Event('input', { bubbles: true })));
  await act(async () => { await new Promise((r) => setTimeout(r, 200)); });
  await flush();
}

beforeEach(() => { looks.length = 0; });
afterEach(() => {
  vi.restoreAllMocks();
  act(() => root?.unmount());
  host?.remove();
});

async function chooseTime(h: HTMLElement, id: string) {
  const select = h.querySelector<HTMLSelectElement>('select[aria-label="Tageszeit"]')!;
  Object.getOwnPropertyDescriptor(HTMLSelectElement.prototype, 'value')!.set!.call(select, id);
  await act(async () => select.dispatchEvent(new Event('change', { bubbles: true })));
  await flush();
}

describe('Tageszeit in der Live-Vorschau (B-460/AC-05)', () => {
  it('vier Einträge Tag, Dämmerung, Nacht, Morgengrauen; Start mit Tag, Fortschritt 0, voller Helligkeit', async () => {
    const spy = vi.spyOn(backend, 'worldPreview');
    const h = await mount();
    const labels = [...h.querySelectorAll<HTMLOptionElement>('select[aria-label="Tageszeit"] option')].map((o) => o.textContent);
    expect(labels).toEqual(['Tag', 'Dämmerung', 'Nacht', 'Morgengrauen']);
    expect(spy).toHaveBeenCalledWith(expect.objectContaining({ phase: 'day', progress: 0 }));
    expect(lastLook().brightness).toBe(1);
  });

  it('Dämmerung mit Fortschritt 0,5: Phase dusk, Helligkeit nach daylight() (1 − p)', async () => {
    const spy = vi.spyOn(backend, 'worldPreview');
    const h = await mount();
    await chooseTime(h, 'dusk');
    await setProgress(h, '0.5');
    expect(spy).toHaveBeenLastCalledWith(expect.objectContaining({ phase: 'dusk', progress: 0.5 }));
    expect(lastLook().brightness).toBeCloseTo(0.45 + 0.55 * 0.5, 10);
  });

  it('Nacht: tiefste Helligkeit', async () => {
    const spy = vi.spyOn(backend, 'worldPreview');
    const h = await mount();
    await chooseTime(h, 'night');
    expect(spy).toHaveBeenLastCalledWith(expect.objectContaining({ phase: 'night', progress: 0 }));
    expect(lastLook().brightness).toBeCloseTo(0.45, 10);
  });

  it('Morgengrauen: Sim-Phase night mit 0,9 + 0,1·p (p = 0,5 → 0,95), Helligkeit nach daylight() gleich p', async () => {
    const spy = vi.spyOn(backend, 'worldPreview');
    const h = await mount();
    await chooseTime(h, 'dawn');
    await setProgress(h, '0.5');
    const args = spy.mock.calls.at(-1)![0];
    expect(args.phase).toBe('night');
    expect(args.progress).toBeCloseTo(0.95, 10);
    expect(lastLook().brightness).toBeCloseTo(0.45 + 0.55 * 0.5, 10);
  });
});

describe('Hintergrund-Schichten in der Live-Vorschau (B-460/AC-05)', () => {
  it('Wald: drei Schichten von fern nach nah, an; Abwählen blendet genau diese Schicht aus', async () => {
    const h = await mount('forest');
    expect(layers(h).map((b) => b.getAttribute('aria-label'))).toEqual(['bg:forest:1', 'bg:forest:2', 'bg:forest:3']);
    expect(h.textContent).toContain('Schicht 1 (fern)');
    expect(h.textContent).toContain('Schicht 3 (nah)');
    expect(lastLook().hiddenLayers.size).toBe(0);
    await act(async () => layers(h)[1]!.click());
    expect([...lastLook().hiddenLayers]).toEqual(['bg:forest:2']);
    await act(async () => layers(h)[1]!.click());
    expect(lastLook().hiddenLayers.size).toBe(0);
  });

  it('Biom ohne Schichten: Hinweis, keine Schalter', async () => {
    const h = await mount('nirgendwo');
    expect(layers(h)).toHaveLength(1);
    expect(layers(h)[0]!.hasAttribute('disabled')).toBe(true);
    expect(h.textContent).toContain('keine Hintergrund-Schichten in diesem Biom');
  });
});
