// @vitest-environment happy-dom
import { act } from 'react';
import { createRoot } from 'react-dom/client';
import { afterEach, describe, expect, it } from 'vitest';
import { SplitStack } from './SplitStack';

(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

/** Ablage im Speicher statt localStorage, Schlüssel ohne App-Präfix. */
function memoryStore(init: Record<string, string> = {}) {
  const data = { ...init };
  return { data, loadText: (k: string, fb: string) => data[k] ?? fb, savePref: (k: string, v: string) => { data[k] = v; } };
}

const cleanups: Array<() => void> = [];
afterEach(() => cleanups.splice(0).forEach((f) => f()));

function mount(store: ReturnType<typeof memoryStore>) {
  const host = document.createElement('div');
  document.body.appendChild(host);
  const root = createRoot(host);
  act(() => root.render(<SplitStack store={store} prefKey="mcp.split.log" dflt={45} top={<p>oben</p>} bottom={<p>unten</p>} />));
  cleanups.push(() => {
    act(() => root.unmount());
    host.remove();
  });
  const stack = host.querySelector('.wk-stack') as HTMLElement;
  const bar = host.querySelector('[role=separator]') as HTMLElement;
  return { stack, bar, share: () => stack.style.getPropertyValue('--wk-stack-bottom') };
}

describe('SplitStack (B-875)', () => {
  it('ohne gemerkten Wert gilt die Vorgabe, der Splitter ist waagerecht', () => {
    const { share, bar } = mount(memoryStore());
    expect(share()).toBe('45');
    expect(bar.getAttribute('aria-orientation')).toBe('horizontal');
  });

  it('liest den gemerkten Anteil und klemmt ihn auf 20–80 %', () => {
    expect(mount(memoryStore({ 'mcp.split.log': '62' })).share()).toBe('62');
    expect(mount(memoryStore({ 'mcp.split.log': '95' })).share()).toBe('80');
    expect(mount(memoryStore({ 'mcp.split.log': 'kaputt' })).share()).toBe('45');
  });

  it('Pfeiltaste ändert den Anteil und merkt ihn, Pos1 setzt auf die Vorgabe zurück', () => {
    const store = memoryStore({ 'mcp.split.log': '50' });
    const { bar, share } = mount(store);
    act(() => { bar.dispatchEvent(new KeyboardEvent('keydown', { key: 'ArrowUp', bubbles: true })); });
    expect(share()).toBe('52');
    expect(store.data['mcp.split.log']).toBe('52');
    act(() => { bar.dispatchEvent(new KeyboardEvent('keydown', { key: 'Home', bubbles: true })); });
    expect(store.data['mcp.split.log']).toBe('45');
  });
});
