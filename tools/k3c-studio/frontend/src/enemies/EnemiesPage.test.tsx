// @vitest-environment happy-dom
import { Theme } from '@radix-ui/themes';
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { backend } from '../api';
import type { AssetSlot } from '../api';
import type { FigureItem } from '../playback/figureView';
import { BIOMES } from '@game/model/biome';
import { optionTexts } from '../../../../webkit/testing/options';
import { pickOption, selectedText } from '../../../../webkit/testing/select';
import { EnemiesPage } from './EnemiesPage';

// B-464 (WZ11.1) im DOM: Gegner-Seite mit einer Attrappe statt Phaser; die Bühne bekommt, was die Seite wählt.
(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

const calls = vi.hoisted(() => ({ shown: [] as unknown[], controls: [] as unknown[], kampf: [] as unknown[] }));
vi.mock('../stage/PreviewStage', () => ({
  PreviewStage: class {
    async showFigures(items: unknown) { calls.shown.push(items); return (items as unknown[]).map(() => ({ gap: null, frames: 4 })); }
    setControl(c: unknown) { calls.controls.push(c); }
    setKampf(w: unknown) { calls.kampf.push(w); }
    destroy() {}
  },
}));

let root: Root | null = null;
let host: HTMLElement | null = null;
const flush = () => act(async () => { for (let i = 0; i < 5; i++) await new Promise((r) => setTimeout(r, 0)); });
const lastItems = () => calls.shown.at(-1) as FigureItem[];

async function mount(): Promise<HTMLElement> {
  host = document.createElement('div');
  document.body.append(host);
  root = createRoot(host);
  await act(async () => root!.render(<Theme><EnemiesPage /></Theme>));
  await flush();
  return host;
}

/** Eintrag mit diesem Text in der Auswahl wählen */
async function choose(h: HTMLElement, label: string, text: string) {
  await pickOption(h, label, text);
  await flush();
}

beforeEach(() => { calls.shown.length = 0; calls.controls.length = 0; });
afterEach(() => { vi.restoreAllMocks(); act(() => root?.unmount()); host?.remove(); });

describe('Gegner-Seite (B-464/AC-01 bis AC-06, AC-13)', () => {
  it('zeigt die Gruppen je Biom nach Tiefe und als erste den Wald mit den Gegnern von Portal und Nacht', async () => {
    const h = await mount();
    expect(await optionTexts(h, 'Gruppe')).toEqual([...BIOMES].sort((a, b) => a.depth - b.depth).map((b) => b.name).concat('ohne Biom', 'Bosse (je Tiefe)'));
    expect(lastItems().map((i) => i.id)).toEqual(['enemy-greed', 'enemy-goblin', 'enemy-goblinArcher', 'enemy-wolf']);
    expect([...h.querySelectorAll('.wd-card')].map((c) => (c as HTMLElement).dataset.enemy)).toEqual(['greed', 'goblin', 'goblinArcher', 'wolf']);
  });

  it('Gruppe „ohne Biom“ zeigt alphaWolf; Animation, Getroffen und Sterben gehen an die Bühne', async () => {
    const h = await mount();
    await choose(h, 'Gruppe', 'ohne Biom');
    expect(lastItems().some((i) => i.id === 'enemy-alphaWolf')).toBe(true);
    await choose(h, 'Animation', 'attack');
    expect(lastItems().every((i) => i.anim === 'attack')).toBe(true);
    await choose(h, 'Aktion', 'Sterben');
    expect(lastItems()[0]!.effect).toEqual({ kind: 'kill' });
    expect(h.querySelector('.wd-bar')!.textContent).toContain('Sterbe-Animation');
  });

  it('Monarch-Vergleich ist zuschaltbar: Monarch beritten links neben den Gegnern', async () => {
    const h = await mount();
    const check = h.querySelector<HTMLElement>('[aria-label="Monarch-Vergleich"]')!;
    await act(async () => check.click());
    await flush();
    expect(lastItems()[0]).toMatchObject({ id: 'monarch-vergleich', mount: { key: 'horse' } });
    expect(lastItems()).toHaveLength(5);
  });

  it('Merkmale stehen an der Karte, ohne Darstellung mit Lücke', async () => {
    const h = await mount();
    const greed = h.querySelector('[data-enemy="greed"]')!;
    expect(greed.querySelector('[data-trait="stealsGold"] .wd-gap')!.textContent).toContain('ohne Darstellung');
  });

  it('Grafik-Auswahl je Gegner-ID: Präferenz 1 voreingestellt, eine andere Zuordnung wechselt das Sheet, das Register bleibt unverändert', async () => {
    const slots: AssetSlot[] = [{ id: 'greed', art: 'grafik', kategorie: 'figuren', beschreibung: 'greed', status: 'zugeordnet', wahl: [{ datei: 'sprites/goblin/idle.png', sheet: 'goblin' }, { datei: 'sprites/hell-hound/idle.png', sheet: 'hell-hound' }] }];
    vi.spyOn(backend, 'assetsList').mockResolvedValue({ slots, error: '', preview: '/', stand: 's' });
    const set = vi.spyOn(backend, 'assetsSet');
    const h = await mount();
    expect(selectedText(h, 'Grafik greed')).toBe('Präferenz 1: goblin');
    await choose(h, 'Grafik greed', 'Präferenz 2: hell-hound');
    expect(lastItems()[0]!.spec.sheet).toBe('hell-hound');
    expect(set).not.toHaveBeenCalled();
  });

  it('Gruppe „Bosse“: 6 Bosse mit Rückfall-Grafik und Fähigkeit; nur das Event-Banner bleibt Lücke (B-469)', async () => {
    const h = await mount();
    await choose(h, 'Gruppe', 'Bosse (je Tiefe)');
    expect(lastItems().map((i) => i.id)).toEqual(['goblinLeader', 'trollKing', 'ratQueen', 'lavaGolem', 'shardTitan', 'crystalHeartWarden'].map((id) => `enemy-${id}`));
    const goblin = h.querySelector('[data-enemy="goblinLeader"]')!;
    expect(goblin.textContent).toContain('Event-Banner');
    for (const what of ['Boss-Leiste', 'Warnkreis', 'Phasen-Text']) expect(goblin.textContent).not.toContain(what);
    expect(goblin.textContent).toContain('keine eigene Boss-Grafik');
    const warden = h.querySelector('[data-enemy="crystalHeartWarden"]')!;
    expect(warden.textContent).toContain('Endboss');
    expect(warden.textContent).toContain('Event-Banner');
    expect(h.querySelectorAll('.wd-boss .wd-gap')).toHaveLength(6);
  });

});

describe('Boss-Anzeigen des Spiels (B-469)', () => {
  it('Boss-Anzeigen des Spiels (B-469): Beispielzustand geht an die Bühne, Phase und HP wählbar, nur in der Gruppe der Bosse', async () => {
    const h = await mount();
    expect(calls.kampf.at(-1)).toBeNull(); // andere Gruppe: keine Boss-Anzeige
    await choose(h, 'Gruppe', 'Bosse (je Tiefe)');
    const world = () => calls.kampf.at(-1) as { enemies: { kind: string; hp: number; maxHp: number; phase?: number; warn?: { r: number } }[] };
    expect(world().enemies[0]).toMatchObject({ kind: 'goblinLeader', hp: 1000, maxHp: 1000 });
    expect(world().enemies[0]!.warn).toBeUndefined(); // kein Flächenschlag
    await choose(h, 'Boss-Anzeige', 'trollKing');
    expect(world().enemies[0]!.warn).toMatchObject({ r: 3 }); // Flächenschlag: Warnkreis mit dem Radius der Daten
    await choose(h, 'Boss-Anzeige', 'crystalHeartWarden');
    expect(world().enemies[0]).toMatchObject({ phase: 1 });
    expect(world().enemies[0]!.warn).toBeUndefined(); // Phase 1 ruft nur Spinnen
    await choose(h, 'Phase', 'Phase 2');
    expect(world().enemies[0]).toMatchObject({ phase: 2, hp: 550 });
    expect(world().enemies[0]!.warn).toMatchObject({ r: 5 });
    expect(h.querySelector('[aria-label="Phasen-Schwelle"]')!.textContent).toContain('unter 60 % HP');
    await choose(h, 'Phase', 'Phase 3');
    expect(world().enemies[0]!.warn).toMatchObject({ r: 5 }); // Phase 3 erbt den Flächenschlag
    await choose(h, 'Gruppe', BIOMES[0]!.name);
    expect(calls.kampf.at(-1)).toBeNull();
  });
});

describe('Gegner-Seite: Zusatzfelder (B-464/AC-06)', () => {
  it('Zusatzfelder stehen an der Karte (Höhlentroll: aoe, Rattenschwarm: swarmSize) mit Lücke', async () => {
    const h = await mount();
    await choose(h, 'Gruppe', BIOMES.find((b) => b.id === 'cave')!.name);
    expect(h.querySelector('[data-enemy="caveTroll"] [data-extra="aoe"]')!.textContent).toContain('Radius 3');
    expect(h.querySelector('[data-enemy="caveTroll"] [data-extra="aoe"] .wd-gap')).not.toBeNull();
    await choose(h, 'Gruppe', BIOMES.find((b) => b.id === 'mine')!.name);
    expect(h.querySelector('[data-extra="swarmSize"]')).not.toBeNull();
  });
});
