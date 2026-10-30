// @vitest-environment happy-dom
import { Theme } from '@radix-ui/themes';
import { act, useState } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, describe, expect, it } from 'vitest';
import type { SiteLook } from '../api';
import { BuildingInspector } from './BuildingInspector';
import { buildingToggles } from './stageLogic';

// B-666 im DOM: Voreinstellungen, gewählte Bauplätze mit Segment-Knöpfen, Zeilen je Art mit Zusammenfassung, Chips je Platz.
(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

const TOGGLES = buildingToggles([{ kind: 'wall', x: 191 }, { kind: 'wall', x: 215 }, { kind: 'wall', x: 239 }, { kind: 'tower', x: 330 }, { kind: 'tavern', x: 160 }]);

let root: Root | null = null;
let host: HTMLElement | null = null;
const state: { looks: Record<string, SiteLook>; selected: string[]; jumps: string[] } = { looks: {}, selected: [], jumps: [] };

/** Rahmen mit dem Zustand, den die Live-Vorschau hält (usePreviewWorld) */
function Harness({ off }: { off: string[] }) {
  const [looks, setLooks] = useState<Record<string, SiteLook>>({});
  const [selected, setSelected] = useState<string[]>([]);
  state.looks = looks;
  state.selected = selected;
  return (
    <BuildingInspector toggles={TOGGLES} off={new Set(off)} onSet={() => {}} looks={looks} onLooks={setLooks} selected={selected}
      onSelect={(keys, jump) => { state.jumps.push(`${keys.join(',')}|${jump}`); setSelected(keys); }} />
  );
}

async function mount(off: string[] = []): Promise<HTMLElement> {
  host = document.createElement('div');
  document.body.append(host);
  root = createRoot(host);
  state.jumps = [];
  await act(async () => root!.render(<Theme><Harness off={off} /></Theme>));
  return host;
}

const button = (h: HTMLElement, label: string) => [...h.querySelectorAll('button')].find((b) => b.textContent!.trim() === label || b.getAttribute('aria-label') === label) as HTMLButtonElement;
const click = (el: HTMLElement, init: MouseEventInit = {}) => act(async () => { el.dispatchEvent(new MouseEvent('click', { bubbles: true, ...init })); });
const summary = (h: HTMLElement, label: string) => [...h.querySelectorAll('.wd-group')].find((g) => g.textContent!.includes(label))!.querySelector('.wd-summary')!.textContent;
const sites = (h: HTMLElement) => [...h.querySelectorAll<HTMLButtonElement>('.wd-site')];
const segment = (card: HTMLElement, group: string, label: string) => [...card.querySelectorAll<HTMLButtonElement>(`[aria-label="${group}"] button`)].find((b) => b.textContent!.startsWith(label))!;

afterEach(() => {
  act(() => root?.unmount());
  host?.remove();
});

describe('Gebäude-Zustände im Inspektor (B-666)', () => {
  it('sagt, dass die Zustände nicht gespeichert werden, und setzt sie zurück', async () => {
    const h = await mount();
    expect(h.querySelector('[role="note"]')!.textContent).toContain('Nur Vorschau: Zustände werden nicht gespeichert');
    await click(button(h, 'Voll ausgebaut'));
    expect(Object.keys(state.looks).length).toBeGreaterThan(0);
    await click(button(h, 'Zurücksetzen'));
    expect(state.looks).toEqual({});
  });

  it('eine Zeile je Art mit Zusammenfassung; die Voreinstellungen setzen alle Plätze', async () => {
    const h = await mount();
    expect(summary(h, 'Mauern')).toBe('Stufe Auto · 100 %');
    await click(button(h, 'Voll ausgebaut'));
    expect(summary(h, 'Mauern')).toMatch(/Stufe [1-9] · 100 %/);
    await click(button(h, 'Nichts gebaut'));
    expect(summary(h, 'Mauern')).toBe('Stufe Auto · nicht gebaut');
    await click(button(h, 'Wie Hub-Stufe'));
    expect(state.looks).toEqual({});
    expect(button(h, 'Wie Hub-Stufe').getAttribute('aria-pressed')).toBe('true');
  });

  it('Aufklappen zeigt die Plätze als Chips mit x-Stelle; ein Klick wählt den Platz und springt hin, Umschalt erweitert', async () => {
    const h = await mount();
    expect(sites(h)).toHaveLength(0);
    await click(button(h, 'Mauern aufklappen'));
    expect(sites(h).map((s) => s.textContent)).toEqual(['191', '215', '239']);
    await click(sites(h)[1]!);
    expect(state.selected).toEqual(['wall@215']);
    expect(state.jumps.at(-1)).toBe('wall@215|true');
    await click(sites(h)[2]!, { shiftKey: true });
    expect(state.selected).toEqual(['wall@215', 'wall@239']);
    expect(h.querySelector('.wd-selected')!.textContent).toContain('2 gewählt: Mauern');
  });

});

describe('Auswahl und Zustand der Bauplätze (B-666)', () => {
  it('der gewählte Platz zeigt Baustufe und Zustand als Knöpfe; „Auf alle Mauern anwenden“ überträgt sie', async () => {
    const h = await mount();
    await click(button(h, 'Mauern aufklappen'));
    await click(sites(h)[1]!);
    const card = h.querySelector<HTMLElement>('.wd-selected')!;
    expect(card.textContent).toContain('Mauern bei x 215');
    expect(card.querySelectorAll('select, [role="combobox"]')).toHaveLength(0); // keine Auswahllisten mehr
    await click(segment(card, 'Zustand', '50'));
    await click(segment(card, 'Baustufe', '3'));
    expect(state.looks).toEqual({ 'wall@215': { level: 3, hpPercent: 50 } });
    expect(summary(h, 'Mauern')).toBe('gemischt · gemischt');
    await click(button(h, 'Auf alle 3 Mauern anwenden'));
    expect(Object.keys(state.looks).sort()).toEqual(['wall@191', 'wall@215', 'wall@239']);
    expect(summary(h, 'Mauern')).toBe('Stufe 3 · 50 %');
  });

  it('„nicht gebaut“ sperrt die Baustufe, der Chip zeigt es gestrichelt', async () => {
    const h = await mount();
    await click(button(h, 'Mauern aufklappen'));
    await click(sites(h)[0]!);
    const card = h.querySelector<HTMLElement>('.wd-selected')!;
    await click(segment(card, 'Zustand', '–'));
    expect(state.looks['wall@191']).toEqual({ unbuilt: true });
    expect([...card.querySelectorAll<HTMLButtonElement>('[aria-label="Baustufe"] button')].every((b) => b.disabled)).toBe(true);
    expect(sites(h)[0]!.dataset.unbuilt).toBe('true');
    expect(sites(h)[0]!.getAttribute('aria-label')).toContain('nicht gebaut');
  });

  it('ein Klick auf den Namen einer Art wählt alle ihre sichtbaren Plätze; ausgeblendete sind nicht anklickbar', async () => {
    const h = await mount(['wall@239']);
    await click(button(h, 'Mauern wählen'));
    expect(state.selected).toEqual(['wall@191', 'wall@215']);
    expect(state.jumps.at(-1)).toBe('wall@191,wall@215|false');
    expect(sites(h)).toHaveLength(3); // die Art wurde dabei aufgeklappt
    expect(sites(h)[2]!.disabled).toBe(true);
  });
});
