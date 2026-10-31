// @vitest-environment happy-dom
import { act, createElement } from 'react';
import { createRoot } from 'react-dom/client';
import { afterEach, describe, expect, it, vi } from 'vitest';
import { FacetRail } from '@webkit/organisms';
import { confirmText } from './candidateLogic';
import { usePageKeys } from './usePageKeys';
import { chipFromView, DEFAULT_VIEW, uniqueChip } from './viewState';

// Befunde des Reviews zu WZ33 (WZ33.7): jeder behobene Fehler bekommt einen Test.
(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

const unmounts: (() => void)[] = [];
/** Hängt den Hook an eine Wegwerf-Komponente (Testing Library gibt es im Studio nicht). */
function renderHook(hook: () => void): void {
  const root = createRoot(document.createElement('div'));
  act(() => root.render(createElement(() => { hook(); return null; })));
  unmounts.push(() => act(() => root.unmount()));
}

afterEach(() => {
  unmounts.splice(0).forEach((fn) => fn());
  document.body.innerHTML = '';
});

describe('Eigene Chips: ID-Kollision', () => {
  it('ein anderer Name mit derselben ID überschreibt den vorhandenen Chip nicht', () => {
    const foo = chipFromView('Foo!', DEFAULT_VIEW);
    const bar = chipFromView('foo?', DEFAULT_VIEW);
    expect(bar.id).toBe(foo.id);
    expect(uniqueChip(bar, [foo]).id).toBe(`${foo.id}-2`);
    expect(uniqueChip(foo, [foo]).id).toBe(foo.id); // gleicher Name ersetzt den Chip
  });
});

describe('Bestätigung in der Zeile', () => {
  it('nennt eine unbekannte Größe statt „–“ (Recherche-Treffer)', () => {
    expect(confirmText({ groesse: 0, lizenz: 'CC0', marke: 'frei' }, 'tavern')).toBe('Größe unbekannt · CC0 · in tavern');
  });
});

describe('Tastatur der Seite', () => {
  const press = (target: Element, code: string) => {
    const e = new KeyboardEvent('keydown', { code, bubbles: true, cancelable: true });
    target.dispatchEvent(e);
    return e;
  };

  it('Pfeiltasten wählen den Slot, außer in Dialogen und Auswahllisten (eigene Tastatur)', () => {
    const onMove = vi.fn();
    renderHook(() => usePageKeys({ onMove, onPlay: vi.fn() }));
    press(document.body, 'ArrowDown');
    expect(onMove).toHaveBeenCalledWith(1);
    const dialog = document.createElement('div');
    dialog.setAttribute('role', 'dialog');
    dialog.innerHTML = '<span id="in"></span>';
    document.body.append(dialog);
    onMove.mockClear();
    press(dialog.querySelector('#in')!, 'ArrowDown');
    expect(onMove).not.toHaveBeenCalled();
  });

  it('die Liste der Asset-Slots behält ↑↓ trotz role="listbox"', () => {
    const onMove = vi.fn();
    renderHook(() => usePageKeys({ onMove, onPlay: vi.fn() }));
    const list = document.createElement('div');
    list.setAttribute('role', 'listbox');
    list.setAttribute('aria-label', 'Asset-Slots');
    list.innerHTML = '<span id="row"></span>';
    document.body.append(list);
    press(list.querySelector('#row')!, 'ArrowUp');
    expect(onMove).toHaveBeenCalledWith(-1);
  });

  it('eine schon behandelte Taste (defaultPrevented) löst nichts aus', () => {
    const onMove = vi.fn();
    renderHook(() => usePageKeys({ onMove, onPlay: vi.fn() }));
    const block = (e: Event) => e.preventDefault();
    document.body.addEventListener('keydown', block);
    press(document.body, 'ArrowDown');
    document.body.removeEventListener('keydown', block);
    expect(onMove).not.toHaveBeenCalled();
  });
});

describe('Facetten: lange Listen (echte Daten, 2026-10-30)', () => {
  const options = Array.from({ length: 12 }, (_, i) => ({ value: `q${i}`, label: `Quelle ${i}`, count: 12 - i }));
  const mount = (selected: string[]) => {
    const host = document.createElement('div');
    document.body.append(host);
    const root = createRoot(host);
    act(() => root.render(createElement(FacetRail, { facets: [{ id: 'quelle', title: 'Quelle', options }], selected: { quelle: selected }, onToggle: vi.fn(), onClear: vi.fn() })));
    unmounts.push(() => act(() => root.unmount()));
    return host;
  };
  const labels = (h: HTMLElement) => [...h.querySelectorAll('button[role="checkbox"]')].length;

  it('zeigt nur die ersten acht Optionen und klappt den Rest auf', () => {
    const h = mount([]);
    expect(labels(h)).toBe(8);
    const more = [...h.querySelectorAll('button')].find((b) => b.textContent === '4 weitere anzeigen')!;
    act(() => more.click());
    expect(labels(h)).toBe(12);
    expect([...h.querySelectorAll('button')].some((b) => b.textContent === 'weniger anzeigen')).toBe(true);
  });

  it('eine gewählte Option bleibt auch hinter dem Limit sichtbar', () => {
    const h = mount(['q11']);
    expect(labels(h)).toBe(9);
    expect(h.textContent).toContain('Quelle 11 (1)');
  });
});
