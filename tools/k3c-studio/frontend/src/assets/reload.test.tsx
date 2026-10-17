// @vitest-environment happy-dom
import { Theme } from '@radix-ui/themes';
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, describe, expect, it } from 'vitest';
import { backend, STALE, type AssetChoice } from '../api';
import { AssetsPage } from './AssetsPage';

// B-459/AC-07 im DOM: der Mock (ohne Wails) schreibt als fremder Prozess, die Seite lädt neu bzw. lehnt Speichern ab.
(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

const mock = backend as typeof backend & { external: (id: string, wahl: AssetChoice[], quiet?: boolean) => void };
const TOWN = 'grafik/gothicvania-town/props-einzeln';
let root: Root | null = null;
let host: HTMLElement | null = null;

const flush = () => act(async () => { for (let i = 0; i < 5; i++) await new Promise((r) => setTimeout(r, 0)); });

/** Wie eine Eingabe von Hand: Wert über den nativen Setter, dann input-Ereignis (React hört darauf). */
function type(el: HTMLInputElement, value: string) {
  Object.getOwnPropertyDescriptor(HTMLInputElement.prototype, 'value')!.set!.call(el, value);
  el.dispatchEvent(new Event('input', { bubbles: true }));
}

async function mount(): Promise<HTMLElement> {
  localStorage.clear();
  localStorage.setItem('k3c-studio.grafik.selected', 'farm');
  host = document.createElement('div');
  document.body.append(host);
  root = createRoot(host);
  await act(async () => root!.render(<Theme><AssetsPage art="grafik" /></Theme>));
  await flush();
  return host;
}

const filter = (h: HTMLElement) => h.querySelector<HTMLInputElement>('input[placeholder^="Slot, Beschreibung"]')!;
const cutW = (h: HTMLElement) => h.querySelectorAll<HTMLInputElement>('.as-cut input')[2]!;

afterEach(() => {
  act(() => root?.unmount());
  host?.remove();
});

describe('Neuladen und Schreibregel (B-459/AC-07)', () => {
  it('assets:changed lädt neu und hält Auswahl, Filter und Ausschnitt-Entwurf', async () => {
    const h = await mount();
    await act(async () => type(filter(h), 'farm'));
    await act(async () => type(cutW(h), '40'));
    await act(async () => mock.external('farm', [{ datei: `${TOWN}/well.png`, ausschnitt: { x: 0, y: 0, w: 65, h: 65 } }, { datei: `${TOWN}/wagon.png` }, { datei: `${TOWN}/crate.png` }]));
    await flush();
    expect(h.textContent).toContain(`3. ${TOWN}/crate.png`); // neue Daten sind da
    expect(h.querySelector('[aria-selected="true"]')?.textContent).toContain('farm');
    expect(filter(h).value).toBe('farm');
    expect(cutW(h).value).toBe('40');
  });

  it('Speichern auf veraltetem Stand: neu laden, Meldung, Eingaben bleiben', async () => {
    const h = await mount();
    await act(async () => type(cutW(h), '40'));
    // Fremder Schreiber zwischen Laden und Speichern, Ereignis kommt erst danach an: Stand der Seite ist veraltet.
    await act(async () => mock.external('storage', [], true));
    const save = [...h.querySelectorAll('button')].find((b) => b.textContent === 'Ausschnitt speichern')!;
    await act(async () => save.click());
    await flush();
    expect(h.textContent).toContain(STALE);
    expect(cutW(h).value).toBe('40');
    expect(h.querySelector('[aria-selected="true"]')?.textContent).toContain('farm');
    // Rückweg: nach dem Neuladen gelingt das zweite Speichern mit dem frischen Stand.
    await act(async () => save.click());
    await flush();
    expect(h.textContent).not.toContain(STALE);
    expect(h.textContent).toContain('Ausschnitt 0,0,40,65');
  });
});
