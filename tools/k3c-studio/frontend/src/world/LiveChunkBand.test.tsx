// @vitest-environment happy-dom
import { Theme } from '@radix-ui/themes';
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { backend } from '../api';
import { WorldPage } from './WorldPage';

// Chunk-Infos im Bodenstreifen der Live-Vorschau (B-485) im DOM: Bühne 600 × 400 px samt Rahmen, wie Phaser misst (happy-dom misst sonst 0).
const stage = { wheel: (_e: WheelEvent) => {} };
vi.mock('../stage/PreviewStage', () => ({ PreviewStage: class {
  constructor(_host: HTMLElement, _failed: unknown, _pan: unknown, wheel: (e: WheelEvent) => void) { stage.wheel = wheel; }
  show() {} refresh() {} setLook() {} setPosition() {} destroy() {}
} }));
(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

let root: Root | null = null;
let host: HTMLElement | null = null;
const restore: (() => void)[] = [];
const flush = () => act(async () => { for (let i = 0; i < 5; i++) await new Promise((r) => setTimeout(r, 0)); });

function stub(prop: 'offsetWidth' | 'offsetHeight', stageValue: number) {
  const original = Object.getOwnPropertyDescriptor(HTMLElement.prototype, prop)!;
  Object.defineProperty(HTMLElement.prototype, prop, { configurable: true, get(this: HTMLElement) { return this.classList.contains('wd-stage') ? stageValue : 0; } });
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
const altWheel = (deltaY: number) => act(() => {
  stage.wheel(Object.assign(new Event('wheel', { cancelable: true }), { deltaX: 0, deltaY, ctrlKey: false, shiftKey: false, altKey: true }) as WheelEvent);
});
const cells = (h: HTMLElement) => [...h.querySelectorAll<HTMLElement>('.wd-stage .wd-band .wd-band-chunk')];

beforeEach(async () => {
  stub('offsetWidth', 600);
  stub('offsetHeight', 400);
  const view = await backend.planLoad();
  await backend.planSave('island1', 0, [], 0, view.stand);
});
afterEach(() => {
  act(() => root?.unmount());
  host?.remove();
  while (restore.length > 0) restore.pop()!();
});

describe('Chunk-Infos am oberen Rand (B-485)', () => {
  it('liegt fest oben in der Bühne, eine Zelle je sichtbarem Chunk mit Name, Hub ohne Platz', async () => {
    const h = await mount();
    const band = h.querySelector<HTMLElement>('.wd-stage .wd-band')!;
    expect(parseFloat(band.style.top)).toBe(0); // fest am oberen Rand, wo im Spiel die HUD-Leiste liegt
    expect(parseFloat(band.style.height)).toBeGreaterThanOrEqual(20); // Höhe der HUD-Leiste, mindestens eine Zeile
    expect(cells(h).length).toBeGreaterThan(0);
    const hub = cells(h).find((c) => c.classList.contains('wd-band-hub'))!;
    expect(hub.textContent).toBe('🏰 Hub');
    expect(hub.dataset.mode).toBe('line');
  });

  it('herausgezoomt kompakt bzw. nur Farbe, hineingezoomt mehrzeilig; nie breiter als der Chunk', async () => {
    const h = await mount();
    for (let i = 0; i < 12; i++) altWheel(100);
    expect(cells(h).every((c) => c.dataset.mode === 'color' || c.dataset.mode === 'compact')).toBe(true);
    for (let i = 0; i < 24; i++) altWheel(-100);
    expect(cells(h).every((c) => c.dataset.mode === 'line')).toBe(true); // nur eine Zeile Inhalt: bleibt einzeilig
    expect(cells(h).every((c) => parseFloat(c.style.width) <= 30 * 48 && parseFloat(c.style.left) >= 0)).toBe(true); // Chunk 30 Units bei Maßstab 48
    expect(cells(h).every((c) => parseFloat(c.style.left) + parseFloat(c.style.width) <= 600)).toBe(true); // nie über die Bühne hinaus
  });

  it('ein geplanter Chunk zeigt blau Name, Platz, Seed und feste Objekte', async () => {
    const view = await backend.planLoad();
    await backend.planSave('island1', 0, [{ slot: 1, kind: 'forest', seed: 'abc', objects: [{ kind: 'tree', at: 5 }] }], 0, view.stand);
    const h = await mount();
    for (let i = 0; i < 6; i++) altWheel(-100); // Bodenstreifen hoch genug für drei Zeilen
    for (let i = 0; i < 3; i++) await act(async () => { document.body.dispatchEvent(new KeyboardEvent('keydown', { key: 'ArrowRight', bubbles: true })); });
    const planned = cells(h).find((c) => c.classList.contains('wd-band-planned'))!;
    expect(planned.dataset.mode).toBe('full');
    expect([...planned.querySelectorAll('.wd-band-line')].map((l) => l.textContent)).toEqual(['🧩 normal +1', 'Seed „abc“', '1 feste Obj.']); // Art aus dem Level (der Mock kennt nur „normal“)
  });
});
