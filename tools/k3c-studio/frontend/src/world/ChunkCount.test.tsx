// @vitest-environment happy-dom
import { Theme } from '@radix-ui/themes';
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { backend } from '../api';
import { WorldPage } from './WorldPage';

// B-485/AC-04 (WZ17.5) im DOM: Inselgröße je Stufe im Editor mit dem Mock-Backend (ohne Wails).
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
const size = (h: HTMLElement) => h.querySelector<HTMLSelectElement>('[aria-label="Inselgröße"]')!;

async function choose(h: HTMLElement, value: string) {
  const el = size(h);
  Object.getOwnPropertyDescriptor(HTMLSelectElement.prototype, 'value')!.set!.call(el, value);
  await act(async () => el.dispatchEvent(new Event('change', { bubbles: true })));
}

async function typeIn(el: HTMLInputElement, value: string) {
  Object.getOwnPropertyDescriptor(HTMLInputElement.prototype, 'value')!.set!.call(el, value);
  await act(async () => el.dispatchEvent(new Event('input', { bubbles: true })));
}

beforeEach(async () => { // der Mock hält Planung und Anzahl im Speicher: jeder Test beginnt mit Biom-Länge ohne Planung
  const view = await backend.planLoad();
  await backend.planSave('island1', 0, [], 0, view.stand);
});

afterEach(() => {
  vi.restoreAllMocks();
  act(() => root?.unmount());
  host?.remove();
});

describe('Inselgröße je Stufe (B-485/AC-04)', () => {
  it('bietet „Biom-Länge“ und die sechs Inselgrößen mit deutschem Namen und Anzahl, vorgewählt ist die Biom-Länge', async () => {
    const h = await mount();
    expect([...size(h).options].map((o) => o.textContent)).toEqual([
      'Biom-Länge', 'sehr klein · 16', 'klein · 24', 'mittel · 40', 'groß · 64', 'sehr groß · 100', 'mega groß · 160',
    ]);
    expect(size(h).value).toBe('0');
  });

  it('„klein“ setzt die Anzahl 24: Chunk- und Live-Vorschau fragen mit derselben Quelle, der Streifen hat 24 Chunks', async () => {
    const level = vi.spyOn(backend, 'worldLevel');
    const preview = vi.spyOn(backend, 'worldPreview');
    const h = await mount();
    await choose(h, '24');
    await settle();
    const chunkSource = level.mock.calls.at(-1)![1];
    const liveSource = preview.mock.calls.at(-1)![0].source;
    expect(chunkSource.chunkCount).toBe(24);
    expect({ island: liveSource.island, stage: liveSource.stage, planned: liveSource.planned, chunkCount: liveSource.chunkCount })
      .toEqual({ island: chunkSource.island, stage: chunkSource.stage, planned: chunkSource.planned, chunkCount: 24 });
    expect(h.querySelectorAll('.wd-chunk')).toHaveLength(24);
    expect(button(h, 'Speichern').disabled).toBe(false);
  });

  it('ein Platz jenseits der Anzahl zeigt den Fehler am Platz und sperrt Speichern', async () => {
    const h = await mount();
    await choose(h, '24');
    await act(async () => button(h, 'Chunk hinzufügen').click());
    await typeIn(h.querySelector<HTMLInputElement>('[aria-label="Platz +1"]')!, '11');
    await settle();
    expect(h.querySelector('.wd-planned[data-slot="11"] .wd-error')!.textContent).toContain('Platz +11');
    expect(button(h, 'Speichern').disabled).toBe(true);
  });

  it('ein Fehler der Anzahl (nicht erzeugbar) steht am Entwurf und sperrt Speichern', async () => {
    vi.spyOn(backend, 'planCheck').mockResolvedValue({ ok: false, slot: 0, error: 'Level nicht erzeugbar: kein Platz für die Portale bei Chunk-Anzahl 16' });
    const h = await mount();
    await choose(h, '16');
    await settle();
    expect(h.querySelector('.wd-plan > .wd-error')!.textContent).toContain('Chunk-Anzahl 16');
    expect(button(h, 'Speichern').disabled).toBe(true);
  });

});

describe('Inselgröße speichern und verwerfen (B-485/AC-04)', () => {
  it('Speichern schreibt die Anzahl mit REG-Hinweis; „Biom-Länge“ entfernt den Eintrag wieder', async () => {
    const save = vi.spyOn(backend, 'planSave');
    const h = await mount();
    await choose(h, '24');
    await settle();
    await act(async () => button(h, 'Speichern').click());
    await flush();
    expect(save.mock.calls.at(-1)!.slice(0, 4)).toEqual(['island1', 0, [], 24]);
    expect(h.querySelector('.wd-notice')!.textContent).toContain('REG-Änderung');
    expect(button(h, 'Speichern').disabled).toBe(true);
    expect((await backend.planLoad()).islands[0].chunkCounts).toEqual({ '0': 24 });
    expect(size(h).value).toBe('24');
    await choose(h, '0');
    await settle();
    await act(async () => button(h, 'Speichern').click());
    await flush();
    expect((await backend.planLoad()).islands[0].chunkCounts).toEqual({});
  });

  it('Verwerfen stellt die gespeicherte Größe wieder her', async () => {
    const h = await mount();
    await choose(h, '160');
    expect(button(h, 'Verwerfen').disabled).toBe(false);
    await act(async () => button(h, 'Verwerfen').click());
    expect(size(h).value).toBe('0');
    expect(button(h, 'Speichern').disabled).toBe(true);
  });
});
