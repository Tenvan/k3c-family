// Gemeinsame Test-Helfer des Bereichs Assets (WB1): Seite mit Theme einhängen, Klicks und Warten in act().
import { Theme } from '@webkit/templates';
import { act, type ReactNode } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { resetCatalogCache } from './catalog/catalogStore';

(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

let root: Root | null = null;

export const wait = (ms: number) => act(async () => new Promise((r) => setTimeout(r, ms)));

/** Höchste Wartezeit einer Bedingung; unter Last reichen Millisekunden, das Limit fängt nur echte Fehler. */
const LIMIT_MS = 5000;

/** Wartet, bis `cond` einen Wert liefert, und gibt ihn zurück; nach `ms` scheitert der Test mit `what` (B-837, B-858). */
export async function until<T>(what: string, cond: () => T | null | undefined | false, ms = LIMIT_MS): Promise<T> {
  const end = Date.now() + ms;
  for (;;) {
    const v = cond();
    if (v) return v;
    if (Date.now() > end) throw new Error(`nicht erreicht nach ${ms} ms: ${what}`);
    await wait(5);
  }
}

/** Ein Takt (setTimeout 0) in act(): nur für Gegenproben und Zeitschritte, nie als Warten auf Inhalt (B-859). */
export const settle = () => wait(0);

export async function mount(node: ReactNode): Promise<HTMLElement> {
  const host = document.createElement('div');
  document.body.appendChild(host);
  root = createRoot(host);
  act(() => root!.render(<Theme>{node}</Theme>));
  await settle();
  return host;
}

export function unmount() {
  act(() => root?.unmount());
  root = null;
  document.body.innerHTML = '';
  // der Katalog-Cache lebt auf Modul-Ebene (B-732): jeder Test beginnt wie nach einem Neustart
  resetCatalogCache();
}

export async function click(el: Element) {
  await act(async () => (el as HTMLElement).click());
  await settle();
}

export const buttons = (host: ParentNode, text: string) =>
  [...host.querySelectorAll('button')].filter((b) => b.textContent?.trim() === text);

/** Tippt in die Suche eines ListFilter (B-728, B-729). */
export async function tippe(host: HTMLElement, text: string) {
  const input = host.querySelector<HTMLInputElement>('input[aria-label="Suchen"]')!;
  await act(async () => {
    Object.getOwnPropertyDescriptor(HTMLInputElement.prototype, 'value')!.set!.call(input, text);
    input.dispatchEvent(new Event('input', { bubbles: true }));
  });
}

/** Chip `wert` in der Chip-Zeile `dimension` eines ListFilter. */
export const chip = (host: HTMLElement, dimension: string, wert: string) =>
  [...host.querySelectorAll(`[role="group"][aria-label="${dimension}"] button`)].find((b) => b.textContent?.startsWith(wert))!;
