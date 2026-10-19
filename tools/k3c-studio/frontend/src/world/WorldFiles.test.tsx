// @vitest-environment happy-dom
import { Theme } from '@radix-ui/themes';
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, describe, expect, it, vi } from 'vitest';
import { backend } from '../api';
import { WorldPage } from './WorldPage';

// Welten als Dateien im Welten Editor (DOM, Mock-Backend): Welt wählen, anlegen, löschen; Insel + und −.
vi.mock('../stage/PreviewStage', () => ({ PreviewStage: class { show() {} refresh() {} setLook() {} setPosition() {} destroy() {} } }));
(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

let root: Root | null = null;
let host: HTMLElement | null = null;

const wait = (ms = 0) => act(async () => { await new Promise((r) => setTimeout(r, ms)); });
const flush = async () => { for (let i = 0; i < 6; i++) await wait(); };

async function mount(): Promise<HTMLElement> {
  host = document.createElement('div');
  document.body.append(host);
  root = createRoot(host);
  await act(async () => root!.render(<Theme><WorldPage /></Theme>));
  await flush();
  return host;
}

const button = (h: HTMLElement, label: string) => [...h.querySelectorAll('button')].find((b) => b.textContent === label)!;
const worldSelect = (h: HTMLElement) => h.querySelector<HTMLSelectElement>('select[aria-label="Welt"]')!;
const islands = (h: HTMLElement) => h.querySelector<HTMLSelectElement>('select[aria-label="Insel"]')!.options.length;

async function typeIn(el: HTMLInputElement, value: string) {
  Object.getOwnPropertyDescriptor(HTMLInputElement.prototype, 'value')!.set!.call(el, value);
  await act(async () => el.dispatchEvent(new Event('input', { bubbles: true })));
}

afterEach(async () => {
  act(() => root?.unmount());
  host?.remove();
  for (const w of (await backend.planWorlds()).worlds) if (w.id !== 'standard') await backend.planWorldRemove(w.id);
  await backend.planWorldSelect('standard');
});

describe('Welten im Welten Editor', () => {
  it('zeigt die Standard-Welt; Welt − ist dort gesperrt', async () => {
    const h = await mount();
    expect([...worldSelect(h).options].map((o) => o.textContent)).toEqual(['Standard']);
    expect((button(h, 'Welt −') as HTMLButtonElement).disabled).toBe(true);
    expect((button(h, 'Welt +') as HTMLButtonElement).disabled).toBe(true); // ohne Namen
  });

  it('legt eine Welt mit Namen an, wählt sie und lädt ihre Inseln (eine Insel); die Standard-Welt bleibt unberührt', async () => {
    const h = await mount();
    await typeIn(h.querySelector<HTMLInputElement>('input[aria-label="Name der neuen Welt"]')!, 'Sued Kueste');
    await act(async () => button(h, 'Welt +').click());
    await flush();
    expect(worldSelect(h).value).toBe('sued-kueste');
    expect([...worldSelect(h).options].map((o) => o.textContent)).toEqual(['Standard', 'Sued Kueste']);
    await act(async () => button(h, 'Insel +').click());
    await flush();
    expect(islands(h)).toBe(2);
    Object.getOwnPropertyDescriptor(HTMLSelectElement.prototype, 'value')!.set!.call(worldSelect(h), 'standard');
    await act(async () => worldSelect(h).dispatchEvent(new Event('change', { bubbles: true })));
    await flush();
    expect(islands(h)).toBe(1);
  });

  it('löscht die gewählte Welt nach Rückfrage und wechselt zur Standard-Welt; ohne Zustimmung bleibt sie', async () => {
    const h = await mount();
    await typeIn(h.querySelector<HTMLInputElement>('input[aria-label="Name der neuen Welt"]')!, 'Test');
    await act(async () => button(h, 'Welt +').click());
    await flush();
    const confirm = vi.fn().mockReturnValueOnce(false).mockReturnValueOnce(true);
    window.confirm = confirm;
    await act(async () => button(h, 'Welt −').click());
    await flush();
    expect(worldSelect(h).value).toBe('test');
    await act(async () => button(h, 'Welt −').click());
    await flush();
    expect(worldSelect(h).value).toBe('standard');
    expect(worldSelect(h).options).toHaveLength(1);
  });
});
