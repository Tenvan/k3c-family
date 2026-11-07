// @vitest-environment happy-dom
import { Theme } from '@radix-ui/themes';
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, describe, expect, it, vi } from 'vitest';
import { FramePlayer, counterWidth } from './FramePlayer';

// WB2/AC-01, AC-05: Abspielen, Schieber und Zähler haben feste Breiten; wechselnde Zahlen und Knopftexte verschieben nichts.

(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

let root: Root | null = null;
afterEach(() => {
  act(() => root?.unmount());
  root = null;
  document.body.innerHTML = '';
});

function mount(node: React.ReactNode): HTMLElement {
  const host = document.createElement('div');
  document.body.append(host);
  root = createRoot(host);
  act(() => root!.render(<Theme>{node}</Theme>));
  return host;
}

const rerender = (node: React.ReactNode) => act(() => root!.render(<Theme>{node}</Theme>));

/** Breiten-Stile der drei Teile: Knopf, Schieber-Hülle, Zähler. */
function widths(host: HTMLElement) {
  const play = host.querySelector<HTMLElement>('[data-part="play"]')!;
  const slider = host.querySelector<HTMLElement>('[data-part="slider"]')!;
  const count = host.querySelector<HTMLElement>('[data-part="count"]')!;
  return { play: play.style.width, slider: slider.style.width, count: count.style.width, nums: count.style.fontVariantNumeric };
}

describe('FramePlayer', () => {
  it('feste Breiten über alle Frames und beide Knopftexte', () => {
    const props = { frames: 12, onFrame: vi.fn(), onPlaying: vi.fn() };
    const host = mount(<FramePlayer {...props} frame={0} playing={false} />);
    const first = widths(host);
    expect(first.play).not.toBe('');
    expect(first.slider).toBe('240px');
    expect(first.count).not.toBe('');
    expect(first.nums).toBe('tabular-nums');
    rerender(<FramePlayer {...props} frame={9} playing />);
    expect(widths(host)).toEqual(first);
    expect(host.querySelector('[data-part="count"]')!.textContent).toBe('Frame 10/12');
    expect(host.querySelector('[data-part="play"]')!.textContent).toBe('Anhalten');
  });

  it('Zählerbreite reicht für die längste Anzeige', () => {
    expect(counterWidth(9, 'Frame')).toBe(`${'Frame 9/9'.length}ch`);
    expect(counterWidth(12, 'Frame')).toBe(`${'Frame 12/12'.length}ch`);
    expect(counterWidth(120, '')).toBe(`${'120/120'.length}ch`);
  });

  it('Knopf schaltet Abspielen, bei einem Frame gesperrt', () => {
    const onPlaying = vi.fn();
    const host = mount(<FramePlayer frames={4} frame={0} playing={false} onFrame={vi.fn()} onPlaying={onPlaying} />);
    act(() => host.querySelector<HTMLButtonElement>('[data-part="play"]')!.click());
    expect(onPlaying).toHaveBeenCalledWith(true);
    rerender(<FramePlayer frames={1} frame={0} playing={false} onFrame={vi.fn()} onPlaying={onPlaying} />);
    expect(host.querySelector<HTMLButtonElement>('[data-part="play"]')!.disabled).toBe(true);
  });
});
