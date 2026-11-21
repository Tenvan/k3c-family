// @vitest-environment happy-dom
import { Theme } from '@radix-ui/themes';
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, describe, expect, it, vi } from 'vitest';
import { backend } from '../api';
import { WorldPage } from './WorldPage';

// WZ62.7/AC-06 (B-1173): Werte-Tabelle mit Gruppen, Suche, Filter, Standardwert und Abweichung, Rücksetzen je Zeile und
// geprüfter Eingabe.

(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

let root: Root | null = null;
afterEach(() => {
  act(() => root?.unmount());
  root = null;
  document.body.innerHTML = '';
  vi.restoreAllMocks();
});

const flush = async () => {
  for (let i = 0; i < 5; i++) await act(async () => new Promise((r) => setTimeout(r, 0)));
};
async function mount(world = 'wueste'): Promise<HTMLElement> {
  localStorage.clear();
  localStorage.setItem('k3c-workbench.world.id', world);
  const host = document.createElement('div');
  document.body.appendChild(host);
  root = createRoot(host);
  act(() => root!.render(<Theme><WorldPage /></Theme>));
  await flush();
  return host;
}
const q = (host: ParentNode, label: string) => host.querySelector<HTMLInputElement>(`[aria-label="${label}"]`)!;
const click = async (el: Element) => {
  await act(async () => (el as HTMLElement).click());
  await flush();
};
const key = async (el: HTMLElement, k: string, init: KeyboardEventInit = {}) => {
  await act(async () => el.dispatchEvent(new KeyboardEvent('keydown', { key: k, bubbles: true, ...init })));
  await flush();
};
const type = async (el: HTMLInputElement, text: string) => {
  await act(async () => {
    Object.getOwnPropertyDescriptor(HTMLInputElement.prototype, 'value')!.set!.call(el, text);
    el.dispatchEvent(new Event('input', { bubbles: true }));
  });
};
const rows = (host: ParentNode) => [...host.querySelectorAll('[role="row"][data-status]')].map((r) => r.getAttribute('data-key')!.split(':')[1]!.replaceAll('\u0000', '.'));
const chip = (host: ParentNode, text: string) => [...host.querySelectorAll('[role="group"][aria-label="Zeilen"] button')].find((b) => b.textContent?.startsWith(text))!;

describe('Werte-Tabelle (B-1173)', () => {
  it('zeigt Gruppen nach Thema mit Zahl der Felder und Überschreibungen, Gruppen lassen sich einklappen', async () => {
    const host = await mount();
    const group = host.querySelector('[data-group="buildings:tower"]')!;
    expect(group.textContent).toContain('4 Felder');
    expect(group.textContent).toContain('1 überschrieben');
    expect(rows(group)).toEqual(['tower.cost', 'tower.hp', 'tower.range', 'tower.needsArcher']);
    await click(group.querySelector('[aria-expanded]')!);
    expect(rows(host.querySelector('[data-group="buildings:tower"]')!)).toEqual([]);
    await click(host.querySelector('[data-group="buildings:tower"] [aria-expanded]')!);
    expect(rows(host.querySelector('[data-group="buildings:tower"]')!)).toHaveLength(4);
  });

  it('Standardwert, Wert und Abweichung je Zeile', async () => {
    const host = await mount();
    expect(host.querySelector('[aria-label="Basiswert purse.startGold"]')?.textContent).toBe('14');
    expect(q(host, 'Wert purse.startGold').value).toBe('20');
    expect(host.querySelector('[aria-label="Abweichung purse.startGold"]')?.textContent).toBe('+6 · +43 %');
    expect(host.querySelector('[aria-label="Abweichung purse.maxGold"]')).toBeNull();
  });

  it('Suche (Pfad und Basiswert) und Filter „Überschrieben“ und „Diese Sitzung“', async () => {
    const host = await mount();
    await type(q(host, 'Feld suchen'), 'tower cost');
    expect(rows(host)).toEqual(['tower.cost']);
    await type(q(host, 'Feld suchen'), '');
    await click(chip(host, 'Überschrieben'));
    expect(rows(host).sort()).toEqual(['purse.dropOnHit', 'purse.startGold', 'tower.range', 'wolf.hp']);
    await click(chip(host, 'Diese Sitzung'));
    expect(rows(host)).toEqual([]);
    expect(host.textContent).toContain('Kein Feld passt zum Filter');
    await click(chip(host, 'Alle'));
    await key(q(host, 'Wert farm.cost'), 'ArrowUp');
    await key(q(host, 'Wert farm.cost'), 'Enter');
    await click(chip(host, 'Diese Sitzung'));
    expect(rows(host)).toEqual(['farm.cost']);
  });

  it('Pfeil hoch ändert Zahlen (Umschalt ×10), Enter schreibt, Esc verwirft', async () => {
    const host = await mount();
    const set = vi.spyOn(backend, 'worldOverrideSet');
    const input = q(host, 'Wert wall.cost');
    await key(input, 'ArrowUp');
    expect(input.value).toBe('4');
    await key(input, 'ArrowDown', { shiftKey: true });
    expect(input.value).toBe('-6');
    await key(input, 'Escape');
    expect(input.value).toBe('3');
    expect(set).not.toHaveBeenCalled();
    await key(input, 'ArrowUp', { shiftKey: true });
    await key(input, 'Enter');
    expect(set).toHaveBeenCalledWith('wueste', 'buildings', ['wall', 'cost'], 13);
  });

  it('ungültige Eingabe wird beim Tippen gemeldet und nicht geschickt', async () => {
    const host = await mount();
    const set = vi.spyOn(backend, 'worldOverrideSet');
    const input = q(host, 'Wert wall.hp');
    await type(input, 'viel');
    expect(input.getAttribute('aria-invalid')).toBe('true');
    expect(host.querySelector('[role="alert"]')?.textContent).toBe('Zahl erwartet');
    await key(input, 'Enter');
    expect(set).not.toHaveBeenCalled();
    await type(input, '45,5');
    expect(input.getAttribute('aria-invalid')).toBe('false');
    await key(input, 'Enter');
    expect(set).toHaveBeenCalledWith('wueste', 'buildings', ['wall', 'hp'], 45.5);
  });

  it('Ja/Nein-Werte sind ein Schalter, Zurücksetzen je Zeile entfernt die Überschreibung', async () => {
    const host = await mount();
    const set = vi.spyOn(backend, 'worldOverrideSet');
    await click(q(host, 'Wert tower.needsArcher'));
    expect(set).toHaveBeenCalledWith('wueste', 'buildings', ['tower', 'needsArcher'], false);
    await click(q(host, 'Zurücksetzen tower.range'));
    expect(set).toHaveBeenLastCalledWith('wueste', 'buildings', ['tower', 'range'], null);
    expect(host.querySelector('[aria-label="Status tower.range"]')?.textContent).toBe('geerbt');
  });

  it('Alt+Pfeil springt in das Feld der nächsten Zeile', async () => {
    const host = await mount();
    const first = q(host, 'Wert wall.cost');
    first.focus();
    await key(first, 'ArrowDown', { altKey: true });
    expect(document.activeElement).toBe(q(host, 'Wert wall.hp'));
  });
});
