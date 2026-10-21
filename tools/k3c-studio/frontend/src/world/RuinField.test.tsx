// @vitest-environment happy-dom
import { Theme } from '@radix-ui/themes';
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { backend } from '../api';
import { mockWorldPreview } from '../api/mockWorld';
import { slotHints } from './chunkMarks';
import { normalized, ruinOf, setRuin } from './plan';
import { WorldPage } from './WorldPage';

// WZ14.7 (B-468/AC-05): Ruine je geplantem Chunk setzen, prüfen, entfernen und in der Vorschau zeigen (Mock-Backend).
vi.mock('../stage/PreviewStage', () => ({ PreviewStage: class { show() {} refresh() {} setLook() {} setPosition() {} destroy() {} } }));
(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

let root: Root | null = null;
let host: HTMLElement | null = null;

const wait = (ms = 0) => act(async () => { await new Promise((r) => setTimeout(r, ms)); });
const flush = async () => { for (let i = 0; i < 4; i++) await wait(); };
const settle = async () => { await wait(260); await flush(); };

async function mount(): Promise<HTMLElement> {
  host = document.createElement('div');
  document.body.append(host);
  root = createRoot(host);
  await act(async () => root!.render(<Theme><WorldPage /></Theme>));
  await flush();
  return host;
}

const button = (h: HTMLElement, label: string) => [...h.querySelectorAll('button')].find((b) => b.textContent === label || b.getAttribute('aria-label') === label)!;
const field = (h: HTMLElement, label: string) => h.querySelector<HTMLInputElement>(`[aria-label="${label}"]`)!;
const click = async (el: HTMLElement) => { await act(async () => el.click()); await flush(); };

async function type(el: HTMLInputElement, value: string) {
  Object.getOwnPropertyDescriptor(HTMLInputElement.prototype, 'value')!.set!.call(el, value);
  await act(async () => el.dispatchEvent(new Event('input', { bubbles: true })));
}

beforeEach(async () => {
  const view = await backend.planLoad();
  await backend.planSave('island1', 0, [], 0, view.stand);
});

afterEach(() => {
  vi.restoreAllMocks();
  act(() => root?.unmount());
  host?.remove();
});

describe('Ruine im Entwurf (reine Logik)', () => {
  it('setRuin setzt höchstens eine Ruine, null entfernt sie; normalized hält buildings in fester Reihenfolge', () => {
    const draft = [{ slot: 2, kind: 'clearing', objects: [{ kind: 'tree', at: 10 }] }];
    const set = setRuin(draft, 0, { kind: 'smithy', at: 30 });
    expect(ruinOf(set[0]!)).toEqual({ kind: 'smithy', at: 30 });
    expect(JSON.stringify(normalized(set))).toBe('[{"slot":2,"kind":"clearing","objects":[{"kind":"tree","at":10}],"buildings":[{"kind":"smithy","at":30}]}]');
    expect(JSON.stringify(normalized(setRuin(set, 0, null)))).toBe('[{"slot":2,"kind":"clearing","objects":[{"kind":"tree","at":10}]}]');
  });

  it('der Hinweis am Chunk nennt die Ruine', () => {
    const [hint] = slotHints([{ slot: 2, kind: 'clearing', buildings: [{ kind: 'smithy', at: 30 }] }], { ok: true, slot: 0, error: '' });
    expect(hint!.label).toContain('Ruine: Schmiede bei 30');
  });

  it('die Mock-Vorschau zeigt die Ruine des Entwurfs als Platz mit Kennung', () => {
    const args = { seed: 'k3c', source: { biome: 'forest', planned: [{ slot: 2, kind: 'clearing', buildings: [{ kind: 'storage', at: 25 }] }] }, hubLevel: 5, phase: 'day', progress: 0, buildings: {} };
    const sites = mockWorldPreview(args).state!.sites as { kind: string; x: number; ruin?: boolean }[];
    const ruins = sites.filter((s) => s.ruin);
    expect(ruins.map((s) => [s.kind, s.x])).toEqual([['storage', (5 + 2) * 30 + 25]]);
  });
});

describe('Feld „Ruine“ im Editor (WZ14.7)', () => {
  it('setzen, Art ändern, speichern, entfernen', async () => {
    const h = await mount();
    await click(button(h, 'Chunk hinzufügen'));
    await click(button(h, 'Ruine'));
    expect(field(h, 'Ruine Platz +1').value).not.toBe('');
    await type(field(h, 'Lage Ruine Platz +1'), '30');
    await settle();
    expect(button(h, 'Speichern').disabled).toBe(false);
    await click(button(h, 'Speichern'));
    const saved = (await backend.planLoad()).islands[0]!.planned['0']!;
    expect(saved[0]!.buildings).toEqual([{ kind: 'workshop', at: 30 }]);
    await click(button(h, 'Ruine Platz +1 entfernen'));
    expect(field(h, 'Ruine Platz +1')).toBeNull();
    expect(button(h, 'Ruine')).toBeDefined();
  });

  it('eine unerlaubte Art zeigt den Fehler des Backends am Platz, Speichern ist gesperrt', async () => {
    const h = await mount();
    await click(button(h, 'Chunk hinzufügen'));
    await click(button(h, 'Ruine'));
    await type(field(h, 'Ruine Platz +1'), 'tower');
    await settle();
    const error = h.querySelector<HTMLElement>('.wd-planned[data-slot="1"] .wd-error')!;
    expect(error.textContent).toContain('tower');
    expect(button(h, 'Speichern').disabled).toBe(true);
  });
});
