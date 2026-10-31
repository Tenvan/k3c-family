// @vitest-environment happy-dom
import { act, useState } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, describe, expect, it } from 'vitest';
import { Splitter, splitKeyDelta } from './Splitter';

(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

let root: Root | null = null;
afterEach(() => {
  act(() => root?.unmount());
  document.body.innerHTML = '';
});

function Probe({ start, horizontal }: { start: number; horizontal?: boolean }) {
  const [v, setV] = useState(start);
  return <Splitter horizontal={horizontal} label="Höhe der Bühne" value={v} onChange={setV} onReset={() => setV(40)} />;
}

function mount(start: number, horizontal?: boolean): HTMLElement {
  const host = document.createElement('div');
  document.body.append(host);
  root = createRoot(host);
  act(() => root!.render(<Probe start={start} horizontal={horizontal} />));
  return host.querySelector<HTMLElement>('[role="separator"]')!;
}

const press = (el: HTMLElement, key: string) => act(() => { el.dispatchEvent(new KeyboardEvent('keydown', { key, bubbles: true })); });

describe('Splitter (Tastatur)', () => {
  it('Pfeiltasten je Richtung, andere Tasten ändern nichts', () => {
    expect([splitKeyDelta(true, 'ArrowUp'), splitKeyDelta(true, 'ArrowDown'), splitKeyDelta(true, 'ArrowLeft')]).toEqual([2, -2, 0]);
    expect([splitKeyDelta(false, 'ArrowRight'), splitKeyDelta(false, 'ArrowLeft'), splitKeyDelta(false, 'ArrowUp')]).toEqual([2, -2, 0]);
  });

  it('ist fokussierbar, benannt und ↑/↓ ändern den Anteil des unteren Bereichs in den Grenzen', () => {
    const s = mount(40, true);
    expect(s.tabIndex).toBe(0);
    expect(s.getAttribute('aria-label')).toBe('Höhe der Bühne');
    expect(s.getAttribute('aria-orientation')).toBe('horizontal');
    press(s, 'ArrowUp');
    expect(s.getAttribute('aria-valuenow')).toBe('42');
    press(s, 'ArrowDown');
    press(s, 'ArrowDown');
    expect(s.getAttribute('aria-valuenow')).toBe('38');
    for (let i = 0; i < 30; i++) press(s, 'ArrowDown');
    expect(s.getAttribute('aria-valuenow')).toBe('20');
    press(s, 'Home');
    expect(s.getAttribute('aria-valuenow')).toBe('40');
  });

  it('senkrecht: → vergrößert, ← verkleinert, höchstens 80', () => {
    const s = mount(78);
    press(s, 'ArrowRight');
    press(s, 'ArrowRight');
    expect(s.getAttribute('aria-valuenow')).toBe('80');
    press(s, 'ArrowLeft');
    expect(s.getAttribute('aria-valuenow')).toBe('78');
  });
});
