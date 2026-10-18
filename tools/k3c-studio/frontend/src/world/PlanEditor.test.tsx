// @vitest-environment happy-dom
import { Theme } from '@radix-ui/themes';
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { backend } from '../api';
import { WorldPage } from './WorldPage';

// B-461/AC-06, AC-09, AC-11, AC-12 im DOM: der Editor für geplante Chunks mit dem Mock-Backend (ohne Wails).
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

async function type(el: HTMLInputElement, value: string) {
  Object.getOwnPropertyDescriptor(HTMLInputElement.prototype, 'value')!.set!.call(el, value);
  await act(async () => el.dispatchEvent(new Event('input', { bubbles: true })));
}

/** Entwurf am Platz +1 anlegen (Standard-Art des Bioms) */
async function addChunk(h: HTMLElement) {
  await act(async () => button(h, 'Chunk hinzufügen').click());
  await flush();
}

beforeEach(async () => { // der Mock hält die Planung im Speicher: jeder Test beginnt ohne gespeicherte Planung
  const view = await backend.planLoad();
  await backend.planSave('island1', 0, [], view.stand);
});

afterEach(() => {
  vi.restoreAllMocks();
  act(() => root?.unmount());
  host?.remove();
});

describe('Editor für geplante Chunks (B-461/AC-09)', () => {
  it('zeigt Insel, Stufe und „keine Planung“, solange nichts geplant ist', async () => {
    const h = await mount();
    expect(field(h, 'Insel')).not.toBeNull();
    expect(field(h, 'Stufe')).not.toBeNull();
    expect(h.querySelector('.wd-chunklist')!.textContent).toContain('Keine Planung');
    expect(button(h, 'Speichern').disabled).toBe(true);
  });

  it('Chunk anlegen, Art, Chunk-Seed und Objekt bearbeiten; Speichern erst bei gültigem Entwurf', async () => {
    const h = await mount();
    await addChunk(h);
    expect(h.querySelectorAll('.wd-planned')).toHaveLength(1);
    await type(field(h, 'Art Platz +1'), 'clearing');
    await type(field(h, 'Chunk-Seed Platz +1'), 'lichtung');
    await act(async () => button(h, 'Objekt').click());
    await type(field(h, 'Lage Objekt 1 Platz +1'), '30');
    await settle();
    expect(field(h, 'Art Platz +1').value).toBe('clearing');
    expect(field(h, 'Lage Objekt 1 Platz +1').value).toBe('30');
    expect(button(h, 'Speichern').disabled).toBe(false);
  });

  it('Speichern zeigt den REG-Hinweis, danach ist der Entwurf nicht mehr geändert', async () => {
    const h = await mount();
    await addChunk(h);
    await settle();
    await act(async () => button(h, 'Speichern').click());
    await flush();
    expect(h.querySelector('.wd-notice')!.textContent).toContain('REG-Änderung: Commit mit Begründung');
    expect(button(h, 'Speichern').disabled).toBe(true);
    expect(h.querySelectorAll('.wd-planned')).toHaveLength(1);
  });

  it('Verwerfen setzt den Entwurf auf die gespeicherte Planung zurück', async () => {
    const h = await mount();
    await addChunk(h);
    await act(async () => button(h, 'Verwerfen').click());
    await flush();
    expect(h.querySelectorAll('.wd-planned')).toHaveLength(0);
  });

  it('Chunk löschen', async () => {
    const h = await mount();
    await addChunk(h);
    await act(async () => button(h, 'Chunk Platz +1 löschen').click());
    expect(h.querySelectorAll('.wd-planned')).toHaveLength(0);
  });
});

describe('Prüffehler am Platz (B-461/AC-06)', () => {
  it('ein ungültiger Entwurf zeigt den Fehler an seinem Platz, Speichern ist gesperrt', async () => {
    const h = await mount();
    await addChunk(h);
    await type(field(h, 'Art Platz +1'), 'volcano');
    await settle();
    const error = h.querySelector<HTMLElement>('.wd-planned[data-slot="1"] .wd-error')!;
    expect(error.textContent).toContain('Platz +1');
    expect(error.textContent).toContain('volcano');
    expect(button(h, 'Speichern').disabled).toBe(true);
  });

  it('der Fehler zu Platz 0 steht an dem Chunk mit Platz 0', async () => {
    const h = await mount();
    await addChunk(h);
    await type(field(h, 'Platz +1'), '0');
    await settle();
    expect(h.querySelector('.wd-planned[data-slot="0"] .wd-error')!.textContent).toContain('Platz 0');
  });

  it('ein Fehler ohne Platz in der Liste (Summen, kein Platz mehr) steht über der Liste', async () => {
    vi.spyOn(backend, 'planCheck').mockResolvedValue({ ok: false, slot: 0, error: 'kein Platz mehr: 5 freie Chunks, gebraucht bis zu 9' });
    const h = await mount();
    await addChunk(h);
    await settle();
    expect(h.querySelector('.wd-plan > .wd-error')!.textContent).toContain('kein Platz mehr');
    expect(button(h, 'Speichern').disabled).toBe(true);
  });
});

describe('Stand-Prüfung (B-461/AC-12)', () => {
  it('eine von außen geänderte Datei lehnt das Speichern ab, der Entwurf bleibt', async () => {
    const h = await mount();
    await addChunk(h);
    await settle();
    (backend as unknown as { touch: () => void }).touch();
    await act(async () => button(h, 'Speichern').click());
    await flush();
    expect(h.querySelector('.wd-notice')!.textContent).toContain('data/islands.json geändert, bitte neu laden');
    expect(h.querySelector('.wd-notice')!.textContent).toContain('Entwurf bleibt');
    expect(h.querySelectorAll('.wd-planned')).toHaveLength(1);
    expect(button(h, 'Speichern').disabled).toBe(false);
  });
});

describe('Speichern schlägt fehl (B-486)', () => {
  it('wirft der Aufruf (Bindung, Serialisierung), steht der Grund am Editor und der Entwurf bleibt', async () => {
    vi.spyOn(backend, 'planSave').mockRejectedValue('Wails: Argument nicht lesbar');
    const h = await mount();
    await addChunk(h);
    await settle();
    await act(async () => button(h, 'Speichern').click());
    await flush();
    expect(h.querySelector('.wd-notice')!.textContent).toContain('Speichern fehlgeschlagen');
    expect(h.querySelector('.wd-notice')!.textContent).toContain('Wails: Argument nicht lesbar');
    expect(h.querySelectorAll('.wd-planned')).toHaveLength(1);
    expect(button(h, 'Speichern').disabled).toBe(false);
  });
});

describe('Chunk- und Live-Vorschau lesen dieselbe Quelle (B-461/AC-11)', () => {
  it('beide fragen mit Insel, Stufe und demselben Entwurf', async () => {
    const level = vi.spyOn(backend, 'worldLevel');
    const preview = vi.spyOn(backend, 'worldPreview');
    const h = await mount();
    await addChunk(h);
    await settle();
    const lastLevel = level.mock.calls.at(-1)![1];
    const lastPreview = preview.mock.calls.at(-1)![0].source;
    expect(lastLevel.island).toBe('island1');
    expect(lastLevel.stage).toBe(0);
    expect(lastLevel.planned).toHaveLength(1);
    expect({ island: lastPreview.island, stage: lastPreview.stage, planned: lastPreview.planned }).toEqual({ island: lastLevel.island, stage: lastLevel.stage, planned: lastLevel.planned });
  });

  it('bei einem ungültigen Entwurf bleibt das letzte gültige Bild stehen', async () => {
    const level = vi.spyOn(backend, 'worldLevel');
    const h = await mount();
    await addChunk(h);
    await settle();
    const calls = level.mock.calls.length;
    await type(field(h, 'Art Platz +1'), 'volcano');
    await settle();
    expect(level.mock.calls.length).toBe(calls);
    expect(h.querySelector('.wd-strip')).not.toBeNull();
  });
});
