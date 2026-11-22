// @vitest-environment happy-dom
import { act } from 'react';
import { createRoot } from 'react-dom/client';
import { Theme } from '@webkit/templates';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { ReviewPage } from './index';
import { button, click, fakeApi, mount, unmount, until } from './testing';

// B-732: Review-Ergebnis, Specs und Frames überleben den Abbau der Seite; „Neu laden“ holt neu.

const fake = vi.hoisted(() => ({ api: {} as ReturnType<typeof import('./testing').fakeApi> }));
vi.mock('../../api', async (echt) => {
  const m = await echt<typeof import('../../api')>();
  return { ...m, backend: new Proxy(m.backend, { get: (t, k: string) => (fake.api as Record<string, unknown>)[k] ?? Reflect.get(t, k) }) };
});

beforeEach(() => {
  fake.api = fakeApi();
});
afterEach(unmount);

const eintrag = (host: ParentNode, id: string) => host.querySelector<HTMLElement>(`[data-asset="${id}"]`);

describe('Review-Cache (B-732)', () => {
  it('lädt nach dem Abbau der Seite weder Review noch Frames neu', async () => {
    const host = await mount(<ReviewPage />);
    await until('Eintrag monarch', () => eintrag(host, 'monarch'));
    await click(eintrag(host, 'monarch'));
    await until('Frames geladen', () => host.querySelector('img[alt="Nachher"]'));
    const frames = fake.api.assetsFrame.mock.calls.length;
    expect(fake.api.assetsReview).toHaveBeenCalledTimes(1);
    // zweite Seite mit eigener Wurzel, wie nach einem Tab-Wechsel (unmount() aus testing.tsx leert den Cache)
    const other = document.createElement('div');
    document.body.appendChild(other);
    const root = createRoot(other);
    await act(async () => root.render(<Theme><ReviewPage /></Theme>));
    await until('Liste aus dem Cache', () => eintrag(other, 'monarch'));
    await click(eintrag(other, 'monarch'));
    await until('Vorschau aus dem Cache', () => other.querySelector('img[alt="Nachher"]'));
    expect(fake.api.assetsReview).toHaveBeenCalledTimes(1);
    expect(fake.api.assetsGet).toHaveBeenCalledTimes(1);
    expect(fake.api.assetsFrame.mock.calls.length).toBe(frames);
    act(() => root.unmount());
  });

  it('„Neu laden“ holt neu und lässt die Liste dabei stehen', async () => {
    const host = await mount(<ReviewPage />);
    await until('Liste', () => eintrag(host, 'monarch') && button(host, 'Neu laden'));
    let fertig!: () => void;
    const real = fake.api.assetsReview.getMockImplementation()!;
    fake.api.assetsReview.mockImplementation((b) => new Promise((r) => (fertig = () => r(real(b)))));
    await click(button(host, 'Neu laden'));
    await until('zweite Anfrage und Ladeanzeige', () => fake.api.assetsReview.mock.calls.length === 2 && host.querySelector('[data-loading="review"]'));
    expect(eintrag(host, 'monarch')).not.toBeNull();
    await act(async () => fertig());
    await until('Ladeanzeige weg', () => !host.querySelector('[data-loading="review"]'));
  });
});
