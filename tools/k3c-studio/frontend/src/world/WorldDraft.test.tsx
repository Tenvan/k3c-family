// @vitest-environment happy-dom
import { Theme } from '@radix-ui/themes';
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, describe, expect, it, vi } from 'vitest';
import { backend } from '../api';
import { WorldPage } from './WorldPage';
import { optionTexts } from '../../../../webkit/testing/options';
import { pickOption, selectedText } from '../../../../webkit/testing/select';

// WZ18.6 (B-488/AC-05) im DOM mit dem Mock-Backend: Welt-Entwurf mit Welt-Kopf und Inseln, sofortige Prüfung mit Fehler
// am Feld, Speichern nur Gültiges mit REG-Hinweis, Stale lässt den Entwurf stehen. Jeder Test arbeitet in einer eigenen,
// neu angelegten Welt (die Standard-Welt des Mocks bleibt).
vi.mock('../stage/PreviewStage', () => ({ PreviewStage: class { show() {} refresh() {} setLook() {} setPosition() {} destroy() {} } }));
(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

let root: Root | null = null;
let host: HTMLElement | null = null;

const wait = (ms = 0) => act(async () => { await new Promise((r) => setTimeout(r, ms)); });
const flush = async () => { for (let i = 0; i < 6; i++) await wait(); };
const settle = async () => { await wait(260); await flush(); }; // Prüfung ist um 200 ms verzögert

const button = (h: HTMLElement, label: string) =>
  [...h.querySelectorAll('button')].find((b) => b.textContent === label || b.getAttribute('aria-label') === label)! as HTMLButtonElement;
const field = (h: HTMLElement, label: string) => h.querySelector<HTMLInputElement>(`[aria-label="${label}"]`)!;
const fieldError = (h: HTMLElement, island: number, name: string) => h.querySelector(`.wd-error[data-island="${island}"][data-field="${name}"]`);
const islandOptions = (h: HTMLElement) => optionTexts(h, 'Insel');

async function type(el: HTMLInputElement, value: string) {
  Object.getOwnPropertyDescriptor(HTMLInputElement.prototype, 'value')!.set!.call(el, value);
  await act(async () => el.dispatchEvent(new Event('input', { bubbles: true })));
}
const click = async (el: HTMLElement) => { await act(async () => el.click()); await flush(); };

/** Seite mit einer neuen, gewählten Welt „Probe“ (eine Insel). */
async function mount(): Promise<HTMLElement> {
  host = document.createElement('div');
  document.body.append(host);
  root = createRoot(host);
  await act(async () => root!.render(<Theme><WorldPage /></Theme>));
  await flush();
  await type(field(host, 'Name der neuen Welt'), 'Probe');
  await click(button(host, 'Welt +'));
  return host;
}

afterEach(async () => {
  act(() => root?.unmount());
  host?.remove();
  for (const w of (await backend.planWorlds()).worlds) if (w.id !== 'standard') await backend.planWorldRemove(w.id);
  await backend.planWorldSelect('standard');
});

describe('Welt-Entwurf (B-488/AC-05)', () => {
  it('benennt die Welt um und setzt Notizen; Vorschau, Speichern mit REG-Hinweis, die id bleibt', async () => {
    const h = await mount();
    expect(button(h, 'Welt speichern').disabled).toBe(true); // ohne Änderung
    await type(field(h, 'Name der Welt'), 'Nordmeer');
    await type(field(h, 'Notizen der Welt'), 'kalt');
    await settle();
    expect(h.querySelector('pre[aria-label="Vorschau der Welt-Datei"]')!.textContent).toContain('"name": "Nordmeer"');
    expect(button(h, 'Welt +').disabled).toBe(true); // Welt wechseln und anlegen nur ohne offenen Entwurf
    await click(button(h, 'Welt speichern'));
    expect(h.querySelector('.wd-notice')!.textContent).toContain('REG-Änderung');
    expect((await backend.planWorlds()).active).toBe('probe');
    expect(selectedText(h, 'Welt')).toBe('Nordmeer');
    expect(button(h, 'Welt speichern').disabled).toBe(true);
  });

  it('legt Inseln an, ändert Name und id, ordnet sie um und entfernt eine; gespeichert in dieser Reihenfolge', async () => {
    const h = await mount();
    await click(button(h, 'Insel +'));
    await click(button(h, 'Insel +'));
    await type(field(h, 'Name Insel 2'), 'Eisinsel');
    await type(field(h, 'ID Insel 2'), 'eisinsel');
    await click(button(h, 'Insel 2 hoch'));
    expect(field(h, 'Name Insel 1').value).toBe('Eisinsel');
    expect(button(h, 'Insel 1 hoch').disabled).toBe(true);
    await click(button(h, 'Insel 3 entfernen'));
    await settle();
    await click(button(h, 'Welt speichern'));
    expect(await islandOptions(h)).toEqual(['Eisinsel', 'Insel 1']);
  });

  it('nach dem Speichern einer neuen id lässt sich weiter ändern und speichern; offene Planung sperrt das Speichern', async () => {
    const h = await mount();
    await type(field(h, 'ID Insel 1'), 'eis');
    await settle();
    await click(button(h, 'Welt speichern'));
    await type(field(h, 'Name Insel 1'), 'Eis');
    await settle();
    await click(button(h, 'Chunk hinzufügen'));
    expect(button(h, 'Welt speichern').disabled).toBe(true); // die Planung ginge beim Neuladen verloren
    await click(button(h, 'Verwerfen'));
    await settle();
    await click(button(h, 'Welt speichern'));
    expect(await islandOptions(h)).toEqual(['Eis']);
  });

  it('setzt Stufen und Skalierung einer Insel', async () => {
    const h = await mount();
    await click(field(h, 'Insel 1 Tiefe 2'));
    await type(field(h, 'Insel 1 HP'), '2.5');
    await settle();
    await click(button(h, 'Welt speichern'));
    const view = await backend.planLoad();
    expect(view.islands[0]!.depths).toEqual([0, 1, 3, 4]);
    expect(view.islands[0]!.depthScaling.hp).toBe(2.5);
    expect(await optionTexts(h, 'Stufe')).toHaveLength(4);
  });
});

describe('Planung und Chunk-Anzahl in der gewählten Welt (Editor aus WZ8/WZ17)', () => {
  it('speichert Chunk und Inselgröße der Stufe in die Welt „Probe“, nicht in die Standard-Welt', async () => {
    const h = await mount();
    await pickOption(h, 'Inselgröße', 'klein · 24');
    await click(button(h, 'Chunk hinzufügen'));
    await settle();
    await click(button(h, 'Speichern'));
    const island = (await backend.planLoad()).islands[0]!;
    expect([island.planned['0']?.length, island.chunkCounts['0']]).toEqual([1, 24]);
    await backend.planWorldSelect('standard');
    expect((await backend.planLoad()).islands[0]!.chunkCounts['0']).toBeUndefined();
  });
});

describe('Prüfung mit Fehler am Feld (B-488 › Ausnahme- und Fehlerfälle)', () => {
  it('doppelte id: Fehler an der zweiten Insel, Speichern gesperrt', async () => {
    const h = await mount();
    await click(button(h, 'Insel +'));
    await type(field(h, 'ID Insel 2'), 'island1');
    await settle();
    expect(fieldError(h, 1, 'id')!.textContent).toContain('Insel island1: doppelt');
    expect(button(h, 'Welt speichern').disabled).toBe(true);
  });

  it('ohne Endboss und Faktor 0: Fehler an Stufen bzw. Skalierung; leerer Name an der Welt', async () => {
    const h = await mount();
    await click(field(h, 'Insel 1 Tiefe 4'));
    await settle();
    expect(fieldError(h, 0, 'depths')!.textContent).toContain('kein Endboss');
    await click(field(h, 'Insel 1 Tiefe 4'));
    await type(field(h, 'Insel 1 Tempo'), '0');
    await settle();
    expect(fieldError(h, 0, 'depthScaling')!.textContent).toContain('Faktoren > 0');
    await type(field(h, 'Insel 1 Tempo'), '1');
    await type(field(h, 'Name der Welt'), '');
    await settle();
    expect(fieldError(h, -1, 'name')!.textContent).toContain('name ist leer');
    expect(button(h, 'Welt speichern').disabled).toBe(true);
  });
});

describe('Stand-Prüfung und Spiel', () => {
  it('von außen geänderte Datei: Speichern abgelehnt, der Entwurf bleibt', async () => {
    const h = await mount();
    await type(field(h, 'Name der Welt'), 'Nordmeer');
    await settle();
    (backend as unknown as { touch: () => void }).touch();
    await click(button(h, 'Welt speichern'));
    expect(h.querySelector('.wd-notice')!.textContent).toContain('data/worlds/probe.json geändert, bitte neu laden');
    expect(h.querySelector('.wd-notice')!.textContent).toContain('Entwurf bleibt');
    expect(field(h, 'Name der Welt').value).toBe('Nordmeer');
    expect(button(h, 'Welt speichern').disabled).toBe(false);
  });

  it('„Im Spiel öffnen“ startet die gewählte Welt frisch mit ihrer id', async () => {
    await mount();
    const res = await backend.worldOpenGame('http://127.0.0.1:5183', 'k3c', 'forest', 'new');
    expect(decodeURIComponent(res.url)).toContain('game.html?autostart=1&save=probe-k3c&seed=k3c&world=probe');
    expect((await backend.worldOpenGame('http://127.0.0.1:5183', 'k3c', 'forest', 'full')).error).toContain('Standard-Welt');
  });
});
