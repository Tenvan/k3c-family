// @vitest-environment happy-dom
import { Theme } from '@radix-ui/themes';
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { backend } from '../api';
import type { AssetSlot } from '../api';
import type { FigureItem } from '../playback/figureView';
import { optionTexts } from '../../../../webkit/testing/options';
import { pickOption, selectedText } from '../../../../webkit/testing/select';
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

const button = (h: HTMLElement, label: string) =>
  [...h.querySelectorAll('button')].find((b) => b.getAttribute('aria-label') === label || b.textContent === label) as HTMLButtonElement | undefined;
const click = async (h: HTMLElement, label: string) => { await act(async () => button(h, label)!.click()); await flush(); };
const hasField = (h: HTMLElement, label: string) => h.querySelector(`[role="combobox"][aria-label="${label}"]`) !== null;
const toolbar = (h: HTMLElement) => h.querySelector('[role="toolbar"]')!;
/** Eintrag mit diesem Text in der Auswahl wählen */
async function choose(h: HTMLElement, label: string, text: string) {
  await pickOption(h, label, text);
  await flush();
}

const slot = (id: string, wahl: AssetSlot['wahl']): AssetSlot => ({ id, art: 'grafik', kategorie: 'figuren', beschreibung: id, wahl, status: 'zugeordnet' });

beforeEach(() => { calls.shown.length = 0; calls.controls.length = 0; localStorage.clear(); });
afterEach(() => { vi.restoreAllMocks(); act(() => root?.unmount()); host?.remove(); });

describe('Monarch-Seite (B-463/AC-01, AC-03, AC-04, AC-10)', () => {
  it('zeigt die vier Spielerfarben beritten auf dem Standard-Reittier', async () => {
    const h = await mount();
    expect(lastItems()).toHaveLength(4);
    expect(lastItems().every((i) => i.mount?.key === 'horse' && i.anim === 'idle')).toBe(true);
    expect(h.querySelectorAll('.wd-colors li[data-index]')).toHaveLength(4);
    expect(h.querySelectorAll('.wd-badge')).toHaveLength(4);
    expect(h.querySelector('.wd-colors')!.textContent).toContain('Spieler 4');
  });

  it('Reittier wechseln: alle 13 stehen zur Wahl, die Bühne bekommt das gewählte', async () => {
    const h = await mount();
    expect(await optionTexts(h, 'Reittier')).toHaveLength(13);
    await choose(h, 'Reittier', 'unicorn');
    expect(lastItems().every((i) => i.mount?.key === 'unicorn')).toBe(true);
  });

  it('Modus als Segment: Stehen, Laufen, Sprinten (beritten, nichts zu Fuß), Aktion und Skill; Sprinten ist schneller als Laufen', async () => {
    const h = await mount();
    expect([...h.querySelectorAll('[aria-label="Modus"] button')].map((b) => b.textContent)).toEqual(['Stehen', 'Laufen', 'Sprinten', 'Aktion …', 'Skill …']);
    await click(h, 'Laufen');
    const walk = lastItems()[0]!.mount!.speed;
    expect(walk).toBeGreaterThan(0);
    await click(h, 'Sprinten');
    expect(lastItems()[0]!.mount!.speed).toBeGreaterThan(walk);
    expect(toolbar(h).querySelector('.wd-gap-chip')).toBeNull(); // keine Lücke für Laufen oder Sprinten
  });

  it('Animation einzeln wählbar: idle, run, attack; Steuerung (Schleife, Einzelbild, Tempo) geht an die Bühne', async () => {
    const h = await mount();
    expect(await optionTexts(h, 'Animation')).toEqual(['idle', 'run', 'attack']);
    await choose(h, 'Animation', 'attack');
    expect(lastItems().every((i) => i.anim === 'attack')).toBe(true);
    await click(h, 'Bild vor');
    expect(calls.controls.at(-1)).toMatchObject({ frame: 1 });
    await click(h, '2×');
    expect(calls.controls.at(-1)).toMatchObject({ tempo: 2 });
  });
});

describe('Aktionen und Skills (B-463/AC-05 bis AC-09)', () => {
  it('Schlagen: attack mit dem Effekt swing, beritten; Animation und Bewegung sind währenddessen ausgeblendet (B-671/AC-03)', async () => {
    const h = await mount();
    expect(hasField(h, 'Aktion')).toBe(false);
    expect(hasField(h, 'Skill')).toBe(false);
    await click(h, 'Aktion …');
    await choose(h, 'Aktion', 'Schlagen');
    expect(lastItems().every((i) => i.anim === 'attack' && i.effect?.kind === 'swing' && i.mount?.key === 'horse' && i.mount.speed === 0)).toBe(true);
    expect(hasField(h, 'Animation')).toBe(false);
    expect(hasField(h, 'Bewegung')).toBe(false);
    await click(h, 'Stehen');
    expect(hasField(h, 'Animation')).toBe(true);
    expect(hasField(h, 'Aktion')).toBe(false);
    expect(lastItems().every((i) => i.effect === undefined)).toBe(true);
  });

  it('Münze geben zeigt die Münze des Spiels und die Lücke des fehlenden Wurfs; Münze aufheben hat keine Lücke', async () => {
    const h = await mount();
    await click(h, 'Aktion …');
    await choose(h, 'Aktion', 'Münze geben');
    expect(lastItems()[0]).toMatchObject({ effect: { kind: 'coinGive' }, prop: 'coin' });
    expect(toolbar(h).querySelector('.wd-gap-chip')!.getAttribute('title')).toContain('Wurf-Animation');
    await choose(h, 'Aktion', 'Münze aufheben');
    expect(lastItems()[0]).toMatchObject({ effect: { kind: 'coin' }, prop: 'coin' });
    expect(toolbar(h).querySelector('.wd-gap-chip')).toBeNull();
  });

  it('Getroffen, Tod und Respawn: Effekte und Deckkraft des Spiels, Lücken markiert', async () => {
    const h = await mount();
    await click(h, 'Aktion …');
    await choose(h, 'Aktion', 'Tod');
    expect(lastItems()[0]).toMatchObject({ effect: { kind: 'down' }, alpha: 0.25 });
    await choose(h, 'Aktion', 'Respawn');
    expect(lastItems()[0]).toMatchObject({ fade: { from: 0.25, to: 1 } });
    await choose(h, 'Aktion', 'Getroffen');
    expect(lastItems()[0]).toMatchObject({ effect: { kind: 'hit' } });
    expect(toolbar(h).querySelector('.wd-gap-chip')!.getAttribute('title')).toContain('Treffer-Animation');
  });

  it('die Skill-Liste zeigt alle Skills mit Linie, Stufe, Art und dem Effekt des Spiels statt der Lücke (B-562/AC-01)', async () => {
    const h = await mount();
    await click(h, 'Skills 26');
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
    expect(zoomButton(h, 'Zoomstufe').disabled).toBe(true);
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
    await act(async () => zoomButton(h, 'Zoomstufe').click()); // Klick auf den Wert = Spielmaßstab
    await flush();
    expect(calls.zooms.at(-1)).toBe(1);
  });

  it('Auswahl, Wiedergabe und Zoom stehen in einer Leiste; die Bühne steht ohne Skill-Tabelle (B-671/AC-01)', async () => {
    const h = await mount();
    const bar = toolbar(h);
    for (const label of ['Reittier', 'Modus', 'Animation', 'Wiedergabe', 'Tempo', 'Zoom']) expect(bar.querySelector(`[aria-label="${label}"]`), label).not.toBeNull();
    expect(h.querySelector('[aria-label="Skills"]')).toBeNull();
    const [main] = [...h.querySelector('.wd-monarch')!.children];
    expect(main!.querySelector('[aria-label="Figuren-Bühne"]')).not.toBeNull();
    expect(main!.querySelector('[aria-label="Spielerfarben"]')).not.toBeNull();
  });
});

describe('Skills-Panel und Skill abspielen (B-671)', () => {
  it('das Panel ist standardmäßig zu, öffnet über den Knopf und bleibt nach Neuladen offen (AC-02)', async () => {
    let h = await mount();
    expect(h.querySelector('[aria-label="Skills"]')).toBeNull();
    await click(h, 'Skills 26');
    expect(h.querySelector('[aria-label="Skills"]')).not.toBeNull();
    expect(h.querySelector('.wd-monarch')!.getAttribute('data-panel')).toBe('open');
    act(() => root!.unmount());
    host!.remove();
    h = await mount();
    expect(h.querySelector('[aria-label="Skills"]')).not.toBeNull();
    await click(h, 'Skills schließen');
    act(() => root!.unmount());
    host!.remove();
    h = await mount();
    expect(h.querySelector('[aria-label="Skills"]')).toBeNull();
  });

  it('Filter nach Linie als Segment', async () => {
    const h = await mount();
    await click(h, 'Skills 26');
    expect([...h.querySelectorAll('[aria-label="Linie"] button')].map((b) => b.textContent)).toEqual(['Alle', 'Tank', 'Zauberer', 'Heiler', 'Dieb']);
    await click(h, 'Dieb');
    expect([...h.querySelectorAll('.wd-skills tbody tr')].map((r) => (r as HTMLElement).dataset.skill)).toEqual(['backstab', 'smokeBomb']);
  });

  it('Modus „Skill“: Auswahl nach Linie, alle vier Monarchen spielen den Effekt aus skillLook; Bewegung und Animation entfallen (AC-03, AC-06)', async () => {
    const h = await mount();
    await click(h, 'Skill …');
    expect(hasField(h, 'Skill')).toBe(true);
    expect(hasField(h, 'Animation')).toBe(false);
    expect(hasField(h, 'Aktion')).toBe(false);
    expect(selectedText(h, 'Skill')).toBe('Zauberer · fireball');
    expect(lastItems()).toHaveLength(4);
    expect(lastItems().every((i) => i.timed?.[0]?.kind === 'burst' && i.anim === 'idle' && i.mount!.speed === 0)).toBe(true);
    expect(toolbar(h).querySelector('.wd-gap-chip')!.getAttribute('title')).toContain('Wirk-Animation fehlt');
  });

  it('▶ bei meteor im Panel springt auf „Skill“ und spielt meteor mit Bewegung „fällt“', async () => {
    const h = await mount();
    await click(h, 'Skills 26');
    await click(h, 'Skill meteor abspielen');
    expect(selectedText(h, 'Skill')).toBe('Zauberer · meteor');
    const fx = lastItems()[0]!.timed![0]!;
    expect(fx.kind === 'burst' && fx.look.motion).toBe('fall');
    expect(h.querySelector('[data-skill="meteor"]')!.getAttribute('aria-selected')).toBe('true');
  });
});
