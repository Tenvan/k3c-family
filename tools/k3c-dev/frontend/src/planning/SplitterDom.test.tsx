// @vitest-environment happy-dom
import { act } from 'react';
import { createRoot } from 'react-dom/client';
import { afterEach, describe, expect, it, vi } from 'vitest';
import { Splitter } from '@webkit/atoms';

vi.mock('../api', () => ({ backend: {} }));

(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

const BOX = { left: 100, top: 0, right: 900, bottom: 400, width: 800, height: 400, x: 100, y: 0 };

type Mount = { bar: HTMLElement; onChange: ReturnType<typeof vi.fn>; onReset: ReturnType<typeof vi.fn> };
const mounted: Array<() => void> = [];

function mount(opts: { horizontal?: boolean; box?: boolean } = {}): Mount {
  const host = document.createElement('div');
  document.body.appendChild(host);
  const onChange = vi.fn();
  const onReset = vi.fn();
  const root = createRoot(host);
  act(() => root.render(<div id="parent"><Splitter value={50} horizontal={opts.horizontal} onChange={onChange} onReset={onReset} /></div>));
  const parent = host.querySelector('#parent') as HTMLElement;
  if (opts.box !== false) parent.getBoundingClientRect = () => BOX as DOMRect;
  mounted.push(() => {
    act(() => root.unmount());
    host.remove();
  });
  return { bar: host.querySelector('[role=separator]') as HTMLElement, onChange, onReset };
}

const fire = (target: EventTarget, type: string, init: Record<string, number> = {}) =>
  act(() => { target.dispatchEvent(new MouseEvent(type, { bubbles: true, cancelable: true, ...init })); });

afterEach(() => mounted.splice(0).forEach((undo) => undo()));

describe('Splitter in DOM-Umgebung (B-406)', () => {
  it('AC-01: Doppelklick ruft onReset einmal, onChange nie', () => {
    const { bar, onChange, onReset } = mount();
    fire(bar, 'dblclick');
    expect(onReset).toHaveBeenCalledTimes(1);
    expect(onChange).not.toHaveBeenCalled();
  });

  it('AC-01: Ziehen senkrecht setzt den Anteil von links, geklemmt auf 20–80 %', () => {
    const { bar, onChange } = mount();
    fire(bar, 'pointerdown');
    fire(window, 'pointermove', { clientX: 300, clientY: 0 });
    fire(window, 'pointermove', { clientX: 1000, clientY: 0 });
    fire(window, 'pointermove', { clientX: 100, clientY: 0 });
    expect(onChange.mock.calls.map((c) => c[0])).toEqual([25, 80, 20]);
  });

  it('AC-01: Ziehen waagerecht misst von der unteren Kante', () => {
    const { bar, onChange } = mount({ horizontal: true });
    fire(bar, 'pointerdown');
    fire(window, 'pointermove', { clientX: 0, clientY: 200 });
    expect(onChange).toHaveBeenCalledWith(50);
  });

  it('AC-01: nach pointerup ändert Bewegen nichts mehr', () => {
    const { bar, onChange } = mount();
    fire(bar, 'pointerdown');
    fire(window, 'pointermove', { clientX: 300, clientY: 0 });
    fire(window, 'pointerup');
    fire(window, 'pointermove', { clientX: 500, clientY: 0 });
    expect(onChange).toHaveBeenCalledTimes(1);
  });

  it('AC-01: ohne messbare Box tut pointerdown nichts', () => {
    const { bar, onChange } = mount({ box: false });
    bar.parentElement!.getBoundingClientRect = () => undefined as unknown as DOMRect;
    fire(bar, 'pointerdown');
    fire(window, 'pointermove', { clientX: 300, clientY: 0 });
    expect(onChange).not.toHaveBeenCalled();
  });
});
