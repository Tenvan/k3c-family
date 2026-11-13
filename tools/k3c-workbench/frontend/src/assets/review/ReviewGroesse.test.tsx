// @vitest-environment happy-dom
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { clearSizeCache } from '../size/sizeCache';
import { ReviewPage } from './index';
import { button, click, fakeApi, mount, unmount } from './testing';

// B-733/AC-03 (Darstellung wie mit #514 umgesetzt, Default B-733 › Offene Fragen): die Review-Seite schaltet zwischen
// Vorher/Nachher und dem Größenvergleich des Arbeitsstands um; die Wahl gilt für alle Einträge.

const fake = vi.hoisted(() => ({ api: {} as ReturnType<typeof import('./testing').fakeApi> }));
vi.mock('../../api', async (echt) => {
  const m = await echt<typeof import('../../api')>();
  return { ...m, backend: new Proxy(m.backend, { get: (t, k: string) => (fake.api as Record<string, unknown>)[k] ?? Reflect.get(t, k) }) };
});

beforeEach(() => {
  fake.api = fakeApi();
});
afterEach(() => {
  unmount();
  clearSizeCache();
});

const eintrag = (host: ParentNode, id: string) => host.querySelector<HTMLElement>(`[data-asset="${id}"]`);

describe('Größenvergleich im Review (B-733/AC-03)', () => {
  it('schaltet zwischen Vorher/Nachher und Größenvergleich, die Wahl bleibt beim Eintragswechsel', async () => {
    const host = await mount(<ReviewPage />);
    await click(eintrag(host, 'monarch'));
    expect(host.querySelector('img[alt="Nachher"]')).not.toBeNull();
    expect(host.querySelector('[data-size-scene]')).toBeNull();
    await click(button(host, 'Größenvergleich'));
    expect(host.querySelector('[data-size-scene]')).not.toBeNull();
    expect(host.querySelector('img[alt="Nachher"]')).toBeNull();
    // anderer Eintrag: die Ansicht bleibt Größenvergleich (kein Vorher/Nachher; mauer kennt die Mock-Größe nicht)
    await click(eintrag(host, 'mauer'));
    expect(host.querySelector('img[alt="Nachher"]')).toBeNull();
    await click(button(host, 'Vorher/Nachher'));
    expect(host.querySelector('img[alt="Nachher"]')).not.toBeNull();
  });

  it('gelöschtes Asset: Hinweis statt Größenvergleich', async () => {
    const host = await mount(<ReviewPage />);
    await click(eintrag(host, 'fass'));
    await click(button(host, 'Größenvergleich'));
    expect(host.querySelector('[data-size-scene]')).toBeNull();
    expect(host.textContent).toContain('Gelöscht: kein Arbeitsstand für den Größenvergleich');
  });
});
