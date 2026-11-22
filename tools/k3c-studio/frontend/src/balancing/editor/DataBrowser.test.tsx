// @vitest-environment happy-dom
import { Theme } from '@radix-ui/themes';
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, describe, expect, it, vi } from 'vitest';
import type { BalanceDataFileValue } from '../../api';
import { DataBrowser, searchValues } from './DataBrowser';

// B-661/AC-05 (WZ32.4): Daten-Browser filtert über alle variierbaren Dateien, „+ fest“ und „+ Bereich“, „in den Varianten“.

(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

const VALUES: BalanceDataFileValue[] = [
  { file: 'economy.json', path: 'gatherables.tree.markCost', value: 1 },
  { file: 'economy.json', path: 'purse.startGold', value: 14 },
  { file: 'buildings.json', path: 'wall.cost.gold', value: 2 },
  { file: 'buildings.json', path: 'tower.cost.gold', value: 4 },
  { file: 'waves.json', path: 'perExtraPlayer', value: 0.25 },
];

let root: Root | null = null;
const flush = async () => { for (let i = 0; i < 3; i++) await act(async () => new Promise((r) => setTimeout(r, 0))); };

async function mount(props: Partial<Parameters<typeof DataBrowser>[0]> = {}) {
  const host = document.createElement('div');
  document.body.appendChild(host);
  root = createRoot(host);
  const all = { values: VALUES, error: '', used: new Set<string>(), onAdd: () => {}, ...props };
  act(() => root!.render(<Theme><DataBrowser {...all} /></Theme>));
  await flush();
  return host;
}

async function search(host: HTMLElement, text: string) {
  const input = host.querySelector<HTMLInputElement>('input[aria-label="Werte in data/ durchsuchen"]')!;
  await act(async () => {
    Object.getOwnPropertyDescriptor(HTMLInputElement.prototype, 'value')!.set!.call(input, text);
    input.dispatchEvent(new Event('input', { bubbles: true }));
  });
}

const paths = (host: HTMLElement) => [...host.querySelectorAll('.bal-ed-val')].map((e) => e.getAttribute('data-path'));

afterEach(() => {
  act(() => root?.unmount());
  root = null;
  document.body.innerHTML = '';
});

describe('Daten-Browser (AC-05)', () => {
  it('sucht über Datei und Pfad aller Dateien, gruppiert nach Datei', async () => {
    const host = await mount();
    expect(host.textContent).toContain('3 Dateien · 5 Werte');
    expect([...host.querySelectorAll('[data-file]')].map((e) => e.getAttribute('data-file'))).toEqual(['economy.json', 'buildings.json', 'waves.json']);
    await search(host, 'cost gold');
    expect(paths(host)).toEqual(['wall.cost.gold', 'tower.cost.gold']);
    await search(host, 'WAVES');
    expect(paths(host)).toEqual(['perExtraPlayer']);
    await search(host, 'gibtsnicht');
    expect(host.textContent).toContain('Kein Wert passt');
  });

  it('„+ fest“ und „+ Bereich“ melden den Wert; genutzte zeigen „in den Varianten“', async () => {
    const onAdd = vi.fn();
    const host = await mount({ onAdd, used: new Set(['economy.json:purse.startGold']) });
    const used = host.querySelector('[data-path="purse.startGold"]')!;
    expect(used.textContent).toContain('in den Varianten');
    expect(used.querySelector('button')).toBeNull();
    await act(async () => host.querySelector<HTMLElement>('[aria-label="wall.cost.gold als feste Variante"]')!.click());
    await act(async () => host.querySelector<HTMLElement>('[aria-label="wall.cost.gold als Bereich"]')!.click());
    expect(onAdd.mock.calls.map(([v, k]) => [v.path, k])).toEqual([['wall.cost.gold', 'fest'], ['wall.cost.gold', 'bereich']]);
  });

  it('zeigt Ladefehler und Ladezustand', async () => {
    expect((await mount({ values: null, error: 'weg' })).querySelector('[role="alert"]')!.textContent).toContain('weg');
    act(() => root?.unmount());
    expect((await mount({ values: null })).textContent).toContain('Lade Werte');
  });

  it('searchValues: jedes Wort muss passen', () => {
    expect(searchValues(VALUES, 'economy mark')).toHaveLength(1);
    expect(searchValues(VALUES, '')).toHaveLength(5);
  });
});
