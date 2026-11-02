// @vitest-environment happy-dom
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { Theme } from '@webkit/templates';
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import type { AssetChoice } from '../api';
import { clampRect, fitScale, nudgeRect } from './assets';
import { CutEditor } from './CutEditor';

// B-684: Umschalter Rahmen/Ausschnitt mit gemerkter Wahl, Grafik wird in die Kachel eingepasst.
(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

let observers: Array<(w: number) => void> = [];
class FakeResizeObserver {
  private cb: ResizeObserverCallback;
  constructor(cb: ResizeObserverCallback) { this.cb = cb; }
  observe() { observers.push((w) => this.cb([{ contentRect: { width: w } } as ResizeObserverEntry], this as unknown as ResizeObserver)); }
  disconnect() { /* nichts */ }
  unobserve() { /* nichts */ }
}

const choice: AssetChoice = { datei: 'grafik/gothicvania-town/props-einzeln/well.png', ausschnitt: { x: 4, y: 2, w: 30, h: 10 } };
let root: Root | null = null;
let host: HTMLElement | null = null;
const wait = (ms: number) => act(async () => { await new Promise((r) => setTimeout(r, ms)); });

async function mount(): Promise<void> {
  host = document.createElement('div');
  document.body.append(host);
  root = createRoot(host);
  await act(async () => root!.render(<Theme><CutEditor choice={choice} onCut={async () => true} /></Theme>));
  await wait(0);
  await wait(0);
  await act(async () => { observers.forEach((f) => f(200)); });
}
const btn = (text: string) => [...host!.querySelectorAll('button')].find((b) => b.textContent === text)!;
const stage = () => host!.querySelector<HTMLElement>('.as-stage')!;
const canvasWidth = () => parseFloat(stage().querySelector('canvas')!.style.width);

beforeEach(() => { observers = []; localStorage.clear(); vi.stubGlobal('ResizeObserver', FakeResizeObserver); });
afterEach(() => {
  act(() => root?.unmount());
  host?.remove();
  root = null;
  host = null;
  vi.unstubAllGlobals();
});

describe('CutEditor: Rahmen und Ausschnitt (B-684/AC-01)', () => {
  it('startet mit dem Rahmen, wechselt zur Ausschnitt-Ansicht und merkt die Wahl', async () => {
    await mount();
    expect(stage().dataset.view).toBe('rahmen');
    expect(btn('Rahmen').getAttribute('aria-pressed')).toBe('true');
    await act(async () => btn('Ausschnitt').click());
    expect(stage().dataset.view).toBe('ausschnitt');
    expect(localStorage.getItem('k3c-studio.assets.cutview')).toBe('ausschnitt');
    act(() => root?.unmount());
    host?.remove();
    observers = [];
    await mount();
    expect(stage().dataset.view).toBe('ausschnitt');
  });

  it('der Rahmen lässt sich per Tastatur verschieben und mit Shift in der Größe ändern', async () => {
    await mount();
    const canvas = stage().querySelector('canvas')!;
    await act(async () => { canvas.dispatchEvent(new KeyboardEvent('keydown', { key: 'ArrowRight', bubbles: true })); });
    expect((host!.querySelector('input[type=number]') as HTMLInputElement).value).toBe('5');
    await act(async () => { canvas.dispatchEvent(new KeyboardEvent('keydown', { key: 'ArrowDown', shiftKey: true, bubbles: true })); });
    expect((host!.querySelectorAll('input[type=number]')[3] as HTMLInputElement).value).toBe('11');
  });
});

describe('CutEditor: Einpassen (B-684/AC-02)', () => {
  it('der Maßstab folgt der Kachelbreite, auch nach dem Verschieben des Splitters, ohne Scrollen', async () => {
    await mount();
    const wide = canvasWidth();
    expect(wide).toBeCloseTo(65 * fitScale({ w: 200, h: 360 }, { w: 65, h: 65 }), 3);
    await act(async () => { observers.forEach((f) => f(100)); });
    expect(canvasWidth()).toBeCloseTo(wide / 2, 3);
    expect(canvasWidth()).toBeLessThanOrEqual(100);
    await act(async () => btn('Ausschnitt').click());
    expect(canvasWidth()).toBeCloseTo(100, 3); // 30×10 füllt die Breite
    expect(stage().querySelector('canvas')!.style.height).not.toBe('');
  });

  it('Pixelgrafik bleibt scharf, nichts scrollt', () => {
    const css = readFileSync(resolve(process.cwd(), 'tools/k3c-studio/frontend/src/styles/assets.css'), 'utf8');
    const rule = (sel: string) => css.split('\n').find((l) => l.startsWith(sel)) ?? '';
    expect(rule('.as-stage canvas')).toContain('image-rendering: pixelated');
    expect(rule('.as-stage {')).toContain('overflow: hidden');
  });
});

describe('Einpass-Logik', () => {
  it('fitScale streckt und schrumpft, ohne zu verzerren', () => {
    expect(fitScale({ w: 96, h: 96 }, { w: 128, h: 128 })).toBe(0.75);
    expect(fitScale({ w: 200, h: 100 }, { w: 20, h: 20 })).toBe(5);
    expect(fitScale({ w: 0, h: 100 }, { w: 20, h: 20 })).toBe(0);
  });
  it('clampRect hält den Rahmen in der Grafik; nudgeRect ignoriert andere Tasten', () => {
    expect(clampRect({ x: -3, y: 90, w: 50, h: 50 }, { w: 64, h: 64 })).toEqual({ x: 0, y: 14, w: 50, h: 50 });
    expect(nudgeRect({ x: 0, y: 0, w: 4, h: 4 }, 'a', false, { w: 8, h: 8 })).toBeUndefined();
    expect(nudgeRect({ x: 0, y: 0, w: 4, h: 4 }, 'ArrowLeft', false, { w: 8, h: 8 })).toEqual({ x: 0, y: 0, w: 4, h: 4 });
  });
});
