// Test-Helfer der Review-Seite: Fixtures aus tools/paper/fixtures/ui/ (Stand des Review-Test-Repos, Basis HEAD~1),
// eine Attrappe von reviewApi und Mount/Klick mit act. Nur von *.test.tsx importiert.
import { Theme } from '@webkit/templates';
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { act, type ReactNode } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { vi } from 'vitest';
import type { Frame, PixelDiff, Review, Spec, Vorlage } from './types';

(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

const UI = resolve(__dirname, '../../../../../paper/fixtures/ui');

export function fixture<T>(name: string): T {
  return JSON.parse(readFileSync(resolve(UI, `${name}.json`), 'utf8')) as T;
}

/** Attrappe von reviewApi mit den Fixture-Antworten; Tests überschreiben einzelne Methoden. */
export function fakeApi() {
  return {
    review: vi.fn((_basis: string) => Promise.resolve(fixture<Review>('AssetsReview'))),
    frame: vi.fn((_id: string, _a: string, _f: number, _v: string, _b: string) => Promise.resolve(fixture<Frame>('AssetsFrame'))),
    pixelDiff: vi.fn((_id: string, _a: string, _f: number, _b: string) => Promise.resolve(fixture<PixelDiff>('AssetsPixelDiff'))),
    get: vi.fn((_id: string) => Promise.resolve(fixture<Spec>('AssetsGet'))),
    prompts: vi.fn(() => Promise.resolve(fixture<Vorlage[]>('AssetsPrompts'))),
  };
}

let root: Root | null = null;

export async function flush() {
  for (let i = 0; i < 6; i++) await act(async () => new Promise((r) => setTimeout(r, 0)));
}

export async function mount(node: ReactNode): Promise<HTMLElement> {
  const host = document.createElement('div');
  document.body.append(host);
  root = createRoot(host);
  await act(async () => root!.render(<Theme>{node}</Theme>));
  await flush();
  return host;
}

export function unmount() {
  act(() => root?.unmount());
  root = null;
  document.body.innerHTML = '';
}

export async function click(el: Element | null | undefined) {
  if (!el) throw new Error('Element fehlt');
  await act(async () => (el as HTMLElement).click());
  await flush();
}

export const button = (host: ParentNode, text: string) =>
  [...host.querySelectorAll('button')].find((b) => b.textContent?.trim() === text);

export const key = (el: Element, k: string) =>
  act(async () => {
    el.dispatchEvent(new KeyboardEvent('keydown', { key: k, bubbles: true }));
  });
