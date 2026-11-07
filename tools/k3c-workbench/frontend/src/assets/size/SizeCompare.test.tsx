// @vitest-environment happy-dom
import { afterEach, describe, expect, it, vi } from 'vitest';
import { CatalogPage } from '../catalog/CatalogPage';
import { buttons, click, mount, unmount } from '../testing';
import { clearSizeCache } from './sizeCache';

// B-733: Größenvergleich im Katalog: Szene in Spielhöhe mit Maßstab, Monarch rechts zugewandt, Biom wählbar.

afterEach(() => {
  unmount();
  clearSizeCache();
  vi.restoreAllMocks();
});

async function zeigeGroesse() {
  const host = await mount(<CatalogPage />);
  await click(host.querySelector('[data-asset="monarch"]')!);
  await click(buttons(host, 'Größenvergleich')[0]!);
  return host;
}

describe('Größenvergleich (B-733)', () => {
  it('zeigt Asset und Monarch in Spielgröße mit Maßstab hoch und breit (AC-01)', async () => {
    const host = await zeigeGroesse();
    const scene = host.querySelector('[data-size-scene]')!;
    expect(scene.getAttribute('viewBox')).toMatch(/^0 0 \d+ 1080$/);
    expect(scene.querySelector('[data-massstab="hoch"]')).not.toBeNull();
    expect(scene.querySelector('[data-massstab="breit"]')).not.toBeNull();
    // Mock: Maßstab des Monarchen 81 · 1,5 / 145 px je Einheit, Rahmen 145 Einheiten über dem Fuß → 121,5 px
    expect(scene.querySelector('[data-hoehe-px]')?.getAttribute('data-hoehe-px')).toBe('122');
    expect(scene.querySelector('[data-figur="asset"]')?.getAttribute('transform')).toContain('scale(1 1)');
    expect(scene.querySelector('[data-figur="monarch"]')?.getAttribute('transform')).toContain('scale(-1 1)');
    expect(host.textContent).toContain('Spielrolle Spieler');
  });

  it('Biom-Auswahl wechselt den Hintergrund (AC-02)', async () => {
    const host = await zeigeGroesse();
    expect(host.querySelector('[data-hintergrund]')).not.toBeNull();
    await click(buttons(host, 'ohne')[0]!);
    expect(host.querySelector('[data-hintergrund]')).toBeNull();
    await click(buttons(host, 'wald')[0]!);
    expect(host.querySelector('[data-hintergrund]')).not.toBeNull();
  });
});

describe('Zoom-Stufen (B-733)', () => {
  it('Größenvergleich startet bei 0,5×, 4× vervierfacht die Spiel-Pixel', async () => {
    const host = await zeigeGroesse();
    const scene = () => host.querySelector('[data-size-scene]')!;
    const breite = Number(scene().getAttribute('data-breite'));
    expect(Number(scene().getAttribute('width'))).toBe(breite * 0.5);
    await click(buttons(host, '4×')[0]!);
    expect(Number(scene().getAttribute('width'))).toBe(breite * 4);
  });

  it('Vorschau im Katalog startet bei 2× und folgt der Stufe', async () => {
    const host = await mount(<CatalogPage />);
    await click(host.querySelector('[data-asset="monarch"]')!);
    expect(host.querySelector('img.as-frame')?.getAttribute('data-zoom')).toBe('2');
    await click(buttons(host, '0,5×')[0]!);
    expect(host.querySelector('img.as-frame')?.getAttribute('data-zoom')).toBe('0.5');
  });
});
