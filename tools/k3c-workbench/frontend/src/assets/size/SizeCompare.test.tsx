// @vitest-environment happy-dom
import { afterEach, describe, expect, it, vi } from 'vitest';
import { CatalogPage } from '../catalog/CatalogPage';
import { buttons, click, expandTrees, mount, unmount, until } from '../testing';
import { groesseresMass, stufenPx } from './SizeScene';
import { clearSizeCache } from './sizeCache';

// B-733: Größenvergleich im Katalog: Szene in Spielhöhe mit Maßstab, Monarch rechts zugewandt, Biom wählbar.
// B-1170 (WZ62.3): eine mit dem Fenster skalierende Ansicht mit Lineal der Größenstufen (data/sizes.json), ohne Zoomstufen.

afterEach(() => {
  unmount();
  clearSizeCache();
  vi.restoreAllMocks();
});

async function zeigeGroesse(id = 'monarch') {
  const host = await mount(<CatalogPage />);
  await expandTrees(host);
  await until(`Kachel ${id}`, () => host.querySelector(`[data-asset="${id}"]`));
  await click(host.querySelector(`[data-asset="${id}"]`)!);
  await until('Knopf Größenvergleich', () => buttons(host, 'Größenvergleich')[0]);
  await click(buttons(host, 'Größenvergleich')[0]!);
  await until('Szene', () => host.querySelector('[data-size-scene] [data-figur="asset"]') && host.querySelector('[data-hintergrund]') && host.querySelector('[data-lineal]'));
  return host;
}

describe('Größenvergleich (B-733)', () => {
  it('zeigt Asset und Monarch in Spielgröße mit Lineal in M und Maßstab in Units (AC-01)', async () => {
    const host = await zeigeGroesse();
    const scene = host.querySelector('[data-size-scene]')!;
    expect(scene.getAttribute('viewBox')).toMatch(/^0 -?\d+ \d+ \d+$/);
    expect(scene.querySelector('[data-lineal]')).not.toBeNull();
    expect(scene.querySelector('[data-massstab="breit"]')).not.toBeNull();
    // Mock: Maßstab des Monarchen 81 · 1,5 / 145 px je Einheit, Rahmen 145 Einheiten über dem Fuß → 121,5 px = 1 M
    expect(scene.querySelector('[data-hoehe-px]')?.getAttribute('data-hoehe-px')).toBe('122');
    expect(scene.querySelector('[data-hoehe-m]')?.getAttribute('data-hoehe-m')).toBe('1');
    expect(scene.querySelector('[data-figur="asset"]')?.getAttribute('transform')).toContain('scale(1 1)');
    expect(scene.querySelector('[data-figur="monarch"]')?.getAttribute('transform')).toContain('scale(-1 1)');
    expect(host.textContent).toContain('Spielrolle Spieler');
  });

  it('Biom-Auswahl wechselt den Hintergrund (AC-02)', async () => {
    const host = await zeigeGroesse();
    expect(host.querySelector('[data-hintergrund]')).not.toBeNull();
    await click(buttons(host, 'ohne')[0]!);
    await until('ohne Hintergrund', () => !host.querySelector('[data-hintergrund]'));
    await click(buttons(host, 'wald')[0]!);
    await until('Hintergrund wald', () => host.querySelector('[data-hintergrund]'));
  });
});

describe('Lineal der Größenstufen und Fensterskalierung (B-1170/AC-03)', () => {
  it('die Szene skaliert über viewBox mit dem Fenster, es gibt keine Zoomstufen', async () => {
    const host = await zeigeGroesse();
    const scene = host.querySelector('[data-size-scene]')!;
    expect(scene.getAttribute('preserveAspectRatio')).toBe('xMidYMax meet');
    expect(scene.hasAttribute('width')).toBe(false);
    expect(host.querySelector('[data-size-fill]')).not.toBeNull();
    expect(host.querySelector('[role="group"][aria-label="Zoom"]')).toBeNull();
    for (const z of ['0,5×', '1×', '2×', '4×']) expect(buttons(host, z), z).toHaveLength(0);
  });

  it('das Lineal trägt eine Marke je Größenstufe und die Linie „1 M = Monarch“', async () => {
    const host = await zeigeGroesse();
    const lineal = host.querySelector('[data-lineal]')!;
    for (const id of ['gegner-klein', 'buerger', 'gegner-gross', 'gebaeude-mittel', 'boss']) {
      expect(lineal.querySelector(`[data-stufe-marke="${id}"]`), id).not.toBeNull();
    }
    expect(host.querySelector('[data-linie="eins"]')?.getAttribute('data-linie-px')).toBe('122');
    expect(host.querySelector('[data-linie="eins"]')?.textContent).toContain('1 M');
    expect(lineal.textContent).toContain('0,5 M');
  });

  it('die Stufe des Assets laut sizes.json bekommt eine Linie und ist hervorgehoben', async () => {
    const host = await zeigeGroesse();
    await until('Linie der Stufe', () => host.querySelector('[data-linie="stufe"]'));
    const linie = host.querySelector('[data-linie="stufe"]')!;
    expect(linie.textContent).toContain('gebaeude-mittel');
    expect(linie.getAttribute('data-linie-px')).toBe(String(Math.round(1.1 * 121.5)));
    expect(host.querySelector('[data-stufe="gebaeude-mittel"]')?.getAttribute('data-accent-color')).toBe('orange');
  });

  it('eine gewählte Stufe zeichnet ihre Höhe über die Szene, ein zweiter Klick nimmt sie zurück', async () => {
    const host = await zeigeGroesse();
    expect(host.querySelector('[data-linie="gewaehlt"]')).toBeNull();
    await click(host.querySelector('[data-stufe="boss"]')!);
    const linie = host.querySelector('[data-linie="gewaehlt"]')!;
    expect(linie.getAttribute('data-linie-px')).toBe(String(Math.round(1.3 * 121.5)));
    expect(linie.textContent).toContain('boss');
    await click(host.querySelector('[data-stufe="boss"]')!);
    expect(host.querySelector('[data-linie="gewaehlt"]')).toBeNull();
  });

  it('stufenPx rechnet M in Spiel-Pixel um und kennt unbekannte Stufen nicht', () => {
    const s = { bezugPx: 100, stufen: [{ id: 'boss', m: 1.3, was: '' }], assets: {} };
    expect(stufenPx(s, 'boss')).toBeCloseTo(130);
    expect(stufenPx(s, 'x')).toBe(0);
    expect(stufenPx(undefined, 'boss')).toBe(0);
  });
});

describe('Vorher und Nachher teilen den Maßstab (Review-Befund)', () => {
  it('das größere Maß je Richtung gilt für beide Szenen', () => {
    expect(groesseresMass({ breite: 900, hoehe: 300 }, { breite: 1100, hoehe: 250 })).toEqual({ breite: 1100, hoehe: 300 });
  });
});

describe('Zoom-Stufen der Vorschau (B-733)', () => {
  it('Vorschau im Katalog startet bei 2× und folgt der Stufe', async () => {
    const host = await mount(<CatalogPage />);
    await expandTrees(host);
    await until('Kachel monarch', () => host.querySelector('[data-asset="monarch"]'));
    await click(host.querySelector('[data-asset="monarch"]')!);
    await until('Vorschau bei 2×', () => host.querySelector('img.as-frame')?.getAttribute('data-zoom') === '2');
    await click(buttons(host, '0,5×')[0]!);
    await until('Vorschau bei 0,5×', () => host.querySelector('img.as-frame')?.getAttribute('data-zoom') === '0.5');
  });
});
