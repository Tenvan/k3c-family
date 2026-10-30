// @vitest-environment happy-dom
import { Theme } from '@radix-ui/themes';
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { backend, type HubRow } from '../api';
import { mockHubCheck } from '../api/mockHub';
import { pickOption } from '../../../../webkit/testing/select';
import { PlannerPage } from './PlannerPage';

// B-670 im DOM: eine Auswahl für Bühne, Matrix und Platz-Chips (AC-04), Chip-Leiste statt Auswahllisten je Platz (AC-03),
// „+ Platz“ im Matrix-Kopf (AC-05). Die Bühne ist eine Attrappe, die Klicks meldet und Rahmen und Sprünge mitschreibt.
// Die Mock-Welt (api/previewFixture.json) hat die Hub-Mitte bei x 450: Taverne 418 (−32), Rüstkammer 490 (+40), Turm 398 (±52).
(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

type Hit = { kind: string; x: number; shift: boolean; double: boolean } | null;
interface Stage { marks: (readonly number[])[]; jumps: number[]; site: (hit: Hit) => void }
const stages: Stage[] = [];
vi.mock('../stage/PreviewStage', () => ({
  PreviewStage: class {
    state: Stage;
    constructor(_parent: unknown, _fail: unknown, _pan: unknown, _wheel: unknown, site: (hit: Hit) => void) { this.state = { marks: [], jumps: [], site }; stages.push(this.state); }
    show() {}
    setLook() {}
    setPosition() {}
    destroy() {}
    setMarks(m: { selected: readonly number[] }) { this.state.marks.push(m.selected); }
    focusX(x: number) { this.state.jumps.push(x); }
  },
}));

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
const frame = () => stages[0]!.marks.at(-1);
const stageClick = async (kind: string, x: number) => { await act(async () => stages[0]!.site({ kind, x, shift: false, double: false })); await flush(); };

beforeEach(() => { stages.length = 0; });
afterEach(() => {
  vi.restoreAllMocks();
  act(() => root?.unmount());
  host?.remove();
});

describe('Gebäude-Planer: eine Auswahl für Bühne, Matrix und Platz-Chips (B-670/AC-04)', () => {
  it('Klick auf die Taverne in der Bühne wählt ihre Zeile; Rahmen ohne Sprung, Inspektor zeigt ihre Felder', async () => {
    const h = await mount();
    expect(h.querySelector('.pl-inspector')!.textContent).toContain('Bauplatz wählen');
    await stageClick('tavern', 418);
    expect(chosenRow(h)).toBe('tavern@-32');
    expect(frame()).toEqual([418]);
    expect(stages[0]!.jumps).toEqual([]);
    expect(h.querySelector('.pl-editor')!.textContent).toContain('Taverne');
    expect(h.querySelector<HTMLInputElement>('input[aria-label="Platz (Units)"]')!.value).toBe('-32');
  });

  it('ein Turm wählt den Baugrund der Zone 1, freie Fläche hebt die Auswahl auf', async () => {
    const h = await mount();
    await stageClick('tower', 398);
    expect(chosenRow(h)).toBe('zone1');
    await act(async () => stages[0]!.site(null));
    await flush();
    expect(chosenRow(h)).toBe('');
    expect(frame()).toEqual([]);
  });

  it('Klick auf eine Matrix-Zeile: Rahmen und Sprung in der Bühne, der Platz-Chip ist gewählt', async () => {
    const h = await mount();
    await click(h.querySelector('tr[data-key="armory@40"] button.pl-row')!);
    expect(frame()).toEqual([490]);
    expect(stages[0]!.jumps.at(-1)).toBe(490);
    await click(button(h, 'Rüstkammern: Plätze'));
    expect(button(h, 'Rüstkammern bei x 490').getAttribute('aria-pressed')).toBe('true');
  });

  it('Klick auf einen Platz-Chip wählt die Zeile und springt zum Platz', async () => {
    const h = await mount();
    await click(button(h, 'Heilplätze: Plätze'));
    await click(button(h, 'Heilplätze bei x 410'));
    expect(chosenRow(h)).toBe('healer@-40');
    expect(frame()).toEqual([410]);
    expect(stages[0]!.jumps.at(-1)).toBe(410);
    expect(h.querySelector('.pl-editor')!.textContent).toContain('Heil');
  });

  it('der Vorschau-Zustand der gewählten Plätze steht im Inspektor (Segment-Knöpfe aus B-666)', async () => {
    const h = await mount();
    await stageClick('tavern', 418);
    const card = h.querySelector<HTMLElement>('.pl-inspector [aria-label="Gewählte Bauplätze"]')!;
    expect(card.querySelector('[aria-label="Zustand"]')).not.toBeNull();
  });
});

describe('Gebäude-Planer: Chip-Leiste statt Auswahllisten je Platz (B-670/AC-03)', () => {
  it('keine Auswahlliste je Bauplatz, auch mit aufgeklappten Mauern; der Chip blendet die Art aus und ein', async () => {
    const preview = vi.spyOn(backend, 'worldPreview');
    const h = await mount();
    const walls = button(h, 'Mauern: Plätze');
    await click(walls);
    const chips = h.querySelectorAll('[aria-label="Mauern: Bauplätze"] button');
    expect(chips.length).toBeGreaterThan(10);
    expect(h.querySelectorAll('.pl-chipbar [role="combobox"]')).toHaveLength(0);
    expect(h.querySelectorAll('[role="combobox"]')).toHaveLength(1); // nur die Tageszeit
    const kind = [...h.querySelectorAll<HTMLButtonElement>('.pl-kind')].find((b) => b.textContent!.startsWith('Mauern'))!;
    expect(kind.textContent).toBe(`Mauern ${chips.length}`);
    await click(kind);
    const off = preview.mock.lastCall![0].buildings;
    expect(Object.keys(off).filter((k) => k.startsWith('wall@'))).toHaveLength(chips.length);
    expect(kind.closest('[data-state]')!.getAttribute('data-state')).toBe('none');
    await click(kind);
    expect(kind.closest('[data-state]')!.getAttribute('data-state')).toBe('all'); // wieder eingeblendet (Welt ohne Auswahl, keine neue Anfrage nötig)
    expect(kind.getAttribute('aria-pressed')).toBe('true');
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
