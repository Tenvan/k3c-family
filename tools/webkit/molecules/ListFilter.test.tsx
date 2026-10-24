// @vitest-environment happy-dom
import { Theme } from '@radix-ui/themes';
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, describe, expect, it } from 'vitest';
import { createPrefs } from '../lib/prefs';
import { pickValue, selectedValue } from '../testing/select';
import { ListFilter, type ListFilterProps } from './ListFilter';

(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

interface Row {
  id: string;
  art: string;
  status: string;
}
const rows: Row[] = [
  { id: 'b-hit', art: 'sfx', status: 'Lücke' },
  { id: 'a-bow', art: 'sfx', status: 'zugeordnet' },
  { id: 'c-tag', art: 'musik', status: 'Lücke' },
];
const prefs = createPrefs('test.');
const base: ListFilterProps<Row> = {
  items: rows,
  itemKey: (r) => r.id,
  renderItem: (r) => <span data-row>{r.id}</span>,
  searchText: (r) => `${r.id} ${r.art}`,
  dimensions: [
    { key: 'art', label: 'Art', values: (r) => r.art, order: ['sfx', 'musik'] },
    { key: 'status', label: 'Status', values: (r) => r.status },
  ],
  sorts: [
    { key: 'name', label: 'Name', compare: (a, b) => a.id.localeCompare(b.id) },
    { key: 'art', label: 'Art', compare: (a, b) => a.art.localeCompare(b.art) },
  ],
};

let root: Root | null = null;
let host: HTMLElement;
afterEach(() => {
  act(() => root?.unmount());
  host?.remove();
  localStorage.clear();
});

async function mount(props: Partial<ListFilterProps<Row>> = {}): Promise<HTMLElement> {
  host = document.createElement('div');
  document.body.append(host);
  root = createRoot(host);
  await act(async () => root!.render(<Theme><ListFilter {...base} {...props} /></Theme>));
  return host;
}

const rowIds = () => [...host.querySelectorAll('[data-row]')].map((e) => e.textContent);
const button = (text: string) => [...host.querySelectorAll('button')].find((b) => b.textContent?.startsWith(text))!;
const click = (el: HTMLElement) => act(async () => el.click());
const status = () => host.querySelector('[role="status"]')!.textContent;

describe('ListFilter', () => {
  it('zeigt alle Einträge nach der ersten Sortierung mit „n von m“', async () => {
    await mount();
    expect(rowIds()).toEqual(['a-bow', 'b-hit', 'c-tag']);
    expect(status()).toBe('3 von 3');
  });

  it('Chip filtert, Zähler stimmt, Zurücksetzen hebt auf', async () => {
    await mount();
    await click(button('Lücke'));
    expect(rowIds()).toEqual(['b-hit', 'c-tag']);
    expect(status()).toBe('2 von 3');
    await click(button('Zurücksetzen'));
    expect(rowIds()).toHaveLength(3);
  });

  it('Suche filtert; ohne Treffer Hinweis mit „Filter zurücksetzen“', async () => {
    await mount();
    const input = host.querySelector<HTMLInputElement>('input[aria-label="Suchen"]')!;
    const set = Object.getOwnPropertyDescriptor(HTMLInputElement.prototype, 'value')!.set!;
    await act(async () => { set.call(input, 'nix'); input.dispatchEvent(new Event('input', { bubbles: true })); });
    expect(rowIds()).toEqual([]);
    await click(button('Filter zurücksetzen'));
    expect(rowIds()).toHaveLength(3);
  });

  it('Sortierung wechselt', async () => {
    await mount();
    await pickValue(host, 'Sortierung', 'art');
    expect(rowIds()).toEqual(['c-tag', 'b-hit', 'a-bow']);
  });

  it('Gruppieren mit einklappbaren Abschnitten', async () => {
    await mount({ defaultGroup: 'art' });
    expect(host.querySelectorAll('[data-group]')).toHaveLength(2);
    expect(button('▾ sfx').textContent).toContain('(2)');
    await click(button('▾ sfx'));
    expect(rowIds()).toEqual(['c-tag']);
    await click(button('▸ sfx'));
    expect(rowIds()).toHaveLength(3);
  });

  it('Taste „/“ fokussiert die Suche, nicht beim Tippen in einem Feld', async () => {
    await mount();
    const input = host.querySelector<HTMLInputElement>('input[aria-label="Suchen"]')!;
    await act(async () => { window.dispatchEvent(new KeyboardEvent('keydown', { key: '/', bubbles: true, cancelable: true })); });
    expect(document.activeElement).toBe(input);
    input.blur();
    const other = document.createElement('input');
    document.body.append(other);
    other.focus();
    const ev = new KeyboardEvent('keydown', { key: '/', bubbles: true, cancelable: true });
    await act(async () => { other.dispatchEvent(ev); });
    expect(ev.defaultPrevented).toBe(false);
    expect(document.activeElement).toBe(other);
    other.remove();
  });

});

describe('ListFilter: Merken', () => {
  it('merkt Chips, Sortierung und Gruppe (nicht die Suche) und stellt sie wieder her', async () => {
    const mem = { storageKey: 'seite.liste', prefs };
    await mount(mem);
    await click(button('Lücke'));
    await pickValue(host, 'Sortierung', 'art');
    await pickValue(host, 'Gruppierung', 'art');
    act(() => root!.unmount());
    host.remove();
    await mount(mem);
    expect(selectedValue(host, 'Sortierung')).toBe('art');
    expect(selectedValue(host, 'Gruppierung')).toBe('art');
    expect(rowIds()).toEqual(['b-hit', 'c-tag']);
  });

  it('ein gemerkter Wert für eine unbekannte Dimension oder Sortierung stört nicht', async () => {
    prefs.saveJSON('seite.liste', { query: 'x', chips: { weg: ['a'], art: ['musik'] }, sort: 'weg', group: 'weg' });
    await mount({ storageKey: 'seite.liste', prefs });
    expect(rowIds()).toEqual(['c-tag']);
    expect(selectedValue(host, 'Sortierung')).toBe('name');
    expect(selectedValue(host, 'Gruppierung')).toBe('');
  });

  it('leere Liste zeigt den Hinweis des Aufrufers', async () => {
    await mount({ items: [], emptyHint: 'Keine Slots.' });
    expect(host.textContent).toContain('Keine Slots.');
  });
});
