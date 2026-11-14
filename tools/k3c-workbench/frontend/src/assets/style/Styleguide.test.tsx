// @vitest-environment happy-dom
import { afterEach, describe, expect, it } from 'vitest';
import { mount, unmount, until } from '../testing';
import { StylePage } from './StylePage';

// WB1.2/AC-02: Styleguide als Materialtafel (nur Anzeige).

afterEach(unmount);

describe('Styleguide (WB1.2)', () => {
  it('zeigt je Material die Töne licht, basis, schatten mit Hex-Wert', async () => {
    const host = await mount(<StylePage />);
    const gold = await until('Material gold', () => host.querySelector<HTMLElement>('[data-material="gold"]'));
    for (const [ton, hex] of [['licht', '#ffe36e'], ['basis', '#f2c33a'], ['schatten', '#c48a1e']]) {
      const swatch = gold.querySelector<HTMLElement>(`[data-ton="${ton}"]`)!;
      expect(swatch.textContent, ton).toContain(hex);
      expect(swatch.querySelector<HTMLElement>('[data-swatch]')!.style.background, ton).not.toBe('');
    }
    expect(host.querySelectorAll('[data-material]').length).toBeGreaterThan(10);
  });

  it('zeigt Einzeltöne und Papier', async () => {
    const host = await mount(<StylePage />);
    await until('Einzeltöne und Papier', () => host.textContent?.includes('Einzeltöne') && host.textContent.includes('Papier'));
  });
});
