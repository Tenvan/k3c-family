// @vitest-environment happy-dom
import { Theme } from '@radix-ui/themes';
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, describe, expect, it, vi } from 'vitest';
import type { WorldDetail } from '../api';
import { StoryTab } from './StoryTab';
import { partPrompt } from './storyPrompts';
import { worldPrompt } from './worldPrompts';
import { WorldPage } from './WorldPage';

// B-1216: Reiter Geschichte (Baum mit Prompt je Knoten), Prompts „Welt ändern lassen“ und Werte als Baum.

(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

let root: Root | null = null;
afterEach(() => {
  act(() => root?.unmount());
  root = null;
  document.body.innerHTML = '';
  vi.restoreAllMocks();
});

const flush = async () => {
  for (let i = 0; i < 5; i++) await act(async () => new Promise((r) => setTimeout(r, 0)));
};
const click = async (el: Element | null | undefined) => {
  expect(el).toBeTruthy();
  await act(async () => (el as HTMLElement).click());
  await flush();
};
async function mountPage(tab: string): Promise<HTMLElement> {
  localStorage.clear();
  localStorage.setItem('k3c-workbench.world.tab', tab);
  const host = document.createElement('div');
  document.body.appendChild(host);
  root = createRoot(host);
  act(() => root!.render(<Theme><WorldPage /></Theme>));
  await flush();
  return host;
}

const line = ['Oru KRONE no MEER.'];
const detail: WorldDetail = {
  id: 'saga', name: 'Saga', notes: '', extends: '', error: '',
  story: { lexicon: ['oru', 'no'], start: line, end: line, bottles: [line], etappen: { '1': line } },
  islands: [
    { id: 'island1', name: 'Insel 1', depths: [0], biomes: ['forest'], plannedChunks: {}, chunkCounts: {}, story: { arrival: line, boss: line, done: line } },
    { id: 'island2', name: 'Insel 2', depths: [0], biomes: ['cave'], plannedChunks: {}, chunkCounts: {}, etappe: 2 },
  ],
};

function mountStory(d: WorldDetail): HTMLElement {
  const host = document.createElement('div');
  document.body.appendChild(host);
  root = createRoot(host);
  act(() => root!.render(<Theme><StoryTab detail={d} /></Theme>));
  return host;
}

describe('Geschichte als Baum (B-1216/AC-02)', () => {
  it('zeigt Start, Etappen mit Inseln, Ende, Flaschenpost und Wortschatz; Fehlendes ist markiert', () => {
    const host = mountStory(detail);
    const nodes = [...host.querySelectorAll('[data-story-node]')].map((n) => n.getAttribute('data-story-node'));
    expect(nodes).toEqual(['start', 'island1.arrival', 'island1.boss', 'island1.done', 'etappe.1', 'island2.arrival', 'island2.boss', 'island2.done', 'etappe.2', 'end', 'bottles', 'lexicon']);
    expect(host.querySelector('[data-story-node="start"]')?.textContent).toContain('„Oru KRONE no MEER.“');
    expect(host.querySelector('[data-story-node="island2.boss"]')?.textContent).toContain('fehlt');
    expect(host.querySelector('[data-story-node="island1.boss"]')?.textContent).not.toContain('fehlt');
    expect(host.querySelector('[data-story-node="lexicon"]')?.textContent).toContain('oru · no');
  });

  it('der Prompt je Knoten nennt Teil, Ist-Zustand, world_story_set und world_check', async () => {
    const host = mountStory(detail);
    const writes: string[] = [];
    Object.defineProperty(navigator, 'clipboard', { configurable: true, value: { writeText: async (t: string) => void writes.push(t) } });
    await click(host.querySelector('[data-story-node="island1.boss"] button'));
    expect(writes).toHaveLength(1);
    expect(writes[0]).toContain('Endboss der Insel island1');
    expect(writes[0]).toContain('„Oru KRONE no MEER.“');
    expect(writes[0]).toContain('world_story_set');
    expect(writes[0]).toContain('world_check {id: "saga"}');
    expect(partPrompt(detail, { kind: 'island', island: 'island2', entry: 'boss' })).toContain('Jetzt: fehlt.');
  });

  it('der Reiter erscheint in der Seite und zeigt die Geschichte der gewählten Welt', async () => {
    const host = await mountPage('geschichte');
    expect(host.querySelector('[aria-label="Geschichte der Welt"]')).toBeTruthy();
    expect(host.querySelector('[data-story-node="start"]')).toBeTruthy();
  });
});

describe('Welt ändern lassen (B-1216/AC-03)', () => {
  it('der Prompt nennt Welt, Inseln, Werkzeug und den Wunsch', () => {
    const p = worldPrompt(detail, 'island-size', 'island2', 'mittel');
    expect(p).toContain('Welt-ID saga');
    expect(p).toContain('Insel island2');
    expect(p).toContain('chunkCounts');
    expect(p).toContain('world_check {id: "saga"}');
    expect(p).toContain('Was soll sich ändern: mittel');
  });

  it('die Seite bietet die Prompt-Auswahl auf dem Reiter Inseln', async () => {
    const host = await mountPage('inseln');
    expect(host.querySelector('[aria-label="Welt ändern lassen"]')).toBeTruthy();
    expect(host.querySelector('[aria-label="Was soll sich ändern?"]')).toBeTruthy();
  });
});

describe('Werte als Baum (B-1216/AC-04)', () => {
  it('der Baum zeigt Dateien mit Symbol und Namen; ein Knoten begrenzt die Felder', async () => {
    const host = await mountPage('werte');
    const tree = host.querySelector('[role="tree"]')!;
    expect(tree.querySelector('[data-node="buildings"]')?.textContent).toContain('Gebäude');
    const all = host.querySelectorAll('[role="row"][data-status]').length;
    await click(tree.querySelector('[data-node="buildings"]'));
    const inFile = [...host.querySelectorAll('[role="row"][data-status]')].map((r) => r.getAttribute('data-key')!.split(':')[0]);
    expect(inFile.length).toBeGreaterThan(0);
    expect(inFile.length).toBeLessThan(all);
    expect(new Set(inFile)).toEqual(new Set(['buildings']));
    await click(tree.querySelector('[data-node="buildings:tower"]'));
    const inGroup = [...host.querySelectorAll('[role="row"][data-status]')].map((r) => r.getAttribute('data-key')!.split(':')[1]!.split('\u0000')[0]);
    expect(new Set(inGroup)).toEqual(new Set(['tower']));
  });
});
