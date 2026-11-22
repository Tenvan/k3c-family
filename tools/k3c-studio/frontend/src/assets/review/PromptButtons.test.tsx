// @vitest-environment happy-dom
import { act } from 'react';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { fillPrompt } from '../prompts/fill';
import { reviewWerte } from './prompt';
import { PromptButtons } from './PromptButtons';
import { button, click, fakeApi, fixture, mount, unmount, until } from './testing';
import type { Review, Vorlage } from '../api';

// RV1.4 / AC-06: Prompt-Buttons am Review-Eintrag kopieren den gefüllten Prompt.

const fake = vi.hoisted(() => ({ api: {} as ReturnType<typeof import('./testing').fakeApi> }));
vi.mock('../../api', async (echt) => {
  const m = await echt<typeof import('../../api')>();
  return { ...m, backend: new Proxy(m.backend, { get: (t, k: string) => (fake.api as Record<string, unknown>)[k] ?? Reflect.get(t, k) }) };
});

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
    await until('Vorlagen', () => button(host, 'Änderung zurücknehmen') && button(host, 'Nachbessern …'));
    expect(button(host, 'Änderung zurücknehmen')).toBeDefined();
    expect(button(host, 'Nachbessern …')).toBeDefined();
    expect(button(host, 'Material ändern')).toBeUndefined();
  });

  it('Button gesperrt ohne Eingabe', async () => {
    const host = await mount(<PromptButtons asset={monarch} basis="HEAD~1" />);
    await until('Nachbessern gesperrt', () => button(host, 'Nachbessern …')?.disabled);
  });

  it('kopiert den gefüllten Prompt', async () => {
    const host = await mount(<PromptButtons asset={monarch} basis="HEAD~1" />);
    await until('Eingabefeld', () => host.querySelector('input[aria-label="Auftrag an Claude"]'));
    await tippe(host, 'etwas dunkler');
    await until('Nachbessern frei', () => button(host, 'Nachbessern …') && !button(host, 'Nachbessern …')!.disabled);
    await click(button(host, 'Nachbessern …'));
    const vorlage = fixture<Vorlage[]>('AssetsPrompts').find((v) => v.id === 'nachbessern')!;
    await until('Prompt kopiert', () => writes.length > 0 && host.textContent?.includes('Kopiert.'));
    expect(writes).toEqual([fillPrompt(vorlage, reviewWerte(monarch, 'HEAD~1', 'etwas dunkler'))]);
    expect(host.textContent).toContain('Kopiert.');
  });

  it('Fehler beim Kopieren sichtbar', async () => {
    clipboard(() => Promise.reject(new Error('gesperrt')));
    const host = await mount(<PromptButtons asset={monarch} basis="HEAD~1" />);
    await until('Eingabefeld', () => host.querySelector('input[aria-label="Auftrag an Claude"]'));
    await tippe(host, 'etwas dunkler');
    await until('Änderung zurücknehmen', () => button(host, 'Änderung zurücknehmen'));
    await click(button(host, 'Änderung zurücknehmen'));
    await until('Fehlertext', () => host.textContent?.includes('gesperrt'));
  });
});
