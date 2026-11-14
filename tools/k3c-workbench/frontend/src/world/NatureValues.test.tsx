// @vitest-environment happy-dom
import { Theme } from '@radix-ui/themes';
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, describe, expect, it } from 'vitest';
import { backend, type WorldLevel } from '../api';
import { natureRows } from './NatureValues';
import { WorldPage } from './WorldPage';

// B-891/AC-05: Der Welten-Editor zeigt je Stufe die Natur-Werte mit Quelle (Seed oder fest) und setzt sie über das Backend.

(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

let root: Root | null = null;

afterEach(() => {
  act(() => root?.unmount());
  root = null;
  document.body.innerHTML = '';
});

const level = (over: Partial<WorldLevel>): WorldLevel => ({
  seed: 'k3c', biomeId: 'forest', widthUnits: 0, chunkWidthUnits: 50, hubCenterUnits: 0, chunks: [], entities: [], planned: [],
  warnings: [], error: '', ...over,
});

describe('natureRows', () => {
  it('listet Eigenschaften, dann Varianten, mit geltendem und festem Wert', () => {
    const rows = natureRows(level({
      nature: { traits: { forestDensity: 4 }, weights: { treeSpecies: { tanne: 2, birke: 0 } } },
      natureFixed: { weights: { treeSpecies: { birke: 0 } } },
      natureTraits: [{ key: 'forestDensity', name: 'Walddichte', min: 1, max: 5, default: 3, scales: ['tree'] },
        { key: 'glow', name: 'Glut', min: 0, max: 3, default: 1, display: true }],
      natureWeights: [{ key: 'treeSpecies', name: 'Baumarten', kind: 'tree', options: [{ key: 'tanne', min: 0, max: 5 }, { key: 'birke', min: 0, max: 5 }] }],
    }));
    expect(rows.map((r) => r.key)).toEqual(['forestDensity', 'glow', 'treeSpecies.tanne', 'treeSpecies.birke']);
    expect(rows[0]).toMatchObject({ value: 4, fixed: undefined, hint: 'Anzahl tree · neutral 3' });
    expect(rows[1]!.hint).toContain('nur Darstellung');
    expect(rows[3]).toMatchObject({ value: 0, fixed: 0 });
  });

  it('ist leer ohne Natur-Felder (älteres Backend)', () => {
    expect(natureRows(level({}))).toEqual([]);
  });
});

describe('Welten-Editor › Natur', () => {
  it('zeigt die Natur der Stufe und übernimmt einen festen Wert', async () => {
    localStorage.clear();
    const host = document.createElement('div');
    document.body.appendChild(host);
    root = createRoot(host);
    act(() => root!.render(<Theme><WorldPage /></Theme>));
    for (let i = 0; i < 5; i++) await act(async () => new Promise((r) => setTimeout(r, 0)));
    const row = host.querySelector('[data-key="forestDensity"]');
    expect(row?.getAttribute('data-status')).toBe('seed');
    expect(host.querySelector('[aria-label="Quelle Walddichte"]')?.textContent).toBe('Seed: 2');

    const set = await backend.worldNatureSet('standard', 'island1', 0, 'forestDensity', 5, 'k3c');
    expect(set.natureFixed?.traits?.forestDensity).toBe(5);
    expect(set.nature?.traits?.forestDensity).toBe(5);
    const bad = await backend.worldNatureSet('standard', 'island1', 0, 'forestDensity', 9, 'k3c');
    expect(bad.rejected).toContain('außerhalb');
    const reset = await backend.worldNatureSet('standard', 'island1', 0, 'forestDensity', null, 'k3c');
    expect(reset.natureFixed?.traits?.forestDensity).toBeUndefined();
  });
});
