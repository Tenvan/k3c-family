// @vitest-environment happy-dom
import { afterEach, describe, expect, it } from 'vitest';
import { pickValue } from '@webkit/testing/select';
import { click, unmount, until } from '../testing';
import { mountCatalog } from './catalogTesting';

// WZ62.2/AC-02 (B-1169): Katalog nach Welt filtern, gesamt oder exklusiv, Herkunft geerbter Assets, Zustand in der Adresse.

afterEach(() => {
  unmount();
  window.history.replaceState(null, '', '#');
});

const entry = (host: ParentNode, id: string) => host.querySelector<HTMLElement>(`[data-asset="${id}"]`);
const ids = (host: ParentNode) => [...host.querySelectorAll('[data-asset]')].map((e) => e.getAttribute('data-asset'));

describe('Katalog nach Welt (B-1169)', () => {
  it('Welt aus der Adresse: gesamt zeigt eigene und geerbte mit Herkunft, ersetzte fehlen', async () => {
    window.history.replaceState(null, '', '#welt=sengoku');
    const host = await mountCatalog();
    await until('Welt geladen', () => !host.textContent?.includes('Lädt Welt'));
    expect(ids(host)).toEqual(expect.arrayContaining(['monarch', 'pferd', 'banner']));
    expect(entry(host, 'mauer')).toBeNull();
    expect(entry(host, 'pferd')!.textContent).toContain('exklusiv');
    expect(entry(host, 'monarch')!.textContent).toContain('geerbt von standard');
  });

  it('exklusiv zeigt nur die Assets der Welt', async () => {
    window.history.replaceState(null, '', '#welt=sengoku&modus=exklusiv');
    const host = await mountCatalog();
    await until('nur Pferd', () => ids(host).join() === 'pferd');
  });

  it('Auswahl schreibt Welt und Modus in die Adresse, Zurücksetzen auf „Alle Welten“ löscht sie', async () => {
    const host = await mountCatalog();
    expect(ids(host).length).toBeGreaterThan(3);
    await pickValue(host, 'Welt', 'sengoku');
    await until('Welt in der Adresse', () => window.location.hash === '#welt=sengoku');
    await click([...host.querySelectorAll('button')].find((b) => b.textContent?.startsWith('exklusiv'))!);
    expect(window.location.hash).toBe('#welt=sengoku&modus=exklusiv');
    await until('nur Pferd', () => ids(host).join() === 'pferd');
    await pickValue(host, 'Welt', '');
    expect(window.location.hash === '' || window.location.hash === '#').toBe(true);
    await until('alle Assets', () => ids(host).length > 3);
  });

  it('Gruppe filtert zusammen mit der Welt', async () => {
    window.history.replaceState(null, '', '#welt=sengoku&gruppe=figuren');
    const host = await mountCatalog();
    await until('nur Figuren', () => ids(host).join() === 'monarch');
  });
});
