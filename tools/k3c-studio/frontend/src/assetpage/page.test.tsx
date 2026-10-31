// @vitest-environment happy-dom
import { Theme } from '@radix-ui/themes';
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { backend, STALE, type AssetChoice } from '../api';
import { resetMockSources } from '../api/mockSources';
import { AssetsPage } from './AssetsPage';

// B-677 (AC-02, AC-03, AC-04, AC-06) und B-459/AC-07 im DOM: die Seite Assets mit dem Mock. credits.ts liest public/**/CREDITS.md?raw,
// das Vitest des Studios liefert nichts außerhalb seiner Wurzel aus (die Szenen-Sicht importiert es).
vi.mock('@game/tools/credits', () => ({ CREDITS: [] }));
(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

const mock = backend as typeof backend & { external: (id: string, wahl: AssetChoice[], quiet?: boolean) => void };
const TOWN = 'grafik/gothicvania-town/props-einzeln';
let root: Root | null = null;
let host: HTMLElement | null = null;

const flush = () => act(async () => { for (let i = 0; i < 6; i++) await new Promise((r) => setTimeout(r, 0)); });
const click = async (el: Element | null | undefined) => { await act(async () => { (el as HTMLElement).click(); }); await flush(); };
const button = (scope: ParentNode, text: string) => [...scope.querySelectorAll('button')].find((b) => b.textContent?.trim().startsWith(text));

/** Wie eine Eingabe von Hand: Wert über den nativen Setter, dann input-Ereignis (React hört darauf). */
function type(el: HTMLInputElement, value: string) {
  Object.getOwnPropertyDescriptor(HTMLInputElement.prototype, 'value')!.set!.call(el, value);
  el.dispatchEvent(new Event('input', { bubbles: true }));
}

async function mount(prefs: Record<string, string> = {}): Promise<HTMLElement> {
  for (const [k, v] of Object.entries(prefs)) localStorage.setItem(`k3c-studio.${k}`, v);
  host = document.createElement('div');
  document.body.append(host);
  root = createRoot(host);
  await act(async () => root!.render(<Theme><AssetsPage /></Theme>));
  await flush();
  return host;
}

const search = (h: HTMLElement) => h.querySelector<HTMLInputElement>('input[aria-label="Suche"]')!;
const cutW = (h: HTMLElement) => h.querySelectorAll<HTMLInputElement>('.as-cut input')[2]!;
const chip = (h: HTMLElement, name: string) => button(h.querySelector('[aria-label="Fokus"]')!, name) as HTMLButtonElement;
const slotIds = (h: HTMLElement) => [...h.querySelectorAll('[role="listbox"] .ap-card-id, [role="listbox"] .as-row-id')].map((e) => e.textContent);

beforeEach(() => { localStorage.clear(); resetMockSources(); });
afterEach(() => {
  act(() => root?.unmount());
  host?.remove();
  document.body.innerHTML = '';
  root = null;
  host = null;
});

describe('Fokus-Chips, Suche und Sichten (B-677)', () => {
  it('zeigt die festen Chips mit Zähler, Inspektor ohne Auswahl und die Abspielleiste', async () => {
    const h = await mount();
    for (const name of ['Alles', 'Grafik', 'Sound', 'Lücken', 'Aus dem Netz', 'Lizenz prüfen', 'Vorrat']) expect(chip(h, name), name).toBeTruthy();
    expect(chip(h, 'Alles').getAttribute('aria-pressed')).toBe('true');
    expect(chip(h, 'Sound').textContent).toMatch(/\d/);
    expect(h.querySelector('[role="toolbar"][aria-label="Abspielleiste"]')?.textContent).toContain('Stille');
    expect(h.textContent).toContain('Kein Asset-Slot gewählt');
  });

  it('Chip Sound blendet Grafik-Slots aus; Suche und Sicht bleiben beim Chip-Wechsel (AC-04)', async () => {
    const h = await mount();
    expect(slotIds(h)).toEqual(expect.arrayContaining(['farm', 'music:tag']));
    await act(async () => type(search(h), 'music'));
    await click(chip(h, 'Sound'));
    expect(chip(h, 'Sound').getAttribute('aria-pressed')).toBe('true');
    expect(slotIds(h).every((id) => id?.startsWith('music:'))).toBe(true);
    await click(chip(h, 'Grafik'));
    expect(search(h).value).toBe('music');
    expect(slotIds(h)).toEqual([]);
  });

  it('Chip Lücken zeigt nur Slots ohne aktive Zuordnung oder mit fehlender Datei (AC-02)', async () => {
    const h = await mount();
    await click(chip(h, 'Lücken'));
    expect(slotIds(h)).toEqual(expect.arrayContaining(['tavern', 'music:abend', 'portal']));
    expect(slotIds(h)).not.toContain('farm');
  });

  it('Sicht Dateien: Stufe je Datei, Auswahlleiste nur bei Auswahl, Laden öffnet die Bestätigung (AC-03)', async () => {
    const h = await mount({ 'assets.view': JSON.stringify({ query: '', facets: { art: [], kategorie: [], status: [], stufe: ['netz'], quelle: [], lizenz: [] }, sicht: 'dateien', gruppe: 'slot' }) });
    expect(h.querySelectorAll('[data-testid="file-row"]').length).toBeGreaterThan(0);
    expect(h.textContent).toContain('Netz');
    expect(h.querySelector('[data-testid="selection-bar"]')).toBeNull();
    await click(h.querySelector('[aria-label="Taverne (Pixel) wählen"]'));
    expect(h.querySelector('[data-testid="selection-sum"]')?.textContent).toMatch(/^1 gewählt · /);
    await click(button(h, 'Nur laden'));
    expect(document.body.textContent).toContain('Herunterladen bestätigen');
    await click(button(h, 'Auswahl leeren'));
    expect(h.querySelector('[data-testid="selection-bar"]')).toBeNull();
  });

  it('Ansicht merken legt einen eigenen Chip an, der ein Neuladen übersteht und sich entfernen lässt (AC-02)', async () => {
    let h = await mount();
    await click(chip(h, 'Sound'));
    await click(button(h, 'Ansicht merken'));
    await act(async () => type(h.querySelector<HTMLInputElement>('input[aria-label="Name der Ansicht"]')!, 'Nur Töne'));
    await click(button(h, 'Speichern'));
    expect(chip(h, 'Nur Töne')).toBeTruthy();
    expect(JSON.parse(localStorage.getItem('k3c-studio.assets.chips')!)[0]).toMatchObject({ label: 'Nur Töne', facets: { art: ['sound'] } });
    act(() => root!.unmount());
    host!.remove();
    h = await mount();
    expect(chip(h, 'Nur Töne')).toBeTruthy();
    await click(h.querySelector('[aria-label="Nur Töne entfernen"]'));
    expect(button(h.querySelector('[aria-label="Fokus"]')!, 'Nur Töne')).toBeUndefined();
  });
});

describe('Neuladen und Schreibregel (B-459/AC-07)', () => {
  const FARM = { 'assets.selected': 'farm' };

  it('assets:changed lädt neu und hält Auswahl, Suche und Ausschnitt-Entwurf', async () => {
    const h = await mount(FARM);
    await act(async () => type(search(h), 'farm'));
    await act(async () => type(cutW(h), '40'));
    await act(async () => mock.external('farm', [{ datei: `${TOWN}/well.png`, ausschnitt: { x: 0, y: 0, w: 65, h: 65 } }, { datei: `${TOWN}/wagon.png` }, { datei: `${TOWN}/crate.png` }]));
    await flush();
    expect(h.textContent).toContain(`3. ${TOWN}/crate.png`);
    expect(h.querySelector('.ap-card[aria-pressed="true"]')?.textContent).toContain('farm');
    expect(search(h).value).toBe('farm');
    expect(cutW(h).value).toBe('40');
  });

  it('Speichern auf veraltetem Stand: neu laden, Meldung, Eingaben bleiben', async () => {
    const h = await mount(FARM);
    await act(async () => type(cutW(h), '40'));
    await act(async () => mock.external('storage', [], true));
    await click(button(h, 'Ausschnitt speichern'));
    expect(h.textContent).toContain(STALE);
    expect(cutW(h).value).toBe('40');
    await click(button(h, 'Ausschnitt speichern'));
    expect(h.textContent).not.toContain(STALE);
    expect(h.textContent).toContain('Ausschnitt 0,0,40,65');
  });

  it('„Neu laden“ holt fremde Änderungen, auch ohne Ereignis, und hält Auswahl und Suche', async () => {
    const h = await mount(FARM);
    await act(async () => type(search(h), 'farm'));
    await act(async () => mock.external('farm', [{ datei: `${TOWN}/well.png` }, { datei: `${TOWN}/crate.png` }], true));
    await flush();
    expect(h.textContent).not.toContain(`2. ${TOWN}/crate.png`);
    await click(button(h, 'Neu laden'));
    expect(h.textContent).toContain(`2. ${TOWN}/crate.png`);
    expect(h.querySelector('.ap-card[aria-pressed="true"]')?.textContent).toContain('farm');
    expect(search(h).value).toBe('farm');
  });
});

describe('Bühne (Variante „Bühne“)', () => {
  const stage = (h: HTMLElement) => h.querySelector<HTMLElement>('[data-testid="stage"]')!;
  const zoom = (h: HTMLElement) => h.querySelector('[data-testid="stage-zoom"]')?.textContent;

  it('steht unter dem Raster und zeigt die Szene des gewählten Slots', async () => {
    let h = await mount();
    expect(stage(h).dataset.slot).toBe('');
    expect(stage(h).querySelector('.as-scene')).toBeNull();
    await click(h.querySelector('.ap-card'));
    const first = h.querySelector('.ap-card[aria-pressed="true"] .ap-card-id')!.textContent;
    expect(stage(h).dataset.slot).toBe(first);
    expect(stage(h).querySelector('.as-scene-view canvas')).not.toBeNull();
    act(() => root!.unmount());
    host!.remove();
    h = await mount({ 'assets.selected': 'farm' });
    expect(stage(h).dataset.slot).toBe('farm');
    expect(h.querySelector('.ap-side .as-scene')).toBeNull(); // die Leiste rechts hat keine zweite Szene mehr
  });

  it('Zoom ändert den Maßstab der Szene und wird gemerkt', async () => {
    const h = await mount({ 'assets.selected': 'farm' });
    expect(zoom(h)).toBe('100 %');
    await click(h.querySelector('[aria-label="Vergrößern"]'));
    expect(zoom(h)).toBe('150 %');
    expect(stage(h).querySelector('.as-scene-view')?.getAttribute('data-zoom')).toBe('1.5');
    await click(h.querySelector('[aria-label="Verkleinern"]'));
    await click(h.querySelector('[aria-label="Verkleinern"]'));
    expect(zoom(h)).toBe('75 %');
    expect(localStorage.getItem('k3c-studio.assets.stagezoom')).toContain('0.75');
  });

  it('Vollbild schaltet nur die Layout-Klasse, Esc verlässt es', async () => {
    const h = await mount({ 'assets.selected': 'farm' });
    const full = button(stage(h), 'Vollbild')!;
    await click(full);
    expect(stage(h).classList.contains('ap-stage-full')).toBe(true);
    expect(full.getAttribute('aria-pressed')).toBe('true');
    await act(async () => { window.dispatchEvent(new KeyboardEvent('keydown', { key: 'Escape' })); });
    expect(stage(h).classList.contains('ap-stage-full')).toBe(false);
  });

  it('Splitter zwischen Raster und Bühne: Pfeiltasten ändern die Höhe, gemerkt pro Viewer', async () => {
    let h = await mount();
    const split = () => h.querySelector<HTMLElement>('.ap-stagesplit')!;
    expect(split().getAttribute('role')).toBe('separator');
    expect(split().getAttribute('aria-valuenow')).toBe('38');
    await act(async () => { split().dispatchEvent(new KeyboardEvent('keydown', { key: 'ArrowUp', bubbles: true })); });
    expect(split().getAttribute('aria-valuenow')).toBe('40');
    expect((h.querySelector('.ap-body .ap-main')!.parentElement as HTMLElement).style.getPropertyValue('--ap-stg')).toBe('40%');
    expect(localStorage.getItem('k3c-studio.assets.stage')).toBe('40');
    act(() => root!.unmount());
    host!.remove();
    h = await mount();
    expect(split().getAttribute('aria-valuenow')).toBe('40');
    expect([...h.querySelector('.ap-main')!.children].map((c) => c.className.split(' ').find((n) => n.startsWith('ap-')))).toEqual(
      ['ap-list', 'ap-stagesplit', 'ap-stage', 'ap-split', 'ap-stack', 'ap-side']); // Tab-Reihenfolge: Raster, Bühne, Leiste
  });

  it('Präferenz in der Belegung wählen hebt sie auf der Bühne hervor (geteilter Zustand)', async () => {
    const h = await mount({ 'assets.selected': 'farm' });
    await click(h.querySelector('[aria-label="Präferenz 2 wählen"]'));
    expect(h.querySelectorAll('.ap-choice')[1]?.classList.contains('ap-choice-on')).toBe(true);
    expect(stage(h).dataset.slot).toBe('farm');
  });
});