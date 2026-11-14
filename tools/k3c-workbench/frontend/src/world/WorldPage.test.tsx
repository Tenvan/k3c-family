// @vitest-environment happy-dom
import { Theme } from '@radix-ui/themes';
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, describe, expect, it, vi } from 'vitest';
import { backend } from '../api';
import { LABELS, PAGES } from '../Header';
import { WorldPage } from './WorldPage';

// B-879/AC-01: Die Seite Welten-Editor zeigt mit dem Mock-Backend (Testlauf = Mock) die Welten, die Chunk-Vorschau und
// den Inspektor des gewählten Abschnitts; AC-02: eine Eingabe in den Werten geht als Override an das Backend.

(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

let root: Root | null = null;

async function flush() {
  for (let i = 0; i < 5; i++) await act(async () => new Promise((r) => setTimeout(r, 0)));
}

async function mount(): Promise<HTMLElement> {
  localStorage.clear();
  const host = document.createElement('div');
  document.body.appendChild(host);
  root = createRoot(host);
  act(() => root!.render(<Theme><WorldPage /></Theme>));
  await flush();
  return host;
}

afterEach(() => {
  act(() => root?.unmount());
  root = null;
  document.body.innerHTML = '';
  vi.restoreAllMocks();
});

const click = async (el: Element | null | undefined) => {
  expect(el).toBeTruthy();
  await act(async () => (el as HTMLElement).click());
  await flush();
};

describe('Welten-Editor (B-879/AC-01)', () => {
  it('ist eine Seite im Bereich Development', () => {
    expect(PAGES).toContain('welten');
    expect(LABELS.welten).toBe('Welten-Editor');
  });

  it('zeigt Welten-Liste, Chunk-Vorschau und Inspektor des gewählten Abschnitts', async () => {
    const host = await mount();
    expect([...host.querySelectorAll('[data-world]')].map((e) => e.getAttribute('data-world'))).toEqual(['standard', 'wueste']);
    expect(host.querySelector('[aria-label="Chunk-Vorschau"] [role="status"]')?.textContent).toContain('12 Abschnitte');
    expect(host.querySelectorAll('.wd-chunk')).toHaveLength(12);
    expect(host.querySelector('[aria-label="Inspektor"]')?.textContent).toContain('Wähle einen Abschnitt');

    await click(host.querySelector('[data-chunk="7"]'));
    const inspector = host.querySelector('[aria-label="Inspektor"]')!;
    expect(inspector.textContent).toContain('Abschnitt 7');
    expect(inspector.textContent).toContain('Platz +1');
    expect(inspector.textContent).toContain('geplant');
    expect(inspector.querySelector('[data-planned="1"]')?.textContent).toContain('Truhe');
  });

  it('wechselt die Welt und merkt sie', async () => {
    const host = await mount();
    await click(host.querySelector('[data-world="wueste"]'));
    expect(host.querySelector('[data-scroll="detail"] h1')?.textContent).toContain('Wüste');
    expect(localStorage.getItem('k3c-workbench.world.id')).toBe('wueste');
    expect(host.querySelector('[aria-label="Eltern-Welt"]')?.textContent).toContain('standard');
  });

  it('der Prompt an Claude nennt Welt-Datei, Insel, Tiefe und Platz', async () => {
    const host = await mount();
    const open = vi.spyOn(backend, 'openInClaude').mockResolvedValue();
    await click(host.querySelector('[data-chunk="3"]'));
    await click(host.querySelector('[aria-label="Abschnitt in Claude ändern"]'));
    const prompt = open.mock.calls[0]![0];
    expect(prompt).toContain('data/worlds/standard.json');
    expect(prompt).toContain('Platz −2');
    expect(prompt).toContain('plannedChunks › "0"');
  });
});

describe('Werte übersteuern (B-879/AC-02)', () => {
  it('eine Eingabe setzt den Override, ↺ setzt ihn zurück', async () => {
    const host = await mount();
    const set = vi.spyOn(backend, 'worldOverrideSet');
    const input = host.querySelector<HTMLInputElement>('[aria-label="Wert wall.cost"]')!;
    expect(input.value).toBe('3');
    await act(async () => {
      const setter = Object.getOwnPropertyDescriptor(HTMLInputElement.prototype, 'value')!.set!;
      setter.call(input, '7');
      input.dispatchEvent(new Event('input', { bubbles: true }));
    });
    await act(async () => input.dispatchEvent(new KeyboardEvent('keydown', { key: 'Enter', bubbles: true })));
    await flush();
    expect(set).toHaveBeenCalledWith('standard', 'buildings', ['wall', 'cost'], 7);
    expect(host.querySelector('[aria-label="Status wall.cost"]')?.textContent).toBe('überschrieben');
    await click(host.querySelector('[aria-label="Zurücksetzen wall.cost"]'));
    expect(set).toHaveBeenLastCalledWith('standard', 'buildings', ['wall', 'cost'], null);
    expect(host.querySelector('[aria-label="Status wall.cost"]')?.textContent).toBe('geerbt');
  });

  it('ein abgelehnter Wert zeigt den Grund und lässt das Feld geerbt', async () => {
    const host = await mount();
    const input = host.querySelector<HTMLInputElement>('[aria-label="Wert wall.hp"]')!;
    await act(async () => {
      Object.getOwnPropertyDescriptor(HTMLInputElement.prototype, 'value')!.set!.call(input, 'viel');
      input.dispatchEvent(new Event('input', { bubbles: true }));
    });
    await act(async () => input.dispatchEvent(new KeyboardEvent('keydown', { key: 'Enter', bubbles: true })));
    await flush();
    expect(host.textContent).toContain('Zahl erwartet');
    expect(host.querySelector('[aria-label="Status wall.hp"]')?.textContent).toBe('geerbt');
  });
});
