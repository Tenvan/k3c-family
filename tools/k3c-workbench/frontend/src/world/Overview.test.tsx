// @vitest-environment happy-dom
import { Theme } from '@radix-ui/themes';
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, describe, expect, it, vi } from 'vitest';
import { backend, type WorldDetail, type WorldIsland, type WorldLevel } from '../api';
import { bandText, etappeOf, groupEtappen, marksOf } from './crossModel';
import { levelModel } from './levelView';
import { Overview } from './Overview';

// B-963/AC-01: Der Querschnitt zeigt alle Inseln und Stufen nach Etappen gruppiert; ein Klick auf die Seekarte springt zur Insel.

(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

const island = (id: string, depths: number[], etappe?: number): WorldIsland =>
  ({ id, name: `Insel ${id}`, depths, biomes: depths.map((d) => (d === 0 ? 'forest' : 'cave')), plannedChunks: {}, chunkCounts: {}, ...(etappe ? { etappe } : {}) });

const DETAIL: WorldDetail = {
  id: 'sengoku', name: 'Sengoku', notes: '', extends: '', error: '',
  islands: [island('a', [0, 4], 1), island('b', [0, 1, 4], 2), island('c', [0, 3, 4], 2)],
};

const level = (width: number): WorldLevel => ({
  seed: 'k3c', biomeId: 'forest', widthUnits: width, chunkWidthUnits: 50, hubCenterUnits: width / 2,
  chunks: [{ index: 0, kind: 'edge', startUnits: 0 }, { index: 1, kind: 'hub', startUnits: 50 }, { index: 2, kind: 'harbor', startUnits: 100 }],
  entities: [{ kind: 'portal', x: 20 }, { kind: 'exit', x: width - 5 }], planned: [], warnings: [], error: '',
});

let root: Root | null = null;
afterEach(() => { act(() => root?.unmount()); root = null; document.body.innerHTML = ''; vi.restoreAllMocks(); });

async function mount(onPick = vi.fn()): Promise<HTMLElement> {
  vi.spyOn(backend, 'worldLevel').mockImplementation(async (_w, _i, stage) => level(1000 - stage * 200));
  const host = document.createElement('div');
  document.body.appendChild(host);
  root = createRoot(host);
  act(() => root!.render(<Theme><Overview detail={DETAIL} seed="k3c" choice={null} onPick={onPick} /></Theme>));
  for (let i = 0; i < 5; i++) await act(async () => new Promise((r) => setTimeout(r, 0)));
  return host;
}

describe('Querschnitt (B-963/AC-01)', () => {
  it('Etappe: Feld oder Listenplatz + 1, gruppiert und sortiert', () => {
    expect(etappeOf(island('x', [0]), 4)).toBe(5);
    expect(etappeOf(island('x', [0], 2), 0)).toBe(2);
    expect(groupEtappen(DETAIL).map((g) => [g.etappe, g.islands.map((i) => i.island.id)])).toEqual([[1, ['a']], [2, ['b', 'c']]]);
    const gs = groupEtappen(DETAIL);
    expect(bandText(gs[0]!, undefined)).toBe('Etappe 1 · 1 Insel · 🔓 ab Spielstart');
    expect(bandText(gs[1]!, gs[0])).toBe('Etappe 2 · 2 Inseln · frei wählbar · 🔒 nach Etappe 1');
  });

  it('Marker: Burg, Portal, Hafen, Treppe und Endboss in der tiefsten Stufe', () => {
    const m = levelModel(level(1000));
    expect(marksOf(m, false).map((k) => k.kind).sort()).toEqual(['castle', 'harbor', 'portal', 'stairs']);
    expect(marksOf(m, true).map((k) => k.kind)).toContain('boss');
  });

  it('rendert alle Inseln und Stufen nach Etappen gruppiert', async () => {
    const host = await mount();
    const cross = host.querySelector('[aria-label="Querschnitt"]')!;
    expect([...cross.querySelectorAll('.wd-x-band')].map((b) => b.textContent)).toEqual([
      'Etappe 1 · 1 Insel · 🔓 ab Spielstart', 'Etappe 2 · 2 Inseln · frei wählbar · 🔒 nach Etappe 1']);
    expect([...cross.querySelectorAll('[data-island]')].map((e) => e.getAttribute('data-island'))).toEqual(['a', 'b', 'c']);
    expect([...cross.querySelectorAll('[data-island]')].map((e) => e.querySelectorAll('.wd-x-bar').length)).toEqual([2, 3, 3]);
    expect(cross.querySelectorAll('.wd-x-mark').length).toBeGreaterThan(0);
  });

  it('Streifen sind proportional zur Länge', async () => {
    const host = await mount();
    const bars = [...host.querySelectorAll('[data-island="b"] .wd-x-bar')] as HTMLElement[];
    expect(bars.map((b) => b.style.width)).toEqual(['100%', '80%', '60%']);
  });

  it('Seekarte: Klick springt zur Insel und hebt sie hervor', async () => {
    const host = await mount();
    const target = host.querySelector('#insel-c') as HTMLElement;
    const scroll = vi.fn();
    target.scrollIntoView = scroll;
    const nodes = [...host.querySelectorAll('[aria-label="Seekarte"] [data-jump]')];
    expect(nodes.map((n) => n.getAttribute('data-jump'))).toEqual(['a', 'b', 'c']);
    await act(async () => (nodes[2] as HTMLElement).click());
    expect(scroll).toHaveBeenCalledOnce();
    expect(target.className).toContain('wd-x-hit');
    expect(host.querySelector('#insel-a')!.className).not.toContain('wd-x-hit');
  });

  it('ein Klick auf einen Streifen wählt die Stufe zum Bearbeiten', async () => {
    const onPick = vi.fn();
    const host = await mount(onPick);
    await act(async () => (host.querySelectorAll('[data-island="b"] .wd-x-bar')[1] as HTMLElement).click());
    expect(onPick).toHaveBeenCalledWith('b', 1);
  });
});
