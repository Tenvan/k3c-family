// @vitest-environment happy-dom
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { act } from 'react';
import { backend } from '../../api';
import { buttons, click, flush, unmount } from '../testing';
import { mountCatalog, select, until } from './catalogTesting';

// WB1.3/AC-03: Prompt-Buttons am Asset und am angeklickten Teil kopieren den gefüllten Prompt aus AssetsPrompts.

const writeText = vi.fn(async (text: string) => void text);

beforeEach(() => {
  writeText.mockClear();
  Object.defineProperty(navigator, 'clipboard', { value: { writeText }, configurable: true });
  // happy-dom rechnet kein Layout: Bild und Fläche 160×200, Maßstab 1.
  vi.spyOn(Element.prototype, 'getBoundingClientRect').mockReturnValue({ left: 0, top: 0, width: 160, height: 200, x: 0, y: 0, right: 160, bottom: 200, toJSON: () => ({}) } as DOMRect);
  Object.defineProperty(HTMLImageElement.prototype, 'naturalWidth', { get: () => 160, configurable: true });
  Object.defineProperty(HTMLImageElement.prototype, 'naturalHeight', { get: () => 200, configurable: true });
});

afterEach(() => {
  unmount();
  vi.restoreAllMocks();
});

/** Katalog mit gewähltem monarch; wartet, bis Bild und Prompt-Vorlagen da sind (B-837). */
async function monarch() {
  const host = await mountCatalog();
  await select(host, 'monarch');
  const erste = (await backend.assetsPrompts()).find((p) => p.ort === 'asset')!;
  await until('Prompt-Buttons', () => buttons(host, erste.titel).length > 0);
  return host;
}

async function pick(host: HTMLElement, x: number, y: number) {
  const layer = host.querySelector<HTMLElement>('[data-hit-layer]')!;
  await act(async () => layer.dispatchEvent(new MouseEvent('click', { bubbles: true, clientX: x, clientY: y })));
  await flush();
}

describe('Prompt-Buttons im Katalog (WB1.3)', () => {
  it('am Asset: sechs Buttons, Klick kopiert den gefüllten Prompt', async () => {
    const prompts = await backend.assetsPrompts();
    const host = await monarch();
    for (const p of prompts.filter((x) => x.ort === 'asset')) expect(buttons(host, p.titel), p.titel).toHaveLength(1);
    await click(buttons(host, 'Material ändern')[0] ?? buttons(host, prompts[0].titel)[0]);
    expect(writeText).toHaveBeenCalledTimes(1);
    const text = writeText.mock.calls[0][0];
    expect(text).toContain('monarch');
    expect(text).toContain('public/grafik/figuren/monarch.paper.json');
    expect(text).toContain('gültig, Version 1');
    expect(text).not.toMatch(/\{[a-z]+\}/);
  });

  it('ohne gewähltes Teil keine Teil-Buttons; Klick in die Vorschau wählt das Teil', async () => {
    const prompts = await backend.assetsPrompts();
    const teil = prompts.filter((x) => x.ort === 'teil');
    const host = await monarch();
    for (const p of teil) expect(buttons(host, p.titel), p.titel).toHaveLength(0);
    await pick(host, 80, 60);
    expect(host.querySelector('[data-teil-mark="kopf"]')).toBeTruthy();
    expect(host.textContent).toContain('Teil: kopf');
    for (const p of teil) expect(buttons(host, p.titel), p.titel).toHaveLength(1);
    await click(buttons(host, teil[0].titel)[0]);
    expect(writeText.mock.calls.at(-1)![0]).toContain('kopf');
  });

  it('Klick daneben hebt die Auswahl auf', async () => {
    const host = await monarch();
    await pick(host, 80, 60);
    await pick(host, 1, 1);
    expect(host.querySelector('[data-teil-mark]')).toBeNull();
  });
});
