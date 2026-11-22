// @vitest-environment happy-dom
import { Theme } from '@radix-ui/themes';
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, describe, expect, it, vi } from 'vitest';
import { backend } from '../api';
import { WorldPage } from './WorldPage';

// WZ62.6/AC-05 (B-1172): Eine Welt lässt sich im Welten-Editor löschen (mit Namen bestätigt); feste Welten und Welten mit Erben
// werden mit Grund abgelehnt.

(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

let root: Root | null = null;
afterEach(() => {
  act(() => root?.unmount());
  root = null;
  document.body.innerHTML = '';
  vi.restoreAllMocks();
});

async function flush() {
  for (let i = 0; i < 6; i++) await act(async () => new Promise((r) => setTimeout(r, 0)));
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
const byLabel = (host: ParentNode, label: string) => host.querySelector<HTMLElement>(`[aria-label="${label}"]`)!;
const click = async (el: HTMLElement) => {
  await act(async () => el.click());
  await flush();
};
async function typeName(host: ParentNode, text: string) {
  const input = byLabel(host, 'Name zur Bestätigung') as HTMLInputElement;
  await act(async () => {
    Object.getOwnPropertyDescriptor(HTMLInputElement.prototype, 'value')!.set!.call(input, text);
    input.dispatchEvent(new Event('input', { bubbles: true }));
  });
}

describe('Welt löschen (B-1172)', () => {
  it('Welt ohne Erben: Name bestätigen, Welt verschwindet aus der Liste', async () => {
    expect((await backend.worldCreate('wegwerf', 'Wegwerf', '')).error).toBe('');
    const host = await mount();
    await click(host.querySelector('[data-world="wegwerf"]')!);
    await click(byLabel(host, 'Welt löschen'));
    expect((byLabel(host, 'Endgültig löschen') as HTMLButtonElement).disabled).toBe(true);
    await typeName(host, 'Wegwerf');
    await click(byLabel(host, 'Endgültig löschen'));
    expect(host.querySelector('[data-world="wegwerf"]')).toBeNull();
    expect(host.textContent).toContain('Welt wegwerf gelöscht');
  });

  it('feste Welt: Ablehnung mit Grund, Welt bleibt', async () => {
    const host = await mount();
    await click(host.querySelector('[data-world="standard"]')!);
    await click(byLabel(host, 'Welt löschen'));
    await typeName(host, 'Standard');
    await click(byLabel(host, 'Endgültig löschen'));
    expect(host.querySelector('[role="alert"]')?.textContent).toContain('feste Welt');
    expect(host.querySelector('[data-world="standard"]')).not.toBeNull();
  });

  it('Welt mit Erben: Ablehnung nennt die Erben', async () => {
    expect((await backend.worldCreate('kind', 'Kind', 'wueste')).error).toBe('');
    const host = await mount();
    await click(host.querySelector('[data-world="wueste"]')!);
    await click(byLabel(host, 'Welt löschen'));
    await typeName(host, 'Wüste');
    await click(byLabel(host, 'Endgültig löschen'));
    expect(host.querySelector('[role="alert"]')?.textContent).toContain('Erben (kind)');
    expect((await backend.worldDelete('kind')).error).toBe('');
  });
});
