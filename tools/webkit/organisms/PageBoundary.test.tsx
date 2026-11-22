// @vitest-environment happy-dom
import { Theme } from '@radix-ui/themes';
import { act } from 'react';
import { createRoot } from 'react-dom/client';
import { describe, expect, it, vi } from 'vitest';
import { PageBoundary } from './PageBoundary';

(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

let broken = true;
function Bomb() {
  if (broken) throw new Error('kaputt');
  return <p>heil</p>;
}

describe('PageBoundary', () => {
  it('zeigt den Fehler statt einer leeren Seite und lädt nach dem Klick neu', () => {
    vi.spyOn(console, 'error').mockImplementation(() => undefined);
    const host = document.createElement('div');
    document.body.appendChild(host);
    const root = createRoot(host);
    act(() => root.render(<Theme><PageBoundary name="Spieltest"><Bomb /></PageBoundary></Theme>));
    const box = host.querySelector('[data-page-error]')!;
    expect(box.textContent).toContain('Spieltest');
    expect(box.textContent).toContain('kaputt');
    broken = false;
    act(() => (box.querySelector('button') as HTMLButtonElement).click());
    expect(host.textContent).toContain('heil');
    act(() => root.unmount());
  });
});
