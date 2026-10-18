// @vitest-environment happy-dom
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, describe, expect, it, vi } from 'vitest';
import { RangeSlider, thumbBox } from './RangeSlider';

// B-485/AC-05: Bereichs-Schieber für die Weltposition; die Spur ist die ganze Stufe, der Griff der sichtbare Bereich.
(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

let root: Root | null = null;
let host: HTMLElement | null = null;

type Props = Parameters<typeof RangeSlider>[0];
const base: Props = { label: 'Position', widthUnits: 900, center: 450, range: [50, 850], main: { left: 400, units: 100 }, onCenter: () => {} };

async function mount(props: Partial<Props> = {}) {
  host = document.createElement('div');
  document.body.append(host);
  root = createRoot(host);
  await act(async () => root!.render(<RangeSlider {...base} {...props} />));
  const track = host.querySelector<HTMLElement>('.wd-range')!;
  track.getBoundingClientRect = () => ({ left: 0, width: 1000, top: 0, height: 20, right: 1000, bottom: 20, x: 0, y: 0, toJSON: () => ({}) });
  return host;
}
const pointer = (target: EventTarget, type: string, clientX: number) => act(async () => {
  target.dispatchEvent(Object.assign(new MouseEvent(type, { bubbles: true, clientX }), { pointerId: 1 }));
});
const thumb = (h: HTMLElement) => h.querySelector<HTMLElement>('[role="slider"]')!;

afterEach(() => {
  act(() => root?.unmount());
  host?.remove();
});

describe('Griff = sichtbarer Bereich (B-485/AC-05)', () => {
  it('Breite und Lage des Griffs sind Sichtbereich und linke Kante relativ zur Stufe', () => {
    expect(thumbBox({ left: 400, units: 100 }, 900)).toEqual({ left: (400 / 900) * 100, width: (100 / 900) * 100 });
  });

  it('Grenzen: breiter als die Stufe = ganze Spur, nie außerhalb, leere Stufe = ganze Spur', () => {
    expect(thumbBox({ left: 0, units: 2000 }, 900)).toEqual({ left: 0, width: 100 });
    expect(thumbBox({ left: 880, units: 100 }, 900)).toEqual({ left: 100 - (100 / 900) * 100, width: (100 / 900) * 100 });
    expect(thumbBox({ left: 0, units: 100 }, 0)).toEqual({ left: 0, width: 100 });
  });

  it('der Griff ist ein Schieber mit Wert, Grenzen und Text; ein zweiter Griff zeigt die Live-Vorschau', async () => {
    const h = await mount({ second: { left: 425, units: 50 } });
    const t = thumb(h);
    expect(t.getAttribute('aria-label')).toBe('Position');
    expect(t.getAttribute('aria-valuenow')).toBe('450');
    expect(t.getAttribute('aria-valuemin')).toBe('50');
    expect(t.getAttribute('aria-valuemax')).toBe('850');
    expect(parseFloat(t.style.width)).toBeCloseTo((100 / 900) * 100);
    const live = h.querySelector<HTMLElement>('.wd-range-second')!;
    expect(parseFloat(live.style.width)).toBeCloseTo((50 / 900) * 100);
    expect(parseFloat(live.style.left)).toBeCloseTo((425 / 900) * 100);
  });

  it('ohne zweiten Bereich gibt es keinen zweiten Griff', async () => {
    const h = await mount();
    expect(h.querySelector('.wd-range-second')).toBeNull();
  });
});

describe('Bedienung des Schiebers (B-485/AC-05)', () => {
  it('Ziehen des Griffs verschiebt die Mitte um die Welt-Strecke unter dem Zeiger', async () => {
    const onCenter = vi.fn();
    const h = await mount({ onCenter });
    await pointer(thumb(h), 'pointerdown', 450);
    await pointer(window, 'pointermove', 550); // 100 px von 1000 = 90 Units von 900
    expect(onCenter).toHaveBeenLastCalledWith(540);
    await pointer(window, 'pointerup', 550);
    onCenter.mockClear();
    await pointer(window, 'pointermove', 700);
    expect(onCenter).not.toHaveBeenCalled(); // nach dem Loslassen kein Ziehen mehr
  });

  it('ein abgebrochenes Ziehen (pointercancel, Aushängen) hinterlässt keine Verschiebung', async () => {
    const onCenter = vi.fn();
    const h = await mount({ onCenter });
    await pointer(thumb(h), 'pointerdown', 450);
    await pointer(window, 'pointercancel', 450);
    await pointer(window, 'pointermove', 700);
    expect(onCenter).not.toHaveBeenCalled();
    await pointer(thumb(h), 'pointerdown', 450);
    act(() => root?.unmount());
    root = null;
    await pointer(window, 'pointermove', 700);
    expect(onCenter).not.toHaveBeenCalled();
  });

  it('Klick auf die Spur springt dorthin', async () => {
    const onCenter = vi.fn();
    const h = await mount({ onCenter });
    await pointer(h.querySelector('.wd-range')!, 'pointerdown', 100);
    expect(onCenter).toHaveBeenLastCalledWith(90);
  });

  it('Pfeile verschieben um ein Zehntel des Sichtbereichs, Pos1 und Ende an die Grenzen', async () => {
    const onCenter = vi.fn();
    const h = await mount({ onCenter });
    const key = (k: string) => act(async () => { thumb(h).dispatchEvent(new KeyboardEvent('keydown', { key: k, bubbles: true })); });
    const page = vi.fn();
    document.addEventListener('keydown', page);
    await key('ArrowRight');
    document.removeEventListener('keydown', page);
    expect(onCenter).toHaveBeenLastCalledWith(460);
    expect(page).not.toHaveBeenCalled(); // die Tasten der Seite verschieben nicht ein zweites Mal
    await key('ArrowLeft');
    expect(onCenter).toHaveBeenLastCalledWith(440);
    await key('Home');
    expect(onCenter).toHaveBeenLastCalledWith(50);
    await key('End');
    expect(onCenter).toHaveBeenLastCalledWith(850);
  });
});
