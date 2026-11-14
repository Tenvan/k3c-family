// @vitest-environment happy-dom
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { clearSizeCache } from '../size/sizeCache';
import { ReviewPage } from './index';
import { button, click, fakeApi, mount, unmount, until } from './testing';

// B-733/AC-03 (Darstellung wie mit #514 umgesetzt, Default B-733 › Offene Fragen): die Review-Seite schaltet zwischen
// Vorher/Nachher und dem Größenvergleich um (seit B-853 Vorher und Nachher nebeneinander); die Wahl gilt für alle Einträge.

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
    await until('Eintrag monarch', () => eintrag(host, 'monarch'));
    await click(eintrag(host, 'monarch'));
    await until('Vorher/Nachher', () => host.querySelector('img[alt="Nachher"]'));
    expect(host.querySelector('[data-size-scene]')).toBeNull();
    await click(button(host, 'Größenvergleich'));
    await until('Größenvergleich', () => host.querySelector('[data-size-scene]') && !host.querySelector('img[alt="Nachher"]'));
    // anderer Eintrag: die Ansicht bleibt Größenvergleich (kein Vorher/Nachher; mauer kennt die Mock-Größe nicht)
    await click(eintrag(host, 'mauer'));
    await until('Knopf Vorher/Nachher', () => button(host, 'Vorher/Nachher'));
    expect(host.querySelector('img[alt="Nachher"]')).toBeNull();
    await click(button(host, 'Vorher/Nachher'));
    await until('Vorher/Nachher wieder', () => host.querySelector('img[alt="Nachher"]'));
  });

  it('gelöschtes Asset: nur das Vorher-Bild, kein Nachher', async () => {
    const host = await mount(<ReviewPage />);
    await until('Eintrag fass', () => eintrag(host, 'fass'));
    await click(eintrag(host, 'fass'));
    await until('Knopf Größenvergleich', () => button(host, 'Größenvergleich'));
    await click(button(host, 'Größenvergleich'));
    await until('Hinweis', () => host.textContent?.includes('Gelöscht: kein Arbeitsstand, nur das Vorher-Bild'));
    await until('Vorher', () => host.querySelector('[data-size-stand="Vorher (32eb7a7)"]'));
    expect(host.querySelector('[data-size-stand="Nachher"]')).toBeNull();
  });
});

describe('Größenvergleich für das Vorher-Bild (B-853)', () => {
  it('geändertes Asset: Vorher (Basis) und Nachher nebeneinander, das Vorher-Bild kommt aus der Basis', async () => {
    const host = await mount(<ReviewPage />);
    await until('Eintrag monarch', () => eintrag(host, 'monarch'));
    await click(eintrag(host, 'monarch'));
    await until('Knopf Größenvergleich', () => button(host, 'Größenvergleich'));
    await click(button(host, 'Größenvergleich'));
    await until('zwei Szenen', () => host.querySelectorAll('[data-size-scene]').length === 2);
    const staende = [...host.querySelectorAll('[data-size-stand]')].map((e) => e.getAttribute('data-size-stand'));
    expect(staende).toEqual(['Vorher (32eb7a7)', 'Nachher']);
    expect(fake.api.assetsFrame.mock.calls.some((c) => c[0] === 'monarch' && c[4] === '32eb7a7')).toBe(true);
  });

  it('neues Asset: ohne Basis nur der Arbeitsstand', async () => {
    const host = await mount(<ReviewPage />);
    await until('Eintrag pferd', () => eintrag(host, 'pferd'));
    await click(eintrag(host, 'pferd'));
    await until('Knopf Größenvergleich', () => button(host, 'Größenvergleich'));
    await click(button(host, 'Größenvergleich'));
    await until('Fehler oder Szene', () => host.querySelector('[data-size-compare]'));
    expect(host.querySelector('[data-size-stand]')).toBeNull();
  });
});
