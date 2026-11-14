// @vitest-environment happy-dom
import { act } from 'react';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { ReviewPage } from './index';
import { button, click, fakeApi, fixture, mount, unmount, until } from './testing';
import type { Review } from '../api';

// RV1.3 / AC-03, AC-05: Liste, Basis-Auswahl und Fehlerzustände der Review-Seite gegen die Fixtures.

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

describe('ReviewPage', () => {
  it('Liste und Detail je eigener Scrollbereich, Kopf außerhalb (WB2/AC-02)', async () => {
    const host = await mount(<ReviewPage />);
    await until('Eintrag monarch', () => eintrag(host, 'monarch'));
    await click(eintrag(host, 'monarch'));
    await until('Vorschau Nachher', () => host.querySelector('img[alt="Nachher"]'));
    const liste = host.querySelector('[data-scroll="list"]')!;
    const detail = host.querySelector('[data-scroll="detail"]')!;
    expect(liste.contains(eintrag(host, 'monarch'))).toBe(true);
    expect(detail.querySelector('img[alt="Nachher"]')).not.toBeNull();
    expect(liste.contains(detail)).toBe(false);
    expect(host.querySelector('.wk-listdetail-head')?.textContent).toContain('Asset-Review');
  });

  it('listet alle Einträge mit Status', async () => {
    const host = await mount(<ReviewPage />);
    await until('Liste und Basis-Stand', () => host.querySelectorAll('[data-asset]').length === 5 && host.textContent?.includes('32eb7a7'));
    for (const [id, status] of [['monarch', 'geaendert'], ['pferd', 'neu'], ['fass', 'geloescht'], ['banner', 'ungueltig'], ['mauer', 'geaendert']]) {
      expect(eintrag(host, id)?.getAttribute('data-status'), id).toBe(status);
    }
    expect(eintrag(host, 'mauer')?.textContent).toContain('Stil');
    expect(eintrag(host, 'monarch')?.textContent).toContain('umhang.material');
    expect(host.textContent).toContain('HEAD~1');
    expect(host.textContent).toContain('32eb7a7');
    expect(fake.api.assetsReview).toHaveBeenCalledWith('HEAD');
  });

  it('ungültige Spec rot mit Fehlertext, die übrigen bleiben sichtbar (AC-05)', async () => {
    const host = await mount(<ReviewPage />);
    const banner = await until('Eintrag banner', () => eintrag(host, 'banner'));
    const rot = [...banner.querySelectorAll('[data-accent-color="red"]')].map((e) => e.textContent).join(' ');
    expect(rot).toContain('Material „scharlach“ fehlt in der Stil-Datei');
    expect(host.querySelectorAll('[data-asset]').length).toBe(5);
  });

  it('Basis wählen ruft AssetsReview mit genau dem Wert', async () => {
    const host = await mount(<ReviewPage />);
    const anfragen = () => fake.api.assetsReview.mock.calls.length;
    await until('erste Anfrage', () => anfragen() === 1 && button(host, 'HEAD~1'));
    await click(button(host, 'HEAD~1'));
    await until('Anfrage HEAD~1', () => anfragen() === 2 && button(host, 'origin/main'));
    await click(button(host, 'origin/main'));
    await until('Anfrage origin/main', () => anfragen() === 3);
    const feld = host.querySelector<HTMLInputElement>('input[aria-label="Commit-SHA"]')!;
    await act(async () => {
      const setter = Object.getOwnPropertyDescriptor(HTMLInputElement.prototype, 'value')!.set!;
      setter.call(feld, '32eb7a7');
      feld.dispatchEvent(new Event('input', { bubbles: true }));
    });
    await click(button(host, 'Vergleichen'));
    await until('Anfrage Commit-SHA', () => anfragen() === 4);
    expect(fake.api.assetsReview.mock.calls.map((c) => c[0])).toEqual(['HEAD', 'HEAD~1', 'origin/main', '32eb7a7']);
  });

});

describe('ReviewPage: Fehler und Basis-Stand', () => {
  it('Fehler der Basis sichtbar, Liste leer', async () => {
    fake.api.assetsReview.mockImplementation(() => Promise.reject(new Error('Basis „main“ ungültig: HEAD, HEAD~1, Commit-SHA oder origin/main')));
    const host = await mount(<ReviewPage />);
    await until('Fehlermeldung', () => host.querySelector('[role="alert"]'));
    expect(host.querySelector('[role="alert"]')?.textContent).toContain('Basis „main“ ungültig');
    expect(host.querySelectorAll('[data-asset]').length).toBe(0);
  });

  it('Basis-Stand lokal mit Hinweis', async () => {
    fake.api.assetsReview.mockImplementation(() => Promise.resolve({ ...fixture<Review>('AssetsReview'), basisStand: 'lokal' }));
    const host = await mount(<ReviewPage />);
    await until('Basis-Stand lokal', () => host.textContent?.includes('Basis-Stand: lokal'));
  });
});

describe('ReviewPage nach dem Gesamt-Review', () => {
  it('Frames fragen den aufgelösten Basis-Commit an, nicht den Namen (Befund 1)', async () => {
    const host = await mount(<ReviewPage />);
    await until('Eintrag monarch', () => eintrag(host, 'monarch'));
    await click(eintrag(host, 'monarch'));
    const basen = () => new Set(fake.api.assetsFrame.mock.calls.map((c) => c[4]));
    await until('Frames beider Seiten', () => basen().size === 2);
    expect(basen()).toEqual(new Set(['32eb7a7', '']));
  });

  it('Neu laden fragt das Review erneut an (Befund 2)', async () => {
    const host = await mount(<ReviewPage />);
    await until('Neu laden', () => button(host, 'Neu laden'));
    await click(button(host, 'Neu laden'));
    await until('zweite Anfrage', () => fake.api.assetsReview.mock.calls.length === 2);
    expect(fake.api.assetsReview.mock.calls.map((c) => c[0])).toEqual(['HEAD', 'HEAD']);
  });
});
