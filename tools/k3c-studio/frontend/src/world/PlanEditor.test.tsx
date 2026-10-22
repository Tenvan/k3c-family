// @vitest-environment happy-dom
import { Theme } from '@radix-ui/themes';
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { backend } from '../api';
import { iconOf, nameOf } from './kindIcons';
import { CHUNK_KINDS, OBJECT_KINDS } from './plan';
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
  await backend.planSave('island1', 0, [], 0, view.stand);
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
    expect(h.querySelector('.wd-notice')!.textContent).toContain('data/worlds/standard.json geändert, bitte neu laden');
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

describe('Laden und Prüfen abgelehnt (B-487)', () => {
  it('lehnt planLoad ab, steht „Laden fehlgeschlagen: <Grund>“ am Editor', async () => {
    vi.spyOn(backend, 'planLoad').mockRejectedValue('Wails: Bindung fehlt');
    const h = await mount();
    expect(h.querySelector('section[aria-label="Planung"]')!.textContent).toContain('Laden fehlgeschlagen: Wails: Bindung fehlt');
  });

  it('lehnt planCheck ab, ist Speichern gesperrt, der Grund steht am Editor und der Entwurf bleibt', async () => {
    const h = await mount();
    vi.spyOn(backend, 'planCheck').mockRejectedValue('Wails: Argument nicht lesbar');
    await addChunk(h);
    await settle();
    expect(h.querySelector('.wd-error')!.textContent).toContain('Prüfung fehlgeschlagen: Wails: Argument nicht lesbar');
    expect(button(h, 'Speichern').disabled).toBe(true);
    expect(h.querySelectorAll('.wd-planned')).toHaveLength(1);
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

describe('Durchsuchbare Auswahlliste mit Emoji (B-485/AC-01)', () => {
  const options = (h: HTMLElement, label: string) => [...h.querySelectorAll<HTMLElement>(`[role="listbox"][aria-label="${label} Liste"] [role="option"]`)];
  const EMOJI_TEXT = /^\p{Extended_Pictographic} \S/u;

  it('Art: großes Dropdown mit allen Einträgen und Emoji; Tippen stellt Treffer nach oben, der Rest bleibt abgeblendet', async () => {
    const h = await mount();
    await addChunk(h);
    const art = field(h, 'Art Platz +1');
    expect(options(h, 'Art Platz +1')).toHaveLength(0); // zu, bis man ins Feld klickt
    await act(async () => art.click());
    const all = options(h, 'Art Platz +1');
    expect(all).toHaveLength(CHUNK_KINDS.length);
    expect(all.every((o) => EMOJI_TEXT.test(o.textContent ?? ''))).toBe(true);
    expect(h.querySelectorAll('[role="option"].wd-dim')).toHaveLength(0);
    await type(art, 'cr');
    const ranked = options(h, 'Art Platz +1');
    expect(ranked).toHaveLength(CHUNK_KINDS.length);
    expect(ranked[0].textContent).toBe(`${iconOf('crystals')} Kristalle`);
    expect(ranked.filter((o) => o.classList.contains('wd-dim')).length).toBeGreaterThan(0);
    expect(ranked.findIndex((o) => o.classList.contains('wd-dim'))).toBeGreaterThan(0);
    await act(async () => ranked.find((o) => o.textContent?.endsWith(' Felsen'))!.click());
    expect(field(h, 'Art Platz +1').dataset.value).toBe('rocks');
    expect(field(h, 'Art Platz +1').value).toBe('Felsen'); // das Feld zeigt den deutschen Namen
    expect(options(h, 'Art Platz +1')).toHaveLength(0);
  });

  it('deutscher Name in Feld und Liste: „lich“ und „clearing“ stellen die Lichtung nach oben, der Name wird zur Art', async () => {
    const h = await mount();
    await addChunk(h);
    const art = field(h, 'Art Platz +1');
    expect(art.value).toBe(nameOf(art.dataset.value!));
    await type(art, 'lich');
    expect(options(h, 'Art Platz +1')[0].textContent).toBe(`${iconOf('clearing')} Lichtung`);
    await type(art, 'clearing');
    expect(options(h, 'Art Platz +1')[0].textContent).toBe(`${iconOf('clearing')} Lichtung`);
    await type(art, 'Lichtung');
    expect(field(h, 'Art Platz +1').dataset.value).toBe('clearing');
    await act(async () => art.dispatchEvent(new FocusEvent('focusout', { bubbles: true })));
    expect(field(h, 'Art Platz +1').value).toBe('Lichtung');
    expect(h.querySelector('.wd-strip')!.textContent).toContain('Truhe'); // auch der Streifen zeigt deutsche Namen
  });

  it('das Feld zeigt das Emoji der Art; das Objekt wählt man in derselben Liste', async () => {
    const h = await mount();
    await addChunk(h);
    await type(field(h, 'Art Platz +1'), 'stream');
    expect(h.querySelector('.wd-planned .wd-pick-icon')!.textContent).toBe(iconOf('stream'));
    await act(async () => button(h, 'Objekt').click());
    const objekt = field(h, 'Objekt 1 Platz +1');
    expect(objekt.dataset.value).toBe(OBJECT_KINDS[0]);
    expect(objekt.value).toBe(nameOf(OBJECT_KINDS[0]));
    await act(async () => objekt.click());
    expect(options(h, 'Objekt 1 Platz +1')).toHaveLength(OBJECT_KINDS.length);
    await act(async () => options(h, 'Objekt 1 Platz +1').find((o) => o.textContent?.endsWith(' Portal'))!.click());
    expect(field(h, 'Objekt 1 Platz +1').dataset.value).toBe('portal');
    expect(h.querySelectorAll('.wd-object .wd-pick-icon')[0].textContent).toBe(iconOf('portal'));
  });
});

describe('Hinweise am Chunk der Chunk-Vorschau (B-485)', () => {
  const marked = (h: HTMLElement) => [...h.querySelectorAll<HTMLElement>('.wd-chunk[data-mark]')];
  // Mock-Level: ein Hub-Abschnitt (5), Platz +1 ist der Abschnitt rechts daneben.
  const PLUS_ONE = '6 ';

  it('ohne Planung keine Markierung; ein Entwurf am Platz +1 markiert genau diesen Chunk als „planned“', async () => {
    const h = await mount();
    expect(marked(h)).toHaveLength(0);
    await addChunk(h);
    await settle();
    const [chunk] = marked(h);
    expect(marked(h)).toHaveLength(1);
    expect(chunk.textContent!.startsWith(PLUS_ONE)).toBe(true);
    expect(chunk.dataset.mark).toBe('planned');
    expect(chunk.querySelector('.wd-mark.wd-mark-planned')).not.toBeNull();
    expect(chunk.title).toContain(`Geplant: ${nameOf(field(h, 'Art Platz +1').dataset.value!)}`);
    expect(chunk.title).toContain('ohne Chunk-Seed');
  });

  it('ein Prüffehler am Platz zeigt „error“; nach Verwerfen verschwindet die Markierung', async () => {
    const h = await mount();
    await addChunk(h);
    await type(field(h, 'Art Platz +1'), 'volcano');
    await settle();
    expect(marked(h).map((c) => c.dataset.mark)).toEqual(['error']);
    expect(marked(h)[0].title).toContain('volcano');
    await act(async () => button(h, 'Verwerfen').click());
    await flush();
    expect(marked(h)).toHaveLength(0);
  });
});
