// @vitest-environment happy-dom
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { backend } from '../../api';
import type { Vorlage } from '../api';
import { buttons, click, mount, unmount, until } from '../testing';
import { StylePage } from './StylePage';

// WB1.2/AC-02: Styleguide als Materialtafel (nur Anzeige). B-727: Prompt-Buttons für Material, Ton und Papier.

const writeText = vi.fn(async (text: string) => void text);

beforeEach(() => {
  writeText.mockClear();
  Object.defineProperty(navigator, 'clipboard', { value: { writeText }, configurable: true });
});

afterEach(() => {
  unmount();
  vi.restoreAllMocks();
});

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

/** Klickt den Wahlknopf unter `knopf` und wartet auf alle Prompt-Buttons des Orts; liefert Auswahl und Titel. */
async function waehle(host: HTMLElement, knopf: string, ort: Vorlage['ort']) {
  const el = await until(knopf, () => host.querySelector<HTMLElement>(knopf));
  await click(el);
  expect(el.getAttribute('aria-pressed'), knopf).toBe('true');
  const titel = (await backend.assetsPrompts()).filter((v) => v.ort === ort).map((v) => v.titel);
  const auswahl = await until('Auswahl', () => host.querySelector<HTMLElement>(`[data-auswahl="${ort}"]`));
  await until('Prompt-Buttons', () => titel.length > 0 && titel.every((t) => buttons(auswahl, t).length === 1));
  return { auswahl, titel };
}

describe('Styleguide-Prompts (B-727)', () => {
  it('AC-01/AC-02: Material, Ton und Papier haben Buttons; der Prompt nennt den Namen', async () => {
    const host = await mount(<StylePage />);
    await until('Hinweis', () => host.textContent?.includes('zeigt Prompts'));
    const material = await waehle(host, '[data-material="koenigsrot"] [data-wahl="material"]', 'material');
    expect(material.titel).toEqual(['Material ändern', 'Neues Material', 'Nutzer zeigen']);
    await click(buttons(material.auswahl, 'Material ändern')[0]);
    await until('kopiert', () => writeText.mock.calls.length === 1);
    expect(writeText.mock.calls[0][0]).toBe('Stil: Material `koenigsrot` ändern (Stil-Datei public/grafik/stil.paper.json)');

    const ton = await waehle(host, '[data-einzelton="huf"] [data-wahl="ton"]', 'ton');
    expect(host.querySelector('[data-auswahl="material"]')).toBeNull();
    await click(buttons(ton.auswahl, 'Nutzer zeigen')[0]);
    await until('kopiert', () => writeText.mock.calls.length === 2);
    expect(writeText.mock.calls[1][0]).toContain('`huf`');

    const papier = await waehle(host, '[data-wahl="papier"]', 'papier');
    await click(buttons(papier.auswahl, papier.titel[0])[0]);
    await until('kopiert', () => writeText.mock.calls.length === 3);
    expect(writeText.mock.calls[2][0]).toContain('stil.paper.json');
    for (const [text] of writeText.mock.calls) expect(text).not.toMatch(/\{[a-z]+\}/);
  });

  it('ohne Vorlagen für den Ort ein Hinweis statt leerer Fläche', async () => {
    vi.spyOn(backend, 'assetsPrompts').mockResolvedValue([]);
    const host = await mount(<StylePage />);
    await click(await until('Material gold', () => host.querySelector<HTMLElement>('[data-material="gold"] [data-wahl="material"]')));
    const auswahl = await until('Auswahl', () => host.querySelector<HTMLElement>('[data-auswahl="material"]'));
    expect(auswahl.textContent).toContain('Keine Prompt-Vorlagen für Material: gold');
  });
});
