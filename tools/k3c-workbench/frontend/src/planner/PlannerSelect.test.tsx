// @vitest-environment happy-dom
import { Theme } from '@radix-ui/themes';
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, describe, expect, it, vi } from 'vitest';
import { backend, type HubRow } from '../api';
import { mockHubCheck } from '../api/mockHub';
import { pickOption } from '../../../../webkit/testing/select';
import { PlannerPage } from './PlannerPage';

// B-670 im DOM: Auswahl einer Zeile in der Matrix (AC-04), „+ Platz“ im Matrix-Kopf (AC-05). Bühne und Platz-Chips des
// Studios gibt es in der Workbench nicht (B-923).
(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

let root: Root | null = null;
let host: HTMLElement | null = null;
const flush = () => act(async () => { for (let i = 0; i < 5; i++) await new Promise((r) => setTimeout(r, 0)); });

async function mount(): Promise<HTMLElement> {
  host = document.createElement('div');
  document.body.append(host);
  root = createRoot(host);
  await act(async () => root!.render(<Theme><PlannerPage /></Theme>));
  await flush();
  return host;
}

const click = async (el: Element) => { await act(async () => (el as HTMLElement).click()); await flush(); };
const button = (h: HTMLElement, text: string) => [...h.querySelectorAll('button')].find((b) => b.textContent === text || b.getAttribute('aria-label') === text)!;
const chosenRow = (h: HTMLElement) => h.querySelector<HTMLElement>('.pl-matrix tr.pl-selected')?.dataset.key ?? '';

afterEach(() => {
  vi.restoreAllMocks();
  act(() => root?.unmount());
  host?.remove();
});

describe('Gebäude-Planer: Auswahl einer Zeile (B-670/AC-04)', () => {
  it('Klick auf die Taverne wählt ihre Zeile, der Inspektor zeigt ihre Felder', async () => {
    const h = await mount();
    expect(h.querySelector('.pl-inspector')!.textContent).toContain('Bauplatz in der Matrix wählen');
    await click(h.querySelector('tr[data-key="tavern@-32"] button.pl-row')!);
    expect(chosenRow(h)).toBe('tavern@-32');
    expect(h.querySelector('.pl-editor')!.textContent).toContain('Taverne');
    expect(h.querySelector<HTMLInputElement>('input[aria-label="Platz (Units)"]')!.value).toBe('-32');
  });

  it('der Baugrund der Zone 1 zeigt Mauer und Turm, die Auswahl wechselt mit dem Klick', async () => {
    const h = await mount();
    await click(h.querySelector('tr[data-key="zone1"] button.pl-row')!);
    expect(chosenRow(h)).toBe('zone1');
    expect(h.querySelector('input[aria-label="Turm Zone 1"]')).not.toBeNull();
    await click(h.querySelector('tr[data-key="armory@40"] button.pl-row')!);
    expect(chosenRow(h)).toBe('armory@40');
  });
});

/** Prüfung im Mock mit den neu angehängten Plätzen als Zeilen (der Mock kennt sonst nur die Fixture) */
function checkWithNewRows(candidate: string) {
  const view = mockHubCheck(candidate);
  const sites = (JSON.parse(candidate) as { sites: { kind: string; offsetUnits: number; hubLevel: number }[] }).sites;
  const extra = sites.slice(8).map((s, i): HubRow => ({
    key: `${s.kind}@${s.offsetUnits}`, kind: s.kind, source: 'sites', index: 8 + i, hubLevel: s.hubLevel, line: 0, bothSides: false, towerUnits: 0, jitterUnits: 0,
    cells: Array.from({ length: 7 }, (_, j) => ({ available: j + 1 >= s.hubLevel, offsetUnits: s.offsetUnits, fromDepth: 0, needsDeeper: false, onlyDepth: -1 })),
  }));
  return { ...view, matrix: { ...view.matrix, rows: [...view.matrix.rows, ...extra] } };
}

describe('Gebäude-Planer: „+ Platz“ im Matrix-Kopf (B-670/AC-05)', () => {
  it('legt eine Zeile in sites an (Platz 0, höchste Hub-Stufe) und wählt sie; Art und Liste im Inspektor', async () => {
    const check = vi.spyOn(backend, 'hubCheck').mockImplementation(async (c) => checkWithNewRows(c));
    const h = await mount();
    expect(h.textContent).not.toContain('Platz hinzufügen');
    expect(h.querySelector('[aria-label="Platz hinzufügen"]')).toBeNull();
    await click(button(h, '+ Platz'));
    const last = () => JSON.parse(check.mock.lastCall![0]) as { sites: unknown[]; islandSites: unknown[] };
    expect(last().sites.at(-1)).toEqual({ kind: 'armory', offsetUnits: 0, hubLevel: 7 });
    expect(chosenRow(h)).toBe('armory@0');
    expect(h.querySelector<HTMLInputElement>('input[aria-label="Platz (Units)"]')!.value).toBe('0');
    await pickOption(h, 'Art', 'healer');
    await flush();
    expect(last().sites.at(-1)).toEqual({ kind: 'healer', offsetUnits: 0, hubLevel: 7 });
    expect(chosenRow(h)).toBe('healer@0');
    const islands = last().islandSites.length;
    await click(h.querySelector('[aria-label="Liste"] button:last-child')!);
    expect(last().sites).toHaveLength(8);
    expect(last().islandSites).toHaveLength(islands + 1);
    expect(last().islandSites.at(-1)).toEqual({ kind: 'healer', offsetUnits: 0, hubLevel: 7 });
  });
});
