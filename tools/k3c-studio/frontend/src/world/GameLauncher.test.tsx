// @vitest-environment happy-dom
import { Theme } from '@radix-ui/themes';
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { backend } from '../api';
import { GameLauncher } from './GameLauncher';

// B-460/AC-06 im DOM: „Im Spiel öffnen“ neu und voll ausgebaut ruft das Binding mit der Adresse des Spielservers;
// nicht startbare Seeds und Biome sind ausgegraut mit Grund, die Adresse bleibt gemerkt.
(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

let root: Root | null = null;
let host: HTMLElement | null = null;
const flush = () => act(async () => { for (let i = 0; i < 5; i++) await new Promise((r) => setTimeout(r, 0)); });

async function mount(seed: string, biome: string): Promise<HTMLElement> {
  host = document.createElement('div');
  document.body.append(host);
  root = createRoot(host);
  await act(async () => root!.render(<Theme><GameLauncher seed={seed} biome={biome} /></Theme>));
  await flush();
  return host;
}

const button = (h: HTMLElement, label: string) => [...h.querySelectorAll('button')].find((b) => b.textContent === label)!;
const base = (h: HTMLElement) => h.querySelector<HTMLInputElement>('input[aria-label="Spielserver"]')!;

beforeEach(() => localStorage.clear());
afterEach(() => {
  vi.restoreAllMocks();
  act(() => root?.unmount());
  host?.remove();
});

describe('Im Spiel öffnen (B-460/AC-06)', () => {
  it('neu: ruft das Binding mit Adresse, Seed und Biom und meldet den Spielstand', async () => {
    const spy = vi.spyOn(backend, 'worldOpenGame');
    const h = await mount('probe', 'forest');
    expect(base(h).value).toBe('http://127.0.0.1:5183');
    await act(async () => button(h, 'Im Spiel öffnen').click());
    await flush();
    expect(spy).toHaveBeenCalledWith('http://127.0.0.1:5183', 'probe', 'forest', 'new');
    expect(h.textContent).toContain('Spielstand „probe“');
  });

  it('voll ausgebaut: Art full, auch außerhalb des Waldes', async () => {
    const spy = vi.spyOn(backend, 'worldOpenGame');
    const h = await mount('probe', 'cave');
    expect(button(h, 'Voll ausgebaut öffnen').disabled).toBe(false);
    await act(async () => button(h, 'Voll ausgebaut öffnen').click());
    await flush();
    expect(spy).toHaveBeenCalledWith('http://127.0.0.1:5183', 'probe', 'cave', 'full');
    expect(h.textContent).toContain('Spielstand „voll-probe“');
  });

  it('Höhle und unpassender Seed: neu ausgegraut mit Grund, es wird nichts aufgerufen', async () => {
    const spy = vi.spyOn(backend, 'worldOpenGame');
    const h = await mount('Probe', 'cave');
    const open = button(h, 'Im Spiel öffnen');
    expect(open.disabled).toBe(true);
    expect(open.title).toBe('Start nur für den Wald (Tiefe 0)');
    expect(h.textContent).toContain('nicht möglich: Start nur für den Wald');
    await act(async () => open.click());
    expect(spy).not.toHaveBeenCalled();
  });

  it('ein Hinweis des Servers (kein Dev-Mode) erscheint, nichts wird gemeldet als geöffnet', async () => {
    vi.spyOn(backend, 'worldOpenGame').mockResolvedValue({ url: '', save: '', error: 'Voll ausgebaut geht nur im Dev-Mode (Server mit K3C_DEV starten)' });
    const h = await mount('probe', 'forest');
    await act(async () => button(h, 'Voll ausgebaut öffnen').click());
    await flush();
    expect(h.textContent).toContain('Dev-Mode');
    expect(h.textContent).not.toContain('Im Browser geöffnet');
  });

  it('die Adresse des Spielservers bleibt gemerkt und geht ans Binding', async () => {
    const spy = vi.spyOn(backend, 'worldOpenGame');
    const h = await mount('probe', 'forest');
    const input = base(h);
    Object.getOwnPropertyDescriptor(HTMLInputElement.prototype, 'value')!.set!.call(input, 'http://localhost:8090');
    await act(async () => input.dispatchEvent(new Event('input', { bubbles: true })));
    expect(localStorage.getItem('k3c-studio.world.gameBase')).toBe('http://localhost:8090');
    await act(async () => button(h, 'Im Spiel öffnen').click());
    await flush();
    expect(spy).toHaveBeenCalledWith('http://localhost:8090', 'probe', 'forest', 'new');
  });

  it('eine Ausnahme des Bindings wird ein Hinweis, die Knöpfe sind wieder frei', async () => {
    vi.spyOn(backend, 'worldOpenGame').mockRejectedValue(new Error('Bindung fehlt'));
    const h = await mount('probe', 'forest');
    await act(async () => button(h, 'Im Spiel öffnen').click());
    await flush();
    expect(h.textContent).toContain('Bindung fehlt');
    expect(button(h, 'Im Spiel öffnen').disabled).toBe(false);
  });
});
