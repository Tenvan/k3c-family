// @vitest-environment happy-dom
import { Theme } from '@radix-ui/themes';
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, describe, expect, it } from 'vitest';
import { BuildingFilter } from './BuildingFilter';
import { buildingToggles } from './stageLogic';

// B-477/AC-05 im DOM: Gebäude-Filter nach Art gruppiert, Gruppenschalter, aufklappbare Plätze.
(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

const TOGGLES = buildingToggles([{ kind: 'wall', x: 322 }, { kind: 'wall', x: 345 }, { kind: 'tower', x: 330 }]);

let root: Root | null = null;
let host: HTMLElement | null = null;
let calls: { keys: string[]; on: boolean }[] = [];

async function mount(off: string[] = []): Promise<HTMLElement> {
  host = document.createElement('div');
  document.body.append(host);
  root = createRoot(host);
  calls = [];
  await act(async () => root!.render(<Theme><BuildingFilter toggles={TOGGLES} off={new Set(off)} onSet={(keys, on) => calls.push({ keys, on })} /></Theme>));
  return host;
}

const heads = (h: HTMLElement) => [...h.querySelectorAll<HTMLElement>('.wd-group-name')];
const items = (h: HTMLElement) => [...h.querySelectorAll<HTMLElement>('.wd-group-items button[role="checkbox"]')];
const groupBox = (h: HTMLElement, label: string) => h.querySelector<HTMLElement>(`button[role="checkbox"][aria-label="${label} alle"]`)!;

afterEach(() => {
  act(() => root?.unmount());
  host?.remove();
});

describe('Gebäude-Filter (B-477/AC-05)', () => {
  it('zeigt eine Zeile je Art mit Zähler, die Plätze sind eingeklappt', async () => {
    const h = await mount();
    expect(heads(h).map((b) => b.textContent!.replace(/\s+/g, ' ').trim())).toEqual(['▸ Mauern 2/2', '▸ Türme 1/1']);
    expect(items(h)).toHaveLength(0);
  });

  it('Aufklappen zeigt die Plätze der Art mit ihrer Position', async () => {
    const h = await mount();
    await act(async () => heads(h)[0]!.click());
    expect(heads(h)[0]!.getAttribute('aria-expanded')).toBe('true');
    expect(items(h).map((b) => b.getAttribute('aria-label'))).toEqual(['wall@322', 'wall@345']);
    await act(async () => heads(h)[0]!.click());
    expect(items(h)).toHaveLength(0);
  });

  it('der Gruppenschalter schaltet alle Plätze der Art aus; sind alle aus, wieder an', async () => {
    const h = await mount();
    await act(async () => groupBox(h, 'Mauern').click());
    expect(calls).toEqual([{ keys: ['wall@322', 'wall@345'], on: false }]);
    await act(async () => root!.unmount());
    root = null;
    host!.remove();
    const again = await mount(['wall@322', 'wall@345']);
    await act(async () => groupBox(again, 'Mauern').click());
    expect(calls).toEqual([{ keys: ['wall@322', 'wall@345'], on: true }]);
  });

  it('gemischte Auswahl: Zähler 1/2, Gruppenschalter gemischt, ein Klick schaltet alle Plätze der Art ein', async () => {
    const h = await mount(['wall@345']);
    expect(heads(h)[0]!.textContent).toContain('1/2');
    expect(groupBox(h, 'Mauern').getAttribute('aria-checked')).toBe('mixed');
    await act(async () => groupBox(h, 'Mauern').click());
    expect(calls).toEqual([{ keys: ['wall@322', 'wall@345'], on: true }]);
  });

  it('ein einzelner Platz schaltet nur sich', async () => {
    const h = await mount();
    await act(async () => heads(h)[1]!.click());
    await act(async () => items(h)[0]!.click());
    expect(calls).toEqual([{ keys: ['tower@330'], on: false }]);
  });
});
