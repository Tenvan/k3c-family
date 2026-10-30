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

const calls = vi.hoisted(() => ({ shown: [] as unknown[], controls: [] as unknown[], kampf: [] as unknown[], click: null as ((i: number | null) => void) | null, marks: [] as (number | null)[] }));
vi.mock('../stage/PreviewStage', () => ({
  PreviewStage: class {
    async showFigures(items: unknown) { calls.shown.push(items); return (items as unknown[]).map(() => ({ gap: null, frames: 4 })); }
    setControl(c: unknown) { calls.controls.push(c); }
    setKampf(w: unknown) { calls.kampf.push(w); }
    setFigureClick(f: (i: number | null) => void) { calls.click = f; }
    setFigureMark(i: number | null) { calls.marks.push(i); }
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

const button = (h: HTMLElement, label: string) =>
  [...h.querySelectorAll('button')].find((b) => b.getAttribute('aria-label') === label || b.textContent === label) as HTMLButtonElement | undefined;
const click = async (h: HTMLElement, label: string) => { await act(async () => button(h, label)!.click()); await flush(); };
const rows = (h: HTMLElement) => [...h.querySelectorAll<HTMLElement>('.wd-enemy-table tbody tr')];
const inspector = (h: HTMLElement) => h.querySelector<HTMLElement>('[aria-label="Inspektor"]')!;

beforeEach(() => { calls.shown.length = 0; calls.controls.length = 0; calls.marks.length = 0; });
afterEach(() => { vi.restoreAllMocks(); act(() => root?.unmount()); host?.remove(); });

describe('Gegner-Seite (B-464/AC-01 bis AC-06, AC-13)', () => {
  it('zeigt die Gruppen je Biom nach Tiefe und als erste den Wald mit den Gegnern von Portal und Nacht', async () => {
    const h = await mount();
    expect(await optionTexts(h, 'Gruppe')).toEqual([...BIOMES].sort((a, b) => a.depth - b.depth).map((b) => b.name).concat('ohne Biom', 'Bosse (je Tiefe)'));
    expect(lastItems().map((i) => i.id)).toEqual(['enemy-greed', 'enemy-goblin', 'enemy-goblinArcher', 'enemy-wolf']);
    expect(rows(h).map((r) => r.dataset.enemy)).toEqual(['greed', 'goblin', 'goblinArcher', 'wolf']);
  });

  it('Gruppe „ohne Biom“ zeigt alphaWolf; Animation, Getroffen und Sterben als ein Segment gehen an die Bühne', async () => {
    const h = await mount();
    expect([...h.querySelectorAll('[aria-label="Modus"] button')].map((b) => b.textContent)).toEqual(['idle', 'run', 'attack', 'Getroffen', 'Sterben']);
    await choose(h, 'Gruppe', 'ohne Biom');
    expect(lastItems().some((i) => i.id === 'enemy-alphaWolf')).toBe(true);
    await click(h, 'attack');
    expect(lastItems().every((i) => i.anim === 'attack')).toBe(true);
    await click(h, 'Sterben');
    expect(lastItems()[0]!.effect).toEqual({ kind: 'kill' });
    expect(h.querySelector('[role="toolbar"] .wd-gap-chip')!.getAttribute('title')).toContain('Sterbe-Animation');
  });

  it('Monarch-Vergleich ist zuschaltbar: Monarch beritten links neben den Gegnern', async () => {
    const h = await mount();
    await click(h, 'Monarch-Vergleich');
    expect(lastItems()[0]).toMatchObject({ id: 'monarch-vergleich', mount: { key: 'horse' } });
    expect(lastItems()).toHaveLength(5);
  });

  it('Merkmale stehen im Inspektor, ohne Darstellung mit einer Lückenzeile', async () => {
    const h = await mount();
    expect(inspector(h).dataset.enemy).toBe('greed');
    expect(inspector(h).querySelector('[data-trait="stealsGold"] .wd-gap-line')!.textContent).toContain('ohne Darstellung');
  });

  it('Grafik-Auswahl je Gegner-ID: Präferenz 1 voreingestellt, eine andere Zuordnung wechselt das Sheet, das Register bleibt unverändert', async () => {
    const slots: AssetSlot[] = [{ id: 'greed', art: 'grafik', kategorie: 'figuren', beschreibung: 'greed', status: 'zugeordnet', wahl: [{ datei: 'sprites/goblin/idle.png', sheet: 'goblin' }, { datei: 'sprites/hell-hound/idle.png', sheet: 'hell-hound' }] }];
    vi.spyOn(backend, 'assetsList').mockResolvedValue({ slots, error: '', preview: '/', stand: 's' });
    const set = vi.spyOn(backend, 'assetsSet');
    const h = await mount(); // greed ist als erster gewählt, der Inspektor zeigt seine Grafik-Auswahl
    expect(selectedText(h, 'Grafik greed')).toBe('Präferenz 1: goblin');
    await choose(h, 'Grafik greed', 'Präferenz 2: hell-hound');
    expect(lastItems()[0]!.spec.sheet).toBe('hell-hound');
    expect(set).not.toHaveBeenCalled();
  });

  it('Gruppe „Bosse“: 6 Bosse mit Art, Rückfall-Grafik und Fähigkeit; nur das Event-Banner bleibt Lücke (B-469, B-672/AC-04)', async () => {
    const h = await mount();
    await choose(h, 'Gruppe', 'Bosse (je Tiefe)');
    expect(lastItems().map((i) => i.id)).toEqual(['goblinLeader', 'trollKing', 'ratQueen', 'lavaGolem', 'shardTitan', 'crystalHeartWarden'].map((id) => `enemy-${id}`));
    expect([...h.querySelectorAll('.wd-enemy-table th')].map((t) => t.textContent)).toContain('Art');
    expect(rows(h).map((r) => r.children[1]!.textContent)).toEqual(['Mini', 'Mini', 'Mini', 'Mini', 'Mini', 'End']);
    const goblin = inspector(h);
    expect(goblin.dataset.enemy).toBe('goblinLeader');
    expect(goblin.textContent).toContain('Event-Banner');
    for (const what of ['Boss-Leiste', 'Warnkreis', 'Phasen-Text']) expect(goblin.textContent).not.toContain(what);
    expect(goblin.textContent).toContain('keine eigene Boss-Grafik');
    expect(goblin.textContent).toContain('ruft 2× goblin alle 10 s');
    await act(async () => rows(h)[5]!.click());
    await flush();
    const warden = inspector(h);
    expect(warden.textContent).toContain('Endboss');
    expect(warden.textContent).toContain('Event-Banner');
    expect(warden.querySelectorAll('[aria-label="Phasen"] li')).toHaveLength(3);
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
  const pickRow = async (h: HTMLElement, id: string) => { await act(async () => h.querySelector<HTMLElement>(`.wd-enemy-table tr[data-enemy="${id}"]`)!.click()); await flush(); };
  it('Zusatzfelder stehen im Inspektor (Höhlentroll: aoe, Rattenschwarm: swarmSize) mit Lückenzeile', async () => {
    const h = await mount();
    await choose(h, 'Gruppe', BIOMES.find((b) => b.id === 'cave')!.name);
    await pickRow(h, 'caveTroll');
    expect(inspector(h).querySelector('[data-extra="aoe"]')!.textContent).toContain('Radius 3');
    expect(inspector(h).querySelector('[data-extra="aoe"] .wd-gap-line')).not.toBeNull();
    await choose(h, 'Gruppe', BIOMES.find((b) => b.id === 'mine')!.name);
    await pickRow(h, 'ratSwarm');
    expect(inspector(h).querySelector('[data-extra="swarmSize"]')).not.toBeNull();
  });
});

describe('Tabelle, Inspektor und Auswahl (B-672)', () => {
  it('Lücken stehen als Zahl in der Tabelle und je als eine Zeile im Inspektor, lange Badges gibt es nicht (AC-02)', async () => {
    const h = await mount();
    const archer = h.querySelector<HTMLElement>('.wd-enemy-table tr[data-enemy="goblinArcher"]')!;
    expect(archer.querySelector('[data-gaps]')!.textContent).toBe('4');
    expect(archer.textContent).toContain('Goblin Archer');
    expect(archer.textContent).toContain('10'); // Reichweite
    await act(async () => archer.click());
    await flush();
    expect(inspector(h).querySelectorAll('.wd-gap-line')).toHaveLength(4);
    expect(h.querySelectorAll('.wd-gap')).toHaveLength(0);
    expect(h.querySelector('[role="toolbar"] .wd-gap-chip')).not.toBeNull();
  });

  it('Klick auf Zeile oder Figur wählt denselben Gegner und markiert ihn, auch mit Monarch-Vergleich (AC-03)', async () => {
    const h = await mount();
    await act(async () => h.querySelector<HTMLElement>('tr[data-enemy="wolf"]')!.click());
    await flush();
    expect(calls.marks.at(-1)).toBe(3);
    act(() => calls.click!(2)); // Figur 2 = Goblin Archer
    await flush();
    expect(inspector(h).dataset.enemy).toBe('goblinArcher');
    expect(h.querySelector('tr[data-enemy="goblinArcher"]')!.getAttribute('aria-selected')).toBe('true');
    await click(h, 'Monarch-Vergleich');
    expect(calls.marks.at(-1)).toBe(3); // mit Monarch davor rückt der Archer eine Figur weiter
    act(() => calls.click!(1)); // Figur 1 = Greed (Figur 0 ist der Monarch)
    await flush();
    expect(inspector(h).dataset.enemy).toBe('greed');
    act(() => calls.click!(0)); // der Monarch ist kein Gegner: die Auswahl bleibt
    await flush();
    expect(inspector(h).dataset.enemy).toBe('greed');
  });

});

describe('Angriffe und Fähigkeiten abspielen (B-672)', () => {
  it('Angriff: Fernkämpfer mit Geschoss bis zur Reichweite, mit Monarch-Vergleich auf ihn zu; Nahkämpfer mit Treffer (AC-07)', async () => {
    const h = await mount();
    await click(h, 'attack');
    const archer = lastItems().find((i) => i.id === 'enemy-goblinArcher')!;
    expect(archer.timed!.find((t) => t.kind === 'projectile')).toMatchObject({ range: 10, dir: 1 });
    const goblin = lastItems().find((i) => i.id === 'enemy-goblin')!;
    expect(goblin.timed!.map((t) => t.kind)).toEqual(['burst']);
    await click(h, 'Monarch-Vergleich');
    expect(lastItems().find((i) => i.id === 'enemy-goblinArcher')!.timed!.find((t) => t.kind === 'projectile')).toMatchObject({ dir: -1 });
  });

  it('Bosse: ▶ einer Fähigkeit spielt sie mit Radius und Intervall der Daten; summon zeigt die Gegner neben dem Boss (AC-06)', async () => {
    const h = await mount();
    await choose(h, 'Gruppe', 'Bosse (je Tiefe)');
    await act(async () => h.querySelector<HTMLElement>('tr[data-enemy="trollKing"]')!.click());
    await flush();
    await click(h, 'Troll-König · aoe abspielen');
    expect(selectedText(h, 'Fähigkeit')).toBe('Troll-König · aoe');
    expect(lastItems().find((i) => i.id === 'enemy-trollKing')!.timed).toEqual([{ kind: 'warn', radius: 3, cycleMs: 4000 }]);
    const world = calls.kampf.at(-1) as { enemies: { warn?: unknown }[] };
    expect(world.enemies.every((e) => e.warn === undefined)).toBe(true); // kein zweiter, stehender Warnkreis
    await choose(h, 'Fähigkeit', 'Goblin-Anführer · summon');
    const summons = lastItems().filter((i) => i.id.startsWith('summon-'));
    expect(summons).toHaveLength(2);
    expect(summons.every((s) => s.spec.sheet === 'goblin' && s.timed?.[0]?.kind === 'appear')).toBe(true);
    expect(inspector(h).dataset.enemy).toBe('goblinLeader');
    await choose(h, 'Fähigkeit', 'Lava-Golem · flameTrail');
    expect(h.querySelector('[role="toolbar"] .wd-gap-chip')!.getAttribute('title')).toContain('flameTrail');
    await choose(h, 'Gruppe', BIOMES[0]!.name);
    expect([...h.querySelectorAll('[aria-label="Modus"] button')].some((b) => b.textContent === 'Fähigkeit …')).toBe(false);
    expect(lastItems().some((i) => i.timed)).toBe(false);
  });
});
