// @vitest-environment happy-dom
import { Theme } from '@radix-ui/themes';
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, describe, expect, it, vi } from 'vitest';
import { backend } from '../api';
import { WorldPage } from './WorldPage';
import { forgetMode, toMode } from './worldTesting';

// WZ37.1 (B-682/AC-01, AC-02) im DOM mit dem Mock-Backend: Ansicht „Werte“ zeigt Basiswert, Wert und Status, setzt und
// setzt zurück; „Welt speichern“ schickt nur Abweichungen; Kind-Welten stehen vor dem Speichern da.
vi.mock('../stage/PreviewStage', () => ({ PreviewStage: class { show() {} refresh() {} setLook() {} setPosition() {} destroy() {} } }));
(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

let root: Root | null = null;
let host: HTMLElement | null = null;

const wait = (ms = 0) => act(async () => { await new Promise((r) => setTimeout(r, ms)); });
const flush = async () => { for (let i = 0; i < 6; i++) await wait(); };
const settle = async () => { await wait(260); await flush(); }; // Prüfung ist um 200 ms verzögert
const labeled = (h: HTMLElement, label: string) => h.querySelector<HTMLElement>(`[aria-label="${label}"]`)!;
const button = (h: HTMLElement, label: string) =>
  [...h.querySelectorAll('button')].find((b) => b.textContent === label || b.getAttribute('aria-label') === label)! as HTMLButtonElement;
const statusOf = (h: HTMLElement, path: string) => labeled(h, `Status ${path}`).textContent;

/** Wert eingeben und übernehmen (Verlassen des Felds). */
async function enter(h: HTMLElement, path: string, value: string) {
  const el = labeled(h, `Wert ${path}`) as HTMLInputElement;
  Object.getOwnPropertyDescriptor(HTMLInputElement.prototype, 'value')!.set!.call(el, value);
  await act(async () => el.dispatchEvent(new Event('input', { bubbles: true })));
  await act(async () => el.dispatchEvent(new FocusEvent('focusout', { bubbles: true })));
  await flush();
}

async function mount(): Promise<HTMLElement> {
  host = document.createElement('div');
  document.body.append(host);
  root = createRoot(host);
  await act(async () => root!.render(<Theme><WorldPage /></Theme>));
  await flush();
  await toMode(host, 'Werte');
  return host;
}

afterEach(async () => {
  act(() => root?.unmount());
  host?.remove();
  forgetMode();
  vi.restoreAllMocks();
  const d = (await backend.planLoad());
  await backend.planWorldSave({ name: d.name, notes: d.notes, islands: d.islands.map((i) => ({ from: i.id, id: i.id, name: i.name, depths: i.depths, depthScaling: i.depthScaling })), overrides: {} }, d.stand);
});

describe('Ansicht „Werte“ (B-682)', () => {
  it('AC-02: zeigt Basiswert und Status geerbt/überschrieben je Feld', async () => {
    const h = await mount();
    expect(labeled(h, 'Basiswert wall.cost.wood').textContent).toBe('10');
    expect((labeled(h, 'Wert wall.cost.wood') as HTMLInputElement).value).toBe('10');
    expect(statusOf(h, 'wall.cost.wood')).toBe('geerbt');
    expect(button(h, 'Zurücksetzen wall.cost.wood').disabled).toBe(true);
    await enter(h, 'wall.cost.wood', '7');
    expect(statusOf(h, 'wall.cost.wood')).toBe('überschrieben');
    expect(labeled(h, 'Basiswert wall.cost.wood').textContent).toBe('10');
    expect(statusOf(h, 'wall.levels.1.hp')).toBe('geerbt');
  });

  it('AC-01: setzen, zurücksetzen, Wert gleich Basis ist geerbt; Speichern schickt nur Abweichungen', async () => {
    const h = await mount();
    await enter(h, 'wall.cost.wood', '7');
    await enter(h, 'wall.levels.1.hp', '650');
    await act(async () => button(h, 'Zurücksetzen wall.levels.1.hp').click());
    await flush();
    expect(statusOf(h, 'wall.levels.1.hp')).toBe('geerbt');
    expect((labeled(h, 'Wert wall.levels.1.hp') as HTMLInputElement).value).toBe('500');
    await enter(h, 'wall.levels.1.hp', '500'); // gleich dem Basiswert: kein Override
    expect(statusOf(h, 'wall.levels.1.hp')).toBe('geerbt');
    expect(h.querySelector('.wd-changes')?.textContent).toBe('1 Änderung');
    await settle();
    const save = vi.spyOn(backend, 'planWorldSave');
    await act(async () => button(h, 'Welt speichern').click());
    await flush();
    expect(save).toHaveBeenCalledTimes(1);
    expect(save.mock.calls[0]![0].overrides).toEqual({ buildings: { wall: { cost: { wood: 7 } } } });
    expect((await backend.planLoad()).overrides).toEqual({ buildings: { wall: { cost: { wood: 7 } } } });
    expect(statusOf(h, 'wall.cost.wood')).toBe('überschrieben');
    expect(h.querySelector('.wd-changes')).toBeNull();
  });

  it('ungültiger Override: Meldung mit Welt, Datei und Pfad, Speichern gesperrt', async () => {
    const h = await mount();
    await enter(h, 'wall.cost.wood', 'viel');
    await settle();
    expect(h.querySelector('.wd-error[data-field="overrides"]')?.textContent).toBe('data/worlds/standard.json: overrides › data/buildings.json › wall.cost.wood: Zahl erwartet, ist Text');
    expect(button(h, 'Welt speichern').disabled).toBe(true);
  });

  it('listet die Kind-Welten vor dem Speichern', async () => {
    const real = backend.planWorldValues.bind(backend);
    vi.spyOn(backend, 'planWorldValues').mockImplementation(async () => ({ ...(await real()), children: ['nordmeer', 'eis'] }));
    const h = await mount();
    expect(labeled(h, 'Kind-Welten').textContent).toContain('nordmeer, eis');
    await enter(h, 'wall.cost.wood', '7');
    expect(labeled(h, 'Wirkt auf Kind-Welten').textContent).toBe('wirkt auch auf: nordmeer, eis');
  });

  it('markiert ungültige Nachfolger und nennt den Grund (B-693)', async () => {
    const real = backend.planWorldValues.bind(backend);
    vi.spyOn(backend, 'planWorldValues').mockImplementation(async () => ({ ...(await real()), children: ['nordmeer', 'eis'], childErrors: { eis: 'lädt nicht' } }));
    const h = await mount();
    await enter(h, 'wall.cost.wood', '7');
    const kids = labeled(h, 'Wirkt auf Kind-Welten');
    expect(kids.textContent).toBe('wirkt auch auf: nordmeer, eis (ungültig)');
    expect(kids.getAttribute('title')).toContain('eis: lädt nicht');
  });
});
