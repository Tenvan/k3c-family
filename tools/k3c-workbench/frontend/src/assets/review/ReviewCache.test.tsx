// @vitest-environment happy-dom
import { act } from 'react';
import { createRoot } from 'react-dom/client';
import { Theme } from '@webkit/templates';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { ReviewPage } from './index';
import { button, click, fakeApi, flush, mount, unmount } from './testing';

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
    await click(eintrag(host, 'monarch'));
    const frames = fake.api.assetsFrame.mock.calls.length;
    expect(fake.api.assetsReview).toHaveBeenCalledTimes(1);
    // zweite Seite mit eigener Wurzel, wie nach einem Tab-Wechsel (unmount() aus testing.tsx leert den Cache)
    const other = document.createElement('div');
    document.body.appendChild(other);
    const root = createRoot(other);
    await act(async () => root.render(<Theme><ReviewPage /></Theme>));
    await flush();
    expect(eintrag(other, 'monarch')).not.toBeNull();
    await click(eintrag(other, 'monarch'));
    expect(other.querySelector('img[alt="Nachher"]')).not.toBeNull();
    expect(fake.api.assetsReview).toHaveBeenCalledTimes(1);
    expect(fake.api.assetsGet).toHaveBeenCalledTimes(1);
    expect(fake.api.assetsFrame.mock.calls.length).toBe(frames);
    act(() => root.unmount());
  });

  it('„Neu laden“ holt neu und lässt die Liste dabei stehen', async () => {
    const host = await mount(<ReviewPage />);
    let fertig!: () => void;
    const real = fake.api.assetsReview.getMockImplementation()!;
    fake.api.assetsReview.mockImplementation((b) => new Promise((r) => (fertig = () => r(real(b)))));
    await click(button(host, 'Neu laden'));
    expect(fake.api.assetsReview).toHaveBeenCalledTimes(2);
    expect(eintrag(host, 'monarch')).not.toBeNull();
    expect(host.querySelector('[data-loading="review"]')).not.toBeNull();
    await act(async () => fertig());
    await flush();
    expect(host.querySelector('[data-loading="review"]')).toBeNull();
  });
});
