// @vitest-environment happy-dom
import { Theme } from '@radix-ui/themes';
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, describe, expect, it } from 'vitest';
import { NAVIGATE_EVENT } from '@webkit/lib/navigate';
import { CatalogLinks } from './CatalogLinks';

// WZ62.4/AC-04 (B-1171): Die Links öffnen den Katalog mit dem Welt-Filter (exklusiv oder gesamt).

(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

let root: Root | null = null;
afterEach(() => {
  act(() => root?.unmount());
  root = null;
  document.body.innerHTML = '';
  window.history.replaceState(null, '', '#');
});

async function klick(modus: 'exklusiv' | 'gesamt'): Promise<string[]> {
  const host = document.createElement('div');
  document.body.appendChild(host);
  root = createRoot(host);
  act(() => root!.render(<Theme><CatalogLinks welt="sengoku" /></Theme>));
  const seen: string[] = [];
  const on = (e: Event) => seen.push((e as CustomEvent<string>).detail);
  window.addEventListener(NAVIGATE_EVENT, on);
  await act(async () => host.querySelector<HTMLElement>(`[data-katalog="${modus}"]`)!.click());
  window.removeEventListener(NAVIGATE_EVENT, on);
  return seen;
}

describe('Link Welten-Editor → Katalog (B-1171)', () => {
  it('exklusiv: Adresse mit Welt und Modus, Wechsel auf Katalog', async () => {
    expect(await klick('exklusiv')).toEqual(['katalog']);
    expect(window.location.hash).toBe('#welt=sengoku&modus=exklusiv');
  });

  it('gesamt: Adresse nur mit der Welt', async () => {
    expect(await klick('gesamt')).toEqual(['katalog']);
    expect(window.location.hash).toBe('#welt=sengoku');
  });
});
