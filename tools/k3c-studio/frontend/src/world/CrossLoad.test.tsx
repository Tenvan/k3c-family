// @vitest-environment happy-dom
import { Theme } from '@radix-ui/themes';
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, describe, expect, it, vi } from 'vitest';
import { backend, type WorldDetail, type WorldIsland, type WorldLevel } from '../api';
import { Overview } from './Overview';

// Review B-963: Der Querschnitt lädt nur Fehlendes (höchstens 3 gleichzeitig), die Etappe löst kein Neuladen aus,
// eine Stufe lässt sich gezielt neu laden; das Etappe-Feld speichert bei Blur/Enter und setzt bei Ablehnung zurück.

(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

const island = (id: string, etappe?: number): WorldIsland =>
  ({ id, name: `Insel ${id}`, depths: [0, 1, 2], biomes: ['forest', 'cave', 'mine'], plannedChunks: {}, chunkCounts: {}, ...(etappe ? { etappe } : {}) });
const detail = (etappe?: number): WorldDetail => ({ id: 'w', name: 'W', notes: '', extends: '', error: '', islands: [island('a', 1), island('b', etappe)] });

const level = (): WorldLevel => ({
  seed: 'k3c', biomeId: 'forest', widthUnits: 500, chunkWidthUnits: 50, hubCenterUnits: 250,
  chunks: [{ index: 0, kind: 'hub', startUnits: 0 }], entities: [], planned: [], warnings: [], error: '',
});

let root: Root | null = null;
afterEach(() => { act(() => root?.unmount()); root = null; document.body.innerHTML = ''; vi.restoreAllMocks(); });

const settle = async () => { for (let i = 0; i < 8; i++) await act(async () => new Promise((r) => setTimeout(r, 0))); };

interface Opts { d: WorldDetail; refresh?: string; onEtappe?: (i: string, e: number | null) => Promise<boolean> | void }

function render(o: Opts) {
  act(() => root!.render(<Theme><Overview detail={o.d} seed="k3c" choice={null} onPick={vi.fn()} refresh={o.refresh} onEtappe={o.onEtappe} /></Theme>));
}

async function start(o: Opts) {
  const host = document.createElement('div');
  document.body.appendChild(host);
  root = createRoot(host);
  render(o);
  await settle();
  return host;
}

describe('Querschnitt lädt gezielt', () => {
  it('Etappe ändern lädt nichts neu, Frühere Werte bleiben; höchstens 3 gleichzeitig', async () => {
    let running = 0;
    let peak = 0;
    const spy = vi.spyOn(backend, 'worldLevel').mockImplementation(async () => {
      peak = Math.max(peak, ++running);
      await new Promise((r) => setTimeout(r, 1));
      running--;
      return level();
    });
    const host = await start({ d: detail() });
    expect(spy).toHaveBeenCalledTimes(6);
    expect(peak).toBeLessThanOrEqual(3);
    render({ d: detail(2) });
    await settle();
    expect(spy).toHaveBeenCalledTimes(6);
    expect(host.querySelectorAll('.wd-x-bar')).toHaveLength(6);
  });

  it('refresh lädt nur die genannte Stufe neu', async () => {
    const spy = vi.spyOn(backend, 'worldLevel').mockImplementation(async () => level());
    await start({ d: detail() });
    spy.mockClear();
    render({ d: detail(), refresh: 'b:1#1' });
    await settle();
    expect(spy.mock.calls.map((c) => [c[1], c[2]])).toEqual([['b', 1]]);
  });
});

describe('Etappe-Feld', () => {
  const field = (host: HTMLElement) => host.querySelector('[data-island="b"] input') as HTMLInputElement;
  const type = (input: HTMLInputElement, v: string) => {
    const set = Object.getOwnPropertyDescriptor(HTMLInputElement.prototype, 'value')!.set!;
    set.call(input, v);
  };

  it('Blur speichert, leer = null, Ablehnung setzt zurück', async () => {
    vi.spyOn(backend, 'worldLevel').mockImplementation(async () => level());
    const onEtappe = vi.fn(async () => true);
    const host = await start({ d: detail(2), onEtappe });
    const input = field(host);
    expect(input.value).toBe('2');
    type(input, '1');
    await act(async () => { input.dispatchEvent(new FocusEvent('focusout', { bubbles: true })); });
    expect(onEtappe).toHaveBeenLastCalledWith('b', 1);
    type(input, '');
    await act(async () => { input.dispatchEvent(new FocusEvent('focusout', { bubbles: true })); });
    expect(onEtappe).toHaveBeenLastCalledWith('b', null);
    onEtappe.mockResolvedValueOnce(false);
    type(input, '5');
    await act(async () => { input.dispatchEvent(new FocusEvent('focusout', { bubbles: true })); });
    expect(input.value).toBe('2');
    onEtappe.mockClear();
    type(input, '0');
    await act(async () => { input.dispatchEvent(new FocusEvent('focusout', { bubbles: true })); });
    expect(onEtappe).not.toHaveBeenCalled();
    expect(input.value).toBe('2');
  });
});
