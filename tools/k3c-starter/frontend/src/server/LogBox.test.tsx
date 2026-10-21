// @vitest-environment happy-dom
import { act } from 'react';
import { createRoot } from 'react-dom/client';
import { describe, expect, it } from 'vitest';
import { LogBox } from './LogBox';

(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

/** happy-dom rechnet kein Layout: Maße und Lesestelle setzt der Test selbst. */
function mount() {
  const host = document.createElement('div');
  document.body.append(host);
  const root = createRoot(host);
  const show = (lines: string[]) => act(async () => root.render(<LogBox lines={lines} />));
  const box = () => host.querySelector('pre')!;
  const layout = (el: HTMLElement, scrollHeight: number) => {
    Object.defineProperty(el, 'scrollHeight', { value: scrollHeight, configurable: true });
    Object.defineProperty(el, 'clientHeight', { value: 100, configurable: true });
  };
  return { show, box, layout };
}

describe('Log-Box', () => {
  it('rutscht mit neuen Zeilen mit, solange sie am Ende stand', async () => {
    const { show, box, layout } = mount();
    await show(['eins']);
    layout(box(), 500);
    await show(['eins', 'zwei']);
    expect(box().scrollTop).toBe(500);
  });

  it('bleibt stehen, wenn nach oben gescrollt wurde', async () => {
    const { show, box, layout } = mount();
    await show(['eins']);
    layout(box(), 500);
    box().scrollTop = 40; // Lesestelle weit vom Ende
    await act(async () => void box().dispatchEvent(new Event('scroll')));
    await show(['eins', 'zwei']);
    expect(box().scrollTop).toBe(40);
  });
});
