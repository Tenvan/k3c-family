// @vitest-environment happy-dom
import { Theme } from '@radix-ui/themes';
import { act, useState } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { DEFAULT_CONTROL, gap, type Control } from './figure';
import { FigureStage } from './FigureStage';
import type { FigureItem } from './figureView';
import { GapBadge, PlaybackControls } from './PlaybackControls';

// B-463 (WZ10.1) im DOM: Steuerung, Lücken-Marke und die Bühne mit einer Attrappe statt Phaser.
(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

const calls = vi.hoisted(() => ({ shown: [] as unknown[], controls: [] as unknown[], destroyed: 0, info: [{ gap: null, frames: 6 }] as unknown[] }));
vi.mock('../stage/PreviewStage', () => ({
  PreviewStage: class {
    async showFigures(items: unknown) { calls.shown.push(items); return items ? calls.info : []; }
    setControl(c: unknown) { calls.controls.push(c); }
    destroy() { calls.destroyed++; }
  },
}));

let root: Root | null = null;
let host: HTMLElement | null = null;
const flush = () => act(async () => { for (let i = 0; i < 4; i++) await new Promise((r) => setTimeout(r, 0)); });

async function mount(node: React.ReactNode): Promise<HTMLElement> {
  host = document.createElement('div');
  document.body.append(host);
  root = createRoot(host);
  await act(async () => root!.render(<Theme>{node}</Theme>));
  await flush();
  return host;
}

const button = (h: HTMLElement, label: string) => [...h.querySelectorAll('button')].find((b) => b.textContent === label || b.getAttribute('aria-label') === label)!;

beforeEach(() => { calls.shown.length = 0; calls.controls.length = 0; calls.destroyed = 0; });
afterEach(() => { act(() => root?.unmount()); host?.remove(); });

function Controlled({ frames = 6, onChange }: { frames?: number; onChange?: (c: Control) => void }) {
  const [control, setControl] = useState<Control>(DEFAULT_CONTROL);
  return <PlaybackControls control={control} frames={frames} onChange={(c) => { setControl(c); onChange?.(c); }} />;
}

describe('Steuerung (B-463/AC-01)', () => {
  it('Schleife an und aus', async () => {
    const h = await mount(<Controlled />);
    expect(button(h, 'Schleife').getAttribute('aria-pressed')).toBe('true');
    await act(async () => button(h, 'Schleife').click());
    expect(button(h, 'Schleife').getAttribute('aria-pressed')).toBe('false');
  });

  it('Einzelbild vor und zurück im Kreis, Weiter nimmt die Wiedergabe wieder auf', async () => {
    const seen: Control[] = [];
    const h = await mount(<Controlled onChange={(c) => seen.push(c)} />);
    await act(async () => button(h, 'Bild vor').click());
    await act(async () => button(h, 'Bild zurück').click());
    await act(async () => button(h, 'Bild zurück').click());
    expect(seen.map((c) => c.frame)).toEqual([1, 0, 5]);
    expect(h.textContent).toContain('Bild 6 von 6');
    await act(async () => button(h, 'Weiter').click());
    expect(seen.at(-1)!.frame).toBeNull();
  });

  it('Tempo 0,5×, 1× und 2×', async () => {
    const h = await mount(<Controlled />);
    const select = h.querySelector<HTMLSelectElement>('[aria-label="Tempo"]')!;
    expect([...select.options].map((o) => o.textContent)).toEqual(['0.5×', '1×', '2×']);
    await act(async () => { select.value = '2'; select.dispatchEvent(new Event('change', { bubbles: true })); });
    expect(select.value).toBe('2');
  });
});

describe('Lücken-Marke (B-463)', () => {
  it('zeigt „Lücke“ mit dem Grund, ohne Lücke nichts', async () => {
    const h = await mount(<><GapBadge gap={gap('anim', 'Lücke: run fehlt')} /><GapBadge gap={null} /></>);
    const badges = h.querySelectorAll('.wd-gap');
    expect(badges).toHaveLength(1);
    expect(badges[0]!.textContent).toBe('Lücke: run fehlt');
    expect(badges[0]!.getAttribute('title')).toBe('Lücke: run fehlt');
  });
});

describe('Figuren-Bühne (B-463): eine Phaser-Instanz je Seite', () => {
  const item = (id: string): FigureItem => ({ id, x: 0, spec: { sheet: 'medieval-king' }, anim: 'idle' });

  it('zeigt die Figuren, reicht die Steuerung durch und meldet Lücke und Bildzahl', async () => {
    const items = [item('a'), item('b')];
    const onInfo = vi.fn();
    await mount(<FigureStage items={items} control={DEFAULT_CONTROL} onInfo={onInfo} />);
    expect(calls.shown).toEqual([items]);
    expect(calls.controls.at(-1)).toBe(DEFAULT_CONTROL);
    expect(onInfo).toHaveBeenCalledWith(calls.info);
  });

  it('neue Figuren ersetzen die alten, eine neue Steuerung gilt sofort, Abbau zerstört die Bühne', async () => {
    function Page({ n, control }: { n: number; control: Control }) {
      const [items] = useState(() => [item('x')]);
      return <FigureStage items={n > 0 ? items : null} control={control} />;
    }
    await mount(<Page n={1} control={DEFAULT_CONTROL} />);
    const faster = { ...DEFAULT_CONTROL, tempo: 2 };
    await act(async () => root!.render(<Theme><Page n={1} control={faster} /></Theme>));
    await flush();
    expect(calls.controls.at(-1)).toEqual(faster);
    await act(async () => root!.render(<Theme><Page n={0} control={faster} /></Theme>));
    await flush();
    expect(calls.shown.at(-1)).toBeNull();
    act(() => root!.unmount());
    root = null;
    expect(calls.destroyed).toBe(1);
  });
});
