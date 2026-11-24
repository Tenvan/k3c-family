// @vitest-environment happy-dom
import { Theme } from '@radix-ui/themes';
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, describe, expect, it, vi } from 'vitest';
import type { EndBossOption, WorldIsland } from '../api';
import { EndBossSelect, SkinTable, skinProblem } from './WorldBoss';

(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;
let root: Root | null = null;
afterEach(() => { act(() => root?.unmount()); root = null; document.body.innerHTML = ''; });

const bosses: EndBossOption[] = [{ id: 'forestChieftain', name: 'Wald-Häuptling' }];
const island: WorldIsland = { id: 'island1', name: 'Eins', depths: [0], biomes: ['wald'], plannedChunks: {}, chunkCounts: {}, endBoss: 'altesAlt' };

function mount(node: React.ReactElement): HTMLElement {
  const host = document.createElement('div');
  document.body.appendChild(host);
  root = createRoot(host);
  act(() => root!.render(<Theme>{node}</Theme>));
  return host;
}

describe('Skin-Paare (SI/WE6.4, B-984)', () => {
  it('lässt nur <gruppe>/<asset> derselben Gruppe zu', () => {
    expect(skinProblem('gegner/wolf', 'gegner/ninja')).toBe('');
    expect(skinProblem('wolf', 'gegner/ninja')).not.toBe('');
    expect(skinProblem('gegner/wolf', 'objekte/tanne')).not.toBe('');
  });
});

describe('Endboss je Insel (WE6.4, B-981)', () => {
  it('zeigt den Standard, markiert einen unbekannten Bestandswert', () => {
    const host = mount(<EndBossSelect island={island} options={bosses} onChange={() => undefined} />);
    expect(host.textContent).toContain('altesAlt (unbekannt)');
    expect(host.querySelector('[aria-label="Endboss von Eins"]')).not.toBeNull();
  });
});

describe('Skin-Tabelle (WE6.4)', () => {
  it('sperrt das Speichern bei einem ungültigen Paar und speichert gültige Paare', () => {
    const onSave = vi.fn();
    const host = mount(<SkinTable skins={{ 'gegner/wolf': 'gegner/ninja' }} onSave={onSave} />);
    const save = [...host.querySelectorAll('button')].find((b) => b.textContent === 'Skins speichern') as HTMLButtonElement;
    expect(save.disabled).toBe(false);
    expect(host.textContent).not.toContain('derselben Gruppe');
  });
});

describe('Skin-Tabelle sperrt ungültige Paare (WE6.4)', () => {
  it('ein Paar ohne Gruppe macht das Speichern unmöglich und zeigt den Grund', () => {
    const onSave = vi.fn();
    const host = mount(<SkinTable skins={{ wolf: 'gegner/ninja' }} onSave={onSave} />);
    const save = [...host.querySelectorAll('button')].find((b) => b.textContent === 'Skins speichern') as HTMLButtonElement;
    expect(save.disabled).toBe(true);
    expect(host.textContent).toContain('Form <gruppe>/<asset>');
  });
});
