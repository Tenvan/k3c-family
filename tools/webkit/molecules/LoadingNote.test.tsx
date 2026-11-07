// @vitest-environment happy-dom
import { Theme } from '@radix-ui/themes';
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, describe, expect, it } from 'vitest';
import { LoadingNote } from './LoadingNote';

// WB2/AC-03: Lade-Anzeige als Zeile oder als Überlage über einer Grafik (absolut, verschiebt nichts).

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

describe('LoadingNote', () => {
  it('Zeile mit Text und Status-Rolle', () => {
    const host = mount(<LoadingNote id="list" text="Lädt Assets …" />);
    const note = host.querySelector<HTMLElement>('[data-loading="list"]')!;
    expect(note.getAttribute('role')).toBe('status');
    expect(note.textContent).toBe('Lädt Assets …');
    expect(note.className).not.toContain('wk-loading-overlay');
  });

  it('Überlage: absolut positioniert, Text nur für Screenreader', () => {
    const host = mount(<LoadingNote id="frame" text="Lädt Frame" overlay />);
    const note = host.querySelector<HTMLElement>('[data-loading="frame"]')!;
    expect(note.className).toContain('wk-loading-overlay');
    expect(note.getAttribute('aria-label')).toBe('Lädt Frame');
  });
});
