// @vitest-environment happy-dom
import { Theme } from '@radix-ui/themes';
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { backend } from '../api';
import { mockWorldPreview } from '../api/mockWorld';
import { LivePreview } from './LivePreview';

// B-460/AC-04 im DOM: die Live-Vorschau fragt die Vorschau-Welt ab, zeigt Hub-Stufen aus levels und Gebäude-Schalter und
// reicht die Welt an die Bühne. Die Phaser-Bühne ersetzt hier eine Attrappe (ein Canvas gibt es im Test nicht).
(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

interface Shown { shown: unknown[]; destroyed: boolean; fail: (files: string[]) => void }
const stages: Shown[] = [];
vi.mock('./PreviewStage', () => ({
  PreviewStage: class {
    state: Shown;
    constructor(_parent: unknown, fail: (files: string[]) => void) { this.state = { shown: [], destroyed: false, fail }; stages.push(this.state); }
    show(world: unknown) { this.state.shown.push(world); }
    refresh() {}
    destroy() { this.state.destroyed = true; }
  },
}));

const ARGS = { seed: 'k3c', source: { biome: 'forest' }, hubLevel: 5, phase: 'day', progress: 0, buildings: {} };
type World = { sites: unknown[]; biome: { id: string }; seed: string };

let root: Root | null = null;
let host: HTMLElement | null = null;
const flush = () => act(async () => { for (let i = 0; i < 5; i++) await new Promise((r) => setTimeout(r, 0)); });
const lastWorld = (stage = 0) => stages[stage]!.shown.at(-1) as World;
const element = (seed: string, active: boolean) => <Theme><LivePreview seed={seed} biome="forest" active={active} /></Theme>;

async function mount(active = true, seed = 'k3c'): Promise<HTMLElement> {
  host = document.createElement('div');
  document.body.append(host);
  root = createRoot(host);
  await act(async () => root!.render(element(seed, active)));
  await flush();
  return host;
}

const boxes = (h: HTMLElement) => [...h.querySelectorAll<HTMLElement>('.wd-buildings button[role="checkbox"]')];

beforeEach(() => { stages.length = 0; });
afterEach(() => {
  vi.restoreAllMocks();
  act(() => root?.unmount());
  host?.remove();
});

describe('Live-Vorschau (B-460/AC-04)', () => {
  it('fragt die Welt zu Seed, Biom und der höchsten Hub-Stufe ab und zeichnet sie auf der Bühne', async () => {
    const spy = vi.spyOn(backend, 'worldPreview');
    const h = await mount();
    expect(spy).toHaveBeenCalledWith(expect.objectContaining({ seed: 'k3c', source: { biome: 'forest' }, hubLevel: 5, buildings: {} }));
    expect(stages).toHaveLength(1);
    expect(lastWorld().biome.id).toBe('forest');
    expect(boxes(h)).toHaveLength(lastWorld().sites.length);
  });

  it('eine eingeklappte Vorschau (nicht aktiv) baut keine Bühne auf', async () => {
    await mount(false);
    expect(stages).toHaveLength(0);
  });

  it('Gebäude abwählen: neue Anfrage mit dem Schlüssel, der Platz fehlt in der Welt, der Schalter bleibt', async () => {
    const spy = vi.spyOn(backend, 'worldPreview');
    const h = await mount();
    const count = boxes(h).length;
    const key = boxes(h)[0]!.getAttribute('aria-label')!;
    await act(async () => boxes(h)[0]!.click());
    await flush();
    expect(spy).toHaveBeenLastCalledWith(expect.objectContaining({ buildings: { [key]: false } }));
    expect(lastWorld().sites).toHaveLength(count - 1);
    expect(boxes(h)).toHaveLength(count);
    expect(boxes(h)[0]!.getAttribute('aria-checked')).toBe('false');
    await act(async () => boxes(h)[0]!.click());
    await flush();
    expect(lastWorld().sites).toHaveLength(count);
  });

});

describe('Live-Vorschau: Wechsel, Fehler, Lebenszyklus (B-460/AC-04)', () => {
  it('Seed wechseln bei abgewählten Gebäuden: Auswahl beginnt neu, die Bühne zeigt das neue Level mit allen Gebäuden', async () => {
    const h = await mount();
    const count = boxes(h).length;
    await act(async () => boxes(h)[0]!.click());
    await flush();
    await act(async () => root!.render(element('zwei', true)));
    await flush();
    expect(lastWorld().seed).toBe('zwei');
    expect(lastWorld().sites).toHaveLength(count);
    expect(boxes(h).every((b) => b.getAttribute('aria-checked') === 'true')).toBe(true);
  });

  it('gleiche Eingaben → gleiche Welt auf der Bühne (zweimal gemountet)', async () => {
    await mount();
    const first = JSON.stringify(lastWorld());
    await act(async () => root!.unmount());
    root = null;
    host!.remove();
    await mount();
    expect(JSON.stringify(lastWorld(1))).toBe(first);
    expect(JSON.stringify(lastWorld(1).sites)).toBe(JSON.stringify(mockWorldPreview(ARGS).state!.sites));
  });

  it('ein Hinweis des Servers (Zustand null) erscheint, die Seite bleibt heil, die Bühne bleibt leer', async () => {
    vi.spyOn(backend, 'worldPreview').mockResolvedValue(mockWorldPreview({ ...ARGS, seed: 'fehler' }));
    const h = await mount();
    expect(h.textContent).toContain('Seed braucht');
    expect(boxes(h)).toHaveLength(0);
    expect(stages[0]!.shown.at(-1)).toBeNull();
  });

  it('nicht geladene Grafiken werden gemeldet', async () => {
    const h = await mount();
    await act(async () => stages[0]!.fail(['grafik/gebaeude/farm.png']));
    expect(h.textContent).toContain('Grafiken nicht geladen: grafik/gebaeude/farm.png');
  });

  it('beim Verlassen wird die Bühne abgebaut', async () => {
    await mount();
    await act(async () => root!.unmount());
    root = null;
    expect(stages[0]!.destroyed).toBe(true);
  });
});
