// @vitest-environment happy-dom
import { act } from 'react';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { fillPrompt, reviewWerte } from './prompt';
import { PromptButtons } from './PromptButtons';
import { button, click, fakeApi, fixture, mount, unmount } from './testing';
import type { Review, Vorlage } from './types';

// RV1.4 / AC-06: Prompt-Buttons am Review-Eintrag kopieren den gefüllten Prompt.

const fake = vi.hoisted(() => ({ api: {} as ReturnType<typeof import('./testing').fakeApi> }));
vi.mock('./api', () => ({ reviewApi: new Proxy({}, { get: (_t, k: string) => (fake.api as Record<string, unknown>)[k] }) }));

const monarch = fixture<Review>('AssetsReview').assets.find((a) => a.id === 'monarch')!;
let writes: string[];

function clipboard(writeText: (t: string) => Promise<void>) {
  Object.defineProperty(navigator, 'clipboard', { configurable: true, value: { writeText } });
}

beforeEach(() => {
  fake.api = fakeApi();
  writes = [];
  clipboard(async (t) => void writes.push(t));
});
afterEach(unmount);

async function tippe(host: HTMLElement, wert: string) {
  const feld = host.querySelector<HTMLInputElement>('input[aria-label="Auftrag an Claude"]')!;
  await act(async () => {
    Object.getOwnPropertyDescriptor(HTMLInputElement.prototype, 'value')!.set!.call(feld, wert);
    feld.dispatchEvent(new Event('input', { bubbles: true }));
  });
}

describe('PromptButtons', () => {
  it('zeigt nur Vorlagen am Review-Eintrag', async () => {
    const host = await mount(<PromptButtons asset={monarch} basis="HEAD~1" />);
    expect(button(host, 'Änderung zurücknehmen')).toBeDefined();
    expect(button(host, 'Nachbessern …')).toBeDefined();
    expect(button(host, 'Material ändern')).toBeUndefined();
  });

  it('Button gesperrt ohne Eingabe', async () => {
    const host = await mount(<PromptButtons asset={monarch} basis="HEAD~1" />);
    expect(button(host, 'Nachbessern …')?.disabled).toBe(true);
  });

  it('kopiert den gefüllten Prompt', async () => {
    const host = await mount(<PromptButtons asset={monarch} basis="HEAD~1" />);
    await tippe(host, 'etwas dunkler');
    await click(button(host, 'Nachbessern …'));
    const vorlage = fixture<Vorlage[]>('AssetsPrompts').find((v) => v.id === 'nachbessern')!;
    expect(writes).toEqual([fillPrompt(vorlage, reviewWerte(monarch, 'HEAD~1', 'etwas dunkler'))]);
    expect(host.textContent).toContain('Kopiert.');
  });

  it('Fehler beim Kopieren sichtbar', async () => {
    clipboard(() => Promise.reject(new Error('gesperrt')));
    const host = await mount(<PromptButtons asset={monarch} basis="HEAD~1" />);
    await tippe(host, 'etwas dunkler');
    await click(button(host, 'Änderung zurücknehmen'));
    expect(host.textContent).toContain('gesperrt');
  });
});
