// @vitest-environment happy-dom
import { Theme } from '@radix-ui/themes';
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { LivePreview } from './LivePreview';

// B-690/AC-02 im DOM (Weltmodus, B-666/AC-04, B-667): Klick auf die Bühne läuft über onSite → pickKeys → setSel (Auswahl im
// Inspektor, Rahmen per setMarks) und onPickSite; ein Klick auf einen Chip im Inspektor springt per focusX. Die Bühne ist eine Attrappe.
(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

type Hit = { kind: string; x: number; shift: boolean; double: boolean } | null;
interface Stage { world: { sites: { kind: string; x: number }[] } | null; marks: (readonly number[])[]; jumps: number[]; site: (hit: Hit) => void }
const stages: Stage[] = [];
vi.mock('./PreviewStage', () => ({
  PreviewStage: class {
    state: Stage;
    constructor(_parent: unknown, _fail: unknown, _pan: unknown, _wheel: unknown, site: (hit: Hit) => void) { this.state = { world: null, marks: [], jumps: [], site }; stages.push(this.state); }
    show(world: Stage['world']) { this.state.world = world; }
    setLook() {}
    setPosition() {}
    settle() {}
    destroy() {}
    setMarks(m: { selected: readonly number[] }) { this.state.marks.push(m.selected); }
    focusX(x: number) { this.state.jumps.push(x); }
  },
}));

let root: Root | null = null;
let host: HTMLElement | null = null;
const picked = vi.fn();
const flush = () => act(async () => { for (let i = 0; i < 5; i++) await new Promise((r) => setTimeout(r, 0)); });

async function mount(): Promise<HTMLElement> {
  host = document.createElement('div');
  const tree = document.createElement('div');
  document.body.append(host, tree);
  root = createRoot(host);
  await act(async () => root!.render(<Theme><LivePreview seed="k3c" biome="forest" position={{ left: 40, scale: 12 }} onPan={() => {}} onWheel={() => {}} treeHost={tree} onPickSite={picked} /></Theme>));
  await flush();
  return tree;
}

const stage = () => stages[0]!;
const frame = () => stage().marks.at(-1);
const stageClick = async (hit: Hit) => { await act(async () => stage().site(hit)); await flush(); };
const aria = (tree: HTMLElement, label: string) => tree.querySelector<HTMLButtonElement>(`[aria-label="${label}"]`)!;
const wallChips = (tree: HTMLElement) => [...tree.querySelectorAll<HTMLButtonElement>('.wd-site')];
const walls = () => stage().world!.sites.filter((s) => s.kind === 'wall');

beforeEach(() => { stages.length = 0; picked.mockClear(); });
afterEach(() => {
  vi.restoreAllMocks();
  act(() => root?.unmount());
  host?.remove();
  document.body.innerHTML = '';
});

describe('Weltmodus-Bühne: Auswahl und Rahmen (B-690/AC-02)', () => {
  it('Klick auf einen Platz in der Bühne wählt ihn im Inspektor, setzt den Rahmen ohne Sprung und meldet onPickSite', async () => {
    const tree = await mount();
    const wall = walls()[0]!;
    await stageClick({ kind: 'wall', x: wall.x, shift: false, double: false });
    await act(async () => aria(tree, 'Mauern aufklappen').click());
    expect(wallChips(tree).filter((b) => b.getAttribute('aria-pressed') === 'true').map((b) => b.textContent)).toEqual([String(wall.x)]);
    expect(frame()).toEqual([wall.x]);
    expect(stage().jumps).toEqual([]);
    expect(picked).toHaveBeenCalledTimes(1);
  });

  it('Umschalt-Klick ergänzt, Doppelklick wählt alle der Art, freie Fläche leert die Auswahl ohne onPickSite', async () => {
    await mount();
    const [a, b] = walls();
    await stageClick({ kind: 'wall', x: a!.x, shift: false, double: false });
    await stageClick({ kind: 'wall', x: b!.x, shift: true, double: false });
    expect(frame()).toEqual([a!.x, b!.x]);
    await stageClick({ kind: 'wall', x: a!.x, shift: false, double: true });
    expect([...frame()!].sort((p, q) => p - q)).toEqual(walls().map((w) => w.x).sort((p, q) => p - q));
    picked.mockClear();
    await stageClick(null);
    expect(frame()).toEqual([]);
    expect(picked).not.toHaveBeenCalled();
  });

  it('Klick auf einen Chip im Inspektor wählt den Platz und springt per focusX dorthin', async () => {
    const tree = await mount();
    await act(async () => aria(tree, 'Mauern aufklappen').click());
    const chip = wallChips(tree)[1]!;
    const wall = walls()[1]!;
    await act(async () => chip.click());
    await flush();
    expect(stage().jumps).toEqual([wall.x]);
    expect(frame()).toEqual([wall.x]);
    expect(picked).not.toHaveBeenCalled();
  });
});
