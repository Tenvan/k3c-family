// Gemeinsame Test-Helfer des Bereichs Assets (WB1): Seite mit Theme einhängen, Klicks und Warten in act().
import { Theme } from '@webkit/templates';
import { act, type ReactNode } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { resetCatalogCache } from './catalog/catalogStore';

(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

let root: Root | null = null;

export const wait = (ms: number) => act(async () => new Promise((r) => setTimeout(r, ms)));

export async function flush() {
  for (let i = 0; i < 8; i++) await wait(0);
}

export async function mount(node: ReactNode): Promise<HTMLElement> {
  const host = document.createElement('div');
  document.body.appendChild(host);
  root = createRoot(host);
  act(() => root!.render(<Theme>{node}</Theme>));
  await flush();
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
  await flush();
}

export const buttons = (host: ParentNode, text: string) =>
  [...host.querySelectorAll('button')].filter((b) => b.textContent?.trim() === text);
