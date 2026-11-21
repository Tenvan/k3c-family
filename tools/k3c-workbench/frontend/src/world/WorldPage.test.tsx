// @vitest-environment happy-dom
import { Theme } from '@radix-ui/themes';
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, describe, expect, it, vi } from 'vitest';
import { backend } from '../api';
import { ASSET_LABELS, ASSET_PAGES } from '../Header';
import { storyGaps, storyPrompt } from './StoryState';
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
  it('ist eine Seite im Bereich Studio', () => {
    expect(ASSET_PAGES).toContain('welten');
    expect(ASSET_LABELS.welten).toBe('Welten-Editor');
  });

  it('B-1168/AC-01: feste Welten ohne Datei stehen mit Hinweis in der Liste und legen sich per Klick aus Standard an und öffnen sich (WL1.2)', async () => {
    const host = await mount();
    const nil = host.querySelector('[data-world="nil"]')!;
    expect(nil.textContent).toContain('noch nicht umgesetzt');
    expect(nil.getAttribute('role')).toBe('button');
    const create = vi.spyOn(backend, 'worldCreate').mockResolvedValue({ id: 'nil', warnings: [], error: 'Test' });
    await click(nil);
    expect(create).toHaveBeenCalledWith('nil', expect.any(String), 'standard');
    expect(host.querySelector('[data-world="standard"]')?.textContent).not.toContain('noch nicht umgesetzt');
  });

  it('zeigt Welten-Liste, Chunk-Vorschau und Inspektor des gewählten Abschnitts', async () => {
    const host = await mount();
    expect([...host.querySelectorAll('[data-world]')].map((e) => e.getAttribute('data-world'))).toEqual(['standard', 'sengoku', 'nil', 'britannien', 'wueste']);
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

describe('Welt anlegen (B-954/AC-02)', () => {
  const type = async (el: HTMLInputElement, text: string) => {
    await act(async () => {
      Object.getOwnPropertyDescriptor(HTMLInputElement.prototype, 'value')!.set!.call(el, text);
      el.dispatchEvent(new Event('input', { bubbles: true }));
    });
  };

  it('legt die Welt an, wählt sie und lehnt eine vorhandene mit Grund ab', async () => {
    const host = await mount();
    const create = vi.spyOn(backend, 'worldCreate');
    await click(host.querySelector('[aria-label="Welt anlegen"]'));
    await type(host.querySelector<HTMLInputElement>('[aria-label="Welt-ID"]')!, 'standard');
    await type(host.querySelector<HTMLInputElement>('[aria-label="Welt-Name"]')!, 'Doppelt');
    await click(host.querySelector('[aria-label="Anlegen"]'));
    expect(host.querySelector('[role="alert"]')?.textContent).toContain('gibt es schon');
    await type(host.querySelector<HTMLInputElement>('[aria-label="Welt-ID"]')!, 'hanse');
    await type(host.querySelector<HTMLInputElement>('[aria-label="Welt-Name"]')!, 'Hanse');
    await click(host.querySelector('[aria-label="Anlegen"]'));
    expect(create).toHaveBeenLastCalledWith('hanse', 'Hanse', '');
    expect(host.querySelector('[data-world="hanse"]')).toBeTruthy();
    expect(host.querySelector('[data-scroll="detail"] h1')?.textContent).toContain('Hanse');
  });

  it('zeigt nach dem Anlegen „Geschichte fehlt“ und kopiert den Auftrag mit Welt-ID und world_story_set (B-975/AC-04)', async () => {
    const host = await mount();
    const writes: string[] = [];
    Object.defineProperty(navigator, 'clipboard', { configurable: true, value: { writeText: async (t: string) => void writes.push(t) } });
    await click(host.querySelector('[aria-label="Welt anlegen"]'));
    await type(host.querySelector<HTMLInputElement>('[aria-label="Welt-ID"]')!, 'saga');
    await type(host.querySelector<HTMLInputElement>('[aria-label="Welt-Name"]')!, 'Saga');
    await click(host.querySelector('[aria-label="Anlegen"]'));
    const story = host.querySelector('[aria-label="Geschichte"]')!;
    expect(story.textContent).toContain('Geschichte fehlt');
    await click(story.querySelector('[aria-label="Geschichte schreiben"]'));
    expect(writes).toHaveLength(1);
    expect(writes[0]).toContain('Welt-ID saga');
    expect(writes[0]).toContain('world_story_set {id: "saga"');
    expect(writes[0]).toContain('Klang:');
    expect(writes[0]).toContain('Insel island1: story');
    expect(story.querySelector('[role="status"]')?.textContent).toBe('Auftrag kopiert');
  });

  it('nennt den Stand einer vorhandenen und einer unvollständigen Geschichte', () => {
    const entry = ['Oru KRONE no MEER.'];
    const base = { id: 'saga', name: 'Saga', notes: '', extends: '', error: '' };
    const island = { id: 'island1', name: 'Insel 1', depths: [0], biomes: ['forest'], plannedChunks: {}, chunkCounts: {} };
    const full = { ...base, story: { lexicon: ['oru', 'no'], start: entry, end: entry, bottles: [entry] },
      islands: [{ ...island, story: { arrival: entry, boss: entry, done: entry } }] };
    expect(storyGaps(full)).toEqual([]);
    expect(storyGaps({ ...full, story: { lexicon: ['oru'], start: entry }, islands: [{ ...island, story: { done: entry } }, { ...island, id: 'island2' }] }))
      .toEqual(['story.end', 'story.bottles', 'Insel island1: story.arrival', 'Insel island1: story.boss', 'Insel island2: story']);
    expect(storyPrompt(full)).toContain('Alle Einträge sind vorhanden');
  });

  it('Anlegen bleibt gesperrt, solange die ID ungültig ist', async () => {
    const host = await mount();
    await click(host.querySelector('[aria-label="Welt anlegen"]'));
    await type(host.querySelector<HTMLInputElement>('[aria-label="Welt-ID"]')!, 'Ungültig');
    await type(host.querySelector<HTMLInputElement>('[aria-label="Welt-Name"]')!, 'X');
    expect(host.querySelector<HTMLButtonElement>('[aria-label="Anlegen"]')!.disabled).toBe(true);
  });
});
