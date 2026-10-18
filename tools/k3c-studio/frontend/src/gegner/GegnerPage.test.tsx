// @vitest-environment happy-dom
import { Theme } from '@radix-ui/themes';
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { backend } from '../api';
import type { AssetSlot } from '../api';
import type { FigureItem } from '../abspiel/figureView';
import { BIOMES } from '@game/model/biome';
import { GegnerPage } from './GegnerPage';

// B-464 (WZ11.1) im DOM: Gegner-Seite mit einer Attrappe statt Phaser; die Bühne bekommt, was die Seite wählt.
(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

const calls = vi.hoisted(() => ({ shown: [] as unknown[], controls: [] as unknown[] }));
vi.mock('../stage/PreviewStage', () => ({
  PreviewStage: class {
    async showFigures(items: unknown) { calls.shown.push(items); return (items as unknown[]).map(() => ({ gap: null, frames: 4 })); }
    setControl(c: unknown) { calls.controls.push(c); }
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
  await act(async () => root!.render(<Theme><GegnerPage /></Theme>));
  await flush();
  return host;
}

const field = (h: HTMLElement, label: string) => h.querySelector<HTMLSelectElement>(`[aria-label="${label}"]`)!;
async function choose(el: HTMLSelectElement, value: string) {
  await act(async () => { el.value = value; el.dispatchEvent(new Event('change', { bubbles: true })); });
  await flush();
}

beforeEach(() => { calls.shown.length = 0; calls.controls.length = 0; });
afterEach(() => { vi.restoreAllMocks(); act(() => root?.unmount()); host?.remove(); });

describe('Gegner-Seite (B-464/AC-01 bis AC-06, AC-13)', () => {
  it('zeigt die Gruppen je Biom nach Tiefe und als erste den Wald mit den Gegnern von Portal und Nacht', async () => {
    const h = await mount();
    const group = field(h, 'Gruppe');
    expect([...group.options].map((o) => o.textContent)).toEqual([...BIOMES].sort((a, b) => a.depth - b.depth).map((b) => b.name).concat('ohne Biom', 'Bosse (je Tiefe)'));
    expect(lastItems().map((i) => i.id)).toEqual(['enemy-greed', 'enemy-goblin', 'enemy-goblinArcher', 'enemy-wolf']);
    expect([...h.querySelectorAll('.wd-card')].map((c) => (c as HTMLElement).dataset.enemy)).toEqual(['greed', 'goblin', 'goblinArcher', 'wolf']);
  });

  it('Gruppe „ohne Biom“ zeigt alphaWolf; Animation, Getroffen und Sterben gehen an die Bühne', async () => {
    const h = await mount();
    await choose(field(h, 'Gruppe'), 'none');
    expect(lastItems().some((i) => i.id === 'enemy-alphaWolf')).toBe(true);
    await choose(field(h, 'Animation'), 'attack');
    expect(lastItems().every((i) => i.anim === 'attack')).toBe(true);
    await choose(field(h, 'Aktion'), 'kill');
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
    const pick = field(h, 'Grafik greed');
    expect(pick.value).toBe('0');
    await choose(pick, '1');
    expect(lastItems()[0]!.spec.sheet).toBe('hell-hound');
    expect(set).not.toHaveBeenCalled();
  });

  it('Gruppe „Bosse“: 6 Bosse mit Lücken der Anzeigen, Rückfall-Grafik und Fähigkeit', async () => {
    const h = await mount();
    await choose(field(h, 'Gruppe'), 'bosse');
    expect(lastItems().map((i) => i.id)).toEqual(['goblinLeader', 'trollKing', 'ratQueen', 'lavaGolem', 'shardTitan', 'crystalHeartWarden'].map((id) => `enemy-${id}`));
    const goblin = h.querySelector('[data-enemy="goblinLeader"]')!;
    expect(goblin.textContent).toContain('Boss-Leiste');
    expect(goblin.textContent).toContain('Event-Banner');
    expect(goblin.textContent).not.toContain('Warnkreis');
    expect(goblin.textContent).toContain('keine eigene Boss-Grafik');
    const warden = h.querySelector('[data-enemy="crystalHeartWarden"]')!;
    for (const what of ['Warnkreis', 'Boss-Leiste', 'Phasen-Text', 'Event-Banner']) expect(warden.textContent).toContain(what);
    expect(warden.textContent).toContain('Endboss');
    expect(h.querySelectorAll('.wd-boss .wd-gap')).toHaveLength(15);
  });
});
