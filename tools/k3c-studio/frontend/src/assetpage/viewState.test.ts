// @vitest-environment happy-dom
import { act, createElement } from 'react';
import { createRoot } from 'react-dom/client';
import { afterEach, beforeEach, describe, expect, it } from 'vitest';
import { BUILTIN_CHIPS, activeChip, facetsOf, noFacets, type CustomChip, type Facets } from './filter';
import { allChips, applyChip, chipFromView, DEFAULT_VIEW, isCustomChip, isViewState, loadView, useCustomChips, useViewState, type ViewState } from './viewState';

// B-677 (AC-02, AC-04): Ansicht und eigene Chips überstehen ein Neuladen; die Suche wird nicht gemerkt.
(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

const VIEW_KEY = 'k3c-studio.assets.view';
const CHIPS_KEY = 'k3c-studio.assets.chips';
const facets = (o: Partial<Facets>): Facets => ({ ...noFacets(), ...o });
const view = (o: Partial<ViewState> = {}): ViewState => ({ ...DEFAULT_VIEW, facets: noFacets(), ...o });
const custom = (o: Partial<CustomChip> = {}): CustomChip => ({ id: 'eigen:test', label: 'Test', facets: { art: ['grafik'] }, sicht: 'dateien', gruppe: 'quelle', query: 'baum', ...o });

const mounted: (() => void)[] = [];
function renderHook<T>(hook: () => T): { result: { current: T }; unmount: () => void } {
  const result = { current: undefined as T };
  const root = createRoot(document.createElement('div'));
  const Probe = () => {
    result.current = hook();
    return null;
  };
  act(() => root.render(createElement(Probe)));
  const unmount = () => act(() => root.unmount());
  mounted.push(unmount);
  return { result, unmount };
}

beforeEach(() => localStorage.clear());
afterEach(() => {
  mounted.splice(0).forEach((fn) => {
    try {
      fn();
    } catch {
      // schon abgebaut
    }
  });
});

describe('Prüfung gemerkter Werte (B-677)', () => {
  it('nimmt eine gültige Ansicht an', () => {
    expect(isViewState(DEFAULT_VIEW)).toBe(true);
    expect(isViewState(view({ sicht: 'szene', gruppe: 'pack', facets: facets({ art: ['sound'] }) }))).toBe(true);
  });

  it.each([
    ['null', null],
    ['ohne Facetten', { query: '', sicht: 'slots', gruppe: 'kategorie' }],
    ['unbekannte Sicht', { ...DEFAULT_VIEW, sicht: 'liste' }],
    ['unbekannte Gruppierung', { ...DEFAULT_VIEW, gruppe: 'farbe' }],
    ['Facette fehlt', { ...DEFAULT_VIEW, facets: { art: [], kategorie: [] } }],
    ['Facette mit Zahl', { ...DEFAULT_VIEW, facets: { ...noFacets(), art: [1] } }],
  ])('weist eine Ansicht zurück: %s', (_name, v) => {
    expect(isViewState(v)).toBe(false);
  });

  it('nimmt einen gültigen eigenen Chip an, auch mit nur einem Teil der Facetten', () => {
    expect(isCustomChip(custom())).toBe(true);
    expect(isCustomChip(custom({ facets: {} }))).toBe(true);
  });

  it.each([
    ['null', null],
    ['ohne Namen', { ...custom(), label: undefined }],
    ['ohne Suche', { ...custom(), query: undefined }],
    ['unbekannte Sicht', { ...custom(), sicht: 'liste' }],
    ['unbekannte Gruppierung', { ...custom(), gruppe: 'farbe' }],
    ['Facette mit Zahl', { ...custom(), facets: { art: [3] } }],
    ['ohne Facetten', { ...custom(), facets: undefined }],
  ])('weist einen eigenen Chip zurück: %s', (_name, c) => {
    expect(isCustomChip(c)).toBe(false);
  });
});

describe('Gemerkte Ansicht (B-677/AC-04)', () => {
  it('beginnt ohne gemerkten Wert mit der Standardansicht', () => {
    expect(loadView()).toEqual(DEFAULT_VIEW);
  });

  it('lädt die gemerkte Ansicht ohne die Suche', () => {
    localStorage.setItem(VIEW_KEY, JSON.stringify(view({ sicht: 'dateien', gruppe: 'pack', query: 'baum', facets: facets({ status: ['luecke'] }) })));
    expect(loadView()).toEqual(view({ sicht: 'dateien', gruppe: 'pack', query: '', facets: facets({ status: ['luecke'] }) }));
  });

  it('fällt bei einem defekten Wert auf die Standardansicht zurück', () => {
    localStorage.setItem(VIEW_KEY, '{kaputt');
    expect(loadView()).toEqual(DEFAULT_VIEW);
    localStorage.setItem(VIEW_KEY, JSON.stringify({ ...DEFAULT_VIEW, sicht: 'liste' }));
    expect(loadView()).toEqual(DEFAULT_VIEW);
  });

  it('merkt jede Änderung außer der Suche und übersteht ein Neuladen', () => {
    const first = renderHook(useViewState);
    act(() => first.result.current[1]({ sicht: 'szene', facets: facets({ art: ['sound'] }), query: 'ton' }));
    expect(first.result.current[0]).toMatchObject({ sicht: 'szene', query: 'ton' });
    first.unmount();
    const second = renderHook(useViewState);
    expect(second.result.current[0]).toEqual(view({ sicht: 'szene', facets: facets({ art: ['sound'] }), query: '' }));
  });
});

describe('Fokus-Chips anwenden (B-677/AC-02)', () => {
  const current = view({ query: 'baum', sicht: 'dateien', gruppe: 'quelle', facets: facets({ art: ['grafik'] }) });
  const builtin = (id: string) => BUILTIN_CHIPS.find((c) => c.id === id)!;

  it('setzt bei einem festen Chip dessen Facetten und lässt Suche, Sicht und Gruppierung', () => {
    const next = applyChip(current, builtin('luecken'));
    expect(next.facets).toEqual(facets({ status: ['luecke', 'fehlt'] }));
    expect(next).toMatchObject({ query: 'baum', sicht: 'dateien', gruppe: 'quelle' });
  });

  it('setzt bei „Alles“ alle Facetten zurück', () => {
    expect(applyChip(current, builtin('alles')).facets).toEqual(noFacets());
  });

  it('setzt bei einem eigenen Chip Facetten, Sicht, Gruppierung und Suche', () => {
    const next = applyChip(view(), custom({ query: 'haus', sicht: 'szene', gruppe: 'pack', facets: { stufe: ['netz'] } }));
    expect(next).toEqual(view({ facets: facets({ stufe: ['netz'] }), sicht: 'szene', gruppe: 'pack', query: 'haus' }));
  });

  it('übernimmt bei einem eigenen Chip auch eine leere Suche', () => {
    expect(applyChip(current, custom({ query: '' })).query).toBe('');
  });

  it('macht den angeklickten Chip wieder zum aktiven', () => {
    const chip = custom({ id: 'eigen:netz-sound', facets: { stufe: ['netz'], art: ['sound'] } });
    const next = applyChip(current, chip);
    expect(activeChip(next.facets, allChips([chip]))).toBe('eigen:netz-sound');
  });
});

describe('Eigener Chip aus der Ansicht (B-677/AC-02)', () => {
  it('merkt Name, Facetten, Sicht, Gruppierung und Suche', () => {
    const chip = chipFromView('Mein Filter', view({ query: 'baum', sicht: 'dateien', gruppe: 'pack', facets: facets({ art: ['grafik'] }) }));
    expect(chip).toMatchObject({ label: 'Mein Filter', sicht: 'dateien', gruppe: 'pack', query: 'baum' });
    expect(chip.facets.art).toEqual(['grafik']);
    expect(isCustomChip(chip)).toBe(true);
  });

  it('bildet die ID aus dem Namen in Kleinbuchstaben ohne Sonderzeichen', () => {
    expect(chipFromView('Grafik Lücke', view()).id).toBe('eigen:grafik-lücke');
    expect(chipFromView('Netz & Sound!', view()).id).toBe('eigen:netz-sound-');
  });

  it('zeigt die festen Chips vor den eigenen', () => {
    const chips = allChips([custom()]);
    expect(chips.slice(0, BUILTIN_CHIPS.length)).toEqual([...BUILTIN_CHIPS]);
    expect(chips.at(-1)?.id).toBe('eigen:test');
  });

  it('wählt bei gleichen Facetten den eigenen Chip vor dem festen (Kommentar activeChip: „eigene vor festen“)', () => {
    const same = custom({ id: 'eigen:grafik', label: 'Grafik 2', facets: { art: ['grafik'] } });
    expect(activeChip(facetsOf(same), allChips([same]))).toBe('eigen:grafik');
  });
});

describe('Eigene Chips merken (B-677/AC-02)', () => {
  it('beginnt ohne gemerkte Chips leer', () => {
    expect(renderHook(useCustomChips).result.current.chips).toEqual([]);
  });

  it('übersteht ein Neuladen nach dem Anlegen', () => {
    const first = renderHook(useCustomChips);
    act(() => first.result.current.add(custom()));
    expect(first.result.current.chips.map((c) => c.id)).toEqual(['eigen:test']);
    first.unmount();
    expect(renderHook(useCustomChips).result.current.chips).toEqual([custom()]);
  });

  it('ersetzt einen Chip mit gleicher ID statt ihn doppelt anzulegen', () => {
    const hook = renderHook(useCustomChips);
    act(() => hook.result.current.add(custom()));
    act(() => hook.result.current.add(custom({ label: 'Umbenannt' })));
    expect(hook.result.current.chips).toHaveLength(1);
    expect(hook.result.current.chips[0]!.label).toBe('Umbenannt');
  });

  it('entfernt einen Chip dauerhaft', () => {
    const first = renderHook(useCustomChips);
    act(() => first.result.current.add(custom()));
    act(() => first.result.current.add(custom({ id: 'eigen:zwei', label: 'Zwei' })));
    act(() => first.result.current.remove('eigen:test'));
    first.unmount();
    expect(renderHook(useCustomChips).result.current.chips.map((c) => c.id)).toEqual(['eigen:zwei']);
  });

  it('ignoriert gemerkte Chips mit defekter Form, ohne die Ansicht zu stören', () => {
    localStorage.setItem(CHIPS_KEY, JSON.stringify([{ id: 'eigen:x' }]));
    expect(renderHook(useCustomChips).result.current.chips).toEqual([]);
    localStorage.setItem(CHIPS_KEY, 'kein json');
    expect(renderHook(useCustomChips).result.current.chips).toEqual([]);
    expect(loadView()).toEqual(DEFAULT_VIEW);
  });
});
