// @vitest-environment happy-dom
import { Theme } from '@radix-ui/themes';
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { backend } from '../api';
import type { AssetSlot } from '../api';
import type { FigureItem } from '../playback/figureView';
import { optionTexts } from '../../../../webkit/testing/options';
import { pickOption, selectTrigger, selectedText } from '../../../../webkit/testing/select';
import { MonarchPage } from './MonarchPage';

// B-463 (WZ10.2) im DOM: Monarch-Seite mit einer Attrappe statt Phaser; die Bühne bekommt, was die Seite wählt.
(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

const calls = vi.hoisted(() => ({ shown: [] as unknown[], controls: [] as unknown[], zooms: [] as number[] }));
vi.mock('../stage/PreviewStage', () => ({
  PreviewStage: class {
    async showFigures(items: unknown) { calls.shown.push(items); return [{ gap: null, frames: 6 }]; }
    setControl(c: unknown) { calls.controls.push(c); }
    setFigureZoom(z: number) { calls.zooms.push(z); }
    destroy() {}
  },
}));

let root: Root | null = null;
let host: HTMLElement | null = null;
const flush = () => act(async () => { for (let i = 0; i < 5; i++) await new Promise((r) => setTimeout(r, 0)); });
const lastItems = () => calls.shown.at(-1) as FigureItem[];

async function mount(node: React.ReactNode = <MonarchPage />): Promise<HTMLElement> {
  host = document.createElement('div');
  document.body.append(host);
  root = createRoot(host);
  await act(async () => root!.render(<Theme>{node}</Theme>));
  await flush();
  return host;
}

const field = (h: HTMLElement, label: string) => selectTrigger(h, label) as HTMLButtonElement;
/** Eintrag mit diesem Text in der Auswahl wählen */
async function choose(h: HTMLElement, label: string, text: string) {
  await pickOption(h, label, text);
  await flush();
}

const slot = (id: string, wahl: AssetSlot['wahl']): AssetSlot => ({ id, art: 'grafik', kategorie: 'figuren', beschreibung: id, wahl, status: 'zugeordnet' });

beforeEach(() => { calls.shown.length = 0; calls.controls.length = 0; });
afterEach(() => { vi.restoreAllMocks(); act(() => root?.unmount()); host?.remove(); });

describe('Monarch-Seite (B-463/AC-01, AC-03, AC-04, AC-10)', () => {
  it('zeigt die vier Spielerfarben beritten auf dem Standard-Reittier', async () => {
    const h = await mount();
    expect(lastItems()).toHaveLength(4);
    expect(lastItems().every((i) => i.mount?.key === 'horse' && i.anim === 'idle')).toBe(true);
    expect(h.querySelectorAll('.wd-colors li')).toHaveLength(4);
    expect(h.querySelectorAll('.wd-badge')).toHaveLength(4);
    expect(h.querySelector('.wd-colors')!.textContent).toContain('Spieler 4');
  });

  it('Reittier wechseln: alle 13 stehen zur Wahl, die Bühne bekommt das gewählte', async () => {
    const h = await mount();
    expect(await optionTexts(h, 'Reittier')).toHaveLength(13);
    await choose(h, 'Reittier', 'unicorn');
    expect(lastItems().every((i) => i.mount?.key === 'unicorn')).toBe(true);
  });

  it('Bewegung: nur Stehen, Laufen und Sprinten beritten, nichts zu Fuß; Sprinten ist schneller als Laufen', async () => {
    const h = await mount();
    expect(await optionTexts(h, 'Bewegung')).toEqual(['Stehen', 'Laufen (beritten)', 'Sprinten (beritten)']);
    await choose(h, 'Bewegung', 'Laufen (beritten)');
    const walk = lastItems()[0]!.mount!.speed;
    await choose(h, 'Bewegung', 'Sprinten (beritten)');
    expect(lastItems()[0]!.mount!.speed).toBeGreaterThan(walk);
    expect(h.querySelector('.wd-monarch-main > .wd-bar .wd-gap')).toBeNull(); // keine Lücke für Laufen oder Sprinten
  });

  it('Animation einzeln wählbar: idle, run, attack; Steuerung (Schleife, Einzelbild, Tempo) geht an die Bühne', async () => {
    const h = await mount();
    expect(await optionTexts(h, 'Animation')).toEqual(['idle', 'run', 'attack']);
    await choose(h, 'Animation', 'attack');
    expect(lastItems().every((i) => i.anim === 'attack')).toBe(true);
    await act(async () => [...h.querySelectorAll('button')].find((b) => b.getAttribute('aria-label') === 'Bild vor')!.click());
    await flush();
    expect(calls.controls.at(-1)).toMatchObject({ frame: 1 });
    await choose(h, 'Tempo', '2×');
    expect(calls.controls.at(-1)).toMatchObject({ tempo: 2 });
  });
});

describe('Aktionen und Skills (B-463/AC-05 bis AC-09)', () => {
  it('Schlagen: attack mit dem Effekt swing, beritten; die Bewegung ist währenddessen gesperrt', async () => {
    const h = await mount();
    await choose(h, 'Aktion', 'Schlagen');
    expect(lastItems().every((i) => i.anim === 'attack' && i.effect?.kind === 'swing' && i.mount?.key === 'horse')).toBe(true);
    expect(field(h, 'Bewegung').disabled).toBe(true);
    expect(field(h, 'Animation').disabled).toBe(true);
    await choose(h, 'Aktion', 'Keine (Bewegung)');
    expect(field(h, 'Bewegung').disabled).toBe(false);
    expect(lastItems().every((i) => i.effect === undefined)).toBe(true);
  });

  it('Münze geben zeigt die Münze des Spiels und die Lücke des fehlenden Wurfs; Münze aufheben hat keine Lücke', async () => {
    const h = await mount();
    await choose(h, 'Aktion', 'Münze geben');
    expect(lastItems()[0]).toMatchObject({ effect: { kind: 'coinGive' }, prop: 'coin' });
    expect(h.querySelector('.wd-monarch-main > .wd-bar')!.textContent).toContain('Wurf-Animation');
    await choose(h, 'Aktion', 'Münze aufheben');
    expect(lastItems()[0]).toMatchObject({ effect: { kind: 'coin' }, prop: 'coin' });
    expect(h.querySelector('.wd-monarch-main > .wd-bar .wd-gap')).toBeNull();
  });

  it('Getroffen, Tod und Respawn: Effekte und Deckkraft des Spiels, Lücken markiert', async () => {
    const h = await mount();
    await choose(h, 'Aktion', 'Tod');
    expect(lastItems()[0]).toMatchObject({ effect: { kind: 'down' }, alpha: 0.25 });
    await choose(h, 'Aktion', 'Respawn');
    expect(lastItems()[0]).toMatchObject({ fade: { from: 0.25, to: 1 } });
    await choose(h, 'Aktion', 'Getroffen');
    expect(lastItems()[0]).toMatchObject({ effect: { kind: 'hit' } });
    expect(h.querySelector('.wd-monarch-main > .wd-bar')!.textContent).toContain('Treffer-Animation');
  });

  it('die Skill-Liste zeigt alle Skills mit Linie, Stufe, Art und dem Effekt des Spiels statt der Lücke (B-562/AC-01)', async () => {
    const h = await mount();
    const rows = h.querySelectorAll('.wd-skills tbody tr');
    expect(rows).toHaveLength(26);
    expect(h.querySelector('[data-skill="fireball"]')!.textContent).toContain('Zauberer');
    expect(h.querySelector('.wd-skills')!.textContent).not.toContain('keine Darstellung im Spiel');
    expect(h.querySelector('[data-skill="backstab"]')!.textContent).toContain('Dieb');
    expect(h.querySelector('[data-skill="backstab"] .wd-skill-swatch')).not.toBeNull();
  });
});

describe('Grafik-Auswahl (B-463/AC-12)', () => {
  const slots = [
    slot('monarch:2', [{ datei: 'sprites/medieval-king-2/idle.png', sheet: 'medieval-king-2' }, { datei: 'sprites/medieval-king/idle.png', sheet: 'medieval-king' }, { datei: 'sprites/x/idle.png', sheet: 'gibtsnicht' }]),
    slot('mount:horse', [{ datei: 'sprites/horse/idle.png', sheet: 'horse' }, { datei: 'sprites/white-horse/idle.png', sheet: 'white-horse' }]),
  ];

  it('Präferenz 1 ist voreingestellt, eine andere Zuordnung wechselt das Sheet der Vorschau, das Register bleibt unverändert', async () => {
    vi.spyOn(backend, 'assetsList').mockResolvedValue({ slots, error: '', preview: '/', stand: 's' });
    const set = vi.spyOn(backend, 'assetsSet');
    const h = await mount();
    expect(selectedText(h, 'Grafik Monarch 2')).toBe('Präferenz 1: medieval-king-2');
    expect(lastItems()[1]!.spec.sheet).toBe('medieval-king-2');
    await choose(h, 'Grafik Monarch 2', 'Präferenz 2: medieval-king');
    expect(lastItems()[1]!.spec.sheet).toBe('medieval-king');
    await choose(h, 'Grafik Reittier', 'Präferenz 2: white-horse');
    expect(lastItems()[0]!.mount!.sheet).toBe('white-horse');
    expect(set).not.toHaveBeenCalled();
  });

  it('ein Sheet ohne Eintrag in data/sprites.json ist eine Lücke, die Vorschau zeigt die Spiel-Figur', async () => {
    vi.spyOn(backend, 'assetsList').mockResolvedValue({ slots, error: '', preview: '/', stand: 's' });
    const h = await mount();
    await choose(h, 'Grafik Monarch 2', 'Präferenz 3: gibtsnicht');
    expect(h.querySelector('.wd-colors li[data-index="1"] .wd-gap')!.textContent).toContain('gibtsnicht');
    expect(lastItems()[1]!.spec.sheet).toBe('medieval-king-2');
  });
});

describe('Zoom und Layout (B-502)', () => {
  // happy-dom übernimmt altKey nicht aus dem WheelEvent-Init: die Felder auf das Ereignis setzen
  const wheel = (h: HTMLElement, f: { deltaY: number; altKey: boolean }) => {
    const e = Object.assign(new Event('wheel', { bubbles: true, cancelable: true }), f);
    h.querySelector('.wd-monarch-stage')!.dispatchEvent(e);
  };
  const zoomButton = (h: HTMLElement, label: string) => [...h.querySelectorAll('button')].find((b) => b.getAttribute('aria-label') === label)! as HTMLButtonElement;

  it('Knöpfe und Alt + Mausrad zoomen die Ansicht, Zurücksetzen stellt den Spielmaßstab her', async () => {
    const h = await mount();
    expect(calls.zooms.at(-1)).toBe(1);
    expect(zoomButton(h, 'Zoom zurücksetzen').disabled).toBe(true);
    await act(async () => zoomButton(h, 'Vergrößern').click());
    await flush();
    expect(calls.zooms.at(-1)).toBe(1.25);
    expect(h.querySelector('[aria-label="Zoomstufe"]')!.textContent).toBe('125 %');
    await act(async () => wheel(h, { deltaY: 100, altKey: true }));
    await flush();
    expect(calls.zooms.at(-1)).toBe(1);
    await act(async () => wheel(h, { deltaY: -100, altKey: false })); // ohne Alt: nichts
    expect(calls.zooms.at(-1)).toBe(1);
    await act(async () => zoomButton(h, 'Vergrößern').click());
    await act(async () => zoomButton(h, 'Zoom zurücksetzen').click());
    await flush();
    expect(calls.zooms.at(-1)).toBe(1);
  });

  it('zwei Spalten: Skills links, Monarchen rechts', async () => {
    const h = await mount();
    const [left, right] = [...h.querySelector('.wd-monarch')!.children];
    expect(left!.querySelector('[aria-label="Skills"]')).not.toBeNull();
    expect(right!.querySelector('[aria-label="Figuren-Bühne"]')).not.toBeNull();
    expect(right!.querySelector('[aria-label="Spielerfarben"]')).not.toBeNull();
  });
});
