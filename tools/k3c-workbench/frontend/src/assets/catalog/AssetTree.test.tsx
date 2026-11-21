// @vitest-environment happy-dom
import { Theme } from '@radix-ui/themes';
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, describe, expect, it, vi } from 'vitest';
import type { AssetEintrag } from '../api';
import { AssetTree } from './AssetTree';

// B-1216: Baum der Assets mit Gruppen (Symbol, Name, Zahl), Pfeil zum Auf- und Zuklappen und Auswahl per Klick.

(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

let root: Root | null = null;
afterEach(() => {
  act(() => root?.unmount());
  root = null;
  document.body.innerHTML = '';
});

const asset = (id: string, gruppe: string, rig = 'figur', gueltig = true): AssetEintrag => ({ id, rig, gruppe, pfad: `${gruppe}/${id}`, gueltig, geaendert: false });

function mount(assets: AssetEintrag[], selected: string, onSelect: (id: string) => void): HTMLElement {
  const host = document.createElement('div');
  document.body.appendChild(host);
  root = createRoot(host);
  act(() => root!.render(<Theme><AssetTree assets={assets} selected={selected} onSelect={onSelect} badges={(a) => (a.id === 'monarch' ? <b data-badge>im Diff</b> : null)} /></Theme>));
  return host;
}

describe('Asset-Baum (B-1216)', () => {
  const assets = [asset('monarch', 'figuren'), asset('wache', 'figuren'), asset('mauer', 'gebaeude-paper', 'gebaeude'), asset('banner', 'requisiten', 'requisit', false)];

  it('zeigt Gruppen mit Symbol, Namen und Zahl und darunter die Assets mit Abzeichen', () => {
    const host = mount(assets, '', () => undefined);
    const groups = [...host.querySelectorAll('[role="group"]')].map((g) => g.getAttribute('data-group'));
    expect(groups).toEqual(['figuren', 'gebaeude-paper', 'requisiten']);
    const figuren = host.querySelector('[data-group="figuren"]')!;
    expect(figuren.textContent).toContain('Figuren');
    expect(figuren.textContent).toContain('2');
    expect(figuren.querySelectorAll('[data-asset]')).toHaveLength(2);
    expect(figuren.querySelector('[data-badge]')).toBeTruthy();
    expect(host.querySelector('[data-asset="banner"]')?.getAttribute('data-invalid')).toBe('true');
  });

  it('der Pfeil klappt eine Gruppe zu und auf, ein Klick wählt das Asset', async () => {
    const picked = vi.fn();
    const host = mount(assets, 'wache', picked);
    expect(host.querySelector('[data-asset="wache"]')?.getAttribute('aria-selected')).toBe('true');
    await act(async () => (host.querySelector('[aria-label="Figuren einklappen"]') as HTMLElement).click());
    expect(host.querySelector('[data-group="figuren"] [data-asset]')).toBeNull();
    await act(async () => (host.querySelector('[aria-label="Figuren aufklappen"]') as HTMLElement).click());
    await act(async () => (host.querySelector('[data-asset="monarch"]') as HTMLElement).click());
    expect(picked).toHaveBeenCalledWith('monarch');
  });
});
