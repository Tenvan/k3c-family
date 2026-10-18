// @vitest-environment happy-dom
import { Theme } from '@radix-ui/themes';
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { backend } from '../api';
import { WorldPage } from './WorldPage';

// B-485/AC-06 (WZ17.5) im DOM: Seed-Chunk in feste Objekte umwandeln, mit dem Mock-Backend (ohne Wails). Dass das Bild
// gleich bleibt, prüft der Go-Test TestUmwandlungAendertDasBildNicht am echten Level.
vi.mock('../stage/PreviewStage', () => ({ PreviewStage: class { show() {} refresh() {} setLook() {} setPosition() {} destroy() {} } }));
(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

let root: Root | null = null;
let host: HTMLElement | null = null;

const wait = (ms = 0) => act(async () => { await new Promise((r) => setTimeout(r, ms)); });
const flush = async () => { for (let i = 0; i < 4; i++) await wait(); };
const settle = async () => { await wait(260); await flush(); }; // Prüfung ist um 200 ms verzögert

async function mount(): Promise<HTMLElement> {
  host = document.createElement('div');
  document.body.append(host);
  root = createRoot(host);
  await act(async () => root!.render(<Theme><WorldPage /></Theme>));
  await flush();
  return host;
}

const button = (h: HTMLElement, label: string) => [...h.querySelectorAll('button')].find((b) => b.textContent === label || b.getAttribute('aria-label') === label)!;
const field = (h: HTMLElement, label: string) => h.querySelector<HTMLInputElement>(`[aria-label="${label}"]`)!;
const CONVERT = 'In feste Objekte umwandeln Platz +1';

async function type(el: HTMLInputElement, value: string) {
  Object.getOwnPropertyDescriptor(HTMLInputElement.prototype, 'value')!.set!.call(el, value);
  await act(async () => el.dispatchEvent(new Event('input', { bubbles: true })));
}

/** Chunk am Platz +1 mit Chunk-Seed anlegen und die Prüfung abwarten */
async function seedChunk(h: HTMLElement, seed = 'lichtung') {
  await act(async () => button(h, 'Chunk hinzufügen').click());
  await type(field(h, 'Art Platz +1'), 'clearing');
  if (seed) await type(field(h, 'Chunk-Seed Platz +1'), seed);
  await settle();
}

beforeEach(async () => {
  const view = await backend.planLoad();
  await backend.planSave('island1', 0, [], 0, view.stand);
});

afterEach(() => {
  vi.restoreAllMocks();
  act(() => root?.unmount());
  host?.remove();
});

describe('Seed-Chunk in feste Objekte umwandeln (B-485/AC-06)', () => {
  it('ohne Chunk-Seed gesperrt, der Hinweis nennt den Grund', async () => {
    const h = await mount();
    await seedChunk(h, '');
    expect(button(h, CONVERT).disabled).toBe(true);
    expect(button(h, CONVERT).title).toContain('Chunk-Seed');
  });

  it('mit Chunk-Seed: Hinweis „Das Bild bleibt gleich“; Umwandeln übernimmt die Objekte und entfernt den Seed', async () => {
    const objects = vi.spyOn(backend, 'planChunkObjects');
    const level = vi.spyOn(backend, 'worldLevel');
    const h = await mount();
    await seedChunk(h);
    expect(button(h, CONVERT).disabled).toBe(false);
    expect(button(h, CONVERT).title).toBe('Das Bild bleibt gleich');
    await act(async () => button(h, CONVERT).click());
    await flush();
    const [seed, source, slot] = objects.mock.calls.at(-1)!;
    expect([seed, slot, source.island, source.stage, source.chunkCount]).toEqual(['k3c', 1, 'island1', 0, 0]);
    expect(source.planned).toEqual([{ slot: 1, kind: 'clearing', seed: 'lichtung' }]);
    const want = (await objects.mock.results.at(-1)!.value).objects;
    expect(want.length).toBeGreaterThan(0);
    expect(field(h, 'Chunk-Seed Platz +1').value).toBe('');
    expect(field(h, `Lage Objekt ${want.length} Platz +1`).value).toBe(String(want.at(-1).at));
    expect(button(h, CONVERT).disabled).toBe(true); // jetzt feste Objekte
    await settle();
    expect(level.mock.calls.at(-1)![1].planned).toEqual([{ slot: 1, kind: 'clearing', objects: want }]);
  });

  it('mit festen Objekten gesperrt', async () => {
    const h = await mount();
    await seedChunk(h);
    await act(async () => button(h, 'Objekt').click());
    await settle();
    expect(button(h, CONVERT).disabled).toBe(true);
    expect(button(h, CONVERT).title).toContain('feste Objekte');
  });

  it('solange der Entwurf ungültig ist, gesperrt', async () => {
    const h = await mount();
    await seedChunk(h);
    await type(field(h, 'Art Platz +1'), 'volcano');
    await settle();
    expect(button(h, CONVERT).disabled).toBe(true);
  });

  it('ein Fehler der Umwandlung steht am Chunk, der Entwurf bleibt', async () => {
    vi.spyOn(backend, 'planChunkObjects').mockRejectedValue('Wails: Bindung fehlt');
    const h = await mount();
    await seedChunk(h);
    await act(async () => button(h, CONVERT).click());
    await flush();
    expect(h.querySelector('.wd-planned[data-slot="1"] .wd-error')!.textContent).toContain('Bindung fehlt');
    expect(field(h, 'Chunk-Seed Platz +1').value).toBe('lichtung');
  });
});
