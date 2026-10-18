// @vitest-environment happy-dom
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, describe, expect, it } from 'vitest';
import { DEFAULT_SCALE, MAX_SCALE } from '../world/viewState';
import { usePlanerView } from './usePlanerView';

(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

let root: Root | null = null;
afterEach(() => { act(() => root?.unmount()); root = null; });

function mount() {
  let hook!: ReturnType<typeof usePlanerView>;
  const Probe = () => { hook = usePlanerView(); return null; };
  act(() => { root = createRoot(document.createElement('div')); root.render(<Probe />); });
  return () => hook;
}

describe('usePlanerView', () => {
  it('Ziehen verschiebt die linke Kante, nie unter 0', () => {
    const get = mount();
    act(() => get().onPan(DEFAULT_SCALE * 10));
    expect(get().position.left).toBe(10);
    act(() => get().onPan(-DEFAULT_SCALE * 50));
    expect(get().position.left).toBe(0);
  });

  it('Strg + Rad zoomt, der Maßstab bleibt in den Grenzen', () => {
    const get = mount();
    const prevented: boolean[] = [];
    const wheel = (deltaY: number) => {
      const e = { deltaX: 0, deltaY, ctrlKey: true, shiftKey: false, preventDefault: () => prevented.push(true) } as unknown as WheelEvent;
      act(() => get().onWheel(e));
    };
    wheel(-100);
    expect(prevented).toHaveLength(1);
    expect(get().position.scale).toBeGreaterThan(DEFAULT_SCALE);
    for (let i = 0; i < 60; i++) wheel(-100);
    expect(get().position.scale).toBeLessThanOrEqual(MAX_SCALE);
  });
});
