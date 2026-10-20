// @vitest-environment happy-dom
import { Theme } from '@radix-ui/themes';
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { backend } from '../api';
import { GameLauncher } from './GameLauncher';

// B-460/AC-06, B-513 im DOM: „Im Spiel öffnen“ neu und voll ausgebaut ruft das Binding des gewählten Ziels (interne Engine oder Spielserver);
// nicht startbare Seeds und Biome sind ausgegraut mit Grund, die Wahl bleibt gemerkt.
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
const target = (h: HTMLElement) => h.querySelector<HTMLSelectElement>('select[aria-label="Spiel läuft auf"]')!;
const SERVER = 'http://127.0.0.1:8080';

async function choose(h: HTMLElement, value: string) {
  const select = target(h);
  select.value = value;
  await act(async () => select.dispatchEvent(new Event('change', { bubbles: true })));
}

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
    expect(target(h).value).toBe('engine');
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

  it('eine Ausnahme des Bindings wird ein Hinweis, die Knöpfe sind wieder frei', async () => {
    vi.spyOn(backend, 'worldOpenGame').mockRejectedValue(new Error('Bindung fehlt'));
    const h = await mount('probe', 'forest');
    await act(async () => button(h, 'Im Spiel öffnen').click());
    await flush();
    expect(h.textContent).toContain('Bindung fehlt');
    expect(button(h, 'Im Spiel öffnen').disabled).toBe(false);
  });
});

describe('Spiel-Adresse wählen (B-513)', () => {
  it('Liste: interne Engine und erkannter Spielserver mit Adresse und Heimnetz-Adresse', async () => {
    const h = await mount('probe', 'forest');
    const options = [...target(h).options];
    expect(options.map((o) => o.value)).toEqual(['engine', SERVER]);
    expect(options[0].textContent).toContain('Interne Engine des Studios');
    expect(options[1].textContent).toContain(`Spielserver ${SERVER}`);
    expect(options[1].textContent).toContain('192.168.1.20:8080');
    expect(options[1].disabled).toBe(false);
  });

  it('Spielserver gewählt: ruft das Server-Binding mit seiner Adresse und merkt die Wahl', async () => {
    const engine = vi.spyOn(backend, 'worldOpenGame');
    const server = vi.spyOn(backend, 'worldOpenGameServer');
    const h = await mount('probe', 'forest');
    await choose(h, SERVER);
    expect(localStorage.getItem('k3c-studio.world.gameTarget')).toBe(SERVER);
    await act(async () => button(h, 'Voll ausgebaut öffnen').click());
    await flush();
    expect(server).toHaveBeenCalledWith(SERVER, 'probe', 'forest', 'full');
    expect(engine).not.toHaveBeenCalled();
    act(() => root?.unmount());
    host?.remove();
    const again = await mount('probe', 'forest');
    expect(target(again).value).toBe(SERVER);
  });

  it('nicht erreichbarer Server: ausgegraut mit Hinweis, gemerkte Wahl fällt auf die Engine zurück', async () => {
    vi.spyOn(backend, 'worldGameServers').mockResolvedValue({ servers: [{ base: SERVER, lan: '', host: 'pc', reachable: false, dev: false, version: '' }] });
    localStorage.setItem('k3c-studio.world.gameTarget', SERVER);
    const engine = vi.spyOn(backend, 'worldOpenGame');
    const h = await mount('probe', 'forest');
    const option = [...target(h).options].find((o) => o.value === SERVER)!;
    expect(option.disabled).toBe(true);
    expect(option.textContent).toContain('nicht erreichbar');
    expect(target(h).value).toBe('engine');
    await act(async () => button(h, 'Im Spiel öffnen').click());
    await flush();
    expect(engine).toHaveBeenCalledWith('http://127.0.0.1:5183', 'probe', 'forest', 'new');
  });

  it('Server ohne Dev-Mode wird genannt; „Neu suchen“ prüft erneut', async () => {
    const spy = vi.spyOn(backend, 'worldGameServers').mockResolvedValue({ servers: [{ base: SERVER, lan: '', host: 'pc', reachable: true, dev: false, version: 'v1' }] });
    const h = await mount('probe', 'forest');
    expect(target(h).options[1].textContent).toContain('ohne Dev-Mode');
    await act(async () => button(h, 'Neu suchen').click());
    await flush();
    expect(spy).toHaveBeenCalledTimes(2);
  });
});
