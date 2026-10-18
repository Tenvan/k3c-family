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

interface Shown { shown: unknown[]; views: { left: number; scale: number }[]; destroyed: boolean; fail: (files: string[]) => void; pan: (px: number) => void }
const stages: Shown[] = [];
vi.mock('./PreviewStage', () => ({
  PreviewStage: class {
    state: Shown;
    constructor(_parent: unknown, fail: (files: string[]) => void, pan: (px: number) => void) { this.state = { shown: [], views: [], destroyed: false, fail, pan }; stages.push(this.state); }
    show(world: unknown) { this.state.shown.push(world); }
    setPosition(view: { left: number; scale: number }) { this.state.views.push(view); }
    setLook() {}
    destroy() { this.state.destroyed = true; }
  },
}));

const ARGS = { seed: 'k3c', source: { biome: 'forest' }, hubLevel: 5, phase: 'day', progress: 0, buildings: {} };
type World = { sites: unknown[]; biome: { id: string }; seed: string };

let root: Root | null = null;
let host: HTMLElement | null = null;
const flush = () => act(async () => { for (let i = 0; i < 5; i++) await new Promise((r) => setTimeout(r, 0)); });
const lastWorld = (stage = 0) => stages[stage]!.shown.at(-1) as World;
const VIEW = { left: 40, scale: 12 };
const element = (seed: string, view = VIEW, onPan: (px: number) => void = () => {}) => <Theme><LivePreview seed={seed} biome="forest" position={view} onPan={onPan} onWheel={() => {}} /></Theme>;

async function mount(seed = 'k3c', onPan: (px: number) => void = () => {}): Promise<HTMLElement> {
  host = document.createElement('div');
  document.body.append(host);
  root = createRoot(host);
  await act(async () => root!.render(element(seed, VIEW, onPan)));
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

  it('Position und Zoom kommen von außen und gehen an die Bühne; eine neue Ansicht wird nachgezogen (B-477/AC-01)', async () => {
    await mount();
    expect(stages[0]!.views.at(-1)).toEqual(VIEW);
    await act(async () => root!.render(element('k3c', { left: 100, scale: 24 })));
    expect(stages[0]!.views.at(-1)).toEqual({ left: 100, scale: 24 });
  });

  it('Ziehen und Mausrad der Bühne gehen als Pixel an die Seite (B-477/AC-01)', async () => {
    const pans: number[] = [];
    await mount('k3c', (px) => pans.push(px));
    stages[0]!.pan(-120);
    expect(pans).toEqual([-120]);
  });

  it('meldet die Bauplätze für die Sprungpunkte', async () => {
    const seen: unknown[][] = [];
    host = document.createElement('div');
    document.body.append(host);
    root = createRoot(host);
    await act(async () => root!.render(<Theme><LivePreview seed="k3c" biome="forest" position={VIEW} onPan={() => {}} onWheel={() => {}} onSites={(s) => seen.push([...s])} /></Theme>));
    await flush();
    expect(seen.at(-1)).toHaveLength(lastWorld().sites.length);
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
    await act(async () => root!.render(element('zwei')));
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
